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
| `begin(uid)`, `finalize()`, `end()` | Descriptor methods; synchronous `ShaderExtractionHooks::lifecycle` notifications | Hooks observe lifecycle but do not replace native methods. |
| `set/getTextureMaxWidth`, `set/getAllowTexture1D` | Descriptor getters/setters and extraction template | Cannot implement these virtual methods independently in Rust. |
| `addUniform(name, DoubleGetter/BoolGetter/Float3Getter) -> bool` | Manual getters; `ShaderUniformControl` can retain the live OCIO getter, choose descriptor or external storage, and report added/duplicate | Full custom subclassing is still absent; external uniform layouts are the caller's responsibility. |
| `addUniform(name, SizeGetter, VectorFloatGetter/VectorIntGetter[, maxSize]) -> bool` | Manual live array getters and `ShaderUniformControl`; maximum size is reported on native 2.5+ | Native 2.4 has no `maxSize` argument. Caller-owned layouts and buffers require separate management. |
| `addDynamicProperty`, count, index/type lookup and presence | Descriptor `add_dynamic_property` and dynamic-property access; `add_uniform_dynamic_double` binds a stored scalar property without a Rust callback | These methods are non-virtual in OCIO; they use its creator-owned store rather than a replaceable subclass hook. |
| `addTexture(...)`, `add3DTexture(...)` | Descriptor insertion; texture callback; optional streaming sink | 2.4 returns `void`, 2.5+ returns a binding. Rust may choose streamed storage and 2.5+ binding but cannot replace all creator methods. |
| `addToDeclareShaderCode(code)` (2.4) | `add_to_declare_shader_code` | On native 2.5+ it maps to parameter declarations for non-Vulkan languages; split methods are required for Vulkan. |
| `addToParameterDeclareShaderCode`, `addToTextureDeclareShaderCode` (2.5+) | Descriptor methods and code-fragment hooks (`v2_5`) | Hook rewrites fragments but does not provide arbitrary native subclassing. |
| Helper, function header/body/footer append methods | Descriptor methods and code-fragment hooks | Hook rewrites fragments but does not provide arbitrary native subclassing. |
| `createShaderText` (five sections in 2.4, six in 2.5+) | `create_shader_text`, `ShaderExtractionHooks::assemble`; default delegates to OCIO | Caller supplies any necessary Vulkan uniform-block handling when returning custom assembled text. |
| Descriptor uniform/texture enumeration, data, offsets and bindings | Typed descriptor queries, borrowed texture slices and uniform cache | Descriptor semantics apply; streamed textures are intentionally not enumerated. |

## Design constraints for closing the creator gap

- `ShaderUniformControl` can decide registration results and own or delegate
  uniform storage for all scalar and array getter overloads. A complete Rust
  creator must still independently control the remaining native virtual methods.
- Several OCIO getters capture raw pointers to creator-owned dynamic properties.
  Retained getters hold the native creator alive and are thread-confined; native
  code must not access those properties concurrently without synchronization.
- Registration callback errors and panics are contained, and callback contexts
  are disabled on success and failure, even if a getter retains the native
  creator. Do not reenter extraction on the same processor from a callback.
- The 2.4 and 2.5+ virtual method ABIs differ. Test both actual native versions,
  not only a feature flag against one build, and compare generated shader and
  resources with OCIO's native descriptor. Cover failure, duplicate resources,
  property lifetime, binding conflicts and actual GPU execution where relevant.
