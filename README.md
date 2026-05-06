# ocio-rs

Rust bindings for OpenColorIO v2, organized as:

- `ocio-sys`: low-level C++ bridge (761 C functions, stub/real dual mode)
- `ocio-rs` (root crate): safe Rust wrappers (38 classes, 733+ methods)

## Build strategy

This workspace targets static linking of OpenColorIO and its dependency graph.

- Default: builds in **stub mode** (compiles, but OCIO calls return no-ops/defaults)
- Set `OCIO_SOURCE_DIR` env var to enable real OCIO (statically linked)
- Or set `OCIO_INSTALL_DIR` for a prebuilt OCIO installation

```bash
# Stub mode (no OCIO source needed)
cargo check
cargo test

# Real OCIO (PowerShell)
$env:OCIO_SOURCE_DIR = "E:\path\to\OpenColorIO"
cargo test
```

## API Parity Checker

Check completeness of our Rust bindings against the OCIO C++ API:

```bash
cargo run --bin check_parity
cargo run --bin check_parity -- --verbose
cargo run --bin check_parity -- --json report.json
```

Three-level comparison:

| Level | What it checks | Input |
| ----- | -------------- | ----- |
| L1 | bridge.hpp ↔ lib.rs | FFI declaration mismatch |
| L2 | bridge.hpp ↔ Rust wrappers | Missing safe methods |
| L3 | OCIO C++ headers ↔ bridge.hpp | Unbridged C++ methods (requires `--check-l3`) |

Exit code is non-zero when gaps are found (CI friendly).

## Testing

### Unit tests (no-crash smoke tests)

```bash
cargo test --workspace          # ~345 tests
```

### Integration tests (behavioral)

```bash
cargo test --test matrix_op_tests
```

Integration tests verify actual pixel processing through the transform→processor→apply pipeline.
In stub mode they only verify API doesn't crash; with real OCIO they validate output values.

## Benchmarks

```bash
cargo bench
```

Benchmarks measure the transform→processor→apply pipeline latency for single-pixel
and batch operations. See `benches/pipeline.rs`.

## Comparing C++ vs Rust performance

To verify our Rust bindings don't introduce meaningful overhead over the C++ OCIO API,
run the same pixel-processing workload on both sides and compare timing.

### Step 1: Build the C++ benchmark

Create a small C++ program that uses OCIO directly:

```cpp
// bench_ocio_cpp.cpp
#include <OpenColorIO/OpenColorIO.h>
#include <chrono>
#include <iostream>
namespace OCIO = OCIO_NAMESPACE;

int main() {
    auto config = OCIO::Config::CreateRaw();
    auto transform = OCIO::MatrixTransform::Create();
    auto processor = config->getProcessor(transform);
    auto cpu = processor->getOptimizedCPUProcessor(OCIO::BIT_DEPTH_F32,
                                                    OCIO::BIT_DEPTH_F32,
                                                    OCIO::OPTIMIZATION_DEFAULT);
    float pixel[4] = { 0.5f, 0.5f, 0.5f, 1.0f };
    const int ITERS = 1'000'000;

    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < ITERS; ++i) {
        cpu->applyRGBA(pixel);
    }
    auto end = std::chrono::high_resolution_clock::now();
    auto us = std::chrono::duration_cast<std::chrono::microseconds>(end - start).count();
    std::cout << "C++: " << us / 1000.0 << " ms (" << ITERS << " pixels, "
              << (us * 1000.0 / ITERS) << " ns/pixel)" << std::endl;
}
```

Compile with your OCIO install:

```bash
clang++ -O2 -std=c++17 bench_ocio_cpp.cpp -lOpenColorIO -o bench_cpp
```

### Step 2: Run the Rust benchmark

```bash
OCIO_SOURCE_DIR=path/to/ocio cargo bench
```

Or for a quick manual timing test:

```bash
OCIO_SOURCE_DIR=path/to/ocio cargo run --release --example basic
```

### Step 3: Compare

Run both on the same machine with the same OCIO library version. The Rust bindings
should show negligible overhead (< 5%) compared to direct C++ calls, since the FFI
boundary is a thin `extern "C"` wrapper around the same C++ methods.

Key comparisons:

| Metric | C++ | Rust |
| ------ | --- | ---- |
| Config load | `Config::CreateRaw()` | `Config::raw()` |
| Processor build | `config->getProcessor(t)` | `config.processor_from_transform(&t)` |
| Pixel apply (1M) | `cpu->applyRGBA(pixel)` × 1M | `cpu.apply_rgba(&mut pixel)` × 1M |

## Current API surface

38 OCIO classes with safe Rust wrappers:

| Class | Methods | Notes |
| ----- | ------- | ----- |
| Config | 90+ | Full config lifecycle, processors, color spaces, looks, displays/views |
| ColorSpace | 25+ | Properties, transforms, aliases, categories |
| ColorSpaceSet | 8 | Index/name lookups (setters for individual entries not yet wrapped) |
| Processor / CPUProcessor / GPUProcessor | 15+ | apply_rgba, apply_rgba_pixels, bit depth, cache IDs |
| MatrixTransform | 15+ | Static factories (fit/identity/sat/scale/view), matrix/offset get/set |
| CDLTransform | 15+ | Slope, offset, power, saturation, SOP |
| BuiltinTransform | 10 | Style, description, static style enumeration |
| ExposureContrastTransform | 8 | Exposure, contrast, gamma, pivot, dynamic property |
| LogTransform / LogAffineTransform / LogCameraTransform | 6-10 each | Base, slopes, offsets |
| Lut1DTransform / Lut3DTransform | 8-12 each | Values, interpolation, bit depth, domain |
| GroupTransform | 8 | Append, prepend, get by index, clear |
| FormatMetadata | 15 | Attributes, children, name/id |
| DynamicProperty | 15+ | Typed getters (GradingPrimary/Tone/RGBCurve/HueCurve) |
| Baker, Context, FileRules, FileTransform, Look, NamedTransform, ViewTransform | 8-20 each | Full property access |
| DisplayViewTransform, ColorSpaceTransform, LookTransform | 6-10 each | Src/dst, direction, bypass |
| FixedFunctionTransform, RangeTransform | 6-10 each | Style, params, min/max |
| ExponentTransform, ExponentWithLinearTransform | 5-8 each | Value, gamma, offset, negative style |
| AllocationTransform | 8 | Allocation, vars, direction |
| GradingPrimary/RGBCurve/Tone/HueCurve Transform | 8-12 each | Value, style, dynamic property, control points |
| BuiltinConfigRegistry | 6 | Enumerate built-in configs |
| GpuShaderDesc | 15 | Shader text, textures, language, function name |
