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
| CPU processing | Wrapped, with bundled runtime coverage for single-pixel, packed-F32, and strided RGB/RGBA paths |
| GPU shader extraction | Wrapped, with bundled runtime coverage for shader text, uniforms, textures, and descriptor configuration round trips |
| Dynamic properties | Wrapped, with bundled runtime coverage for processor/CPU dynamic-property semantics across exposure and grading controls |
| Error propagation | Available, still being expanded case by case |
| docs.rs documentation | Seeded, still expanding |
| CI real-OCIO validation | Manual bundled full test job |

The v0.2 line focuses on replacing generated stubs with real OCIO bridge
implementations, removing APIs that are not present upstream, and backing the
remaining surface with bundled and no-default-features test coverage.

Current release checklist highlights:

- Safe-wrapper parity against the C++ bridge is in place for the OCIO 2.5 API
  surface exposed by this crate.
- The parity checker currently reports clean results across all three layers:
  `1004` bridge/lib.rs declarations, `993` bridge-backed safe-wrapper matches,
  and `788/822` OCIO C++ header methods accounted for by the bridge policy.
- `cargo test --workspace --no-default-features` passes.
- `cargo test --examples --no-default-features` passes.
- `cargo test --workspace --features bundled` passes.
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
  passes.
- `cargo doc --workspace --no-deps --no-default-features` passes.
- `cargo package -p ocio-sys --allow-dirty --offline` passes.
- `cargo build --features bundled --offline` passes from the extracted
  `target/package/ocio-sys-0.2.0` package directory.
- `cargo test --workspace --features bundled` now covers the dedicated
  `baker_behavior`, `builtin_config_registry_behavior`, `config_behavior`,
  `context_behavior`, `file_rules_behavior`, `dynamic_property_behavior`,
  `gpu_shader_desc_behavior`, `cpu_processor_behavior`, and `matrix_op`
  integration suites in addition to crate unit tests.
- Bundled real-OCIO builds are validated from a recursive repository checkout.
- The published `ocio-sys` crate now vendors the upstream OpenColorIO source
  tree required by `--features bundled`.
- The published `ocio-sys` crate also vendors the transitive dependency
  sources used by the current bundled build configuration, and the extracted
  package now passes offline bundled compilation in release audit.

Latest release-audit result:

- `./tools/release_audit.ps1 -IncludeBundled -Offline` passes end to end.
- `./tools/release_audit.ps1 -IncludeTopLevelPackage -Offline` passes all
  repository-side checks and reports only the known registry sequencing warning
  for top-level `cargo package`.
- The release audit now validates the extracted `ocio-sys` package with
  `cargo build --features bundled --offline` in addition to repository builds.
- The current bundled validation path exercises `373` crate tests plus nine
  dedicated integration suites covering baker output, builtin-config registry
  enumeration, config behavior, context resolution, file rules, dynamic
  properties, GPU shader descriptors, CPU processor execution, and matrix
  processing behavior.

The GitHub Actions workflow keeps bundled validation as a manual job because it
requires a recursive checkout and a slower native OCIO build, but the manual
path executes the bundled test suite rather than stopping at `--no-run`.

Current runtime semantics worth calling out explicitly:

- `DynamicProperty` exposure values set on a `Processor` seed newly created
  `CPUProcessor` instances, while each `CPUProcessor` then owns its own runtime
  dynamic-property state.
- `Context::resolve_file_location()` uses the working directory as a fallback
  only when no explicit search paths are configured; once search paths are set,
  resolution follows those paths.
- `Baker` round-trips its configured properties in bundled mode and emits real
  LUT text; for a no-crosstalk `raw -> raw` `resolve_cube` bake, upstream OCIO
  emits a 1D LUT (`LUT_1D_SIZE`) rather than forcing a 3D cube.
- `GpuShaderDesc::clone_desc()` preserves descriptor configuration such as
  language, function name, pixel name, and resource prefix, but extracted
  shader payloads are not guaranteed to be copied into the clone.
- `CPUProcessor::apply_rgb(a)_pixels` respects caller-provided stride values and
  leaves padding lanes untouched in the bundled validation path.

Release note: `ocio-sys` must be published before `ocio-rs` for matching
versions because the top-level crate depends on the registry version of
`ocio-sys` during `cargo package` verification. At the moment, that publish
order remains the only known blocker observed by the repository audit.
