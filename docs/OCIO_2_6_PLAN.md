# OpenColorIO 2.6 full-coverage plan

The objective is usable, correct Rust access to the public OpenColorIO API
through 2.6.0, not merely matching method names. The supported native range is
`>= 2.4.1, < 2.7`; later interfaces remain opt-in through their Cargo features.
Work is prioritized by missing capabilities, safety, and application impact.

## Work breakdown

1. **Inventory and traceability.** Compare the public 2.4.1, 2.5.2 and 2.6.0
   headers by class, signature, overload, enum, callback and free function;
   map each to the bridge, safe API, version gate, and behavior test. Account
   explicitly for constructors, ownership, exceptions and unsupported or
   intentionally unsafe native interop. Name-level parity is a starting point,
   not proof of signature or behavioral parity.
2. **Architecture and safety.** Preserve normal native operations and existing
   extension points. Document and test callback lifetime, ownership, error and
   panic containment, reentrancy, thread safety, and resource cleanup. Keep OCIO
   semantics at the boundary. Calling an existing native subclass is in scope;
   reimplementing the C++ class hierarchy or virtual tables in Rust is not.
3. **Behavior by subsystem.** Audit configuration, colorspaces and views,
   contexts and files, transforms and metadata, processors and image layouts,
   GPU shader resources and dynamic properties, app helpers, and global APIs.
   Exercise both normal and boundary/error cases against real native libraries.
4. **Performance and compatibility.** Preserve batched FFI calls, borrowed
   buffers and zero-copy views where lifetimes allow; benchmark hot paths against
   direct C++ and run hardware GPU comparisons where applicable. Check version
   gates, feature combinations, static/shared builds and supported platforms.
5. **Documentation and release readiness.** Keep examples, compatibility and
   limitations current. Before any completeness or release claim, review the
   inventory evidence, native and cross-platform results, performance regressions,
   CI state, and release/package checks for the exact candidate commit.

The current inventory and runtime validation are incomplete; the status in
`OCIO_2_6.md` must not be read as a full-library coverage claim.

## Historical validation notes

The following notes describe the revisions and worktrees that were current
when each check was run. References to the "current" tree below are historical,
not claims about the latest commit. Use the closeout ledger and the actual CI
run for candidate-specific evidence.

Native 2.5.0 baseline compatibility was tested on Windows with an isolated
bundled source build. Its GPU descriptor suite passed 63 tests after fixing
2.5.1-only virtual method guards and distinguishing 2.5.0 per-kind texture
indices from shader bindings. `native_smoke` reported 2.5.0. The `v2_5` Rust
feature still requires 2.5.1. The full 2.5.0 bundled baseline workspace then
passed on Windows after fixing transform-type and GPU-language enum translation:
391 library tests, integration suites, 16 build-configuration tests and 40 doc
tests. This does not establish hardware Vulkan, cross-platform, or release
candidate coverage.
The exact `v2_5` minimum, native 2.5.1, also passed a Windows isolated
`bundled,v2_5` workspace run after the version-boundary fixes: 412 library
tests, integration suites, 16 build-configuration tests and 40 doc tests.
The runtime smoke check reported 2.5.1. Build outputs remain in
`target/ocio251-validation`; this is not cross-platform or offline evidence.
The current uncommitted working tree subsequently passed `cargo test --workspace
--features v2_6` on Windows with the static native 2.6.0 installation, including
409 library tests, integration tests, 14 build-configuration tests and 39 doc
tests. This is not evidence for another platform, a bundled build, or a future
release commit.
The same current working tree passed `cargo test --workspace --features v2_5`
against the Windows static native 2.5.2 installation, including 410 library
tests, integration tests, 14 build-configuration tests and 39 doc tests. This
does not establish bundled or cross-platform compatibility.
The same working tree also passed `cargo test --workspace` against the Windows
static native 2.4.1 installation: 389 library tests, integration tests, 14
build-configuration tests and 39 doc tests. This verifies the minimum native
version on this platform, not the full version/feature/platform matrix.
The local revision before the latest creator uniform additions passed the complete workspace test command on Windows
against static native 2.6.0 (`system v2_6`, 406 library tests), 2.5.2
(`system v2_5`) and 2.4.1 (`system`, 386 library tests), including integration,
build-configuration and documentation tests. The 2.6 GPU descriptor behavior
suite separately passed all 74 tests.
`native_smoke` reported the expected native versions. These are Windows static
system-build results, not
proof of every feature combination or platform. The separate
2.5.2 clean bundled rebuild remained unverified at that point because permission
review timed out and the sandboxed MSVC/CMake compiler check failed. These
results do not replace the cross-platform and release-candidate gates below.

