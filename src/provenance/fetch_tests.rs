//! Deterministic tests for the fetch path (acquisition + cache + engine) that
//! exercise `reqwest` end-to-end against a local mock GitHub server — never the
//! real API and never the production cache file.
//!
//! The mock serves the exact endpoints `acquire` calls, from a synthetic
//! fixture built from the real registry (so tests don't depend on the current
//! `bundled/` tree).  A request log lets tests assert *which* network calls
//! happened, distinguishing cache hits from misses.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use indexmap::IndexMap;

use super::acquire::Github;
use super::cache::{self, Cache};
use super::engine::Engine;
use super::find;
use super::manifest::{ProvenancePin, git_blob_sha1};

// ---------------------------------------------------------------------------
// Fixture + mock server
// ---------------------------------------------------------------------------

/// Files larger than this are served like GitHub serves >1MB files: the
/// Contents API returns only the blob SHA, and content must be fetched via
/// the blobs API.
const MOCK_INLINE_LIMIT: usize = 1024;

#[derive(Clone)]
struct RepoData {
    head: String,
    /// path_in_repo -> (blob sha, content)
    files: HashMap<String, (String, Vec<u8>)>,
}

/// Build a synthetic fixture for the given registry keys: each file gets
/// deterministic content and a self-consistent git blob SHA; each repo a
/// deterministic fake HEAD commit.
fn fixture(keys: &[&str]) -> HashMap<String, RepoData> {
    let mut repos: HashMap<String, RepoData> = HashMap::new();
    for &key in keys {
        let (cluster, file) = find(key).expect("registry key");
        let content = format!("// synthetic content for {key}\n").into_bytes();
        let blob = git_blob_sha1(&content);
        let rd = repos
            .entry(cluster.repo.to_string())
            .or_insert_with(|| RepoData {
                head: git_blob_sha1(cluster.repo.as_bytes()),
                files: HashMap::new(),
            });
        rd.files
            .insert(file.path_in_repo.to_string(), (blob, content));
    }
    repos
}

struct MockGitHub {
    base_url: String,
    log: Arc<Mutex<Vec<String>>>,
}

impl MockGitHub {
    fn start(fixture: HashMap<String, RepoData>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        let log = Arc::new(Mutex::new(Vec::new()));
        let fixture = Arc::new(fixture);
        let log_t = log.clone();
        // Detached: the listener lives as long as the test process.
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut s) => handle_conn(&mut s, &fixture, &log_t),
                    Err(_) => break,
                }
            }
        });
        Self { base_url, log }
    }

    fn requests(&self) -> Vec<String> {
        self.log.lock().unwrap().clone()
    }
}

fn handle_conn(
    stream: &mut TcpStream,
    fixture: &HashMap<String, RepoData>,
    log: &Mutex<Vec<String>>,
) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    // Drain the rest of the request headers.
    loop {
        let mut h = String::new();
        match reader.read_line(&mut h) {
            Ok(0) => break,
            Ok(_) if h == "\r\n" || h == "\n" => break,
            Ok(_) => {}
            Err(_) => break,
        }
    }

    let path = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .to_string();
    log.lock().unwrap().push(path.clone());

    let (status, body) = match route(&path, fixture) {
        Some(b) => ("200 OK", b),
        None => ("404 Not Found", String::new()),
    };
    let resp = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(resp.as_bytes());
    let _ = stream.flush();
}

