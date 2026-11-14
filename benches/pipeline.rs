//! Performance benchmarks for the OCIO-rs pipeline.
//!
//! Measures overhead of Rust wrapper calls through the transform-processor-apply chain.
//! In stub mode: measures Rust-side overhead only (C++ stubs are no-ops).
//! With real OCIO: measures full pipeline including actual color processing.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
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
            cpu.apply_rgba(black_box(&mut pixel));
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
            cpu.apply_rgba(black_box(&mut pixel));
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
            cpu.apply_rgba(black_box(&mut pixel));
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

fn bench_ocio_2_6(c: &mut Criterion) {
    #[cfg(feature = "v2_6")]
    if !ocio_rs::is_stub_build() {
        use ocio_rs::{transform::FixedFunctionTransform, Config, FixedFunctionStyle};
        let config = Config::raw().unwrap();
        let transform = FixedFunctionTransform::create_with_params(
            FixedFunctionStyle::AcesRgbToHmj20,
            &[0.7347, 0.2653, 0.0, 1.0, 0.0001, -0.0770, 0.32168, 0.33767],
        )
        .unwrap();
        let cpu = config
            .processor_from_transform(&transform, TransformDirection::Forward)
            .unwrap()
            .default_cpu_processor()
            .unwrap();
        let input = [0.2, 0.4, 0.6, 1.0].repeat(1024);
        let mut group = c.benchmark_group("ocio_2_6_hmj");
        group.throughput(criterion::Throughput::Elements(1024));
        group.bench_function("batch_1024", |b| {
            b.iter_batched_ref(
                || input.clone(),
                |pixels| {
                    cpu.try_apply_rgba_pixels(black_box(pixels), 1024, 4)
                        .unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
        group.bench_function("per_pixel_1024", |b| {
            b.iter_batched_ref(
                || input.clone(),
                |pixels| {
                    for pixel in pixels.chunks_exact_mut(4) {
                        cpu.try_apply_rgba(black_box(pixel.try_into().unwrap()))
                            .unwrap();
                    }
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    #[cfg(not(feature = "v2_6"))]
    let _ = c;
}

criterion_group!(
    benches,
    bench_matrix_pipeline,
    bench_matrix_scale,
    bench_full_pipeline,
    bench_batch_pixels,
    bench_ocio_2_6,
);
criterion_main!(benches);
