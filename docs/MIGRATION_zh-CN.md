# 迁移说明

## 0.2.0

`0.2.0` 是 alpha 阶段的破坏性清理：安全 Rust 层优先对齐 OpenColorIO 2.5.2 的真实模型，不继续保留此前生成层里的错误兼容 shim。

`0.2.0` 应被视为"基本可用的早期版本"：核心 OCIO 2.5 Rust 绑定已广泛就绪、有 C++ bridge 支撑、可供早期用户使用，但更深层的边界情况可靠性工作仍在后续版本中继续。它尚不能被当作所有 C++ OpenColorIO 工作流或所有边缘生产环境的直接替代品。

### GPU shader descriptor

`GpuShaderDesc` 现在暴露类型化的 GPU 资源：

- 使用 `textures_2d()` / `texture_2d(index)` 获取 1D/2D texture 元数据和值。
- 使用 `textures_3d()` / `texture_3d(index)` 获取 3D texture 元数据和值。
- 使用 `uniforms()` / `uniform(index)` 获取 uniform 元数据和值。
- `texture_values(index)` 现在返回拥有所有权的 `Vec<f32>`。
- 数量、binding index、uniform buffer size 等接口返回明确的 Rust 整数类型，不再返回指针形状的兼容值。

bundled 真实 OCIO 验证现在也会覆盖真实 shader 提取。按当前 Rust wrapper 语义：

- language、function name、pixel name、resource prefix 等描述符配置会在安全 API 中正常往返；
- 提取出的 shader text、uniform、texture 会通过结构化 accessor 做一致性验证；
- `clone_desc()` 会保留描述符配置，但不应被视作“已经提取好的 shader payload 的深拷贝”。

### ViewTransform

`ViewTransform` 已收敛到 OCIO 的真实定义：

- 用 `ViewTransform::create(ReferenceSpaceType)` 创建。
- 用 `transform(direction)` 和 `set_transform(transform, direction)` 按 `ViewTransformDirection` 读写 transform。
- category 相关操作使用 `has_category`、`add_category`、`remove_category`、`num_categories`、`category(index)`、`clear_categories`。

display/view/looks/rule 相关 helper 不再属于 `ViewTransform`。这类映射请使用 `DisplayViewTransform`。

### Processor metadata

`ProcessorMetadata` 现在作为独立的安全 Rust wrapper 存在，不再与 `FormatMetadata` 混淆。
使用 `Processor::processor_metadata()` 获取；bundled 覆盖验证了独立的 file/look 变更以及从真实
processor 提取元数据的行为。

### Config 和文件规则

`Config::processor`、`Config::processor_with_context`、`Config::processor_from_configs` 现在调用真实 OCIO processor API。

`Config::serialize()` 和 `Config::archive()` 在 linked real OCIO build 时返回真实 OCIO 文本输出。在 stub 模式下返回 `None`。

`color_space_from_filepath_by_ref_type` 替换为：

```rust
let (color_space, rule_index) = config
    .color_space_from_filepath_with_rule_index(path)
    .expect("file rules should resolve a color space");
```

这与 OCIO 的 file rules 模型一致，会同时返回解析出的 color space 名称和命中的 rule index。

旧的 `get_color_space_from_filepath_by_ref_type()` 入口仅作为 deprecated compatibility alias 保留。需要 file-rule 解析细节时优先使用 `color_space_from_filepath_with_rule_index()`，仅需解析后的 color space 名称时使用 `color_space_from_filepath()`。

`Config` 中旧的 `get_processor_v*` 重载命名也在逐步淘汰，改用更清晰的 Rust 名称。迁移时优先使用 `processor()`、`processor_with_context()`、`processor_display()`、`processor_display_with_context()`、`processor_from_transform_default_direction()`、`processor_from_transform()`、`processor_from_transform_with_context()`、`processor_named_transform()`、`processor_named_transform_with_context()`、`processor_named_transform_name()` 和 `processor_named_transform_name_with_context()`。

### Grading 和 LUT

`GradingPrimaryTransform::create(style)` 与 `GradingToneTransform::create(style)` 现在会按指定 style 创建真实 OCIO transform。`value()` / `set_value()` 会复制真实 OCIO 字段，不再返回默认零值。

`GradingRGBCurveTransform::create(style)` 与 `GradingHueCurveTransform::create(style)` 现在也会创建真实 OCIO transform。Hue 曲线现在使用 `HueCurveType` 与 `HSYTransformStyle`，不再复用 `RGBCurveType`，也不再暴露不属于 OCIO Hue 曲线模型的 `bypass_lin_to_log` helper。`DynamicProperty::grading_hue_curve_*` 系列方法也改为使用 `HueCurveType`。

