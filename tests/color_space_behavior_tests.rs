//! ColorSpace behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/colorspace.rs`. In bundled/real mode they validate metadata,
//! alias/category handling, attached transforms, config registration, and
//! processor execution across custom color spaces.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::MatrixTransform;
use ocio_rs::{Allocation, ColorSpace, ColorSpaceDirection, Config, ReferenceSpaceType};

fn color_space_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn scaled_color_space(
    name: &str,
    alias: &str,
    category: &str,
    equality_group: &str,
    scale: [f64; 4],
    inverse_scale: [f64; 4],
) -> ColorSpace {
    let cs = ColorSpace::create().expect("color space create");
    cs.set_name(name).expect("set name");
    cs.set_family("Unit/ColorSpaces").expect("set family");
    cs.set_description("color space behavior test")
        .expect("set description");
    cs.set_equality_group(equality_group)
        .expect("set equality group");
    cs.set_encoding("scene-linear").expect("set encoding");
    cs.set_is_data(false);
    cs.set_allocation(Allocation::Lg2);
    cs.set_allocation_vars(&[-8.0, 8.0])
        .expect("set allocation variables");
    cs.add_alias(alias).expect("add alias");
    cs.add_category(category).expect("add category");
    cs.set_interchange_attribute("amf_transform_ids", "urn:test:colorspace")
        .expect("set interchange attribute");

    let to_reference = MatrixTransform::scale(&scale).expect("to-reference matrix");
    let from_reference = MatrixTransform::scale(&inverse_scale).expect("from-reference matrix");
    cs.try_set_transform(&to_reference, ColorSpaceDirection::ToReference)
        .expect("attach to-reference transform");
    cs.try_set_transform(&from_reference, ColorSpaceDirection::FromReference)
        .expect("attach from-reference transform");
    cs
}

#[test]
fn color_space_metadata_alias_category_round_trip_behavior() {
    let _guard = color_space_test_lock();
    if is_stub() {
        return;
    }

    let cs = scaled_color_space(
        "UnitColorSpaceA",
        "unit_cs_a",
        "unit_category",
        "unit-group-a",
        [2.0, 1.0, 0.5, 1.0],
        [0.5, 1.0, 2.0, 1.0],
    );

    assert_eq!(cs.name().as_deref(), Some("UnitColorSpaceA"));
    assert_eq!(cs.family().as_deref(), Some("Unit/ColorSpaces"));
    assert_eq!(
        cs.description().as_deref(),
        Some("color space behavior test")
    );
    assert_eq!(cs.equality_group().as_deref(), Some("unit-group-a"));
    assert_eq!(cs.encoding().as_deref(), Some("scene-linear"));
    assert_eq!(cs.reference_space_type(), ReferenceSpaceType::Scene);
    assert!(!cs.is_data());
    assert_eq!(cs.allocation(), Allocation::Lg2);
    assert_eq!(cs.allocation_vars(), vec![-8.0, 8.0]);

    assert_eq!(cs.num_aliases(), 1);
    assert_eq!(cs.alias(0).as_deref(), Some("unit_cs_a"));
    assert!(cs.has_alias("unit_cs_a"));

    assert_eq!(cs.num_categories(), 1);
    assert_eq!(cs.category().as_deref(), Some("unit_category"));
    assert!(cs.has_category("unit_category"));

    let _ = cs.interop_id();
    assert_eq!(
        cs.interchange_attribute("amf_transform_ids").as_deref(),
        Some("urn:test:colorspace")
    );
}

#[test]
fn color_space_attached_transform_and_copy_behavior() {
    let _guard = color_space_test_lock();
    if is_stub() {
        return;
    }

    let cs = scaled_color_space(
        "UnitColorSpaceA",
        "unit_cs_a",
        "unit_category",
        "unit-group-a",
        [2.0, 1.0, 0.5, 1.0],
        [0.5, 1.0, 2.0, 1.0],
    );

    assert!(cs.is_transform_defined(ColorSpaceDirection::ToReference));
    assert!(cs.is_transform_defined(ColorSpaceDirection::FromReference));

    match cs
        .transform(ColorSpaceDirection::ToReference)
        .expect("to-reference transform")
    {
        ocio_rs::transform::Transform::Matrix(matrix) => {
            let values = matrix.matrix();
            assert_close(values[0], 2.0, 1e-10);
            assert_close(values[5], 1.0, 1e-10);
            assert_close(values[10], 0.5, 1e-10);
            assert_close(values[15], 1.0, 1e-10);
        }
        _ => panic!("expected ToReference transform to be MatrixTransform"),
    }

    let copy = cs.create_editable_copy().expect("editable copy");
    copy.set_name("UnitColorSpaceCopy").expect("rename copy");
    copy.remove_alias("unit_cs_a")
        .expect("remove alias from copy");
    copy.remove_category("unit_category")
        .expect("remove category from copy");

    assert_eq!(copy.name().as_deref(), Some("UnitColorSpaceCopy"));
    assert!(!copy.has_alias("unit_cs_a"));
    assert!(!copy.has_category("unit_category"));

    assert_eq!(cs.name().as_deref(), Some("UnitColorSpaceA"));
    assert!(cs.has_alias("unit_cs_a"));
    assert!(cs.has_category("unit_category"));
}

