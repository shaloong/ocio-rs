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
    assert_eq!(curve.slope(0).unwrap(), 0.5);
    assert_eq!(copy.slope(0).unwrap(), 0.7);
    copy.set_control_point(0, 0.0, 0.1).unwrap();
    assert!(!curve.equals(&copy).unwrap());
    assert_eq!(curve.num_control_points().unwrap(), 2);
    assert_eq!(curve.control_point(1).unwrap(), (1.0, 1.0));
    assert_eq!(curve.slope(0).unwrap(), 0.5);
    let mut points = [[-7.0; 3]; 2];
    curve.copy_points_into(0, &mut points).unwrap();
    assert_eq!(points[0], [0.0, 0.0, 0.5]);
    assert_eq!(points[1], [1.0, 1.0, curve.slope(1).unwrap()]);
    let mut tail = [[-7.0; 3]; 1];
    curve.copy_points_into(1, &mut tail).unwrap();
    assert_eq!(tail[0], points[1]);
    assert!(curve.copy_points_into(usize::MAX, &mut tail).is_err());
    assert_eq!(tail[0], points[1]);
    assert!(curve.copy_points_into(2, &mut tail).is_err());
    assert_eq!(tail[0], points[1]);
    curve.copy_points_into(2, &mut []).unwrap();
    assert!(curve.copy_points_into(3, &mut []).is_err());
    assert!(curve.control_point(99).is_err());
    assert!(curve.set_slope(99, 0.5).is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn point_list_construction_can_defer_native_validation() {
    let empty = GradingBSplineCurve::create_from_points_unvalidated(&[]).unwrap();
    assert_eq!(empty.num_control_points().unwrap(), 0);
    assert!(empty.validate().is_err());
    let points = [[1.0, 1.0], [0.0, 0.0]];
    assert!(GradingBSplineCurve::create_from_points(&points).is_err());
    let mut curve = GradingBSplineCurve::create_from_points_unvalidated(&points).unwrap();
    assert_eq!(curve.control_point(0).unwrap(), (1.0, 1.0));
    assert!(curve.validate().is_err());
    curve.set_control_point(0, 0.0, 0.0).unwrap();
    curve.set_control_point(1, 1.0, 1.0).unwrap();
    curve.validate().unwrap();
    #[cfg(feature = "v2_5")]
    {
        use ocio_rs::{BSplineType, HueCurveType};
        let typed = GradingBSplineCurve::create_from_points_with_spline_type_unvalidated(
            &points,
            BSplineType::Standard,
        )
        .unwrap();
        assert!(typed.validate().is_err());
        let hue = GradingBSplineCurve::create_from_points_with_hue_type_unvalidated(
            &points,
            HueCurveType::HueFx,
        )
        .unwrap();
        assert!(hue.validate().is_err());
    }
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
    let red_alias = value.curve(RGBCurveType::Red).unwrap();
    let shared = value.curve_shared(RGBCurveType::Red).unwrap();
    assert_eq!(shared.view().serialize().unwrap(), red.serialize().unwrap());
    assert!(value.serialize().unwrap().contains("<red="));
    let shared_set = GradingRGBCurve::create_from_shared_curves([&shared; 4]).unwrap();
    let view = value.curve_ref(RGBCurveType::Red).unwrap();
    red.set_slope(0, 0.37).unwrap();
    assert!(shared_set.is_identity().unwrap());
    assert_eq!(shared.view().slope(0).unwrap(), 0.37);
    assert_eq!(red_alias.slope(0).unwrap(), 0.37);
    assert_eq!(view.slope(0).unwrap(), 0.37);
    assert_eq!(
        view.slopes_are_default().unwrap(),
        red.slopes_are_default().unwrap()
    );
    let mut independent = view.create_editable_copy().unwrap();
    independent.set_slope(0, 0.63).unwrap();
    assert_eq!(view.slope(0).unwrap(), 0.37);
    assert_eq!(
        view.num_control_points().unwrap(),
        red.num_control_points().unwrap()
    );
    assert_eq!(
        view.control_point(0).unwrap(),
        red.control_point(0).unwrap()
    );
    assert!(view.control_point(usize::MAX).is_err());
    let (x, y) = red.control_point(0).unwrap();
    red.set_control_point(0, x, y + 0.1).unwrap();
    assert!(!value.equals(&original).unwrap());
    let transform = GradingRGBCurveTransform::create(GradingStyle::Lin).unwrap();
    value.set_transform(&transform).unwrap();
    let shared_value = ocio_rs::SharedGradingRGBCurve::from_transform(&transform).unwrap();
    let borrowed = shared_value.curve_ref(RGBCurveType::Red).unwrap();
    let mut points = vec![[0.0; 3]; borrowed.num_control_points().unwrap()];
    borrowed.copy_points_into(0, &mut points).unwrap();
    assert_eq!(points[0][2], borrowed.slope(0).unwrap());
    assert_eq!(shared_value.slope(RGBCurveType::Red, 0).unwrap(), 0.37);
    assert!(shared_value.slope(RGBCurveType::Red, usize::MAX).is_err());
    let shared_equal = ocio_rs::SharedGradingRGBCurve::from_transform(&transform).unwrap();
    assert!(shared_value.equals(&shared_equal).unwrap());
    let destination = GradingRGBCurveTransform::create(GradingStyle::Lin).unwrap();
    shared_value.set_transform(&destination).unwrap();
    let forwarded = ocio_rs::SharedGradingRGBCurve::from_transform(&destination).unwrap();
    assert!(shared_value.equals(&forwarded).unwrap());
    assert_eq!(
        shared_value.serialize().unwrap(),
        value.serialize().unwrap()
    );
    assert_eq!(
        shared_value
            .curve_ref(RGBCurveType::Red)
            .unwrap()
            .slope(0)
            .unwrap(),
        0.37
    );
    shared_value.validate().unwrap();
    assert_eq!(
        shared_value.is_identity().unwrap(),
        value.is_identity().unwrap()
    );
    let shared_child = shared_value.curve_shared(RGBCurveType::Red).unwrap();
    assert!(shared_value
        .create_editable_copy()
        .unwrap()
        .equals(&value)
        .unwrap());
    let copy = GradingRGBCurve::from_transform(&transform).unwrap();
    assert!(copy.equals(&value).unwrap());
    drop(transform);
    drop(shared_value);
    assert_eq!(shared_child.view().slope(0).unwrap(), 0.37);
    drop(value);
    assert_eq!(red.slope(0).unwrap(), 0.37);
    drop(red);
    assert_eq!(red_alias.slope(0).unwrap(), 0.37);
    drop(red_alias);
    assert_eq!(shared.view().slope(0).unwrap(), 0.37);
    let mut independent = shared.view().create_editable_copy().unwrap();
    independent.set_slope(0, 0.7).unwrap();
    assert_eq!(shared.view().slope(0).unwrap(), 0.37);
    let mut spline = GradingBSplineCurve::create_from_points(&[[0.0, 0.0], [1.0, 1.0]]).unwrap();
    let set = GradingRGBCurve::create_from_curves([&spline, &spline, &spline, &spline]).unwrap();
    spline.set_slope(0, 0.2).unwrap();
    assert!(set.is_identity().unwrap());
    assert!(GradingBSplineCurve::create_from_points(&[[1.0, 1.0], [0.0, 0.0]]).is_err());
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
    let view = value.curve_ref(HueCurveType::HueFx).unwrap();
    let shared = value.curve_shared(HueCurveType::HueFx).unwrap();
    assert_eq!(view.serialize().unwrap(), curve.serialize().unwrap());
    assert!(!value.serialize().unwrap().is_empty());
    assert_eq!(view.slope(0).unwrap(), 0.37);
    assert_eq!(view.spline_type().unwrap(), curve.spline_type().unwrap());
    view.validate().unwrap();
    let transform = GradingHueCurveTransform::create(GradingStyle::Lin).unwrap();
    value.set_transform(&transform).unwrap();
    let shared_value = ocio_rs::SharedGradingHueCurve::from_transform(&transform).unwrap();
    assert_eq!(shared_value.slope(HueCurveType::HueFx, 0).unwrap(), 0.37);
    assert!(shared_value.slope(HueCurveType::HueFx, usize::MAX).is_err());
    let shared_equal = ocio_rs::SharedGradingHueCurve::from_transform(&transform).unwrap();
    assert!(shared_value.equals(&shared_equal).unwrap());
    let destination = GradingHueCurveTransform::create(GradingStyle::Lin).unwrap();
    shared_value.set_transform(&destination).unwrap();
    let forwarded = ocio_rs::SharedGradingHueCurve::from_transform(&destination).unwrap();
    assert!(shared_value.equals(&forwarded).unwrap());
    assert!(forwarded.draw_curve_only().unwrap());
    assert_eq!(
        shared_value.serialize().unwrap(),
        value.serialize().unwrap()
    );
    assert_eq!(
        shared_value
            .curve_ref(HueCurveType::HueFx)
            .unwrap()
            .slope(0)
            .unwrap(),
        0.37
    );
    assert!(shared_value.draw_curve_only().unwrap());
    shared_value.validate().unwrap();
    assert_eq!(
        shared_value.is_identity().unwrap(),
        value.is_identity().unwrap()
    );
    let shared_child = shared_value.curve_shared(HueCurveType::HueFx).unwrap();
    assert!(shared_value
        .create_editable_copy()
        .unwrap()
        .equals(&value)
        .unwrap());
    let copy = GradingHueCurve::from_transform(&transform).unwrap();
    assert!(copy.draw_curve_only().unwrap());
    assert!(copy.equals(&value).unwrap());
    drop(transform);
    drop(shared_value);
    assert_eq!(shared_child.view().slope(0).unwrap(), 0.37);
    drop(curve);
    assert_eq!(shared.view().slope(0).unwrap(), 0.37);
    let curves: Vec<_> = [
        HueCurveType::HueHue,
        HueCurveType::HueSat,
        HueCurveType::HueLum,
        HueCurveType::LumSat,
        HueCurveType::SatSat,
        HueCurveType::LumLum,
        HueCurveType::SatLum,
        HueCurveType::HueFx,
    ]
    .into_iter()
    .map(|kind| value.curve(kind).unwrap())
    .collect();
    let shared_curves: Vec<_> = [
        HueCurveType::HueHue,
        HueCurveType::HueSat,
        HueCurveType::HueLum,
        HueCurveType::LumSat,
        HueCurveType::SatSat,
        HueCurveType::LumLum,
        HueCurveType::SatLum,
        HueCurveType::HueFx,
    ]
    .into_iter()
    .map(|kind| value.curve_shared(kind).unwrap())
    .collect();
    let shared_assembled = GradingHueCurve::create_from_shared_curves([
        &shared_curves[0],
        &shared_curves[1],
        &shared_curves[2],
        &shared_curves[3],
        &shared_curves[4],
        &shared_curves[5],
        &shared_curves[6],
        &shared_curves[7],
    ])
    .unwrap();
    assert_eq!(
        shared_assembled
            .curve_ref(HueCurveType::HueFx)
            .unwrap()
            .slope(0)
            .unwrap(),
        0.37
    );
    let mut assembled = GradingHueCurve::create_from_curves([
        &curves[0], &curves[1], &curves[2], &curves[3], &curves[4], &curves[5], &curves[6],
        &curves[7],
    ])
    .unwrap();
    assert_eq!(
        assembled
            .curve(HueCurveType::HueFx)
            .unwrap()
            .slope(0)
            .unwrap(),
        0.37
    );
    drop(curves);
    drop(value);
    assert_eq!(shared.view().slope(0).unwrap(), 0.37);
}
