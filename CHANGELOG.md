# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4](https://github.com/tycho/gloam/compare/0.1.3...0.1.4) - 2026-03-21

### Fixed

- *(doc)* update README to reflect new command line arguments

### Other

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
