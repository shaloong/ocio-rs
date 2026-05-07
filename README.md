# ocio-rs

OpenColorIO v2 的 Rust 绑定，提供安全、零开销的 OCIO C++ API 封装。

## 架构

```text
ocio-rs/
├── ocio-sys/          # 底层 C++ 桥接层（stub/real 双模式）
│   ├── bridge.hpp     # C FFI 声明
│   ├── bridge.cpp     # C++ 实现（stub + 真实 OCIO 双模式）
│   └── lib.rs         # Rust FFI 声明
├── src/               # 安全 Rust 封装（38 个类，733+ 方法）
├── tools/generator/   # 代码生成器（自动从 OCIO 头文件生成桥接代码）
├── tests/             # 集成测试
└── benches/           # 性能基准测试
```

## 快速开始

```bash
# 克隆（含 OCIO 子模块）
git clone --recursive https://github.com/your/ocio-rs.git

# Stub 模式构建（无需 OCIO）
cargo build
cargo test

# 真实 OCIO 模式（需要先编译 OCIO）
$env:OCIO_SOURCE_DIR = "third_party/OpenColorIO"
cargo build
```

## 构建策略

- **默认 stub 模式**：无需 OCIO 源码，所有 API 返回默认值/空操作，确保编译和测试可运行
- **真实 OCIO 模式**：通过环境变量启用，静态链接完整的 OpenColorIO 库
  - `OCIO_SOURCE_DIR`：指向 OCIO 源码目录（启用 bundled 编译）
  - `OCIO_INSTALL_DIR`：指向预编译的 OCIO 安装目录
  - `OCIO_RS_ENABLE_REAL=1`：启用真实 OCIO（配合上述目录使用）

## 代码生成器

`tools/generator/` 是一个独立工具，读取 OCIO C++ 头文件自动生成桥接代码：

```bash
cd tools/generator
cargo run -- --check        # 预览模式，不写文件
cargo run                    # 生成 bridge.hpp、bridge.cpp、lib.rs
cargo run -- --ocio-path /path/to/ocio/include/OpenColorIO
```

生成器状态：stub 模式 0 错误，真实 OCIO 模式 29 个已知的类型映射边缘问题（正持续修复）。

OCIO 发新版时的工作流：更新 submodule → 运行生成器 → 修复少量差异 → 发布。

## API 覆盖

| 类 | 方法数 | 说明 |
| --- | --- | --- |
| Config | 90+ | 配置生命周期、处理器、色彩空间、显示器/视图 |
| ColorSpace | 25+ | 属性、变换、别名、类别 |
| Processor / CPUProcessor / GPUProcessor | 15+ | apply_rgba、apply_rgba_pixels、位深、缓存 ID |
| MatrixTransform | 15+ | 静态工厂（fit/identity/sat/scale/view）、矩阵/偏移 |
| CDLTransform | 15+ | Slope、offset、power、saturation、SOP |
| Lut1DTransform / Lut3DTransform | 8-12 | 值、插值、位深、域 |
| GroupTransform | 8 | 追加、前置、索引访问、清空 |
| ExposureContrastTransform | 8 | 曝光、对比度、gamma、pivot、动态属性 |
| BuiltinTransform | 10 | 样式、描述、静态样式枚举 |
| FixedFunctionTransform | 6-10 | 样式、参数 |
| FormatMetadata | 15 | 属性、子元素、名称/ID |
| DynamicProperty | 15+ | 类型化访问器（GradingPrimary/Tone/RGBCurve/HueCurve） |
| Baker / Context / FileRules / FileTransform / Look / NamedTransform / ViewTransform | 8-20 | 完整属性访问 |
| DisplayViewTransform / ColorSpaceTransform / LookTransform | 6-10 | 源/目标、方向、绕过 |
| RangeTransform / ExponentTransform / ExponentWithLinearTransform | 6-10 | 范围、值、gamma、偏移 |
| AllocationTransform | 8 | 分配、变量、方向 |
| LogTransform / LogAffineTransform / LogCameraTransform | 6-10 | 底数、斜率、偏移 |
| GradingPrimary/RGBCurve/Tone/HueCurve Transform | 8-12 | 值、样式、动态属性、控制点 |
| BuiltinConfigRegistry | 6 | 枚举内置配置 |
| GpuShaderDesc | 15 | 着色器文本、纹理、语言 |

## API 一致性检查

```bash
cargo run --bin check_parity              # 摘要报告
cargo run --bin check_parity -- --verbose # 逐类详情
cargo run --bin check_parity -- --check-l3  # 含三级检查（需 OCIO 源码）
```

三级比较：

| 级别 | 检查内容 | 用途 |
| --- | --- | --- |
| L1 | bridge.hpp ↔ lib.rs | FFI 声明不匹配 |
| L2 | bridge.hpp ↔ Rust 封装 | 缺失的安全方法 |
| L3 | OCIO C++ 头 ↔ bridge.hpp | 未桥接的 C++ 方法 |

退出码非零即表示发现差异（CI 友好）。

## 测试

```bash
cargo test --workspace           # ~345 个单元测试
cargo test --test matrix_op_tests  # 集成测试（矩阵运算验证）
```

集成测试在 stub 模式下验证 API 不崩溃，在真实 OCIO 模式下验证像素计算的正确性。

## 性能基准

```bash
cargo bench                     # Criterion 基准测试
cargo run --bin bench_ocio_rs   # 手动计时测试
```

实测结果（MatrixTransform + applyRGBA，100 万次迭代）：

| 实现 | 耗时/像素 |
| --- | --- |
| C++ 直接调用 | 7-10 纳秒 |
| Rust FFI 调用 | ~15 纳秒 |

5-8 纳秒的 FFI 开销来自 `extern "C"` 边界跨越和 shared_ptr 解引用。实际图像处理中，该开销会分摊到每次调用的百万级像素上，可忽略不计。

## 许可

本项目遵循 OCIO 上游许可条款。
