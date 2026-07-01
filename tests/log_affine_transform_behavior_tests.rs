//! LogAffineTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/log_affine.rs`. In bundled/real mode they validate affine
//! parameter round trips, editable-copy independence, and real processor
//! execution for the documented log-affine formula on positive inputs.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::LogAffineTransform;
use ocio_rs::TransformDirection;

fn log_affine_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configured_log_affine_transform() -> LogAffineTransform {
    let transform = LogAffineTransform::create().expect("log affine transform create");
    transform.set_base(2.0);
    transform.set_log_side_slope_value(&[1.0, 1.0, 1.0]);
    transform.set_log_side_offset_value(&[0.0, 0.0, 0.0]);
    transform.set_lin_side_slope_value(&[2.0, 2.0, 2.0]);
    transform.set_lin_side_offset_value(&[0.5, 0.5, 0.5]);
    transform
}

#[test]
fn log_affine_transform_value_copy_and_direction_behavior() {
    let _guard = log_affine_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = configured_log_affine_transform();

    assert_close(transform.base(), 2.0, 1e-10);
    assert_vec_close(&transform.log_side_slope_value(), &[1.0, 1.0, 1.0], 1e-10);
    assert_vec_close(&transform.log_side_offset_value(), &[0.0, 0.0, 0.0], 1e-10);
    assert_vec_close(&transform.lin_side_slope_value(), &[2.0, 2.0, 2.0], 1e-10);
    assert_vec_close(&transform.lin_side_offset_value(), &[0.5, 0.5, 0.5], 1e-10);
    assert_eq!(transform.direction(), TransformDirection::Forward);

    let copy = transform
        .create_editable_copy()
        .expect("log affine transform editable copy");
    copy.set_base(10.0);
    copy.set_log_side_slope_value(&[0.9, 0.9, 0.9]);
    copy.set_log_side_offset_value(&[0.1, 0.1, 0.1]);
    copy.set_lin_side_slope_value(&[1.0, 1.0, 1.0]);
    copy.set_lin_side_offset_value(&[0.0, 0.0, 0.0]);
    copy.set_direction(TransformDirection::Inverse);

    assert_close(copy.base(), 10.0, 1e-10);
    assert_vec_close(&copy.log_side_slope_value(), &[0.9, 0.9, 0.9], 1e-10);
    assert_vec_close(&copy.log_side_offset_value(), &[0.1, 0.1, 0.1], 1e-10);
    assert_vec_close(&copy.lin_side_slope_value(), &[1.0, 1.0, 1.0], 1e-10);
    assert_vec_close(&copy.lin_side_offset_value(), &[0.0, 0.0, 0.0], 1e-10);
    assert_eq!(copy.direction(), TransformDirection::Inverse);

    assert_close(transform.base(), 2.0, 1e-10);
    assert_vec_close(&transform.log_side_slope_value(), &[1.0, 1.0, 1.0], 1e-10);
    assert_vec_close(&transform.log_side_offset_value(), &[0.0, 0.0, 0.0], 1e-10);
    assert_vec_close(&transform.lin_side_slope_value(), &[2.0, 2.0, 2.0], 1e-10);
    assert_vec_close(&transform.lin_side_offset_value(), &[0.5, 0.5, 0.5], 1e-10);
    assert_eq!(transform.direction(), TransformDirection::Forward);
}

#[test]
fn log_affine_transform_processor_behavior() {
    let _guard = log_affine_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = configured_log_affine_transform();

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

    // Formula:
    // logSideSlope * log(linSideSlope * color + linSideOffset, base) + logSideOffset
    // Here that reduces to log2(2 * color + 0.5).
    let original = [0.25f32, 0.75, 1.75, 0.6];
    let mut logged = original;
    forward_cpu.apply_rgba(&mut logged);

    assert_close(logged[0] as f64, 0.0, 2e-5);
    assert_close(logged[1] as f64, 1.0, 2e-5);
    assert_close(logged[2] as f64, 2.0, 2e-5);
    assert_close(logged[3] as f64, original[3] as f64, 2e-5);

    let mut restored = logged;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 5e-5);
    assert_close(restored[1] as f64, original[1] as f64, 5e-5);
    assert_close(restored[2] as f64, original[2] as f64, 5e-5);
    assert_close(restored[3] as f64, original[3] as f64, 5e-5);
}
