use ocio_rs::GradingBSplineCurve;

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn spline_points_slopes_copy_and_errors() {
    let mut curve = GradingBSplineCurve::create(2).unwrap();
    curve.set_control_point(0, 0.0, 0.0).unwrap();
    curve.set_control_point(1, 1.0, 1.0).unwrap();
    curve.set_slope(0, 0.5).unwrap();
    curve.validate().unwrap();
    let mut copy = curve.create_editable_copy().unwrap();
    assert!(curve.equals(&copy).unwrap());
    copy.set_slope(0, 0.7).unwrap();
    assert!(!curve.equals(&copy).unwrap());
    assert_eq!(curve.num_control_points().unwrap(), 2);
    assert_eq!(curve.control_point(1).unwrap(), (1.0, 1.0));
    assert_eq!(curve.slope(0).unwrap(), 0.5);
    assert!(curve.control_point(99).is_err());
    assert!(curve.set_slope(99, 0.5).is_err());
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn typed_spline_constructors_select_native_types() {
    use ocio_rs::{BSplineType, HueCurveType};
    let mut curve = GradingBSplineCurve::create_with_spline_type(2, BSplineType::Diagonal).unwrap();
    assert_eq!(curve.spline_type().unwrap(), BSplineType::Diagonal);
    curve.set_spline_type(BSplineType::Standard).unwrap();
    assert_eq!(curve.spline_type().unwrap(), BSplineType::Standard);
    let hue = GradingBSplineCurve::create_with_hue_type(2, HueCurveType::HueFx).unwrap();
    assert_eq!(hue.spline_type().unwrap(), BSplineType::PeriodicZero);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn rgb_curve_object_shares_curve_and_copies_to_transform() {
    use ocio_rs::{
        transform::GradingRGBCurveTransform, GradingRGBCurve, GradingStyle, RGBCurveType,
    };
    let mut value = GradingRGBCurve::create(GradingStyle::Lin).unwrap();
    let original = value.create_editable_copy().unwrap();
    let mut red = value.curve(RGBCurveType::Red).unwrap();
    red.set_slope(0, 0.37).unwrap();
    assert!(!value.equals(&original).unwrap());
    let transform = GradingRGBCurveTransform::create(GradingStyle::Lin).unwrap();
    value.set_transform(&transform).unwrap();
    let copy = GradingRGBCurve::from_transform(&transform).unwrap();
    assert!(copy.equals(&value).unwrap());
    drop(value);
    assert_eq!(red.slope(0).unwrap(), 0.37);
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn hue_curve_object_preserves_draw_mode_and_all_curve_types() {
    use ocio_rs::{
        transform::GradingHueCurveTransform, GradingHueCurve, GradingStyle, HueCurveType,
    };
    let mut value = GradingHueCurve::create(GradingStyle::Lin).unwrap();
    value.set_draw_curve_only(true).unwrap();
    let mut curve = value.curve(HueCurveType::HueFx).unwrap();
    curve.set_slope(0, 0.37).unwrap();
    let transform = GradingHueCurveTransform::create(GradingStyle::Lin).unwrap();
    value.set_transform(&transform).unwrap();
    let copy = GradingHueCurve::from_transform(&transform).unwrap();
    assert!(copy.draw_curve_only().unwrap());
    assert!(copy.equals(&value).unwrap());
}
