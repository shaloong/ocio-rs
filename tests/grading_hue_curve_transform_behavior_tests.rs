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
            GradingCurvePoint::new(1.0 / 3.0, 0.0, 1.0),
        ],
        hue_sat: vec![],
        hue_lum: vec![],
        lum_sat: vec![
            GradingCurvePoint::new(0.0, 0.0, 1.0),
            GradingCurvePoint::new(0.75, -0.1, 1.2),
            GradingCurvePoint::new(1.0, 0.0, 1.0),
        ],
    };
    transform.set_value(&value);
    transform.set_rgb_to_hsy(HSYTransformStyle::None);

    let round_trip = transform.value();
    assert_eq!(round_trip.lum_sat, value.lum_sat);
    assert_eq!(transform.num_control_points(HueCurveType::HueHue), 3);
    let (hue_x, hue_y) = transform.control_point(HueCurveType::HueHue, 1);
    assert_close(hue_x as f64, (1.0 / 6.0) as f64, 1e-6);
    assert_close(hue_y as f64, 0.2, 1e-6);
    assert_close(transform.slope(HueCurveType::HueHue, 1) as f64, 0.5, 1e-6);
    assert!(!transform.slopes_are_default(HueCurveType::HueHue));
    let (lum_sat_x, lum_sat_y) = transform.control_point(HueCurveType::LumSat, 1);
    assert_close(lum_sat_x as f64, 0.75, 1e-6);
    assert_close(lum_sat_y as f64, -0.1, 1e-6);
    assert_close(transform.slope(HueCurveType::LumSat, 1) as f64, 1.2, 1e-6);
    assert_eq!(transform.rgb_to_hsy(), HSYTransformStyle::None);

    transform.make_dynamic();
    assert!(transform.is_dynamic());
    transform.make_non_dynamic();
    assert!(!transform.is_dynamic());

    let copy = transform
        .create_editable_copy()
        .expect("grading hue curve editable copy");
    copy.set_direction(TransformDirection::Inverse);
    copy.set_rgb_to_hsy(HSYTransformStyle::Default);
    copy.set_slope(HueCurveType::LumSat, 1, 0.25);

    assert_eq!(copy.direction(), TransformDirection::Inverse);
    assert_eq!(copy.rgb_to_hsy(), HSYTransformStyle::Default);
    assert_close(copy.slope(HueCurveType::LumSat, 1) as f64, 0.25, 1e-6);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert_eq!(transform.rgb_to_hsy(), HSYTransformStyle::None);
    assert_close(transform.slope(HueCurveType::LumSat, 1) as f64, 1.2, 1e-6);

    transform.set_style(GradingStyle::Lin);
    assert_eq!(transform.style(), GradingStyle::Lin);
    assert_eq!(transform.value(), baseline_lin.value());
    assert_eq!(transform.rgb_to_hsy(), HSYTransformStyle::None);
}
