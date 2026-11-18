# OpenColorIO 2.6 compatibility audit

See [the full-coverage plan](OCIO_2_6_PLAN.md) for remaining work, validation
tiers, and the criteria for claiming completeness.
The [GPU creator signature-family audit](OCIO_GPU_CREATOR_AUDIT.md) records
the current mappings and the gaps in custom creator ownership and callbacks.
The [grading curve signature-family audit](OCIO_GRADING_CURVE_AUDIT.md) tracks
its overloads, ownership, and retained read-only access.
The [global API audit](OCIO_GLOBAL_API_AUDIT.md) maps the global declarations
and records remaining validation limitations.
The [config IO proxy audit](OCIO_CONFIG_IO_PROXY_AUDIT.md) maps its virtual
signatures, ownership, and byte-oriented access.
Editable and shared read-only color spaces, looks, and view transforms expose
fallible interchange-attribute snapshots. Each call obtains one native map;
the returned Rust map is independent of subsequent object changes.
Shared const transforms can be assigned to editable color spaces, looks,
view transforms and named transforms; OCIO copies them on assignment. View
and named transforms may clear either direction with the const setter.
These are native editable copies, not necessarily deep graph copies: OCIO
copies a `GroupTransform` container while retaining its children. This also
applies when a group is copied through a transform getter, setter, or config
copy. Changes to shared children can affect both configs; coordinate mutation
and clear affected processor caches before rebuilding processors. Use
`GroupTransform::create_deep_copy` for an independent built-in transform graph.

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
`with_config_data` provides the same scoped access for native config bytes.

| Upstream addition | Rust coverage |
| --- | --- |
| Display and view alias authoring, inspection and resolution | `Config` alias methods and `ViewWithAliases` |
| Built-in color space matching, ID lookup and local ID generation | `Config::locate_builtin_color_space`, `find_color_space_for_id`, `generate_local_id_for_color_space` |
| HDR config compatibility helper | `Config::check_hdr_display_compatibility` |
| ACES RGB to normalized HMJ | `FixedFunctionStyle::AcesRgbToHmj20` |
| SMPTE ST 2136-1 CLF | Existing file/group transform APIs and `metadata_id_element()` |
| Updated built-in configs and transforms | Existing built-in registries; use the native identifiers |

The 2.6 Config additions above have these direct paths:

| Native operation | Rust path and result |
| --- | --- |
| `LocateBuiltinColorSpace` | `Config::locate_builtin_color_space[_ref]` and `ConstConfig::locate_builtin_color_space`; an empty native result becomes `None`. Both input configs can be retained without a deep copy. |
| `findColorSpaceForID` | `find_const_color_space_for_id` retains the returned native shared object; editable `Config::find_color_space_for_id` creates an independent copy. A missing ID becomes `None`. |
| `generateLocalIDForColorSpace` | `generate_local_id_for_color_space` copies the native result into owned Rust text; native errors remain errors. The bridge's temporary text must not be retained across later calls. |
| `getNumDisplayViewAliases`, `getDisplayViewAlias`, `hasDisplayViewAlias` | Matching methods exist on editable and const configs. A missing or out-of-range alias yields native empty text, represented as `None`. |
| `getUseDisplayAliases`, `setUseDisplayAliases` | `use_display_aliases` on both config handles and `set_use_display_aliases` on editable `Config`. |
| `getCanonicalDisplayName`, `getCanonicalViewName`, `getResolvedDisplayViewColorSpaceName`, `getDisplayDescription` | Matching fallible methods on both config handles copy returned text while its native owner is alive. |
| Alias-aware `addSharedView` and `addDisplayView` overloads | `add_shared_view_with_aliases` and `add_display_view_with_aliases` pass all view fields plus an alias vector; empty vectors and native alias collisions have behavior regressions. |

The native enum inserts HMJ before the HSY variants. The bridge translates
style values in both directions at the API boundary, not in the pixel loop.

Independent B-spline and RGB/Hue curve set objects use native shared ownership.
Editable transform value getters create native copies. Retained read-only
values can be passed to another transform's native setter without a bridge-side
deep copy. A retained constituent spline can outlive its parent curve set.

For rendering, build the processor once and pass the caller's mutable buffer
to `CPUProcessor::try_apply_rgba_pixels`. Tightly packed RGBA uses one native
batch call without a Rust pixel-buffer allocation or copy. A strided buffer
without enough trailing space for native pointer stepping may use an additional
call and a stack-sized final pixel. OCIO itself may allocate.

