# GPU creator interface audit

Scope: public `GPUProcessor`, `GpuShaderCreator`, and `GpuShaderDesc`
declarations in the OpenColorIO 2.4.2 and 2.6.0 headers. This is a manual
signature-family audit, not a claim that every combination or behavior has
been verified. The `v2_5` Cargo feature controls access to 2.5+ Rust APIs;
the bridge selects the native ABI separately.
The GPU descriptor suite passed on Windows with native 2.4.1 (64 tests, system
static), 2.5.2 (80 tests, bundled), and 2.6.0 (80 tests, bundled), using their
respective baseline, `v2_5`, and `v2_6` Rust interfaces. These results do not
establish hardware execution or full
creator parity. The suite includes persistent callback failure and ownership
checks, including an opt-in Rust clone factory. An always-attached arbitrary
`getCacheID` callback remains unsupported.

| Native operation | Current Rust surface | Remaining issue |
| --- | --- | --- |
| `GPUProcessor::extractGpuShaderInfo(GpuShaderDescRcPtr&)` | `GPUProcessor::try_extract_shader_info` | Behavioral tests cover representative transforms, not the full transform matrix. |
| `GPUProcessor::extractGpuShaderInfo(GpuShaderCreatorRcPtr&)` | `GpuShaderDesc::try_borrow_creator`, `GPUProcessor::try_extract_to_creator`, extraction hooks and unsafe native interop | The typed handle retains OCIO shared ownership and exclusively borrows its descriptor. Hooks are not an arbitrary Rust implementation. |
| `clone() const -> GpuShaderCreatorRcPtr` | `GpuShaderCreator::try_clone_creator`, `GpuShaderDesc::clone_desc`, `PersistentPolicyCreator::try_clone_with_policy` | The opt-in Rust factory supplies a distinct owned creator through the native virtual method. The callback is scoped to that operation. Separately, `try_fork_with_policy` can recreate persistent configuration and hooks with a new policy, without copying shader output or resources. |
| Unique ID, language, function name, pixel name, resource prefix, next resource index | `GpuShaderCreator` and `GpuShaderDesc` getters/setters; persistent creators can update the first five settings | Creator methods dispatch through the native base without requiring descriptor conversion. Persistent configuration changes do not clear existing code/resources or automatically update a custom cache ID. |
| Descriptor set and texture binding start | `GpuShaderCreator` and `GpuShaderDesc` fallible setters and getters (`v2_5`) | Absent in native 2.4. Descriptor-backed creators retain the bridge's existing binding-range checks; external non-descriptor creators are responsible for their own resource layouts. |
| `getCacheID() const noexcept` | `GpuShaderCreator::try_cache_id`, `GpuShaderDesc::try_cache_id`; fixed ID via `ShaderCreatorOptions` and `set_custom_cache_id`; per-extraction callback via `ShaderCacheIdPolicy` | The dynamic callback is scoped to a persistent creator extraction, not an arbitrary native creator. Native cloning does not preserve these policies, and changing an ID does not rename existing resources. |
| `begin(uid)`, `finalize()`, `end()` | `GpuShaderCreator` direct virtual calls; synchronous or persistent lifecycle notifications; `ShaderFinalizeControl` optionally replaces finalization in either mode | Begin/end hooks remain notifications; this is not full Rust subclassing. |
| `set/getTextureMaxWidth`, `set/getAllowTexture1D` | Creator virtual access, synchronous hooks and `PersistentPolicyCreator` with retained Rust policy | Arbitrary Rust subclassing remains absent. Policy widths must avoid OCIO's known LUT packing bug. |
| `addUniform(name, DoubleGetter/BoolGetter/Float3Getter) -> bool` | Creator constants, dynamic double, and thread-safe Rust live getters; descriptor manual getters; synchronous or persistent `ShaderUniformControl` | Full Rust-defined custom subclassing remains absent; external uniform layouts are the caller's responsibility. |
| `addUniform(name, SizeGetter, VectorFloatGetter/VectorIntGetter[, maxSize]) -> bool` | Creator copied arrays and Rust live array getters, descriptor live array getters and `ShaderUniformControl`; maximum size is reported on native 2.5+ | Native 2.4 has no `maxSize` argument. Caller-owned layouts remain the application's responsibility. |
| `addDynamicProperty`, count, index/type lookup and presence | `GpuShaderCreator` and descriptor dynamic-property access; `add_uniform_dynamic_double` binds a stored scalar property without a Rust callback | These methods are non-virtual in OCIO; they use its creator-owned store rather than a replaceable subclass hook. The non-descriptor native fixture exercises registration, duplicate rejection and shared lifetime on native 2.4.1 and 2.6.0. |
| `addTexture(...)`, `add3DTexture(...)` | Direct `GpuShaderCreator` virtual calls, descriptor insertion, texture callback, streaming sink, per-texture `ShaderTextureAction`, persistent `ShaderTextureControl` | 2.4 returns `void`, 2.5.0 returns per-kind indices, and 2.5.1+ returns a binding. Imported native creators must not retain pointers borrowed during registration. Other Rust-defined creator virtual methods remain non-customizable. |
| `addToDeclareShaderCode(code)` (2.4) | `add_to_declare_shader_code` | On native 2.5+ it maps to parameter declarations for non-Vulkan languages; split methods are required for Vulkan. |
| `addToParameterDeclareShaderCode`, `addToTextureDeclareShaderCode` (2.5+) | Descriptor methods and code-fragment hooks (`v2_5`) | Hook rewrites fragments but does not provide arbitrary native subclassing. |
| Helper, function header/body/footer append methods | Direct `GpuShaderCreator` virtual calls, descriptor methods, and synchronous or persistent code-fragment hooks | Hook rewrites fragments but does not provide arbitrary Rust subclassing. |
| `createShaderText` (five sections in 2.4, six in 2.5+) | `create_shader_text`, synchronous and persistent `ShaderExtractionHooks::assemble`; default delegates to OCIO | Caller supplies any necessary Vulkan uniform-block handling when returning custom assembled text. |
| Descriptor `CreateShaderDesc`, `clone`, `getShaderText` | `GpuShaderDesc::create`, `try_clone_desc`, `try_shader_text` or borrowed `shader_text_ref` | The borrowed text cannot outlive or be safely mutated through its descriptor. |
| `getNumUniforms`, `getUniform`, `getUniformBufferSize` | `try_num_uniforms`, `try_uniform`, typed scalar and caller-buffer readers, `uniform_cache`, `try_uniform_buffer_size` | The buffer size and uniform offsets are only available on 2.5+; cached native getters remain live. |
| `getNumTextures`, `getTexture`, `getTextureValues`, `getTextureShaderBindingIndex` | `try_num_textures`, `try_texture_2d`, `texture_values_ref`, `texture_shader_binding_index` | The binding index is only available on 2.5+; borrowed texels retain the descriptor borrow and avoid a Rust payload copy. |
| `getNum3DTextures`, `get3DTexture`, `get3DTextureValues`, `get3DTextureShaderBindingIndex` | `try_num_3d_textures`, `try_texture_3d`, `texture_3d_values_ref`, `texture_3d_shader_binding_index` | The binding index is only available on 2.5+; streamed textures are intentionally not enumerated. |

