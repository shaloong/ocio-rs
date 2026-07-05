# Releasing ocio-rs

This crate publishes in two steps:

1. Publish `ocio-sys`.
2. Publish `ocio-rs`.

The top-level crate depends on the registry version of `ocio-sys` during
`cargo package` verification, so `ocio-rs` cannot be fully verified against a
new `0.2.x` release until the matching `ocio-sys` version already exists in the
registry.

## Pre-release checks

Run these from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo test --workspace --no-default-features
cargo test --examples --no-default-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --no-default-features
cargo run --bin check_parity --quiet -- --check-l3
cargo package -p ocio-sys --allow-dirty
```

From a recursive checkout, also run:

```bash
cargo test --workspace --features bundled
cargo build --features bundled --offline
```

You can run the same repository-side audit with the helper script:

```powershell
pwsh -File tools/release_audit.ps1 -IncludeBundled -IncludeTopLevelPackage -Offline
```

The script treats the known top-level `cargo package` failure as a warning when
the registry does not yet contain `ocio-sys 0.2.0`, so it can distinguish
repository regressions from the expected publish-order blocker.

Current packaging caveat:

- The repository supports `--features bundled` from a recursive checkout.
- The published `ocio-sys` crate now vendors the upstream OpenColorIO source
  tree inside the package payload.
- The published `ocio-sys` crate also vendors the transitive dependency sources
  needed by the current bundled build configuration, and the extracted package
  is now validated with `cargo build --features bundled --offline`.

Current audit status:

- `./tools/release_audit.ps1 -IncludeBundled -Offline` passes.
- The same audit now verifies the extracted `ocio-sys` package with
  `cargo build --features bundled --offline`.
- `./tools/release_audit.ps1 -IncludeTopLevelPackage -Offline` reports only the
  expected warning that `ocio-sys 0.2.0` must exist in the registry before the
  top-level crate can be fully packaged.

The repository also exposes the same flow as manual GitHub Actions workflows:

- `CI` (workflow_dispatch bundled job)
- `Release Audit`

## Publish order

Publish the low-level crate first:

```bash
cargo publish -p ocio-sys
```

After crates.io has indexed that release, verify and publish the top-level
crate:

```bash
cargo package --allow-dirty
cargo publish
```

## Notes

- Stub mode is the default CI path and is useful for API-shape and wrapper
  checks, but it is not a substitute for bundled real-OCIO validation.
- If your local Cargo config replaces `crates-io` with a mirror, release-time
  verification depends on that mirror being reachable and in sync with the new
  `ocio-sys` release.
