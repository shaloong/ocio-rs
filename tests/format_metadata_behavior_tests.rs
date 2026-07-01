//! FormatMetadata behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/format_metadata.rs`. In bundled/real mode they validate attribute and
//! child-element editing on real metadata roots sourced from OCIO objects.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::MatrixTransform;
use ocio_rs::{Baker, Config, TransformDirection};

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
    baker.set_config(&config);
    baker.set_format("resolve_cube").expect("set baker format");

    let metadata = baker.format_metadata().expect("baker format metadata");
    let baseline_attributes = metadata.num_attributes();
    let baseline_children = metadata.num_children();
    metadata
        .set_element_name("Baker")
        .expect("attempt to set root element name");
    metadata
        .set_element_value("unit-test-root")
        .expect("attempt to set root element value");
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
    assert_eq!(metadata.attribute_value("origin").as_deref(), Some("ocio-rs"));
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
        .child_element(baseline_children)
        .expect("first added child");
    assert_eq!(first_child.element_name().as_deref(), Some("InputDescriptor"));
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
    assert_eq!(copy_metadata.attribute_value("origin").as_deref(), Some("ocio-rs"));
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
    transform_metadata
        .set_element_name("Matrix")
        .expect("attempt to set root element name");
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
