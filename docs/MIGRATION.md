# Migration Notes

## 0.2.0

`0.2.0` is an alpha-stage breaking cleanup that aligns the safe Rust layer with OpenColorIO 2.5.2 rather than preserving previously generated compatibility shims.

### GPU shader descriptors

`GpuShaderDesc` now exposes typed GPU resources:

- Use `textures_2d()` / `texture_2d(index)` for 1D/2D texture metadata and values.
- Use `textures_3d()` / `texture_3d(index)` for 3D texture metadata and values.
- Use `uniforms()` / `uniform(index)` for uniform metadata and current values.
- `texture_values(index)` now returns an owned `Vec<f32>`.
- `num_uniforms()`, `num_textures()`, `num_3d_textures()`, and binding-index APIs return concrete Rust integer types instead of pointer-shaped compatibility values.

The old pointer-shaped GPU descriptor methods were not reliable in real OCIO mode and should not be used by application code.

### View transforms

`ViewTransform` is now modeled as OpenColorIO defines it:

- Create it with `ViewTransform::create(ReferenceSpaceType)`.
- Read/write the transform in a specific `ViewTransformDirection` with `transform(direction)` and `set_transform(transform, direction)`.
- Use category APIs: `has_category`, `add_category`, `remove_category`, `num_categories`, `category(index)`, and `clear_categories`.

Display/view/looks/rule helpers were removed from `ViewTransform`. Use `DisplayViewTransform` for display/view mappings.

### Config processors and file rules

The high-level `Config::processor`, `Config::processor_with_context`, and `Config::processor_from_configs` wrappers now call the real OCIO processor APIs.

`Config::serialize()` and `Config::archive()` now return the real OCIO text output when the crate is linked against a real OCIO build. In stub mode they return `None`.

`color_space_from_filepath_by_ref_type` has been replaced by:

```rust
let (color_space, rule_index) = config
    .color_space_from_filepath_with_rule_index(path)
    .expect("file rules should resolve a color space");
```

This matches OCIO's file-rule model and returns the matching rule index alongside the resolved color-space name.

### Grading and LUT transforms

`GradingPrimaryTransform::create(style)` and `GradingToneTransform::create(style)` now construct real OCIO transforms with the requested style. `value()` and `set_value()` copy real OCIO grading fields instead of returning default zero data.

`GradingRGBCurveTransform::create(style)` and `GradingHueCurveTransform::create(style)` now construct real OCIO transforms. Hue curves now use `HueCurveType` and `HSYTransformStyle`; they no longer reuse `RGBCurveType` or expose the non-OCIO `bypass_lin_to_log` helper. The `DynamicProperty::grading_hue_curve_*` methods use `HueCurveType` for the same reason.

`Lut1DTransform::set_length` and `Lut3DTransform::set_grid_size` now pass concrete integer values through the C ABI.

### Fixed functions and log camera

`FixedFunctionStyle` now matches the OpenColorIO 2.5.2 enum values exactly. Existing code that persisted integer enum discriminants must migrate those values instead of reusing old integer data.

`FixedFunctionTransform::create_with_params` now follows OCIO validation. Pass parameters only to styles that accept them, for example `FixedFunctionStyle::Rec2100Surround` with one gamma parameter.

`LogCameraTransform::create(lin_side_break_values)` now passes the break values into `LogCameraTransform::Create`; the argument is no longer ignored.

### Built-in configs

`BuiltinConfigRegistry::config_by_name` and `config_by_index` now return an actual `Config` created via `Config::CreateFromBuiltinConfig`.

Use `config_yaml_by_name` or `config_yaml_by_index` when you need the raw built-in YAML text.

### Baker output

`Baker::bake()` now truly treats its argument as a filesystem path in Rust space. The OCIO stream output is collected with `Baker::bake_to_string()` first and then written to disk by Rust.

The older ABI wiring passed a path pointer into an `ostream*` slot, which was not reliable in real OCIO mode.

### Group transform writing

`GroupTransform` now has a safe `write_to_string(&Config, format_name)` helper for OCIO's serialized transform output.

The old low-level `write(...)` entry point remains `unsafe` for callers that need to provide raw ABI objects directly.

### Grading curve values

`GradingRGBCurveTransform` and `GradingHueCurveTransform` now expose safe Rust value snapshots:

- Use `value()` to read the curve data into Rust structs.
- Use `set_value(&...)` to replace the curve data from Rust structs.

The old raw handle accessors are now explicitly named as raw/deprecated escape hatches instead of looking like the primary API.

### Raw ABI escape hatches

Several APIs that still require OCIO-owned external objects remain available, but are now explicitly marked as deprecated/raw escape hatches in the Rust layer instead of looking like ordinary safe wrappers.

This includes viewing-rules handles, config-IO proxy handles, legacy context-var pointer overloads, raw CPU image-descriptor entry points, and raw GPU shader-creator extraction.

