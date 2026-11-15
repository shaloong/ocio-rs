# OpenColorIO 2.6 compatibility audit

See [the full-coverage plan](OCIO_2_6_PLAN.md) for remaining work, validation
tiers, and the criteria for claiming completeness.
The [GPU creator signature-family audit](OCIO_GPU_CREATOR_AUDIT.md) records
the current mappings and the gaps in custom creator ownership and callbacks.
The [grading curve signature-family audit](OCIO_GRADING_CURVE_AUDIT.md) tracks
its overloads, ownership, and outstanding read-only access.
The [global API audit](OCIO_GLOBAL_API_AUDIT.md) maps the global declarations
and records remaining validation limitations.
The [config IO proxy audit](OCIO_CONFIG_IO_PROXY_AUDIT.md) maps its virtual
signatures, ownership, and byte-oriented access.

The comparison baseline is the public headers at upstream `v2.5.2` and
`v2.6.0`. The `v2_6` feature adds to `v2_5`; it does not change the
default vendored version or the Rust ABI values of existing enums.

`ConfigIOProvider` may borrow configuration, LUT, and hash data from its own
thread-safe storage, avoiding an intermediate Rust allocation. OCIO's C++
interface requires owned return values, so transfer still copies to native
storage; this is not a zero-copy native API.
LUT hashes can contain arbitrary bytes, including NUL. The byte-oriented proxy
APIs preserve them; the text convenience getter replaces invalid UTF-8.
An empty hash signals a missing file to OCIO. LUT queries also accept reusable
`&CStr` paths, avoiding repeated Rust path allocation; OCIO's virtual methods
may still allocate native return values.
`read_lut_data_cstr` and `read_fast_lut_file_hash_cstr` call the two native
virtual methods independently and preserve empty results; the `try_*` lookup
conveniences interpret an empty hash as unavailable for custom providers.
The corresponding `with_*_cstr` methods let a synchronous visitor inspect the
native result without a second payload copy; OCIO still allocates its native
return value, and the borrowed slice may not outlive the callback.

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
`CPUProcessor::try_convert_packed_to_planar` and `try_convert_planar_to_packed`
also pass borrowed, validated buffers directly to OCIO. Missing source alpha is
interpreted according to the native library: zero in 2.4/2.5 and one in 2.6.

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
`try_is_color_space_linear_cstr` preserves native missing-space errors instead
of conflating them with a negative linearity result.
Color-space and named-transform index queries also offer fallible `&CStr`
variants; a missing entry returns OCIO's `-1`, distinct from bridge errors.
The fallible default-file-rule query likewise distinguishes native evaluation
errors from a non-default match, including when using a reusable `&CStr` path.
Reuse the built-in config passed to matching to retain its native cache.

The parity checker measures its recognized API surface, not every overload,
global, or runtime behavior. Its name matching can conflate getters and setters,
and a matching bridge name alone does not prove the implementation is correct.
`Baker::bake_to_writer` streams native output through a bounded buffer into a
caller-provided Rust writer. Bytes already written remain on failure;
`bake_to_bytes` is available when an owned in-memory result is required.
`Baker::const_config` shares OCIO's immutable config without a deep copy;
`Baker::config` returns an independent editable copy.
`Config::archive_to_bytes` and `archive_to_writer` preserve binary OCIOZ data;
the deprecated text methods reject binary output instead of silently truncating
at NUL bytes. The writer avoids another archive-sized bridge buffer, although
OCIO itself assembles the archive in memory.
On Unix, OCIO 2.4–2.6 may create an archive whose generated `config.ocio`
entry has no permission bits; native extraction may restore those zero bits,
making the extracted file unreadable until its owner changes the permissions.
The binding does not silently change permissions on extracted user files.
The exact-tag CI also runs behavior tests and
all three API feature levels. DirectX and Vulkan test frameworks are upstream
test infrastructure, not additional application-facing binding APIs.

## Remaining full-library parity work

This is not a full-library parity claim. In particular, the existing checker
does not account for all application helpers in `OpenColorAppHelpers.h`.
The color-space menu parameters and immutable menu are now bound at their
native version floors. The menu borrows native strings without allocating.
Read-only current contexts support indexed search paths and variables,
environment mode, resolution with used-variable tracking, and retained IO
proxies without making an editable context copy. Text results are owned because
other native aliases may change the underlying context.
An editable context can merge variables directly from a read-only context;
both expose OCIO's diagnostic stream output, which may contain sensitive data
and is not a stable interchange format.
Shared immutable configurations can create processors with a shared context,
transform, display/view, color-space objects, or named transforms (by name or
object). Static processors retain OCIO
caching; dynamic processors are rebuilt from an independent config copy to
avoid sharing mutable properties.
They may also be installed as the current configuration directly; OCIO makes
its own independent copy when installing either editable or read-only inputs.
An individual context variable can be removed without clearing the others;
config and context IO proxies can also be detached while separately retained
proxy handles remain usable.
Menu parameters, menus, mixing managers, sliders, legacy viewing pipelines,
and merging parameters expose OCIO's diagnostic stream representations as
owned strings; these are not stable interchange or round-trip formats.
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
`GpuShaderDesc::cache_id_ref` borrows the native cache ID without allocating;
its exclusive borrow prevents safe descriptor mutation while it is in use.
Custom creators may supply a native-owned cache ID through
`ShaderCreatorOptions`. The caller is responsible for changing it when the
creator configuration or output changes, including after descriptor mutation;
`set_custom_cache_id` updates or clears it after extraction. This does not
rename already-generated resources, and cloning does not preserve the override.
Custom shader assembly, code-fragment rewriting, texture callbacks, uniform
registration notifications, and creator lifecycle notifications are supported.
Default hook assembly delegates to OCIO, including its Vulkan uniform block.
An overridden assembler must supply its own required language-specific wrapping.
`ShaderFinalizeControl` can instead replace the earlier native finalization
stage; implementations returning custom text are responsible for complete
language-specific declarations and wrappers, and returning `None` delegates to
OCIO. It does not provide a complete native creator subclass.
`GPUProcessor::try_extract_with_streamed_textures` permits uploading native
LUT data during extraction without retaining a descriptor-owned copy. Its sink
must own or upload the data before returning; the resulting descriptor does
not enumerate the streamed textures. `ShaderUniformControl` can retain native
getters and control registration results, including alongside streamed textures;
externally managed uniforms are not included in descriptor buffer offsets.
`try_extract_with_texture_control` additionally allows each texture to be
retained in the descriptor or handled externally, while keeping a global binding
sequence across texture types on OCIO 2.5+.
The Vulkan GPU validation tool exercises streamed 1D, 2D, and 3D resources and
an external dynamic uniform, including combined uniform-plus-LUT pipelines.
It also executes mixed external and descriptor-owned 2D/3D texture pipelines
in both registration orders.
These hooks do not
replace the full native `GpuShaderCreator` subclass interface: Rust callers
cannot yet implement every virtual method or control all resource storage.
