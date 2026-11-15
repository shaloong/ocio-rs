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

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn group_returns_concrete_handles_for_supported_transform_types() {
    use ocio_rs::transform::*;
    use ocio_rs::{FixedFunctionStyle, GradingStyle};
    let _guard = group_transform_test_lock();
    macro_rules! check {
        ($transform:expr, $variant:ident) => {{
            let group = GroupTransform::create().unwrap();
            let value = $transform;
            group.append_transform(&value).unwrap();
            assert!(
                matches!(
                    group.try_transform(0).unwrap(),
                    Some(Transform::$variant(_))
                ),
                stringify!($variant)
            );
            let look = ocio_rs::Look::create().unwrap();
            look.try_set_transform(&value).unwrap();
            assert!(
                matches!(look.try_transform().unwrap(), Some(Transform::$variant(_))),
                stringify!($variant)
            );
            let shared = look.try_const_transform().unwrap().unwrap();
            assert!(
                matches!(
                    shared.create_editable_copy().unwrap(),
                    Transform::$variant(_)
                ),
                stringify!($variant)
            );
        }};
    }
    check!(AllocationTransform::create().unwrap(), Allocation);
    check!(BuiltinTransform::create().unwrap(), Builtin);
    check!(CDLTransform::create().unwrap(), CDL);
    check!(ColorSpaceTransform::create().unwrap(), ColorSpace);
    check!(DisplayViewTransform::create().unwrap(), DisplayView);
    check!(ExponentTransform::create().unwrap(), Exponent);
    check!(
        ExponentWithLinearTransform::create().unwrap(),
        ExponentWithLinear
    );
    check!(
        ExposureContrastTransform::create().unwrap(),
        ExposureContrast
    );
    check!(FileTransform::create().unwrap(), File);
    check!(
        FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap(),
        FixedFunction
    );
    #[cfg(feature = "v2_5")]
    check!(
        GradingHueCurveTransform::create(GradingStyle::Log).unwrap(),
        GradingHueCurve
    );
    check!(
        GradingPrimaryTransform::create(GradingStyle::Log).unwrap(),
        GradingPrimary
    );
    check!(
        GradingRGBCurveTransform::create(GradingStyle::Log).unwrap(),
        GradingRGBCurve
    );
    check!(
        GradingToneTransform::create(GradingStyle::Log).unwrap(),
        GradingTone
    );
    check!(GroupTransform::create().unwrap(), Group);
    check!(LogAffineTransform::create().unwrap(), LogAffine);
    check!(
        LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap(),
        LogCamera
    );
    check!(LogTransform::create().unwrap(), Log);
    check!(LookTransform::create().unwrap(), Look);
    check!(Lut1DTransform::create().unwrap(), Lut1D);
    check!(Lut3DTransform::create().unwrap(), Lut3D);
    check!(MatrixTransform::create().unwrap(), Matrix);
    check!(RangeTransform::create().unwrap(), Range);
}

fn scale_matrix(scale: [f64; 4]) -> MatrixTransform {
    MatrixTransform::scale(&scale).expect("scale matrix")
}

