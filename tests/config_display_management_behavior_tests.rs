//! Config display-management behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/config.rs`. In bundled/real mode they validate shared-view, display,
//! and virtual-display lifecycle behavior.

mod common;
use common::*;

#[cfg(feature = "v2_5")]
use std::collections::BTreeSet;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::MatrixTransform;
#[cfg(feature = "v2_5")]
use ocio_rs::SearchReferenceSpaceType;
use ocio_rs::{ReferenceSpaceType, ViewTransform, ViewTransformDirection};

fn config_display_management_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn identity_view_transform(name: &str) -> ViewTransform {
    let view_transform =
        ViewTransform::create(ReferenceSpaceType::Scene).expect("create view transform");
    view_transform.set_name(name).expect("set name");
    let identity = MatrixTransform::identity().expect("identity matrix");
    view_transform.set_transform(Some(&identity), ViewTransformDirection::ToReference);
    view_transform.set_transform(Some(&identity), ViewTransformDirection::FromReference);
    view_transform
}

#[cfg(feature = "v2_5")]
fn virtual_view_names(
    config: &ocio_rs::Config,
    reference_space: SearchReferenceSpaceType,
) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for index in 0..config.virtual_display_num_views(reference_space) {
        let name = config
            .virtual_display_view(reference_space, index)
            .expect("virtual display view name");
        names.insert(name);
    }
    names
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_shared_view_and_display_lifecycle_behavior() {
    let _guard = config_display_management_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");
    let initial_displays = config.num_displays_all();
    assert_eq!(
        config
            .try_num_displays_all()
            .expect("all-display count query"),
        initial_displays
    );

    let view_transform = identity_view_transform("UnitLifecycleSharedTransform");
    config.add_view_transform(&view_transform);

    config
        .add_shared_view(
            "UnitLifecycleSharedView",
            "UnitLifecycleSharedTransform",
            "raw",
            "",
            "",
            "shared lifecycle test",
        )
        .expect("add shared view");
    config
        .add_display_shared_view("UnitLifecycleDisplay", "UnitLifecycleSharedView")
        .expect("add display shared view");

    assert!(config.has_view("UnitLifecycleDisplay", "UnitLifecycleSharedView"));
    assert!(config.is_view_shared("UnitLifecycleDisplay", "UnitLifecycleSharedView"));
    assert_eq!(config.num_views("UnitLifecycleDisplay"), 1);
    assert_eq!(
        config.view("UnitLifecycleDisplay", 0).as_deref(),
        Some("UnitLifecycleSharedView")
    );
    assert_eq!(
        config
            .try_view("UnitLifecycleDisplay", 0)
            .expect("display view query")
            .as_deref(),
        Some("UnitLifecycleSharedView")
    );
    assert_eq!(
        config
            .display_view_transform_name("UnitLifecycleDisplay", "UnitLifecycleSharedView")
            .as_deref(),
        Some("UnitLifecycleSharedTransform")
    );
    assert_eq!(config.num_displays_all(), initial_displays + 1);
    let all_display_index = config
        .try_display_all_index("UnitLifecycleDisplay")
        .expect("all-display index query");
    assert!(all_display_index >= 0);
    assert_eq!(
        config
            .try_display_all(all_display_index)
            .expect("all-display name query")
            .as_deref(),
        Some("UnitLifecycleDisplay")
    );
    assert!(config
        .try_display_all_index("UnitLifecycleDisplay\0")
        .is_err());

    config
        .remove_view("UnitLifecycleDisplay", "UnitLifecycleSharedView")
        .expect("remove display view");
    assert!(!config.has_view("UnitLifecycleDisplay", "UnitLifecycleSharedView"));

    config
        .add_display_shared_view("UnitLifecycleDisplay", "UnitLifecycleSharedView")
        .expect("re-add display shared view");
    assert!(config.has_view("UnitLifecycleDisplay", "UnitLifecycleSharedView"));

    config
        .remove_shared_view("UnitLifecycleSharedView")
        .expect("remove shared view");
    assert!(!config.has_view("UnitLifecycleDisplay", "UnitLifecycleSharedView"));

    config
        .add_shared_view(
            "UnitLifecycleSharedView",
            "UnitLifecycleSharedTransform",
            "raw",
            "",
            "",
            "shared lifecycle test",
        )
        .expect("re-add shared view");
    assert!(config.has_view("UnitLifecycleDisplay", "UnitLifecycleSharedView"));

    config.try_clear_shared_views().expect("clear shared views");
    assert!(!config.has_view("UnitLifecycleDisplay", "UnitLifecycleSharedView"));

    config.try_clear_displays().expect("clear displays");
    assert_eq!(config.num_displays_all(), 0);
    assert_eq!(config.num_displays(), 0);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn clear_all_preserves_semantics_on_the_baseline_interface() {
    let _guard = config_display_management_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");
    let view_transform = identity_view_transform("BaselineClearSharedTransform");
    config.add_view_transform(&view_transform);
    config
        .add_shared_view(
            "BaselineClearSharedView",
            "BaselineClearSharedTransform",
            "raw",
            "",
            "",
            "baseline clear-all test",
        )
        .expect("add shared view");
    config
        .add_display_shared_view("BaselineClearDisplay", "BaselineClearSharedView")
        .expect("attach shared view to display");
    assert_eq!(config.num_views("BaselineClearDisplay"), 1);
    config
        .try_clear_shared_views()
        .expect("clear baseline shared views");
    assert_eq!(config.num_views("BaselineClearDisplay"), 0);
    config
        .set_active_displays("DisplayA,DisplayB")
        .expect("set active displays");
    config
        .set_active_views("ViewA,ViewB")
        .expect("set active views");

    config.try_clear_all().expect("clear all collections");

    assert!(config.active_displays().unwrap_or_default().is_empty());
    assert!(config.active_views().unwrap_or_default().is_empty());
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_virtual_display_lifecycle_behavior() {
    let _guard = config_display_management_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");

    let view_transform = identity_view_transform("UnitLifecycleVirtualTransform");
    config.add_view_transform(&view_transform);

    config
        .add_shared_view(
            "UnitLifecycleVirtualSharedView",
            "UnitLifecycleVirtualTransform",
            "raw",
            "",
            "",
            "virtual shared lifecycle test",
        )
        .expect("add shared view");
    config
        .add_virtual_display_shared_view("UnitLifecycleVirtualSharedView")
        .expect("add virtual display shared view");
    config
        .add_virtual_display_view(
            "UnitLifecycleVirtualView",
            "UnitLifecycleVirtualTransform",
            "raw",
            "",
            "",
            "virtual lifecycle test",
        )
        .expect("add virtual display view");

    assert!(config.has_virtual_view("UnitLifecycleVirtualSharedView"));
    assert!(config.has_virtual_view("UnitLifecycleVirtualView"));
    assert!(config.is_virtual_view_shared("UnitLifecycleVirtualSharedView"));
    assert!(!config.is_virtual_view_shared("UnitLifecycleVirtualView"));

    assert_eq!(
        config
            .virtual_display_view_transform_name("UnitLifecycleVirtualView")
            .as_deref(),
        Some("UnitLifecycleVirtualTransform")
    );
    assert_eq!(
        config
            .try_virtual_display_view_transform_name("UnitLifecycleVirtualView")
            .expect("virtual display transform-name query")
            .as_deref(),
        Some("UnitLifecycleVirtualTransform")
    );
    assert_eq!(
        config
            .virtual_display_view_color_space_name("UnitLifecycleVirtualView")
            .as_deref(),
        Some("raw")
    );
    assert_eq!(
        config
            .try_virtual_display_view_color_space_name("UnitLifecycleVirtualView")
            .expect("virtual display color-space query")
            .as_deref(),
        Some("raw")
    );
    assert_eq!(
        config
            .try_virtual_display_view_looks("UnitLifecycleVirtualView")
            .expect("virtual display looks query"),
        config.virtual_display_view_looks("UnitLifecycleVirtualView")
    );
    assert_eq!(
        config
            .try_virtual_display_view_rule("UnitLifecycleVirtualView")
            .expect("virtual display rule query"),
        config.virtual_display_view_rule("UnitLifecycleVirtualView")
    );
    assert_eq!(
        config
            .try_virtual_display_view_description("UnitLifecycleVirtualView")
            .expect("virtual display description query"),
        config.virtual_display_view_description("UnitLifecycleVirtualView")
    );
    assert!(config
        .try_virtual_display_view_description("UnitLifecycleVirtualView\0")
        .is_err());

    let scene_views = virtual_view_names(&config, SearchReferenceSpaceType::Scene);
    let all_views = virtual_view_names(&config, SearchReferenceSpaceType::All);
    assert_eq!(
        scene_views,
        BTreeSet::from([String::from("UnitLifecycleVirtualSharedView")])
    );
    assert!(all_views.is_empty());
    assert_eq!(
        config.virtual_display_num_views(SearchReferenceSpaceType::Scene),
        1
    );
    assert_eq!(
        config
            .try_virtual_display_num_views(SearchReferenceSpaceType::Scene)
            .expect("virtual display view count"),
        1
    );
    assert_eq!(
        config
            .try_virtual_display_view(SearchReferenceSpaceType::Scene, 0)
            .expect("virtual display view query")
            .as_deref(),
        Some("UnitLifecycleVirtualSharedView")
    );

    config
        .remove_virtual_display_view("UnitLifecycleVirtualView")
        .expect("remove virtual display view");
    assert!(!config.has_virtual_view("UnitLifecycleVirtualView"));
    assert!(config.has_virtual_view("UnitLifecycleVirtualSharedView"));
    assert_eq!(
        config.virtual_display_num_views(SearchReferenceSpaceType::Scene),
        1
    );

    config
        .try_clear_virtual_display()
        .expect("clear virtual display");
    assert_eq!(
        config.virtual_display_num_views(SearchReferenceSpaceType::Scene),
        0
    );
    assert!(!config.has_virtual_view("UnitLifecycleVirtualSharedView"));
    assert!(!config.has_virtual_view("UnitLifecycleVirtualView"));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_display_mutation_errors_surface_behavior() {
    let _guard = config_display_management_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");

    let add_display_err = config
        .add_display("UnitDisplay", "BrokenView", "", "")
        .expect_err("empty color-space display view should fail");
    assert!(
        matches!(add_display_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {add_display_err:?}"
    );

    let missing_remove_err = config
        .remove_view("MissingDisplay", "MissingView")
        .expect_err("missing display/view removal should fail");
    assert!(
        matches!(missing_remove_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {missing_remove_err:?}"
    );

    let view_transform = identity_view_transform("UnitErrorSharedTransform");
    config.add_view_transform(&view_transform);
    config
        .add_shared_view(
            "UnitErrorSharedView",
            "UnitErrorSharedTransform",
            "raw",
            "",
            "",
            "shared error test",
        )
        .expect("add shared view");
    config
        .add_display_shared_view("UnitErrorDisplay", "UnitErrorSharedView")
        .expect("add display shared view");

    let duplicate_shared_view_err = config
        .add_display_shared_view("UnitErrorDisplay", "UnitErrorSharedView")
        .expect_err("duplicate display shared view should fail");
    assert!(
        matches!(duplicate_shared_view_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {duplicate_shared_view_err:?}"
    );

    assert_eq!(
        config
            .try_num_displays_all()
            .expect("all-display query after prior OCIO error"),
        config.num_displays_all()
    );
    assert_eq!(
        config
            .try_active_displays()
            .expect("active-display query after prior OCIO error"),
        config.active_displays()
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_icc_display_instantiation_surfaces_missing_virtual_display_behavior() {
    let _guard = config_display_management_test_lock();
    if is_stub() {
        return;
    }

    let config = ocio_rs::Config::raw().expect("raw config");
    let err = config
        .try_instantiate_display_from_icc_profile("missing-profile.icc")
        .expect_err("missing virtual display should be an OCIO error");
    assert!(
        matches!(err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );
    assert!(config
        .try_instantiate_display_from_monitor_name("monitor\0name")
        .is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_virtual_display_mutation_errors_surface_behavior() {
    let _guard = config_display_management_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");

    let view_transform = identity_view_transform("UnitErrorVirtualTransform");
    config.add_view_transform(&view_transform);
    config
        .add_shared_view(
            "UnitErrorVirtualSharedView",
            "UnitErrorVirtualTransform",
            "raw",
            "",
            "",
            "virtual error test",
        )
        .expect("add shared view");

    let empty_view_err = config
        .add_virtual_display_view("", "UnitErrorVirtualTransform", "raw", "", "", "")
        .expect_err("empty virtual display view name should fail");
    assert!(
        matches!(empty_view_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {empty_view_err:?}"
    );

    config
        .add_virtual_display_shared_view("UnitErrorVirtualSharedView")
        .expect("add virtual display shared view");
    let duplicate_shared_view_err = config
        .add_virtual_display_shared_view("UnitErrorVirtualSharedView")
        .expect_err("duplicate virtual display shared view should fail");
    assert!(
        matches!(duplicate_shared_view_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {duplicate_shared_view_err:?}"
    );
}
