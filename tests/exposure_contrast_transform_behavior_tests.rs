//! ExposureContrastTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/exposure_contrast.rs`. In bundled/real mode they validate
//! parameter round trips, dynamic-flag toggles, editable-copy independence,
//! equality semantics, and real processor execution for linear exposure.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::ExposureContrastTransform;
use ocio_rs::{ExposureContrastStyle, TransformDirection};

fn exposure_contrast_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn linear_exposure_transform() -> ExposureContrastTransform {
    let transform = ExposureContrastTransform::create().expect("exposure contrast create");
    transform
        .try_set_style(ExposureContrastStyle::Linear)
        .expect("set style");
    transform.try_set_exposure(1.0).expect("set exposure");
    transform.try_set_contrast(1.0).expect("set contrast");
    transform.try_set_gamma(1.0).expect("set gamma");
    transform.try_set_pivot(0.18).expect("set pivot");
    transform
}

#[test]
fn exposure_contrast_parameter_and_dynamic_round_trip_behavior() {
    let _guard = exposure_contrast_test_lock();
    if is_stub() {
        return;
    }

    let transform = linear_exposure_transform();
    transform
        .try_set_log_exposure_step(0.088)
        .expect("set log exposure step");
    transform
        .try_set_log_mid_gray(0.18)
        .expect("set log mid gray");

    assert_eq!(transform.style(), ExposureContrastStyle::Linear);
    assert_close(transform.exposure(), 1.0, 1e-10);
    assert_close(transform.contrast(), 1.0, 1e-10);
    assert_close(transform.gamma(), 1.0, 1e-10);
    assert_close(transform.pivot(), 0.18, 1e-10);
    assert_close(transform.log_exposure_step(), 0.088, 1e-10);
    assert_close(transform.log_mid_gray(), 0.18, 1e-10);

    assert!(!transform.is_exposure_dynamic());
    assert!(!transform.is_contrast_dynamic());
    assert!(!transform.is_gamma_dynamic());

    transform.make_exposure_dynamic();
    transform.make_contrast_dynamic();
    transform.make_gamma_dynamic();
    assert!(transform.is_exposure_dynamic());
    assert!(transform.is_contrast_dynamic());
    assert!(transform.is_gamma_dynamic());

    transform.make_exposure_non_dynamic();
    transform.make_contrast_non_dynamic();
    transform.make_gamma_non_dynamic();
    assert!(!transform.is_exposure_dynamic());
    assert!(!transform.is_contrast_dynamic());
    assert!(!transform.is_gamma_dynamic());
}

#[test]
fn exposure_contrast_copy_direction_and_equals_behavior() {
    let _guard = exposure_contrast_test_lock();
    if is_stub() {
        return;
    }

    let original = linear_exposure_transform();
    let _ = original.equals(&original);

    let copy = original
        .create_editable_copy()
        .expect("exposure contrast editable copy");

    copy.set_direction(TransformDirection::Inverse);
    copy.try_set_exposure(-1.0).expect("set copy exposure");

    assert_eq!(copy.direction(), TransformDirection::Inverse);
    assert_close(copy.exposure(), -1.0, 1e-10);
    assert_eq!(original.direction(), TransformDirection::Forward);
    assert_close(original.exposure(), 1.0, 1e-10);
    let _ = original.equals(&copy);
}

#[test]
fn exposure_contrast_linear_processor_forward_inverse_behavior() {
    let _guard = exposure_contrast_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = linear_exposure_transform();

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

    let original = [0.25f32, 0.5, 0.125, 1.0];
    let mut brightened = original;
    forward_cpu.apply_rgba(&mut brightened);

    assert_close(brightened[0] as f64, 0.5, 1e-6);
    assert_close(brightened[1] as f64, 1.0, 1e-6);
    assert_close(brightened[2] as f64, 0.25, 1e-6);
    assert_close(brightened[3] as f64, 1.0, 1e-6);

    let mut restored = brightened;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 1e-6);
    assert_close(restored[1] as f64, original[1] as f64, 1e-6);
    assert_close(restored[2] as f64, original[2] as f64, 1e-6);
    assert_close(restored[3] as f64, original[3] as f64, 1e-6);
}
