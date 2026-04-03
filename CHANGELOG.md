# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.8](https://github.com/tycho/gloam/compare/0.4.7...0.4.8) - 2026-04-03

### Other

- update bundled XML specs and headers

## [0.4.7](https://github.com/tycho/gloam/compare/0.4.6...0.4.7) - 2026-04-02

### Fixed

- *(vulkan)* ensure we don't leak handle on double-gloamVulkanInitialize

### Other

- *(template)* whitespace fixes for generated C headers
- cargo dependency update
- update bundled XML specs and headers

## [0.4.6](https://github.com/tycho/gloam/compare/0.4.5...0.4.6) - 2026-03-27

### Fixed

- *(vulkan)* make VK_NO_PROTOTYPES only apply to upstream Vulkan headers

## [0.4.5](https://github.com/tycho/gloam/compare/0.4.4...0.4.5) - 2026-03-27

### Fixed

- *(vulkan)* move vk_platform.h to vulkan/vk_platform.h
- *(vulkan)* define VK_NO_PROTOTYPES for --external-headers

### Other

- *(template)* loader template style and ownership cleanup

## [0.4.4](https://github.com/tycho/gloam/compare/0.4.3...0.4.4) - 2026-03-27

### Added

- *(vulkan)* add new --external-headers option for Vulkan

### Fixed

- *(generator)* clean up whitespace usage for function declarations
- *(vulkan)* unbreak loading device functions in enabled-list path
- *(vulkan)* load instance functions when enabling device extensions
- *(c)* use correct naming for gloamVulkanGetInstanceVersion
- *(generator)* ensure trailing newline at end of generated sources

### Other

- *(doc)* ensure documentation matches reality
- *(vulkan)* simplify internals, unify pfn range loading functions

## [0.4.3](https://github.com/tycho/gloam/compare/0.4.2...0.4.3) - 2026-03-26

### Added

- *(c)* implement GetInstanceVersion/GetLoadedInstance/GetLoadedDevice for Vulkan

## [0.4.2](https://github.com/tycho/gloam/compare/0.4.1...0.4.2) - 2026-03-26

### Fixed

- *(fetch)* use correct URL for vk_video headers

## [0.4.1](https://github.com/tycho/gloam/compare/0.4.0...0.4.1) - 2026-03-26

### Added

- *(tests)* add better integration test coverage

### Fixed

- *(c)* Visual Studio warnings about APIENTRY redefinition and implicit int-to-char conversion
- *(c)* build on Visual Studio in C11 mode (enum aliasing)
- *(c)* 'cargo test' from within Cygwin

### Other

- *(doc)* improve README and add contribution guides

## [0.4.0](https://github.com/tycho/gloam/compare/0.3.0...0.4.0) - 2026-03-24

### Added

- *(vulkan)* [**breaking**] remove the --unchecked experiment
- *(vulkan)* [**breaking**] add three-phase enabled-path loader API, simplify public surface

### Fixed

- *(vulkan)* only enumerate instance and device extensions once

## [0.3.0](https://github.com/tycho/gloam/compare/0.2.4...0.3.0) - 2026-03-24

### Added

- *(vulkan)* implement **--unchecked** mode: allows creating a Vulkan loader
  similar to Volk, with no extension or version detection. Many Vulkan users
  prefer to do extension detection themselves, because they need to set up
  various per-extension context creation structures anyway. This version is
  much smaller in terms of binary size (even though we are already very small).
  We're roughly half the size of Volk overall right now.

### Fixed

- *(header)* add protections around VK extension guard macros and extension
  version/name

### Other

- *(bundled)* treat `glsl_exts.xml` in this repo as the canon version, since it
  doesn't exist in any real upstream source. It's just a list of GLSL-only
  extensions that aren't represented in the Khronos XML files.
- *(template)* move extension string hashing to common function
- *(refactoring)* refactored much of the Rust code to be easier to reason about

## [0.2.4](https://github.com/tycho/gloam/compare/0.2.3...0.2.4) - 2026-03-23

### Added

- *(generator)* coalesce consecutive identical #ifdef/#endif blocks in generated header

### Other

- *(c)* ensure 'return'/'break' statements are on their own line

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
