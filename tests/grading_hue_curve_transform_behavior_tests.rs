//! GradingHueCurveTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/grading_hue_curve.rs`. In bundled/real mode they validate
//! curve editing, style-reset semantics, HSY conversion state, dynamic toggles,
//! and editable-copy independence.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::grading::{GradingCurvePoint, GradingHueCurveValue};
use ocio_rs::transform::GradingHueCurveTransform;
use ocio_rs::{GradingStyle, HSYTransformStyle, HueCurveType, TransformDirection};

fn grading_hue_curve_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn identity_hue_curve() -> Vec<GradingCurvePoint> {
    vec![
        GradingCurvePoint::new(0.0, 0.0, 1.0),
        GradingCurvePoint::new(1.0, 1.0, 1.0),
    ]
}

fn periodic_hue_curve() -> Vec<GradingCurvePoint> {
    vec![
        GradingCurvePoint::new(0.0, 0.0, 1.0),
        GradingCurvePoint::new(0.25, 0.2, 0.9),
        GradingCurvePoint::new(0.5, 0.4, 1.0),
    ]
}

#[test]
fn grading_hue_curve_round_trip_style_reset_and_copy_behavior() {
    let _guard = grading_hue_curve_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform =
        GradingHueCurveTransform::create(GradingStyle::Log).expect("grading hue curve create");
    let baseline_lin = GradingHueCurveTransform::create(GradingStyle::Lin)
        .expect("grading hue curve lin baseline");

    assert_eq!(transform.style(), GradingStyle::Log);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert!(!transform.is_dynamic());

    let value = GradingHueCurveValue {
        hue_hue: vec![
            GradingCurvePoint::new(0.0, 0.0, 1.0),
            GradingCurvePoint::new(1.0 / 6.0, 0.2, 0.5),
            GradingCurvePoint::new(1.0 / 3.0, 0.4, 1.0),
        ],
        hue_sat: periodic_hue_curve(),
        hue_lum: periodic_hue_curve(),
        lum_sat: vec![
            GradingCurvePoint::new(0.0, 0.0, 1.0),
            GradingCurvePoint::new(0.75, -0.1, 1.2),
            GradingCurvePoint::new(1.0, 0.0, 1.0),
        ],
    };
    transform
        .set_value(&value)
        .expect("set grading hue curve transform value");
    transform
        .try_set_rgb_to_hsy(HSYTransformStyle::None)
        .expect("set rgb to hsy");

    let round_trip = transform.value().expect("grading hue curve value");
    assert_eq!(round_trip.lum_sat, value.lum_sat);
    assert_eq!(
        transform
            .num_control_points(HueCurveType::HueHue)
            .expect("grading hue point count"),
        3
    );
    let (hue_x, hue_y) = transform
        .control_point(HueCurveType::HueHue, 1)
        .expect("grading hue control point");
    assert_close(hue_x as f64, 1.0 / 6.0, 1e-6);
    assert_close(hue_y as f64, 0.2, 1e-6);
    assert_close(
        transform
            .slope(HueCurveType::HueHue, 1)
            .expect("grading hue slope") as f64,
        0.5,
        1e-6,
    );
    assert!(!transform
        .slopes_are_default(HueCurveType::HueHue)
        .expect("grading hue slopes are default"));
    let (lum_sat_x, lum_sat_y) = transform
        .control_point(HueCurveType::LumSat, 1)
        .expect("grading lum_sat control point");
    assert_close(lum_sat_x as f64, 0.75, 1e-6);
    assert_close(lum_sat_y as f64, -0.1, 1e-6);
    assert_close(
        transform
            .slope(HueCurveType::LumSat, 1)
            .expect("grading lum_sat slope") as f64,
        1.2,
        1e-6,
    );
    assert_eq!(transform.rgb_to_hsy(), HSYTransformStyle::None);

    transform.try_make_dynamic().expect("make dynamic");
    assert!(transform.is_dynamic());
    transform.try_make_non_dynamic().expect("make non dynamic");
    assert!(!transform.is_dynamic());

    let copy = transform
        .create_editable_copy()
        .expect("grading hue curve editable copy");
    copy.try_set_direction(TransformDirection::Inverse)
        .expect("set copy direction");
    copy.try_set_rgb_to_hsy(HSYTransformStyle::Default)
        .expect("set copy rgb to hsy");
    copy.set_slope(HueCurveType::LumSat, 1, 0.25)
        .expect("set copy hue slope");

    assert_eq!(copy.direction(), TransformDirection::Inverse);
    assert_eq!(copy.rgb_to_hsy(), HSYTransformStyle::Default);
    assert_close(
        copy.slope(HueCurveType::LumSat, 1).expect("copy hue slope") as f64,
        0.25,
        1e-6,
    );
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert_eq!(transform.rgb_to_hsy(), HSYTransformStyle::None);
    assert_close(
        transform
            .slope(HueCurveType::LumSat, 1)
            .expect("original hue slope") as f64,
        1.2,
        1e-6,
    );

    transform
        .try_set_style(GradingStyle::Lin)
        .expect("set style lin");
    assert_eq!(transform.style(), GradingStyle::Lin);
    assert_eq!(
        transform.value().expect("hue value after style reset"),
        baseline_lin.value().expect("baseline lin hue value")
    );
    assert_eq!(transform.rgb_to_hsy(), HSYTransformStyle::None);
}

