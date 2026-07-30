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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
    transform
        .set_value(&value)
        .expect("set grading rgb curve transform value");
    transform
        .try_set_bypass_lin_to_log(true)
        .expect("set bypass lin to log");

    let round_trip = transform.value().expect("grading rgb curve value");
    assert_eq!(round_trip.red, value.red);
    assert_eq!(round_trip.green, value.green);
    assert_eq!(round_trip.blue, value.blue);
    assert_eq!(round_trip.master, value.master);
    assert_eq!(
        transform
            .num_control_points(RGBCurveType::Red)
            .expect("grading rgb point count"),
        3
    );
    let (red_x, red_y) = transform
        .control_point(RGBCurveType::Red, 1)
        .expect("grading rgb control point");
    assert_close(red_x as f64, 0.5, 1e-6);
    assert_close(red_y as f64, 0.6, 1e-6);
    assert_close(
        transform
            .slope(RGBCurveType::Red, 1)
            .expect("grading rgb slope") as f64,
        0.8,
        1e-6,
    );
    assert!(!transform
        .slopes_are_default(RGBCurveType::Red)
        .expect("grading rgb slopes are default"));
    assert!(transform.bypass_lin_to_log());

    transform.try_make_dynamic().expect("make dynamic");
    assert!(transform.is_dynamic());
    transform.try_make_non_dynamic().expect("make non dynamic");
    assert!(!transform.is_dynamic());

    let copy = transform
        .create_editable_copy()
        .expect("grading rgb curve editable copy");
    copy.try_set_direction(TransformDirection::Inverse)
        .expect("set copy direction");
    copy.try_set_bypass_lin_to_log(false)
        .expect("set copy bypass");
    copy.set_slope(RGBCurveType::Blue, 1, 0.33)
        .expect("set copy rgb slope");

    assert_eq!(copy.direction(), TransformDirection::Inverse);
    assert!(!copy.bypass_lin_to_log());
    assert_close(
        copy.slope(RGBCurveType::Blue, 1).expect("copy rgb slope") as f64,
        0.33,
        1e-6,
    );
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert!(transform.bypass_lin_to_log());
    assert_close(
        transform
            .slope(RGBCurveType::Blue, 1)
            .expect("original rgb slope") as f64,
        1.1,
        1e-6,
    );

    transform
        .try_set_style(GradingStyle::Lin)
        .expect("set style lin");
    assert_eq!(transform.style(), GradingStyle::Lin);
    assert_eq!(
        transform.value().expect("rgb value after style reset"),
        baseline_lin.value().expect("baseline lin rgb value")
    );
    assert!(transform.bypass_lin_to_log());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
#[allow(deprecated)]
fn grading_rgb_curve_raw_value_handle_survives_parent_drop() {
    let _guard = grading_rgb_curve_transform_test_lock();
    if is_stub() {
        return;
    }

    let seeded = sample_rgb_curve_value();
    let wrapper =
        GradingRGBCurveTransform::create(GradingStyle::Log).expect("wrapper grading rgb curve");
    wrapper
        .set_value(&seeded)
        .expect("seed wrapper grading rgb curve");

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

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn grading_rgb_curve_invalid_operations_surface_errors() {
    let _guard = grading_rgb_curve_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform =
        GradingRGBCurveTransform::create(GradingStyle::Log).expect("grading rgb curve create");

    let negative_count_err = transform
        .set_num_control_points(RGBCurveType::Red, -1)
        .expect_err("negative rgb point count should fail");
    assert!(
        matches!(negative_count_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {negative_count_err:?}"
    );

    let negative_index_err = transform
        .control_point(RGBCurveType::Red, -1)
        .expect_err("negative rgb point index should fail");
    assert!(
        matches!(negative_index_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {negative_index_err:?}"
    );

    transform
        .set_num_control_points(RGBCurveType::Red, 2)
        .expect("seed rgb point count");

    transform
        .set_num_control_points(RGBCurveType::Red, 1)
        .expect_err("too few rgb points should fail");
    transform
        .set_num_control_points(RGBCurveType::Red, 2)
        .expect("restore rgb point count");

    transform
        .control_point(RGBCurveType::Red, 99)
        .expect_err("out-of-range rgb point should fail");
    transform
        .set_slope(RGBCurveType::Red, 99, 0.5)
        .expect_err("out-of-range rgb slope should fail");
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn grading_rgb_curve_try_setters_surface_errors() {
    let _guard = grading_rgb_curve_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform =
        GradingRGBCurveTransform::create(GradingStyle::Log).expect("grading rgb curve create");

    // Valid operations should succeed with readback
    transform
        .try_set_style(GradingStyle::Lin)
        .expect("try_set_style Lin");
    assert_eq!(transform.style(), GradingStyle::Lin);

    transform
        .try_set_direction(TransformDirection::Inverse)
        .expect("try_set_direction Inverse");
    assert_eq!(transform.direction(), TransformDirection::Inverse);

    transform.try_make_dynamic().expect("try_make_dynamic");
    assert!(transform.is_dynamic());

    transform
        .try_make_non_dynamic()
        .expect("try_make_non_dynamic");
    assert!(!transform.is_dynamic());

    transform
        .try_set_bypass_lin_to_log(true)
        .expect("try_set_bypass_lin_to_log true");
    assert!(transform.bypass_lin_to_log());

    transform
        .try_set_bypass_lin_to_log(false)
        .expect("try_set_bypass_lin_to_log false");
    assert!(!transform.bypass_lin_to_log());

    // Verify all state transitions round-trip correctly
    transform
        .try_set_style(GradingStyle::Video)
        .expect("try_set_style Video");
    assert_eq!(transform.style(), GradingStyle::Video);

    transform
        .try_set_style(GradingStyle::Log)
        .expect("try_set_style Log");
    assert_eq!(transform.style(), GradingStyle::Log);

    transform
        .try_set_direction(TransformDirection::Forward)
        .expect("try_set_direction Forward");
    assert_eq!(transform.direction(), TransformDirection::Forward);
}
