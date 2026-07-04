//! Dynamic property behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the API-shape smoke tests in the
//! unit suite. In bundled/real mode they validate that dynamic properties are
//! exposed through both `Processor` and `CPUProcessor`, and that mutating them
//! affects CPU processing results.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::grading::GradingCurvePoint;
use ocio_rs::transform::{
    ExposureContrastTransform, GradingHueCurveTransform, GradingPrimaryTransform,
    GradingRGBCurveTransform, GradingToneTransform,
};
use ocio_rs::{
    DynamicPropertyType, ExposureContrastStyle, GradingStyle, HueCurveType, RGBCurveType,
    TransformDirection,
};

fn dynamic_property_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn dynamic_exposure_processor() -> Option<ocio_rs::Processor> {
    let config = create_test_config()?;
    let transform = ExposureContrastTransform::create().ok()?;
    transform.set_style(ExposureContrastStyle::Linear);
    transform.set_exposure(0.0);
    transform.set_contrast(1.0);
    transform.set_gamma(1.0);
    transform.make_exposure_dynamic();

    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()?;
    Some(processor)
}

fn dynamic_grading_primary_processor() -> Option<ocio_rs::Processor> {
    let config = create_test_config()?;
    let transform = GradingPrimaryTransform::create(GradingStyle::Log).ok()?;
    transform.make_dynamic();
    config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()
}

fn dynamic_grading_tone_processor() -> Option<ocio_rs::Processor> {
    let config = create_test_config()?;
    let transform = GradingToneTransform::create(GradingStyle::Log).ok()?;
    transform.make_dynamic();
    config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()
}

fn dynamic_grading_rgb_curve_processor() -> Option<ocio_rs::Processor> {
    let config = create_test_config()?;
    let transform = GradingRGBCurveTransform::create(GradingStyle::Log).ok()?;
    transform.make_dynamic();
    config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()
}

fn dynamic_grading_hue_curve_processor() -> Option<ocio_rs::Processor> {
    let config = create_test_config()?;
    let transform = GradingHueCurveTransform::create(GradingStyle::Log).ok()?;
    transform.make_dynamic();
    config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()
}

#[test]
fn dynamic_exposure_processor_property_seeds_cpu_behavior() {
    let _guard = dynamic_property_test_lock();
    if is_stub() {
        return;
    }

    let processor = dynamic_exposure_processor().expect("dynamic exposure processor");

    assert!(processor.is_dynamic());
    assert!(processor.has_dynamic_property_kind(DynamicPropertyType::Exposure));

    let processor_prop = processor
        .dynamic_property(DynamicPropertyType::Exposure)
        .expect("processor dynamic property");

    assert_eq!(
        processor_prop.property_type(),
        DynamicPropertyType::Exposure
    );
    assert_close(
        processor_prop
            .double_value()
            .expect("processor prop double value"),
        0.0,
        1e-8,
    );
    processor_prop
        .set_double_value(1.0)
        .expect("set processor prop double value");
    assert_close(
        processor_prop
            .double_value()
            .expect("processor prop double value after update"),
        1.0,
        1e-8,
    );

    let cpu = processor
        .default_cpu_processor()
        .expect("default cpu processor");
    assert!(cpu.is_dynamic());
    assert!(cpu.has_dynamic_property_kind(DynamicPropertyType::Exposure));

    let cpu_prop = cpu
        .dynamic_property(DynamicPropertyType::Exposure)
        .expect("cpu dynamic property");
    assert_eq!(cpu_prop.property_type(), DynamicPropertyType::Exposure);
    assert_close(
        cpu_prop.double_value().expect("cpu prop double value"),
        1.0,
        1e-8,
    );

    let mut pixel = [0.25f32, 0.5, 0.125, 1.0];
    cpu.apply_rgba(&mut pixel);
    assert_close(pixel[0] as f64, 0.5, 1e-6);
    assert_close(pixel[1] as f64, 1.0, 1e-6);
    assert_close(pixel[2] as f64, 0.25, 1e-6);
    assert_close(pixel[3] as f64, 1.0, 1e-6);
}