The current working tree subsequently passed both RGB and hue curve transform
behavior suites (5 and 6 tests) using Windows `bundled,v2_6` with the local
2.6.0 source, outside the sandbox; `native_smoke` reported 2.6.0 and verified
native processing. Build artifacts may have been reused, and this is not a
clean bundled matrix or a release-candidate test. The same attempt in the
sandbox failed during CMake's MSVC compiler probe with C1902, before project
compilation; the outside-sandbox success narrows that failure to the environment.
The same targeted bundled suites subsequently passed against local 2.5.2
source outside the sandbox (5 RGB and 6 hue tests); `native_smoke` reported
2.5.2. Existing build artifacts may have been reused. Neither this targeted
check nor the 2.6.0 check establishes a clean bundled build or a full matrix.
Subsequently, the 2.6.0 RGB and hue suites were rerun outside the sandbox with
a previously nonexistent isolated Cargo target directory. Windows bundled CMake
output is keyed by the Cargo output path, source and generator, so this used a
separate native build directory as well; all 11 tests passed and `native_smoke`
reported 2.6.0. This is a clean targeted Windows build, not a full workspace,
offline, cross-platform, or release-candidate matrix. The isolated artifacts
remain in `target/ocio26-clean-validation`.
The same isolated native build then passed `cargo test --locked --workspace
--features bundled,v2_6 -- --test-threads=1` outside the sandbox: 411 library
tests, integration suites, 16 build-configuration tests and 39 doc tests.
The native artifacts were reused from the preceding isolated build; this does
not establish offline operation, another platform, or a final commit's status.
An independent previously nonexistent `target/ocio252-clean-validation` Cargo
target and corresponding native CMake output were also used to build 2.5.2
bundled from source outside the sandbox. The targeted RGB and hue suites passed
(5 and 6 tests), and `native_smoke` reported 2.5.2. This is not a full bundled
workspace or offline/cross-platform validation. Its build artifacts remain in
the isolated target directory.
That isolated 2.5.2 native build subsequently passed `cargo test --locked
--workspace --features bundled,v2_5 -- --test-threads=1` outside the sandbox:
412 library tests, integration suites, 16 build-configuration tests and 39 doc
tests. It reused the preceding build and does not prove offline or cross-platform
behavior or validate a future release commit.

The current local code also passed the manual Vulkan validation tool on an
NVIDIA GeForce RTX 3050 Laptop GPU using native 2.6.0 static system libraries.
The largest observed CPU/GPU difference was 0.003625 in the hardware-filtered
3D LUT case (tool threshold 0.005); inverse HMJ differed by at most 0.000003.
The persistent texture-policy case also executes a shader extracted with the
borrowed dynamic cache-ID callback (maximum difference 0.001250).
This validates the tool's listed cases, not all GPU formats, drivers, hardware,
or the eventual release commit. No PR CI job was added.

