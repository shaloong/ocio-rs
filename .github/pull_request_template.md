## Summary

- 

## Verification

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- [ ] `cargo test --workspace --no-default-features`
- [ ] `cargo test --examples --no-default-features`
- [ ] `cargo run --bin check_parity --quiet`
- [ ] `cargo test --workspace --features bundled --quiet` (if real-OCIO behavior changed)
- [ ] `cargo package -p ocio-sys --allow-dirty --offline` (if packaging or vendored bundled sources changed)

## Scope

- [ ] Stub mode only
- [ ] Real installed OCIO
- [ ] Bundled OCIO
- [ ] Packaging / release docs

## Notes

- 
