//! Look behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/look.rs`. In bundled/real mode they validate metadata, interchange
//! attributes, attached transform round trips, editable-copy independence, and
//! look application through a display pipeline including `looks_bypass`.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::{DisplayViewTransform, MatrixTransform, Transform};
use ocio_rs::{
    Allocation, ColorSpace, Config, Look, ReferenceSpaceType, TransformDirection, ViewTransform,
    ViewTransformDirection,
};

fn look_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn identity_color_space(name: &str) -> ColorSpace {
    let cs = ColorSpace::create().expect("color space create");
    cs.set_name(name).expect("set color space name");
    cs.set_family("Unit/Looks").expect("set color space family");
    cs.set_description("look behavior driver color space")
        .expect("set color space description");
    cs.set_is_data(false);
    cs.set_allocation(Allocation::Lg2);
    cs.set_allocation_vars(&[-8.0, 8.0])
        .expect("set allocation variables");
    cs
}

fn identity_view_transform(name: &str) -> ViewTransform {
    let vt = ViewTransform::create(ReferenceSpaceType::Scene).expect("view transform create");
    vt.set_name(name).expect("set view transform name");

    let identity = MatrixTransform::identity().expect("identity matrix");
    vt.set_transform(Some(&identity), ViewTransformDirection::ToReference);
    vt.set_transform(Some(&identity), ViewTransformDirection::FromReference);
    vt
}

fn scaling_look(name: &str, process_space: &str) -> Look {
    let look = Look::create().expect("look create");
    look.set_name(name).expect("set look name");
    look.set_process_space(process_space)
        .expect("set process space");
    look.set_description("look behavior test")
        .expect("set look description");
    look.set_interchange_attribute("amf_transform_ids", "urn:test:look")
        .expect("set interchange attribute");

    let forward = MatrixTransform::scale(&[1.5, 1.0, 1.0, 1.0]).expect("forward look matrix");
    let inverse = MatrixTransform::scale(&[2.0 / 3.0, 1.0, 1.0, 1.0]).expect("inverse look matrix");
    look.set_transform(&forward);
    look.set_inverse_transform(&inverse);
    look
}

fn configured_look_pipeline() -> Config {
    let config = create_test_config().expect("raw config");
    let source_color_space = identity_color_space("UnitLookInput");
    let view_transform = identity_view_transform("UnitLookViewTransform");
    let look = scaling_look("UnitLook", "UnitLookInput");

    config.add_color_space(&source_color_space);
    config.add_view_transform(&view_transform);
    config.add_look(&look);
    config
        .add_display_view_detailed(
            "UnitDisplay",
            "UnitView",
            "UnitLookViewTransform",
            "UnitLookInput",
            "UnitLook",
            "",
            "unit look display pipeline",
        )
        .expect("add display view");
    config
}

#[test]
fn look_metadata_transform_and_copy_behavior() {
    let _guard = look_test_lock();
    if is_stub() {
        return;
    }

    let look = scaling_look("UnitLook", "UnitLookInput");

    assert_eq!(look.name().as_deref(), Some("UnitLook"));
    assert_eq!(look.process_space().as_deref(), Some("UnitLookInput"));
    assert_eq!(look.description().as_deref(), Some("look behavior test"));
    assert_eq!(
        look.interchange_attribute("amf_transform_ids").as_deref(),
        Some("urn:test:look")
    );
    assert_eq!(
        look.interchange_attributes()
            .get("amf_transform_ids")
            .map(String::as_str),
        Some("urn:test:look")
    );

    match look
        .try_transform()
        .expect("look forward transform query")
        .expect("look forward transform")
    {
        Transform::Matrix(matrix) => {
            let values = matrix.matrix();
            assert_close(values[0], 1.5, 1e-10);
            assert_close(values[5], 1.0, 1e-10);
            assert_close(values[10], 1.0, 1e-10);
        }
        _ => panic!("expected look forward transform to be MatrixTransform"),
    }

    match look
        .try_inverse_transform()
        .expect("look inverse transform query")
        .expect("look inverse transform")
    {
        Transform::Matrix(matrix) => {
            let values = matrix.matrix();
            assert_close(values[0], 2.0 / 3.0, 1e-10);
            assert_close(values[5], 1.0, 1e-10);
            assert_close(values[10], 1.0, 1e-10);
        }
        _ => panic!("expected look inverse transform to be MatrixTransform"),
    }

    let copy = look.create_editable_copy().expect("look editable copy");
    copy.set_name("UnitLookCopy").expect("rename copy");
    copy.set_description("look behavior copy")
        .expect("change copy description");

    assert_eq!(copy.name().as_deref(), Some("UnitLookCopy"));
    assert_eq!(copy.description().as_deref(), Some("look behavior copy"));
    assert_eq!(look.name().as_deref(), Some("UnitLook"));
    assert_eq!(look.description().as_deref(), Some("look behavior test"));
}