For two-dimensional packed buffers, `CPUProcessor::try_apply_packed_image`
and `try_convert_packed_image` accept validated borrowed buffers.
`PackedImageLayout` supports OCIO's RGB,
BGR, RGBA, BGRA and ABGR orderings, channel spacing, and signed pixel and row
strides. Unpadded tails are supported by splitting affected regions and copying
at most one pixel at a time; tight forward RGBA retains the native fast path.
`CPUProcessor::try_convert_packed_to_planar` and `try_convert_planar_to_packed`
also accept borrowed, validated buffers and may use a one-pixel tail fallback.
A later failure can leave earlier output modified. Missing source alpha is
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
invalidate its strings. `find_color_space_for_id` returns an independent
editable copy; `find_const_color_space_for_id` instead retains the native
read-only shared object without a deep copy, with one ownership-handle
allocation. The same read-only lookup is available on `ConstConfig` without
first copying the config; `ConstConfig::generate_local_id_for_color_space`
also calls the native const method directly, returning the generated owned text.
OCIO's ID lookup searches names and aliases, not the `interop_id` attribute
itself; it applies its namespace and config-specific local-ID fallback rules.
Alias authoring materializes the vector required by the C++ API.
Built-in matching also accepts a `ConfigRef` to editable or shared read-only
configurations. It does not copy either config, so reused native objects retain
their OCIO fingerprint caches.
Read-only `ConstConfig` also exposes the 2.6 canonical display/view, alias,
resolved color-space, display description and HDR compatibility queries.
Display aliases follow the config's `use_display_aliases` setting; view aliases
remain active independently. A `Baker` stores an editable config copy when
configured, so an earlier `Baker::const_config` snapshot does not track later
changes to the source config.
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

This is not a full-library parity claim. The checker now includes the seven
application-helper classes, but checks selected method names rather than all
overloads, signatures, ownership and behavior. The separate
[app-helper audit](OCIO_APP_HELPERS_AUDIT.md) records those distinctions.
See [the Context audit](OCIO_CONTEXT_AUDIT.md) for ownership, resolution and
the native variable-clear cache invalidation workaround.
The [named-item audit](OCIO_NAMED_ITEMS_AUDIT.md) records look, named transform,
and view-transform ownership and diagnostic stream behavior.
Group child insertion and replacement reject direct or indirect cycles while
retaining native sharing for acyclic graphs. Cycle checks occur on edits, not
in the CPU or GPU pixel-processing path.
Native group editable copies share their child transforms; opt into
`GroupTransform::create_deep_copy` for a separate graph with internal sharing
preserved. See [the group audit](OCIO_GROUP_TRANSFORM_AUDIT.md) for the remaining
in-place removal limitation.
Editable transforms returned from color spaces, named items and groups use
type-correct native bridge handles. Group children retain native sharing;
look, view and color-space accessors documented as copies remain independent.
Editable metadata children are path-backed and detect ancestor clearing across
binding-created handles; a stale child cannot silently target a newly inserted
element at the same index. Unsynchronized external native mutations remain the
application's responsibility.
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
`try_extract_with_texture_policy` also delegates native texture-width and 1D
policy getters and setter notifications to synchronous Rust callbacks during extraction. These are
detached on return; callback errors use a conservative temporary native fallback
and are returned to the caller. The opt-in checked variant probes optimized
texture lengths then validates each dynamic width; it has the extra extraction,
false-positive and unchanged-resource-state limitations documented by the API.
The caller must provide a nonempty cache ID that changes whenever the policy or
output changes, since OCIO's template-derived ID cannot describe dynamic policy.
`PersistentPolicyCreator` keeps a Rust texture policy attached to the same
native creator between calls. It detaches callbacks before releasing Rust state,
requires explicit cache invalidation, and does not itself preflight LUT packing.
`ShaderCacheIdPolicy` and `try_extract_with_cache_policy` optionally provide a
dynamic native cache ID for one extraction. The callback is detached on return;
outside that call the creator's fixed ID remains in effect. Errors and panics
are returned to the caller, and the callback must not reenter the creator. It
can borrow a `CStr` stored in the policy without a Rust-side string allocation;
OCIO copies the ID for its native cache-key construction.
Its optional persistent texture controller selects descriptor or external
storage and can consume borrowed texels without a Rust-side copy. External
resources created before an extraction error remain the caller's responsibility.
`create_with_resource_control` also keeps uniform registration control attached;
retained uniform getters keep the native creator alive after wrapper destruction.
`create_with_hooks` additionally retains lifecycle notifications and code
fragment hooks; it does not install a custom shader assembler.
`create_with_assembler` also retains an assembler, which may delegate to the
native implementation or provide complete shader text with the appropriate
language-specific uniform wrapper.
`create_with_finalizer` additionally retains finalization control: delegation
continues to the assembler, while a replacement bypasses it.
The Vulkan GPU validation tool exercises streamed 1D, 2D, and 3D resources and
an external dynamic uniform, including combined uniform-plus-LUT pipelines.
It also executes mixed external and descriptor-owned 2D/3D texture pipelines
in both registration orders.
These hooks do not replace the full native `GpuShaderCreator` subclass
interface: Rust callers cannot implement every virtual method. Importing a
native C++ creator remains available through low-level interop; reimplementing
all native subclasses in Rust is outside the thin-FFI completion boundary.