/// Serve the GitHub endpoints `acquire` uses.  Returns the JSON body, or `None`
/// for a 404.
fn route(path: &str, fixture: &HashMap<String, RepoData>) -> Option<String> {
    let b64 = |c: &[u8]| BASE64.encode(c);
    let p = path.split('?').next().unwrap_or(path);
    let segs: Vec<&str> = p
        .trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();
    if segs.first() != Some(&"repos") {
        return None;
    }
    let repo = format!("{}/{}", segs.get(1)?, segs.get(2)?);
    let rd = fixture.get(&repo)?;
    let rest = &segs[3..];

    let json = match rest {
        ["git", "ref", "heads", _branch] => serde_json::json!({ "object": { "sha": rd.head } }),
        ["tags"] => serde_json::json!([]),
        ["commits"] => serde_json::json!([{ "sha": rd.head }]),
        ["git", "blobs", sha] => {
            let (_path, content) = rd.files.values().find(|(b, _)| b == sha)?;
            serde_json::json!({ "content": b64(content), "encoding": "base64" })
        }
        ["contents", parts @ ..] => {
            let path_in_repo = parts.join("/");
            let (blob, content) = rd.files.get(&path_in_repo)?;
            if content.len() > MOCK_INLINE_LIMIT {
                // Mirror GitHub's >1MB behavior: blob SHA only, no inline
                // content — the caller must fetch the blob separately.
                serde_json::json!({ "sha": blob, "encoding": "none" })
            } else {
                serde_json::json!({ "sha": blob, "content": b64(content), "encoding": "base64" })
            }
        }
        _ => return None,
    };
    Some(json.to_string())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn engine_for(server: &MockGitHub, cache: Cache) -> Engine {
    let gh = Github::with_base_url(&server.base_url).unwrap();
    Engine::from_parts(gh, cache)
}

/// Fully seed the cache for a key from the fixture (HEAD + commit + tree + blob).
fn seed(cache: &Cache, fixture: &HashMap<String, RepoData>, key: &str, head_age_secs: i64) {
    let (cluster, file) = find(key).unwrap();
    let rd = &fixture[cluster.repo];
    let (blob, content) = &rd.files[file.path_in_repo];
    let now = cache::now();
    cache
        .set_head(cluster.repo, cluster.branch, &rd.head, now - head_age_secs)
        .unwrap();
    cache
        .put_commit(&rd.head, cluster.repo, "desc", now)
        .unwrap();
    cache
        .put_tree_entry(&rd.head, file.path_in_repo, blob)
        .unwrap();
    cache.put_blob(blob, content, now).unwrap();
}

fn fixture_content(fixture: &HashMap<String, RepoData>, key: &str) -> Vec<u8> {
    let (cluster, file) = find(key).unwrap();
    fixture[cluster.repo].files[file.path_in_repo].1.clone()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn full_cache_hit_makes_no_requests() {
    let fix = fixture(&["xxhash.h"]);
    let server = MockGitHub::start(fix.clone());
    let cache = Cache::open_in_memory().unwrap();
    seed(&cache, &fix, "xxhash.h", 0);

    let engine = engine_for(&server, cache);
    let out = engine.resolve_head(&["xxhash.h"]).unwrap();

    assert_eq!(out["xxhash.h"].content, fixture_content(&fix, "xxhash.h"));
    assert!(
        server.requests().is_empty(),
        "a full cache hit must not touch the network: {:?}",
        server.requests()
    );
}

#[test]
fn cache_miss_pulls_through_then_hits() {
    let fix = fixture(&["gl.xml"]);
    let server = MockGitHub::start(fix.clone());
    let engine = engine_for(&server, Cache::open_in_memory().unwrap());

    // First pass: cold cache → fetches from the mock.
    let out = engine.resolve_head(&["gl.xml"]).unwrap();
    assert_eq!(out["gl.xml"].content, fixture_content(&fix, "gl.xml"));
    let after_first = server.requests().len();
    assert!(
        after_first > 0,
        "a cold cache must pull through to the network"
    );

    // Second pass: everything is cached → no further requests.
    let _ = engine.resolve_head(&["gl.xml"]).unwrap();
    assert_eq!(
        server.requests().len(),
        after_first,
        "a warm cache must not re-request: {:?}",
        server.requests()
    );
}

#[test]
fn stale_head_unchanged_upstream_reuses_cached_blobs() {
    let fix = fixture(&["vk.xml"]);
    let server = MockGitHub::start(fix.clone());
    let cache = Cache::open_in_memory().unwrap();
    // Seed with a HEAD fetched long ago (past the HEAD TTL) but warm blobs.
    seed(&cache, &fix, "vk.xml", cache::HEAD_TTL_SECS * 10);

    let engine = engine_for(&server, cache);
    let _ = engine.resolve_head(&["vk.xml"]).unwrap();

    let reqs = server.requests();
    // HEAD is re-resolved (stale)...
    assert!(
        reqs.iter().any(|p| p.contains("/git/ref/heads/")),
        "a stale HEAD should be re-resolved: {reqs:?}"
    );
    // ...but the unchanged content is served from cache: no contents/blob calls.
    assert!(
        !reqs
            .iter()
            .any(|p| p.contains("/contents/") || p.contains("/git/blobs/")),
        "unchanged upstream must not re-download content: {reqs:?}"
    );
}

#[test]
fn unchanged_large_blob_is_not_redownloaded_at_new_commit() {
    // Simulates the weekly upstream-commit case: HEAD advances (no cached
    // tree entry for the new commit) but a large file's blob is unchanged
    // and already cached.  The engine must resolve the SHA via the Contents
    // API and then serve the content from cache — never the blobs API.
    let mut fix = fixture(&["vk.xml"]);
    let (cluster, file) = find("vk.xml").unwrap();
    let content = vec![b'v'; MOCK_INLINE_LIMIT * 4];
    let blob = git_blob_sha1(&content);
    fix.get_mut(cluster.repo)
        .unwrap()
        .files
        .insert(file.path_in_repo.to_string(), (blob.clone(), content.clone()));

    let server = MockGitHub::start(fix);
    let cache = Cache::open_in_memory().unwrap();
    // Warm blob only — no HEAD row, no tree entry for the new commit.
    cache.put_blob(&blob, &content, cache::now()).unwrap();

    let engine = engine_for(&server, cache);
    let out = engine.resolve_head(&["vk.xml"]).unwrap();
    assert_eq!(out["vk.xml"].content, content);

    let reqs = server.requests();
    assert!(
        reqs.iter().any(|p| p.contains("/contents/")),
        "the new commit's blob SHA must be resolved: {reqs:?}"
    );
    assert!(
        !reqs.iter().any(|p| p.contains("/git/blobs/")),
        "an unchanged large blob must not be re-downloaded: {reqs:?}"
    );
}

#[test]
fn changed_large_blob_is_downloaded_via_blob_api() {
    // Counterpart: a large file whose blob is NOT cached must pull through
    // the blobs API (and end up cached for next time).
    let mut fix = fixture(&["vk.xml"]);
    let (cluster, file) = find("vk.xml").unwrap();
    let content = vec![b'w'; MOCK_INLINE_LIMIT * 4];
    let blob = git_blob_sha1(&content);
    fix.get_mut(cluster.repo)
        .unwrap()
        .files
        .insert(file.path_in_repo.to_string(), (blob, content.clone()));

    let server = MockGitHub::start(fix);
    let engine = engine_for(&server, Cache::open_in_memory().unwrap());
    let out = engine.resolve_head(&["vk.xml"]).unwrap();
    assert_eq!(out["vk.xml"].content, content);
    assert!(
        server.requests().iter().any(|p| p.contains("/git/blobs/")),
        "a cold large blob must be fetched: {:?}",
        server.requests()
    );
}

#[test]
fn lock_resolves_by_blob_cache_first() {
    let fix = fixture(&["gl.xml"]);
    let server = MockGitHub::start(fix.clone());
    let engine = engine_for(&server, Cache::open_in_memory().unwrap());

    let (cluster, file) = find("gl.xml").unwrap();
    let rd = &fix[cluster.repo];
    let (blob, _content) = &rd.files[file.path_in_repo];
    let mut pins = IndexMap::new();
    pins.insert(
        "gl.xml".to_string(),
        ProvenancePin {
            repo: cluster.repo.to_string(),
            repo_url: cluster.repo_url.to_string(),
            path_in_repo: file.path_in_repo.to_string(),
            commit: rd.head.clone(),
            describe: "desc".to_string(),
            blob: blob.clone(),
        },
    );

    // Cold: lock fetches the blob by SHA.
    let out = engine.resolve_pinned(&pins, &["gl.xml"]).unwrap();
    assert_eq!(out["gl.xml"].content, fixture_content(&fix, "gl.xml"));
    assert!(
        server.requests().iter().any(|p| p.contains("/git/blobs/")),
        "a cold lock resolve fetches the blob: {:?}",
        server.requests()
    );

    // Warm: served from cache, no further requests.
    let n = server.requests().len();
    let _ = engine.resolve_pinned(&pins, &["gl.xml"]).unwrap();
    assert_eq!(
        server.requests().len(),
        n,
        "a warm lock resolve must not re-request: {:?}",
        server.requests()
    );
}
