# Changelog

All notable changes to `ocio-rs` will be documented in this file.

The format is based on Keep a Changelog, and this project follows Semantic
Versioning as closely as practical for a still-maturing binding crate.

## [Unreleased]

### Added

- Real bundled-package validation for `ocio-sys`, including offline packaged
  bundled builds for the current vendored configuration
- Behavioral coverage for `DynamicProperty` interactions across `Processor` and
  `CPUProcessor`
- Stronger `MatrixTransform` behavioral tests for `fit` and `view`
- Release and contribution documentation for build, packaging, and verification

### Changed

- Broad OCIO 2.5 wrapper and bridge parity work has replaced many placeholder
  or stale APIs with real bridge-backed behavior
- README and release guidance now match the actual build gating logic for real
  OCIO mode
- Project status and migration guidance now describe `0.2.0` as a broadly
  usable early-adopter OCIO 2.5 line rather than a placeholder cleanup release

### Fixed

- Packaged bundled builds no longer depend on cloning upstream OCIO dependency
  sources for the current supported bundled configuration

## [0.1.1] - 2026-05-20

### Added

- Initial published crate line targeting OpenColorIO 2.5.2
