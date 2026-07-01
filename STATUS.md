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
| GPU shader extraction | Wrapped, with bundled runtime coverage for shader text, uniforms, textures, descriptor configuration, descriptor-side dynamic-property access, manual shader assembly, and manual texture/uniform insertion |
| Dynamic properties | Wrapped, with bundled runtime coverage for processor/CPU semantics plus GPU-descriptor property enumeration and mutation |
| Error propagation | Available, still being expanded case by case |
| docs.rs documentation | Seeded, still expanding |
| CI real-OCIO validation | Manual bundled and release-audit workflows |

The v0.2 line focuses on replacing generated stubs with real OCIO bridge
implementations, removing APIs that are not present upstream, and backing the
remaining surface with bundled and no-default-features test coverage.

Current release checklist highlights:

- Safe-wrapper parity against the C++ bridge is in place for the OCIO 2.5 API
  surface exposed by this crate.
- The parity checker currently reports clean results across all three layers:
  `1061` bridge/lib.rs declarations, `1050` bridge-backed safe-wrapper
  matches, and `822/822` OCIO C++ header methods accounted for, including
  normalized coverage for static `Create` constructor-style entry points.
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
  `allocation_transform_behavior`, `baker_behavior`,
  `builtin_config_registry_behavior`, `color_space_behavior`,
  `color_space_set_behavior`,
  `color_space_transform_behavior`,
  `cdl_transform_behavior`, `config_behavior`,
  `config_collection_behavior`, `config_core_behavior`,
  `config_io_proxy_behavior`,
  `config_runtime_settings_behavior`,
  `config_display_management_behavior`,
  `context_behavior`,
  `file_rules_behavior`, `file_transform_behavior`,
  `dynamic_property_behavior`, `gpu_shader_desc_behavior`,
  `exponent_transform_behavior`, `exponent_with_linear_transform_behavior`,
  `exposure_contrast_transform_behavior`,
  `fixed_function_transform_behavior`,
  `grading_primary_transform_behavior`,
  `grading_tone_transform_behavior`,
  `grading_rgb_curve_transform_behavior`,
  `grading_hue_curve_transform_behavior`,
  `format_metadata_behavior`,
  `group_transform_behavior`,
  `processor_metadata_behavior`,
  `processor_behavior`,
  `range_transform_behavior`,
  `runtime_helpers_behavior`,
  `cpu_processor_behavior`, `matrix_op`, `builtin_transform_behavior`,
  `builtin_transform_registry_behavior`, `display_view_transform_behavior`,
  `view_transform_behavior`, `look_behavior`, `look_transform_behavior`,
  `log_affine_transform_behavior`, `log_camera_transform_behavior`,
  `log_transform_behavior`, `lut1d_transform_behavior`,
  `lut3d_transform_behavior`, `named_transform_behavior`, and
  `viewing_rules_behavior` integration suites in addition to crate unit tests.
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
- The current bundled validation path exercises `378` crate tests plus forty-seven
  dedicated integration suites covering baker output, builtin-config registry
  enumeration, builtin-transform registry enumeration, builtin-transform
  execution, color-space metadata and processor behavior, config behavior,
  color-space-set behavior, config collection behavior, config core behavior,
  config display-management behavior, config-IO proxy behavior, config
  runtime-settings behavior, context resolution, file rules, file-transform
  behavior, dynamic properties, GPU shader descriptors, CPU processor
  execution, matrix
  processing behavior, named-transform execution, cdl-transform behavior,
  color-space-transform behavior, runtime-helper behavior,
  allocation-transform behavior,
  display-view-transform behavior, look behavior, look-transform behavior,
  exponent-transform behavior, exponent-with-linear-transform behavior,
  grading-primary-transform behavior, grading-tone-transform behavior,
  grading-rgb-curve-transform behavior, grading-hue-curve-transform behavior,
  format-metadata behavior, group-transform behavior, processor helper behavior,
  log-affine-transform behavior, log-camera-transform behavior,
  log-transform behavior, lut1d-transform behavior, lut3d-transform behavior,
  exposure-contrast-transform behavior, fixed-function-transform behavior,
  range-transform behavior, and view-transform display-pipeline behavior.
  It also now covers safe `ViewingRules` construction, mutation, editable-copy
  independence, and config attachment behavior in bundled mode, plus
  standalone and processor-extracted `ProcessorMetadata` behavior.

GitHub Actions keeps bundled validation on manual workflows because it requires
a recursive checkout and a slower native OCIO build. The manual paths now cover
both the dedicated bundled test job in `ci.yml` and the broader `Release Audit`
workflow, which also validates packaging and offline bundled compilation.

