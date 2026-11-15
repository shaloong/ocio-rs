# GPU creator interface audit

Scope: public `GPUProcessor`, `GpuShaderCreator`, and `GpuShaderDesc`
declarations in the OpenColorIO 2.4.2 and 2.6.0 headers. This is a manual
signature-family audit, not a claim that every combination or behavior has
been verified. The `v2_5` Cargo feature controls access to 2.5+ Rust APIs;
the bridge selects the native ABI separately.

| Native operation | Current Rust surface | Remaining issue |
| --- | --- | --- |
| `GPUProcessor::extractGpuShaderInfo(GpuShaderDescRcPtr&)` | `GPUProcessor::try_extract_shader_info` | Behavioral tests cover representative transforms, not the full transform matrix. |
| `GPUProcessor::extractGpuShaderInfo(GpuShaderCreatorRcPtr&)` | `try_extract_with_hooks`, `try_extract_with_streamed_textures`; unsafe native-creator interop | Hooks are backed by the library's own C++ subclass, not an arbitrary Rust implementation. |
| `clone() const -> GpuShaderCreatorRcPtr` | `GpuShaderDesc::clone_desc` | Native base cloning does not reproduce every subclass resource; Rust cannot supply its own clone implementation. |
| Unique ID, language, function name, pixel name, resource prefix, next resource index | `GpuShaderDesc` getters/setters | Concrete descriptor access only; no custom creator implementation. |
| Descriptor set and texture binding start | `GpuShaderDesc::try_set_descriptor_set_index`, `try_descriptor_set_index`, `try_texture_binding_start` (`v2_5`) | Absent in native 2.4. |
| `getCacheID() const noexcept` | `GpuShaderDesc::try_cache_id`; fixed custom ID via `ShaderCreatorOptions` | No arbitrary dynamic callback. The caller must invalidate its ID when configuration or output changes; native cloning does not preserve this override. |
| `begin(uid)`, `finalize()`, `end()` | Lifecycle notifications; `ShaderFinalizeControl` optionally replaces finalization | Begin/end remain notifications; this is not full native subclassing. |
| `set/getTextureMaxWidth`, `set/getAllowTexture1D` | Descriptor getters/setters and extraction template | Cannot implement these virtual methods independently in Rust. |
| `addUniform(name, DoubleGetter/BoolGetter/Float3Getter) -> bool` | Manual getters; `ShaderUniformControl` can retain the live OCIO getter, choose descriptor or external storage, and report added/duplicate | Full custom subclassing is still absent; external uniform layouts are the caller's responsibility. |
| `addUniform(name, SizeGetter, VectorFloatGetter/VectorIntGetter[, maxSize]) -> bool` | Manual live array getters and `ShaderUniformControl`; maximum size is reported on native 2.5+ | Native 2.4 has no `maxSize` argument. Caller-owned layouts and buffers require separate management. |
| `addDynamicProperty`, count, index/type lookup and presence | Descriptor `add_dynamic_property` and dynamic-property access; `add_uniform_dynamic_double` binds a stored scalar property without a Rust callback | These methods are non-virtual in OCIO; they use its creator-owned store rather than a replaceable subclass hook. |
| `addTexture(...)`, `add3DTexture(...)` | Descriptor insertion; texture callback; streaming sink; per-texture `ShaderTextureAction` | 2.4 returns `void`, 2.5+ returns a binding. Other creator virtual methods remain non-customizable. |
| `addToDeclareShaderCode(code)` (2.4) | `add_to_declare_shader_code` | On native 2.5+ it maps to parameter declarations for non-Vulkan languages; split methods are required for Vulkan. |
| `addToParameterDeclareShaderCode`, `addToTextureDeclareShaderCode` (2.5+) | Descriptor methods and code-fragment hooks (`v2_5`) | Hook rewrites fragments but does not provide arbitrary native subclassing. |
| Helper, function header/body/footer append methods | Descriptor methods and code-fragment hooks | Hook rewrites fragments but does not provide arbitrary native subclassing. |
| `createShaderText` (five sections in 2.4, six in 2.5+) | `create_shader_text`, `ShaderExtractionHooks::assemble`; default delegates to OCIO | Caller supplies any necessary Vulkan uniform-block handling when returning custom assembled text. |
| Descriptor uniform/texture enumeration, data, offsets and bindings | Typed descriptor queries, borrowed texture slices and uniform cache | Descriptor semantics apply; streamed textures are intentionally not enumerated. |

## Design constraints for closing the creator gap

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
  Fixing or safely preflighting all packed LUTs requires further work.

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