The descriptor declarations above were checked against 2.4.1, 2.5.2 and
2.6.0. `GpuShaderCreator::try_borrow_desc` provides scoped descriptor access
only when the underlying creator is a descriptor. Its scoped view does not
expose `DerefMut`, preventing the descriptor handle from being moved out.

The legacy raw texture query declarations now describe name and value outputs
as pointer-to-pointer parameters, matching how the bridge has always written
them. This corrects the source-level FFI contract without changing the binary
calling convention. The old pointer-encoded index is checked before narrowing
to native `unsigned`, including binding-index queries. Existing native tests
exercise 1D/2D and 3D outputs and reject oversized indices on 64-bit targets;
typed texture-info and borrowed-value paths remain preferable for new code.

The high-level `texture_info` and 1D/2D and 3D binding-index convenience
queries now use the existing metadata bridge without first allocating and
copying the texture payload. `texture_info` still copies the two names into
owned Rust strings; binding-index queries do not copy names or texels. The
full `texture_2d` and `texture_3d` methods continue to return owned texels.
Metadata queries can now succeed independently of a subsequent payload read.
The allocation regression tracks large Rust allocations on the calling thread
and uses full-resource reads as a positive control. It passed with native
2.4.1 and 2.6.0; it does not measure allocations inside C++ or other threads.

