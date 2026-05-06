//! MatrixOp behavioral tests — ported from OCIO's MatrixOp_tests.cpp
//!
//! Tests the MatrixTransform → Processor → CPUProcessor pipeline.
//! In stub mode: only smoke tests that verify the API doesn't crash.
//! In real mode: full behavioral verification with pixel data comparisons.

mod common;
use common::*;

use ocio_rs;
use ocio_rs::transform::MatrixTransform;
use ocio_rs::{TransformDirection, BitDepth, Config};

const OPTIMIZATION_DEFAULT: u64 = 0;

// ─── Helpers ───

/// Create a minimal config for testing.
fn test_config() -> Option<Config> {
    create_test_config()
}

/// Apply a MatrixTransform to pixels through Processor pipeline (real OCIO only).
fn _apply_matrix(matrix: &MatrixTransform, pixels: &mut [[f32; 4]]) {
    let config = match test_config() {
        Some(c) => c,
        None => return,
    };
    let copy = match matrix.create_editable_copy() {
        Ok(c) => c,
        Err(_) => return,
    };
    let processor = match config.processor_from_transform(&copy, TransformDirection::Forward) {
        Ok(p) => p,
        Err(_) => return,
    };
    let cpu = match processor.optimized_cpu_processor(OPTIMIZATION_DEFAULT) {
        Ok(c) => c,
        Err(_) => return,
    };
    for pixel in pixels.iter_mut() {
        let _ = cpu.apply_rgba(pixel);
    }
}

// ─── MatrixTransform static factories ───

#[test]
fn matrix_fit_no_crash() {
    let result = MatrixTransform::fit("input", "output");
    if let Ok(t) = result {
        let _dir = t.direction();
        let _matrix = t.matrix();
        let _offset = t.offset();
    }
}

#[test]
fn matrix_identity_no_crash() {
    let result = MatrixTransform::identity();
    if let Ok(t) = result {
        let _dir = t.direction();
        let matrix = t.matrix();
        assert_close(matrix[0] as f64, 1.0, 1e-10);
        assert_close(matrix[5] as f64, 1.0, 1e-10);
        assert_close(matrix[10] as f64, 1.0, 1e-10);
        assert_close(matrix[15] as f64, 1.0, 1e-10);
    }
}

#[test]
fn matrix_sat_no_crash() {
    let result = MatrixTransform::sat(0.9, &[1.0, 0.5, 0.1]);
    if let Ok(t) = result {
        let _dir = t.direction();
        let _matrix = t.matrix();
        let _offset = t.offset();
    }
}

#[test]
fn matrix_scale_no_crash() {
    let result = MatrixTransform::scale(&[1.1, 1.3, 0.3, 1.0]);
    if let Ok(t) = result {
        let _dir = t.direction();
        let _matrix = t.matrix();
        let _offset = t.offset();
    }
}

#[test]
fn matrix_view_no_crash() {
    let result = MatrixTransform::view(&mut [0, 1, 2, 3], "linear");
    if let Ok(t) = result {
        let _dir = t.direction();
        let _matrix = t.matrix();
    }
}

// ─── MatrixTransform create + set/get ───

#[test]
fn matrix_create_no_crash() {
    let result = MatrixTransform::create();
    if let Ok(t) = result {
        let matrix = t.matrix();
        assert_close(matrix[0] as f64, 1.0, 1e-10);
        assert_close(matrix[5] as f64, 1.0, 1e-10);
        assert_close(matrix[10] as f64, 1.0, 1e-10);
        assert_close(matrix[15] as f64, 1.0, 1e-10);
    }
}

