//! Config behavioral tests for real OCIO builds.
//!
//! In stub mode these tests return early after verifying the entry points can
//! be reached. In real mode they assert concrete OCIO behavior.

mod common;
use common::*;

use ocio_rs::{Config, ReferenceSpaceType, ViewTransform};

const OPTIMIZATION_DEFAULT: u64 = 0;

#[test]
fn config_processor_from_configs_identity_behavior() {
    if is_stub() {
        return;
    }

    let src_config = create_test_config().expect("src raw config");
    let dst_config = create_test_config().expect("dst raw config");

    let processor = Config::processor_from_configs(&src_config, "raw", &dst_config, "raw")
        .expect("processor_from_configs");
    let cpu = processor
        .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("optimized_cpu_processor");

    let mut pixel = [0.25f32, 0.5, 0.75, 1.0];
    let original = pixel;
    cpu.apply_rgba(&mut pixel);

    assert_close(pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(pixel[3] as f64, original[3] as f64, 1e-6);
}

#[test]
fn config_processor_from_configs_with_contexts_identity_behavior() {
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("driver raw config");
    let src_config = create_test_config().expect("src raw config");
    let dst_config = create_test_config().expect("dst raw config");
    let src_ctx = src_config.current_context().expect("src current_context");
    let dst_ctx = dst_config.current_context().expect("dst current_context");

    let processor = config
        .processor_from_configs_with_contexts(
            &src_ctx,
            &src_config,
            "raw",
            &dst_ctx,
            &dst_config,
            "raw",
        )
        .expect("processor_from_configs_with_contexts");
    let cpu = processor
        .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("optimized_cpu_processor");

    let mut pixel = [0.1f32, 0.2, 0.3, 0.4];
    let original = pixel;
    cpu.apply_rgba(&mut pixel);

    assert_close(pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(pixel[3] as f64, original[3] as f64, 1e-6);
}

#[test]
fn config_processor_from_configs_with_interchange_identity_behavior() {
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("driver raw config");
    let src_config = create_test_config().expect("src raw config");
    let dst_config = create_test_config().expect("dst raw config");

    let processor = config
        .processor_from_configs_with_interchange(&src_config, "raw", "", &dst_config, "raw", "")
        .expect("processor_from_configs_with_interchange");
    let cpu = processor
        .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("optimized_cpu_processor");

    let mut pixel = [0.15f32, 0.35, 0.55, 1.0];
    let original = pixel;
    cpu.apply_rgba(&mut pixel);

    assert_close(pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(pixel[3] as f64, original[3] as f64, 1e-6);
}

#[test]
fn config_processor_from_configs_with_contexts_and_interchange_identity_behavior() {
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("driver raw config");
    let src_config = create_test_config().expect("src raw config");
    let dst_config = create_test_config().expect("dst raw config");
    let src_ctx = src_config.current_context().expect("src current_context");
    let dst_ctx = dst_config.current_context().expect("dst current_context");

    let processor = config
        .processor_from_configs_with_contexts_and_interchange(
            &src_ctx,
            &src_config,
            "raw",
            "",
            &dst_ctx,
            &dst_config,
            "raw",
            "",
        )
        .expect("processor_from_configs_with_contexts_and_interchange");
    let cpu = processor
        .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("optimized_cpu_processor");

    let mut pixel = [0.05f32, 0.25, 0.45, 1.0];
    let original = pixel;
    cpu.apply_rgba(&mut pixel);

    assert_close(pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(pixel[3] as f64, original[3] as f64, 1e-6);
}

#[test]
fn config_processor_from_configs_to_display_identity_behavior() {
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("driver raw config");
    let src_config = create_test_config().expect("src raw config");
    let dst_config = create_test_config().expect("dst raw config");
    dst_config
        .add_display("UnitDisplay", "UnitView", "raw", "")
        .expect("add_display");

    let processor = config
        .processor_from_configs_to_display(
            &src_config,
            "raw",
            &dst_config,
            "UnitDisplay",
            "UnitView",
            ocio_rs::TransformDirection::Forward,
        )
        .expect("processor_from_configs_to_display");
    let cpu = processor
        .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("optimized_cpu_processor");

    let mut pixel = [0.6f32, 0.3, 0.1, 1.0];
    let original = pixel;
    cpu.apply_rgba(&mut pixel);

    assert_close(pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(pixel[3] as f64, original[3] as f64, 1e-6);
}

#[test]
fn config_processor_from_configs_to_display_with_interchange_identity_behavior() {
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("driver raw config");
    let src_config = create_test_config().expect("src raw config");
    let dst_config = create_test_config().expect("dst raw config");
    dst_config
        .add_display("UnitDisplay", "UnitView", "raw", "")
        .expect("add_display");

    let processor = config
        .processor_from_configs_to_display_with_interchange(
            &src_config,
            "raw",
            "",
            &dst_config,
            "UnitDisplay",
            "UnitView",
            "",
            ocio_rs::TransformDirection::Forward,
        )
        .expect("processor_from_configs_to_display_with_interchange");
    let cpu = processor
        .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("optimized_cpu_processor");

    let mut pixel = [0.7f32, 0.2, 0.4, 1.0];
    let original = pixel;
    cpu.apply_rgba(&mut pixel);

    assert_close(pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(pixel[3] as f64, original[3] as f64, 1e-6);
}

#[test]
fn config_processor_from_configs_to_display_with_contexts_identity_behavior() {
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

    let processor = config
        .processor_from_configs_to_display_with_contexts(
            &src_ctx,
            &src_config,
            "raw",
            &dst_ctx,
            &dst_config,
            "UnitDisplay",
            "UnitView",
            ocio_rs::TransformDirection::Forward,
        )
        .expect("processor_from_configs_to_display_with_contexts");
    let cpu = processor
        .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("optimized_cpu_processor");

    let mut pixel = [0.2f32, 0.4, 0.8, 1.0];
    let original = pixel;
    cpu.apply_rgba(&mut pixel);

    assert_close(pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(pixel[3] as f64, original[3] as f64, 1e-6);
}

#[test]
fn config_processor_from_configs_to_display_with_contexts_and_interchange_identity_behavior() {
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

    let processor = config
        .processor_from_configs_to_display_with_contexts_and_interchange(
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
        )
        .expect("processor_from_configs_to_display_with_contexts_and_interchange");
    let cpu = processor
        .optimized_cpu_processor(OPTIMIZATION_DEFAULT)
        .expect("optimized_cpu_processor");

    let mut pixel = [0.9f32, 0.6, 0.3, 1.0];
    let original = pixel;
    cpu.apply_rgba(&mut pixel);

    assert_close(pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(pixel[3] as f64, original[3] as f64, 1e-6);
}

#[test]
fn config_display_view_metadata_round_trip_behavior() {
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let view_transform =
        ViewTransform::create(ReferenceSpaceType::Scene).expect("create view transform");
    view_transform.set_name("UnitTestView").expect("set name");
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
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let view_transform =
        ViewTransform::create(ReferenceSpaceType::Scene).expect("create view transform");
    view_transform
        .set_name("SharedUnitViewTransform")
        .expect("set name");
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
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let view_transform =
        ViewTransform::create(ReferenceSpaceType::Scene).expect("create view transform");
    view_transform
        .set_name("VirtualUnitView")
        .expect("set name");
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
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let view_transform =
        ViewTransform::create(ReferenceSpaceType::Scene).expect("create view transform");
    view_transform
        .set_name("VirtualSharedUnitTransform")
        .expect("set name");
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