#[test]
fn dynamic_exposure_cpu_property_round_trip_and_output_behavior() {
    let _guard = dynamic_property_test_lock();
    if is_stub() {
        return;
    }

    let processor = dynamic_exposure_processor().expect("dynamic exposure processor");
    let cpu = processor
        .default_cpu_processor()
        .expect("default cpu processor");
    let cpu_prop = cpu
        .dynamic_property(DynamicPropertyType::Exposure)
        .expect("cpu dynamic property");

    assert!(cpu.is_dynamic());
    assert!(cpu.has_dynamic_property_kind(DynamicPropertyType::Exposure));
    assert_eq!(cpu_prop.property_type(), DynamicPropertyType::Exposure);
    assert_close(
        cpu_prop.double_value().expect("cpu prop double value"),
        0.0,
        1e-8,
    );

    cpu_prop
        .set_double_value(-1.0)
        .expect("set cpu prop double value");
    assert_close(
        cpu_prop
            .double_value()
            .expect("cpu prop double value after update"),
        -1.0,
        1e-8,
    );

    let input = [0.25f32, 0.5, 0.125, 1.0];

    let mut darkened = input;
    cpu.apply_rgba(&mut darkened);
    assert_close(darkened[0] as f64, 0.125, 1e-6);
    assert_close(darkened[1] as f64, 0.25, 1e-6);
    assert_close(darkened[2] as f64, 0.0625, 1e-6);
    assert_close(darkened[3] as f64, 1.0, 1e-6);
}

#[test]
fn dynamic_grading_primary_round_trip_between_processor_and_cpu() {
    let _guard = dynamic_property_test_lock();
    if is_stub() {
        return;
    }

    let processor = dynamic_grading_primary_processor().expect("dynamic grading primary processor");
    assert!(processor.is_dynamic());
    assert!(processor.has_dynamic_property_kind(DynamicPropertyType::GradingPrimary));

    let processor_prop = processor
        .dynamic_property(DynamicPropertyType::GradingPrimary)
        .expect("processor grading primary property");
    assert_eq!(
        processor_prop.property_type(),
        DynamicPropertyType::GradingPrimary
    );

    let mut value = processor_prop
        .grading_primary_value()
        .expect("grading primary value");
    value.brightness.red = 0.125;
    value.contrast.master = 1.15;
    value.gain.blue = 1.25;
    value.saturation = 0.9;
    processor_prop
        .set_grading_primary_value(&value)
        .expect("set grading primary value");

    let round_trip = processor_prop
        .grading_primary_value()
        .expect("processor grading primary round trip");
    assert_close(round_trip.brightness.red, 0.125, 1e-8);
    assert_close(round_trip.contrast.master, 1.15, 1e-8);
    assert_close(round_trip.gain.blue, 1.25, 1e-8);
    assert_close(round_trip.saturation, 0.9, 1e-8);

    let cpu = processor
        .default_cpu_processor()
        .expect("default cpu processor");
    assert!(cpu.is_dynamic());
    assert!(cpu.has_dynamic_property_kind(DynamicPropertyType::GradingPrimary));

    let cpu_prop = cpu
        .dynamic_property(DynamicPropertyType::GradingPrimary)
        .expect("cpu grading primary property");
    let cpu_value = cpu_prop
        .grading_primary_value()
        .expect("cpu grading primary value");
    assert_close(cpu_value.brightness.red, 0.125, 1e-8);
    assert_close(cpu_value.contrast.master, 1.15, 1e-8);
    assert_close(cpu_value.gain.blue, 1.25, 1e-8);
    assert_close(cpu_value.saturation, 0.9, 1e-8);

    let mut cpu_update = cpu_value.clone();
    cpu_update.offset.green = -0.05;
    cpu_prop
        .set_grading_primary_value(&cpu_update)
        .expect("set cpu grading primary value");
    let cpu_after_update = cpu_prop
        .grading_primary_value()
        .expect("cpu grading primary after cpu update");
    assert_close(cpu_after_update.offset.green, -0.05, 1e-8);
    let processor_after_cpu = processor_prop
        .grading_primary_value()
        .expect("processor grading primary after cpu update");
    assert_close(processor_after_cpu.offset.green, 0.0, 1e-8);
}

