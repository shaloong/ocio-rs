# OpenColorIO 2.6 compatibility audit

The comparison baseline is the public headers at upstream `v2.5.2` and
`v2.6.0`. The `v2_6` feature adds to `v2_5`; it does not change the
default vendored version or the Rust ABI values of existing enums.

| Upstream addition | Rust coverage |
| --- | --- |
| Display and view alias authoring, inspection and resolution | `Config` alias methods and `ViewWithAliases` |
| Built-in color space matching, ID lookup and local ID generation | `Config::locate_builtin_color_space`, `find_color_space_for_id`, `generate_local_id_for_color_space` |
| HDR config compatibility helper | `Config::check_hdr_display_compatibility` |
| ACES RGB to normalized HMJ | `FixedFunctionStyle::AcesRgbToHmj20` |
| SMPTE ST 2136-1 CLF | Existing file/group transform APIs and `metadata_id_element()` |
| Updated built-in configs and transforms | Existing built-in registries; use the native identifiers |

The native enum inserts HMJ before the HSY variants. The bridge translates
style values in both directions at the API boundary, not in the pixel loop.

For rendering, build the processor once and pass the caller's mutable buffer
to `CPUProcessor::try_apply_rgba_pixels`. A batch crosses the bridge once
for processing, with no Rust pixel-buffer allocation or copy. This does not
claim that OCIO itself never allocates. The HMJ benchmark compares batch and
per-pixel calls using the same native processor:

```sh
cargo bench --features bundled,v2_6 --bench pipeline -- ocio_2_6_hmj
```

Set `OCIO_SOURCE_DIR` to the 2.6.0 checkout before running this command.
Config queries take reusable `&CStr` arguments without converting them in
the bridge. Returned config text is owned because the native config can
invalidate its strings. ID lookup returns an independent editable color
space; alias authoring materializes the vector required by the C++ API.
Reuse the built-in config passed to matching to retain its native cache.

The parity checker measures its recognized API surface, not every overload,
global, or runtime behavior. The exact-tag CI also runs behavior tests and
all three API feature levels. DirectX and Vulkan test frameworks are upstream
test infrastructure, not additional application-facing binding APIs.

## Remaining full-library parity work

This is not a full-library parity claim. In particular, the existing checker
does not account for all application helpers in `OpenColorAppHelpers.h`.
The color-space menu parameters and immutable menu are now bound at their
native version floors. The menu borrows native strings without allocating.
The mixing manager also delegates selection and conversions to OCIO; its
slider is an exclusive borrow with no allocation or shared-pointer increment.
The legacy viewing pipeline includes all correction stages, looks overrides,
and both processor overloads. The display-view identity helper also exposes
the native dynamic exposure, contrast, and gamma stages.
Config editing/merging and the remaining display-view namespace functions
still need bindings.
Track overloads and
namespace functions by signature rather than inferring their coverage from
method-name matches. Hardware GPU execution and comparison against a
direct-C++ performance baseline remain separate validation gates.
