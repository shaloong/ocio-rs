use ocio_rs::transform::MatrixTransform;
use ocio_rs::{CPUProcessor, Config, TransformDirection};
use std::ffi::{c_long, c_void};
use std::hint::black_box;
use std::ptr::NonNull;
use std::time::Instant;

unsafe extern "C" {
    fn ocio_bench_create() -> *mut c_void;
    fn ocio_bench_destroy(handle: *mut c_void);
    fn ocio_bench_apply_single(handle: *mut c_void, rgba: *mut f32) -> bool;
    fn ocio_bench_apply_batch(handle: *mut c_void, rgba: *mut f32, count: c_long) -> bool;
    fn ocio_bench_compiled_version() -> i32;
}

struct Native(NonNull<c_void>);
impl Drop for Native {
    fn drop(&mut self) {
        unsafe { ocio_bench_destroy(self.0.as_ptr()) }
    }
}

fn run_native(
    cpu: &Native,
    pixels: &mut [f32],
    single: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let ok = unsafe {
        if single {
            ocio_bench_apply_single(cpu.0.as_ptr(), black_box(pixels.as_mut_ptr()))
        } else {
            ocio_bench_apply_batch(
                cpu.0.as_ptr(),
                black_box(pixels.as_mut_ptr()),
                (pixels.len() / 4).try_into()?,
            )
        }
    };
    if !ok {
        return Err("native OCIO call failed".into());
    }
    Ok(())
}

fn run_rust(
    cpu: &CPUProcessor,
    pixels: &mut [f32],
    single: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if single {
        cpu.try_apply_rgba(black_box(pixels.try_into().map_err(|_| "not one pixel")?))?;
    } else {
        let count = (pixels.len() / 4).try_into()?;
        cpu.try_apply_rgba_pixels(black_box(pixels), count, 4)?;
    }
    Ok(())
}

fn median(samples: &mut [f64]) -> f64 {
    samples.sort_by(f64::total_cmp);
    samples[samples.len() / 2]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if ocio_rs::is_stub_build() {
        return Err("benchmark requires native OCIO".into());
    }
    let header_version = unsafe { ocio_bench_compiled_version() };
    if header_version != ocio_rs::version_hex() || header_version < 0x0206_0000 {
        return Err(
            "native benchmark headers and linked OCIO must match and be at least 2.6".into(),
        );
    }
    let matrix = MatrixTransform::identity()?;
    matrix.set_matrix(&[
        0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 1.,
    ])?;
    let cpu = Config::raw()?
        .processor_from_transform(&matrix, TransformDirection::Forward)?
        .default_cpu_processor()?;
    let native = Native(
        NonNull::new(unsafe { ocio_bench_create() }).ok_or("native processor creation failed")?,
    );
    println!("OCIO {} — native C ABI trampoline versus safe Rust wrapper; median of 7 alternating rounds", ocio_rs::version().unwrap_or_default());
    for (count, iterations, single) in [
        (1usize, 200_000, true),
        (16, 30_000, false),
        (1024, 2_000, false),
    ] {
        let input = [0.15_f32, 0.45, 0.85, 0.7].repeat(count);
        let mut reference = input.clone();
        let mut actual = input.clone();
        run_native(&native, &mut reference, single)?;
        run_rust(&cpu, &mut actual, single)?;
        if reference != actual || actual[..4] != [0.85, 0.45, 0.15, 0.7] {
            return Err("native and Rust processor results do not match the expected swap".into());
        }
        let mut native_times = Vec::new();
        let mut rust_times = Vec::new();
        for round in 0..7 {
            for rust_first in [round % 2 == 0, round % 2 != 0] {
                let mut pixels = input.clone();
                for _ in 0..100 {
                    if rust_first {
                        run_rust(&cpu, &mut pixels, single)?
                    } else {
                        run_native(&native, &mut pixels, single)?
                    }
                }
                let start = Instant::now();
                for _ in 0..iterations {
                    if rust_first {
                        run_rust(&cpu, &mut pixels, single)?
                    } else {
                        run_native(&native, &mut pixels, single)?
                    }
                }
                let ns = start.elapsed().as_secs_f64() * 1e9 / f64::from(iterations);
                black_box(&pixels);
                if rust_first {
                    rust_times.push(ns);
                } else {
                    native_times.push(ns);
                }
            }
        }
        let native_ns = median(&mut native_times);
        let rust_ns = median(&mut rust_times);
        println!("{count:>4} pixels: C++ {native_ns:.2} ns/call, Rust {rust_ns:.2} ns/call, ratio {:.3}x", rust_ns / native_ns);
    }
    Ok(())
}