`uniform_name` uses the native `getUniform` name directly instead of sampling
the value or allocating an array snapshot. The bridge creates the C++
`UniformData` locally, so callers do not have to construct native function
objects. Its returned raw name remains descriptor-owned. A live scalar getter
regression verifies name lookup does not invoke the getter; the allocation
regression also covers a large array uniform.
In contrast, an array's current value count requires invoking OCIO's size
getter. For Rust-provided live arrays this refreshes the preallocated native
snapshot by calling user code; the binding does not create a Rust-owned array
copy, but the callback or an external native getter may do additional work.

The five native uniform overload shapes are `(name, double getter)`,
`(name, bool getter)`, `(name, float3 getter)`, `(name, size getter, float
vector getter[, unsigned max size])`, and the corresponding integer vector
form. The maximum exists in 2.5+ but not 2.4; the bridge dispatches according
to the linked native ABI. The index and type dynamic-property overloads are
likewise dispatched separately and return retained native shared handles.
Existing creator regressions distinguish all five types, live callback values,
duplicate registrations and vector capacity; they do not certify arbitrary
third-party native subclasses.

Rust GPU callbacks use a common unwind boundary that also catches a panic
caused by dropping the original panic payload; it intentionally forgets the
second payload if necessary to prevent unwinding through C. A subprocess
regression covers descriptor and creator live getters, creator array getters,
getter destruction, and a persistent texture policy. GPU descriptor and
runtime-helper suites passed on Windows
with native 2.4.1 (66 and 13 tests), 2.5.2 (82 and 13), and 2.6.0 (82 and 13).
This applies to unwinding panics, not `panic=abort`, and does not certify every
third-party callback implementation.

Texture scalar counts use the native metadata query without copying texels.
The fallible count methods preserve native errors; their compatibility wrappers
return zero on failure. These queries do not validate GPU hardware execution.
OCIO 2.4.1 through 2.6.0 multiplies dimensions and channel count in `unsigned`
before widening to `size_t` when storing a texture. The bridge limits scalar
counts for descriptor storage and reads to both `unsigned` and Rust slice
`ptrdiff_t` limits. The forwarding creator checks the native-storage limit only
when retaining a texture; its stream-only callbacks retain the wider
address-space limit. Imported custom creators are checked conservatively,
including non-descriptors, because they may delegate to native storage. Raw native
interop remains subject to the native caller's own safety contract.
The low-level creator also rejects non-cubic 3D dimensions and non-unit depth
for 1D/2D textures before native dispatch. Regressions exercise these checks
without allocating huge buffers, including the unsigned upper boundary.
Owned texture and uniform snapshots use fallible reservation, including when a
dynamic uniform grows between reads. Capacity overflow and allocator-reported
reservation failures return errors; borrowed texture reads do not allocate.
Descriptor texture queries reject unknown native channel and dimension values
before calculating payload sizes rather than treating them as RGB or 1D.
Conversion boundary tests cover invalid discriminants; they do not simulate a
third-party native descriptor returning malformed metadata end to end.

## Design constraints for closing the creator gap

`GpuShaderCreator::from_native_shared_ptr` provides unsafe interop with an
existing C++ shared creator without inventing a second control block. A manual
fixture in `tools/creator-validation` uses a native non-descriptor subclass
with custom clone, extracts a matrix shader, and checks ownership after both the
original shared pointer and Rust owners are dropped. It passed on Windows with
matching native 2.4.1 and 2.6.0 static headers and runtimes. Its versioned
binding configuration also passed on 2.5.2 and 2.6.0. Lifecycle, shader-code
virtual dispatch, exception propagation, and the version-specific five/six
section assembly arguments, and 1D/2D and 3D texture registration passed on
2.4.1 and 2.6.0. The fixture is not a
Rust-defined arbitrary creator or a full GPU execution test.
Borrowed creator configuration strings and cache IDs avoid a Rust allocation;
their exclusive borrow prevents safe mutation until the reference expires.
The bridge rejects `clone()` returning the identical native object. The unsafe
native import contract additionally requires distinct clones (and descriptor
views) not to share mutable storage whose mutation could invalidate borrowed
pointers; object identity alone cannot establish this. It also requires returned
pointers to remain valid and stable across that borrow. Descriptor-derived and native
non-descriptor creator checks cover these borrows on Windows; they do not
establish a general performance benchmark.

