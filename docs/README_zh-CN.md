# ocio-rs

[![CI](https://github.com/shaloong/ocio-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/shaloong/ocio-rs/actions)
[![crates.io](https://img.shields.io/crates/v/ocio-rs)](https://crates.io/crates/ocio-rs)
[![docs.rs](https://img.shields.io/docsrs/ocio-rs)](https://docs.rs/ocio-rs)
[![license](https://img.shields.io/crates/l/ocio-rs)](LICENSE)

[OpenColorIO](https://opencolorio.org/) 的 Rust 绑定。

当前版本面向 OpenColorIO v2.5.2。OCIO 2.5 的主要 safe wrapper 与 bundled
real-OCIO 构建链路已经基本到位，但发布加固与长尾行为验证仍在继续。

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

# 使用预装的 OCIO
OCIO_RS_ENABLE_REAL=1 OCIO_INSTALL_DIR=/path/to/ocio cargo build
```

`OCIO_SOURCE_DIR` 目前只在 bundled 构建路径内被消费；单独设置它不会启用真实
OCIO 模式。

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