`Lut1DTransform::set_length` 与 `Lut3DTransform::set_grid_size` 现在通过 C ABI 传递明确的整数值。

动态属性这条链现在也有了 bundled 真实运行时验证：

- `Processor::dynamic_property(DynamicPropertyType::Exposure)` 暴露的是创建 `CPUProcessor` 时的动态曝光种子值；
- 每个创建出来的 `CPUProcessor` 在创建后拥有自己的运行时动态属性状态，并会真实影响 CPU 像素结果。

CPU 执行 helper 也已经超出 smoke test 覆盖范围。`CPUProcessor::apply_rgb(a)_pixels`
与 `CPUProcessor::apply_rgb(a)_packed_bit_depth(..., BitDepth::F32, ...)` 现在会对真实
matrix processor 做 bundled 行为验证，包括带 padding 的 RGB/RGBA buffer 上的 stride 保持行为。

### FixedFunction 和 LogCamera

`FixedFunctionStyle` 现在严格匹配 OpenColorIO 2.5.2 的枚举值。此前如果持久化过整数枚举值，需要迁移旧整数，不要直接复用。

`FixedFunctionTransform::create_with_params` 现在遵循 OCIO 的参数校验。只有接受参数的 style 才能传参数，例如 `FixedFunctionStyle::Rec2100Surround` 需要一个 gamma 参数。

`LogCameraTransform::create(lin_side_break_values)` 现在会把 break values 传给 `LogCameraTransform::Create`，参数不再被忽略。

### BuiltinConfigRegistry

`BuiltinConfigRegistry::config_by_name` 和 `config_by_index` 现在返回通过 `Config::CreateFromBuiltinConfig` 创建出的真实 `Config`。

如果需要内置 config 的原始 YAML 文本，请使用 `config_yaml_by_name` 或 `config_yaml_by_index`。

### Bundled 构建

Windows bundled 构建现在强制使用 Release CMake profile，并优先链接 Release transitive libraries，避免 Rust 测试运行 bundled OCIO 时遇到 Debug CRT 不匹配。

Stub 模式仍为默认（不带 flag 的 `cargo build`）。真实 OCIO 模式通过
设置 `OCIO_RS_ENABLE_REAL=1` 并指定 `OCIO_INSTALL_DIR`，或使用
`--features bundled` 来启用。单独设置 `OCIO_SOURCE_DIR` 不会启用真实
OCIO 模式。已发布的 `ocio-sys` crate 包含上游 OpenColorIO 源码树和传递依赖，
打包后的 crate 通过 `cargo build --features bundled --offline` 验证。

### Baker 输出

`Baker::bake()` 现在真正将参数视为 Rust 空间中的文件系统路径。OCIO stream 输出
先通过 `Baker::bake_to_string()` 收集，再由 Rust 写入磁盘。旧的 ABI 接线将路径指针
传入 `ostream*` 槽位，在 real OCIO 模式下不可靠。

### GroupTransform 写入

`GroupTransform` 现在有安全的 `write_to_string(&Config, format_name)` helper，
用于 OCIO 的序列化 transform 输出。旧的低级 `write(...)` 入口仍为 `unsafe`，
供需要直接提供原始 ABI 对象的调用者使用。

### Grading 曲线值

`GradingRGBCurveTransform` 和 `GradingHueCurveTransform` 现在暴露安全的
Rust 值快照：

- 使用 `value()` 读取曲线数据到 Rust 结构体。
- 使用 `set_value(&...)` 从 Rust 结构体替换曲线数据。

旧的原始 handle accessor 现在明确标记为 raw/deprecated escape hatch，
不再作为主 API 呈现。

### Raw ABI escape hatch

多个仍需 OCIO 拥有的外部对象的 API 仍然可用，但在 Rust 层中现在明确
标记为 deprecated/raw escape hatch，而非看似普通安全 wrapper。

包括：viewing-rules handles、config-IO proxy handles、旧版 context-var
指针重载、原始 CPU image-descriptor 入口，以及原始 GPU shader-creator 提取。

附加的兼容别名如 `GradingPrimaryTransform::copy_value`、
`GradingPrimaryTransform::set_value_from_f64`、
`GradingToneTransform::copy_value` 和
`GradingToneTransform::set_value_from_f64` 也已 deprecated，
改用更清晰的 `value()` / `set_value(...)` 方法。

