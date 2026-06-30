# Contributing

Thanks for helping improve ocio-rs.

This project wraps a large C++ API, so changes should prefer real OpenColorIO
behavior over compatibility stubs. When adding or changing an API, please:

- Check the OpenColorIO 2.5 headers in `third_party/OpenColorIO/include`.
- Keep Rust wrappers small and explicit about ownership and nullability.
- Add tests for wrapper behavior when the change touches public API.
- Run `cargo fmt --all`, `cargo clippy --workspace --all-targets --no-default-features`, and `cargo test --workspace --no-default-features`.
- For real OCIO bridge work, also run `cargo test --workspace --features bundled --no-run` from a recursive checkout.
- For releases, publish `ocio-sys` before `ocio-rs`; the top-level crate cannot be fully packaged until the matching `ocio-sys` version exists in the registry.

Generated-code updates should be kept separate from manual bridge fixes when
possible so reviews can distinguish mechanical churn from API decisions.
