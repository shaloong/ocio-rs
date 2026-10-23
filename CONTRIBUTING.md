# Contributing

Thanks for your interest in `ocio-rs`.

This project provides Rust bindings for OpenColorIO `>= 2.4.1, < 2.6`.
The baseline API targets 2.4.1; `v2_5` enables API introduced in 2.5.1.
That means API additions are only part of the work: bridge correctness, runtime
behavior, tests, release packaging, and documentation all matter.

## Ground Rules

- Keep changes aligned with upstream OpenColorIO behavior at both supported API levels.
- Prefer real bridge implementations over Rust-side placeholders or fake return
  values.
- When an upstream OCIO API does not exist, remove or deprecate the Rust-side
  surface instead of inventing behavior.
- Keep documentation honest about stub mode, bundled mode, and any known limits.

## Development Modes

`ocio-rs` supports two main build modes:

- Stub mode: default; used for fast API-shape validation and lightweight CI.
- Real OCIO mode:
  - Installed OCIO via pkg-config: `cargo build --features system`
  - Installed OCIO under a custom prefix: `OCIO_INSTALL_DIR=/path/to/ocio cargo build --features system`
  - Bundled OCIO: `cargo build --features bundled`

The installed-library path supports OpenColorIO `>= 2.4.1, < 2.6`, raised to
`>= 2.5.1, < 2.6` by `v2_5`. Set `OCIO_RS_LINK=dynamic` for shared linking.
Without a backend feature or legacy environment override the backend is stub;
it performs no color processing. `bundled` takes precedence over `system` when
Cargo unifies both. Real backend failures never silently fall back to stub.
`OCIO_INSTALL_DIR` supports both pkg-config metadata and the legacy
`include`/`lib` layout. For bundled builds, set
`SYSTEM_DEPS_OPENCOLORIO_BUILD_INTERNAL=auto` to prefer an installed compatible
library before falling back to the vendored source.
Installed-library discovery requires `pkg-config`. Pure bundled builds do not
require it, including on Windows. `OCIO_RS_ENABLE_REAL=0/1` remains a legacy
override; invalid values or conflicts with backend features fail the build.

For release hardening, the packaged `ocio-sys` crate should also build with:

```powershell
cargo build --features bundled --offline
```

## Recommended Checks

Before opening a pull request, run the checks that match your change:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo clippy --workspace --all-targets --no-default-features --features v2_5 -- -D warnings
cargo test --workspace --no-default-features
cargo test --workspace --no-default-features --features v2_5
cargo test --examples --no-default-features
cargo run --bin check_parity --quiet -- --check-l3
```

If your change affects real OCIO behavior, also run:

```powershell
cargo test --workspace --no-default-features --features bundled,v2_5 -- --test-threads=1
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
- Prefer small, reviewable commits with English Conventional Commit messages.
- Gate new APIs consistently across safe wrappers, FFI, bridge, and tests.
- Keep English and Chinese build and migration documentation consistent.
- Update `README.md`, `STATUS.md`, or `docs/RELEASING.md` when behavior,
  packaging, or project status changes.

## Pull Requests

`main` is the default branch and release line. Start feature/fix branches from
`develop` and explicitly select **develop** as the PR base. GitHub may preselect
`main`; use the [develop comparison page](https://github.com/shaloong/ocio-rs/compare/develop...)
for ordinary contributions. Write PR descriptions and review comments in English.

Only this repository's `develop` branch may be promoted into `main`. Both
branches require PRs and passing checks. Preserve contributor history with merge
commits; do not reset or force-push either branch. After promotion, synchronize
`main` back into `develop` so `main` remains its ancestor. See
[RELEASING.md](docs/RELEASING.md) for that required release step.

Helpful pull requests usually include:

- What changed
- Why it changed
- Whether the change is stub-only, real-OCIO, or both
- What verification was run
- Any remaining gaps or follow-up work
