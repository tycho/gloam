//! Local provenance cache (SQLite via `rusqlite`).
//!
//! Exists only to serve `--fetch`: it caches upstream blobs and their commit /
//! tree metadata so repeated generation (and the pregen preflight)
//! avoids redundant API traffic, and so `--lock` against older snapshots works
//! without disturbing our notion of each repo's HEAD.
//!
//! Two independent TTL classes:
//!   - **HEAD TTL** (`repos.head_fetched_at`): how long a cached branch HEAD is
//!     trusted before a no-lock `--fetch` re-resolves it.  Governs re-resolution
//!     only — never eviction.
//!   - **Object TTL** (`commits.last_used`, `blobs.last_used`): how long unused
//!     commits/blobs survive before eviction.  Refreshed on every reuse.
//!
//! The cache is pure derived data (rebuildable from the network and the bundle),
//! so we never migrate: a `PRAGMA user_version` mismatch drops and recreates.

use anyhow::{Context, Result};
use rusqlite::{Connection, OptionalExtension, params};

/// Bump on any incompatible schema change; a mismatch drops & recreates.
const SCHEMA_VERSION: i64 = 2;

/// Default HEAD TTL: re-resolve a branch HEAD at most ~daily.
pub const HEAD_TTL_SECS: i64 = 24 * 60 * 60;
/// Default object TTL: evict commits/blobs unused for ~30 days.
pub const OBJECT_TTL_SECS: i64 = 30 * 24 * 60 * 60;

/// Current Unix time in seconds.
pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub struct Cache {
    conn: Connection,
}

impl Cache {
    /// Open (creating if needed) the cache at the platform cache directory:
    /// `%LOCALAPPDATA%\gloam\cache`, `~/.cache/gloam`, or `~/Library/Caches/gloam`.
    pub fn open_default() -> Result<Self> {
        let dirs = directories::ProjectDirs::from("", "", "gloam")
            .context("locating platform cache directory")?;
        let dir = dirs.cache_dir();
        std::fs::create_dir_all(dir)
            .with_context(|| format!("creating cache directory {}", dir.display()))?;
        let path = dir.join("cache.sqlite");
        let conn = Connection::open(&path)
            .with_context(|| format!("opening cache database {}", path.display()))?;
        Self::from_connection(conn)
    }

    /// Open a private in-memory cache — for tests, never the production file.
    pub fn open_in_memory() -> Result<Self> {
        Self::from_connection(Connection::open_in_memory()?)
    }

    /// Wrap an existing connection: enable WAL and ensure the schema matches.
    pub fn from_connection(conn: Connection) -> Result<Self> {
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
            .context("configuring cache database")?;
        let version: i64 = conn.pragma_query_value(None, "user_version", |r| r.get(0))?;
        if version != SCHEMA_VERSION {
            // Pure derived data — drop and recreate rather than migrate.
            conn.execute_batch(
                "DROP TABLE IF EXISTS repos;
                 DROP TABLE IF EXISTS commits;
                 DROP TABLE IF EXISTS tree_entries;
                 DROP TABLE IF EXISTS blobs;",
            )
            .context("dropping stale cache schema")?;
            conn.execute_batch(SCHEMA)
                .context("creating cache schema")?;
            conn.pragma_update(None, "user_version", SCHEMA_VERSION)?;
        }
        Ok(Self { conn })
    }

