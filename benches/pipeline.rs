//! Performance benchmarks for the OCIO-rs pipeline.
//!
//! Measures overhead of Rust wrapper calls through the transform-processor-apply chain.
//! In stub mode: measures Rust-side overhead only (C++ stubs are no-ops).
//! With real OCIO: measures full pipeline including actual color processing.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ocio_rs;
use ocio_rs::transform::MatrixTransform;
use ocio_rs::TransformDirection;

const OPTIMIZATION_DEFAULT: u64 = 0;

/// Benchmark: create a Config (raw built-in), create a MatrixTransform,
/// get a Processor, get a CPUProcessor, and apply to a pixel.
fn bench_matrix_pipeline(c: &mut Criterion) {
    c.bench_function("pipeline_matrix_identity", |b| {
        let config = ocio_rs::Config::raw().unwrap();
        let transform = MatrixTransform::identity().unwrap();
        let processor = config
            .processor_from_transform(&transform, TransformDirection::Forward)
            .unwrap();
        let cpu = processor
            .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
            .unwrap();
        let mut pixel: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

        b.iter(|| {
            let _ = cpu.apply_rgba(black_box(&mut pixel));
        });
    });
}

/// Benchmark: create a scale MatrixTransform and apply through pipeline.
fn bench_matrix_scale(c: &mut Criterion) {
    c.bench_function("pipeline_matrix_scale", |b| {
        let config = ocio_rs::Config::raw().unwrap();
        let transform = MatrixTransform::scale(&[2.0, 1.5, 0.5, 1.0]).unwrap();
        let processor = config
            .processor_from_transform(&transform, TransformDirection::Forward)
            .unwrap();
        let cpu = processor
            .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
            .unwrap();
        let mut pixel: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

        b.iter(|| {
            let _ = cpu.apply_rgba(black_box(&mut pixel));
        });
    });
}

/// Benchmark: end-to-end from config creation to pixel application.
fn bench_full_pipeline(c: &mut Criterion) {
    c.bench_function("pipeline_full_e2e", |b| {
        b.iter(|| {
            let config = ocio_rs::Config::raw().unwrap();
            let transform = MatrixTransform::identity().unwrap();
            let processor = config
                .processor_from_transform(black_box(&transform), TransformDirection::Forward)
                .unwrap();
            let cpu = processor
                .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
                .unwrap();
            let mut pixel: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
            let _ = cpu.apply_rgba(black_box(&mut pixel));
        });
    });
}

/// Benchmark: batch processing of many pixels.
fn bench_batch_pixels(c: &mut Criterion) {
    c.bench_function("pipeline_batch_1024_pixels", |b| {
        let config = ocio_rs::Config::raw().unwrap();
        let transform = MatrixTransform::scale(&[2.0, 1.5, 0.5, 1.0]).unwrap();
        let processor = config
            .processor_from_transform(&transform, TransformDirection::Forward)
            .unwrap();
        let cpu = processor
            .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
            .unwrap();
        let mut pixels: Vec<f32> = vec![0.5; 1024 * 4];

        b.iter(|| {
            cpu.apply_rgba_pixels(black_box(&mut pixels), 1024, 4);
        });
    });
}

criterion_group!(
    benches,
    bench_matrix_pipeline,
    bench_matrix_scale,
    bench_full_pipeline,
    bench_batch_pixels,
);
criterion_main!(benches);
