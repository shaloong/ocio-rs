# ocio-rs

Rust bindings for OpenColorIO, organized as:

- `ocio-sys`: low-level C++ bridge and linking layer
- `ocio-rs` (root crate): safe Rust wrappers

## Build strategy

This workspace targets static linking of OpenColorIO and its dependency graph.

- Default feature in `ocio-sys` is `bundled`.
- In `bundled` mode, `build.rs` expects OpenColorIO source via:
  - `OCIO_SOURCE_DIR`, or
  - `third_party/OpenColorIO` (clone it yourself: `git clone --depth 1 https://github.com/AcademySoftwareFoundation/OpenColorIO.git third_party/OpenColorIO`)
- If no source/install is found, it builds in **stub mode** (compiles, but runtime OCIO calls return errors).

## Quick start

```bash
cargo check
cargo test
cargo run --example basic
```

To enable real OCIO behavior (not stub):

```bash
# PowerShell
$env:OCIO_SOURCE_DIR = "E:\path\to\OpenColorIO"
cargo test
```

Or, use a preinstalled OpenColorIO tree:

```bash
$env:OCIO_INSTALL_DIR = "E:\path\to\ocio-install"
cargo test -p ocio-rs --no-default-features
```

## Current API surface

Safe API currently wraps the main pipeline:

- `Config::raw()`
- `Config::from_file(path)`
- `Config::processor(src, dst)`
- `Processor::apply_rgba(&mut [f32; 4])`

This establishes the full FFI architecture and static-link toolchain path; more OCIO classes and transforms can now be layered in incrementally.