    /// Begin a transaction covering subsequent cache calls on this
    /// connection, committing on `commit()`.  Batching many small writes
    /// (e.g. bundle seeding) into one transaction replaces one WAL commit
    /// per statement with one per batch.
    pub fn transaction(&self) -> Result<rusqlite::Transaction<'_>> {
        Ok(self.conn.unchecked_transaction()?)
    }

    // -- HEAD (repos) --------------------------------------------------------

    /// Return the cached HEAD commit for `repo` if it was fetched within the
    /// HEAD TTL (i.e. still trustworthy without re-resolving).
    pub fn fresh_head(&self, repo: &str, now: i64, ttl: i64) -> Result<Option<String>> {
        let row = self
            .conn
            .query_row(
                "SELECT head_commit, head_fetched_at FROM repos WHERE repo = ?1",
                params![repo],
                |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)),
            )
            .optional()?;
        Ok(match row {
            Some((commit, fetched_at)) if fetched_at + ttl >= now => Some(commit),
            _ => None,
        })
    }

    /// Record (or refresh) a repo's branch HEAD.
    pub fn set_head(&self, repo: &str, branch: &str, commit: &str, now: i64) -> Result<()> {
        self.conn.execute(
            "INSERT INTO repos(repo, branch, head_commit, head_fetched_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(repo) DO UPDATE SET
                branch = excluded.branch,
                head_commit = excluded.head_commit,
                head_fetched_at = excluded.head_fetched_at",
            params![repo, branch, commit, now],
        )?;
        Ok(())
    }

    // -- commits -------------------------------------------------------------

    /// Insert a commit row (no-op if already present, but refresh use).
    /// Commit rows exist for eviction bookkeeping: tree entries are swept
    /// when their commit is evicted.
    pub fn put_commit(&self, commit_sha: &str, repo: &str, now: i64) -> Result<()> {
        self.conn.execute(
            "INSERT INTO commits(commit_sha, repo, last_used)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(commit_sha) DO UPDATE SET
                last_used = excluded.last_used",
            params![commit_sha, repo, now],
        )?;
        Ok(())
    }

    // -- tree entries --------------------------------------------------------

    /// Record that `path_in_repo` at `commit_sha` resolves to `blob_sha`.
    pub fn put_tree_entry(
        &self,
        commit_sha: &str,
        path_in_repo: &str,
        blob_sha: &str,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tree_entries(commit_sha, path_in_repo, blob_sha)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(commit_sha, path_in_repo) DO UPDATE SET blob_sha = excluded.blob_sha",
            params![commit_sha, path_in_repo, blob_sha],
        )?;
        Ok(())
    }

    /// Resolve a path at a commit to its blob SHA (no last-used bump; blobs are
    /// tracked separately).
    pub fn blob_for_path(&self, commit_sha: &str, path_in_repo: &str) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row(
                "SELECT blob_sha FROM tree_entries WHERE commit_sha = ?1 AND path_in_repo = ?2",
                params![commit_sha, path_in_repo],
                |r| r.get::<_, String>(0),
            )
            .optional()?)
    }

    // -- blobs ---------------------------------------------------------------

    /// Store blob content (content-addressed; idempotent).
    pub fn put_blob(&self, blob_sha: &str, content: &[u8], now: i64) -> Result<()> {
        self.conn.execute(
            "INSERT INTO blobs(blob_sha, content, last_used)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(blob_sha) DO UPDATE SET last_used = excluded.last_used",
            params![blob_sha, content, now],
        )?;
        Ok(())
    }

    /// Fetch blob content by SHA, refreshing its last-used timestamp.
    pub fn blob(&self, blob_sha: &str, now: i64) -> Result<Option<Vec<u8>>> {
        let content = self
            .conn
            .query_row(
                "SELECT content FROM blobs WHERE blob_sha = ?1",
                params![blob_sha],
                |r| r.get::<_, Vec<u8>>(0),
            )
            .optional()?;
        if content.is_some() {
            self.conn.execute(
                "UPDATE blobs SET last_used = ?2 WHERE blob_sha = ?1",
                params![blob_sha, now],
            )?;
        }
        Ok(content)
    }

    /// Whether a blob is present (no last-used bump).
    pub fn has_blob(&self, blob_sha: &str) -> Result<bool> {
        Ok(self
            .conn
            .query_row(
                "SELECT 1 FROM blobs WHERE blob_sha = ?1",
                params![blob_sha],
                |_| Ok(()),
            )
            .optional()?
            .is_some())
    }

    // -- eviction ------------------------------------------------------------

    /// Evict commits/blobs unused for longer than `object_ttl`, then sweep
    /// tree entries orphaned by an evicted commit.  HEAD rows are never evicted
    /// (their freshness only governs re-resolution).
    pub fn evict(&self, now: i64, object_ttl: i64) -> Result<()> {
        let cutoff = now - object_ttl;
        self.conn.execute_batch(&format!(
            "DELETE FROM commits WHERE last_used < {cutoff};
             DELETE FROM blobs   WHERE last_used < {cutoff};
             DELETE FROM tree_entries
                WHERE commit_sha NOT IN (SELECT commit_sha FROM commits);",
        ))?;
        Ok(())
    }
}

const SCHEMA: &str = "
CREATE TABLE repos(
    repo            TEXT PRIMARY KEY,
    branch          TEXT NOT NULL,
    head_commit     TEXT NOT NULL,
    head_fetched_at INTEGER NOT NULL
);
CREATE TABLE commits(
    commit_sha TEXT PRIMARY KEY,
    repo       TEXT NOT NULL,
    last_used  INTEGER NOT NULL
);
CREATE TABLE tree_entries(
    commit_sha   TEXT NOT NULL,
    path_in_repo TEXT NOT NULL,
    blob_sha     TEXT NOT NULL,
    PRIMARY KEY (commit_sha, path_in_repo)
);
CREATE TABLE blobs(
    blob_sha  TEXT PRIMARY KEY,
    content   BLOB NOT NULL,
    last_used INTEGER NOT NULL
);
CREATE INDEX idx_blobs_last_used   ON blobs(last_used);
CREATE INDEX idx_commits_last_used ON commits(last_used);
";