#[test]
fn look_display_pipeline_and_looks_bypass_behavior() {
    let _guard = look_test_lock();
    if is_stub() {
        return;
    }

    let config = configured_look_pipeline();
    assert_eq!(
        config
            .display_view_looks("UnitDisplay", "UnitView")
            .as_deref(),
        Some("UnitLook")
    );

    let applied = DisplayViewTransform::create().expect("applied display view transform");
    applied.set_src("UnitLookInput").expect("set src");
    applied.set_display("UnitDisplay").expect("set display");
    applied.set_view("UnitView").expect("set view");
    applied.set_looks_bypass(false);

    let bypassed = DisplayViewTransform::create().expect("bypassed display view transform");
    bypassed.set_src("UnitLookInput").expect("set bypassed src");
    bypassed
        .set_display("UnitDisplay")
        .expect("set bypassed display");
    bypassed.set_view("UnitView").expect("set bypassed view");
    bypassed.set_looks_bypass(true);

    let direct_cpu = config
        .processor_display(
            "UnitLookInput",
            "UnitDisplay",
            "UnitView",
            TransformDirection::Forward,
        )
        .expect("processor display")
        .default_cpu_processor()
        .expect("direct cpu");
    let applied_cpu = config
        .processor_from_transform(&applied, TransformDirection::Forward)
        .expect("applied processor")
        .default_cpu_processor()
        .expect("applied cpu");
    let bypassed_cpu = config
        .processor_from_transform(&bypassed, TransformDirection::Forward)
        .expect("bypassed processor")
        .default_cpu_processor()
        .expect("bypassed cpu");

    let original = [0.4f32, 0.5, 0.6, 1.0];
    let mut direct_pixel = original;
    let mut applied_pixel = original;
    let mut bypassed_pixel = original;
    direct_cpu.apply_rgba(&mut direct_pixel);
    applied_cpu.apply_rgba(&mut applied_pixel);
    bypassed_cpu.apply_rgba(&mut bypassed_pixel);

    assert_close(direct_pixel[0] as f64, 0.6, 1e-6);
    assert_close(direct_pixel[1] as f64, 0.5, 1e-6);
    assert_close(direct_pixel[2] as f64, 0.6, 1e-6);
    assert_close(direct_pixel[3] as f64, 1.0, 1e-6);

    assert_close(applied_pixel[0] as f64, direct_pixel[0] as f64, 1e-6);
    assert_close(applied_pixel[1] as f64, direct_pixel[1] as f64, 1e-6);
    assert_close(applied_pixel[2] as f64, direct_pixel[2] as f64, 1e-6);
    assert_close(applied_pixel[3] as f64, direct_pixel[3] as f64, 1e-6);

    assert_close(bypassed_pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(bypassed_pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(bypassed_pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(bypassed_pixel[3] as f64, original[3] as f64, 1e-6);
}

#[test]
fn look_interchange_attribute_errors_surface_behavior() {
    let _guard = look_test_lock();
    if is_stub() {
        return;
    }

    let look = Look::create().expect("look create");
    let invalid_attr_err = look
        .set_interchange_attribute("definitely_unknown_attr", "value")
        .expect_err("unknown interchange attribute should fail");
    assert!(
        matches!(invalid_attr_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {invalid_attr_err:?}"
    );
}
