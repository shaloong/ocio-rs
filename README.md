# ocio-rs

[![CI](https://github.com/shaloong/ocio-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/shaloong/ocio-rs/actions)
[![crates.io](https://img.shields.io/crates/v/ocio-rs)](https://crates.io/crates/ocio-rs)
[![docs.rs](https://img.shields.io/docsrs/ocio-rs)](https://docs.rs/ocio-rs)
[![license](https://img.shields.io/crates/l/ocio-rs)](LICENSE)

Rust bindings for [OpenColorIO](https://opencolorio.org/).

This project targets OpenColorIO v2.5.2. The OCIO 2.5 wrapper surface is now
largely in place, including bundled real-OCIO builds and broad safe-wrapper
coverage, while release hardening and long-tail behavioral validation are still
ongoing.

Recent bundled validation now covers more than linkability and smoke tests for
several high-use runtime paths, including:

- `Config` multi-config/display-view behavior and virtual/shared-view metadata
- `FileRules` insertion, regex/custom-key round trips, and config attachment
- `ViewingRules` safe wrapper behavior, editable-copy independence, and config attachment
- `DynamicProperty` runtime behavior across `Processor`, `CPUProcessor`, and
  extracted GPU descriptors
- `GpuShaderDesc` extraction structure, resource metadata, creator settings,
  descriptor-side dynamic-property access, and manual shader-text assembly
- `CPUProcessor` packed/pixel execution paths, including stride-preserving
  behavior for `RGB(A)` buffers

> [中文文档](docs/README_zh-CN.md)

> Breaking API updates are tracked in [docs/MIGRATION.md](docs/MIGRATION.md).

```toml
[dependencies]
ocio-rs = "0.2"
```

## Build

**Stub mode** (default): compiles and tests run without an OCIO installation.
APIs return safe defaults for API-shape testing and CI, but do not perform real
color management.

```bash
cargo build
cargo test
```

**Real OCIO mode**:

```bash
# Build OCIO from the bundled submodule and link statically
git clone --recursive https://github.com/shaloong/ocio-rs
cargo build --features bundled

# Use a pre-installed OCIO
OCIO_RS_ENABLE_REAL=1 OCIO_INSTALL_DIR=/path/to/ocio cargo build
```

`OCIO_SOURCE_DIR` is currently only consumed by the bundled build path; setting
it by itself does not enable real OCIO mode.

The published `ocio-sys` crate now vendors the upstream OpenColorIO source tree
plus the transitive dependency sources used by the current bundled build
configuration. The packaged `ocio-sys` crate is validated with
`cargo build --features bundled --offline` during release hardening.

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

| ocio-rs | OCIO   |
| ------- | ------ |
| 0.2.0   | v2.5.2 |
| 0.1.1   | v2.5.2 |
| 0.1.0   | v2.5.1 |

OCIO upgrade workflow: update submodule -> run code generator -> fix compile errors -> release.

See [STATUS.md](STATUS.md) for the current validation matrix and remaining
release caveats before relying on a particular API area in production.

## License

[BSD-3-Clause](LICENSE).

OpenColorIO is a trademark of the Academy Software Foundation. This project is not affiliated with ASWF.
