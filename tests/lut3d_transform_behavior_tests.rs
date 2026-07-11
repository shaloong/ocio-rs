//! Lut3DTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/lut3d.rs`. In bundled/real mode they validate LUT state,
//! editable-copy independence, index ordering, and real processor execution for
//! a simple separable 3D LUT with linear interpolation.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::Lut3DTransform;
use ocio_rs::{BitDepth, Interpolation, TransformDirection};

fn lut3d_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configured_lut3d_transform() -> Lut3DTransform {
    let transform = Lut3DTransform::create().expect("lut3d transform create");
    transform.set_grid_size(2).expect("set LUT grid size");
    transform
        .try_set_interpolation(Interpolation::Linear)
        .expect("set interpolation");
    transform
        .try_set_file_output_bit_depth(BitDepth::F32)
        .expect("set file output bit depth");

    for r in 0..2u64 {
        for g in 0..2u64 {
            for b in 0..2u64 {
                transform
                    .set_value(r, g, b, [2.0 * r as f32, 3.0 * g as f32, 4.0 * b as f32])
                    .expect("set LUT entry");
            }
        }
    }

    transform
}

#[test]
fn lut3d_transform_value_copy_and_direction_behavior() {
    let _guard = lut3d_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = configured_lut3d_transform();

    assert_eq!(transform.grid_size(), 2);
    assert_eq!(transform.interpolation(), Interpolation::Linear);
    assert_eq!(transform.file_output_bit_depth(), BitDepth::F32);
    assert_eq!(transform.direction(), TransformDirection::Forward);

    assert_eq!(transform.value(0, 0, 0), Some([0.0, 0.0, 0.0]));
    assert_eq!(transform.value(1, 0, 0), Some([2.0, 0.0, 0.0]));
    assert_eq!(transform.value(0, 1, 0), Some([0.0, 3.0, 0.0]));
    assert_eq!(transform.value(0, 0, 1), Some([0.0, 0.0, 4.0]));
    assert_eq!(transform.value(1, 1, 1), Some([2.0, 3.0, 4.0]));

    let copy = transform
        .create_editable_copy()
        .expect("lut3d transform editable copy");
    copy.set_value(1, 1, 1, [1.0, 1.0, 1.0])
        .expect("set LUT entry");
    copy.set_direction(TransformDirection::Inverse);
    copy.try_set_file_output_bit_depth(BitDepth::Uint16)
        .expect("set copy file output bit depth");

    assert_eq!(copy.value(1, 1, 1), Some([1.0, 1.0, 1.0]));
    assert_eq!(copy.direction(), TransformDirection::Inverse);
    assert_eq!(copy.file_output_bit_depth(), BitDepth::Uint16);

    assert_eq!(transform.value(1, 1, 1), Some([2.0, 3.0, 4.0]));
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert_eq!(transform.file_output_bit_depth(), BitDepth::F32);
}

#[test]
fn lut3d_transform_rejects_invalid_write_inputs() {
    let _guard = lut3d_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = configured_lut3d_transform();
    assert!(matches!(
        transform.set_values(&[0.0; 23]),
        Err(ocio_rs::OcioError::InvalidInput(_))
    ));
    assert!(matches!(
        transform.set_value(2, 0, 0, [1.0, 1.0, 1.0]),
        Err(ocio_rs::OcioError::InvalidInput(_))
    ));
    assert_eq!(transform.value(1, 1, 1), Some([2.0, 3.0, 4.0]));
}

#[test]
fn lut3d_transform_processor_behavior() {
    let _guard = lut3d_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = configured_lut3d_transform();

    let forward_cpu = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .expect("forward processor")
        .default_cpu_processor()
        .expect("forward cpu");
    let inverse_cpu = config
        .processor_from_transform(&transform, TransformDirection::Inverse)
        .expect("inverse processor")
        .default_cpu_processor()
        .expect("inverse cpu");

    // This 2x2x2 LUT maps each corner [r, g, b] to [2r, 3g, 4b], so with
    // linear interpolation it scales channels independently over [0, 1].
    // Keep the mapped RGB values inside the LUT cube so the inverse path stays
    // within the well-covered domain.
    let original = [0.25f32, 0.25, 0.25, 0.6];
    let mut mapped = original;
    forward_cpu.apply_rgba(&mut mapped);

    assert_close(mapped[0] as f64, 0.5, 2e-5);
    assert_close(mapped[1] as f64, 0.75, 2e-5);
    assert_close(mapped[2] as f64, 1.0, 2e-5);
    assert_close(mapped[3] as f64, original[3] as f64, 2e-5);

    let mut restored = mapped;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 5e-5);
    assert_close(restored[1] as f64, original[1] as f64, 5e-5);
    assert_close(restored[2] as f64, original[2] as f64, 5e-5);
    assert_close(restored[3] as f64, original[3] as f64, 5e-5);
}
