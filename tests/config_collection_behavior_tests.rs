//! Config collection-mutation behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/config.rs`. In bundled/real mode they validate add/remove/clear
//! behavior for config-managed color spaces, looks, named transforms, view
//! transforms, and display-view references.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::MatrixTransform;
use ocio_rs::{
    Allocation, ColorSpace, ColorSpaceDirection, Config, Look, NamedTransform, ReferenceSpaceType,
    TransformDirection, ViewTransform, ViewTransformDirection,
};

fn config_collection_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn identity_color_space(name: &str) -> ColorSpace {
    let cs = ColorSpace::create().expect("color space create");
    cs.set_name(name).expect("set color space name");
    cs.set_family("Unit/ConfigCollections").expect("set family");
    cs.set_description("config collection behavior test")
        .expect("set description");
    cs.set_is_data(false);
    cs.set_allocation(Allocation::Lg2);
    cs.set_allocation_vars(&[-8.0, 8.0])
        .expect("set allocation variables");

    let identity = MatrixTransform::identity().expect("identity matrix");
    cs.set_transform(&identity, ColorSpaceDirection::ToReference);
    cs.set_transform(&identity, ColorSpaceDirection::FromReference);
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
    let forward = MatrixTransform::scale(&[1.1, 1.0, 1.0, 1.0]).expect("look matrix");
    look.set_transform(&forward);
    look
}

