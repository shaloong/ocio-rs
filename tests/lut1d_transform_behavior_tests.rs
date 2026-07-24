//! Lut1DTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/lut1d.rs`. In bundled/real mode they validate LUT state,
//! editable-copy independence, and real processor execution for a simple
//! monotonic 1D LUT with linear interpolation.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::Lut1DTransform;
use ocio_rs::{BitDepth, Interpolation, TransformDirection};

fn lut1d_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configured_lut1d_transform() -> Lut1DTransform {
    let transform = Lut1DTransform::create().expect("lut1d transform create");
    transform.set_length(2).expect("set LUT length");
    transform
        .try_set_interpolation(Interpolation::Linear)
        .expect("set interpolation");
    transform
        .try_set_file_output_bit_depth(BitDepth::F32)
        .expect("set file output bit depth");
    transform
        .set_values(&[
            0.0, 0.0, 0.0, //
            2.0, 2.0, 2.0,
        ])
        .expect("set LUT values");
    transform
}

#[test]
fn lut1d_transform_value_copy_and_direction_behavior() {
    let _guard = lut1d_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = configured_lut1d_transform();

    assert_eq!(transform.length(), 2);
    assert_eq!(transform.interpolation(), Interpolation::Linear);

    // DEFAULT and BEST sit apart from the real modes in OCIO (254/255);
    // they must round-trip and not be misread as Unknown.
    for meta in [Interpolation::Default, Interpolation::Best] {
        transform
            .try_set_interpolation(meta)
            .expect("set meta interpolation mode");
        assert_eq!(transform.interpolation(), meta);
    }
    transform
        .try_set_interpolation(Interpolation::Linear)
        .expect("restore linear interpolation");

    assert_eq!(transform.file_output_bit_depth(), BitDepth::F32);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert_eq!(
        transform.try_value(0).expect("read first LUT entry"),
        Some([0.0, 0.0, 0.0])
    );
    assert_eq!(transform.value(0), Some([0.0, 0.0, 0.0]));
    assert_eq!(transform.value(1), Some([2.0, 2.0, 2.0]));
    assert_eq!(
        transform.try_value(2).expect("out-of-range LUT entry"),
        None
    );

    let values = transform.values();
    assert_vec_close(&values, &[0.0, 0.0, 0.0, 2.0, 2.0, 2.0], 1e-10);

    let copy = transform
        .create_editable_copy()
        .expect("lut1d transform editable copy");
    copy.set_value(1, [1.0, 1.0, 1.0]).expect("set LUT entry");
    copy.set_direction(TransformDirection::Inverse);
    copy.try_set_file_output_bit_depth(BitDepth::Uint16)
        .expect("set copy file output bit depth");

    assert_eq!(copy.value(1), Some([1.0, 1.0, 1.0]));
    assert_eq!(copy.direction(), TransformDirection::Inverse);
    assert_eq!(copy.file_output_bit_depth(), BitDepth::Uint16);

    assert_eq!(transform.value(1), Some([2.0, 2.0, 2.0]));
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert_eq!(transform.file_output_bit_depth(), BitDepth::F32);
}

#[test]
fn lut1d_transform_rejects_invalid_write_inputs() {
    let _guard = lut1d_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = configured_lut1d_transform();
    assert!(matches!(
        transform.set_values(&[0.0; 5]),
        Err(ocio_rs::OcioError::InvalidInput(_))
    ));
    assert!(matches!(
        transform.set_value(2, [1.0, 1.0, 1.0]),
        Err(ocio_rs::OcioError::InvalidInput(_))
    ));
    assert_eq!(
        transform.try_value(2).expect("out-of-range LUT entry"),
        None
    );
    assert_eq!(transform.value(1), Some([2.0, 2.0, 2.0]));
}

#[test]
fn lut1d_transform_processor_behavior() {
    let _guard = lut1d_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = configured_lut1d_transform();

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

    // A 2-point LUT from 0 -> 0 and 1 -> 2 doubles RGB values over [0, 1].
    let original = [0.25f32, 0.5, 1.0, 0.75];
    let mut mapped = original;
    forward_cpu.apply_rgba(&mut mapped);

    assert_close(mapped[0] as f64, 0.5, 2e-5);
    assert_close(mapped[1] as f64, 1.0, 2e-5);
    assert_close(mapped[2] as f64, 2.0, 2e-5);
    assert_close(mapped[3] as f64, original[3] as f64, 2e-5);

    let mut restored = mapped;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 5e-5);
    assert_close(restored[1] as f64, original[1] as f64, 5e-5);
    assert_close(restored[2] as f64, original[2] as f64, 5e-5);
    assert_close(restored[3] as f64, original[3] as f64, 5e-5);
}
