# Project Status

ocio-rs targets broad OCIO 2.5 coverage, but it is still a binding project
under active release hardening rather than a drop-in replacement for every C++
OpenColorIO workflow.

| Area | Status |
|---|---|
| Low-level FFI declarations | Broadly generated and linked to real bridge implementations |
| Stub mode | Available |
| Real OCIO build via installed OCIO | Available |
| Bundled OCIO build | Available, continuously validated |
| Safe Rust wrappers | Broad OCIO 2.5 coverage |
| CPU processing | Wrapped and tested |
| GPU shader extraction | Wrapped and smoke-tested |
| Dynamic properties | Wrapped |
| Error propagation | Available, still being expanded case by case |
| docs.rs documentation | Seeded, still expanding |
| CI real-OCIO validation | Manual bundled full test job |

The v0.2 line focuses on replacing generated stubs with real OCIO bridge
implementations, removing APIs that are not present upstream, and backing the
remaining surface with bundled and no-default-features test coverage.

Current release checklist highlights:

- Safe-wrapper parity against the C++ bridge is in place for the OCIO 2.5 API
  surface exposed by this crate.
- `cargo test --workspace --no-default-features` passes.
- `cargo test --workspace --features bundled` passes.
- `cargo doc --workspace --no-deps --no-default-features` passes.

The GitHub Actions workflow keeps bundled validation as a manual job because it
requires a recursive checkout and a slower native OCIO build, but the manual
path executes the bundled test suite rather than stopping at `--no-run`.

Release note: `ocio-sys` must be published before `ocio-rs` for matching
versions because the top-level crate depends on the registry version of
`ocio-sys` during `cargo package` verification.
