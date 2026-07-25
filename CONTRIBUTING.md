# Contributing

Thanks for your interest in `ocio-rs`.

This project aims to provide broad, accurate Rust bindings for OpenColorIO 2.5.
That means API additions are only part of the work: bridge correctness, runtime
behavior, tests, release packaging, and documentation all matter.

## Ground Rules

- Keep changes aligned with upstream OpenColorIO 2.5 behavior.
- Prefer real bridge implementations over Rust-side placeholders or fake return
  values.
- When an upstream OCIO API does not exist, remove or deprecate the Rust-side
  surface instead of inventing behavior.
- Keep documentation honest about stub mode, bundled mode, and any known limits.

## Development Modes

`ocio-rs` supports two main build modes:

- Stub mode: default; used for fast API-shape validation and lightweight CI.
- Real OCIO mode:
  - Installed OCIO via pkg-config: `OCIO_RS_ENABLE_REAL=1 cargo build`
  - Installed OCIO under a custom prefix: `OCIO_RS_ENABLE_REAL=1 OCIO_INSTALL_DIR=/path/to/ocio cargo build`
  - Bundled OCIO: `cargo build --features bundled`

The installed-library path supports OpenColorIO `>= 2.5.2, < 2.6`.
`OCIO_INSTALL_DIR` supports both pkg-config metadata and the legacy
`include`/`lib` layout. For bundled builds, set
`SYSTEM_DEPS_OPENCOLORIO_BUILD_INTERNAL=auto` to prefer an installed compatible
library before falling back to the vendored source.
Pkg-config-based and bundled builds require a `pkg-config` executable; Windows
contributors can use `pkgconfiglite`, matching CI.

For release hardening, the packaged `ocio-sys` crate should also build with:

```powershell
cargo build --features bundled --offline
```

## Recommended Checks

Before opening a pull request, run the checks that match your change:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo test --workspace --no-default-features
cargo test --examples --no-default-features
cargo run --bin check_parity --quiet
```

If your change affects real OCIO behavior, also run:

```powershell
cargo test --workspace --features bundled --quiet
```

If your change affects packaging or vendored bundled sources, also validate:

```powershell
cargo package -p ocio-sys --allow-dirty --offline
```

## Coding Expectations

- Match existing naming and wrapper patterns unless there is a clear reason not
  to.
- Add focused behavioral tests for real OCIO changes when practical.
- Add build-resolution regressions to `ocio-sys/tests/build_configuration.rs`
  when changing build-script or `system-deps` behavior.
- Prefer small, reviewable commits with conventional commit messages.
- Update `README.md`, `STATUS.md`, or `docs/RELEASING.md` when behavior,
  packaging, or project status changes.

## Pull Requests

Helpful pull requests usually include:

- What changed
- Why it changed
- Whether the change is stub-only, real-OCIO, or both
- What verification was run
- Any remaining gaps or follow-up work
