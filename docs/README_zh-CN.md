# ocio-rs

[![CI](https://github.com/shaloong/ocio-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/shaloong/ocio-rs/actions)
[![crates.io](https://img.shields.io/crates/v/ocio-rs)](https://crates.io/crates/ocio-rs)
[![docs.rs](https://img.shields.io/docsrs/ocio-rs)](https://docs.rs/ocio-rs)
[![license](https://img.shields.io/crates/l/ocio-rs)](LICENSE)

[OpenColorIO](https://opencolorio.org/) v2.5.1 的 Rust 绑定。

> [English](../README.md)

```toml
[dependencies]
ocio-rs = "0.1"
```

## 构建

默认 **stub 模式**：无需 OCIO 即可编译和测试，所有 API 返回安全的默认值。

```bash
cargo build
cargo test
```

启用真实 OCIO（三选一）：

```bash
# bundled：自动 cmake 编译子模块中的 OCIO
cargo build --features bundled

# 预装 OCIO
OCIO_INSTALL_DIR=/path/to/ocio cargo build

# 源码目录
OCIO_SOURCE_DIR=/path/to/ocio cargo build
```

## 架构

```text
ocio-rs/
├── ocio-sys/          C++ 桥接层 • stub/real 双模式 • 自动生成
├── src/               安全 Rust 封装
├── tools/generator/   代码生成器（从 OCIO 头文件生成桥接代码）
├── tests/             集成测试
└── benches/           性能基准
```

## 兼容性

| ocio-rs | OCIO |
|---------|------|
| 0.1.x | v2.5.1 |

OCIO 版本升级：更新子模块 → 运行代码生成器 → 修复差异 → 发版。

## 许可

[BSD-3-Clause](LICENSE)，与上游 OpenColorIO 保持一致。

OpenColorIO 是 Academy Software Foundation 的商标。本项目与 ASWF 无关。
