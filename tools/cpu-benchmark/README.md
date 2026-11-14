# Native C++ versus Rust CPU baseline

This unpublished, isolated tool compares calls through the safe Rust wrapper
against a minimal C ABI trampoline calling OpenColorIO C++ directly. Both
processors use the same linked OCIO library, a channel-swap matrix, F32 RGBA
buffers, and the default CPU processor. It validates output before timing,
warms up each path, alternates their order, and reports the median of seven
rounds for one pixel, 16 pixels, and 1024 pixels. The swap is stable under
repeated application, avoiding allocations or buffer resets in the timed loop.

The direct path still includes one Rust-to-C ABI call per iteration; it is not
a measurement of an inlined pure-C++ caller. Both paths include a minimal
success check. Processor creation and initial allocation are outside the timed
region. Results are diagnostic, not a pass/fail performance threshold or a
claim that other transforms, input sizes, machines, or builds have the same
overhead. Use a quiet machine, repeat the run, and compare release builds.

The tool requires OCIO 2.6 source for the `bundled` feature and an installed
matching OCIO include directory containing the generated `OpenColorABI.h`.
It checks that the compiled headers' version equals the linked runtime.
On Windows, the bundled benchmark assumes static OCIO linking. Example:

```powershell
$env:OCIO_SOURCE_DIR = 'C:\path\to\OpenColorIO-2.6.0'
$env:OCIO_BENCH_INCLUDE_DIR = 'C:\path\to\matching\ocio-install\include'
cargo run --release --locked --manifest-path tools/cpu-benchmark/Cargo.toml
```

The GPU and CPU validation tools have separate Cargo workspaces and lockfiles;
neither introduces a production dependency in `ocio-rs`.