The manual CPU baseline was also run twice in release mode on Windows against
the same native 2.6.0 static installation. For the tool's F32 RGBA channel-swap
workload, the Rust/direct-C-ABI median ratios were 1.43–1.45 for one pixel,
1.14–1.22 for 16 pixels, and 0.95–1.02 for 1024 pixels. The variability means
these are diagnostic observations, not a regression threshold or proof of
zero-cost operation. The three-point curve bulk-read path took 38–40 ns versus
79 ns for individual reads in these runs; it is a different workload from the
image test. Both comparisons use a native C ABI trampoline, not inlined C++, and
do not generalize to other transforms, machines or build configurations. The
benchmark remains a manual/release-candidate check, not a PR CI job.

The subsequent FileRules and ViewingRules index fixes, and FileRules custom-key
error-propagation fix, passed their behavior suites on the Windows 2.5.2 system
build. Their 32-bit-only regressions are present but have not been run on a
32-bit target; integer conversions in other API families still need auditing.
Fixed-function style queries now provide `try_style` so a newer native style
cannot silently become an unrelated enum value when an older Rust interface
feature is selected. The original `style` method retains its documented
compatibility fallback. This behavior was checked with the baseline Rust
interface against native 2.5.2. GPU shader language now similarly reports an
unavailable newer language through `try_language`; the compatibility getter
retains its GLSL 1.2 fallback. Config and context environment mode queries
likewise expose fallible conversion, and shared `ConstConfig` can query its
native mode without copying. Windows 2.4.1 and 2.6.0 targeted config/context
tests passed. Other enum getters still need review.
Every editable transform implementing the sealed `TransformHandle` trait and
shared `ConstTransform` now has a fallible native-direction query that rejects
unrecognized discriminants. Existing concrete `direction()` compatibility
getters retain their fallback behavior. Windows native 2.4.1 and 2.6.0 tests
cover generic and readonly direction queries; a conversion unit test covers
invalid values. This does not complete the remaining enum audit.
Against native 2.6.0 on Windows, baseline and `v2_5` library suites also passed
(385 and 406 tests respectively), including a regression for the 2.6-only HMJ
style; the `v2_6` version-specific behavior suite passed all ten tests, including
display/view alias processing through a real CPU processor. This
is not a complete lower-interface integration matrix.

RGB and hue transform control-point counts now check native `size_t` before
narrowing to `int`; their Rust readers reject negative counts and report
reservation failures. Allocation-transform variable reads use the same checked
count and fallible reservation. Targeted Windows native 2.4.1 and 2.6.0 suites
passed where the corresponding interfaces exist. Synthetic Rust tests cover
negative counts and capacity overflow; the native `size_t > INT_MAX` branch has
not been exercised with an actual multi-billion-point native allocation.
The same checked narrowing now covers dynamic curve properties, builtin-style
counts, and legacy interchange-attribute counts. Compile-time C++ assertions
check the `INT_MAX` boundary; targeted dynamic-property and registry suites
passed against native 2.4.1 and 2.6.0, with interchange tests on 2.6.0. This
does not establish that every native count conversion has been audited.

