//! Config behavioral tests for real OCIO builds.
//!
//! In stub mode these tests return early after verifying the entry points can
//! be reached. In real mode they assert concrete OCIO behavior.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::MatrixTransform;
use ocio_rs::{
    BuiltinConfigRegistry, Config, ReferenceSpaceType, ViewTransform, ViewTransformDirection,
};

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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn cross_config_processors_apply_explicit_interchange_and_contexts() {
    use ocio_rs::transform::MatrixTransform;
    use ocio_rs::{ColorSpace, ColorSpaceDirection, Config};
    let _guard = config_test_lock();
    let src = Config::create().unwrap();
    let dst = Config::create().unwrap();
    for (config, reference_name, color_name, factor, direction) in [
        (
            &src,
            "src_reference",
            "input",
            2.0,
            ColorSpaceDirection::ToReference,
        ),
        (
            &dst,
            "dst_reference",
            "output",
            3.0,
            ColorSpaceDirection::FromReference,
        ),
    ] {
        let reference = ColorSpace::create().unwrap();
        reference.set_name(reference_name).unwrap();
        config.try_add_color_space(&reference).unwrap();
        config.set_role("aces_interchange", reference_name).unwrap();
        let color = ColorSpace::create().unwrap();
        color.set_name(color_name).unwrap();
        color
            .try_set_transform(
                &MatrixTransform::scale(&[factor, factor, factor, 1.0]).unwrap(),
                direction,
            )
            .unwrap();
        config.try_add_color_space(&color).unwrap();
    }
    let src_ctx = src.current_context().unwrap();
    let dst_ctx = dst.current_context().unwrap();
    let processors = [
        Config::processor_from_configs(&src, "input", &dst, "output").unwrap(),
        src.processor_from_configs_with_contexts(&src_ctx, &src, "input", &dst_ctx, &dst, "output")
            .unwrap(),
        src.processor_from_configs_with_interchange(
            &src,
            "input",
            "src_reference",
            &dst,
            "output",
            "dst_reference",
        )
        .unwrap(),
        src.processor_from_configs_with_contexts_and_interchange(
            &src_ctx,
            &src,
            "input",
            "src_reference",
            &dst_ctx,
            &dst,
            "output",
            "dst_reference",
        )
        .unwrap(),
    ];
    for processor in processors {
        let mut pixel = [0.1, 0.2, 0.3, 1.0];
        processor
            .optimized_cpu_processor(0)
            .unwrap()
            .try_apply_rgba(&mut pixel)
            .unwrap();
        for (actual, expected) in pixel.into_iter().zip([0.6, 1.2, 1.8, 1.0]) {
            assert!((actual - expected).abs() < 1e-5, "{actual} != {expected}");
        }
    }
    dst.add_display("Screen", "View", "output", "").unwrap();
    for direction in [
        ocio_rs::TransformDirection::Forward,
        ocio_rs::TransformDirection::Inverse,
    ] {
        let processors = [
            src.processor_from_configs_to_display(&src, "input", &dst, "Screen", "View", direction)
                .unwrap(),
            src.processor_from_configs_to_display_with_contexts(
                &src_ctx, &src, "input", &dst_ctx, &dst, "Screen", "View", direction,
            )
            .unwrap(),
            src.processor_from_configs_to_display_with_interchange(
                &src,
                "input",
                "src_reference",
                &dst,
                "Screen",
                "View",
                "dst_reference",
                direction,
            )
            .unwrap(),
            src.processor_from_configs_to_display_with_contexts_and_interchange(
                &src_ctx,
                &src,
                "input",
                "src_reference",
                &dst_ctx,
                &dst,
                "Screen",
                "View",
                "dst_reference",
                direction,
            )
            .unwrap(),
        ];
        for processor in processors {
            let mut pixel = if direction == ocio_rs::TransformDirection::Forward {
                [0.1, 0.2, 0.3, 1.0]
            } else {
                [0.6, 1.2, 1.8, 1.0]
            };
            let expected = if direction == ocio_rs::TransformDirection::Forward {
                [0.6, 1.2, 1.8, 1.0]
            } else {
                [0.1, 0.2, 0.3, 1.0]
            };
            processor
                .optimized_cpu_processor(0)
                .unwrap()
                .try_apply_rgba(&mut pixel)
                .unwrap();
            for (actual, expected) in pixel.into_iter().zip(expected) {
                assert!((actual - expected).abs() < 1e-5, "{actual} != {expected}");
            }
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn cross_config_processors_use_their_respective_contexts_for_luts() {
    use ocio_rs::transform::FileTransform;
    use ocio_rs::{ColorSpace, ColorSpaceDirection, Config, Context};
    let _guard = config_test_lock();
    let source = Config::create().unwrap();
    let destination = Config::create().unwrap();
    for (config, reference, name, path, direction) in [
        (
            &source,
            "src_ref",
            "input",
            "${SRC_LUT_DIR}/lut1.clf",
            ColorSpaceDirection::ToReference,
        ),
        (
            &destination,
            "dst_ref",
            "output",
            "${DST_LUT_DIR}/lut1.clf",
            ColorSpaceDirection::FromReference,
        ),
    ] {
        let reference_space = ColorSpace::create().unwrap();
        reference_space.set_name(reference).unwrap();
        config.try_add_color_space(&reference_space).unwrap();
        config.set_role("aces_interchange", reference).unwrap();
        let file = FileTransform::create().unwrap();
        file.set_src(path).unwrap();
        let space = ColorSpace::create().unwrap();
        space.set_name(name).unwrap();
        space.try_set_transform(&file, direction).unwrap();
        config.try_add_color_space(&space).unwrap();
    }
    let working_dir =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/configs/context_test1");
    let src_context = Context::create().unwrap();
    let dst_context = Context::create().unwrap();
    src_context
        .set_working_dir(working_dir.to_str().unwrap())
        .unwrap();
    dst_context
        .set_working_dir(working_dir.to_str().unwrap())
        .unwrap();
    src_context.set_string_var("SRC_LUT_DIR", "shot1").unwrap();
    dst_context.set_string_var("DST_LUT_DIR", "shot4").unwrap();
    destination
        .add_display("Screen", "View", "output", "")
        .unwrap();
    let processors = [
        source
            .processor_from_configs_with_contexts(
                &src_context,
                &source,
                "input",
                &dst_context,
                &destination,
                "output",
            )
            .unwrap(),
        source
            .processor_from_configs_with_contexts_and_interchange(
                &src_context,
                &source,
                "input",
                "src_ref",
                &dst_context,
                &destination,
                "output",
                "dst_ref",
            )
            .unwrap(),
        source
            .processor_from_configs_to_display_with_contexts(
                &src_context,
                &source,
                "input",
                &dst_context,
                &destination,
                "Screen",
                "View",
                ocio_rs::TransformDirection::Forward,
            )
            .unwrap(),
        source
            .processor_from_configs_to_display_with_contexts_and_interchange(
                &src_context,
                &source,
                "input",
                "src_ref",
                &dst_context,
                &destination,
                "Screen",
                "View",
                "dst_ref",
                ocio_rs::TransformDirection::Forward,
            )
            .unwrap(),
    ];
    for processor in processors {
        let mut pixel = [0.001, 0.002, 0.003, 1.0];
        processor
            .optimized_cpu_processor(0)
            .unwrap()
            .try_apply_rgba(&mut pixel)
            .unwrap();
        for (actual, expected) in pixel.into_iter().zip([0.4, 0.8, 1.2, 1.0]) {
            assert!((actual - expected).abs() < 1e-5, "{actual} != {expected}");
        }
    }
    assert!(source
        .processor_from_configs_with_contexts(
            &dst_context,
            &source,
            "input",
            &src_context,
            &destination,
            "output"
        )
        .is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_static_view_comparisons_preserve_real_behavior() {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    config
        .add_display("UnitDisplay", "UnitView", "raw", "")
        .expect("add display");
    assert!(
        Config::try_are_views_equal(&config, &config, "UnitDisplay", "UnitView")
            .expect("compare display views")
    );

    let view_transform = configured_view_transform("StaticComparisonVirtualView");
    config.add_view_transform(&view_transform);
    config
        .add_virtual_display_view(
            "UnitVirtualView",
            "StaticComparisonVirtualView",
            "raw",
            "",
            "",
            "",
        )
        .expect("add virtual display view");
    assert!(
        Config::try_are_virtual_views_equal(&config, &config, "UnitVirtualView")
            .expect("compare virtual display views")
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_static_color_space_identification_preserves_errors() {
    let _guard = config_test_lock();
    if is_stub() {
        return;
    }

    let registry = BuiltinConfigRegistry::get().expect("builtin config registry");
    let builtin_name = registry.config_name(0).expect("builtin config name");
    let config = Config::create_from_builtin_config(&builtin_name).expect("builtin config");
    let color_space = config
        .color_space_name_by_index(0)
        .expect("builtin color space name");

    assert_eq!(
        Config::try_identify_builtin_color_space(&config, &config, &color_space)
            .expect("identify builtin color space")
            .as_deref(),
        Some(color_space.as_str())
    );
    let (src_interchange, builtin_interchange) =
        Config::try_identify_interchange_space(&config, &color_space, &config, &color_space)
            .expect("identify interchange spaces");
    assert!(src_interchange.is_some());
    assert!(builtin_interchange.is_some());

    assert!(Config::try_identify_builtin_color_space(
        &config,
        &config,
        "definitely_missing_color_space",
    )
    .is_err());
}
