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
| Seven app-helper classes | [Direct and raw paths mapped](OCIO_APP_HELPERS_AUDIT.md), including both slider and viewing-pipeline overloads, read-only config retrieval without deep copies, indexed/final merge results, and shared merge parameters. |
| `FileRules` and `ViewingRules` | **Direct.** The declarations in 2.4.1, 2.5.2, and 2.6.0 match. See the rule ownership and selector notes below. |
| `Processor`, `CPUProcessor`, `GPUProcessor` | **Direct and raw.** Class declarations match in 2.4.1, 2.5.2, and 2.6.0. The execution and construction mapping is below. |
| `Config` | [Callable families mapped](OCIO_CONFIG_SURFACE_AUDIT.md), including native view-origin queries, role removal, and the versioned display/interop operations. |
| `OpenColorTypes.h` free functions and string constants | **Direct.** All 32 conversions/utilities in each header dispatch to OCIO through the enum, bit-depth, or direction bridge. All 32 exported string constants are present as Rust constants. Their values and the additional 2.6 `METADATA_ID_ELEMENT` value match the 2.6 native definitions. The [enum ABI audit](OCIO_ENUM_ABI_AUDIT.md) separately maps all native declaration changes. |
| Public value types, constructors, and stream operators | Check values used as native inputs or outputs. Checked Rust constructors that produce the same native state are sufficient. Diagnostic formatting need not reproduce every C++ insertion operator. |
| Grading primary and tone values | Rust constructors match the native defaults, including distinct log and video tone windows. Standalone validation delegates to native `GradingPrimary::validate(style)` and `GradingTone::validate()` using the existing value representation, without allocating a transform. |

The selected class inventory and explicitly reviewed excluded classes account
for the 66 `class OCIOEXPORT` names found in the 2.6.0 headers (53 selected
and 13 excluded). This is a fixed review
boundary, not a verified percentage or a new parity gate. Keep L1/L2/L3
as cheap name checks, not signature or behavior claims.

Within the 53 selected classes, [the monitor and built-in registry
audit](OCIO_REGISTRY_AUDIT.md) records the direct and raw paths for all three
classes and the replacement for the removed default-config getter.
[The LUT, matrix, and exponent audit](OCIO_LUT_MATRIX_EXPONENT_AUDIT.md)
maps five more classes and records the LLP64 LUT size/index fix.
[The base, log and range audit](OCIO_BASE_LOG_RANGE_AUDIT.md) also maps the
four selected log/range classes and the absent camera-slope buffer fix.
[The Baker and processor metadata audit](OCIO_BAKER_PROCESSOR_METADATA_AUDIT.md)
maps two more classes and the shared output callback fixes.

On this worktree, the checker unit tests (7) passed. L1 matched all 1,570
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

The vendored GPU generator now counts LUT rows using its interpolation stride
of width minus one, fixing under-allocation without adding a second extraction.
Unpatched system libraries and external source overrides retain the native
limitation; the existing opt-in probe is the portable path there. On Windows,
the GPU descriptor suite passed with the vendored 2.5.2 (81 tests, including
exact red and RGB texels across row boundaries), installed 2.4.1 (64 tests),
and unmodified 2.6.0 source (80 tests). These checks do not execute the shader
on a GPU.

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
