//! NamedTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/named_transform.rs`. In bundled/real mode they validate metadata,
//! aliases, categories, attached transforms, config registration, and
//! processor execution through both object and name-based lookup paths.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::MatrixTransform;
use ocio_rs::{Config, NamedTransform, TransformDirection};

fn named_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configured_named_transform() -> NamedTransform {
    let nt = NamedTransform::create().expect("named transform create");
    nt.set_name("UnitNamedTransform").expect("set name");
    nt.set_family("Unit/Family").expect("set family");
    nt.set_description("named transform behavior test")
        .expect("set description");
    nt.set_encoding("scene-linear").expect("set encoding");
    nt.add_alias("unit_alias").expect("add alias");
    nt.add_category("unit_category").expect("add category");

    let forward = MatrixTransform::scale(&[2.0, 1.0, 0.5, 1.0]).expect("forward matrix");
    let inverse = MatrixTransform::scale(&[0.5, 1.0, 2.0, 1.0]).expect("inverse matrix");
    nt.try_set_transform(&forward, TransformDirection::Forward)
        .expect("attach forward named transform");
    nt.try_set_transform(&inverse, TransformDirection::Inverse)
        .expect("attach inverse named transform");
    nt
}

#[test]
fn named_transform_metadata_alias_category_round_trip_behavior() {
    let _guard = named_transform_test_lock();
    if is_stub() {
        return;
    }

    let nt = configured_named_transform();

    assert_eq!(nt.name().as_deref(), Some("UnitNamedTransform"));
    assert_eq!(nt.family().as_deref(), Some("Unit/Family"));
    assert_eq!(
        nt.description().as_deref(),
        Some("named transform behavior test")
    );
    assert_eq!(nt.encoding().as_deref(), Some("scene-linear"));

    assert_eq!(nt.num_aliases(), 1);
    assert_eq!(nt.alias(0).as_deref(), Some("unit_alias"));
    assert!(nt.has_alias("unit_alias"));

    assert_eq!(nt.num_categories(), 1);
    assert_eq!(nt.category(0).as_deref(), Some("unit_category"));
    assert!(nt.has_category("unit_category"));
}

#[test]
fn named_transform_attached_matrix_round_trip_behavior() {
    let _guard = named_transform_test_lock();
    if is_stub() {
        return;
    }

    let nt = configured_named_transform();

    let forward = nt
        .transform(TransformDirection::Forward)
        .expect("forward transform");
    let inverse = nt
        .transform(TransformDirection::Inverse)
        .expect("inverse transform");

    match forward {
        ocio_rs::transform::Transform::Matrix(matrix) => {
            let values = matrix.matrix();
            assert_close(values[0], 2.0, 1e-10);
            assert_close(values[5], 1.0, 1e-10);
            assert_close(values[10], 0.5, 1e-10);
            assert_close(values[15], 1.0, 1e-10);
        }
        _ => panic!("expected forward named transform to be MatrixTransform"),
    }

    match inverse {
        ocio_rs::transform::Transform::Matrix(matrix) => {
            let values = matrix.matrix();
            assert_close(values[0], 0.5, 1e-10);
            assert_close(values[5], 1.0, 1e-10);
            assert_close(values[10], 2.0, 1e-10);
            assert_close(values[15], 1.0, 1e-10);
        }
        _ => panic!("expected inverse named transform to be MatrixTransform"),
    }
}

#[test]
fn named_transform_editable_copy_is_independent_behavior() {
    let _guard = named_transform_test_lock();
    if is_stub() {
        return;
    }

    let nt = configured_named_transform();
    let copy = nt.create_editable_copy().expect("editable copy");

    assert_eq!(copy.name().as_deref(), Some("UnitNamedTransform"));
    assert!(copy.has_alias("unit_alias"));
    assert!(copy.has_category("unit_category"));

    copy.set_name("UnitNamedTransformCopy")
        .expect("rename copy");
    copy.remove_alias("unit_alias")
        .expect("remove alias from copy");
    copy.remove_category("unit_category")
        .expect("remove category from copy");
    copy.add_alias("copy_alias").expect("add alias to copy");
    copy.add_category("copy_category")
        .expect("add category to copy");
    copy.try_clear_aliases().expect("clear aliases from copy");
    copy.try_clear_categories()
        .expect("clear categories from copy");

    assert_eq!(copy.name().as_deref(), Some("UnitNamedTransformCopy"));
    assert_eq!(copy.num_aliases(), 0);
    assert_eq!(copy.num_categories(), 0);

    assert_eq!(nt.name().as_deref(), Some("UnitNamedTransform"));
    assert!(nt.has_alias("unit_alias"));
    assert!(nt.has_category("unit_category"));
}

#[test]
fn named_transform_config_registration_and_processor_behavior() {
    let _guard = named_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = Config::raw().expect("raw config");
    let initial_count = config.num_named_transforms();

    let nt = configured_named_transform();
    config.add_named_transform(&nt);

    assert_eq!(config.num_named_transforms(), initial_count + 1);
    assert_eq!(
        config.named_transform_index("UnitNamedTransform"),
        initial_count
    );

    let from_config = config
        .named_transform("UnitNamedTransform")
        .expect("named transform from config");
    assert_eq!(from_config.name().as_deref(), Some("UnitNamedTransform"));
    assert!(from_config.has_alias("unit_alias"));
    assert!(from_config.has_category("unit_category"));

    let processor_from_object = config
        .processor_named_transform(&nt, TransformDirection::Forward)
        .expect("processor from object");
    let cpu_from_object = processor_from_object
        .default_cpu_processor()
        .expect("cpu from object");
    let mut pixel = [0.25f32, 0.5, 0.5, 1.0];
    cpu_from_object.apply_rgba(&mut pixel);
    assert_close(pixel[0] as f64, 0.5, 1e-6);
    assert_close(pixel[1] as f64, 0.5, 1e-6);
    assert_close(pixel[2] as f64, 0.25, 1e-6);
    assert_close(pixel[3] as f64, 1.0, 1e-6);

    let processor_from_name = config
        .processor_named_transform_name("UnitNamedTransform", TransformDirection::Inverse)
        .expect("processor from name");
    let cpu_from_name = processor_from_name
        .default_cpu_processor()
        .expect("cpu from name");
    let mut inverse_pixel = [0.5f32, 0.5, 0.25, 1.0];
    cpu_from_name.apply_rgba(&mut inverse_pixel);
    assert_close(inverse_pixel[0] as f64, 0.25, 1e-6);
    assert_close(inverse_pixel[1] as f64, 0.5, 1e-6);
    assert_close(inverse_pixel[2] as f64, 0.5, 1e-6);
    assert_close(inverse_pixel[3] as f64, 1.0, 1e-6);
}

#[test]
fn named_transform_missing_name_errors_surface_behavior() {
    let _guard = named_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = Config::raw().expect("raw config");
    let err = match config.processor_named_transform_name(
        "DefinitelyMissingNamedTransform",
        TransformDirection::Forward,
    ) {
        Ok(_) => panic!("missing named transform should fail"),
        Err(err) => err,
    };
    assert!(
        matches!(err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );
}