fn scaling_named_transform(name: &str) -> NamedTransform {
    let nt = NamedTransform::create().expect("named transform create");
    nt.set_name(name).expect("set named transform name");
    let forward = MatrixTransform::scale(&[0.9, 1.0, 1.0, 1.0]).expect("named transform matrix");
    nt.set_transform(&forward, TransformDirection::Forward);
    nt
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_collection_registration_and_usage_behavior() {
    let _guard = config_collection_test_lock();
    if is_stub() {
        return;
    }

    let config = Config::raw().expect("raw config");
    let initial_color_spaces = config.num_color_spaces();
    let initial_looks = config.num_looks();
    let initial_named_transforms = config.num_named_transforms();
    let initial_view_transforms = config.num_view_transforms();
    let initial_displays = config.num_displays_all();

    let used_color_space = identity_color_space("UnitConfigUsedColorSpace");
    let unused_color_space = identity_color_space("UnitConfigUnusedColorSpace");
    let view_transform = identity_view_transform("UnitConfigViewTransform");
    let look = scaling_look("UnitConfigLook", "UnitConfigUsedColorSpace");
    let named_transform = scaling_named_transform("UnitConfigNamedTransform");

    config.add_color_space(&used_color_space);
    config.add_color_space(&unused_color_space);
    config.add_view_transform(&view_transform);
    config.add_look(&look);
    config.add_named_transform(&named_transform);
    config
        .add_display_view_detailed(
            "UnitConfigDisplay",
            "UnitConfigView",
            "UnitConfigViewTransform",
            "UnitConfigUsedColorSpace",
            "UnitConfigLook",
            "",
            "unit config collection pipeline",
        )
        .expect("add display view");

    assert_eq!(config.num_color_spaces(), initial_color_spaces + 2);
    assert_eq!(config.num_looks(), initial_looks + 1);
    assert_eq!(config.num_named_transforms(), initial_named_transforms + 1);
    assert_eq!(config.num_view_transforms(), initial_view_transforms + 1);
    assert_eq!(config.num_displays_all(), initial_displays + 1);

    assert!(config
        .try_color_space("UnitConfigUsedColorSpace")
        .expect("color-space lookup")
        .is_some());
    assert!(config
        .try_color_space("UnitConfigUnusedColorSpace")
        .expect("color-space lookup")
        .is_some());
    assert!(used_color_space
        .try_transform(ColorSpaceDirection::ToReference)
        .expect("color-space transform query")
        .is_some());
    assert_eq!(
        config.look_name_by_index(initial_looks).as_deref(),
        Some("UnitConfigLook")
    );
    assert!(config
        .try_look("UnitConfigLook")
        .expect("look lookup")
        .is_some());
    assert!(config
        .try_named_transform("UnitConfigNamedTransform")
        .expect("named-transform lookup")
        .is_some());
    assert!(config
        .try_view_transform("UnitConfigViewTransform")
        .expect("view-transform lookup")
        .is_some());
    assert_eq!(
        config.named_transform_index("UnitConfigNamedTransform"),
        initial_named_transforms
    );
    assert_eq!(
        config
            .view_transform_name_by_index(initial_view_transforms)
            .as_deref(),
        Some("UnitConfigViewTransform")
    );
    assert_eq!(
        config
            .display_view_transform_name("UnitConfigDisplay", "UnitConfigView")
            .as_deref(),
        Some("UnitConfigViewTransform")
    );
    assert_eq!(
        config
            .display_view_looks("UnitConfigDisplay", "UnitConfigView")
            .as_deref(),
        Some("UnitConfigLook")
    );

    assert!(config.is_color_space_used("UnitConfigUsedColorSpace"));
    assert!(!config.is_color_space_used("UnitConfigUnusedColorSpace"));

    assert!(config
        .try_color_space("UnitConfigUsedColorSpace\0")
        .is_err());
    assert!(config.try_look("UnitConfigLook\0").is_err());
    assert!(config
        .try_named_transform("UnitConfigNamedTransform\0")
        .is_err());
    assert!(config
        .try_view_transform("UnitConfigViewTransform\0")
        .is_err());
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_collection_remove_and_clear_behavior() {
    let _guard = config_collection_test_lock();
    if is_stub() {
        return;
    }

    let config = Config::raw().expect("raw config");

    let used_color_space = identity_color_space("UnitConfigUsedColorSpace");
    let unused_color_space = identity_color_space("UnitConfigUnusedColorSpace");
    let view_transform = identity_view_transform("UnitConfigViewTransform");
    let look = scaling_look("UnitConfigLook", "UnitConfigUsedColorSpace");
    let named_transform = scaling_named_transform("UnitConfigNamedTransform");

    config.add_color_space(&used_color_space);
    config.add_color_space(&unused_color_space);
    config.add_view_transform(&view_transform);
    config.add_look(&look);
    config.add_named_transform(&named_transform);
    config
        .add_display_view_detailed(
            "UnitConfigDisplay",
            "UnitConfigView",
            "UnitConfigViewTransform",
            "UnitConfigUsedColorSpace",
            "UnitConfigLook",
            "",
            "unit config collection pipeline",
        )
        .expect("add display view");

    config
        .remove_named_transform("UnitConfigNamedTransform")
        .expect("remove named transform");
    assert!(config.named_transform("UnitConfigNamedTransform").is_none());

    config.add_named_transform(&named_transform);
    config
        .try_clear_named_transforms()
        .expect("clear named transforms");
    assert_eq!(config.num_named_transforms(), 0);
    assert!(config.named_transform("UnitConfigNamedTransform").is_none());

    config
        .remove_color_space("UnitConfigUnusedColorSpace")
        .expect("remove unused color space");
    assert!(config.color_space("UnitConfigUnusedColorSpace").is_none());
    assert!(config.color_space("UnitConfigUsedColorSpace").is_some());

    config.try_clear_looks().expect("clear looks");
    assert_eq!(config.num_looks(), 0);
    assert!(config.look("UnitConfigLook").is_none());

    config
        .try_clear_view_transforms()
        .expect("clear view transforms");
    assert_eq!(config.num_view_transforms(), 0);
    assert!(config.view_transform("UnitConfigViewTransform").is_none());

    config.try_clear_color_spaces().expect("clear color spaces");
    assert_eq!(config.num_color_spaces(), 0);
    assert!(config.color_space("UnitConfigUsedColorSpace").is_none());

    config.clear_all();
    assert_eq!(config.num_color_spaces(), 0);
    assert_eq!(config.num_looks(), 0);
    assert_eq!(config.num_named_transforms(), 0);
    assert_eq!(config.num_view_transforms(), 0);
    assert_eq!(config.num_displays_all(), 0);
    assert!(config.color_space("UnitConfigUsedColorSpace").is_none());
    assert_eq!(config.num_displays(), 0);
    assert_eq!(config.display(0).as_deref(), Some(""));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_collection_handles_survive_parent_drop_behavior() {
    let _guard = config_collection_test_lock();
    if is_stub() {
        return;
    }

    let (color_space, look, named_transform, view_transform) = {
        let config = Config::raw().expect("raw config");
        let color_space = identity_color_space("UnitOwnedColorSpace");
        let look = scaling_look("UnitOwnedLook", "UnitOwnedColorSpace");
        let named_transform = scaling_named_transform("UnitOwnedNamedTransform");
        let view_transform = identity_view_transform("UnitOwnedViewTransform");

        config.add_color_space(&color_space);
        config.add_look(&look);
        config.add_named_transform(&named_transform);
        config.add_view_transform(&view_transform);

        (
            config
                .color_space("UnitOwnedColorSpace")
                .expect("owned color space"),
            config.look("UnitOwnedLook").expect("owned look"),
            config
                .named_transform("UnitOwnedNamedTransform")
                .expect("owned named transform"),
            config
                .view_transform("UnitOwnedViewTransform")
                .expect("owned view transform"),
        )
    };

    assert_eq!(color_space.name().as_deref(), Some("UnitOwnedColorSpace"));
    assert_eq!(look.name().as_deref(), Some("UnitOwnedLook"));
    assert_eq!(
        named_transform.name().as_deref(),
        Some("UnitOwnedNamedTransform")
    );
    assert_eq!(
        view_transform.name().as_deref(),
        Some("UnitOwnedViewTransform")
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_collection_registration_errors_surface_behavior() {
    let _guard = config_collection_test_lock();
    if is_stub() {
        return;
    }

    let config = Config::raw().expect("raw config");

    let unnamed_color_space = ColorSpace::create().expect("color space create");
    let add_color_space_err = config
        .try_add_color_space(&unnamed_color_space)
        .expect_err("unnamed color space should fail");
    assert!(
        matches!(add_color_space_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {add_color_space_err:?}"
    );

    let unnamed_named_transform = NamedTransform::create().expect("named transform create");
    let add_named_transform_err = config
        .try_add_named_transform(&unnamed_named_transform)
        .expect_err("unnamed named transform should fail");
    assert!(
        matches!(add_named_transform_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {add_named_transform_err:?}"
    );

    let unnamed_look = Look::create().expect("look create");
    let add_look_err = config
        .try_add_look(&unnamed_look)
        .expect_err("unnamed look should fail");
    assert!(
        matches!(add_look_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {add_look_err:?}"
    );

    let empty_view_transform =
        ViewTransform::create(ReferenceSpaceType::Scene).expect("view transform create");
    let add_view_transform_err = config
        .try_add_view_transform(&empty_view_transform)
        .expect_err("view transform without name/transform should fail");
    assert!(
        matches!(add_view_transform_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {add_view_transform_err:?}"
    );
}