Additional compatibility aliases such as `GradingPrimaryTransform::copy_value`, `GradingPrimaryTransform::set_value_from_f64`, `GradingToneTransform::copy_value`, and `GradingToneTransform::set_value_from_f64` are also deprecated in favor of the clearer `value()` / `set_value(...)` methods.

The same cleanup now applies to the remaining C-style compatibility entry points in processor wrappers. Prefer `CPUProcessor::apply_rgb_packed_bit_depth` / `apply_rgba_packed_bit_depth` with the `BitDepth` enum, `CPUProcessor::dynamic_property` / `has_dynamic_property_kind` with `DynamicPropertyType`, and the structured `GpuShaderDesc` accessors such as `uniform()`, `uniforms()`, `texture_2d()`, `textures_2d()`, `texture_3d()`, and `textures_3d()`.

On `Config`, the older `get_processor_v*` overload naming is also being phased out where clearer Rust names now exist. Prefer `processor()`, `processor_with_context()`, `processor_display()`, `processor_display_with_context()`, `processor_from_transform_default_direction()`, `processor_from_transform()`, `processor_from_transform_with_context()`, `processor_named_transform()`, `processor_named_transform_with_context()`, `processor_named_transform_name()`, and `processor_named_transform_name_with_context()`.

The same cleanup now applies to the long-lived transform metadata aliases. `format_metadata_v1()` / `format_metadata_v2()` and `GroupTransform::get_transform_v1()` remain available for compatibility, but should be treated as deprecated aliases for `format_metadata()` and `get_transform()`.

Processor and config enumeration wrappers are also converging on clearer Rust names. Prefer `Processor::optimized_processor_bitdepth()`, `Processor::optimized_cpu_processor_bitdepth()`, `GPUProcessor::extract_shader_info()`, `Config::num_color_spaces()`, `Config::color_space_name_by_index()`, `Config::num_named_transforms()`, and `Config::named_transform_name_by_index()` over the legacy `*_v1` compatibility aliases.

Display/view configuration is following the same pattern. Prefer `Config::default_view()`, `num_views()`, `view()`, `add_display()`, `add_display_view_detailed()`, `virtual_display_num_views()`, and `virtual_display_view()` over the older `get_*_v1`, `get_*_v2`, and `add_display_view_v1/v2` compatibility names unless you explicitly need those OCIO-shaped overloads.

The same applies to active-display and display-enumeration helpers: prefer `active_display()`, `active_view()`, `num_displays_all()`, `display_all()`, and `display_all_index()` over the old `get_active_*` and `get_*_all` names.

The multi-config processor overloads now have descriptive Rust names as well. Prefer `processor_from_color_spaces()`, `processor_from_configs_with_contexts()`, `processor_from_configs_with_interchange()`, `processor_from_configs_with_contexts_and_interchange()`, `processor_from_configs_to_display()`, `processor_from_configs_to_display_with_contexts()`, `processor_from_configs_to_display_with_interchange()`, and `processor_from_configs_to_display_with_contexts_and_interchange()` over the old `get_processor_v1` and `get_processor_from_configs_v*` numbering.

The same naming cleanup now applies to common lookup helpers. Prefer `color_space()`, `color_space_index()`, `look()`, `named_transform()`, `named_transform_index()`, `view_transform()`, `processor_to_builtin_color_space()`, and `processor_from_builtin_color_space()` over the older `get_*` variants when updating call sites for `0.2`.

Crate-level and utility entry points are following the same pattern. Prefer `current_config()`, `Config::config_io_proxy()`, `Context::config_io_proxy()`, `Baker::num_formats()`, `Baker::format_name_by_index()`, and `Baker::format_extension_by_index()` over the corresponding `get_*` compatibility names.

`GpuShaderDesc` is receiving the same treatment around lightweight value accessors. Prefer `uniform_value_count()`, `texture_value_count()`, `texture_3d_value_count()`, `texture_3d_values()`, and `texture_3d_shader_binding_index()` over the remaining `get_*` / `copy*` compatibility helpers when migrating `0.2` call sites.

Dynamic-property discovery is being normalized toward typed enums as well. Prefer `Processor::has_dynamic_property_kind()` and `CPUProcessor::has_dynamic_property_kind()` with `DynamicPropertyType` over raw integer-based checks when updating `0.2` call sites.

The same treatment now applies to `Config` helpers that require external OCIO config, interchange, view, or virtual-view descriptor pointers. They remain available for ABI interop, but are no longer presented as normal Rust-native APIs.

### Bundled builds

Bundled Windows builds now force a Release CMake profile and link against Release transitive libraries where available. This avoids Debug CRT mismatches when Rust tests run against the bundled OCIO build.

### Legacy GPU optimization

`Processor::optimized_legacy_gpu_processor()` remains available for OCIO's older LUT-baking GPU path, but it is now deprecated in the Rust API. Prefer `optimized_gpu_processor()` or `default_gpu_processor()` for OCIO 2.5 workflows.