#[test]
fn dynamic_grading_tone_round_trip_between_processor_and_cpu() {
    let _guard = dynamic_property_test_lock();
    if is_stub() {
        return;
    }

    let processor = dynamic_grading_tone_processor().expect("dynamic grading tone processor");
    assert!(processor.is_dynamic());
    assert!(processor.has_dynamic_property_kind(DynamicPropertyType::GradingTone));

    let processor_prop = processor
        .dynamic_property(DynamicPropertyType::GradingTone)
        .expect("processor grading tone property");
    assert_eq!(
        processor_prop.property_type(),
        DynamicPropertyType::GradingTone
    );

    let mut value = processor_prop
        .grading_tone_value()
        .expect("grading tone value");
    value.blacks.red = 0.95;
    value.midtones.master = 1.1;
    value.highlights.width = 0.8;
    value.scontrast = 1.2;
    processor_prop
        .set_grading_tone_value(&value)
        .expect("set grading tone value");

    let round_trip = processor_prop
        .grading_tone_value()
        .expect("processor grading tone round trip");
    assert_close(round_trip.blacks.red, 0.95, 1e-8);
    assert_close(round_trip.midtones.master, 1.1, 1e-8);
    assert_close(round_trip.highlights.width, 0.8, 1e-8);
    assert_close(round_trip.scontrast, 1.2, 1e-8);

    let cpu = processor
        .default_cpu_processor()
        .expect("default cpu processor");
    let cpu_prop = cpu
        .dynamic_property(DynamicPropertyType::GradingTone)
        .expect("cpu grading tone property");
    let cpu_value = cpu_prop
        .grading_tone_value()
        .expect("cpu grading tone value");
    assert_close(cpu_value.blacks.red, 0.95, 1e-8);
    assert_close(cpu_value.midtones.master, 1.1, 1e-8);
    assert_close(cpu_value.highlights.width, 0.8, 1e-8);
    assert_close(cpu_value.scontrast, 1.2, 1e-8);

    let mut cpu_update = cpu_value.clone();
    cpu_update.whites.start = 0.65;
    cpu_prop
        .set_grading_tone_value(&cpu_update)
        .expect("set cpu grading tone value");
    let cpu_after_update = cpu_prop
        .grading_tone_value()
        .expect("cpu grading tone after cpu update");
    assert_close(cpu_after_update.whites.start, 0.65, 1e-8);
    let processor_after_cpu = processor_prop
        .grading_tone_value()
        .expect("processor grading tone after cpu update");
    assert_close(processor_after_cpu.whites.start, 0.4, 1e-8);
}