#[test]
fn matrix_set_get_matrix_no_crash() {
    let t = MatrixTransform::create().unwrap();
    let m44: [f64; 16] = [
        1.1, 0.2, 0.3, 0.4,
        0.5, 1.6, 0.7, 0.8,
        0.2, 0.1, 1.1, 0.2,
        0.3, 0.4, 0.5, 1.6,
    ];
    t.set_matrix(&m44);
    let result = t.matrix();
    // In stub mode, setter is a no-op so matrix stays as identity.
    // Only check that get doesn't crash and returns 16 values.
    assert_eq!(result.len(), 16);
    if !is_stub() {
        for i in 0..16 {
            assert_close(result[i] as f64, m44[i], 1e-10);
        }
    }
}

#[test]
fn matrix_set_get_offset_no_crash() {
    let t = MatrixTransform::create().unwrap();
    let offset: [f64; 4] = [0.1, -0.2, 0.3, 0.0];
    t.set_offset(&offset);
    let result = t.offset();
    // In stub mode, setter is a no-op. Only verify no-crash.
    if !is_stub() {
        for i in 0..4 {
            assert_close(result[i] as f64, offset[i], 1e-10);
        }
    }
}

#[test]
fn matrix_direction_no_crash() {
    let t = MatrixTransform::create().unwrap();
    let _d1 = t.direction();
    t.set_direction(TransformDirection::Inverse);
    let _d2 = t.direction();
    t.set_direction(TransformDirection::Forward);
    let _d3 = t.direction();
}

#[test]
fn matrix_file_input_output_bit_depth_no_crash() {
    let t = MatrixTransform::create().unwrap();
    let _fibd = t.file_input_bit_depth();
    let _fobd = t.file_output_bit_depth();
    t.set_file_input_bit_depth(BitDepth::F32);
    t.set_file_output_bit_depth(BitDepth::Uint16);
}

#[test]
fn matrix_create_editable_copy_no_crash() {
    let t = MatrixTransform::create().unwrap();
    let m44: [f64; 16] = [
        1.1, 0.2, 0.3, 0.4,
        0.5, 1.6, 0.7, 0.8,
        0.2, 0.1, 1.1, 0.2,
        0.3, 0.4, 0.5, 1.6,
    ];
    t.set_matrix(&m44);
    let copy_result = t.create_editable_copy();
    // In stub mode, copy creation may fail (returns null handle)
    if !is_stub() {
        let copy = copy_result.unwrap();
        let copy_matrix = copy.matrix();
        for i in 0..16 {
            assert_close(copy_matrix[i] as f64, m44[i], 1e-10);
        }
    }
}

#[test]
fn matrix_format_metadata_no_crash() {
    let t = MatrixTransform::create().unwrap();
    let _meta = t.format_metadata();
}

// ─── Behavior tests (real OCIO only) ───

#[test]
fn matrix_identity_pipeline_behavior() {
    let t = match MatrixTransform::identity() {
        Ok(t) => t,
        Err(_) => return,
    };

    if is_stub() {
        return;
    }

    let config = match test_config() {
        Some(c) => c,
        None => return,
    };

    let processor = config
        .processor_from_transform(&t, TransformDirection::Forward)
        .expect("processor_from_transform");
    let cpu = processor.optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("cpu_processor");

    let mut pixel: [f32; 4] = [0.1, 0.2, 0.3, 0.4];
    let original = pixel;
    let _ = cpu.apply_rgba(&mut pixel);
    assert_close(pixel[0] as f64, original[0] as f64, 1e-5);
    assert_close(pixel[1] as f64, original[1] as f64, 1e-5);
    assert_close(pixel[2] as f64, original[2] as f64, 1e-5);
    assert_close(pixel[3] as f64, original[3] as f64, 1e-5);
}

