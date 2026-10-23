## Summary

<!-- main remains the default/release branch. Explicitly select develop for normal changes. Only develop-to-main release promotions target main. -->

- 

## Verification

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- [ ] `cargo clippy --workspace --all-targets --no-default-features --features v2_5 -- -D warnings`
- [ ] `cargo test --workspace --no-default-features`
- [ ] `cargo test --workspace --no-default-features --features v2_5`
- [ ] `cargo test --examples --no-default-features`
- [ ] `cargo run --bin check_parity --quiet -- --check-l3`
- [ ] `cargo test --workspace --no-default-features --features bundled,v2_5 -- --test-threads=1` (if real-OCIO behavior changed)
- [ ] Release audit of packaged sources (if packaging or vendored bundled sources changed)

## Scope

- [ ] Stub mode only
- [ ] Real installed OCIO
- [ ] Bundled OCIO
- [ ] Packaging / release docs

## Notes

- 
