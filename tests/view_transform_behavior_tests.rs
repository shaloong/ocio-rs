//! ViewTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/view_transform.rs`. In bundled/real mode they validate metadata,
//! category and interchange-attribute handling, attached transform round trips,
//! editable-copy independence, and real processor execution through the config.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::{DisplayViewTransform, MatrixTransform, Transform};
use ocio_rs::{
    Allocation, ColorSpace, ReferenceSpaceType, TransformDirection, ViewTransform,
    ViewTransformDirection,
};

fn view_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn scaled_view_transform() -> ViewTransform {
    let vt = ViewTransform::create(ReferenceSpaceType::Scene).expect("view transform create");
    vt.set_name("UnitViewTransform").expect("set name");
    vt.set_family("Unit/ViewTransforms").expect("set family");
    vt.set_description("view transform behavior test")
        .expect("set description");
    vt.add_category("unit_category").expect("add category");
    vt.set_interchange_attribute("amf_transform_ids", "urn:test:view-transform")
        .expect("set interchange attribute");

    let to_reference = MatrixTransform::scale(&[2.0, 1.0, 0.5, 1.0]).expect("to-reference matrix");
    let from_reference =
        MatrixTransform::scale(&[0.5, 1.0, 2.0, 1.0]).expect("from-reference matrix");
    vt.try_set_transform(Some(&to_reference), ViewTransformDirection::ToReference)
        .expect("attach to-reference view transform");
    vt.try_set_transform(Some(&from_reference), ViewTransformDirection::FromReference)
        .expect("attach from-reference view transform");
    vt
}

fn identity_color_space(name: &str) -> ColorSpace {
    let cs = ColorSpace::create().expect("color space create");
    cs.set_name(name).expect("set color space name");
    cs.set_family("Unit/ViewTransformTests")
        .expect("set color space family");
    cs.set_description("view transform driver color space")
        .expect("set color space description");
    cs.set_is_data(false);
    cs.set_allocation(Allocation::Lg2);
    cs.set_allocation_vars(&[-8.0, 8.0])
        .expect("set allocation variables");
    cs
}

#[test]
fn view_transform_metadata_category_and_interchange_behavior() {
    let _guard = view_transform_test_lock();
    if is_stub() {
        return;
    }

    let vt = scaled_view_transform();

    assert_eq!(vt.name().as_deref(), Some("UnitViewTransform"));
    assert_eq!(vt.family().as_deref(), Some("Unit/ViewTransforms"));
    assert_eq!(
        vt.description().as_deref(),
        Some("view transform behavior test")
    );
    assert_eq!(vt.reference_space_type(), ReferenceSpaceType::Scene);

    assert_eq!(vt.num_categories(), 1);
    assert_eq!(vt.category(0).as_deref(), Some("unit_category"));
    assert!(vt.has_category("unit_category"));

    assert_eq!(
        vt.interchange_attribute("amf_transform_ids").as_deref(),
        Some("urn:test:view-transform")
    );
    assert_eq!(
        vt.interchange_attributes()
            .get("amf_transform_ids")
            .map(String::as_str),
        Some("urn:test:view-transform")
    );
}

#[test]
fn view_transform_attached_transform_and_copy_behavior() {
    let _guard = view_transform_test_lock();
    if is_stub() {
        return;
    }

    let vt = scaled_view_transform();

    match vt
        .try_transform(ViewTransformDirection::ToReference)
        .expect("to-reference transform query")
        .expect("to-reference transform")
    {
        Transform::Matrix(matrix) => {
            let values = matrix.matrix();
            assert_close(values[0], 2.0, 1e-10);
            assert_close(values[5], 1.0, 1e-10);
            assert_close(values[10], 0.5, 1e-10);
            assert_close(values[15], 1.0, 1e-10);
        }
        _ => panic!("expected ToReference transform to be MatrixTransform"),
    }

    let copy = vt.create_editable_copy().expect("editable copy");
    copy.set_name("UnitViewTransformCopy").expect("rename copy");
    copy.remove_category("unit_category")
        .expect("remove category from copy");
    copy.add_category("copy_category")
        .expect("add category to copy");
    copy.try_clear_categories()
        .expect("clear categories from copy");

    assert_eq!(copy.name().as_deref(), Some("UnitViewTransformCopy"));
    assert_eq!(copy.num_categories(), 0);

    assert_eq!(vt.name().as_deref(), Some("UnitViewTransform"));
    assert!(vt.has_category("unit_category"));
}

#[test]
fn view_transform_display_pipeline_round_trip_behavior() {
    let _guard = view_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let vt = scaled_view_transform();
    let source_color_space = identity_color_space("UnitViewInput");
    config.add_color_space(&source_color_space);
    config.add_view_transform(&vt);
    config
        .add_display_view_detailed(
            "UnitDisplay",
            "UnitView",
            "UnitViewTransform",
            "UnitViewInput",
            "",
            "",
            "unit display view",
        )
        .expect("add display view");

    let display_view = DisplayViewTransform::create().expect("display view transform create");
    display_view.set_src("UnitViewInput").expect("set src");
    display_view
        .set_display("UnitDisplay")
        .expect("set display");
    display_view.set_view("UnitView").expect("set view");

    let forward_processor = config
        .processor_from_transform(&display_view, TransformDirection::Forward)
        .expect("forward processor");
    let inverse_processor = config
        .processor_from_transform(&display_view, TransformDirection::Inverse)
        .expect("inverse processor");

    let forward_cpu = forward_processor
        .default_cpu_processor()
        .expect("forward cpu");
    let inverse_cpu = inverse_processor
        .default_cpu_processor()
        .expect("inverse cpu");

    let original = [0.25f32, 0.5, 0.5, 1.0];
    let mut transformed = original;
    forward_cpu.apply_rgba(&mut transformed);

    // In the display pipeline, Forward uses the view transform's
    // FromReference branch for scene-to-display conversion.
    assert_close(transformed[0] as f64, 0.125, 1e-6);
    assert_close(transformed[1] as f64, 0.5, 1e-6);
    assert_close(transformed[2] as f64, 1.0, 1e-6);
    assert_close(transformed[3] as f64, 1.0, 1e-6);

    let mut round_tripped = transformed;
    inverse_cpu.apply_rgba(&mut round_tripped);

    assert_close(round_tripped[0] as f64, original[0] as f64, 1e-6);
    assert_close(round_tripped[1] as f64, original[1] as f64, 1e-6);
    assert_close(round_tripped[2] as f64, original[2] as f64, 1e-6);
    assert_close(round_tripped[3] as f64, original[3] as f64, 1e-6);
}

#[test]
fn view_transform_interchange_attribute_errors_surface_behavior() {
    let _guard = view_transform_test_lock();
    if is_stub() {
        return;
    }

    let vt = ViewTransform::create(ReferenceSpaceType::Scene).expect("view transform create");
    let invalid_attr_err = vt
        .set_interchange_attribute("definitely_unknown_attr", "value")
        .expect_err("unknown interchange attribute should fail");
    assert!(
        matches!(invalid_attr_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {invalid_attr_err:?}"
    );
}
