use ocio_rs::transform::FileTransform;
use ocio_rs::{Config, TransformDirection};

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn processor_format_metadata_is_shared_readonly_or_editable_copy() {
    let config = Config::raw().unwrap();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/clf/matrix_3x4_example.clf");
    let transform = FileTransform::create().unwrap();
    transform.set_src(path.to_string_lossy()).unwrap();
    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap();
    let shared = processor.try_const_format_metadata().unwrap().unwrap();
    let original_name = shared.name().unwrap().unwrap().to_owned();
    let original_id = shared.id().unwrap().unwrap().to_owned();
    assert_eq!(shared.element_name().unwrap().unwrap(), c"ROOT");
    assert!(shared.num_attributes().unwrap() > 0);
    assert_eq!(
        shared.attribute_value(c"id").unwrap().unwrap(),
        original_id.as_c_str()
    );
    assert!(shared.attribute_name(0).unwrap().is_some());
    assert!(shared.attribute_value_by_index(0).unwrap().is_some());
    let original_children = shared.num_children().unwrap();
    assert!(original_children > 0);
    let child = shared.child(0).unwrap().unwrap();
    let original_child = child.element_value().unwrap().unwrap().to_owned();
    assert!(shared.child(-1).is_err());
    let copy = processor.try_format_metadata().unwrap();
    assert_eq!(copy.num_children(), original_children);
    copy.set_name("edited-only").unwrap();
    copy.child_element(0)
        .unwrap()
        .set_element_value("edited-child")
        .unwrap();
    assert_eq!(shared.name().unwrap().unwrap(), original_name.as_c_str());
    assert_eq!(
        child.element_value().unwrap().unwrap(),
        original_child.as_c_str()
    );
    let copy2 = shared.create_editable_copy().unwrap();
    assert_eq!(copy2.num_children(), original_children);
    assert_eq!(copy2.name().as_deref(), original_name.to_str().ok());
    assert_eq!(copy2.id().as_deref(), original_id.to_str().ok());
    let per_transform = processor
        .try_const_transform_format_metadata(0)
        .unwrap()
        .unwrap();
    let op_name = per_transform.element_name().unwrap().unwrap().to_owned();
    let op_copy = processor.try_transform_format_metadata(0).unwrap();
    assert_eq!(op_copy.element_name().as_deref(), op_name.to_str().ok());
    op_copy.set_name("changed-operation").unwrap();
    assert_ne!(per_transform.name().unwrap().unwrap(), c"changed-operation");
    assert!(processor
        .try_const_transform_format_metadata(i32::MAX)
        .is_err());
    assert!(processor.try_const_transform_format_metadata(-1).is_err());
    assert!(processor.try_transform_format_metadata(i32::MAX).is_err());
    assert!(processor.try_transform_format_metadata(-1).is_err());
    drop(processor);
    drop(shared);
    assert_eq!(
        child.element_value().unwrap().unwrap(),
        original_child.as_c_str()
    );
}
