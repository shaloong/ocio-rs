# OpenColorIO 2.6 FFI closeout ledger

Target the public callable API of OpenColorIO 2.4.1 through 2.6.0. A direct
`ocio-sys` function, including a documented unsafe native interop entry point,
counts if a Rust caller can use it with the original OCIO model. Existing
convenience wrappers can stay; equivalent overloads do not need duplicate
owned, borrowed, editable, or read-only Rust spellings.

Use these dispositions for individual signatures: **direct** (ordinary
binding), **raw** (usable low-level/native interop), **not required** (no
additional binding needed), or **gap** (concrete native capability not
callable, or a reproducible behavior/ABI mismatch). Incomplete evidence
is not a gap until the callable native surface has been checked.

| Fixed inventory boundary | Existing evidence and remaining check |
| --- | --- |
| 53 selected exported classes across the core, transform, and app-helper headers | Name matching is automated. Check overload shapes, integer widths, const/ownership semantics, and version gates. Follow the existing family audits where present; count equivalent calls once. |
| Base `Transform`, `ImageDesc`, `PackedImageDesc`, `PlanarImageDesc` | **Direct.** The [base transform](OCIO_BASE_LOG_RANGE_AUDIT.md) and [image descriptor](OCIO_IMAGE_DESC_AUDIT.md) audits map native operations and pointer/layout interop. Native `ImageDesc` has no per-channel strides. |
| `GpuShaderCreator` | [Mapped separately](OCIO_GPU_CREATOR_AUDIT.md), including importing C++ subclasses, invoking virtual methods, and existing Rust hooks. Implementing arbitrary new C++ subclasses entirely in Rust is not required. Verify safe extraction and resource-layout behavior; do not replace native GPU code with a Rust pipeline. |
| Six `DynamicProperty*` classes | **Direct.** [Family audit](OCIO_DYNAMIC_PROPERTY_AUDIT.md) compares the declarations and maps native casts and value operations. Shared RGB/hue setters now forward the actual const handles without an extra editable copy. No new subtype model. |
| `Exception`, `ExceptionMissingFile` | [Checked exception translation](OCIO_EXCEPTION_AUDIT.md) covers both types, including empty diagnostics. Native constructors, exception copying, and unwinding into Rust are not required. Check only public operations that falsely claim success or misclassify a native error. |
| Global runtime functions | [Mapped](OCIO_GLOBAL_API_AUDIT.md). Keep environment and logging synchronization explicit. |
| App-helper namespace functions in `OpenColorAppHelpers.h` | **Direct.** Found 6 declarations in 2.4.1, 8 in 2.5.2, and 9 in 2.6.0, excluding stream operators. Native calls exist for both display processor overloads, identity processor, display add/remove, color-space add, both merge functions, and the single 2.6 HDR compatibility requirement. The class method audit is separate. |
| Seven app-helper classes | [Direct and raw paths mapped](OCIO_APP_HELPERS_AUDIT.md), including both slider and viewing-pipeline overloads, read-only config retrieval without deep copies, indexed/final merge results, and shared merge parameters. Safe merge operations isolate read-only configuration inputs because native heuristics may temporarily change cache flags. |
| `FileRules` and `ViewingRules` | **Direct.** The declarations in 2.4.1, 2.5.2, and 2.6.0 match. See the rule ownership and selector notes below. |
| `Processor`, `CPUProcessor`, `GPUProcessor` | **Direct and raw.** Class declarations match in 2.4.1, 2.5.2, and 2.6.0. The execution and construction mapping is below. |
| `Config` | [Callable families mapped](OCIO_CONFIG_SURFACE_AUDIT.md), including native view-origin queries, role removal, and the versioned display/interop operations. |
| `OpenColorTypes.h` free functions and string constants | **Direct.** All 32 conversions/utilities in each header dispatch to OCIO through the enum, bit-depth, or direction bridge. All 32 exported string constants are present as Rust constants. Their values and the additional 2.6 `METADATA_ID_ELEMENT` value match the 2.6 native definitions. The [enum ABI audit](OCIO_ENUM_ABI_AUDIT.md) separately maps all native declaration changes. |
| Public value types, constructors, and stream operators | **Mapped for the exported value structs in the 2.6 headers.** `GradingRGBM`, `GradingRGBMSW`, `GradingPrimary`, and `GradingTone` have corresponding Rust values; grading control points are passed as `f32` coordinate pairs through spline methods. Native `GpuShaderDesc::UniformData` is queried through typed uniform accessors and cached getters rather than copying C++ function objects across the ABI. Scalar double and bool accessors preserve their native types; the explicitly named `f32` copy path may narrow doubles. Diagnostic formatting need not reproduce every C++ insertion operator. |
| Grading primary and tone values | Rust constructors match the native defaults, including distinct log and video tone windows. Standalone validation delegates to native `GradingPrimary::validate(style)` and `GradingTone::validate()` using the existing value representation, without allocating a transform. |

