//! FormatMetadata behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/format_metadata.rs`. In bundled/real mode they validate attribute and
//! child-element editing on real metadata roots sourced from OCIO objects.

mod common;
use common::*;

use std::ffi::{CStr, CString};
use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::{FixedFunctionTransform, MatrixTransform};
use ocio_rs::{Baker, Config, FixedFunctionStyle, TransformDirection};

fn format_metadata_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn baker_format_metadata_round_trip_and_copy_behavior() {
    let _guard = format_metadata_test_lock();
    if is_stub() {
        return;
    }

    let baker = Baker::create().expect("baker create");
    let config = Config::raw().expect("raw config");
    baker.set_config(&config).expect("attach config");
    baker.set_format("resolve_cube").expect("set baker format");

    let metadata = baker.format_metadata().expect("baker format metadata");
    let baseline_attributes = metadata.num_attributes();
    let baseline_children = metadata.num_children();
    assert!(matches!(
        metadata.set_element_name("Baker"),
        Err(ocio_rs::OcioError::Ocio(_))
    ));
    assert!(matches!(
        metadata.set_element_value("unit-test-root"),
        Err(ocio_rs::OcioError::Ocio(_))
    ));
    metadata
        .add_attribute("origin", "ocio-rs")
        .expect("add origin attribute");
    metadata
        .add_attribute("stage", "format-metadata")
        .expect("add stage attribute");
    metadata
        .add_child_element("InputDescriptor", "raw")
        .expect("add first child");
    metadata
        .add_child_element("Description", "metadata round trip")
        .expect("add second child");
    metadata.set_name("FriendlyName").expect("set name");
    metadata.set_id("urn:test:baker").expect("set id");

    assert_eq!(metadata.element_name().as_deref(), Some("ROOT"));
    assert_eq!(metadata.element_value().as_deref(), Some(""));
    assert_eq!(metadata.name().as_deref(), Some("FriendlyName"));
    assert_eq!(metadata.id().as_deref(), Some("urn:test:baker"));
    assert_eq!(
        metadata.attribute_value("origin").as_deref(),
        Some("ocio-rs")
    );
    assert_eq!(
        metadata
            .try_attribute_value("origin")
            .expect("named attribute value query")
            .as_deref(),
        Some("ocio-rs")
    );
    assert!(metadata.try_attribute_value("origin\0").is_err());
    assert_eq!(
        metadata.attribute_value("stage").as_deref(),
        Some("format-metadata")
    );
    assert!(metadata.num_attributes() >= baseline_attributes + 2);
    assert_eq!(metadata.num_children(), baseline_children + 2);

    let attr_names: Vec<_> = (0..metadata.num_attributes())
        .map(|index| metadata.attribute_name(index).expect("attribute name"))
        .collect();
    let attr_values: Vec<_> = (0..metadata.num_attributes())
        .map(|index| {
            metadata
                .attribute_value_by_index(index)
                .expect("attribute value")
        })
        .collect();
    assert!(attr_names.iter().any(|name| name == "origin"));
    assert!(attr_names.iter().any(|name| name == "stage"));
    assert!(attr_values.iter().any(|value| value == "ocio-rs"));
    assert!(attr_values.iter().any(|value| value == "format-metadata"));

    let first_child = metadata
        .try_child_element(baseline_children)
        .expect("first child query")
        .expect("first added child");
    assert_eq!(
        first_child.element_name().as_deref(),
        Some("InputDescriptor")
    );
    assert_eq!(first_child.element_value().as_deref(), Some("raw"));
    let second_child = metadata
        .child_element(baseline_children + 1)
        .expect("second added child");
    assert_eq!(second_child.element_name().as_deref(), Some("Description"));
    assert_eq!(
        second_child.element_value().as_deref(),
        Some("metadata round trip")
    );

    let copy = baker.create_editable_copy().expect("baker editable copy");
    let copy_metadata = copy.format_metadata().expect("copy format metadata");
    assert_eq!(copy_metadata.element_name().as_deref(), Some("ROOT"));
    assert_eq!(copy_metadata.element_value().as_deref(), Some(""));
    assert_eq!(
        copy_metadata.attribute_value("origin").as_deref(),
        Some("ocio-rs")
    );
    assert_eq!(copy_metadata.num_children(), baseline_children + 2);

    copy_metadata.clear();
    assert_eq!(copy_metadata.num_attributes(), 0);
    assert_eq!(copy_metadata.num_children(), 0);
    assert_eq!(copy_metadata.element_name().as_deref(), Some("ROOT"));
    assert_eq!(copy_metadata.element_value().as_deref(), Some(""));

    assert!(metadata.num_attributes() >= baseline_attributes + 2);
    assert_eq!(metadata.num_children(), baseline_children + 2);
    assert_eq!(metadata.element_name().as_deref(), Some("ROOT"));
}