Current runtime semantics worth calling out explicitly:

- `AllocationTransform` round-trips allocation mode, vars, and direction state
  in bundled mode; `Uniform` allocation behaves as a fit from the configured
  range into `[0, 1]`, while `Lg2` performs a real `log2`-then-fit path rather
  than collapsing to a plain fit or no-op.
- `DynamicProperty` exposure values set on a `Processor` seed newly created
  `CPUProcessor` instances, while each `CPUProcessor` then owns its own runtime
  dynamic-property state.
- `Context::resolve_file_location()` uses the working directory as a fallback
  only when no explicit search paths are configured; once search paths are set,
  resolution follows those paths.
- `Config` now has bundled runtime coverage for collection mutation: added
  color spaces, looks, named transforms, and view transforms become visible
  through the corresponding lookup/count APIs, display-view definitions mark
  referenced color spaces as used, `remove_*` clears object lookup state, and
  `clear_all()` empties the tracked collection counts even though
  `display(0)` currently returns an empty-string sentinel once the display list
  is empty.
- `Config` runtime-setting helpers now have bundled coverage too: active
  display/view strings round-trip through both aggregate and indexed accessors,
  environment-variable metadata round-trips without guaranteeing insertion
  order, and the current OCIO default processor-cache flags evaluate to
  `ENABLED | SHARE_DYN_PROPERTIES` rather than just `ENABLED`. The crate-level
  `current_config()` / `set_current_config()` / `processor_cache_flags()`
  helpers follow the installed config's real runtime state.
- `Config` core loading and metadata helpers now have bundled coverage:
  `from_file`, `from_env`, and `from_stream` all load the same `context_test1`
  schema/display/role metadata; config search-path aggregation currently joins
  entries with `:` on this runtime; strict-parsing and default-luma settings
  round-trip; config cache IDs change after metadata/search-path mutation; and
  serialized YAML reflects the authored name, description, roles, and search
  paths.
- `Config` display-management helpers now have bundled lifecycle coverage:
  shared views attach to displays, survive re-attachment, and disappear from
  display lookup when either the display-view link or the shared view itself is
  removed; `clear_shared_views()` clears those display-facing references, and
  `clear_displays()` empties display counts. For virtual displays, name-based
  metadata lookup may succeed even when the same view is not returned by
  `virtual_display_view(...)` enumeration for a given reference-space filter.
- The top-level runtime helpers now have bundled coverage: `version()` reports
  a real `2.5.x` OCIO runtime string, logging level changes round-trip through
  the global getter/setter pair, and `Config::raw()` currently starts at schema
  version `2.0`; mutating version fields round-trips, while
  `upgrade_to_latest_version()` does not promote that raw config's minor
  version to `2.5`.
- `ConfigIOProxy` round-trips embedded config text and LUT payloads in bundled
  mode, attached proxy objects remain visible through both `Config` and
  `Context`, missing LUT keys currently surface as empty payload views, and a
  config created from proxy-backed assets follows real OCIO path resolution:
  search paths are consulted before a working-directory fallback, including
  context-variable-expanded entries such as `./$SHOT`.
- `FileRules` now has bundled runtime coverage beyond metadata round-trips:
  attached rules on a `Config` actively drive
  `color_space_from_filepath_with_rule_index(...)`, including preserving the
  matched rule index for ordered filename-pattern rules and falling back to the
  configured default-rule color space when no explicit rule matches.
- `ViewingRules` are now exposed as a first-class safe Rust wrapper rather than
  only raw pointer plumbing on `Config`; bundled coverage validates rule
  insertion, color-space and encoding selectors, custom key/value mutation,
  editable-copy independence, and round-trip config attachment.
- `Baker` round-trips its configured properties in bundled mode and emits real
  LUT text; for a no-crosstalk `raw -> raw` `resolve_cube` bake, upstream OCIO
  emits a 1D LUT (`LUT_1D_SIZE`) rather than forcing a 3D cube.
- `NamedTransform` round-trips aliases, categories, and attached forward /
  inverse transforms in bundled mode, and both object-based and name-based
  processor creation execute the expected transform direction.
- `ColorSpace` round-trips aliases, categories, interchange metadata, and
  attached transforms in bundled mode; when validating processors across
  custom color spaces, distinct `equality_group` values matter because equal
  groups may let OCIO optimize the conversion path away.
- `ColorSpaceSet` now has bundled runtime coverage for both manual and
  config-derived sets: editable copies mutate independently, `add/remove`
  set-style operations preserve membership as expected, and
  `Config::color_space_set(Some(category))` follows real OCIO category
  filtering rather than name or substring matching.
