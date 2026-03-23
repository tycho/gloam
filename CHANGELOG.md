# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.4](https://github.com/tycho/gloam/compare/0.2.3...0.2.4) - 2026-03-23

### Added

- *(generator)* coalesce consecutive identical #ifdef/#endif blocks in generated header

## [0.2.3](https://github.com/tycho/gloam/compare/0.2.2...0.2.3) - 2026-03-23

### Fixed

- *(generator)* simplify concatenated extension construction for EGL
- *(types)* drop 'enum' keyword from enum alias typedefs

## [0.2.2](https://github.com/tycho/gloam/compare/0.2.1...0.2.2) - 2026-03-23

### Added

- *(build)* support building without --fetch enabled
- *(cli)* allow explicit extension names to override --baseline exclusion
- *(cli)* add extension exclusion syntax and --baseline flag

### Fixed

- *(baseline)* exclusion must check all APIs in merged builds
- *(preamble)* ensure sorted order for extension listings

### Other

- *(cleanup)* ensure -std=c99 -Wall compiles with no warnings

## [0.2.1](https://github.com/tycho/gloam/compare/0.2.0...0.2.1) - 2026-03-22

### Added

- *(gen)* replace function name pointer array with blob + offset table
- *(resolve)* optimize PFN array ordering to minimize range fragmentation

### Fixed

- *(build.rs)* ensure detected git repo root is ours

### Other

- add 'cargo hack check' step
- use correct minimum Rust version
- *(ci)* bump rustsec/audit-check to current HEAD

## [0.2.0](https://github.com/tycho/gloam/compare/0.1.7...0.2.0) - 2026-03-22

### Removed

- [**breaking**] Removing incomplete and untested Rust loader generator.
  I hadn't been building and testing around the Rust output target, and I don't
  want to keep shipping code that I am not using or testing myself. If someone
  wants to implement a Rust loader generator using some of the same principles
  from the C loader generator, contributions would be welcome.

### Fixed

- *(build)* Try to fix version numbering in release builds.

### Other

- remove LICENSE.asc, add Apache license

## [0.1.7](https://github.com/tycho/gloam/compare/0.1.6...0.1.7) - 2026-03-21

### Added

- *(resolve)* tag extensions with selection reason, show in preamble
- *(gen)* add copyright and provenance preamble to generated files
- *(build)* embed git version info at build time

### Fixed

- *(resolve)* follow extension-to-extension dependencies

### Other

- *(cleanup)* fix some clippy warnings

## [0.1.6](https://github.com/tycho/gloam/compare/0.1.5...0.1.6) - 2026-03-21

### Fixed

- *(vulkan)* ensure we use the proper spec name in the generated file
- *(resolve)* avoid double-inclusion of dependent headers
- *(vulkan)* ensure we include the vk_video headers we need
- *(cli)* treat `--api vk=` and `--api vulkan=` as equivalent
- *(tests)* add test coverage for vulkan generation
- *(fetch)* fall back to bundled file for unmatched upstream URLs
- *(resolve)* algorithmic and readability improvements
- *(tests)* ensure that 'compatibility' profile has features we expect
- *(tests)* check for version and extension support macros

## [0.1.5](https://github.com/tycho/gloam/compare/0.1.4...0.1.5) - 2026-03-21

### Fixed

- *(gl)* use GL_NUM_EXTENSIONS constant instead of magic value
- *(header)* add missing version/extension compile-time guard macros
- *(cli)* make CLI argument descriptions slightly more clear

### Other

- *(c)* fix some generated formatting issues

## [0.1.4](https://github.com/tycho/gloam/compare/0.1.3...0.1.4) - 2026-03-21

### Fixed

- *(doc)* use correct syntax for --predecessors and --promoted examples
- *(doc)* update README to reflect new command line arguments

### Other

- *(github)* use correct branch name for CI workflow
- *(tests)* add tests for --promoted and --predecessors

## [0.1.3](https://github.com/tycho/gloam/compare/0.1.2...0.1.3) - 2026-03-21

### Added

- *(resolve)* add --promoted and --predecessors extension selection flags

### Fixed

- *(resolve)* include enums in predecessor check too
- *(resolve)* seed req_types from command parameter types for Vulkan

## [0.1.2](https://github.com/tycho/gloam/compare/0.1.1...0.1.2) - 2026-03-20

### Fixed

- *(resolve)* avoid removing enums needed by extensions

### Other

- *(cargo)* update rustls-webpki dependency

## [0.1.1](https://github.com/tycho/gloam/compare/0.1.0...0.1.1) - 2026-03-20

### Fixed

- *(c-generator)* add missing xxhash.h dependency to include tree

### Other

- *(ci)* add rust format checking
