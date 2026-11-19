//! GroupTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/group.rs`. In bundled/real mode they validate transform
//! ordering, editable-copy independence, removal/clear behavior, and real
//! serializer output.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::{GroupTransform, Lut3DTransform, MatrixTransform, Transform};
use ocio_rs::{Config, TransformDirection};

fn group_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
#[cfg(ocio_stub)]
fn group_deep_copy_reports_stub_unavailability() {
    assert!(GroupTransform::create()
        .unwrap()
        .create_deep_copy()
        .is_err());
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
            let deep = group.create_deep_copy().unwrap();
            assert!(
                matches!(deep.try_transform(0).unwrap(), Some(Transform::$variant(_))),
                stringify!($variant)
            );
            let look = ocio_rs::Look::create().unwrap();
            look.try_set_transform(&value).unwrap();
            assert!(
                matches!(look.try_transform().unwrap(), Some(Transform::$variant(_))),
                stringify!($variant)
            );
            let shared = look.try_const_transform().unwrap().unwrap();
            let has_metadata = !matches!(
                stringify!($variant),
                "Allocation" | "Builtin" | "ColorSpace" | "DisplayView" | "File" | "Look"
            );
            assert_eq!(
                unsafe { shared.with_readonly_metadata(|metadata| metadata.is_some()) }.unwrap(),
                has_metadata,
                "const metadata dispatch for {}",
                stringify!($variant)
            );
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

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn format_registry_invalid_indices_are_absent() {
    use ocio_rs::{transform::FileTransform, Baker};
    type FormatQuery = fn(i32) -> Option<String>;
    let registries: [(i32, FormatQuery, FormatQuery); 3] = [
        (
            GroupTransform::num_write_formats(),
            GroupTransform::format_name_by_index,
            GroupTransform::format_extension_by_index,
        ),
        (
            FileTransform::num_formats(),
            FileTransform::format_name_by_index,
            FileTransform::format_extension_by_index,
        ),
        (
            Baker::num_formats(),
            Baker::format_name_by_index,
            Baker::format_extension_by_index,
        ),
    ];
    for (count, name, extension) in registries {
        assert!(count > 0);
        for index in 0..count {
            assert!(name(index).is_some());
            assert!(extension(index).is_some());
        }
        for index in [-1, count, i32::MAX] {
            assert_eq!(name(index), None);
            assert_eq!(extension(index), None);
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn const_group_child_retains_the_native_transform() {
    let group = GroupTransform::create().unwrap();
    group
        .append_transform(&scale_matrix([2.0, 1.0, 1.0, 1.0]))
        .unwrap();
    let look = ocio_rs::Look::create().unwrap();
    look.try_set_transform(&group).unwrap();
    let retained_group = look.try_const_transform().unwrap().unwrap();
    assert_eq!(retained_group.group_child_count().unwrap(), 1);
    assert!(retained_group.group_child(-1).is_err());
    assert!(retained_group.group_child(1).is_err());
    let child = retained_group.group_child(0).unwrap().unwrap();
    assert!(child.group_child_count().is_err());
    assert!(child.group_child(0).is_err());
    drop(retained_group);
    drop(look);
    drop(group);

    let cpu = Config::raw()
        .unwrap()
        .processor_from_const_transform(&child, TransformDirection::Forward, None)
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    let mut pixel = [0.25, 0.5, 0.75, 1.0];
    cpu.try_apply_rgba(&mut pixel).unwrap();
    assert_eq!(pixel, [0.5, 0.5, 0.75, 1.0]);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn const_group_child_supports_scoped_raw_reads_without_copying() {
    let lut = Lut3DTransform::create().unwrap();
    lut.set_value(0, 0, 0, [0.1, 0.2, 0.3]).unwrap();
    let metadata = lut.format_metadata().unwrap();
    metadata.set_name("first").unwrap();
    metadata.add_child_element("Description", "detail").unwrap();
    let group = GroupTransform::create().unwrap();
    group.append_transform(&lut).unwrap();
    let no_metadata = ocio_rs::transform::ColorSpaceTransform::create().unwrap();
    group.append_transform(&no_metadata).unwrap();
    let look = ocio_rs::Look::create().unwrap();
    look.try_set_transform(&group).unwrap();
    let parent = look.try_const_transform().unwrap().unwrap();
    let child = parent.group_child(0).unwrap().unwrap();
    let without_metadata = parent.group_child(1).unwrap().unwrap();
    assert!(unsafe { without_metadata.with_readonly_metadata(|value| value.is_none()) }.unwrap());
    assert_eq!(
        child.type_tag().unwrap(),
        ocio_rs::TransformType::Lut3D as i32
    );

    let read = |value: &ocio_rs::transform::ConstTransform| unsafe {
        value
            .with_readonly_raw_handle(|handle| {
                let mut rgb = [0.0f32; 3];
                let output = rgb.as_mut_ptr();
                ocio_sys::ocio_error_clear_last();
                ocio_sys::ocio_lut3d_transform_get_value(
                    handle,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    output.cast(),
                    output.add(1).cast(),
                    output.add(2).cast(),
                );
                assert!(ocio_sys::ocio_error_get_last().is_null());
                rgb
            })
            .unwrap()
    };
    let read_name = |value: &ocio_rs::transform::ConstTransform| unsafe {
        value
            .with_readonly_metadata(|metadata| {
                let metadata = metadata.unwrap();
                let child = metadata.child(0).unwrap().unwrap();
                assert_eq!(
                    child.element_value().unwrap().unwrap().to_bytes(),
                    b"detail"
                );
                metadata.name().unwrap().unwrap().to_bytes().to_vec()
            })
            .unwrap()
    };
    assert_eq!(read(&child), [0.1, 0.2, 0.3]);
    assert_eq!(read_name(&child), b"first");
    // No read is in progress while the separately owned mutable alias changes.
    lut.set_value(0, 0, 0, [0.4, 0.5, 0.6]).unwrap();
    metadata.set_name("second").unwrap();
    assert_eq!(read(&child), [0.4, 0.5, 0.6]);
    assert_eq!(read_name(&child), b"second");
    drop(metadata);
    drop(lut);
    drop(group);
    drop(look);
    drop(parent);
    assert_eq!(read(&child), [0.4, 0.5, 0.6]);
    assert_eq!(read_name(&child), b"second");
}

#[test]
#[cfg(not(feature = "v2_5"))]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn baseline_interface_reports_newer_transform_metadata_as_unsupported() {
    if ocio_rs::version_hex() < 0x0205_0000 {
        return;
    }
    let config = Config::from_stream(
        r#"ocio_profile_version: 2.5
roles: {default: raw}
colorspaces:
  - !<ColorSpace>
    name: raw
    from_scene_reference: !<GradingHueCurveTransform> {style: log}
"#,
    )
    .unwrap();
    let space = config.try_const_color_space("raw").unwrap().unwrap();
    let transform = space
        .try_const_transform(ocio_rs::ColorSpaceDirection::FromReference)
        .unwrap()
        .unwrap();
    assert_eq!(transform.type_tag().unwrap(), 10);
    assert!(unsafe { transform.with_readonly_metadata(|_| ()) }.is_err());
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
fn group_deep_copy_is_independent_and_preserves_internal_sharing() {
    let _guard = group_transform_test_lock();
    let leaf = scale_matrix([2.0, 1.0, 1.0, 1.0]);
    let inner = GroupTransform::create().unwrap();
    inner.append_transform(&leaf).unwrap();
    let root = GroupTransform::create().unwrap();
    root.set_direction(TransformDirection::Inverse);
    root.format_metadata()
        .unwrap()
        .add_attribute("origin", "source")
        .unwrap();
    root.append_transform(&leaf).unwrap();
    root.append_transform(&inner).unwrap();
    root.append_transform(&inner).unwrap();

    let shallow = root.create_editable_copy().unwrap();
    let Some(Transform::Matrix(shallow_leaf)) = shallow.try_transform(0).unwrap() else {
        panic!("matrix expected")
    };
    shallow_leaf.set_offset(&[0.1, 0.0, 0.0, 0.0]).unwrap();
    assert_eq!(leaf.offset()[0], 0.1); // Native copy shares children.

    let deep = root.create_deep_copy().unwrap();
    assert_eq!(deep.direction(), TransformDirection::Inverse);
    let deep_metadata = deep.format_metadata().unwrap();
    assert_eq!(
        deep_metadata.attribute_value("origin").as_deref(),
        Some("source")
    );
    deep_metadata.add_attribute("origin", "copy").unwrap();
    assert_eq!(
        root.format_metadata()
            .unwrap()
            .attribute_value("origin")
            .as_deref(),
        Some("source")
    );
    let Some(Transform::Matrix(deep_leaf)) = deep.try_transform(0).unwrap() else {
        panic!("matrix expected")
    };
    let Some(Transform::Group(deep_inner)) = deep.try_transform(1).unwrap() else {
        panic!("group expected")
    };
    let Some(Transform::Group(deep_inner_alias)) = deep.try_transform(2).unwrap() else {
        panic!("group expected")
    };
    let Some(Transform::Matrix(nested_leaf)) = deep_inner.try_transform(0).unwrap() else {
        panic!("nested matrix expected")
    };
    deep_leaf.set_offset(&[0.7, 0.0, 0.0, 0.0]).unwrap();
    assert_eq!(nested_leaf.offset()[0], 0.7);
    assert_eq!(leaf.offset()[0], 0.1);
    deep_inner.set_direction(TransformDirection::Inverse);
    assert_eq!(deep_inner_alias.direction(), TransformDirection::Inverse);
    assert_eq!(inner.direction(), TransformDirection::Forward);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn group_children_remain_shared_through_look_transform_copies() {
    let _guard = group_transform_test_lock();
    let leaf = scale_matrix([2.0, 1.0, 1.0, 1.0]);
    let group = GroupTransform::create().unwrap();
    group.append_transform(&leaf).unwrap();
    let look = ocio_rs::Look::create().unwrap();
    look.try_set_transform(&group).unwrap();
    let Some(Transform::Group(retrieved)) = look.try_transform().unwrap() else {
        panic!("group expected")
    };
    leaf.set_offset(&[0.25, 0.0, 0.0, 0.0]).unwrap();
    let Some(Transform::Matrix(retrieved_leaf)) = retrieved.try_transform(0).unwrap() else {
        panic!("matrix expected")
    };
    assert_eq!(retrieved_leaf.offset()[0], 0.25);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn group_children_remain_shared_through_config_copies() {
    let _guard = group_transform_test_lock();
    let leaf = scale_matrix([2.0, 1.0, 1.0, 1.0]);
    let group = GroupTransform::create().unwrap();
    group.append_transform(&leaf).unwrap();
    let space = ocio_rs::ColorSpace::create().unwrap();
    space.set_name("group-copy-space").unwrap();
    space
        .try_set_transform(&group, ocio_rs::ColorSpaceDirection::ToReference)
        .unwrap();
    let config = ocio_rs::Config::raw()
        .unwrap()
        .create_editable_copy()
        .unwrap();
    config.try_add_color_space(&space).unwrap();
    let copied_config = config.create_editable_copy().unwrap();
    leaf.set_offset(&[0.375, 0.0, 0.0, 0.0]).unwrap();
    let copied_space = copied_config
        .try_color_space("group-copy-space")
        .unwrap()
        .unwrap();
    let Some(Transform::Group(copied_group)) = copied_space
        .try_transform(ocio_rs::ColorSpaceDirection::ToReference)
        .unwrap()
    else {
        panic!("group expected")
    };
    let Some(Transform::Matrix(copied_leaf)) = copied_group.try_transform(0).unwrap() else {
        panic!("matrix expected")
    };
    assert_eq!(copied_leaf.offset()[0], 0.375);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn shared_group_change_rebuilds_processor_after_cache_clear() {
    let _guard = group_transform_test_lock();
    let leaf = scale_matrix([2.0, 1.0, 1.0, 1.0]);
    let group = GroupTransform::create().unwrap();
    group.append_transform(&leaf).unwrap();
    let source = ocio_rs::ColorSpace::create().unwrap();
    source.set_name("source").unwrap();
    source
        .try_set_transform(&group, ocio_rs::ColorSpaceDirection::ToReference)
        .unwrap();
    let destination = ocio_rs::ColorSpace::create().unwrap();
    destination.set_name("destination").unwrap();
    let config = Config::raw().unwrap().create_editable_copy().unwrap();
    config.try_add_color_space(&source).unwrap();
    config.try_add_color_space(&destination).unwrap();
    let copied = config.create_editable_copy().unwrap();
    let old_cpu = copied
        .processor("source", "destination")
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    leaf.set_offset(&[0.375, 0.0, 0.0, 0.0]).unwrap();
    copied.try_clear_processor_cache().unwrap();
    let new_cpu = copied
        .processor("source", "destination")
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    let mut old_pixel = [0.1, 0.0, 0.0, 1.0];
    let mut new_pixel = old_pixel;
    old_cpu.try_apply_rgba(&mut old_pixel).unwrap();
    new_cpu.try_apply_rgba(&mut new_pixel).unwrap();
    assert!((old_pixel[0] - 0.2).abs() < 1e-6);
    assert!((new_pixel[0] - 0.575).abs() < 1e-6);
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