同样的清理现在也适用于 processor wrapper 中剩余的 C 风格兼容入口。
优先使用 `CPUProcessor::apply_rgb_packed_bit_depth` /
`apply_rgba_packed_bit_depth`（配合 `BitDepth` 枚举）、
`CPUProcessor::dynamic_property` / `has_dynamic_property_kind`
（配合 `DynamicPropertyType`），以及结构化的 `GpuShaderDesc` accessor
如 `uniform()`、`uniforms()`、`texture_2d()`、`textures_2d()`、
`texture_3d()` 和 `textures_3d()`。

`GpuShaderDesc` 中旧的 `get_*` / `copy*` 兼容 helper 也遵循同样的模式。
优先使用 `uniform_value_count()`、`uniform_values_f32()`、
`uniform_values_i32()`、`texture_value_count()`、
`texture_3d_value_count()`、`texture_3d_values()` 和
`texture_3d_shader_binding_index()`。

动态属性发现也在向类型化枚举靠拢。迁移 `0.2` 调用点时，优先使用
`Processor::has_dynamic_property_kind()` 和
`CPUProcessor::has_dynamic_property_kind()`（配合 `DynamicPropertyType`），
而非基于原始整数的检查。

Display/view 元数据 helper 也遵循同样的路径。优先使用
`display_view_rule()`、`display_view_description()`、
`default_view_transform_name()` 等，而非对应的 `get_*` 形式。

同样的清理现在也适用于长期存在的 transform metadata 别名。
`format_metadata_v1()` / `format_metadata_v2()` 和
`GroupTransform::get_transform_v1()` 仍然可用于兼容，但应被视为
`format_metadata()` 和 `transform()` 的 deprecated alias。

`GroupTransform` 本身现在遵循 wrapper 层其余部分的命名风格：
优先使用 `transform(index)` 而非 `get_transform()` / `get_transform_v1()`。

Processor 和 config 枚举 wrapper 也在向更清晰的 Rust 名称收敛。
优先使用 `Processor::optimized_processor_bitdepth()`、
`Processor::optimized_cpu_processor_bitdepth()`、
`GPUProcessor::extract_shader_info()`、`Config::num_color_spaces()`、
`Config::color_space_name_by_index()`、`Config::num_named_transforms()` 和
`Config::named_transform_name_by_index()`，而非旧的 `*_v1` 兼容别名。

Display/view 配置遵循同样的模式。优先使用
`Config::default_view()`、`num_views()`、`view()`、`add_display()`、
`add_display_view_detailed()`、`virtual_display_num_views()` 和
`virtual_display_view()`，而非旧的 `get_*_v1`、`get_*_v2` 和
`add_display_view_v1/v2` 兼容名称，除非你确实需要那些 OCIO-shaped 重载。

active-display 和 display-enumeration helper 也遵循同样路径：
优先使用 `active_display()`、`active_view()`、`num_displays_all()`、
`display_all()` 和 `display_all_index()`，而非旧的 `get_active_*` 和
`get_*_all` 名称。

Multi-config processor 重载现在也有描述性 Rust 名称。
优先使用 `processor_from_color_spaces()`、
`processor_from_configs_with_contexts()`、
`processor_from_configs_with_interchange()` 等，而非旧的
`get_processor_v1` 和 `get_processor_from_configs_v*` 编号。

同样的命名清理现在也适用于常见查找 helper。优先使用
`color_space()`、`color_space_index()`、`look()`、`named_transform()`、
`named_transform_index()`、`view_transform()`、
`processor_to_builtin_color_space()` 和
`processor_from_builtin_color_space()`，而非旧的 `get_*` 变体。

Crate 级别和工具入口也遵循同样模式。优先使用
`current_config()`、`Config::config_io_proxy()`、
`Context::config_io_proxy()`、`Baker::num_formats()`、
`Baker::format_name_by_index()` 和
`Baker::format_extension_by_index()`，而非对应的 `get_*` 兼容名称。

Config 中需要外部 OCIO config、interchange、view 或 virtual-view 描述符
指针的 helper 仍可用于 ABI 互操作，但不再作为普通 Rust 原生 API 呈现。

### Legacy GPU 优化

`Processor::optimized_legacy_gpu_processor()` 仍然可用于 OCIO 旧版 LUT-baking
GPU 路径，但在 Rust API 中已标记为 deprecated。OCIO 2.5 工作流优先使用
`optimized_gpu_processor()` 或 `default_gpu_processor()`。
