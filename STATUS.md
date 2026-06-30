# Project Status

ocio-rs is experimental and is not yet a complete production replacement for
the C++ OpenColorIO API.

| Area | Status |
|---|---|
| Low-level FFI declarations | Generated / in progress |
| Stub mode | Available |
| Real OCIO build via installed OCIO | In progress |
| Bundled OCIO build | Experimental |
| Safe Rust wrappers | Partial |
| CPU processing | Partial |
| GPU shader extraction | Partial |
| Dynamic properties | Partial |
| Error propagation | In progress |
| docs.rs documentation | In progress |
| CI real-OCIO validation | Manual bundled job |

The v0.2 line is focused on replacing generated stubs with real OCIO bridge
implementations, removing APIs that are not present upstream, and adding tests
that exercise real CPU/GPU-facing behavior.

Release note: `ocio-sys` must be published before `ocio-rs` for matching
versions because the top-level crate depends on the registry version of
`ocio-sys` during `cargo package` verification.
