//! LogCameraTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/log_camera.rs`. In bundled/real mode they validate break and
//! linear-slope state, editable-copy independence, and real processor execution
//! across both the near-black linear segment and the log segment.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::LogCameraTransform;
use ocio_rs::TransformDirection;

fn log_camera_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configured_log_camera_transform() -> LogCameraTransform {
    let transform =
        LogCameraTransform::create(&[0.5, 0.5, 0.5]).expect("log camera transform create");
    transform.set_base(2.0);
    transform.set_log_side_slope_value(&[1.0, 1.0, 1.0]);
    transform.set_log_side_offset_value(&[0.0, 0.0, 0.0]);
    transform.set_lin_side_slope_value(&[1.0, 1.0, 1.0]);
    transform.set_lin_side_offset_value(&[0.0, 0.0, 0.0]);
    transform.set_lin_side_break_value(&[0.5, 0.5, 0.5]);
    transform.set_linear_slope_value(&[1.0, 1.0, 1.0]);
    transform
}

#[test]
fn log_camera_transform_value_copy_and_direction_behavior() {
    let _guard = log_camera_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = configured_log_camera_transform();

    assert_close(transform.base(), 2.0, 1e-10);
    assert_vec_close(&transform.log_side_slope_value(), &[1.0, 1.0, 1.0], 1e-10);
    assert_vec_close(&transform.log_side_offset_value(), &[0.0, 0.0, 0.0], 1e-10);
    assert_vec_close(&transform.lin_side_slope_value(), &[1.0, 1.0, 1.0], 1e-10);
    assert_vec_close(&transform.lin_side_offset_value(), &[0.0, 0.0, 0.0], 1e-10);
    assert_vec_close(&transform.lin_side_break_value(), &[0.5, 0.5, 0.5], 1e-10);
    assert_eq!(transform.linear_slope_value(), Some([1.0, 1.0, 1.0]));
    assert_eq!(transform.direction(), TransformDirection::Forward);

    let copy = transform
        .create_editable_copy()
        .expect("log camera transform editable copy");
    copy.set_base(10.0);
    copy.set_lin_side_break_value(&[0.25, 0.25, 0.25]);
    copy.set_linear_slope_value(&[2.0, 2.0, 2.0]);
    copy.set_direction(TransformDirection::Inverse);

    assert_close(copy.base(), 10.0, 1e-10);
    assert_vec_close(&copy.lin_side_break_value(), &[0.25, 0.25, 0.25], 1e-10);
    assert_eq!(copy.linear_slope_value(), Some([2.0, 2.0, 2.0]));
    assert_eq!(copy.direction(), TransformDirection::Inverse);

    assert_close(transform.base(), 2.0, 1e-10);
    assert_vec_close(&transform.lin_side_break_value(), &[0.5, 0.5, 0.5], 1e-10);
    assert_eq!(transform.linear_slope_value(), Some([1.0, 1.0, 1.0]));
    assert_eq!(transform.direction(), TransformDirection::Forward);
}

#[test]
fn log_camera_transform_linear_slope_optional_state_behavior() {
    let _guard = log_camera_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform =
        LogCameraTransform::create(&[0.5, 0.5, 0.5]).expect("log camera transform create");
    transform.set_base(2.0);
    transform.set_log_side_slope_value(&[1.0, 1.0, 1.0]);
    transform.set_log_side_offset_value(&[0.0, 0.0, 0.0]);
    transform.set_lin_side_slope_value(&[1.0, 1.0, 1.0]);
    transform.set_lin_side_offset_value(&[0.0, 0.0, 0.0]);

    assert_eq!(transform.linear_slope_value(), None);

    transform.set_linear_slope_value(&[1.0, 1.0, 1.0]);
    assert_eq!(transform.linear_slope_value(), Some([1.0, 1.0, 1.0]));

    transform.unset_linear_slope_value();
    assert_eq!(transform.linear_slope_value(), None);
}

#[test]
fn log_camera_transform_processor_behavior() {
    let _guard = log_camera_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = configured_log_camera_transform();

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

    // With base=2, log-side slope/offset = 1/0, lin-side slope/offset = 1/0,
    // break=0.5, and explicit linearSlope=1:
    // - for x < 0.5, output is x + linearOffset, where
    //   linearOffset = log2(0.5) - 1 * 0.5 = -1.5
    // - for x >= 0.5, output is log2(x)
    let original = [0.25f32, 0.5, 1.0, 0.6];
    let mut logged = original;
    forward_cpu.apply_rgba(&mut logged);

    assert_close(logged[0] as f64, -1.25, 2e-5);
    assert_close(logged[1] as f64, -1.0, 2e-5);
    assert_close(logged[2] as f64, 0.0, 2e-5);
    assert_close(logged[3] as f64, original[3] as f64, 2e-5);

    let mut restored = logged;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 5e-5);
    assert_close(restored[1] as f64, original[1] as f64, 5e-5);
    assert_close(restored[2] as f64, original[2] as f64, 5e-5);
    assert_close(restored[3] as f64, original[3] as f64, 5e-5);
}