- `PersistentPolicyCreator::try_extract_with_cache_policy` invokes an optional
  Rust cache-ID policy from OCIO's native `noexcept` getter during that call.
  The ID is borrowed from policy-owned `CStr` storage; OCIO 2.4.1, 2.5.2 and
  2.6.0 immediately copy it before another callback. Empty IDs, errors and
  panics are reported after native execution. The bridge returns a
  temporary empty key on callback failure and rejects `begin` before resource
  generation. Native state or external resources are not generally rolled back
  if a later callback fails. A scope guard detaches the cache callback on every
  exit, leaving the creator's fixed cache ID in effect. This does not implement
  a persistent arbitrary native subclass or its custom clone. Windows native
  2.4.1, 2.5.2 and 2.6.0 tests cover changing keys, callback failure/panic,
  empty IDs and reuse; hardware execution remains a release gate.
  `try_query_cache_policy` exercises the same native virtual getter without
  extraction. It copies the policy-owned result for the caller and detaches
  the callback on both success and failure, leaving the fixed ID in effect.
  `set_extraction_cache_policy` can opt subsequent ordinary extractions into
  this policy. The option lives in the Rust owner; attachment remains scoped
  to each extraction and does not change the fixed ID or existing resources.

- `PersistentPolicyCreator` keeps one native creator and Rust policy across
  calls. Rust owns the callback state; the native creator holds only its
  borrowed address, and `Drop` detaches callbacks before dropping the state.
  Callback errors and panics are captured, including the `noexcept` width
  getter; the first error is returned after native execution. Resource state
  may already have changed if extraction fails. Cache identity is explicitly
  supplied and must be changed when policy or output changes. Reusing a creator
  may append resources, and the caller remains responsible for checking LUT
  packing safety. `finish` detaches the policy and returns the generated native
  descriptor; its texture settings then use native stored values. The manual
  Vulkan validation exercises the resulting shader against the CPU. This is
  not yet a complete arbitrary custom creator. `try_fork_with_policy` creates
  a fresh native owner with the same settings and hook selection (including
  whether extraction uses a dynamic cache ID). It uses stored texture settings
  without invoking the source policy, and takes an explicit replacement policy
  and fixed cache ID. It neither copies extracted resources nor shares callback
  state with the source. The stored factory is only called when forking; no
  extra work is done during shader extraction.

- `create_with_texture_control` additionally retains a texture registration
  controller in the same policy state. It may upload borrowed texels directly,
  select descriptor or external storage, and choose bindings where supported.
  External uploads are not rolled back on failure; the caller owns cleanup.
  On OCIO 2.5+, later descriptor insertions account for external binding slots
  and avoid previously assigned bindings; overflow is rejected before mutation.
  The manual Vulkan validation covers mixed external/retained 2D and 3D LUTs
  in both registration orders, including dynamic uniforms and explicit bindings.

- `create_with_hooks` retains lifecycle notifications and code-fragment hooks
  alongside persistent resource controls. It does not install the synchronous
  assembler hook or replace native finalization. Unmodified fragments are
  forwarded; replacements are retained until native code consumes them. The
  first callback error is retained even if the subsequent `End` hook also fails.
  `create_with_assembler` additionally retains shader assembly, using the same
  ownership and error rules; its `None` result preserves native Vulkan wrapping.
  `create_with_finalizer` can replace finalization or delegate to native
  finalization and the assembler; replacement bypasses the assembler and must
  supply language-specific wrappers itself.
  `try_assemble_fragments` invokes the configured lifecycle, fragment,
  assembly and finalization hooks with caller-supplied code rather than a
  GPU processor. It attempts end after a successful begin even on error,
  preserves the first failure, and does not roll back existing state.

- `ShaderTexturePolicy` can supply the two texture-policy virtual getters on
  each query during synchronous extraction, alongside texture/uniform storage
  and finalization control. The callbacks are detached on success and failure.
  A failed or panicking width callback cannot throw through OCIO's `noexcept`
  getter: the bridge supplies a temporary width above the supported LUT limit,
  finishes or aborts native extraction, then reports the first Rust error.
  External resources created before failure remain the caller's responsibility.
  The returned descriptor resumes the template's texture settings, not the
  transient policy. `try_extract_with_texture_policy_checked` probes optimized
  texture lengths then conservatively checks every width returned by the policy;
  this generates the shader twice, may reject non-LUT resources, and assumes
  resource-generating processor state is unchanged across the two extractions.
  The unchecked variant requires callers to account for the native packing bug.
  A nonempty caller-supplied cache ID is mandatory and must change with the
  policy/output; otherwise OCIO resource keys can collide across extractions.
  Policy setter events receive template values before extraction; a rejection
  or panic aborts before native `begin`. Post-extraction descriptor setters no
  longer call Rust. Accepted values are also stored in the native descriptor.

