# ocio-rs

[![CI](https://github.com/shaloong/ocio-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/shaloong/ocio-rs/actions)
[![crates.io](https://img.shields.io/crates/v/ocio-rs)](https://crates.io/crates/ocio-rs)
[![docs.rs](https://img.shields.io/docsrs/ocio-rs)](https://docs.rs/ocio-rs)
[![license](https://img.shields.io/crates/l/ocio-rs)](LICENSE)

Rust bindings for [OpenColorIO](https://opencolorio.org/).

Wraps the full v2.5.2 public API — 38 classes, 22 transform types, 748 functions.

> [中文文档](docs/README_zh-CN.md)

```toml
[dependencies]
ocio-rs = "0.1"
```

## Build

**Stub mode** (default): compiles and tests run without an OCIO installation. APIs return safe defaults — sufficient for development and CI.

```bash
cargo build
cargo test
```

**Real OCIO mode** (three options):

```bash
# Build OCIO from the bundled submodule and link statically
git clone --recursive https://github.com/shaloong/ocio-rs
cargo build --features bundled

# Use a pre-installed OCIO
OCIO_INSTALL_DIR=/path/to/ocio cargo build

# Build from an OCIO source tree
OCIO_SOURCE_DIR=/path/to/ocio cargo build
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

| ocio-rs | OCIO   |
| ------- | ------ |
| 0.1.0   | v2.5.1 |
| 0.1.1   | v2.5.2 |

OCIO upgrade workflow: update submodule → run code generator → fix compile errors → release.

## License

[BSD-3-Clause](LICENSE).

OpenColorIO is a trademark of the Academy Software Foundation. This project is not affiliated with ASWF.
