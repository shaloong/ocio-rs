# GPU creator interface audit

Scope: public `GPUProcessor`, `GpuShaderCreator`, and `GpuShaderDesc`
declarations in the OpenColorIO 2.4.2 and 2.6.0 headers. This is a manual
signature-family audit, not a claim that every combination or behavior has
been verified. The `v2_5` Cargo feature controls access to 2.5+ Rust APIs;
the bridge selects the native ABI separately.
The 59-test GPU descriptor suite also passed with the native 2.5.2 library on
Windows; this does not establish hardware execution or full creator parity.

| Native operation | Current Rust surface | Remaining issue |
| --- | --- | --- |
| `GPUProcessor::extractGpuShaderInfo(GpuShaderDescRcPtr&)` | `GPUProcessor::try_extract_shader_info` | Behavioral tests cover representative transforms, not the full transform matrix. |
| `GPUProcessor::extractGpuShaderInfo(GpuShaderCreatorRcPtr&)` | `GpuShaderDesc::try_borrow_creator`, `GPUProcessor::try_extract_to_creator`, extraction hooks and unsafe native interop | The typed handle retains OCIO shared ownership and exclusively borrows its descriptor. Hooks are not an arbitrary Rust implementation. |
| `clone() const -> GpuShaderCreatorRcPtr` | `GpuShaderCreator::try_clone_creator`, `GpuShaderDesc::clone_desc` | This invokes the native virtual clone. OCIO's descriptor clone returns a generic descriptor and does not reproduce every subclass resource; Rust cannot yet supply its own clone implementation. |
| Unique ID, language, function name, pixel name, resource prefix, next resource index | `GpuShaderDesc` getters/setters | Concrete descriptor access only; no custom creator implementation. |
| Descriptor set and texture binding start | `GpuShaderDesc::try_set_descriptor_set_index`, `try_descriptor_set_index`, `try_texture_binding_start` (`v2_5`) | Absent in native 2.4. |
| `getCacheID() const noexcept` | `GpuShaderCreator::try_cache_id`, `GpuShaderDesc::try_cache_id`; custom ID via `ShaderCreatorOptions` and `set_custom_cache_id` | No arbitrary dynamic callback. The caller must invalidate its ID when configuration or output changes; native cloning does not preserve this override, and changing it does not rename existing resources. |
| `begin(uid)`, `finalize()`, `end()` | Lifecycle notifications; `ShaderFinalizeControl` optionally replaces finalization | Begin/end remain notifications; this is not full native subclassing. |
| `set/getTextureMaxWidth`, `set/getAllowTexture1D` | Creator virtual access, synchronous hooks and `PersistentPolicyCreator` with retained Rust policy | Only the texture-policy virtuals are persistent; arbitrary Rust subclassing and custom clone remain absent. Policy widths must avoid OCIO's known LUT packing bug. |
| `addUniform(name, DoubleGetter/BoolGetter/Float3Getter) -> bool` | Manual getters; `ShaderUniformControl` can retain the live OCIO getter, choose descriptor or external storage, and report added/duplicate | Full custom subclassing is still absent; external uniform layouts are the caller's responsibility. |
| `addUniform(name, SizeGetter, VectorFloatGetter/VectorIntGetter[, maxSize]) -> bool` | Manual live array getters and `ShaderUniformControl`; maximum size is reported on native 2.5+ | Native 2.4 has no `maxSize` argument. Caller-owned layouts and buffers require separate management. |
| `addDynamicProperty`, count, index/type lookup and presence | Descriptor `add_dynamic_property` and dynamic-property access; `add_uniform_dynamic_double` binds a stored scalar property without a Rust callback | These methods are non-virtual in OCIO; they use its creator-owned store rather than a replaceable subclass hook. |
| `addTexture(...)`, `add3DTexture(...)` | Descriptor insertion; texture callback; streaming sink; per-texture `ShaderTextureAction` | 2.4 returns `void`, 2.5+ returns a binding. Other creator virtual methods remain non-customizable. |
| `addToDeclareShaderCode(code)` (2.4) | `add_to_declare_shader_code` | On native 2.5+ it maps to parameter declarations for non-Vulkan languages; split methods are required for Vulkan. |
| `addToParameterDeclareShaderCode`, `addToTextureDeclareShaderCode` (2.5+) | Descriptor methods and code-fragment hooks (`v2_5`) | Hook rewrites fragments but does not provide arbitrary native subclassing. |
| Helper, function header/body/footer append methods | Descriptor methods and code-fragment hooks | Hook rewrites fragments but does not provide arbitrary native subclassing. |
| `createShaderText` (five sections in 2.4, six in 2.5+) | `create_shader_text`, `ShaderExtractionHooks::assemble`; default delegates to OCIO | Caller supplies any necessary Vulkan uniform-block handling when returning custom assembled text. |
| Descriptor uniform/texture enumeration, data, offsets and bindings | Typed descriptor queries, borrowed texture slices and uniform cache; `GpuShaderCreator::try_borrow_desc` gives scoped access when the creator is a descriptor | Descriptor semantics apply; streamed textures are intentionally not enumerated. The scoped view does not expose `DerefMut`, preventing the descriptor handle from being moved out. |

## Design constraints for closing the creator gap

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
  not yet a complete custom creator or custom clone.

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
  arbitrary native creators with independent policies. Fixing the native
  algorithm itself and guarding arbitrary native creators requires further work.
  The manual Vulkan tool exercises both checked descriptor extraction and the
  explicit probe before hooks, including a width-4, five-entry packed LUT; it
  compares GPU pixels with OCIO CPU output without adding a PR CI job.

- `ShaderUniformControl` can decide registration results and own or delegate
  uniform storage for all scalar and array getter overloads. A complete Rust
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
