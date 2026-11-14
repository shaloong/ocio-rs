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

Independent B-spline and RGB/Hue curve set objects use native shared ownership.
Transform value exchange creates a native editable copy, while a constituent
spline handle keeps its native curve alive independently.

For rendering, build the processor once and pass the caller's mutable buffer
to `CPUProcessor::try_apply_rgba_pixels`. A batch crosses the bridge once
for processing, with no Rust pixel-buffer allocation or copy. This does not
claim that OCIO itself never allocates.

For two-dimensional packed buffers, `CPUProcessor::try_apply_packed_image`
and `try_convert_packed_image` pass validated borrowed buffers directly to
native packed image descriptors. `PackedImageLayout` supports OCIO's RGB,
BGR, RGBA, BGRA and ABGR orderings, channel spacing, and signed pixel and row
strides; its documented pointer-step padding requirements must be met.

The HMJ benchmark compares batch and per-pixel calls using the same native
processor. The separate `tools/cpu-benchmark` tool compares the safe Rust
wrapper with a C ABI trampoline that calls OCIO C++ directly; its README
documents the setup and limits of that comparison.

Run the HMJ benchmark with:

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
global, or runtime behavior. Its name matching can conflate getters and setters,
and a matching bridge name alone does not prove the implementation is correct.
The exact-tag CI also runs behavior tests and
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
The display-view and color-space namespace helpers now cover file-backed
authoring, removal, and both native display processor overloads. The
`v2_5` API exposes merge parameters and in-memory config/color-space merges.
`ColorSpaceSet` exposes native union, intersection, difference and name-based
equality. Set-operation results retain OCIO's immutable ownership; callers
may request an editable copy of the set or an individual color space.
The `ConfigMerger` controller supports OCIOM serialization and file-driven merges;
its returned configurations and parameters are independent editable copies.
Track overloads and
namespace functions by signature rather than inferring their coverage from
method-name matches. Hardware GPU execution and comparison against a
direct-C++ performance baseline remain separate validation gates.
The optional `tools/gpu-validation` repository tool compares a matrix, dynamic
exposure, linear and tetrahedral 3D LUTs, and 1D LUTs backed by 1D and 2D textures
on a Vulkan GPU against OCIO CPU output; it also compares RGB↔HMJ forward and
inverse results for a small set of upstream-derived inputs.
It does not yet cover the full range of textures or transform surface.
For repeated GPU uniform updates, `GpuShaderDesc::uniform_cache` captures the
native getters once. Its caller-owned output buffers avoid bridge allocations
on each read; getter implementations may still allocate. The cache exclusively
borrows the descriptor and reports the required size for an undersized buffer.
Dynamic property values remain live, rather than being snapshotted at capture.
Custom shader assembly, code-fragment rewriting, texture callbacks, uniform
registration notifications, and creator lifecycle notifications are supported,
and `GPUProcessor::try_extract_with_streamed_textures` permits uploading native
LUT data during extraction without retaining a descriptor-owned copy. Its sink
must own or upload the data before returning; the resulting descriptor does
not enumerate the streamed textures. The Vulkan GPU validation tool exercises
streamed 1D, 2D, and 3D resources. These hooks do not
replace the full native `GpuShaderCreator` subclass interface: Rust callers
cannot yet implement every virtual method or control all resource storage.