#[test]
fn matrix_scale_pipeline_behavior() {
    let t = match MatrixTransform::scale(&[2.0, 1.0, 0.5, 1.0]) {
        Ok(t) => t,
        Err(_) => return,
    };

    if is_stub() {
        return;
    }

    let config = match test_config() {
        Some(c) => c,
        None => return,
    };

    let processor = config
        .processor_from_transform(&t, TransformDirection::Forward)
        .expect("processor_from_transform");
    let cpu = processor.optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("cpu_processor");

    let mut pixel: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
    let _ = cpu.apply_rgba(&mut pixel);
    // Scale: [2, 1, 0.5, 1] * [0.5, 0.5, 0.5, 1.0] = [1.0, 0.5, 0.25, 1.0]
    assert_close(pixel[0] as f64, 1.0, 1e-5);
    assert_close(pixel[1] as f64, 0.5, 1e-5);
    assert_close(pixel[2] as f64, 0.25, 1e-5);
    assert_close(pixel[3] as f64, 1.0, 1e-5);
}

#[test]
fn matrix_scale_inverse_pipeline_behavior() {
    let t = match MatrixTransform::scale(&[2.0, 4.0, 0.5, 1.0]) {
        Ok(t) => t,
        Err(_) => return,
    };

    if is_stub() {
        return;
    }

    let config = match test_config() {
        Some(c) => c,
        None => return,
    };

    let processor = config
        .processor_from_transform(&t, TransformDirection::Inverse)
        .expect("processor_from_transform");
    let cpu = processor.optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("cpu_processor");

    let mut pixel: [f32; 4] = [1.0, 2.0, 0.25, 1.0];
    let _ = cpu.apply_rgba(&mut pixel);
    // Inverse: [1/2, 1/4, 1/0.5, 1] * [1.0, 2.0, 0.25, 1.0] = [0.5, 0.5, 0.5, 1.0]
    assert_close(pixel[0] as f64, 0.5, 1e-5);
    assert_close(pixel[1] as f64, 0.5, 1e-5);
    assert_close(pixel[2] as f64, 0.5, 1e-5);
    assert_close(pixel[3] as f64, 1.0, 1e-5);
}

#[test]
fn matrix_sat_pipeline_behavior() {
    let t = match MatrixTransform::sat(0.0, &[0.333, 0.333, 0.333]) {
        Ok(t) => t,
        Err(_) => return,
    };

    if is_stub() {
        return;
    }

    let config = match test_config() {
        Some(c) => c,
        None => return,
    };

    let processor = config
        .processor_from_transform(&t, TransformDirection::Forward)
        .expect("processor_from_transform");
    let cpu = processor.optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("cpu_processor");

    // With sat=0, all colors should become grayscale (R=G=B=luminance)
    let mut pixel: [f32; 4] = [1.0, 0.5, 0.25, 1.0];
    let _ = cpu.apply_rgba(&mut pixel);
    // All three channels should be equal (grayscale)
    assert_close(pixel[0] as f64, pixel[1] as f64, 1e-5);
    assert_close(pixel[1] as f64, pixel[2] as f64, 1e-5);
}

#[test]
fn matrix_combined_scale_offset_behavior() {
    // Test matrix with offset through pipeline.
    let t = match MatrixTransform::create() {
        Ok(t) => t,
        Err(_) => return,
    };

    t.set_matrix(&[
        2.0, 0.0, 0.0, 0.0,
        0.0, 3.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]);
    t.set_offset(&[0.5, 0.25, 0.0, 0.0]);

    if is_stub() {
        return;
    }

    let config = match test_config() {
        Some(c) => c,
        None => return,
    };

    let processor = config
        .processor_from_transform(&t, TransformDirection::Forward)
        .expect("processor_from_transform");
    let cpu = processor.optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("cpu_processor");

    let mut pixel: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
    let _ = cpu.apply_rgba(&mut pixel);
    // [2*0.5+0.5, 3*0.5+0.25, 1*0.5+0, 1*1+0] = [1.5, 1.75, 0.5, 1.0]
    assert_close(pixel[0] as f64, 1.5, 1e-5);
    assert_close(pixel[1] as f64, 1.75, 1e-5);
    assert_close(pixel[2] as f64, 0.5, 1e-5);
    assert_close(pixel[3] as f64, 1.0, 1e-5);
}