#[test]
fn processor_transform_format_metadata_access_behavior() {
    let _guard = format_metadata_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = MatrixTransform::scale(&[1.1, 0.9, 1.2, 1.0]).expect("matrix scale");
    let transform_metadata = transform.format_metadata().expect("matrix format metadata");
    assert!(matches!(
        transform_metadata.set_element_name("Matrix"),
        Err(ocio_rs::OcioError::Ocio(_))
    ));
    transform_metadata
        .add_attribute("test_attr", "matrix")
        .expect("set matrix attribute");

    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .expect("processor from matrix");
    let processor_metadata = processor
        .processor_metadata()
        .expect("processor metadata handle");
    let transform_metadata = processor
        .transform_format_metadata(0)
        .expect("transform metadata handle");

    assert!(processor_metadata.num_files() >= 0);
    assert!(processor_metadata.num_looks() >= 0);
    assert_eq!(transform_metadata.element_name().as_deref(), Some("ROOT"));
    assert_eq!(
        transform_metadata.attribute_value("test_attr").as_deref(),
        Some("matrix")
    );
}

#[test]
fn format_metadata_remains_usable_after_parent_drop() {
    let _guard = format_metadata_test_lock();
    if is_stub() {
        return;
    }

    let transform_metadata = {
        let transform = MatrixTransform::scale(&[1.0, 1.0, 1.0, 1.0]).expect("matrix scale");
        let metadata = transform.format_metadata().expect("matrix format metadata");
        metadata
            .add_attribute("owner", "transform")
            .expect("set transform metadata attribute");
        metadata
    };
    assert_eq!(
        transform_metadata.attribute_value("owner").as_deref(),
        Some("transform")
    );

    let baker_metadata = {
        let baker = Baker::create().expect("baker create");
        let config = Config::raw().expect("raw config");
        baker.set_config(&config).expect("attach config");
        baker.set_format("resolve_cube").expect("set baker format");
        let metadata = baker.format_metadata().expect("baker format metadata");
        metadata
            .add_attribute("owner", "baker")
            .expect("set baker metadata attribute");
        metadata
    };
    assert_eq!(
        baker_metadata.attribute_value("owner").as_deref(),
        Some("baker")
    );

    let fixed_function_metadata = {
        let transform =
            FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).expect("fixed func");
        let metadata = transform
            .format_metadata()
            .expect("fixed-function format metadata");
        metadata
            .add_attribute("owner", "fixed-function")
            .expect("set fixed-function metadata attribute");
        metadata
    };
    assert_eq!(
        fixed_function_metadata.attribute_value("owner").as_deref(),
        Some("fixed-function")
    );
}

#[test]
fn legacy_sys_metadata_handles_remain_usable_after_parent_drop() {
    let _guard = format_metadata_test_lock();
    if is_stub() {
        return;
    }

    let attr_name = CString::new("owner").expect("attr name");
    let attr_value = CString::new("legacy-fixed-function").expect("attr value");

    unsafe {
        let transform = ocio_sys::ocio_fixed_function_transform_create_with_params(
            FixedFunctionStyle::AcesRedMod03 as i32,
            std::ptr::null(),
            0,
        );
        assert!(!transform.is_null(), "fixed-function transform handle");

        let metadata = ocio_sys::ocio_fixed_function_transform_get_format_metadata_v1(transform);
        assert!(!metadata.is_null(), "legacy metadata handle");

        ocio_sys::ocio_format_metadata_add_attribute(
            metadata,
            attr_name.as_ptr(),
            attr_value.as_ptr(),
        );
        ocio_sys::ocio_fixed_function_transform_destroy(transform);

        let value_ptr =
            ocio_sys::ocio_format_metadata_get_attribute_value(metadata, attr_name.as_ptr());
        assert!(!value_ptr.is_null(), "legacy metadata attribute value");
        assert_eq!(
            CStr::from_ptr(value_ptr)
                .to_str()
                .expect("utf8 attribute value"),
            "legacy-fixed-function"
        );

        ocio_sys::ocio_format_metadata_destroy(metadata);
    }
}
