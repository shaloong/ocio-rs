//! DisplayViewTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/display_view.rs`. In bundled/real mode they validate
//! metadata round trips, editable-copy independence, processor execution, and
//! equivalence with the config display/view processor entry point.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::{DisplayViewTransform, MatrixTransform};
use ocio_rs::{
    Allocation, ColorSpace, Config, ReferenceSpaceType, TransformDirection, ViewTransform,
    ViewTransformDirection,
};

fn display_view_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn identity_color_space(name: &str, is_data: bool) -> ColorSpace {
    let cs = ColorSpace::create().expect("color space create");
    cs.set_name(name).expect("set name");
    cs.set_family("Unit/DisplayViewTransforms")
        .expect("set family");
    cs.set_description("display view transform driver color space")
        .expect("set description");
    cs.set_is_data(is_data);
    cs.set_allocation(Allocation::Lg2);
    cs.set_allocation_vars(&[-8.0, 8.0])
        .expect("set allocation variables");
    cs
}

fn scaled_view_transform(name: &str) -> ViewTransform {
    let vt = ViewTransform::create(ReferenceSpaceType::Scene).expect("view transform create");
    vt.set_name(name).expect("set name");
    vt.set_description("display view transform behavior test")
        .expect("set description");

    let to_reference = MatrixTransform::scale(&[2.0, 1.0, 0.5, 1.0]).expect("to-reference matrix");
    let from_reference =
        MatrixTransform::scale(&[0.5, 1.0, 2.0, 1.0]).expect("from-reference matrix");
    vt.set_transform(Some(&to_reference), ViewTransformDirection::ToReference);
    vt.set_transform(Some(&from_reference), ViewTransformDirection::FromReference);
    vt
}