#[test]
fn dynamic_grading_rgb_curve_round_trip_between_processor_and_cpu() {
    let _guard = dynamic_property_test_lock();
    if is_stub() {
        return;
    }

    let processor =
        dynamic_grading_rgb_curve_processor().expect("dynamic grading rgb curve processor");
    assert!(processor.is_dynamic());
    assert!(processor.has_dynamic_property_kind(DynamicPropertyType::GradingRgbCurve));

    let processor_prop = processor
        .dynamic_property(DynamicPropertyType::GradingRgbCurve)
        .expect("processor grading rgb curve property");
    assert_eq!(
        processor_prop.property_type(),
        DynamicPropertyType::GradingRgbCurve
    );

    let points = [
        GradingCurvePoint::new(0.0, 0.0, 1.0),
        GradingCurvePoint::new(0.5, 0.6, 0.8),
        GradingCurvePoint::new(1.0, 1.0, 1.0),
    ];
    processor_prop
        .grading_rgb_curve_set_num_control_points(RGBCurveType::Red, points.len() as i32)
        .expect("set processor rgb curve point count");
    for (index, point) in points.iter().enumerate() {
        let index = index as i32;
        processor_prop
            .grading_rgb_curve_set_control_point(RGBCurveType::Red, index, point.x, point.y)
            .expect("set processor rgb curve control point");
        processor_prop
            .grading_rgb_curve_set_slope(RGBCurveType::Red, index, point.slope)
            .expect("set processor rgb curve slope");
    }

    assert_eq!(
        processor_prop
            .grading_rgb_curve_num_control_points(RGBCurveType::Red)
            .expect("processor rgb curve point count"),
        points.len() as i32
    );
    let (x, y) = processor_prop
        .grading_rgb_curve_control_point(RGBCurveType::Red, 1)
        .expect("processor rgb curve control point");
    assert_close(x as f64, 0.5, 1e-6);
    assert_close(y as f64, 0.6, 1e-6);
    assert_close(
        processor_prop
            .grading_rgb_curve_slope(RGBCurveType::Red, 1)
            .expect("processor rgb curve slope") as f64,
        0.8,
        1e-6,
    );
    assert!(!processor_prop
        .grading_rgb_curve_slopes_are_default(RGBCurveType::Red)
        .expect("processor rgb slopes are default"));

    let cpu = processor
        .default_cpu_processor()
        .expect("default cpu processor");
    let cpu_prop = cpu
        .dynamic_property(DynamicPropertyType::GradingRgbCurve)
        .expect("cpu grading rgb curve property");
    assert_eq!(
        cpu_prop
            .grading_rgb_curve_num_control_points(RGBCurveType::Red)
            .expect("cpu rgb curve point count"),
        points.len() as i32
    );
    let (cpu_x, cpu_y) = cpu_prop
        .grading_rgb_curve_control_point(RGBCurveType::Red, 1)
        .expect("cpu rgb curve control point");
    assert_close(cpu_x as f64, 0.5, 1e-6);
    assert_close(cpu_y as f64, 0.6, 1e-6);
    assert_close(
        cpu_prop
            .grading_rgb_curve_slope(RGBCurveType::Red, 1)
            .expect("cpu rgb curve slope") as f64,
        0.8,
        1e-6,
    );

    cpu_prop
        .grading_rgb_curve_set_slope(RGBCurveType::Red, 1, 0.33)
        .expect("set cpu rgb curve slope");
    assert_close(
        cpu_prop
            .grading_rgb_curve_slope(RGBCurveType::Red, 1)
            .expect("cpu rgb curve slope after update") as f64,
        0.33,
        1e-6,
    );
    assert_close(
        processor_prop
            .grading_rgb_curve_slope(RGBCurveType::Red, 1)
            .expect("processor rgb curve slope after cpu update") as f64,
        0.8,
        1e-6,
    );
}