- `ColorSpaceTransform` round-trips src/dst strings and copy state in bundled
  mode, matches named color-space processors for the same config path, and
  `data_bypass=true` preserves RGB values for data-tagged source spaces where
  the forced path would otherwise apply the configured conversion.
- `CDLTransform` round-trips SOP, saturation, style, ID, and SOP description
  state in bundled mode, `CreateFromFile` / `CreateGroupFromFile` load real
  `.ccc` data from disk, and a no-clamp unit-saturation CDL executes as a real
  forward/inverse CPU processing pair rather than a no-op.
- `FileTransform` round-trips source path, CCC ID, interpolation, CDL style,
  and direction state in bundled mode; editable copies keep those properties
  independent, CLF execution follows the file's real bit-depth-normalized OCIO
  semantics instead of behaving like a bare float matrix, and `.ccc` loading
  honors both `ccc_id` selection and the transform's default direction when
  building processors.
- `ExponentTransform` round-trips exponent values, negative-style, and
  direction state in bundled mode, and a `[2, 2, 2, 1]` exponent executes as
  a real forward/inverse CPU processing pair for positive-domain RGB values;
  the current OCIO CPU path is numerically close to alpha-identity rather than
  bit-exact on the alpha channel.
- `ExponentWithLinearTransform` round-trips gamma, offset, negative-style, and
  direction state in bundled mode, and for positive-domain inputs above the
  linear breakpoint, a `[2, 2, 2, 1]` gamma with `[0.1, 0.1, 0.1, 0]` offset
  executes the expected moncurve forward/inverse CPU processing pair rather
  than collapsing to a plain exponent or no-op path.
- `GradingPrimaryTransform` round-trips style, value, dynamic, and direction
  state in bundled mode; changing the style resets the grading payload to that
  style's upstream defaults, the default clamp sentinels stay at OCIO's real
  no-clamp `+/-f64::MAX` values, and a linear grading setup with identity
  contrast/saturation applies the expected offset-plus-`2^exposure`
  forward/inverse CPU behavior.
- `GradingToneTransform` round-trips style, value, dynamic, and direction
  state in bundled mode; changing the style resets the tone payload to that
  style's upstream defaults, including the real `Log/Video` `shadows.start`
  default of `0.6` rather than the earlier Rust helper's stale `0.5`.
- `GradingRGBCurveTransform` round-trips edited control points, slopes,
  dynamic state, and direction in bundled mode; changing the style resets the
  curve payload to the new style's defaults, while `bypass_lin_to_log`
  remains independent state rather than being cleared by `set_style(...)`.
- `GradingHueCurveTransform` round-trips supported hue-curve edits, HSY mode,
  dynamic state, and direction in bundled mode; some curve families normalize
  control-point layout on write, so validation follows OCIO's real constrained
  curve semantics instead of assuming every authored point is a free-form
  round-trip value.
- `FormatMetadata` now has bundled runtime coverage on real Baker and Processor
  metadata roots: the top-level element remains the reserved `ROOT` node, root
  element-value writes do not behave like a free-form payload field, child
  elements and named attributes round-trip, `name` / `id` participate in the
  richer metadata view, and `clear()` removes child/attribute payload while
  leaving the root node itself intact.
- `GroupTransform` now has bundled runtime coverage for child ordering,
  editable-copy independence, CLF serialization, and mutation semantics:
  `prepend` and `append` produce distinct processor results, and bridge-side
  `remove_transform` / `clear_transforms` preserve the group's direction and
  format metadata instead of silently resetting them.
- `Processor`, `CPUProcessor`, and `GPUProcessor` now have bundled runtime
  coverage for non-no-op matrix pipelines: default and optimized CPU helpers
  produce the same scaled RGBA output, default and optimized GPU helpers both
  emit non-empty shader text, and `Processor::create_group_transform()` can be
  round-tripped back into an equivalent processor path. The deprecated legacy
  GPU helper also emits real shader text in bundled mode, even when the
  extracted descriptor does not expose additional uniform or texture resources.
- `GpuShaderDesc` now has bundled runtime coverage for inherited
  `GpuShaderCreator` settings such as unique IDs, descriptor-set binding
  offsets, 1D-texture preferences, extracted dynamic-property access, resource
  index allocation, manual shader-text assembly through the section-based
  `add_to_*` helpers plus `create_shader_text(...)`, and manual 1D/2D/3D
  texture insertion with OCIO-reported binding indices and payload round-trips.
  Manual scalar, bool, float3, and vector uniform insertion is also covered,
  including duplicate-name rejection and typed payload round-trips. In real
  OCIO builds, descriptor-side dynamic properties remain mutable after
  extraction but do not currently alias the source `Processor` property object.
