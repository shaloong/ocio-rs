//! Config behavioral tests for real OCIO builds.
//!
//! In stub mode these tests return early after verifying the entry points can
//! be reached. In real mode they assert concrete OCIO behavior.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::MatrixTransform;
use ocio_rs::{ReferenceSpaceType, ViewTransform, ViewTransformDirection};

fn config_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configured_view_transform(name: &str) -> ViewTransform {
    let view_transform =
        ViewTransform::create(ReferenceSpaceType::Scene).expect("create view transform");
    view_transform.set_name(name).expect("set name");
    let identity = MatrixTransform::identity().expect("identity matrix");
    view_transform.set_transform(Some(&identity), ViewTransformDirection::ToReference);
    view_transform
}

#[test]
fn config_processor_from_configs_with_interchange_rejects_empty_interchange_behavior() {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("driver raw config");
    let src_config = create_test_config().expect("src raw config");
    let dst_config = create_test_config().expect("dst raw config");

    let processor = config.processor_from_configs_with_interchange(
        &src_config,
        "raw",
        "",
        &dst_config,
        "raw",
        "",
    );
    assert!(processor.is_err());
}

#[test]
fn config_processor_from_configs_with_contexts_and_interchange_rejects_empty_interchange_behavior()
{
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("driver raw config");
    let src_config = create_test_config().expect("src raw config");
    let dst_config = create_test_config().expect("dst raw config");
    let src_ctx = src_config.current_context().expect("src current_context");
    let dst_ctx = dst_config.current_context().expect("dst current_context");

    let processor = config.processor_from_configs_with_contexts_and_interchange(
        &src_ctx,
        &src_config,
        "raw",
        "",
        &dst_ctx,
        &dst_config,
        "raw",
        "",
    );
    assert!(processor.is_err());
}

#[test]
fn config_processor_from_configs_to_display_with_interchange_rejects_empty_interchange_behavior() {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("driver raw config");
    let src_config = create_test_config().expect("src raw config");
    let dst_config = create_test_config().expect("dst raw config");
    dst_config
        .add_display("UnitDisplay", "UnitView", "raw", "")
        .expect("add_display");

    let processor = config.processor_from_configs_to_display_with_interchange(
        &src_config,
        "raw",
        "",
        &dst_config,
        "UnitDisplay",
        "UnitView",
        "",
        ocio_rs::TransformDirection::Forward,
    );
    assert!(processor.is_err());
}

#[test]
fn config_processor_from_configs_to_display_with_contexts_and_interchange_rejects_empty_interchange_behavior(
) {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("driver raw config");
    let src_config = create_test_config().expect("src raw config");
    let dst_config = create_test_config().expect("dst raw config");
    let src_ctx = src_config.current_context().expect("src current_context");
    let dst_ctx = dst_config.current_context().expect("dst current_context");
    dst_config
        .add_display("UnitDisplay", "UnitView", "raw", "")
        .expect("add_display");

    let processor = config.processor_from_configs_to_display_with_contexts_and_interchange(
        &src_ctx,
        &src_config,
        "raw",
        "",
        &dst_ctx,
        &dst_config,
        "UnitDisplay",
        "UnitView",
        "",
        ocio_rs::TransformDirection::Forward,
    );
    assert!(processor.is_err());
}

#[test]
fn config_display_view_metadata_round_trip_behavior() {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let view_transform = configured_view_transform("UnitTestView");
    config.add_view_transform(&view_transform);

    config
        .add_display_view_detailed(
            "UnitDisplay",
            "UnitView",
            "UnitTestView",
            "raw",
            "",
            "",
            "Unit display description",
        )
        .expect("add_display_view_detailed");
    config
        .set_default_view_transform_name("UnitTestView")
        .expect("set_default_view_transform_name");

    assert_eq!(
        config.default_view_transform_name().as_deref(),
        Some("UnitTestView")
    );
    assert_eq!(
        config
            .display_view_transform_name("UnitDisplay", "UnitView")
            .as_deref(),
        Some("UnitTestView")
    );
    assert_eq!(
        config
            .display_view_color_space_name("UnitDisplay", "UnitView")
            .as_deref(),
        Some("raw")
    );
    assert_eq!(
        config
            .display_view_description("UnitDisplay", "UnitView")
            .as_deref(),
        Some("Unit display description")
    );
}

