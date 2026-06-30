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