- `ProcessorMetadata` is now modeled as its own safe Rust wrapper instead of
  being conflated with `FormatMetadata`; bundled coverage validates standalone
  file/look mutation plus metadata extraction from a real processor.
- `LogTransform` round-trips base and direction state in bundled mode, uses
  the documented default base of `2.0`, executes the expected `log(color,
  base)` CPU path on positive inputs, and leaves alpha numerically unchanged
  within the current OCIO floating-point tolerance.
- `LogAffineTransform` round-trips base plus log-side / lin-side slope and
  offset state in bundled mode, and with custom affine parameters executes the
  documented `logSideSlope * log(linSideSlope * color + linSideOffset, base) +
  logSideOffset` CPU path rather than behaving like a plain log or no-op.
- `LogCameraTransform` round-trips base, lin-side break, and optional
  `linearSlope` state in bundled mode; with an explicit slope it exercises both
  the near-black linear segment and the log segment in one processor path,
  instead of degenerating to a plain log-affine curve.
- `Lut1DTransform` round-trips LUT length, values, interpolation, bit depth,
  and direction state in bundled mode; a simple monotonic 2-point LUT executes
  as a real forward/inverse CPU mapping rather than behaving like an identity
  table or shape-only metadata object.
- `Lut3DTransform` round-trips grid size, values, interpolation, bit depth,
  and direction state in bundled mode; a simple separable `2x2x2` LUT executes
  as a real forward/inverse CPU mapping inside the LUT cube, and the corner
  index ordering is validated against the bridge flattening order.
- `BuiltinTransformRegistry` and `BuiltinTransform` helper enumeration stay
  coherent in bundled mode, and builtin descriptions may legitimately be empty
  strings for some upstream styles rather than guaranteed human-readable text.
- `BuiltinTransform` instances round-trip style and description metadata in
  bundled mode, editable copies keep independent direction state, and the
  `ACEScct_to_ACES2065-1` builtin style executes as a real forward/inverse CPU
  processing pair rather than a no-op.
- `ViewTransform` round-trips categories, interchange metadata, and attached
  transforms in bundled mode, and when driven through a `DisplayViewTransform`
  display pipeline, `TransformDirection::Forward` uses the view transform's
  `FromReference` branch for scene-to-display processing.
- `DisplayViewTransform` round-trips its display/view/source metadata and copy
  state in bundled mode, matches `Config::processor_display(...)` for the same
  display pipeline, and `data_bypass=true` preserves RGB values for data color
  spaces where the forced pipeline path would otherwise apply the view
  transform.
- `Look` round-trips metadata, interchange attributes, and attached
  forward/inverse transforms in bundled mode, and display pipelines using a
  named look apply the look through both `Config::processor_display(...)` and
  `DisplayViewTransform`, while `looks_bypass=true` suppresses that look
  application.
- `LookTransform` round-trips src/dst/look strings and copy state in bundled
  mode, and `skip_color_space_conversion=true` really changes execution
  semantics: in the current validation path it applies the look directly in
  source space instead of after the configured source/process color-space
  conversions.
- `ExposureContrastTransform` round-trips its fixed parameters and dynamic
  toggles in bundled mode, and a linear exposure of `1.0` behaves as a real
  one-stop gain (`x2`) in forward processing with the inverse processor
  restoring the input. Its `equals` helper should not currently be treated as a
  normal value-semantic equality contract.
- `RangeTransform` round-trips its range endpoints and unset/has flags in
  bundled mode. In the current validation path it remaps and clamps RGB lanes
  as expected while leaving alpha unchanged, and inverse processing restores
  the forward-mapped RGB values for the configured no-clamp case.
- `FixedFunctionTransform` round-trips style/parameter state and editable-copy
  independence in bundled mode. The current validation path confirms
  `RgbToHsv` executes as a real forward/inverse processor pair, and callers
  should not assume `set_style(...)` clears previously stored parameter slots.
- `GpuShaderDesc::clone_desc()` preserves descriptor configuration such as
  language, function name, pixel name, resource prefix, and descriptor-set
  settings, but lower-level 1D texture knobs may fall back to OCIO defaults and
  extracted shader payloads are not guaranteed to be copied into the clone.
- `CPUProcessor::apply_rgb(a)_pixels` respects caller-provided stride values and
  leaves padding lanes untouched in the bundled validation path.

Release note: `ocio-sys` must be published before `ocio-rs` for matching
versions because the top-level crate depends on the registry version of
`ocio-sys` during `cargo package` verification. At the moment, that publish
order remains the only known blocker observed by the repository audit.
