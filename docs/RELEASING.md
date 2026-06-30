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
cargo doc --workspace --no-deps --no-default-features
cargo package -p ocio-sys --allow-dirty
```

From a recursive checkout, also run:

```bash
cargo test --workspace --features bundled
```

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
