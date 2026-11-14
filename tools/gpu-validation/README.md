# Hardware GPU validation

This independent, unpublished tool compares Rust-generated OCIO 2.6 Vulkan
GLSL running on a physical GPU with the OCIO CPU processor. It checks four
RGBA pixels for a matrix transform and for two dynamically updated exposure
values on the same compiled pipeline, including native uniform-buffer offsets.
It also uploads and linearly samples a non-identity 2×2×2 3D LUT and a 1D
LUT represented both as a native 1D texture and as a 2D texture. It rejects
software and virtual adapters. This is a targeted integration test, not
comprehensive GPU, texture, language, driver, or color-pipeline coverage.
It also exercises OCIO 2.6's ACES RGB↔HMJ fixed function in both directions,
using representative upstream-derived high-light and negative-channel inputs.

The tool requires Vulkan, a suitable physical GPU, a C++ toolchain, CMake,
and OCIO 2.6 source usable by the `bundled` feature. The adapter must support
filtering 32-bit float textures. It uses a separate Cargo
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
for other GLSL compilers. For LUT cases, the tool adapts OCIO's
combined sampler to separate texture/sampler bindings required by wgpu, adds
explicit LOD 0 for the compute stage, and expands RGB texels to RGBA. It does
not change OCIO's coordinates or color calculation. The program exits nonzero
on missing hardware, shader or GPU errors, unexpected resource layouts, or
CPU/GPU differences above 1e-5 for most analytic cases, 2e-5 for inverse
HMJ, or 5e-3 for filtered LUTs.
It reports the measured maximum error for each case.
