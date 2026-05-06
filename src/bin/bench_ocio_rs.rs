//! Rust OCIO pixel processing benchmark — for comparison with C++ baseline.
//! Mirrors third_party/bench_ocio_cpp.cpp exactly.
//!
//! Usage:
//!   OCIO_SOURCE_DIR=path/to/ocio cargo run --release --bin bench_ocio_rs

use std::time::Instant;
use ocio_rs;
use ocio_rs::transform::MatrixTransform;
use ocio_rs::TransformDirection;

const OPTIMIZATION_DEFAULT: u64 = 0;

fn main() {
    if ocio_rs::is_stub_build() {
        eprintln!("This benchmark requires real OCIO, not stub mode.");
        eprintln!("Set OCIO_SOURCE_DIR or OCIO_INSTALL_DIR.");
        std::process::exit(1);
    }

    let config = ocio_rs::Config::raw().expect("Config::raw()");
    let mtx = MatrixTransform::create().expect("MatrixTransform::create()");
    mtx.set_matrix(&[
        2.0, 0.0, 0.0, 0.0,
        0.0, 3.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]);
    mtx.set_offset(&[0.5, 0.25, 0.0, 0.0]);

    let processor = config
        .processor_from_transform(&mtx, TransformDirection::Forward)
        .expect("processor_from_transform");
    let cpu = processor
        .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("cpu_processor");

    let n = 1_000_000;

    // Single pixel benchmark
    {
        let mut pixel: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        let t0 = Instant::now();
        for _ in 0..n {
            let _ = cpu.apply_rgba(&mut pixel);
        }
        let elapsed = t0.elapsed();
        println!(
            "Rust single-pixel: {:.1} ms ({} iters, {:.1} ns/pixel)",
            elapsed.as_secs_f64() * 1000.0,
            n,
            elapsed.as_secs_f64() * 1_000_000_000.0 / n as f64
        );
    }

    // Batch benchmark
    {
        let batch_size = 1024;
        let mut pixels: Vec<f32> = vec![0.5; batch_size * 4];
        let t0 = Instant::now();
        for _ in 0..n / batch_size {
            cpu.apply_rgba_pixels(&mut pixels, batch_size as i64, 4);
        }
        let elapsed = t0.elapsed();
        println!(
            "Rust batch: {:.1} ms ({} pixels in {}-pixel batches)",
            elapsed.as_secs_f64() * 1000.0,
            n,
            batch_size
        );
    }
}
