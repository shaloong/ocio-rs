//! CPUProcessor behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after unit-level smoke coverage.
//! In bundled/real mode they validate packed/pixel execution paths and stride
//! handling for representative matrix processors.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::MatrixTransform;
use ocio_rs::{BitDepth, CPUProcessor, Processor, TransformDirection};

fn cpu_processor_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn scaled_cpu_processor(scale: [f64; 4]) -> Option<CPUProcessor> {
    let config = create_test_config()?;
    let transform = MatrixTransform::scale(&scale).ok()?;
    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()?;
    processor.default_cpu_processor().ok()
}

fn scaled_processor(scale: [f64; 4]) -> Option<Processor> {
    let config = create_test_config()?;
    let transform = MatrixTransform::scale(&scale).ok()?;
    config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()
}

fn f32s_to_bytes(values: &[f32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}

fn bytes_to_f32s(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

#[test]
fn cpu_rgba_pixels_stride_preserves_padding_behavior() {
    let _guard = cpu_processor_test_lock();
    if is_stub() {
        return;
    }

    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");

    // Two RGBA pixels with one padding float after each pixel (stride = 5).
    let mut rgba = vec![
        0.25f32, 0.5, 0.75, 1.0, 99.0, //
        0.4, 0.2, 0.8, 1.0, 77.0,
    ];

    cpu.apply_rgba_pixels(&mut rgba, 2, 5);

    assert_close(rgba[0] as f64, 0.5, 1e-6);
    assert_close(rgba[1] as f64, 0.5, 1e-6);
    assert_close(rgba[2] as f64, 0.375, 1e-6);
    assert_close(rgba[3] as f64, 1.0, 1e-6);
    assert_close(rgba[4] as f64, 99.0, 1e-6);

    assert_close(rgba[5] as f64, 0.8, 1e-6);
    assert_close(rgba[6] as f64, 0.2, 1e-6);
    assert_close(rgba[7] as f64, 0.4, 1e-6);
    assert_close(rgba[8] as f64, 1.0, 1e-6);
    assert_close(rgba[9] as f64, 77.0, 1e-6);
}

#[test]
fn cpu_rgb_pixels_stride_preserves_padding_behavior() {
    let _guard = cpu_processor_test_lock();
    if is_stub() {
        return;
    }

    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");

    // Two RGB pixels with one padding float after each pixel (stride = 4).
    let mut rgb = vec![
        0.25f32, 0.5, 0.75, 99.0, //
        0.4, 0.2, 0.8, 77.0,
    ];

    cpu.apply_rgb_pixels(&mut rgb, 2, 4);

    assert_close(rgb[0] as f64, 0.5, 1e-6);
    assert_close(rgb[1] as f64, 0.5, 1e-6);
    assert_close(rgb[2] as f64, 0.375, 1e-6);
    assert_close(rgb[3] as f64, 99.0, 1e-6);

    assert_close(rgb[4] as f64, 0.8, 1e-6);
    assert_close(rgb[5] as f64, 0.2, 1e-6);
    assert_close(rgb[6] as f64, 0.4, 1e-6);
    assert_close(rgb[7] as f64, 77.0, 1e-6);
}

#[test]
fn cpu_rgba_packed_f32_matches_rgba_path_behavior() {
    let _guard = cpu_processor_test_lock();
    if is_stub() {
        return;
    }

    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");

    let packed_input = vec![0.25f32, 0.5, 0.75, 1.0, 0.4, 0.2, 0.8, 1.0];
    let mut packed_bytes = f32s_to_bytes(&packed_input);

    cpu.apply_rgba_packed_bit_depth(&mut packed_bytes, BitDepth::F32, 2, 4);

    let packed_output = bytes_to_f32s(&packed_bytes);
    assert_eq!(packed_output.len(), packed_input.len());
    assert_close(packed_output[0] as f64, 0.5, 1e-6);
    assert_close(packed_output[1] as f64, 0.5, 1e-6);
    assert_close(packed_output[2] as f64, 0.375, 1e-6);
    assert_close(packed_output[3] as f64, 1.0, 1e-6);
    assert_close(packed_output[4] as f64, 0.8, 1e-6);
    assert_close(packed_output[5] as f64, 0.2, 1e-6);
    assert_close(packed_output[6] as f64, 0.4, 1e-6);
    assert_close(packed_output[7] as f64, 1.0, 1e-6);
}

#[test]
fn cpu_rgb_packed_f32_matches_rgb_path_behavior() {
    let _guard = cpu_processor_test_lock();
    if is_stub() {
        return;
    }

    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");

    let packed_input = vec![0.25f32, 0.5, 0.75, 0.4, 0.2, 0.8];
    let mut packed_bytes = f32s_to_bytes(&packed_input);

    cpu.apply_rgb_packed_bit_depth(&mut packed_bytes, BitDepth::F32, 2, 3);

    let packed_output = bytes_to_f32s(&packed_bytes);
    assert_eq!(packed_output.len(), packed_input.len());
    assert_close(packed_output[0] as f64, 0.5, 1e-6);
    assert_close(packed_output[1] as f64, 0.5, 1e-6);
    assert_close(packed_output[2] as f64, 0.375, 1e-6);
    assert_close(packed_output[3] as f64, 0.8, 1e-6);
    assert_close(packed_output[4] as f64, 0.2, 1e-6);
    assert_close(packed_output[5] as f64, 0.4, 1e-6);
}

#[test]
#[should_panic(expected = "CPUProcessor::apply_rgba_pixels: buffer too small")]
fn cpu_rgba_pixels_short_buffer_panics() {
    if is_stub() {
        return;
    }

    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");
    let mut rgba = vec![0.0f32; 7];
    cpu.apply_rgba_pixels(&mut rgba, 2, 4);
}

#[test]
#[should_panic(expected = "CPUProcessor::apply_rgb_packed_bit_depth: buffer too small")]
fn cpu_rgb_packed_short_buffer_panics() {
    if is_stub() {
        return;
    }

    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");
    let mut packed_bytes = vec![0u8; 20];
    cpu.apply_rgb_packed_bit_depth(&mut packed_bytes, BitDepth::F32, 2, 3);
}

#[test]
#[should_panic(expected = "Processor::apply_rgba_pixels: buffer too small")]
fn processor_rgba_pixels_short_buffer_panics() {
    if is_stub() {
        return;
    }

    let processor = scaled_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled processor");
    let mut rgba = vec![0.0f32; 7];
    processor.apply_rgba_pixels(&mut rgba, 2, 4);
}
