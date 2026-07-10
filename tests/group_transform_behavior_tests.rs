//! GroupTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/group.rs`. In bundled/real mode they validate transform
//! ordering, editable-copy independence, removal/clear behavior, and real
//! serializer output.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::{GroupTransform, MatrixTransform, Transform};
use ocio_rs::{Config, TransformDirection};

fn group_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn scale_matrix(scale: [f64; 4]) -> MatrixTransform {
    MatrixTransform::scale(&scale).expect("scale matrix")
}

fn offset_matrix(offset: [f64; 4]) -> MatrixTransform {
    let matrix = MatrixTransform::identity().expect("identity matrix");
    matrix.set_offset(&offset);
    matrix
}

#[test]
fn group_transform_order_copy_and_mutation_behavior() {
    let _guard = group_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let scale = scale_matrix([2.0, 1.0, 1.0, 1.0]);
    let offset = offset_matrix([0.1, 0.0, 0.0, 0.0]);

    let group = GroupTransform::create().expect("group create");
    group.append_transform(&scale).expect("append scale");
    group.append_transform(&offset).expect("append offset");

    assert_eq!(group.num_transforms(), 2);
    assert_eq!(group.direction(), TransformDirection::Forward);
    assert!(matches!(group.transform(0), Some(Transform::Matrix(_))));
    assert!(matches!(group.transform(1), Some(Transform::Matrix(_))));

    let cpu = config
        .processor_from_transform(&group, TransformDirection::Forward)
        .expect("group processor")
        .default_cpu_processor()
        .expect("group cpu");
    let mut appended_pixel = [0.2f32, 0.5, 0.75, 1.0];
    cpu.apply_rgba(&mut appended_pixel);

    assert_close(appended_pixel[0] as f64, 0.5, 1e-6);
    assert_close(appended_pixel[1] as f64, 0.5, 1e-6);
    assert_close(appended_pixel[2] as f64, 0.75, 1e-6);
    assert_close(appended_pixel[3] as f64, 1.0, 1e-6);

    let prepended = GroupTransform::create().expect("prepended group create");
    prepended.append_transform(&scale).expect("append scale");
    prepended
        .prepend_transform(&offset)
        .expect("prepend offset");

    assert_eq!(prepended.num_transforms(), 2);
    let prepended_cpu = config
        .processor_from_transform(&prepended, TransformDirection::Forward)
        .expect("prepended processor")
        .default_cpu_processor()
        .expect("prepended cpu");
    let mut prepended_pixel = [0.2f32, 0.5, 0.75, 1.0];
    prepended_cpu.apply_rgba(&mut prepended_pixel);

    assert_close(prepended_pixel[0] as f64, 0.6, 1e-6);
    assert_close(prepended_pixel[1] as f64, 0.5, 1e-6);
    assert_close(prepended_pixel[2] as f64, 0.75, 1e-6);
    assert_close(prepended_pixel[3] as f64, 1.0, 1e-6);

    let copy = group
        .create_editable_copy()
        .expect("editable copy from group");
    copy.set_direction(TransformDirection::Inverse);
    copy.remove_transform(1).expect("remove child");

    assert_eq!(copy.direction(), TransformDirection::Inverse);
    assert_eq!(copy.num_transforms(), 1);
    assert_eq!(group.direction(), TransformDirection::Forward);
    assert_eq!(group.num_transforms(), 2);

    copy.clear_transforms().expect("clear children");
    assert_eq!(copy.num_transforms(), 0);
    assert_eq!(group.num_transforms(), 2);

    let err = group
        .remove_transform(2)
        .expect_err("out-of-range child removal must fail");
    assert!(err.to_string().contains("out of range"));
    assert_eq!(group.num_transforms(), 2);
}

#[test]
fn group_transform_writer_and_format_query_behavior() {
    let _guard = group_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = Config::raw().expect("raw config");
    let scale = scale_matrix([1.1, 0.9, 1.2, 1.0]);
    let range = ocio_rs::transform::RangeTransform::create().expect("range create");
    range.set_min_in_value(0.0);
    range.set_max_in_value(1.0);
    range.set_min_out_value(0.0);
    range.set_max_out_value(1.0);

    let group = GroupTransform::create().expect("group create");
    group.append_transform(&scale).expect("append scale");
    group.append_transform(&range).expect("append range");

    let formats = GroupTransform::num_write_formats();
    assert!(formats > 0);
    let mut found_clf = false;
    for index in 0..formats {
        let name = GroupTransform::format_name_by_index(index).expect("format name");
        let extension = GroupTransform::format_extension_by_index(index).expect("format extension");
        assert!(!name.trim().is_empty());
        assert!(!extension.trim().is_empty());
        if name == "Academy/ASC Common LUT Format" {
            found_clf = true;
        }
    }
    assert!(found_clf, "expected CLF writer to be available");

    let written = group
        .write_to_string(&config, "Academy/ASC Common LUT Format")
        .expect("group write_to_string")
        .expect("real CLF text");

    assert!(!written.trim().is_empty());
    assert!(written.contains("ProcessList"));
    assert!(written.contains("Matrix"));
    assert!(written.contains("Range"));
}