#[cfg(test)]
mod tests {
    use super::*;

    fn mem() -> Cache {
        Cache::from_connection(Connection::open_in_memory().unwrap()).unwrap()
    }

    #[test]
    fn schema_initialized_with_version() {
        let c = mem();
        let v: i64 = c
            .conn
            .pragma_query_value(None, "user_version", |r| r.get(0))
            .unwrap();
        assert_eq!(v, SCHEMA_VERSION);
    }

    #[test]
    fn blob_roundtrip_and_last_used_bump() {
        let c = mem();
        c.put_blob("deadbeef", b"hello", 100).unwrap();
        assert!(c.has_blob("deadbeef").unwrap());
        assert_eq!(
            c.blob("deadbeef", 200).unwrap().as_deref(),
            Some(&b"hello"[..])
        );
        // last_used was bumped to 200 by the read.
        let lu: i64 = c
            .conn
            .query_row(
                "SELECT last_used FROM blobs WHERE blob_sha='deadbeef'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(lu, 200);
        assert!(c.blob("missing", 200).unwrap().is_none());
    }

    #[test]
    fn head_freshness_respects_ttl() {
        let c = mem();
        c.set_head("acme/repo", "main", "c0ffee", 1000).unwrap();
        // Within TTL.
        assert_eq!(
            c.fresh_head("acme/repo", 1000 + 10, 100)
                .unwrap()
                .as_deref(),
            Some("c0ffee")
        );
        // Past TTL → not fresh (caller must re-resolve).
        assert_eq!(c.fresh_head("acme/repo", 1000 + 200, 100).unwrap(), None);
        // Unknown repo.
        assert_eq!(c.fresh_head("nope/repo", 1000, 100).unwrap(), None);
    }

    /// Whether a commit row exists (test helper — production code never needs
    /// to read commits back; they exist for eviction bookkeeping).
    fn has_commit(c: &Cache, sha: &str) -> bool {
        c.conn
            .query_row(
                "SELECT 1 FROM commits WHERE commit_sha = ?1",
                params![sha],
                |_| Ok(()),
            )
            .optional()
            .unwrap()
            .is_some()
    }

    #[test]
    fn commit_and_tree_roundtrip() {
        let c = mem();
        c.put_commit("abc123", "acme/repo", 50).unwrap();
        c.put_tree_entry("abc123", "src/x.h", "blob99").unwrap();
        assert!(has_commit(&c, "abc123"));
        assert_eq!(
            c.blob_for_path("abc123", "src/x.h").unwrap().as_deref(),
            Some("blob99")
        );
        assert_eq!(c.blob_for_path("abc123", "missing").unwrap(), None);
    }

    #[test]
    fn eviction_drops_cold_objects_and_orphan_tree_entries() {
        let c = mem();
        // Cold commit + its tree entry + a cold blob.
        c.put_commit("cold", "acme/repo", 0).unwrap();
        c.put_tree_entry("cold", "p", "coldblob").unwrap();
        c.put_blob("coldblob", b"x", 0).unwrap();
        // Warm blob.
        c.put_blob("warmblob", b"y", 10_000).unwrap();

        // now=100000, ttl=1000 → cutoff 99000; cold (t=0) evicted, warm kept...
        // bump warm first so it survives.
        c.blob("warmblob", 100_000).unwrap();
        c.evict(100_000, 1_000).unwrap();

        assert!(!c.has_blob("coldblob").unwrap(), "cold blob evicted");
        assert!(c.has_blob("warmblob").unwrap(), "warm blob kept");
        assert!(!has_commit(&c, "cold"), "cold commit evicted");
        assert_eq!(
            c.blob_for_path("cold", "p").unwrap(),
            None,
            "orphaned tree entry swept"
        );
    }

    #[test]
    fn version_mismatch_drops_and_recreates() {
        let conn = Connection::open_in_memory().unwrap();
        let c = Cache::from_connection(conn).unwrap();
        c.put_blob("keep?", b"data", 1).unwrap();
        assert!(c.has_blob("keep?").unwrap());
        // Simulate a future/foreign schema version, then re-init.
        c.conn.pragma_update(None, "user_version", 999).unwrap();
        let conn = c.conn; // move connection out
        let c2 = Cache::from_connection(conn).unwrap();
        assert!(
            !c2.has_blob("keep?").unwrap(),
            "data dropped on version mismatch"
        );
        let v: i64 = c2
            .conn
            .pragma_query_value(None, "user_version", |r| r.get(0))
            .unwrap();
        assert_eq!(v, SCHEMA_VERSION);
    }
}