fn configured_display_pipeline(source_name: &str, is_data: bool) -> Config {
    let config = create_test_config().expect("raw config");
    let source_color_space = identity_color_space(source_name, is_data);
    let view_transform = scaled_view_transform("UnitDisplayViewTransform");
    config.add_color_space(&source_color_space);
    config.add_view_transform(&view_transform);
    config
        .add_display_view_detailed(
            "UnitDisplay",
            "UnitView",
            "UnitDisplayViewTransform",
            source_name,
            "",
            "",
            "unit display pipeline",
        )
        .expect("add display view");
    config
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn display_view_transform_metadata_copy_and_validate_behavior() {
    let _guard = display_view_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = DisplayViewTransform::create().expect("display view transform create");
    transform.set_src("UnitInput").expect("set src");
    transform.set_display("UnitDisplay").expect("set display");
    transform.set_view("UnitView").expect("set view");
    transform
        .try_set_looks_bypass(true)
        .expect("set looks bypass");
    transform
        .try_set_data_bypass(true)
        .expect("set data bypass");
    transform
        .try_set_direction(TransformDirection::Inverse)
        .expect("set direction");
    transform
        .validate()
        .expect("validate display view transform");

    assert_eq!(transform.src().as_deref(), Some("UnitInput"));
    assert_eq!(transform.display().as_deref(), Some("UnitDisplay"));
    assert_eq!(transform.view().as_deref(), Some("UnitView"));
    assert!(transform.looks_bypass());
    assert!(transform.data_bypass());
    assert_eq!(transform.direction(), TransformDirection::Inverse);

    let copy = transform
        .create_editable_copy()
        .expect("display view editable copy");
    copy.set_view("UnitViewCopy").expect("set copy view");
    copy.try_set_data_bypass(false)
        .expect("set copy data bypass");
    copy.try_set_direction(TransformDirection::Forward)
        .expect("set copy direction");

    assert_eq!(copy.view().as_deref(), Some("UnitViewCopy"));
    assert!(!copy.data_bypass());
    assert_eq!(copy.direction(), TransformDirection::Forward);

    assert_eq!(transform.view().as_deref(), Some("UnitView"));
    assert!(transform.data_bypass());
    assert_eq!(transform.direction(), TransformDirection::Inverse);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn display_view_transform_matches_processor_display_behavior() {
    let _guard = display_view_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = configured_display_pipeline("UnitViewInput", false);
    let transform = DisplayViewTransform::create().expect("display view transform create");
    transform.set_src("UnitViewInput").expect("set src");
    transform.set_display("UnitDisplay").expect("set display");
    transform.set_view("UnitView").expect("set view");

    let transform_processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .expect("processor from transform");
    let direct_processor = config
        .processor_display(
            "UnitViewInput",
            "UnitDisplay",
            "UnitView",
            TransformDirection::Forward,
        )
        .expect("processor display");

    let transform_cpu = transform_processor
        .default_cpu_processor()
        .expect("transform cpu");
    let direct_cpu = direct_processor
        .default_cpu_processor()
        .expect("direct cpu");

    let mut transform_pixel = [0.25f32, 0.5, 0.5, 1.0];
    let mut direct_pixel = transform_pixel;
    transform_cpu.apply_rgba(&mut transform_pixel);
    direct_cpu.apply_rgba(&mut direct_pixel);

    assert_close(transform_pixel[0] as f64, 0.125, 1e-6);
    assert_close(transform_pixel[1] as f64, 0.5, 1e-6);
    assert_close(transform_pixel[2] as f64, 1.0, 1e-6);
    assert_close(transform_pixel[3] as f64, 1.0, 1e-6);

    assert_close(direct_pixel[0] as f64, transform_pixel[0] as f64, 1e-6);
    assert_close(direct_pixel[1] as f64, transform_pixel[1] as f64, 1e-6);
    assert_close(direct_pixel[2] as f64, transform_pixel[2] as f64, 1e-6);
    assert_close(direct_pixel[3] as f64, transform_pixel[3] as f64, 1e-6);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn display_view_transform_data_bypass_behavior() {
    let _guard = display_view_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = configured_display_pipeline("UnitDataInput", true);

    let bypassed = DisplayViewTransform::create().expect("bypassed display view transform");
    bypassed.set_src("UnitDataInput").expect("set bypassed src");
    bypassed
        .set_display("UnitDisplay")
        .expect("set bypassed display");
    bypassed.set_view("UnitView").expect("set bypassed view");
    bypassed
        .try_set_data_bypass(true)
        .expect("set bypassed data bypass");

    let forced = DisplayViewTransform::create().expect("forced display view transform");
    forced.set_src("UnitDataInput").expect("set forced src");
    forced
        .set_display("UnitDisplay")
        .expect("set forced display");
    forced.set_view("UnitView").expect("set forced view");
    forced
        .try_set_data_bypass(false)
        .expect("set forced data bypass");

    let bypassed_cpu = config
        .processor_from_transform(&bypassed, TransformDirection::Forward)
        .expect("bypassed processor")
        .default_cpu_processor()
        .expect("bypassed cpu");
    let forced_cpu = config
        .processor_from_transform(&forced, TransformDirection::Forward)
        .expect("forced processor")
        .default_cpu_processor()
        .expect("forced cpu");

    let original = [0.25f32, 0.5, 0.5, 1.0];
    let mut bypassed_pixel = original;
    let mut forced_pixel = original;
    bypassed_cpu.apply_rgba(&mut bypassed_pixel);
    forced_cpu.apply_rgba(&mut forced_pixel);

    assert_close(bypassed_pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(bypassed_pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(bypassed_pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(bypassed_pixel[3] as f64, original[3] as f64, 1e-6);

    assert_close(forced_pixel[0] as f64, 0.125, 1e-6);
    assert_close(forced_pixel[1] as f64, 0.5, 1e-6);
    assert_close(forced_pixel[2] as f64, 1.0, 1e-6);
    assert_close(forced_pixel[3] as f64, 1.0, 1e-6);
}