The selected class inventory and explicitly reviewed excluded classes account
for the 66 `class OCIOEXPORT` names found in the 2.6.0 headers (53 selected
and 13 excluded). This is a fixed review
boundary, not a verified percentage or a new parity gate. Keep L1/L2/L3
as cheap name checks, not signature or behavior claims.

The 13 names excluded from the name-based scanner are not automatically
excluded from the callable API requirement:

| Excluded names | Separate review |
| --- | --- |
| `Transform` | [Base transform](OCIO_BASE_LOG_RANGE_AUDIT.md); shared and editable handles expose common operations. |
| `ImageDesc`, `PackedImageDesc`, `PlanarImageDesc` | [Image descriptors](OCIO_IMAGE_DESC_AUDIT.md); native descriptors and their buffer/layout contracts. |
| `DynamicProperty`, `DynamicPropertyDouble`, `DynamicPropertyGradingPrimary`, `DynamicPropertyGradingRGBCurve`, `DynamicPropertyGradingHueCurve`, `DynamicPropertyGradingTone` | [Dynamic properties](OCIO_DYNAMIC_PROPERTY_AUDIT.md); native casts and typed value access, with hue gated at 2.5+. |
| `GpuShaderCreator` | [GPU creator](OCIO_GPU_CREATOR_AUDIT.md); native subclass interop, virtual calls and documented Rust hooks. |
| `Exception`, `ExceptionMissingFile` | [Exception translation](OCIO_EXCEPTION_AUDIT.md); native errors are translated rather than unwound into Rust. |

The table records the exact inventory boundary; it does not independently
prove that every signature or external subclass behavior is covered.

On the current branch, the name checker matches 1,608 bridge declarations
and 1,390 checked wrapper names. Its selected-class method-name counts match
865/865 against 2.4.1, 1,005/1,005 against 2.5.2, and 1,013/1,013 against
2.6.0. These are lexical checks, not evidence for overload signatures,
ownership, errors, behavior, or the 13 separately reviewed classes.
The dynamic-property and processor behavior suites also passed locally against
native 2.4.1 and 2.6.0, including the 2.6 hue-property paths; this is targeted
runtime evidence, not a substitute for the other excluded-class reviews.

## Finite closeout checklist