fn offset_matrix(offset: [f64; 4]) -> MatrixTransform {
    let matrix = MatrixTransform::identity().expect("identity matrix");
    matrix.set_offset(&offset).expect("set matrix offset");
    matrix
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn group_rejects_direct_and_indirect_cycles_without_mutation() {
    let _guard = group_transform_test_lock();
    let root = GroupTransform::create().unwrap();
    assert!(root.append_transform(&root).is_err());
    assert!(root.prepend_transform(&root).is_err());
    assert_eq!(root.num_transforms(), 0);
    let child = GroupTransform::create().unwrap();
    let leaf = scale_matrix([1.2, 1.0, 1.0, 1.0]);
    child.append_transform(&leaf).unwrap();
    root.append_transform(&child).unwrap();
    assert!(child.append_transform(&root).is_err());
    assert!(child.prepend_transform(&root).is_err());
    assert!(child.set_transform(0, &root).is_err());
    assert_eq!(root.num_transforms(), 1);
    assert_eq!(child.num_transforms(), 1);
    let inner = GroupTransform::create().unwrap();
    child.append_transform(&inner).unwrap();
    assert!(inner.append_transform(&root).is_err());
    assert_eq!(inner.num_transforms(), 0);
    // Reusing a child in an acyclic graph remains valid and retains sharing.
    root.append_transform(&child).unwrap();
    assert_eq!(root.num_transforms(), 2);
    let Some(Transform::Group(shared)) = root.try_transform(0).unwrap() else {
        panic!("expected shared group");
    };
    assert_eq!(shared.num_transforms(), 2);
    shared.set_direction(TransformDirection::Inverse);
    assert_eq!(child.direction(), TransformDirection::Inverse);
    shared.clear_transforms().unwrap();
    assert_eq!(shared.num_transforms(), 0);
    assert_eq!(child.num_transforms(), 2);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn group_removal_rebuilds_wrapper_without_mutating_existing_shared_group() {
    let _guard = group_transform_test_lock();
    let parent = GroupTransform::create().unwrap();
    let child = GroupTransform::create().unwrap();
    let leaf = scale_matrix([1.1, 1.0, 1.0, 1.0]);
    child.append_transform(&leaf).unwrap();
    child.append_transform(&leaf).unwrap();
    parent.append_transform(&child).unwrap();
    child.remove_transform(0).unwrap();
    assert_eq!(child.num_transforms(), 1);
    let Some(Transform::Group(old_shared)) = parent.try_transform(0).unwrap() else {
        panic!("expected previously shared group");
    };
    assert_eq!(old_shared.num_transforms(), 2);
    child.clear_transforms().unwrap();
    assert_eq!(child.num_transforms(), 0);
    assert_eq!(old_shared.num_transforms(), 2);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
    assert!(matches!(
        group.try_transform(0).expect("first child query"),
        Some(Transform::Matrix(_))
    ));
    assert!(matches!(
        group.try_transform(1).expect("second child query"),
        Some(Transform::Matrix(_))
    ));
    let missing_child_err = match group.try_transform(2) {
        Err(err) => err,
        Ok(_) => panic!("out-of-range child query must fail"),
    };
    assert!(
        matches!(missing_child_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {missing_child_err:?}"
    );

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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn group_child_handle_survives_parent_drop_behavior() {
    let _guard = group_transform_test_lock();
    if is_stub() {
        return;
    }

    let child = {
        let group = GroupTransform::create().expect("group create");
        let matrix = scale_matrix([1.5, 1.0, 1.0, 1.0]);
        group.append_transform(&matrix).expect("append matrix");
        group
            .try_transform(0)
            .expect("child query")
            .expect("child transform")
    };

    assert!(matches!(child, Transform::Matrix(_)));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn group_replaces_child_in_place_and_keeps_old_handles_alive() {
    let _guard = group_transform_test_lock();
    let group = GroupTransform::create().expect("group");
    let old = scale_matrix([2.0, 1.0, 1.0, 1.0]);
    let replacement = scale_matrix([3.0, 1.0, 1.0, 1.0]);
    group.append_transform(&old).expect("append");
    let old_handle = group.try_transform(0).unwrap().unwrap();
    group.set_transform(0, &replacement).expect("replace");
    assert_eq!(group.num_transforms(), 1);
    let Some(Transform::Matrix(new_handle)) = group.try_transform(0).unwrap() else {
        panic!("expected matrix child");
    };
    assert_eq!(new_handle.matrix()[0], 3.0);
    let Transform::Matrix(old_handle) = old_handle else {
        panic!("expected old matrix");
    };
    assert_eq!(old_handle.matrix()[0], 2.0);
    let mut updated = replacement.matrix();
    updated[0] = 4.0;
    replacement.set_matrix(&updated).unwrap();
    assert_eq!(
        new_handle.matrix()[0],
        4.0,
        "replacement shares native ownership"
    );
    assert!(group.set_transform(-1, &old).is_err());
    assert!(group.set_transform(1, &old).is_err());
    assert_eq!(group.num_transforms(), 1);
    assert_eq!(new_handle.matrix()[0], 4.0);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
