# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2](https://github.com/JayanAXHF/filessh/compare/v0.1.1...v0.1.2) - 2025-11-12

### Added

- *(TUI)* Added file sizes to the download progress.

### Other

- *(optimization)* Added optimization options to `Cargo.toml`
- Formatted Changelog
- *(deps:release-plz)* setup release-plz
## [0.1.1] - 2025-11-11

### Miscellaneous Tasks

- Removed useless print messages


## [0.1.0] - 2025-11-11

### Features

- Created till functioning tui
- *(TUI)* Made the TUI faster by removing multiple ssh connections
- *(TUI)* Added line gauge, fixed errors
- *(TUI)* Added a remaining files download list
- *(TUI)* Added TachyonFx for smooth transitions
- *(TUI)* Added content viewing
- [**breaking**] Added provisional README
- *(docs)* Added git-cliff support

### Bug Fixes

- *(TUI)* Fixed the tui-logger widget
- *(log)* Fixed logging
- *(lint)* Fixed clippy lints

### Refactor

- Separated tui.rs into separate module

### Documentation

- Committeed CHANGELOG.MD

### Miscellaneous Tasks

- *(release)* V0.1.0
