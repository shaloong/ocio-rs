use ocio_rs::transform::{GradingRGBCurveTransform, MatrixTransform};
use ocio_rs::{
    CPUProcessor, Config, GradingStyle, RGBCurveType, SharedGradingRGBCurve, TransformDirection,
};
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
    fn ocio_bench_curve_create() -> *mut c_void;
    fn ocio_bench_curve_destroy(handle: *mut c_void);
    fn ocio_bench_curve_read(handle: *mut c_void, copy: bool, out: *mut f32) -> bool;
}

struct Native(NonNull<c_void>);
impl Drop for Native {
    fn drop(&mut self) {
        unsafe { ocio_bench_destroy(self.0.as_ptr()) }
    }
}
struct NativeCurve(NonNull<c_void>);
impl Drop for NativeCurve {
    fn drop(&mut self) {
        unsafe { ocio_bench_curve_destroy(self.0.as_ptr()) }
    }
}

fn curve_native(curve: &NativeCurve, copy: bool) -> Result<f32, Box<dyn std::error::Error>> {
    let mut result = 0.0;
    if !unsafe { ocio_bench_curve_read(curve.0.as_ptr(), copy, &mut result) } {
        return Err("native curve access failed".into());
    }
    Ok(black_box(result))
}
fn curve_rust(curve: &SharedGradingRGBCurve, mode: u8) -> Result<f32, Box<dyn std::error::Error>> {
    let result = match mode {
        2 => curve
            .create_editable_copy()?
            .curve(RGBCurveType::Red)?
            .slope(0)?,
        1 => curve.slope(RGBCurveType::Red, 0)?,
        _ => curve.curve_ref(RGBCurveType::Red)?.slope(0)?,
    };
    Ok(black_box(result))
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
    let transform = GradingRGBCurveTransform::create(GradingStyle::Lin)?;
    let curve = SharedGradingRGBCurve::from_transform(&transform)?;
    let native_curve = NativeCurve(
        NonNull::new(unsafe { ocio_bench_curve_create() }).ok_or("native curve creation failed")?,
    );
    for mode in [0, 1, 2] {
        let copy = mode == 2;
        if curve_native(&native_curve, copy)? != curve_rust(&curve, mode)? {
            return Err("native and Rust curve values differ".into());
        }
        let mut native_times = Vec::new();
        let mut rust_times = Vec::new();
        for round in 0..7 {
            for rust_first in [round % 2 == 0, round % 2 != 0] {
                for _ in 0..100 {
                    if rust_first {
                        curve_rust(&curve, mode)?;
                    } else {
                        curve_native(&native_curve, copy)?;
                    }
                }
                let start = Instant::now();
                for _ in 0..100_000 {
                    if rust_first {
                        curve_rust(&curve, mode)?;
                    } else {
                        curve_native(&native_curve, copy)?;
                    }
                }
                let ns = start.elapsed().as_secs_f64() * 1e9 / 100_000.0;
                if rust_first {
                    rust_times.push(ns);
                } else {
                    native_times.push(ns);
                }
            }
        }
        let native_ns = median(&mut native_times);
        let rust_ns = median(&mut rust_times);
        println!(
            "curve {}: C++ {native_ns:.2} ns/call, Rust {rust_ns:.2} ns/call, ratio {:.3}x",
            match mode {
                0 => "borrow+read",
                1 => "single-call read",
                _ => "copy+read",
            },
            rust_ns / native_ns
        );
    }
    let view = curve.curve_ref(RGBCurveType::Red)?;
    let count = view.num_control_points()?;
    let mut bulk = vec![[0.0; 3]; count];
    let mut individual = bulk.clone();
    view.copy_points_into(0, &mut bulk)?;
    for (index, point) in individual.iter_mut().enumerate() {
        let (x, y) = view.control_point(index)?;
        *point = [x, y, view.slope(index)?];
    }
    if bulk != individual {
        return Err("bulk and individual curve reads differ".into());
    }
    let mut bulk_times = Vec::new();
    let mut individual_times = Vec::new();
    for round in 0..7 {
        for bulk_first in [round % 2 == 0, round % 2 != 0] {
            for _ in 0..100 {
                if bulk_first {
                    view.copy_points_into(0, black_box(&mut bulk))?;
                } else {
                    for (index, point) in individual.iter_mut().enumerate() {
                        let (x, y) = view.control_point(index)?;
                        *point = [x, y, view.slope(index)?];
                    }
                }
            }
            let start = Instant::now();
            for _ in 0..100_000 {
                if bulk_first {
                    view.copy_points_into(0, black_box(&mut bulk))?;
                } else {
                    for (index, point) in individual.iter_mut().enumerate() {
                        let (x, y) = view.control_point(index)?;
                        *point = [x, y, view.slope(index)?];
                    }
                    black_box(&individual);
                }
            }
            let ns = start.elapsed().as_secs_f64() * 1e9 / 100_000.0;
            if bulk_first {
                bulk_times.push(ns);
            } else {
                individual_times.push(ns);
            }
        }
    }
    println!(
        "{count} curve points: per-point {:.2} ns, bulk {:.2} ns",
        median(&mut individual_times),
        median(&mut bulk_times)
    );
    let legacy = GradingRGBCurveTransform::create(GradingStyle::Lin)?;
    let checked = GradingRGBCurveTransform::create(GradingStyle::Lin)?;
    let snapshot = legacy.value()?;
    legacy.set_value(&snapshot)?;
    checked.set_value_checked(&snapshot)?;
    if legacy.value()? != checked.value()? {
        return Err("curve write results differ".into());
    }
    let mut old_times = Vec::new();
    let mut checked_times = Vec::new();
    for round in 0..7 {
        for is_checked in [round % 2 == 0, round % 2 != 0] {
            for _ in 0..20 {
                if is_checked {
                    checked.set_value_checked(black_box(&snapshot))?;
                } else {
                    legacy.set_value(black_box(&snapshot))?;
                }
            }
            let start = Instant::now();
            for _ in 0..1000 {
                if is_checked {
                    checked.set_value_checked(black_box(&snapshot))?;
                } else {
                    legacy.set_value(black_box(&snapshot))?;
                }
            }
            let ns = start.elapsed().as_secs_f64() * 1e6;
            if is_checked {
                checked_times.push(ns);
            } else {
                old_times.push(ns);
            }
        }
    }
    println!("RGB snapshot write: incremental {:.2} ns, checked bulk {:.2} ns (different validation semantics)", median(&mut old_times), median(&mut checked_times));
    Ok(())
}