Editable RGB and hue curve writes validate every supplied curve's count before
mutation; native errors during subsequent writes may still leave partial changes.
RGB transforms additionally provide a batched checked snapshot update: it
validates an independent native copy before calling OCIO's setter. Validation
failure leaves the transform unchanged; a failure inside the native commit is
not guaranteed to roll back. The targeted Windows static 2.4.1 and 2.6.0 suites
cover successful updates and invalid later-curve rollback before commit.
Hue transforms have an analogous checked eight-curve update that preserves
other native value state; targeted Windows static 2.5.2 and 2.6.0 tests cover
draw mode preservation and rejection before commit. Native commit failures
still have no general rollback guarantee. Two standalone Windows release runs
with native 2.6.0 measured the default RGB snapshot's incremental write at
34.79–34.96 microseconds and checked bulk write at 1.32–1.42 microseconds.
These have different validation semantics and are not a pure FFI comparison or
a general performance guarantee. Hue snapshot writing has not been benchmarked.
The v2_6 workspace also compiled in stub mode after both additions.
Shared RGB and hue spline views were also exercised across incremental updates,
checked replacement, and source-transform destruction on Windows: RGB with
native 2.4.1 and 2.6.0, hue with native 2.5.2 and 2.6.0. These tests do not
establish concurrent mutation safety or validate arbitrary native aliases.
Fixed-function parameters and builtin registries expose fallible full-width
counts, and fixed-function parameter reads use fallible reservation. Editable
color spaces and named transforms likewise expose full-width fallible alias
counts and indexed lookups while retaining the legacy getters. Targeted Windows
native 2.4.1 and 2.6.0 behavior suites passed, as did synthetic curve-count
boundary checks; huge actual collections and other platforms were not tested.
Read-only color-space alias counts use native `size_t` width; the legacy signed
bridge reports overflow rather than narrowing. Windows native 2.4.1 and 2.6.0
set behavior suites cover ordinary counts, not huge collections.
Editable color spaces and named transforms also expose fallible alias/category
membership queries and allocation-free C-string inputs. Their Windows native
2.4.1 and 2.6.0 behavior suites cover present, absent, and invalid Rust strings;
this is not a complete signature or cross-platform audit of those classes.
Editable color-space allocation variables now also support a caller-owned
buffer, reject invalid native counts, and use fallible allocation for snapshots.
Windows 2.4.1 and 2.6.0 tests cover empty, populated and mismatched buffers.
Editable `Look` exposes fallible name, process-space and description reads;
targeted Windows native 2.4.1 and 2.6.0 tests cover populated and empty values.
For `ViewTransform`, a retained read-only transform was verified on Windows
with native 2.4.1 and 2.6.0 to survive clearing its slot and dropping the view;
this targeted ownership test does not constitute a complete class audit.
Its corresponding setters accept `CStr` without allocating an intermediate
Rust string; OCIO may still copy the value into its own storage. The same
native-version tests cover these setters and reject interior NULs in `str` inputs.
These strings may also be borrowed without copying through an exclusive handle
borrow; a compile-fail test checks that safe mutation is blocked during use.
External native aliases must not mutate the object during such a borrow.
In both native sources, `Look` transform setters dereference their argument, so
the bridge must reject null rather than exposing a misleading clear operation.

For editable `FormatMetadata`, fallible reads now distinguish an invalidated
child handle from a valid empty value or zero count. Existing convenience reads
retain their fallback behavior. The existing metadata behavior suite passed on
Windows with native 2.4.1 and 2.6.0, including ancestor-clear invalidation and
replacement. Editable metadata assignment now preserves native deep-copy
semantics, invalidates destination descendants before mutation, and snapshots
overlapping ancestor/descendant sources. Self-assignment is a no-op. The same
suite covers these cases on native 2.4.1 and 2.6.0; this does not complete the
metadata signature audit or prove other native versions and platforms.
Processor-owned read-only metadata may also be assigned directly to editable
metadata without an intermediate editable copy. The suite checks ownership
after processor drop, copy independence, and destination-handle invalidation
on the same native versions; the stub configuration was compile-checked.

## Remaining execution order

Use the [closeout ledger](OCIO_2_6_CLOSEOUT.md) to distinguish real FFI gaps
from missing evidence, native limits, and optional convenience APIs.

