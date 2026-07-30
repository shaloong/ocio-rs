# ocio-rs

[![CI](https://github.com/shaloong/ocio-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/shaloong/ocio-rs/actions)
[![crates.io](https://img.shields.io/crates/v/ocio-rs)](https://crates.io/crates/ocio-rs)
[![docs.rs](https://img.shields.io/docsrs/ocio-rs)](https://docs.rs/ocio-rs)
[![license](https://img.shields.io/crates/l/ocio-rs)](LICENSE)

Rust bindings for [OpenColorIO](https://opencolorio.org/).

This project supports OpenColorIO 2.4.1+ (OCIO 2.5.1 API is opt-in via the `v2_5`
cargo feature; see Compatibility below). The core OCIO Rust wrapper surface
is now broadly in place, including bundled real-OCIO builds, bridge parity
across the exposed API surface, and broad safe-wrapper coverage. The remaining
work is mostly release hardening and longer-tail behavioral validation rather
than missing core binding coverage.

> [中文文档](docs/README_zh-CN.md)

> Breaking API updates are tracked in [docs/MIGRATION.md](docs/MIGRATION.md).

Recent bundled validation now covers more than linkability and smoke tests for
several high-use runtime paths, including:

- `Config` multi-config/display-view behavior and virtual/shared-view metadata
- `FileRules` insertion, regex/custom-key round trips, and config attachment
- `ViewingRules` safe wrapper behavior, editable-copy independence, and config attachment
- `DynamicProperty` runtime behavior across `Processor`, `CPUProcessor`, and
  extracted GPU descriptors
- `GpuShaderDesc` extraction structure, resource metadata, creator settings,
  descriptor-side dynamic-property access, manual shader-text assembly, and
  manual texture and uniform insertion
- `CPUProcessor` packed/pixel execution paths, including stride-preserving
  behavior for `RGB(A)` buffers

```toml
[dependencies]
ocio-rs = "0.2"
```

## Build

`cargo build` uses deterministic **stub mode** by default, where APIs return
safe defaults for API-shape testing and CI but perform no real color
management. Stub builds
report their OCIO-dependent tests as ignored rather than passed, so a green run
is never mistaken for a verified one.

```bash
cargo build
cargo test
```

**Real OCIO mode**:

```bash
# Build OCIO from the bundled submodule and link statically
git clone --recursive https://github.com/shaloong/ocio-rs
cargo build --features bundled

# Build bundled OCIO as shared libraries
OCIO_RS_LINK=dynamic cargo build --features bundled

# Use a pre-installed OCIO discoverable through pkg-config
cargo build --features system

# Legacy equivalent retained for compatibility
OCIO_RS_ENABLE_REAL=1 cargo build

# Use a pre-installed OCIO from a custom prefix
OCIO_INSTALL_DIR=/path/to/ocio cargo build --features system

# Use a pre-installed shared OCIO library instead of static libs
OCIO_INSTALL_DIR=/path/to/ocio OCIO_RS_LINK=dynamic cargo build --features system

# Force the default stub backend explicitly for legacy scripts
OCIO_RS_ENABLE_REAL=0 cargo build
```

`OCIO_SOURCE_DIR` is currently only consumed by the bundled build path; setting
it by itself does not enable real OCIO mode.

Installed mode accepts OpenColorIO `>= 2.4.1, < 2.6`, raised to
`>= 2.5.1, < 2.6` by the `v2_5` feature. It first probes
`opencolorio` / `OpenColorIO` through pkg-config. `OCIO_INSTALL_DIR` prepends
the prefix's `lib/pkgconfig`, `lib64/pkgconfig`, and `share/pkgconfig`
directories; for older installations without a `.pc` file, the conventional
`include`, `lib`, and `lib64` layout remains supported. `PKG_CONFIG_PATH` can
also be set directly.

Backend features are additive, as Cargo requires for dependency unification:
no backend feature is deterministic stub mode, `system` requires an installed
library, and `bundled` adds the source-build Adapter and takes precedence when
combined with `system`. `OCIO_RS_ENABLE_REAL=0/1` remains as a legacy adapter,
but invalid values and conflicts with real-backend features fail the build.

The `bundled` feature builds the vendored source by default, preserving its
historical behavior. Set
`SYSTEM_DEPS_OPENCOLORIO_BUILD_INTERNAL=auto` to prefer a compatible installed
OpenColorIO and fall back to the bundled source only when probing fails.
Installed-library discovery requires a `pkg-config` executable. The bundled
source build describes its own install prefix directly and does not require
pkg-config, including on Windows.

`OCIO_RS_LINK` defaults to `static` for compatibility. Set it to `dynamic`
(`shared` and `dylib` are also accepted) when `OCIO_INSTALL_DIR` points at an
OpenColorIO install with shared libraries. Make sure your platform loader can
find the OCIO runtime library at execution time, for example via
`LD_LIBRARY_PATH`, `DYLD_LIBRARY_PATH`, `PATH`, or your package manager's normal
runtime setup.

The published `ocio-sys` crate now vendors the upstream OpenColorIO source tree
plus the transitive dependency sources used by the current bundled build
configuration. The packaged `ocio-sys` crate is validated with
`cargo build --features bundled --offline` during release hardening.

## Examples

```bash
# Stub-friendly API walkthrough
cargo run --example basic

# Load a real sample config and run CPU processing
cargo run --example real_config --features bundled
```

## Architecture

```text
ocio-rs/
├── ocio-sys/          C++ bridge • stub/real dual mode • auto-generated
├── src/               Safe Rust wrappers
├── tools/generator/   Code generator (from OCIO headers)
├── tests/             Integration tests
└── benches/           Benchmarks
```

## Compatibility

The baseline API works with any OpenColorIO >= 2.4.1. API added in newer OCIO
versions is opt-in through additive `vX_Y` cargo features; enabling one raises
the version requirement checked against the OpenColorIO resolved at build
time, failing with a clear error on mismatch.

| feature | OCIO API level | gates                                                        |
| ------- | -------------- | ------------------------------------------------------------ |
| (none)  | 2.4.1          | everything else                                               |
| `v2_5`  | 2.5.1          | hue-curve grading, interchange attributes, view/display list management, GPU uniform buffers and shader binding indexes |

Vendored OCIO per release:

| ocio-rs | OCIO   |
| ------- | ------ |
| 0.2.1   | v2.5.2 |
| 0.2.0   | v2.5.2 |
| 0.1.1   | v2.5.2 |
| 0.1.0   | v2.5.1 |

OCIO upgrade workflow: update submodule -> run code generator -> fix compile errors -> release.

See [STATUS.md](STATUS.md) for the current validation matrix and the remaining
release caveats for specific runtime paths.

## License

[BSD-3-Clause](LICENSE).

OpenColorIO is a trademark of the Academy Software Foundation. This project is not affiliated with ASWF.
