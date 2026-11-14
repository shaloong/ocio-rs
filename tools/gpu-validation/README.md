# Hardware GPU validation

This independent, unpublished tool compares Rust-generated OCIO 2.6 Vulkan
GLSL running on a physical GPU with the OCIO CPU processor. It checks four
RGBA pixels for a matrix transform and for two dynamically updated exposure
values on the same compiled pipeline, including native uniform-buffer offsets.
It rejects software and
virtual adapters. This is a targeted integration test, not comprehensive GPU,
texture, language, driver, or color-pipeline coverage.

The tool requires Vulkan, a suitable physical GPU, a C++ toolchain, CMake,
and OCIO 2.6 source usable by the `bundled` feature. It uses a separate Cargo
workspace and lockfile so its GPU dependencies do not enter the library's
production dependency graph. Run from the repository root:

```sh
cargo run --locked --manifest-path tools/gpu-validation/Cargo.toml
```

Set `OCIO_SOURCE_DIR` to an OpenColorIO 2.6 source checkout if the bundled
source is not already available. For example, in PowerShell:

```powershell
$env:OCIO_SOURCE_DIR = 'C:\path\to\OpenColorIO-2.6.0'
cargo run --locked --manifest-path tools/gpu-validation/Cargo.toml
```

The emitted GLSL is parsed and validated by wgpu/Naga before execution through
Vulkan; a Naga rejection is not by itself proof that an OCIO shader is invalid
for other GLSL compilers. The program exits nonzero on missing hardware, shader
or GPU errors, unexpected resource layouts, or CPU/GPU differences above 1e-5.
