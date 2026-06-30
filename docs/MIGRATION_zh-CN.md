# 迁移说明

## 0.2.0

`0.2.0` 是 alpha 阶段的破坏性清理：安全 Rust 层优先对齐 OpenColorIO 2.5.2 的真实模型，不继续保留此前生成层里的错误兼容 shim。

### GPU shader descriptor

`GpuShaderDesc` 现在暴露类型化的 GPU 资源：

- 使用 `textures_2d()` / `texture_2d(index)` 获取 1D/2D texture 元数据和值。
- 使用 `textures_3d()` / `texture_3d(index)` 获取 3D texture 元数据和值。
- 使用 `uniforms()` / `uniform(index)` 获取 uniform 元数据和值。
- `texture_values(index)` 现在返回拥有所有权的 `Vec<f32>`。
- 数量、binding index、uniform buffer size 等接口返回明确的 Rust 整数类型，不再返回指针形状的兼容值。

### ViewTransform

`ViewTransform` 已收敛到 OCIO 的真实定义：

- 用 `ViewTransform::create(ReferenceSpaceType)` 创建。
- 用 `transform(direction)` 和 `set_transform(transform, direction)` 按 `ViewTransformDirection` 读写 transform。
- category 相关操作使用 `has_category`、`add_category`、`remove_category`、`num_categories`、`category(index)`、`clear_categories`。

display/view/looks/rule 相关 helper 不再属于 `ViewTransform`。这类映射请使用 `DisplayViewTransform`。

### Config 和文件规则

`Config::processor`、`Config::processor_with_context`、`Config::processor_from_configs` 现在调用真实 OCIO processor API。

`color_space_from_filepath_by_ref_type` 替换为：

```rust
let (color_space, rule_index) = config
    .color_space_from_filepath_with_rule_index(path)
    .expect("file rules should resolve a color space");
```

这与 OCIO 的 file rules 模型一致，会同时返回解析出的 color space 名称和命中的 rule index。

### Grading 和 LUT

`GradingPrimaryTransform::create(style)` 与 `GradingToneTransform::create(style)` 现在会按指定 style 创建真实 OCIO transform。`value()` / `set_value()` 会复制真实 OCIO 字段，不再返回默认零值。

`GradingRGBCurveTransform::create(style)` 与 `GradingHueCurveTransform::create(style)` 现在也会创建真实 OCIO transform。Hue 曲线现在使用 `HueCurveType` 与 `HSYTransformStyle`，不再复用 `RGBCurveType`，也不再暴露不属于 OCIO Hue 曲线模型的 `bypass_lin_to_log` helper。`DynamicProperty::grading_hue_curve_*` 系列方法也改为使用 `HueCurveType`。

`Lut1DTransform::set_length` 与 `Lut3DTransform::set_grid_size` 现在通过 C ABI 传递明确的整数值。

### FixedFunction 和 LogCamera

`FixedFunctionStyle` 现在严格匹配 OpenColorIO 2.5.2 的枚举值。此前如果持久化过整数枚举值，需要迁移旧整数，不要直接复用。

`FixedFunctionTransform::create_with_params` 现在遵循 OCIO 的参数校验。只有接受参数的 style 才能传参数，例如 `FixedFunctionStyle::Rec2100Surround` 需要一个 gamma 参数。

`LogCameraTransform::create(lin_side_break_values)` 现在会把 break values 传给 `LogCameraTransform::Create`，参数不再被忽略。

### BuiltinConfigRegistry

`BuiltinConfigRegistry::config_by_name` 和 `config_by_index` 现在返回通过 `Config::CreateFromBuiltinConfig` 创建出的真实 `Config`。

如果需要内置 config 的原始 YAML 文本，请使用 `config_yaml_by_name` 或 `config_yaml_by_index`。

### Bundled 构建

Windows bundled 构建现在强制使用 Release CMake profile，并优先链接 Release transitive libraries，避免 Rust 测试运行 bundled OCIO 时遇到 Debug CRT 不匹配。