#[test]
fn config_display_shared_view_metadata_round_trip_behavior() {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let view_transform = configured_view_transform("SharedUnitViewTransform");
    config.add_view_transform(&view_transform);

    config
        .add_shared_view(
            "SharedUnitView",
            "SharedUnitViewTransform",
            "raw",
            "",
            "",
            "Shared display description",
        )
        .expect("add_shared_view");
    config
        .add_display_shared_view("SharedDisplay", "SharedUnitView")
        .expect("add_display_shared_view");

    assert_eq!(
        config
            .display_view_transform_name("SharedDisplay", "SharedUnitView")
            .as_deref(),
        Some("SharedUnitViewTransform")
    );
    assert_eq!(
        config
            .display_view_color_space_name("SharedDisplay", "SharedUnitView")
            .as_deref(),
        Some("raw")
    );
    assert_eq!(
        config
            .display_view_description("SharedDisplay", "SharedUnitView")
            .as_deref(),
        Some("Shared display description")
    );
}

#[test]
fn config_virtual_display_metadata_round_trip_behavior() {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let view_transform = configured_view_transform("VirtualUnitView");
    config.add_view_transform(&view_transform);

    config
        .add_virtual_display_view(
            "VirtualUnitFilm",
            "VirtualUnitView",
            "raw",
            "",
            "",
            "Virtual display description",
        )
        .expect("add_virtual_display_view");

    assert!(config.has_virtual_view("VirtualUnitFilm"));
    assert!(!config.is_virtual_view_shared("VirtualUnitFilm"));
    assert_eq!(
        config
            .virtual_display_view_transform_name("VirtualUnitFilm")
            .as_deref(),
        Some("VirtualUnitView")
    );
    assert_eq!(
        config
            .virtual_display_view_color_space_name("VirtualUnitFilm")
            .as_deref(),
        Some("raw")
    );
    assert_eq!(
        config
            .virtual_display_view_description("VirtualUnitFilm")
            .as_deref(),
        Some("Virtual display description")
    );
}

#[test]
fn config_virtual_display_shared_view_behavior() {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let view_transform = configured_view_transform("VirtualSharedUnitTransform");
    config.add_view_transform(&view_transform);

    config
        .add_shared_view(
            "VirtualSharedUnitView",
            "VirtualSharedUnitTransform",
            "raw",
            "",
            "",
            "Virtual shared description",
        )
        .expect("add_shared_view");
    config
        .add_virtual_display_shared_view("VirtualSharedUnitView")
        .expect("add_virtual_display_shared_view");

    assert!(config.has_virtual_view("VirtualSharedUnitView"));
    assert!(config.is_virtual_view_shared("VirtualSharedUnitView"));
    assert_eq!(
        config
            .virtual_display_view_transform_name("VirtualSharedUnitView")
            .as_deref(),
        Some("VirtualSharedUnitTransform")
    );
    assert_eq!(
        config
            .virtual_display_view_color_space_name("VirtualSharedUnitView")
            .as_deref(),
        Some("raw")
    );
    assert_eq!(
        config
            .virtual_display_view_description("VirtualSharedUnitView")
            .as_deref(),
        Some("Virtual shared description")
    );
}

#[test]
fn config_role_mutation_errors_surface_behavior() {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let err = config
        .set_role("", "raw")
        .expect_err("empty role name should fail");
    assert!(
        matches!(err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );
}

#[test]
fn config_processor_missing_color_space_errors_surface_behavior() {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let err = match config.processor("definitely_missing_colorspace", "raw") {
        Ok(_) => panic!("missing color space should fail"),
        Err(err) => err,
    };
    assert!(
        matches!(err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );
}

#[test]
fn config_builtin_processor_invalid_builtin_errors_surface_behavior() {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let err = match config.processor_from_builtin_color_space(
        "definitely_missing_builtin",
        &config,
        "raw",
    ) {
        Ok(_) => panic!("missing builtin color space should fail"),
        Err(err) => err,
    };
    assert!(
        matches!(err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );
}