- The native 2.4.1 and 2.6.0 LUT shader generator divides by the configured
  texture width and uses `width - 1` as a loop step, without setter validation.
  The safe setter rejects widths 0 and 1. A separate upstream padding-size bug
  remains for some widths and LUT lengths (for example width 2, length 3):
  the calculated storage size is smaller than the padded data, so an unsigned
  subtraction underflows. Prefer a maximum width at least as large as the LUT;
  that workaround does not establish safety for arbitrary generated LUTs.
  `GpuShaderDesc::validate_lut_texture_layout` checks a known LUT length against
  the native padding calculation and native arithmetic limits, without generating
  the shader. It is not automatic: optimization may create or change LUTs, and
  the public GPU processor API does not expose the resulting operation list.
  `GPUProcessor::try_extract_shader_info_checked` offers a more comprehensive,
  conservative opt-in probe of the optimized processor by first extracting
  with a width above the 1,048,576-entry native LUT limit. It generates the
  shader twice and may reject non-LUT textures. The separately callable
  `validate_shader_texture_layout` lets custom Rust hook users perform the same
  probe before extraction with unchanged template settings; it does not validate
  arbitrary native creators with independent policies. The vendored source
  now allocates enough rows for OCIO's interpolation stride. Direct extraction
  there still generates the shader once. The portable checks continue to
  apply the conservative limits of unpatched releases.
  OCIO does not expose the optimized LUT list in its public `GPUProcessor`
  API. The faulty allocation occurs before the first texture-registration
  callback, so a callback cannot intercept it in time. The direct
  `try_extract_shader_info` retains native behavior; for unpatched system or
  external-source builds use the checked variant when unknown LUT lengths may
  be packed across rows.
  The manual Vulkan tool exercises both checked descriptor extraction and the
  explicit probe before hooks, including a width-4, five-entry packed LUT; it
  compares GPU pixels with OCIO CPU output without adding a PR CI job.

- `ShaderUniformControl` can decide registration results and own or delegate
  uniform storage for all scalar and array getter overloads. The persistent
  resource-control constructor retains this control together with texture
  control. Its returned live getters retain the creator even after a failed
  extraction or wrapper destruction; external storage remains caller-owned.
  Manual Vulkan tests combine an externally retained dynamic uniform with
  external and descriptor-owned LUTs, including a property update. A complete Rust
  creator must still independently control the remaining native virtual methods.
- Several OCIO getters capture raw pointers to creator-owned dynamic properties.
  Retained getters hold the native creator alive and are thread-confined; native
  code must not access those properties concurrently without synchronization.
- Registration callback errors and panics are contained, and callback contexts
  are disabled on success and failure, even if a getter retains the native
  creator. Native `end` is called after a successful `begin` even when extraction
  fails with an OCIO exception; a failing `begin` does not receive `end`.
  Do not reenter extraction on the same processor from a callback.
- A custom finalizer sees control before OCIO's language-specific wrapper is
  applied. It must assemble any required declarations and wrapper itself; `None`
  retains native finalization. Its callback is synchronous and is detached on
  success and failure, including when an external uniform getter outlives the
  returned descriptor or a failed extraction.
- The 2.4 and 2.5+ virtual method ABIs differ. Test both actual native versions,
  not only a feature flag against one build, and compare generated shader and
  resources with OCIO's native descriptor. Cover failure, duplicate resources,
  property lifetime, binding conflicts and actual GPU execution where relevant.

## Per-texture storage design

Native 2.5.0 lacks the 2.5.1 texture-binding query virtuals and returns
per-kind texture indices from the generic descriptor. Baseline Rust texture
insertion reports no binding for that native version. The custom descriptor
uses a separate global sequence while extracting shader resources. A real
Windows bundled 2.5.0 baseline suite passed 63 GPU descriptor tests; native
2.4.1, 2.5.2 and 2.6.0 targeted suites also passed. This does not validate
2.5.0 Vulkan execution on hardware.

