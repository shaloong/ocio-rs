# ocio-rs

[![CI](https://github.com/shaloong/ocio-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/shaloong/ocio-rs/actions)
[![crates.io](https://img.shields.io/crates/v/ocio-rs)](https://crates.io/crates/ocio-rs)
[![docs.rs](https://img.shields.io/docsrs/ocio-rs)](https://docs.rs/ocio-rs)
[![license](https://img.shields.io/crates/l/ocio-rs)](LICENSE)

[OpenColorIO](https://opencolorio.org/) 的 Rust 绑定。

当前版本面向 OpenColorIO v2.5.2。OCIO 2.5 的主要 safe wrapper、bundled
real-OCIO 构建链路，以及核心 C++ API 桥接面已经基本到位，但发布加固与长尾
行为验证仍在继续。

目前 bundled 验证已经不只是链接或 smoke test，下面这些高频运行时路径也有了
真实行为覆盖：

- `Config` 的 multi-config/display-view 行为，以及 virtual/shared view 元数据
- `FileRules` 的插入、regex/custom key 往返和挂载到 `Config`
- `DynamicProperty` 在 `Processor`、`CPUProcessor` 与提取后的 GPU descriptor
  之间的运行时语义
- `GpuShaderDesc` 的提取结构、资源元数据、配置往返，以及手工 shader 片段 /
  texture / uniform 插入
- `CPUProcessor` 的 packed/pixels 执行路径，包括 `RGB(A)` buffer 的 stride 保持行为

> [English](../README.md)

> 破坏性 API 变更见 [MIGRATION_zh-CN.md](MIGRATION_zh-CN.md)。

```toml
[dependencies]
ocio-rs = "0.2"
```

## 构建

**Stub 模式**（默认）：无需 OCIO 即可编译和测试。API 返回安全默认值，适合开发与 CI。

```bash
cargo build
cargo test
```

**真实 OCIO 模式**：

```bash
# 编译子模块中的 OCIO，静态链接
git clone --recursive https://github.com/shaloong/ocio-rs
cargo build --features bundled

# 将 bundled OCIO 编译为动态库
OCIO_RS_LINK=dynamic cargo build --features bundled

# 使用预装的 OCIO
OCIO_RS_ENABLE_REAL=1 OCIO_INSTALL_DIR=/path/to/ocio cargo build

# 使用预装的 OCIO 动态库，而不是静态库
OCIO_RS_ENABLE_REAL=1 OCIO_INSTALL_DIR=/path/to/ocio OCIO_RS_LINK=dynamic cargo build
```

`OCIO_SOURCE_DIR` 目前只在 bundled 构建路径内被消费；单独设置它不会启用真实
OCIO 模式。

`OCIO_RS_LINK` 默认是 `static`，以保持兼容性。如果 `OCIO_INSTALL_DIR` 指向的
OpenColorIO 安装提供动态库，可以设置为 `dynamic`（也接受 `shared` 和 `dylib`）。
运行程序时仍需确保系统 loader 能找到 OCIO 动态库，例如通过 `LD_LIBRARY_PATH`、
`DYLD_LIBRARY_PATH`、`PATH`，或包管理器提供的运行时配置。

## 架构

```text
ocio-rs/
├── ocio-sys/          C++ 桥接层 • stub/real 双模式 • 自动生成
├── src/               安全 Rust 封装
├── tools/generator/   代码生成器（从 OCIO 头文件生成）
├── tests/             集成测试
└── benches/           性能基准
```

## 兼容性

| ocio-rs | OCIO   |
| ------- | ------ |
| 0.2.0   | v2.5.2 |
| 0.1.1   | v2.5.2 |
| 0.1.0   | v2.5.1 |

OCIO 版本升级流程：更新子模块 → 运行代码生成器 → 修复编译错误 → 发版。

在生产依赖某个 API 区域之前，请先查看 [STATUS.md](../STATUS.md) 了解当前验证
范围与发布注意事项。

## 许可

[BSD-3-Clause](LICENSE)。

OpenColorIO 是 Academy Software Foundation 的商标。本项目与 ASWF 无关。
