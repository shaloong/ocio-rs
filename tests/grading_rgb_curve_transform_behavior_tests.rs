//! GradingRGBCurveTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/grading_rgb_curve.rs`. In bundled/real mode they validate
//! curve editing, style-reset semantics, bypass state, dynamic toggles, and
//! editable-copy independence.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::grading::{GradingCurvePoint, GradingRGBCurveValue};
use ocio_rs::transform::GradingRGBCurveTransform;
use ocio_rs::{GradingStyle, RGBCurveType, TransformDirection};

fn grading_rgb_curve_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn sample_rgb_curve_value() -> GradingRGBCurveValue {
    GradingRGBCurveValue {
        red: vec![
            GradingCurvePoint::new(0.0, 0.0, 1.0),
            GradingCurvePoint::new(0.5, 0.6, 0.8),
            GradingCurvePoint::new(1.0, 1.0, 1.0),
        ],
        green: vec![
            GradingCurvePoint::new(0.0, 0.0, 1.0),
            GradingCurvePoint::new(1.0, 1.0, 1.0),
        ],
        blue: vec![
            GradingCurvePoint::new(0.0, 0.0, 1.0),
            GradingCurvePoint::new(0.25, 0.2, 1.1),
            GradingCurvePoint::new(1.0, 1.0, 1.0),
        ],
        master: vec![
            GradingCurvePoint::new(0.0, 0.0, 1.0),
            GradingCurvePoint::new(1.0, 1.0, 1.0),
        ],
    }
}

#[test]
fn grading_rgb_curve_round_trip_style_reset_and_copy_behavior() {
    let _guard = grading_rgb_curve_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform =
        GradingRGBCurveTransform::create(GradingStyle::Log).expect("grading rgb curve create");
    let baseline_lin = GradingRGBCurveTransform::create(GradingStyle::Lin)
        .expect("grading rgb curve lin baseline");

    assert_eq!(transform.style(), GradingStyle::Log);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert!(!transform.is_dynamic());

    let value = sample_rgb_curve_value();
    transform.set_value(&value);
    transform.set_bypass_lin_to_log(true);

    let round_trip = transform.value();
    assert_eq!(round_trip.red, value.red);
    assert_eq!(round_trip.green, value.green);
    assert_eq!(round_trip.blue, value.blue);
    assert_eq!(round_trip.master, value.master);
    assert_eq!(transform.num_control_points(RGBCurveType::Red), 3);
    let (red_x, red_y) = transform.control_point(RGBCurveType::Red, 1);
    assert_close(red_x as f64, 0.5, 1e-6);
    assert_close(red_y as f64, 0.6, 1e-6);
    assert_close(transform.slope(RGBCurveType::Red, 1) as f64, 0.8, 1e-6);
    assert!(!transform.slopes_are_default(RGBCurveType::Red));
    assert!(transform.bypass_lin_to_log());

    transform.make_dynamic();
    assert!(transform.is_dynamic());
    transform.make_non_dynamic();
    assert!(!transform.is_dynamic());

    let copy = transform
        .create_editable_copy()
        .expect("grading rgb curve editable copy");
    copy.set_direction(TransformDirection::Inverse);
    copy.set_bypass_lin_to_log(false);
    copy.set_slope(RGBCurveType::Blue, 1, 0.33);

    assert_eq!(copy.direction(), TransformDirection::Inverse);
    assert!(!copy.bypass_lin_to_log());
    assert_close(copy.slope(RGBCurveType::Blue, 1) as f64, 0.33, 1e-6);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert!(transform.bypass_lin_to_log());
    assert_close(transform.slope(RGBCurveType::Blue, 1) as f64, 1.1, 1e-6);

    transform.set_style(GradingStyle::Lin);
    assert_eq!(transform.style(), GradingStyle::Lin);
    assert_eq!(transform.value(), baseline_lin.value());
    assert!(transform.bypass_lin_to_log());
}

#[test]
#[allow(deprecated)]
fn grading_rgb_curve_raw_value_handle_survives_parent_drop() {
    let _guard = grading_rgb_curve_transform_test_lock();
    if is_stub() {
        return;
    }

    let seeded = sample_rgb_curve_value();
    let wrapper =
        GradingRGBCurveTransform::create(GradingStyle::Log).expect("wrapper grading rgb curve");
    wrapper.set_value(&seeded);

    unsafe {
        let handle = wrapper.raw_value_handle();
        assert!(!handle.is_null(), "raw grading rgb curve handle");
        drop(wrapper);

        let target =
            ocio_sys::ocio_grading_rgb_curve_transform_create_with_style(GradingStyle::Log as i32);
        assert!(!target.is_null(), "target grading rgb curve transform");
        ocio_sys::ocio_grading_rgb_curve_transform_set_value(target, handle);

        assert_eq!(
            ocio_sys::ocio_grading_rgb_curve_transform_get_num_control_points(
                target,
                RGBCurveType::Red as i32
            ),
            3
        );
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        ocio_sys::ocio_grading_rgb_curve_transform_get_control_point(
            target,
            RGBCurveType::Red as i32,
            1,
            &mut x,
            &mut y,
        );
        assert_close(x as f64, 0.5, 1e-6);
        assert_close(y as f64, 0.6, 1e-6);

        ocio_sys::ocio_grading_rgb_curve_destroy(handle);
        ocio_sys::ocio_grading_rgb_curve_transform_destroy(target);
    }
}
