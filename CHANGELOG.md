# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Schema] - 2024-??-??

### Added

### Changed

### Fixed

### Removed

## [0.2.3] - 2024-??-??

### Added

### Changed

- take care of mdlinter messages

### Fixed

### Removed

## [0.2.2] - 2024-07-27

### Fixed

- change of zenoh config syntax in provided config files

## [0.2.1] - 2024-07-26

### Changed

- priority for lint definitions
- indentation in doc comments

### Removed

- rust-version definition in Cargo.toml files

## [0.2.0] - 2024-05-15

### Changed

- Adopted to `dimas` version 0.2
- Use different prefixes for robot & workstation agents

### Fixed

- Path to tmp directory for Windows

## [0.0.5] - 2024-04-25

### Changed

- Adopted to `dimas` version 0.1
- changed tracing_subscriber::fmt::init() to init_tracing()

## [0.0.4] - 2024-03-22

### Changed

- Adopted to `dimas` version 0.0.8
- Removed tokio "flavor=current_thread" due to changes in zenoh

## [0.0.3] - 2024-03-18

### Changed

- Adopted to `dimas` version 0.0.7

## [0.0.2] - 2024-03-03

### Changed

- Adopted to `dimas` version 0.0.6

## [0.0.1] - 2024-02-29

### Changed

- Moved montblanc benchmark from repo `dimas` into its own repo `examples`