#[test]
fn dynamic_grading_hue_curve_round_trip_between_processor_and_cpu() {
    let _guard = dynamic_property_test_lock();
    if is_stub() {
        return;
    }

    let processor =
        dynamic_grading_hue_curve_processor().expect("dynamic grading hue curve processor");
    assert!(processor.is_dynamic());
    assert!(processor.has_dynamic_property_kind(DynamicPropertyType::GradingHueCurve));

    let processor_prop = processor
        .dynamic_property(DynamicPropertyType::GradingHueCurve)
        .expect("processor grading hue curve property");
    assert_eq!(
        processor_prop.property_type(),
        DynamicPropertyType::GradingHueCurve
    );

    let points = [
        GradingCurvePoint::new(0.0, 0.0, 1.0),
        GradingCurvePoint::new(1.0 / 6.0, 0.2, 0.5),
        GradingCurvePoint::new(1.0 / 3.0, 0.4, 1.0),
    ];
    processor_prop
        .grading_hue_curve_set_num_control_points(HueCurveType::HueHue, points.len() as i32)
        .expect("set processor hue curve point count");
    for (index, point) in points.iter().enumerate() {
        let index = index as i32;
        processor_prop
            .grading_hue_curve_set_control_point(HueCurveType::HueHue, index, point.x, point.y)
            .expect("set processor hue curve control point");
        processor_prop
            .grading_hue_curve_set_slope(HueCurveType::HueHue, index, point.slope)
            .expect("set processor hue curve slope");
    }

    assert_eq!(
        processor_prop
            .grading_hue_curve_num_control_points(HueCurveType::HueHue)
            .expect("processor hue curve point count"),
        points.len() as i32
    );
    let (x, y) = processor_prop
        .grading_hue_curve_control_point(HueCurveType::HueHue, 1)
        .expect("processor hue curve control point");
    assert_close(x as f64, 1.0 / 6.0, 1e-6);
    assert_close(y as f64, 0.2, 1e-6);
    assert_close(
        processor_prop
            .grading_hue_curve_slope(HueCurveType::HueHue, 1)
            .expect("processor hue curve slope") as f64,
        0.5,
        1e-6,
    );
    assert!(!processor_prop
        .grading_hue_curve_slopes_are_default(HueCurveType::HueHue)
        .expect("processor hue slopes are default"));

    let cpu = processor
        .default_cpu_processor()
        .expect("default cpu processor");
    let cpu_prop = cpu
        .dynamic_property(DynamicPropertyType::GradingHueCurve)
        .expect("cpu grading hue curve property");
    assert_eq!(
        cpu_prop
            .grading_hue_curve_num_control_points(HueCurveType::HueHue)
            .expect("cpu hue curve point count"),
        points.len() as i32
    );
    let (cpu_x, cpu_y) = cpu_prop
        .grading_hue_curve_control_point(HueCurveType::HueHue, 1)
        .expect("cpu hue curve control point");
    assert_close(cpu_x as f64, 1.0 / 6.0, 1e-6);
    assert_close(cpu_y as f64, 0.2, 1e-6);
    assert_close(
        cpu_prop
            .grading_hue_curve_slope(HueCurveType::HueHue, 1)
            .expect("cpu hue curve slope") as f64,
        0.5,
        1e-6,
    );

    cpu_prop
        .grading_hue_curve_set_slope(HueCurveType::HueHue, 1, 0.25)
        .expect("set cpu hue curve slope");
    assert_close(
        cpu_prop
            .grading_hue_curve_slope(HueCurveType::HueHue, 1)
            .expect("cpu hue curve slope after update") as f64,
        0.25,
        1e-6,
    );
    assert_close(
        processor_prop
            .grading_hue_curve_slope(HueCurveType::HueHue, 1)
            .expect("processor hue curve slope after cpu update") as f64,
        0.5,
        1e-6,
    );
}