1. Finish a signature-level inventory against the exact 2.4.1, 2.5.2 and
   2.6.0 public headers. Record evidence per API family, including overloads,
   virtual methods and ownership; do not interpret a passing name-based parity
   check as proof of completeness. Consult the closeout ledger for the current
   status; the linked family audits do not by themselves prove every signature.
   `OCIO_FORMAT_METADATA_AUDIT.md` and `OCIO_CONTEXT_AUDIT.md` record class-level
   signature and ownership mappings. `OCIO_NAMED_TRANSFORM_AUDIT.md` records the
   corresponding named-transform mapping and its validation limits;
   `OCIO_VIEW_TRANSFORM_AUDIT.md` maps the view-transform signatures and
   ownership. `OCIO_IMAGE_DESC_AUDIT.md` maps all three image descriptor
   declarations, including raw pointer getters and native diagnostic output.
   `OCIO_CONFIG_PROCESSOR_AUDIT.md` maps config processor-construction
   overloads and distinguishes editable and read-only config paths.
   Other API families remain open.
   In particular, the current name-based L3 class list excludes `ImageDesc`,
   `PackedImageDesc`, `PlanarImageDesc`, `GpuShaderCreator`, the base
   `Transform`, and the `DynamicProperty` class family. These are not proven
   covered by an L3 pass: image layout safety, creator virtual dispatch, base
   transform operations, and each dynamic-property subtype need explicit
   signature and behavior inventories. `OCIO_EXCEPTION_AUDIT.md` maps both
   native exception types and the checked bridge's error-slot semantics;
   legacy fallback paths still need per-API review.
   Read-only transform validation now dispatches directly to the native subtype
   without an editable copy. Tests cover a valid matrix and an invalid file
   transform retained after its parent is dropped; this is not a full audit of
   the base transform or its subclasses. The sealed editable handle trait
   also exposes native validation, virtual editable copying, normalized type
   tags and evaluation-direction updates without requiring the caller to
   downcast to a specific transform wrapper. The base `Transform` declarations
   at 2.4.1, 2.5.2 and 2.6.0 also include stream insertion, now exposed as
   diagnostic text on editable and const handles. This is not a serialization
   contract, and native stream support for arbitrary subclasses is not implied.
   The CPU processor's existing in-place and separate-source/destination
   native `ImageDesc` entry points are now documented for C++ interop. They
   borrow descriptors, bypass the Rust slice layout checks, and have a manual
   packed/planar fixture. Standalone metadata inspection reports dimensions,
   signed strides, pixel format and packed/planar/custom layout without
   returning pixel addresses. The Windows fixture checks native packed and
   planar descriptors on 2.4.1 and 2.6.0. Rust can also construct owned native
   packed and planar descriptors over exclusively borrowed, checked byte
   slices. These constructors reject layouts requiring a separate tail; the
   existing slice-based CPU APIs still handle those layouts. Safe CPU methods
   can reuse the checked descriptors for in-place processing and conversion
   without reconstructing the native objects. The native processor checks
   dimensions and bit depths before touching pixels. The raw interop fixture
   also checks native virtual dispatch using a C++ subclass with irregular
   channel offsets. Rust can construct that layout with a callback-free native
   adapter over one borrowed slice; per-channel offsets, alignment and
   pointer-step bounds are checked before handing it to OCIO. The unsafe
   `ImageDescProvider` trait separately supports Rust-defined base virtual
   getters with explicit layout, ownership, and callback contracts. Native
   `ImageDesc` has no independent per-channel strides.
