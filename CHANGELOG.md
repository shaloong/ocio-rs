# Changelog

## Unreleased

- Continue replacing generated stub symbols with real OpenColorIO bridge calls.
- Remove wrapper methods for APIs that are not present in OpenColorIO 2.5.
- Add project status, contribution, and security documentation.
- Expand CI coverage for formatting, linting, tests, docs, packaging, and manual bundled builds.
- Harden `Config` wrapper coverage for OCIO 2.5.1/2.5.2 entry points, including
  active display/view management, virtual displays, processor overloads, and
  built-in configs.
- Improve docs.rs build metadata and align parity reporting so compatibility
  aliases no longer show up as hard release blockers.
- Add a release-audit helper script that codifies the pre-publish checks and
  reports the known top-level package blocker separately from real failures.
- Add a manual GitHub Actions `Release Audit` workflow that runs the repository
  audit, bundled tests, and offline packaging checks from a recursive checkout.
- Promote the parity checker into both CI and the release-audit flow so wrapper
  and bridge drift show up as first-class release failures.
- Make `Config::serialize()` and `Config::archive()` return real OCIO text in
  non-stub builds, and deprecate `Processor::optimized_legacy_gpu_processor()`
  so OCIO 2.5 callers prefer the main GPU processor path.
- Fix `Baker::bake()` so it no longer routes a filesystem path through an
  `ostream*` ABI slot; baked output now goes through `bake_to_string()` first
  and is written to disk from Rust.
- Continue normalizing Config and builtin-registry lookup helpers toward
  Rust-style names such as `color_space()`, `named_transform()`,
  `view_transform()`, and `processor_to_builtin_color_space()`, while keeping
  the legacy `get_*` entry points as deprecated compatibility aliases.
- Continue the same cleanup for crate-level and utility entry points, including
  `current_config()`, `config_io_proxy()`, and `Baker` format enumeration
  helpers, while retaining deprecated `get_*` aliases for compatibility.
- Continue the `GpuShaderDesc` naming cleanup by adding Rust-style value
  accessors such as `uniform_value_count()`, `texture_value_count()`,
  `texture_3d_value_count()`, `texture_3d_values()`, and
  `texture_3d_shader_binding_index()`.
- Normalize dynamic-property discovery around typed
  `has_dynamic_property_kind(DynamicPropertyType)` checks on both `Processor`
  and `CPUProcessor`, while keeping raw integer compatibility aliases.
- Continue the Config naming cleanup for display/view metadata helpers,
  including virtual-display queries and `default_view_transform_name()`, while
  keeping deprecated `get_*` compatibility aliases.
- Deprecate `get_color_space_from_filepath_by_ref_type()` as a compatibility
  alias for the file-rule-based `color_space_from_filepath_with_rule_index()`
  flow, matching the Rust API's preferred file-resolution model.
- Add `GroupTransform::write_to_string(&Config, format)` so callers do not need
  to pass raw `ostream*` pointers through the OCIO ABI for serialized transform
  output.
- Replace the grading RGB/hue curve transform value-handle APIs with safe Rust
  snapshot structs for read/write access, leaving raw handle access as explicit
  deprecated escape hatches.
- Mark remaining raw ABI escape hatches in `Config`, `Context`, and
  `CPU/GPUProcessor` as deprecated so they are less likely to be mistaken for
  normal Rust-native APIs.
- Deprecate `GroupTransform::write` and grading primary/tone compatibility
  aliases that now have clearer Rust-native replacements.
- Mark `Config` helpers that require raw OCIO config/interchange/view pointers
  as deprecated escape hatches instead of ordinary wrapper APIs.
- Deprecate the remaining `GpuShaderDesc::get_*` / `copy_*`,
  `CPUProcessor` integer-only compatibility aliases, and
  `FileRules::insert_rule_v1` in favor of the typed Rust wrapper surface.
- Add descriptive `Config` processor overload names for display/context/
  transform/named-transform workflows and deprecate the corresponding
  `get_processor_v*` compatibility aliases.
- Deprecate the remaining transform-level `format_metadata_v1/v2` aliases and
  `GroupTransform::get_transform_v1` so callers are steered toward the
  canonical metadata and child-transform helpers.
- Add clearer processor/config wrapper names around optimization, shader
  extraction, and enumeration helpers, and deprecate the corresponding
  `*_v1` compatibility aliases.
- Continue the Config cleanup by deprecating display/view overload aliases and
  adding clearer names such as `add_display_view_detailed()`,
  `virtual_display_num_views()`, and `virtual_display_view()`.
- Add descriptive names for the remaining multi-config processor overloads and
  deprecate the corresponding `get_processor_v1` /
  `get_processor_from_configs_v*` compatibility entry points.
- Continue the Config naming cleanup for active-display and all-display
  enumeration helpers by adding `active_display()`, `active_view()`,
  `num_displays_all()`, `display_all()`, and `display_all_index()`.

## 0.2.0

- Targets OpenColorIO 2.5.2.
- Experimental release line for real OCIO bridge validation.