During texture-control or streaming callbacks on native 2.5+, proposed default
bindings skip bindings already assigned by earlier registrations, including
explicit overrides. An explicitly requested duplicate is still rejected. After
callbacks are detached, the existing descriptor insertion behavior is retained.
A regression checks the generated Vulkan binding declarations when an earlier
override occupies the next default. The full Windows static-system descriptor
suites passed with native 2.4.1 (62 tests), 2.5.2 (77 tests) and 2.6.0
(77 tests). Additional persistent and streaming regression cases passed on
2.5.2 and 2.6.0. The persistent mixed-resource case also ran on an NVIDIA
RTX 3050 Laptop GPU with Vulkan and native 2.6.0, checking both texture orders
and dynamic uniforms against CPU output (maximum error 0.000664 for those
cases). This is not coverage of other hardware or all binding layouts.

The same collision-aware proposal is also used for ordinary retained-texture
callbacks. The inner native descriptor cannot see earlier callback overrides,
so its own next binding is not necessarily available. A regression overrides
the first texture to the next slot and verifies that the second callback and
generated Vulkan declarations use the following free slot. The complete GPU
descriptor suite passed on Windows against native 2.4.1, 2.5.2 and 2.6.0;
the new binding-specific case applies to the latter two. It checks generated
code, not execution of that particular shader on a GPU.

Persistent texture control now reserves bookkeeping capacity before native
descriptor storage and commits a binding only after that storage succeeds.
Ordinary retained-texture registration also reserves both binding vectors
before native storage, preventing a bookkeeping allocation failure from
leaving a stored resource without an index. Capacity grows geometrically.
The GPU descriptor suites passed after this change on Windows with native
2.5.0 (baseline), 2.5.2 (`v2_5`) and 2.6.0 (`v2_6`); allocator failure was
not injected, and these results do not verify GPU execution.
On Windows, native 2.5.2 and 2.6.0 tests reproduce a width-limit failure and
verify that a successful retry receives the original free binding. This does
not roll back external uploads, earlier successful registrations, or other
creator state; the test does not force an allocator failure or 3D storage error.
After this change the baseline GPU descriptor suites also passed on Windows
with native 2.4.1 and 2.5.0 (64 tests each); the 2.5.0 runtime version was
asserted separately. The new retry test requires the `v2_5` interface and was
therefore exercised on 2.5.2 and 2.6.0, not on those baseline builds.

For the library's custom descriptor, default binding calculation is checked
before inserting a texture: an offset plus resource count that exceeds the
native unsigned range returns an error instead of wrapping. This protection
also applies when adding resources after extraction. For generic descriptors,
the bridge checks explicit texture insertion and binding-start changes before
mutation. This does not intercept texture insertion by OCIO during ordinary
shader extraction or by arbitrary native creators; applications using those
paths must choose binding ranges that cannot overflow. The opt-in
`try_extract_shader_info_checked` also checks the range for existing and
probed textures before extraction, provided resource generation is unchanged
between the probe and the real extraction; it may conservatively reject custom
descriptors using noncontiguous bindings.

`try_extract_with_texture_control` chooses native descriptor storage or external
storage independently for each registration, without copying external textures.
It is opt-in; existing hooks keep their original callback timing and return
values. The following constraints govern the implementation and remaining tests:

- Invoke the storage decision before asking the native descriptor to copy a
  texture. The callback receives the native data only for the duration of the
  call; external upload and rollback remain caller responsibilities.
- On 2.5+, track one logical registration sequence shared by 1D/2D and 3D
  textures, including externally stored entries. The generic OCIO descriptor
  counts only textures actually stored in it, so its proposed binding cannot be
  used as the global default after an external entry. Keep explicit per-kind
  binding maps for the stored entries and reject collisions with any earlier
  external or stored binding. Apply the chosen binding to generated code.
- On 2.4, preserve the void texture-registration ABI and reject binding
  overrides; storage selection itself does not require a binding index.
- A failed callback or duplicate binding aborts extraction, and retained
  getters must never retain the synchronous callback context. As with the
  existing streaming API, successfully uploaded external resources cannot be
  rolled back automatically.
- Test alternating stored and external 1D/2D and 3D textures, both insertion
  orders, explicit and default bindings, collisions, callback errors/panics,
  descriptor queries after extraction, retained getter lifetime, and shader
  execution. Reuse the existing native version jobs and hardware validation
  tool rather than adding a new CI matrix.
