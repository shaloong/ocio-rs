# Changelog

All notable changes to `ocio-rs` will be documented in this file.

The format is based on Keep a Changelog, and this project follows Semantic
Versioning as closely as practical for a still-maturing binding crate.

## [Unreleased]

### Fixed

- CPU pixel-buffer helpers now reject short/default strides, C++ `ptrdiff_t`
  byte-stride overflows, and misaligned multi-byte packed slices before
  passing Rust-owned data to OCIO
- Config display, view, role, virtual-display, and active-list queries now
  expose fallible `try_*` variants that preserve bridge failures; transform
  getters and unknown future transform type tags also avoid silently losing
  bridge errors or owned handles
- Config lookup/default/file-rule queries and `FileRules` string getters now
  expose fallible `try_*` variants, so invalid indices and C++ bridge failures
  are distinguishable from ordinary missing values
- `Context` cache, path, string-variable, and file-resolution queries now
  preserve C++ exceptions through fallible `try_*` APIs
- `ColorSpaceSet` name and object lookups now distinguish bridge failures from
  absent color spaces through fallible `try_*` APIs
- Config metadata and cache-id queries now preserve bridge failures through
  fallible `try_*` APIs, including context-specialized cache identifiers
- Built-in config registry name, YAML, and by-name config queries now preserve
  bridge failures through fallible `try_*` APIs
- Built-in transform registry style and description queries now preserve bridge
  failures through fallible `try_*` APIs
- `ConfigIOProxy` config-text and fast-LUT-hash queries now preserve bridge
  failures through fallible `try_*` APIs
- `ViewingRules` string queries now preserve invalid-index and bridge failures
  through fallible `try_*` APIs
- Owned-handle creation and copy paths for Config, FileRules, ViewingRules,
  ColorSpace, and Look now retain C++ exceptions for their Rust `Result` APIs
- Processor CPU/GPU optimization paths plus ViewTransform and Baker handle
  creation now retain C++ exceptions for their Rust `Result` APIs
- Matrix transform factory helpers now retain C++ exceptions for their Rust
  `Result` APIs
- `GroupTransform::try_transform` now distinguishes OCIO index errors from a
  missing transform while preserving compatibility with `transform`

## [0.2.0] - 2026-07-02

### Added

- Broad OCIO 2.5 safe-wrapper and bridge parity, covering `Config`,
  `Context`, `Processor`, `CPUProcessor`, `GPUProcessor`, `GpuShaderDesc`,
  `DynamicProperty`, `FormatMetadata`, `ProcessorMetadata`, `ViewingRules`,
  `Baker`, `GroupTransform`, `NamedTransform`, `ColorSpace`, `ColorSpaceSet`,
  and the full OCIO 2.5 transform family with real bridge-backed behavior
- Bundled runtime validation for `DynamicProperty` interactions across
  `Processor` and `CPUProcessor`, including per-`CPUProcessor` dynamic state
  isolation and CPU pixel execution for exposure, grading, and matrix paths
- Bundled runtime validation for `GpuShaderDesc` extraction: shader text,
  uniforms, textures, descriptor configuration, dynamic-property access,
  manual shader-text assembly, and manual texture/uniform insertion
- Bundled runtime validation for `Config` collection mutation, display-view
  management, runtime settings, `ConfigIOProxy`, `FileRules`, and
  `ViewingRules` safe wrapper behavior
- Bundled runtime validation for `Baker` output, `CDLTransform`,
  `FileTransform`, `ExponentTransform`, `GradingPrimaryTransform`,
  `GradingToneTransform`, `GradingRGBCurveTransform`,
  `GradingHueCurveTransform`, `FixedFunctionTransform`, `LogTransform`,
  `LogAffineTransform`, `LogCameraTransform`, `Lut1DTransform`,
  `Lut3DTransform`, `RangeTransform`, `ExposureContrastTransform`,
  `ViewTransform`, `DisplayViewTransform`, `Look`, and `LookTransform`
- Real bundled-package validation for `ocio-sys`, including offline packaged
  bundled builds that exercise `cargo build --features bundled --offline`
  from the extracted package directory
- Parity checker reporting `822/822` OCIO C++ header methods bridged with
  `1066` bridge.hpp functions, `1067` `lib.rs` declarations, and `1050`
  bridge-backed safe-wrapper matches
- Multi-platform stub CI coverage across Linux, macOS, and Windows for
  `--no-default-features` test and example execution
- Release and contribution documentation for build, packaging, and
  verification workflows

### Changed

- API surface now targets real OCIO 2.5 behavior: stubs have been replaced
  with bridge-backed implementations across the exposed wrapper layer
- Packaged `ocio-sys` crate vendors the upstream OpenColorIO source tree
  plus transitive dependency sources, enabling offline bundled builds from
  the published package
- README and release guidance now match the actual build gating logic for
  real OCIO mode (`OCIO_RS_ENABLE_REAL=1` or `--features bundled`)
- Project status and migration guidance describe `0.2.0` as a broadly
  usable early-adopter OCIO 2.5 line rather than a placeholder cleanup
  release
- Release-readiness guidance reflects a clean parity report and green
  fmt/clippy/stub/bundled/doc verification

### Fixed

- Packaged bundled builds no longer depend on cloning upstream OCIO
  dependency sources for the current supported bundled configuration

## [0.1.1] - 2026-05-20

### Added

- Initial published crate line targeting OpenColorIO 2.5.2
