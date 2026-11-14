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
| `getCacheID() const noexcept` | `GpuShaderDesc::try_cache_id` | Custom cache-ID implementation is not exposed. |
| `begin(uid)`, `finalize()`, `end()` | Descriptor methods; synchronous `ShaderExtractionHooks::lifecycle` notifications | Hooks observe lifecycle but do not replace native methods. |
| `set/getTextureMaxWidth`, `set/getAllowTexture1D` | Descriptor getters/setters and extraction template | Cannot implement these virtual methods independently in Rust. |
| `addUniform(name, DoubleGetter/BoolGetter/Float3Getter) -> bool` | Manual live Rust getters and native-backed double-property uniforms; `ShaderExtractionHooks::uniform`; post-extraction `GpuUniformCache` | Hook is a registration notification: Rust cannot supply native registration results or directly own the OCIO getter during registration. |
| `addUniform(name, SizeGetter, VectorFloatGetter/VectorIntGetter[, maxSize]) -> bool` | Manual live array getters and uniform notifications; maximum size is reported on native 2.5+ | Same registration-control gap; native 2.4 has no `maxSize` argument. |
| `addDynamicProperty`, count, index/type lookup and presence | Descriptor `add_dynamic_property` and dynamic-property access; `add_uniform_dynamic_double` binds a stored scalar property without a Rust callback | A fully custom creator's dynamic-property store is not exposed as Rust creator hooks. |
| `addTexture(...)`, `add3DTexture(...)` | Descriptor insertion; texture callback; optional streaming sink | 2.4 returns `void`, 2.5+ returns a binding. Rust may choose streamed storage and 2.5+ binding but cannot replace all creator methods. |
| `addToDeclareShaderCode(code)` (2.4) | `add_to_declare_shader_code` | On native 2.5+ it maps to parameter declarations for non-Vulkan languages; split methods are required for Vulkan. |
| `addToParameterDeclareShaderCode`, `addToTextureDeclareShaderCode` (2.5+) | Descriptor methods and code-fragment hooks (`v2_5`) | Hook rewrites fragments but does not provide arbitrary native subclassing. |
| Helper, function header/body/footer append methods | Descriptor methods and code-fragment hooks | Hook rewrites fragments but does not provide arbitrary native subclassing. |
| `createShaderText` (five sections in 2.4, six in 2.5+) | `create_shader_text`, `ShaderExtractionHooks::assemble` | Caller supplies any necessary Vulkan uniform-block handling when using a custom assembler. |
| Descriptor uniform/texture enumeration, data, offsets and bindings | Typed descriptor queries, borrowed texture slices and uniform cache | Descriptor semantics apply; streamed textures are intentionally not enumerated. |

## Design constraints for closing the creator gap

- A Rust creator must be able to decide each native registration result and own
  or delegate resource storage, including all scalar and array getter overloads.
- Several OCIO getters capture raw pointers to creator-owned dynamic properties.
  A retained getter therefore needs explicit creator/property ownership, cannot
  outlive it, and must not become safely transferable across threads while the
  property can be mutated without synchronization.
- Callbacks must not unwind through C++; early failure and cleanup must disable
  borrowed callback contexts before any retained native object can outlive the
  call. Reentrancy and OCIO's processor lock need explicit contracts.
- The 2.4 and 2.5+ virtual method ABIs differ. Test both actual native versions,
  not only a feature flag against one build, and compare generated shader and
  resources with OCIO's native descriptor. Cover failure, duplicate resources,
  property lifetime, binding conflicts and actual GPU execution where relevant.