#[test]
fn dynamic_property_type_mismatch_surfaces_invalid_input_behavior() {
    let _guard = dynamic_property_test_lock();
    if is_stub() {
        return;
    }

    let exposure_processor = dynamic_exposure_processor().expect("dynamic exposure processor");
    let exposure_prop = exposure_processor
        .dynamic_property(DynamicPropertyType::Exposure)
        .expect("exposure property");

    let grading_primary_err = exposure_prop
        .grading_primary_value()
        .expect_err("reading grading primary from exposure property should fail");
    assert!(
        matches!(grading_primary_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {grading_primary_err:?}"
    );

    let set_grading_primary_err = exposure_prop
        .set_grading_primary_value(&ocio_rs::grading::GradingPrimary::new(GradingStyle::Log))
        .expect_err("setting grading primary on exposure property should fail");
    assert!(
        matches!(set_grading_primary_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {set_grading_primary_err:?}"
    );

    let grading_primary_processor =
        dynamic_grading_primary_processor().expect("dynamic grading primary processor");
    let grading_primary_prop = grading_primary_processor
        .dynamic_property(DynamicPropertyType::GradingPrimary)
        .expect("grading primary property");

    let double_err = grading_primary_prop
        .double_value()
        .expect_err("reading double from grading primary property should fail");
    assert!(
        matches!(double_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {double_err:?}"
    );

    let set_double_err = grading_primary_prop
        .set_double_value(1.0)
        .expect_err("setting double on grading primary property should fail");
    assert!(
        matches!(set_double_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {set_double_err:?}"
    );

    let grading_rgb_curve_err = exposure_prop
        .grading_rgb_curve_num_control_points(RGBCurveType::Red)
        .expect_err("reading rgb curve from exposure property should fail");
    assert!(
        matches!(grading_rgb_curve_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {grading_rgb_curve_err:?}"
    );

    let grading_hue_curve_err = exposure_prop
        .grading_hue_curve_num_control_points(HueCurveType::HueHue)
        .expect_err("reading hue curve from exposure property should fail");
    assert!(
        matches!(grading_hue_curve_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {grading_hue_curve_err:?}"
    );
}

#[test]
fn dynamic_property_curve_invalid_operations_surface_errors() {
    let _guard = dynamic_property_test_lock();
    if is_stub() {
        return;
    }

    let rgb_processor =
        dynamic_grading_rgb_curve_processor().expect("dynamic grading rgb curve processor");
    let rgb_prop = rgb_processor
        .dynamic_property(DynamicPropertyType::GradingRgbCurve)
        .expect("grading rgb curve property");

    let negative_count_err = rgb_prop
        .grading_rgb_curve_set_num_control_points(RGBCurveType::Red, -1)
        .expect_err("negative rgb point count should fail");
    assert!(
        matches!(negative_count_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {negative_count_err:?}"
    );

    let negative_index_err = rgb_prop
        .grading_rgb_curve_control_point(RGBCurveType::Red, -1)
        .expect_err("negative rgb control point index should fail");
    assert!(
        matches!(negative_index_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {negative_index_err:?}"
    );

    rgb_prop
        .grading_rgb_curve_set_num_control_points(RGBCurveType::Red, 2)
        .expect("seed rgb point count");

    let too_few_points_err = rgb_prop
        .grading_rgb_curve_set_num_control_points(RGBCurveType::Red, 1)
        .expect_err("too few rgb control points should fail in real OCIO");
    assert!(
        !matches!(too_few_points_err, ocio_rs::OcioError::InvalidInput(_)),
        "expected OCIO runtime validation error, got: {too_few_points_err:?}"
    );

    rgb_prop
        .grading_rgb_curve_set_num_control_points(RGBCurveType::Red, 2)
        .expect("restore rgb point count");

    rgb_prop
        .grading_rgb_curve_control_point(RGBCurveType::Red, 99)
        .expect_err("out-of-range rgb control point should fail");
    rgb_prop
        .grading_rgb_curve_set_slope(RGBCurveType::Red, 99, 0.5)
        .expect_err("out-of-range rgb slope should fail");

    let hue_processor =
        dynamic_grading_hue_curve_processor().expect("dynamic grading hue curve processor");
    let hue_prop = hue_processor
        .dynamic_property(DynamicPropertyType::GradingHueCurve)
        .expect("grading hue curve property");

    let negative_hue_count_err = hue_prop
        .grading_hue_curve_set_num_control_points(HueCurveType::HueHue, -1)
        .expect_err("negative hue point count should fail");
    assert!(
        matches!(negative_hue_count_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {negative_hue_count_err:?}"
    );

    let negative_hue_index_err = hue_prop
        .grading_hue_curve_control_point(HueCurveType::HueHue, -1)
        .expect_err("negative hue control point index should fail");
    assert!(
        matches!(negative_hue_index_err, ocio_rs::OcioError::InvalidInput(_)),
        "unexpected error variant: {negative_hue_index_err:?}"
    );

    hue_prop
        .grading_hue_curve_set_num_control_points(HueCurveType::HueHue, 2)
        .expect("seed hue point count");

    let too_few_hue_points_err = hue_prop
        .grading_hue_curve_set_num_control_points(HueCurveType::HueHue, 1)
        .expect_err("too few hue control points should fail in real OCIO");
    assert!(
        !matches!(too_few_hue_points_err, ocio_rs::OcioError::InvalidInput(_)),
        "expected OCIO runtime validation error, got: {too_few_hue_points_err:?}"
    );

    hue_prop
        .grading_hue_curve_set_num_control_points(HueCurveType::HueHue, 2)
        .expect("restore hue point count");

    hue_prop
        .grading_hue_curve_control_point(HueCurveType::HueHue, 99)
        .expect_err("out-of-range hue control point should fail");
    hue_prop
        .grading_hue_curve_set_slope(HueCurveType::HueHue, 99, 0.5)
        .expect_err("out-of-range hue slope should fail");
}
