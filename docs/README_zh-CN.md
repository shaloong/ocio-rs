# ocio-rs 中文文档

OpenColorIO v2 的 Rust 绑定。提供安全、零开销的 OCIO C++ API 封装，目标是让 Rust 用户无需手动配置 OCIO 即可使用完整的色彩管理管线。

## 目录

- [项目架构](#项目架构)
- [快速开始](#快速开始)
- [构建模式](#构建模式)
- [代码生成器](#代码生成器)
- [API 覆盖](#api-覆盖)
- [版本对齐策略](#版本对齐策略)
- [开发指南](#开发指南)
- [常见问题](#常见问题)

## 项目架构

```text
ocio-rs/
├── ocio-sys/              # 底层 C++ 桥接层
│   ├── build.rs           # 构建脚本（stub/real 双模式路由）
│   ├── src/
│   │   ├── bridge.hpp     # C FFI 函数声明（~660 行，自动生成）
│   │   ├── bridge.cpp     # C++ 实现（~7000 行，自动生成）
│   │   └── lib.rs         # Rust FFI 声明（~600 行，自动生成）
│   └── ...
├── src/                   # 安全 Rust 封装
│   ├── lib.rs             # 38 个类，733+ 方法
│   ├── config.rs          # Config 封装
│   ├── transform/         # 22 种 Transform 子类
│   └── ...
├── tools/
│   └── generator/         # 独立的代码生成器（Rust 项目）
│       ├── Cargo.toml
│       └── src/main.rs    # 从 OCIO C++ 头文件生成桥接代码
├── tests/                 # 集成测试
│   ├── common/mod.rs      # 测试工具函数
│   ├── matrix_op_tests.rs # 矩阵运算验证
│   └── data/              # 测试数据（configs, CDL, CLF）
├── benches/
│   └── pipeline.rs        # Criterion 性能基准
├── docs/
│   └── README_zh-CN.md    # 本文档
└── third_party/
    └── OpenColorIO/       # Git 子模块，指向 OCIO 官方仓库
```

### 四层架构

```
┌─────────────────────────────────┐
│  src/（安全 Rust 封装）          │  ← 用户代码调用这一层
│  Config::raw(), .processor()... │
├─────────────────────────────────┤
│  lib.rs（Rust FFI 声明）        │  ← 自动生成
│  extern "C" { ... }             │
├─────────────────────────────────┤
│  bridge.cpp（C++ 桥接实现）     │  ← 自动生成
│  stub 模式 / 真实 OCIO 模式     │
├─────────────────────────────────┤
│  libOpenColorIO.a（OCIO 库）    │  ← 静态链接
└─────────────────────────────────┘
```

## 快速开始

### 环境要求

- Rust 1.70+（MSVC 或 GNU 工具链）
- Git（用于子模块）
- CMake + C++17 编译器（仅构建真实 OCIO 时需要）

### 克隆项目

```bash
git clone --recursive https://github.com/your/ocio-rs.git
cd ocio-rs
```

如果已经克隆但没有子模块：

```bash
git submodule update --init --recursive
```

### Stub 模式（推荐入门）

无需任何 OCIO 依赖，直接编译和测试：

```bash
cargo build
cargo test --workspace
cargo run --bin check_parity
```

Stub 模式下所有 API 返回默认值或空操作，但完整的 API 表面和类型安全保证仍然生效。

### 真实 OCIO 模式

首先编译 OCIO 静态库（MinGW 示例）：

```bash
cd third_party/OpenColorIO
mkdir build && cd build
cmake .. -G Ninja \
  -DBUILD_SHARED_LIBS=OFF \
  -DOCIO_BUILD_APPS=OFF \
  -DOCIO_BUILD_TESTS=OFF \
  -DOCIO_BUILD_PYTHON=OFF \
  -DCMAKE_POSITION_INDEPENDENT_CODE=ON
ninja
```

然后构建 ocio-rs：

```bash
cd ../..
$env:OCIO_SOURCE_DIR = "$PWD/third_party/OpenColorIO"
cargo build --release
cargo test --release
```

## 构建模式

### Stub 模式（默认）

- 无需任何 OCIO 依赖
- 所有 OCIO 函数返回默认值/nullptr/零值
- 编译时间 < 5 秒
- 适合开发、CI、API 验证

实现方式：`build.rs` 定义 `OCIO_RS_STUB` 宏，`bridge.cpp` 中 `#ifdef OCIO_RS_STUB` 分支使用桩实现。

### 真实 OCIO 模式

通过环境变量启用：

| 环境变量 | 用途 |
| --- | --- |
| `OCIO_RS_ENABLE_REAL=1` | 必须设置，启用真实 OCIO |
| `OCIO_SOURCE_DIR` | OCIO 源码目录（触发 cmake 编译） |
| `OCIO_INSTALL_DIR` | 预编译的 OCIO 安装目录 |
| `CARGO_FEATURE_BUNDLED` | 启用 bundled 特性（自动 cmake 编译） |

链接的传递依赖（Windows MSVC 示例）：
```
OpenColorIO → expat, yaml-cpp, Imath-3_2, pystring, minizip-ng, zlib, bz2, lzma, zstd
```

## 代码生成器

`tools/generator/` 是项目的核心基础设施。它解析 OCIO C++ 头文件，自动生成桥接代码。

### 为什么需要代码生成器

手工维护 761 个 C 函数的版本同步几乎不可能做到零遗漏。OCIO 每个版本都有 API 变化（方法改名、参数调整、类名变更）。代码生成器确保：

1. 桥接代码与 OCIO 版本精确一致
2. 新增 API 自动覆盖
3. 重命名/删除自动同步
4. 减少人工维护成本

### 使用方法

```bash
cd tools/generator

# 检查模式（只解析，不写文件）
cargo run -- --check

# 生成全部三个文件
cargo run

# 使用特定 OCIO 头文件路径
cargo run -- --ocio-path /path/to/ocio/include/OpenColorIO
```

### 当前状态

| 文件 | Stub 模式 | 真实 OCIO 模式 |
| --- | --- | --- |
| bridge.hpp | 0 错误 | — |
| bridge.cpp | 0 错误 | 29 个已知类型映射问题 |
| lib.rs | 0 错误 | — |

剩余 29 个错误都是类型映射的边缘情况（enum 名称、const 转换、输出参数引用），正在逐步修复中。

### OCIO 版本更新工作流

```bash
# 1. 更新 OCIO 子模块
cd third_party/OpenColorIO
git fetch --tags
git checkout v2.6.0  # 或最新 tag

# 2. 运行代码生成器
cd tools/generator
cargo run

# 3. 修复编译问题
cd ../..
cargo build  # 根据编译错误修复

# 4. 运行测试验证
cargo test --workspace

# 5. 提交
git add ocio-sys/src/bridge.* ocio-sys/src/lib.rs
git commit -m "regenerate bridge for OCIO v2.6.0"
```

## API 覆盖

当前已桥接 38 个 OCIO 类：

| 类 | 方法数 | 说明 |
| --- | --- | --- |
| Config | 90+ | 配置文件的完整生命周期管理 |
| ColorSpace | 25+ | 色彩空间属性、变换、别名 |
| Processor | 10+ | 色彩处理器创建和缓存管理 |
| CPUProcessor | 10+ | CPU 像素处理 |
| GPUProcessor | 5+ | GPU 着色器处理 |
| MatrixTransform | 15+ | 矩阵变换（Fit, Identity, Sat, Scale, View） |
| CDLTransform | 15+ | CDL 色彩校正 |
| BuiltinTransform | 10 | 内置变换样式枚举 |
| ExposureContrastTransform | 8 | 曝光/对比度调整 |
| LogTransform | 8 | 对数变换 |
| LogAffineTransform | 6 | 对数仿射变换 |
| LogCameraTransform | 6 | 对数相机变换 |
| Lut1DTransform | 8 | 1D LUT 变换 |
| Lut3DTransform | 12 | 3D LUT 变换 |
| GroupTransform | 8 | 变换组（追加、插入、移除） |
| RangeTransform | 6 | 范围变换 |
| FixedFunctionTransform | 6 | 固定功能变换 |
| ExponentTransform | 5 | 指数变换 |
| ExponentWithLinearTransform | 5 | 带线性的指数变换 |
| AllocationTransform | 8 | 分配变换 |
| FileTransform | 8 | 文件变换 |
| ColorSpaceTransform | 4 | 色彩空间变换 |
| DisplayViewTransform | 8 | 显示器/视图变换 |
| LookTransform | 8 | Look 变换 |
| GradingPrimaryTransform | 8 | 一级调色 |
| GradingRGBCurveTransform | 8 | RGB 曲线调色 |
| GradingToneTransform | 8 | 色调调色 |
| GradingHueCurveTransform | 8 | 色相曲线调色 |
| FormatMetadata | 15 | 格式元数据 |
| DynamicProperty | 15+ | 动态属性（类型化子类访问） |
| Baker | 20+ | 烘焙器 |
| Context | 18 | 上下文变量 |
| FileRules | 22 | 文件规则 |
| Look | 12 | Look 管理 |
| NamedTransform | 8 | 命名变换 |
| ViewTransform | 11 | 视图变换 |
| ColorSpaceSet | 12 | 色彩空间集合 |
| BuiltinConfigRegistry | 6 | 内置配置注册表 |
| GpuShaderDesc | 15 | GPU 着色器描述 |

## 版本对齐策略

### 长期通路

1. **OCIO 作为 Git 子模块**：`third_party/OpenColorIO` 固定指向 OCIO 官方仓库的特定版本
2. **代码生成器自动同步**：运行 `tools/generator` 自动从 OCIO 头文件生成桥接代码
3. **发布版本号对应**：ocio-rs `v2.4.x` 对应 OCIO `v2.4.x`

### 发版流程

```text
OCIO 发新版 → 更新子模块 → 运行生成器 → 修复 API 差异 → 
运行测试 → 更新文档 → 发布 ocio-rs 新版本
```

整个流程预计 < 1 小时（对于小版本更新），< 1 天（对于大版本更新）。

## 开发指南

### 添加新的 OCIO 类封装

1. 确保代码生成器已覆盖该类的所有方法（运行 `check_parity --check-l3`）
2. 在 `src/` 下创建对应的安全封装模块
3. 添加单元测试和集成测试

### 安全封装模式

```rust
// 构造
let config = Config::raw()?;
let transform = MatrixTransform::create()?;

// 配置
transform.set_matrix(&[2.0, 0.0, 0.0, 0.0, ...]);

// 创建处理器
let processor = config.processor_from_transform(&transform, TransformDirection::Forward)?;
let cpu = processor.optimized_cpu_processor(OptimizationFlags::default())?;

// 应用像素处理
let mut pixel: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
cpu.apply_rgba(&mut pixel)?;
```

### 运行测试

```bash
# 所有测试
cargo test --workspace

# 集成测试
cargo test --test matrix_op_tests

# 带真实 OCIO
OCIO_SOURCE_DIR=path cargo test --release

# API 一致性检查
cargo run --bin check_parity -- --check-l3
```

### 性能基准

```bash
# Criterion 基准
cargo bench

# 手动计时
cargo run --release --bin bench_ocio_rs
```

## 性能

FFI 开销约 5-8 纳秒/像素（MatrixTransform + applyRGBA）。实际使用中，批量处理可将开销摊销到每帧百万像素级别，可忽略不计。

优化建议：
- 使用 `apply_rgba_pixels()` 批量接口减少 FFI 调用次数
- 预创建 `Config` 和 `Processor` 对象，避免重复初始化

## 常见问题

### Q: 为什么不直接用 bindgen？

OCIO 是 C++ 库，bindgen 只支持 C 头文件。我们通过手工 C 桥接层（bridge.cpp）将 C++ 类封装为 C 函数，然后用代码生成器自动化该封装。

### Q: Stub 模式有什么用？

Stub 模式允许你在没有 OCIO 的环境下编译、运行测试、做 API 设计验证。所有 API 返回安全的默认值，不会崩溃。

### Q: 为什么 `cargo build` 默认是 stub？

因为 OCIO 的完整编译链（CMake、C++17、十几个传递依赖）非常重。我们希望默认体验是轻量的。

### Q: 如何切换到真实 OCIO？

设置 `OCIO_RS_ENABLE_REAL=1` 和 `OCIO_SOURCE_DIR`（或 `OCIO_INSTALL_DIR`）。

### Q: 代码生成器生成后需要手动修改吗？

大部分情况下不需要。当前有 29 个已知的类型映射边缘情况（ref 参数、抽象类等），这些在生成后需要少量手动修复。目标是将这个数字降到 0。

## 贡献

欢迎提交 PR。请确保：
1. 通过 `cargo test --workspace` 所有测试
2. 通过 `cargo run --bin check_parity` API 一致性检查
3. 如果是 API 变更，更新相关文档