2. Resolve missing capabilities in priority order: sound ownership and mutation,
   native calls without an equivalent Rust entry point, then release verification.
   For GPU creators, account for clone, lifecycle, texture policy and dynamic
   cache ID behavior through native calls and the existing hooks. Additional
   Rust subclass emulation is not a completion requirement.
   A `noexcept` native getter cannot propagate a Rust failure as a C++ exception;
   any proposed callback design must define failure handling and detached-context
   behavior before implementation.
   GPU creator regression coverage now distinguishes a scoped native virtual
   clone from a persistent configuration fork. Forks recreate the enabled
   hooks over independently owned Rust state and start without generated
   resources; neither operation establishes arbitrary subclass parity.
   Audit packed-image pointer stepping as well: in OCIO 2.4.1--2.6.0,
   generic image packing advances each channel pointer after the last pixel.
   For a tightly sized packed allocation, the final non-first-channel pointer
   may be formed beyond one-past the allocation even though it is not
   dereferenced. Packed in-place, packed-to-packed and mixed planar conversions
   now use one-pixel fallbacks for unpadded layouts, retaining the native
   tight-RGBA fast path.
   Resolve this without assuming padding outside the caller's slice or imposing
   a full-image copy on the common path; verify native fast and generic paths.
   The one-dimensional `CPUProcessor` RGB/RGBA float and packed-bit-depth
   entry points handle short non-packed tails in small aligned stack buffers;
   RGB retains the native version-dependent missing-alpha behavior. Tight RGBA
   stays on the native fast path. Continue auditing other processor entry points
   and extreme dimensions beyond the covered image-layout paths.
   The separate single-row packed bit-depth conversion API uses an aligned
   one-pixel fallback for unpadded RGB or non-tight RGBA tails; its normal
   batch path remains unchanged.
   The native packed-layout optimization narrows RGBA channel and x strides to
   `int` before testing tight packing. Affected entry points conservatively
   reject strides outside that range, except that the single-pixel float and
   packed-byte APIs normalize them when representable as `ptrdiff_t`. A future
   safe split path should support other extreme layouts without misclassification.
   Single-row scalar and packed APIs also reject a native row-stride product
   exceeding `ptrdiff_t`, even when the last accessed sample alone could fit.
   Supporting such extreme layouts requires splitting before descriptor creation.
   Image entry points conservatively reject a scanline wider than `INT_MAX / 4`
   because native generic scanline loops multiply `int` pixel indices by four.
   This also limits optimized layouts that may not allocate that buffer; support
   for larger rows requires safely chunking them rather than omitting the guard.
3. For each family, test representative native behavior and error paths at its
   minimum supported version and at 2.6, including feature gates and lifetime
   guarantees. Add regression cases to existing suites; test version-specific
   ABI differences against actual native builds, not just Cargo features.
4. Recheck application-facing docs, migration guidance and examples, then run
   the release-candidate gates below on the exact proposed commit. A release or
   full-coverage claim requires recorded results and explicit resolution of every
   inventoried gap, including intentional limitations and their rationale.

## Validation placement and cost

- **Main/develop pushes and PRs:** existing cross-platform stub suites, lint/docs/parity,
  and Fedora's installed 2.4 system-library regression suite. PRs additionally
  run representative 2.6 native tests and the existing Arch zlib-ng integration.
  The 2.6 PR selection exercises new-version behavior, app helpers, CPU image
  layouts, config I/O proxies, baking, grading splines, and GPU descriptors with the highest interface level,
  plus context cache invalidation, metadata-handle validity, group topology,
  configuration and OCIOZ archive round trips,
  and checked RGB/hue curve snapshot updates, not all combinations. The curve
  tests reuse the existing 2.6 PR job; no additional job or matrix is added.
  The existing 2.6 job also runs the small dynamic-property behavior suite,
  covering native type checks, shared curves, and RGB/hue mutation without a
  separate job. The const named-item suite also covers shared look/view transforms and
  interchange snapshots without adding a separate job.
  Feature-branch pushes are checked through their PR rather than triggering a
  duplicate push workflow; branches without a PR can use manual dispatch.
- **Scheduled and manual:** exact minimum-version builds, full bundled platform
  matrix, all three interface levels against 2.6, and separate installed 2.6
  shared-library discovery/runtime. The existing 2.5.1 minimum-version job also
  runs focused baseline tests against 2.5.0, whose GPU and enum ABI differs from
  both adjacent supported versions; the runtime version is asserted explicitly.
  These are also appropriate release-candidate
  gates. Reuse existing workflow jobs instead of adding a matrix for each API.
- **Manual / relevant change / release candidate:** actual Vulkan hardware
  comparisons, direct-C++ performance baselines, sanitizer and packaging/offline
  release audits. Hardware and performance results depend on the machine and
  are not substituted by a software-rendered CI pass.

Tests should be placed in existing subsystem suites and have deterministic,
small fixtures. Escalate a check into PR CI when it covers a high-risk regression
that cheaper checks do not catch; move expensive cross-products to periodic or
release validation when representative PR coverage remains effective. Record
unrun gates and their scope rather than presenting partial results as complete.
