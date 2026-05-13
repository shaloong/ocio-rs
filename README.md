# ocio-rs

[![CI](https://github.com/shaloong/ocio-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/shaloong/ocio-rs/actions)
[![crates.io](https://img.shields.io/crates/v/ocio-rs)](https://crates.io/crates/ocio-rs)
[![docs.rs](https://img.shields.io/docsrs/ocio-rs)](https://docs.rs/ocio-rs)
[![license](https://img.shields.io/crates/l/ocio-rs)](LICENSE)

Rust bindings for [OpenColorIO](https://opencolorio.org/) v2.5.1.

> [中文文档](docs/README_zh-CN.md)

```toml
[dependencies]
ocio-rs = "0.1"
```

## Build

Default **stub mode**: no OCIO required. All APIs return safe defaults.

```bash
cargo build
cargo test
```

For production binaries that need real color processing, clone with the OCIO submodule and enable the `bundled` feature:

```bash
git clone --recursive https://github.com/shaloong/ocio-rs
cd ocio-rs
cargo build --features bundled   # OCIO is statically linked into your binary
```

Or use a pre-installed OCIO:

```bash
OCIO_INSTALL_DIR=/path/to/ocio cargo build
```

> `cargo add ocio-rs` from crates.io gives you **stub mode** — perfect for development and CI.
> For release binaries, clone the repo and build with `--features bundled` to statically link real OCIO.

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
| 0.1.x   | v2.5.1 |

Upgrading OCIO versions: update submodule → run code generator → fix diffs → release.

## License

[BSD-3-Clause](LICENSE), matching upstream OpenColorIO.

OpenColorIO is a trademark of the Academy Software Foundation. This project is not affiliated with ASWF.