#[test]
#[allow(deprecated)]
fn grading_hue_curve_raw_value_handle_survives_parent_drop() {
    let _guard = grading_hue_curve_transform_test_lock();
    if is_stub() {
        return;
    }

    let wrapper =
        GradingHueCurveTransform::create(GradingStyle::Log).expect("wrapper grading hue curve");
    wrapper
        .set_value(&GradingHueCurveValue {
            hue_hue: vec![
                GradingCurvePoint::new(0.0, 0.0, 1.0),
                GradingCurvePoint::new(0.25, 0.3, 0.7),
            ],
            hue_sat: periodic_hue_curve(),
            hue_lum: periodic_hue_curve(),
            lum_sat: identity_hue_curve(),
        })
        .expect("seed wrapper grading hue curve");

    unsafe {
        let handle = wrapper.raw_value_handle();
        assert!(!handle.is_null(), "raw grading hue curve handle");
        drop(wrapper);

        let target =
            ocio_sys::ocio_grading_hue_curve_transform_create_with_style(GradingStyle::Log as i32);
        assert!(!target.is_null(), "target grading hue curve transform");
        ocio_sys::ocio_grading_hue_curve_transform_set_value(target, handle);

        assert_eq!(
            ocio_sys::ocio_grading_hue_curve_transform_get_num_control_points(
                target,
                HueCurveType::HueHue as i32
            ),
            2
        );
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        ocio_sys::ocio_grading_hue_curve_transform_get_control_point(
            target,
            HueCurveType::HueHue as i32,
            1,
            &mut x,
            &mut y,
        );
        assert_close(x as f64, 0.25, 1e-6);
        assert_close(y as f64, 0.3, 1e-6);

        ocio_sys::ocio_grading_hue_curve_destroy(handle);
        ocio_sys::ocio_grading_hue_curve_transform_destroy(target);
    }
}

#[test]
fn grading_hue_curve_invalid_operations_surface_errors() {
    let _guard = grading_hue_curve_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform =
        GradingHueCurveTransform::create(GradingStyle::Log).expect("grading hue curve create");

    let negative_count_err = transform
        .set_num_control_points(HueCurveType::HueHue, -1)
        .expect_err("negative hue point count should fail");
    assert!(
        matches!(negative_count_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {negative_count_err:?}"
    );

    let negative_index_err = transform
        .control_point(HueCurveType::HueHue, -1)
        .expect_err("negative hue point index should fail");
    assert!(
        matches!(negative_index_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {negative_index_err:?}"
    );

    transform
        .set_num_control_points(HueCurveType::HueHue, 2)
        .expect("seed hue point count");

    transform
        .set_num_control_points(HueCurveType::HueHue, 1)
        .expect_err("too few hue points should fail");
    transform
        .set_num_control_points(HueCurveType::HueHue, 2)
        .expect("restore hue point count");

    transform
        .control_point(HueCurveType::HueHue, 99)
        .expect_err("out-of-range hue point should fail");
    transform
        .set_slope(HueCurveType::HueHue, 99, 0.5)
        .expect_err("out-of-range hue slope should fail");
}

#[test]
fn grading_hue_curve_try_setters_surface_errors() {
    let _guard = grading_hue_curve_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform =
        GradingHueCurveTransform::create(GradingStyle::Log).expect("grading hue curve create");

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
        .try_set_rgb_to_hsy(HSYTransformStyle::None)
        .expect("try_set_rgb_to_hsy None");
    assert_eq!(transform.rgb_to_hsy(), HSYTransformStyle::None);

    transform
        .try_set_rgb_to_hsy(HSYTransformStyle::Default)
        .expect("try_set_rgb_to_hsy Default");
    assert_eq!(transform.rgb_to_hsy(), HSYTransformStyle::Default);

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
