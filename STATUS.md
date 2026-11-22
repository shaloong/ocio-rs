# Project Status

Version 0.4 targets OpenColorIO `>= 2.4.1, < 2.7`. The baseline Rust
interface supports 2.4.1; `v2_5` requires 2.5.1 and `v2_6` requires 2.6.0.
The default backend probes the system library and otherwise builds the bundled
source. The explicit test-only `stub` backend performs no color processing.
The vendored source is 2.6.0.
See the [README](README.md) for configuration and GPU LUT limits.

Core OCIO classes, transforms, processing, GPU resources and app helpers have
native bridge paths. Some operations are exposed through `ocio-sys` or unsafe
native interop rather than a duplicate high-level wrapper. Coverage and passing
tests do not guarantee every input, third-party native subclass, driver or
production environment. Consult the [migration notes](docs/MIGRATION.md) for
breaking changes and synchronization requirements.

Processor and shader handles are thread-confined; a non-dynamic CPU processor
can be validated and converted to `StaticCPUProcessor` for shared use. Some
legacy infallible getters return fallback values on native errors; prefer
available fallible `try_*` methods where errors matter.

## Continuous integration

| Coverage | Trigger |
| --- | --- |
| Stub tests on Linux x64/ARM64, Windows x64 and macOS ARM64; Ubuntu format, lint, documentation, parity and packaging checks | Pushes and pull requests |
| Installed OpenColorIO 2.4.x on Fedora | Pushes and pull requests |
| Representative native OpenColorIO 2.6 behavior | Pull requests |
| Bundled build with system zlib-ng on Arch | Pull requests, scheduled and manual runs |
| Bundled builds on Linux x64/ARM64, macOS Intel/Apple Silicon and Windows x64/ARM64; exact 2.4.1 and 2.5.1 minima; full 2.6 interface and installed-library checks | Scheduled and manual runs |
| Release packaging and offline bundled audit | Manual |
| Stub bridge AddressSanitizer | Manual |

These are CI targets, not a claim that every platform, hardware device or
compiler supported by upstream has been validated. Stub tests cannot establish
native behavior. The parity checker compares names
and does not prove signatures or runtime behavior. For a release candidate,
record the exact commit and successful checks; follow
[release guidance](docs/RELEASING.md) for the full validation and publishing
procedure.