| Area | Evidence and disposition |
| --- | --- |
| 2.6.0 public class delta | A comment/whitespace-normalized comparison against 2.5.2 found public declaration changes only in `Config`. The [Config audit](OCIO_CONFIG_SURFACE_AUDIT.md) maps that delta; the [2.6 audit](OCIO_2_6.md) separately covers helper functions, enums, and runtime behavior. |
| Existing signatures | The selected classes have signature-family records in this ledger or its linked audits. The [ColorSpace audit](OCIO_COLOR_SPACE_AUDIT.md) additionally maps the public 2.6.0 declarations individually, and the [ColorSpaceSet](OCIO_COLOR_SPACE_SET_AUDIT.md) and [Context](OCIO_CONTEXT_AUDIT.md) audits map the public 2.5.2 declarations. Confirm individual overload arguments and ownership where the records describe only a family; neither L3 nor the existence of a family record proves each signature. |
| Not separate binding requirements | Deleted or protected constructors, explicit native destruction, equivalent diagnostic formatting and duplicate Rust ownership spellings. Native subclass instances must remain callable where the public OCIO contract accepts them; reimplementing C++ virtual tables in Rust is not required. |
| Known native boundary | Unpatched external OCIO can reject some GPU LUT layouts; the vendored patch and opt-in checked path are documented. Group removal is a Rust convenience with replacement semantics, not a missing C++ operation. |
| Runtime and packaging | The [complete native matrix](https://github.com/shaloong/ocio-rs/actions/runs/44565115136) and [offline release audit](https://github.com/shaloong/ocio-rs/actions/runs/44565114371) passed on `750c4b8`, including bundled Linux/macOS/Windows, installed 2.6, exact 2.4.1/2.5.1 and the 2.5.0 boundary. The native-copy wrapper (`ef3fedc`), borrowed-spline equality (`f789f0e`) and native proxy import (`46e6380`) also received targeted Windows native tests. Later runtime changes add fallible primary/tone grading value reads, tested locally against 2.4.1, 2.5.2 and 2.6.0; the matrix and release audit predate those changes. Earlier targeted Vulkan hardware checks passed on `9520ba6`; they were not repeated on `750c4b8`. These checks do not prove every signature or external-library behavior. |

A focused scan of public, same-name member declarations found 54 overload
families in 2.4.1, 58 in 2.5.2, and 59 in 2.6.0. It is a review aid, not a
compiler-level inventory. The 2.6 additions to these families are the alias-aware
shared-view and display-view writers. Ownership review also found that reading
children of a retained const group formerly required copying the container;
the [group audit](OCIO_GROUP_TRANSFORM_AUDIT.md) records the direct const path.

A separate case-sensitive scan of 2.6.0 class declarations also found 59
same-name families. The manual review grouped their callable shapes as follows:

| Families | Evidence |
| --- | --- |
| Config enumeration, view lookup and authoring, roles, search paths, cache ID, file matching, and processor construction | [Config surface](OCIO_CONFIG_SURFACE_AUDIT.md) and [processor overloads](OCIO_CONFIG_PROCESSOR_AUDIT.md); aliases include both 2.6 writers. |
| Processor optimization, CPU image application, GPU extraction and creator uniforms/properties | Processor notes below, [image descriptors](OCIO_IMAGE_DESC_AUDIT.md), and [GPU creator](OCIO_GPU_CREATOR_AUDIT.md). |
| Context resolution and search paths; app-helper processor, slider and merge results | [Context](OCIO_CONTEXT_AUDIT.md) and [app helpers](OCIO_APP_HELPERS_AUDIT.md). |
| Color-space, LUT, fixed-function and grading-curve factories; file-rule insertion | [Color space](OCIO_COLOR_SPACE_AUDIT.md), [LUT and matrix](OCIO_LUT_MATRIX_EXPONENT_AUDIT.md), [core transforms](OCIO_TRANSFORM_CORE_AUDIT.md), [grading curves](OCIO_GRADING_CURVE_AUDIT.md), and file-rule notes below. |
| Attribute/child access, group children, and per-transform or Baker format metadata | [Format metadata](OCIO_FORMAT_METADATA_AUDIT.md), [group ownership](OCIO_GROUP_TRANSFORM_AUDIT.md), and [Baker](OCIO_BAKER_PROCESSOR_METADATA_AUDIT.md). |

This scan groups declarations by spelling, not ABI signature; it may miss
constructors, inline declarations, operators or parser-unrecognized forms. It
is not a claim that all instance signatures or native behaviors are proven.

For a finite review queue, a separate case-sensitive declaration scan of the
installed 2.6 headers identified the following named families. The links are
family-level evidence; entries whose audit only summarizes a family still need
individual argument/ownership confirmation. This scan is not a compiler AST:
constructors, inline bodies, macros and complicated declarations may be missed.
In particular, `NamedTransform::getTransform` and static `GetTransform` are
different names, not an overload family; both are in the named-transform audit.

| Recognized overload names | Evidence |
| --- | --- |
| `Config::getProcessor`, `GetProcessorFromConfigs`, `getCacheID` | [Processor construction and cache overloads](OCIO_CONFIG_PROCESSOR_AUDIT.md). |
| `Config::addDisplayView`, `addSharedView`, `getDefaultView`, `getNumViews`, `getView` | [Config views](OCIO_CONFIG_SURFACE_AUDIT.md); the alias-aware writers are also tested with distinct fields. |
| `Config::getColorSpaceFromFilepath`, `getColorSpaceNameByIndex`, `getNumColorSpaces`, `getNamedTransformNameByIndex`, `getNumNamedTransforms`, `getRoleColorSpace`, `getSearchPath` | [Config argument and sentinel mapping](OCIO_CONFIG_SURFACE_AUDIT.md) and file-rule notes below. |
| `LegacyViewingPipeline::getProcessor`, `MixingColorSpaceManager::getSlider`, `ConfigMerger::getMergedConfig` | [App helpers](OCIO_APP_HELPERS_AUDIT.md). |
| `FileRules::insertRule`, `ColorSpace::Create` | Rule notes below and [color-space audit](OCIO_COLOR_SPACE_AUDIT.md). |
| `Processor::getOptimizedCPUProcessor`, `getOptimizedProcessor`; `CPUProcessor::apply`; `GPUProcessor::extractGpuShaderInfo`; `GpuShaderCreator::addUniform`, `getDynamicProperty` | Processor notes below, [image descriptors](OCIO_IMAGE_DESC_AUDIT.md), and [GPU creator](OCIO_GPU_CREATOR_AUDIT.md). |
| `Context::getSearchPath`, `resolveFileLocation`, `resolveStringVar`; `FormatMetadata::getAttributeValue`, `getChildElement` | [Context](OCIO_CONTEXT_AUDIT.md) and [metadata](OCIO_FORMAT_METADATA_AUDIT.md). |
| `GradingBSplineCurve::Create`, `getControlPoint`; `GradingRGBCurve::Create`, `getCurve`; `GradingHueCurve::Create`, `getCurve` | [Grading curves](OCIO_GRADING_CURVE_AUDIT.md). |
| `FixedFunctionTransform::Create`, `Lut1DTransform::Create`, `Lut3DTransform::Create`, `GroupTransform::getTransform` | [Core transforms](OCIO_TRANSFORM_CORE_AUDIT.md), [LUT](OCIO_LUT_MATRIX_EXPONENT_AUDIT.md), and [group ownership](OCIO_GROUP_TRANSFORM_AUDIT.md). |
| Const/editable `getFormatMetadata` on `Baker`, `CDLTransform`, `ExponentTransform`, `ExponentWithLinearTransform`, `ExposureContrastTransform`, `FixedFunctionTransform`, all four grading transforms, `GroupTransform`, the three log transforms, both LUT transforms, `MatrixTransform`, and `RangeTransform` | [Metadata](OCIO_FORMAT_METADATA_AUDIT.md), [group ownership](OCIO_GROUP_TRANSFORM_AUDIT.md), and [Baker](OCIO_BAKER_PROCESSOR_METADATA_AUDIT.md). |

The exact queue above supplements the independently counted 59-family review
aid; the two scans use different parsers and are not equivalent inventories.
Neither accounts for constructors or proves every callable instance signature.

The public static entry points in the 2.6 `Config`, transform, and application
helper class declarations were also inventoried separately. Their factories,
registries, processor construction, CDL file loading, look resolution, format
enumeration, curve constructors, and matrix operations are covered by the
linked class audits. `MatrixTransform::Identity` writes directly to caller
arrays through `ocio-sys`; the existing typed `identity` factory instead
constructs a transform. `ConfigMerger::from_file` makes an editable native copy
of the read-only result returned by OCIO; `ConstConfigMerger::from_file` and
`ConfigMerger::merge_configs_shared` retain the native result without that
binding-side copy. Constructors that are deleted,
protected, or exposed only for native destruction are not callable factories.
This static scan does not establish coverage of every instance signature or
behavior.

Within the 53 selected classes, [the monitor and built-in registry
audit](OCIO_REGISTRY_AUDIT.md) records the direct and raw paths for all three
classes and the replacement for the removed default-config getter.
[The LUT, matrix, and exponent audit](OCIO_LUT_MATRIX_EXPONENT_AUDIT.md)
maps five more classes and records the LLP64 LUT size/index fix.
[The base, log and range audit](OCIO_BASE_LOG_RANGE_AUDIT.md) also maps the
four selected log/range classes and the absent camera-slope buffer fix.
[The Baker and processor metadata audit](OCIO_BAKER_PROCESSOR_METADATA_AUDIT.md)
maps two more classes and the shared output callback fixes.
[The core transform audit](OCIO_TRANSFORM_CORE_AUDIT.md) maps eleven additional
classes, including the look-result operation missed by name-only checking.
[The grading curve audit](OCIO_GRADING_CURVE_AUDIT.md) maps the three curve
value classes and the two curve transform classes, including full-width slope
access in `ocio-sys` and direct forwarding of retained values.

At the time of the core transform audit, the checker unit tests (7) passed. L1 matched all 1,570
bridge declaration names; L2 matched all 1,372 checked wrappers; selected
L3 name matches were 865/865 (2.4.1), 1,005/1,005 (2.5.2), and
1,013/1,013 (2.6.0). The explicit cache-ID entry is counted through the
existing public context query rather than requiring another public method.

`LookTransform::GetLooksResultColorSpace` is now directly exposed with a
retained read-only context and an owned Rust result. Native look resolution,
empty input, invalid look and invalid C string cases passed on Windows with
2.4.1, 2.5.2 and 2.6.0. This static method was absent despite name-only L3
matching other LookTransform methods.

Confirmed processor ABI fix: the six legacy processor-building functions
with signed C `int` flags now preserve the published 32-bit mask, including
`OPTIMIZATION_ALL` on LP64. Rust rejects excess high bits rather than
silently truncating. This does not change the raw signatures or add work
to pixel processing.

Processor construction includes both optimized-processor overloads, default
and optimized CPU/GPU paths, the explicit-bit-depth CPU overload and the
legacy baked GPU overload. The flags-only `optimized_processor` is the
ordinary native overload, not interchangeable with selecting a backend.
Integer bit-depth arguments are checked before converting to native enums,
even when OCIO would skip optimization for an empty pipeline. Numeric
dynamic-property queries likewise validate the linked native enum domain.

The processor getters cover no-op, channel crosstalk, cache ID, dynamic
properties, transform count, group conversion, technical metadata and
whole/per-transform format metadata. Format metadata borrows retain their
processor. CPU exposes the same status queries plus identity and bit depths;
single-pixel RGB/RGBA and both `ImageDesc` overloads dispatch directly to
native OCIO. Raw descriptors and safe descriptors/slices cover different
layout ownership contracts (see the [image audit](OCIO_IMAGE_DESC_AUDIT.md)).
Safe single-pixel calls reject CPU processors whose input or output is not
F32 before modifying the pixel. OCIO requires F32 for those calls but does not
enforce it itself. The binding records compatibility at processor construction,
so successful calls do not make extra native bit-depth queries. On Windows
with native 2.6.0, one diagnostic release benchmark measured 10.56 ns per Rust
single-pixel call versus 7.55 ns for a C ABI trampoline, and 1.02x for the
1024-pixel batch. These are machine-specific observations, not guarantees.
GPU exposes the three status/identity queries in its header and both shader
extraction overloads via the descriptor and creator paths, including raw
interop as described in the [GPU creator audit](OCIO_GPU_CREATOR_AUDIT.md).
Windows processor, CPU, and dynamic-property suites passed with native
2.4.1 (57 tests), 2.5.2 (62), and 2.6.0 (62).

File rules call OCIO for factory/copy, both insertion overloads, built-in
path-search insertion, pattern/extension/regex/space fields, custom keys,
priority, removal, and default-rule handling. The two native reserved names
are exposed as constants. Config path matching exposes both native overloads
and the default-only query. Viewing rules call OCIO for factory/copy, insertion
and removal, name lookup, custom keys, and both selector lists. An empty
custom-key value erases the key. The bridge rejects viewing selector indices
that do not fit native `int` before OCIO narrows them; this applies to editable,
read-only, and raw calls. Outer rule indices and custom-key indices remain
`size_t`. Config setters install native deep copies; const rule handles retain
the installed object without copying, and editable getters return copies.
The stream insertion operators are diagnostics, not required operations.
Windows tests for file rules, viewing rules, and const config items passed
on each of 2.4.1, 2.5.2, and 2.6.0 (16 per version).

The individual 2.6.0 declarations for both rule classes were checked against
their Rust and bridge paths:

| Native declarations | Callable path |
| --- | --- |
| Both: `Create()`, `createEditableCopy()`, `getNumEntries()`, `getIndexForRule(const char *)`, `getName(size_t)` | Editable factories, copies and queries, plus const queries on installed rules; fallible index/name calls preserve native errors. |
| File rules: `getPattern` / `setPattern`, `getExtension` / `setExtension`, `getRegex` / `setRegex`, `getColorSpace` / `setColorSpace` | Separate rule-indexed operations, retaining native pattern/regex replacement semantics. |
| Both: `getNumCustomKeys(size_t)`, `getCustomKeyName(size_t, size_t)`, `getCustomKeyValue(size_t, size_t)`, `setCustomKey(size_t, const char *, const char *)` | Indexed reads and the editable key/value setter; passing an empty value uses native removal semantics. |
| File rules: both `insertRule` overloads, `insertPathSearchRule`, `setDefaultRuleColorSpace`, `removeRule`, `increaseRulePriority`, `decreaseRulePriority`, `isDefault` | Direct editable methods, including distinct pattern/extension and regex argument lists; const queries expose the native default predicate. |
| Viewing rules: `getNumColorSpaces`, `getColorSpace`, `addColorSpace`, `removeColorSpace`, and the four corresponding encoding methods | Separate editable and const selector queries and editable mutations; outer indices and counts are `size_t`, with the additional native narrowing guard described below. |
| Viewing rules: `insertRule(size_t, const char *)`, `removeRule(size_t)` | Editable insertion and removal methods with checked conversion from the legacy Rust `u64` index. |

The two file-rule reserved names are exposed as constants. Deleted copy
operations, private constructors, destructors used for native ownership and
diagnostic stream output do not require additional color-management methods.

The 2.6 declarations were also checked by argument shape: the file-rule
insertion overloads accept `(index, name, color space, pattern, extension)`
and `(index, name, color space, regex)` respectively; the path-search rule
accepts only an index. Viewing-rule insertion accepts `(index, name)`, and
its color-space and encoding removals each take a rule and selector index.
The editable methods retain `u64` compatibility and check conversion to native
`size_t`; const queries take `usize`. Index lookup preserves native missing-name
errors through the fallible APIs. On Windows i686, the file- and viewing-rule
oversized-index tests passed with the stub backend, exercising the 32-bit Rust
conversion guards without a native OCIO library. This does not establish
32-bit native runtime behavior.
The filepath overload returning a rule index and the default-only predicate
were also checked with strict parsing enabled: an explicit pattern selects
its index, and a nonmatching file selects the final default rule. File- and
viewing-rule behavior suites passed on native 2.4.1, 2.5.2 and 2.6.0.

A separate one-off Windows i686 C++/Rust executable comparison matched size,
alignment and field offsets for 11 FFI structs. This is target-specific layout
evidence, not a native OCIO behavior test or a guarantee for other platforms.

The vendored GPU generator now counts LUT rows using its interpolation stride
of width minus one, fixing under-allocation without adding a second extraction.
Unpatched system libraries and external source overrides retain the native
limitation; the existing opt-in probe is the portable path there. On Windows,
the GPU descriptor suite passed with the vendored 2.5.2 (81 tests, including
exact red and RGB texels across row boundaries), installed 2.4.1 (64 tests),
and unmodified 2.6.0 source (80 tests). These checks do not execute the shader
on a GPU.

For separately supplied texture data, OCIO 2.4.1--2.6.0 also multiplies
dimensions in `unsigned` before widening for allocation. The bridge now checks
that product before native storage, including forwarding-creator paths, and
rejects incompatible 3D dimensions before dispatch. See the
[GPU creator audit](OCIO_GPU_CREATOR_AUDIT.md)
for the conservative size limit and raw-native interop boundary. This check is
per texture, not per pixel, and is separate from the LUT row-allocation issue.

## Remaining release gates

1. Complete the bounded signature review; fix only confirmed missing
   native calls or reproducible correctness, ownership, or ABI errors.
   Keep the documented GPU LUT distinction between the fixed vendored source
   and unpatched external libraries. Do not impose the expensive checked
   extraction probes unconditionally on normal workloads.
2. Align public docs with the retained APIs and native limitations. Group
   removal rebuilds a wrapper because upstream lacks in-place removal;
   it is not a missing native FFI.
3. Run the existing appropriate CI jobs, package checks, and targeted manual
   performance/GPU checks on the exact candidate. Do not add a CI job per
   signature family. On the current Windows worktree the complete 2.4.1
   system, 2.5.2 bundled, and 2.6.0 bundled suites passed and confirmed the
   linked runtime versions. This is not candidate or cross-platform proof.
   Windows `cargo package --workspace --allow-dirty --offline` verified both
   archives with an isolated crates.io cache. The user-global mirror could
   not resolve the staged sibling; it was not modified. The `ocio-rs`
   archive includes all 33 test fixtures and passed 22 fixture-dependent
   tests after extraction against the installed native 2.6.0. The extracted
   `ocio-sys` archive also built with Cargo offline and `bundled,v2_5`
   using its packaged OpenColorIO 2.5.2 source. This validates local package
   contents and linkage, not the exact candidate commit or other platforms.

## Historical validation notes

These entries describe earlier revisions. Use the closeout checklist above for
the most recent recorded candidate evidence.

After the subsequent Config, rules, processor, and CI changes, the complete
Windows workspace tests passed again for stub `v2_5` and external-source
`bundled,v2_6` against OpenColorIO 2.6.0, including build-configuration and
doc tests. Packaging and cross-platform candidate checks have not been rerun
on this final worktree state.

The subsequent offline workspace package verification failed for the current
worktree: both crate manifests still say 0.3.0, and Cargo resolved the already
cached published `ocio-sys` 0.3.0 while verifying `ocio-rs`. That older crate
lacks the newly added FFI declarations. The extracted sys staging directory
does include the vendored LUT fix, and the top-level package includes 33 test
fixtures, but neither observation makes the failed verification a pass. A
new coordinated crate version and dependency requirement must be verified on
the release candidate; do not overwrite or reuse the published 0.3.0.

Both manifests and the dependency requirement now use the 0.4.0 candidate
version. Windows offline `cargo package --workspace --allow-dirty` passed
using an isolated Cargo cache and a temporary C: target directory. The wrapper
verification used the newly staged 0.4.0 sys crate. This was a stub build, not
native package verification or a clean candidate commit. The extracted
0.4.0 `ocio-sys` archive was then built offline on Windows with
`bundled,v2_5`, without a source override, using its packaged OCIO source.
That validates the packaged native build, not cross-platform behavior or a
clean candidate commit.

Windows release-mode CPU benchmark against linked OCIO 2.6.0 (median of
seven alternating rounds, C ABI trampoline vs safe Rust) measured 7.66 vs
10.82 ns for one RGBA pixel, 183.90 vs 213.65 ns for 16, and 1355.40 vs
1458.80 ns for 1024. This is a single diagnostic run on this machine, not
a general performance guarantee; the curve microbenchmarks and method details
are described in `tools/cpu-benchmark/README.md`.

The current 0.4.0 worktree also passed the existing Windows Vulkan hardware
validation against OCIO 2.6.0 on an NVIDIA RTX 3050 Laptop GPU. The tested
matrix, dynamic uniforms, LUTs, callbacks and HMJ cases stayed within their
documented tolerances. This is targeted hardware coverage, not a full GPU
surface or cross-platform certification.

For candidate `d93ca8213379963a5d687b92074308bc0e462427`, PR CI run
44251694276 and manual CI run 44252059110 passed. The manual matrix included
Linux/macOS/Windows bundled, exact 2.4.1 and 2.5.1, the 2.5.0 ABI boundary,
and 2.6 interface levels plus installed shared-library discovery. Release
Audit run 44252066445 passed. These checks do not replace signature review.