#[test]
fn color_space_config_registration_lookup_and_processor_behavior() {
    let _guard = color_space_test_lock();
    if is_stub() {
        return;
    }

    let config = Config::raw().expect("raw config");
    let initial_count = config.num_color_spaces();

    let src_cs = scaled_color_space(
        "UnitColorSpaceA",
        "unit_cs_a",
        "unit_category",
        "unit-group-a",
        [2.0, 1.0, 0.5, 1.0],
        [0.5, 1.0, 2.0, 1.0],
    );
    let dst_cs = scaled_color_space(
        "UnitColorSpaceB",
        "unit_cs_b",
        "display_category",
        "unit-group-b",
        [1.0, 1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    );

    config.add_color_space(&src_cs);
    config.add_color_space(&dst_cs);

    assert_eq!(config.num_color_spaces(), initial_count + 2);
    assert!(config.color_space("UnitColorSpaceA").is_some());
    assert!(config.color_space("UnitColorSpaceB").is_some());
    assert_eq!(
        config.canonical_name("unit_cs_a").as_deref(),
        Some("UnitColorSpaceA")
    );
    assert_eq!(
        config.canonical_name("unit_cs_b").as_deref(),
        Some("UnitColorSpaceB")
    );
    assert!(config.color_space_index("UnitColorSpaceA") >= 0);

    let looked_up = config
        .color_space("UnitColorSpaceA")
        .expect("lookup color space");
    assert_eq!(looked_up.name().as_deref(), Some("UnitColorSpaceA"));
    assert!(looked_up.has_alias("unit_cs_a"));
    assert!(looked_up.has_category("unit_category"));

    let processor_by_name = config
        .processor("UnitColorSpaceA", "UnitColorSpaceB")
        .expect("processor by name");
    let cpu_by_name = processor_by_name
        .default_cpu_processor()
        .expect("cpu by name");
    let mut pixel = [0.25f32, 0.5, 0.5, 1.0];
    cpu_by_name.apply_rgba(&mut pixel);
    assert_close(pixel[0] as f64, 0.5, 1e-6);
    assert_close(pixel[1] as f64, 0.5, 1e-6);
    assert_close(pixel[2] as f64, 0.25, 1e-6);
    assert_close(pixel[3] as f64, 1.0, 1e-6);

    let src_lookup = config.color_space("UnitColorSpaceA").expect("src lookup");
    let dst_lookup = config.color_space("UnitColorSpaceB").expect("dst lookup");
    let processor_by_object = config
        .processor_from_color_spaces(&src_lookup, &dst_lookup)
        .expect("processor by object");
    let cpu_by_object = processor_by_object
        .default_cpu_processor()
        .expect("cpu by object");
    let mut second_pixel = [0.25f32, 0.5, 0.5, 1.0];
    cpu_by_object.apply_rgba(&mut second_pixel);
    assert_close(second_pixel[0] as f64, 0.5, 1e-6);
    assert_close(second_pixel[1] as f64, 0.5, 1e-6);
    assert_close(second_pixel[2] as f64, 0.25, 1e-6);
    assert_close(second_pixel[3] as f64, 1.0, 1e-6);
}

#[test]
fn color_space_interop_and_interchange_errors_surface_behavior() {
    let _guard = color_space_test_lock();
    if is_stub() {
        return;
    }

    let cs = ColorSpace::create().expect("color space create");

    cs.set_interop_id("aces:cg").expect("set valid interop id");
    assert_eq!(cs.interop_id().as_deref(), Some("aces:cg"));

    let invalid_interop_id_err = cs
        .set_interop_id("ACES bad namespace")
        .expect_err("invalid interop id should fail");
    assert!(
        matches!(invalid_interop_id_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {invalid_interop_id_err:?}"
    );

    let invalid_interchange_attr_err = cs
        .set_interchange_attribute("definitely_unknown_attr", "value")
        .expect_err("unknown interchange attribute should fail");
    assert!(
        matches!(invalid_interchange_attr_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {invalid_interchange_attr_err:?}"
    );
}
