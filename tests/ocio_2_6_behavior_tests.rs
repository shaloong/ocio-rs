#![cfg(feature = "v2_6")]

use ocio_rs::{
    metadata_id_element,
    transform::{FileTransform, FixedFunctionTransform, GroupTransform, MatrixTransform},
    ColorSpace, Config, FixedFunctionStyle, GpuLanguage, GpuShaderDesc, ReferenceSpaceType,
    TransformDirection, ViewWithAliases,
};

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6")]
fn builtin_matching_and_hdr_check_use_native_config() {
    let builtin =
        Config::create_from_builtin_config("studio-config-v5.0.0_aces-v2.1_ocio-v2.6").unwrap();
    assert!(builtin.check_hdr_display_compatibility().unwrap());
    let source = builtin.create_editable_copy().unwrap();
    for _ in 0..2 {
        assert_eq!(
            source
                .locate_builtin_color_space(c"ACEScg", &builtin)
                .unwrap()
                .as_deref(),
            Some("ACEScg")
        );
    }
    assert!(source
        .locate_builtin_color_space(c"no-such-space", &builtin)
        .is_err());
    let source_baker = ocio_rs::Baker::create().unwrap();
    source_baker.set_config(&source).unwrap();
    let source_const = source_baker.const_config().unwrap().unwrap();
    let builtin_baker = ocio_rs::Baker::create().unwrap();
    builtin_baker.set_config(&builtin).unwrap();
    let builtin_const = builtin_baker.const_config().unwrap().unwrap();
    for result in [
        source.locate_builtin_color_space_ref(c"ACEScg", &builtin),
        source.locate_builtin_color_space_ref(c"ACEScg", &builtin_const),
        source_const.locate_builtin_color_space(c"ACEScg", &builtin),
        source_const.locate_builtin_color_space(c"ACEScg", &builtin_const),
    ] {
        assert_eq!(result.unwrap().as_deref(), Some("ACEScg"));
    }
    assert!(source_const
        .locate_builtin_color_space(c"no-such-space", &builtin_const)
        .is_err());
    assert_eq!(
        source_const
            .locate_builtin_color_space(c"ACEScg", &builtin_const)
            .unwrap()
            .as_deref(),
        Some("ACEScg")
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6")]
fn clf_writer_preserves_smpte_id_element() {
    let config = Config::raw().unwrap();
    let group = GroupTransform::create().unwrap();
    group
        .append_transform(&MatrixTransform::scale(&[2.0, 1.0, 0.5, 1.0]).unwrap())
        .unwrap();
    let id = "urn:uuid:9d768121-0cf9-40a3-a8e3-7b49f79858a7";
    let key = metadata_id_element().unwrap().to_str().unwrap();
    group
        .format_metadata()
        .unwrap()
        .add_child_element(key, id)
        .unwrap();
    let clf = group
        .write_to_string(&config, "Academy/ASC Common LUT Format")
        .unwrap()
        .unwrap();
    assert!(clf.contains("http://www.smpte-ra.org/ns/2136-1/2024"));
    assert!(clf.contains(&format!("<Id>{id}</Id>")));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6")]
fn clf_reader_applies_smpte_only_header_in_place() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/clf/smpte_2136_scale.clf");
    let transform = FileTransform::create().unwrap();
    transform.set_src(path.to_str().unwrap()).unwrap();
    let cpu = Config::raw()
        .unwrap()
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    let mut pixels = [0.25, 0.5, 0.75, 1.0, 0.5, 0.25, 1.0, 0.75];
    cpu.try_apply_rgba_pixels(&mut pixels, 2, 4).unwrap();
    assert_eq!(pixels, [0.5, 0.5, 0.375, 1.0, 1.0, 0.25, 0.5, 0.75]);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6")]
fn shared_and_display_view_aliases_round_trip() {
    let config = Config::raw().unwrap().create_editable_copy().unwrap();
    config.set_version(2, 6).unwrap();
    config.set_use_display_aliases(true).unwrap();
    assert!(config.use_display_aliases().unwrap());
    let aliases = &[c"legacy"];
    let view = || ViewWithAliases {
        view: c"Canonical",
        view_transform: c"",
        color_space: c"raw",
        looks: c"",
        rule: c"",
        description: c"",
        aliases,
    };
    config.add_shared_view_with_aliases(view()).unwrap();
    let no_aliases = ViewWithAliases {
        view: c"NoAliases",
        aliases: &[],
        ..view()
    };
    config.add_shared_view_with_aliases(no_aliases).unwrap();
    assert_eq!(
        config.num_display_view_aliases(c"", c"NoAliases").unwrap(),
        0
    );
    assert_eq!(
        config.display_view_alias(c"", c"NoAliases", 0).unwrap(),
        None
    );
    let conflicting = ViewWithAliases {
        view: c"Conflicting",
        ..view()
    };
    assert!(config.add_shared_view_with_aliases(conflicting).is_err());
    assert_eq!(
        config.num_display_view_aliases(c"", c"Canonical").unwrap(),
        1
    );
    assert_eq!(
        config
            .display_view_alias(c"", c"Canonical", 0)
            .unwrap()
            .as_deref(),
        Some("legacy")
    );
    assert_eq!(
        config.display_view_alias(c"", c"Canonical", 9).unwrap(),
        None
    );
    config
        .add_display_view_with_aliases(c"Display", view())
        .unwrap();
    assert!(config
        .has_display_view_alias(c"Display", c"Canonical", c"legacy")
        .unwrap());
    assert_eq!(
        config
            .canonical_display_name(c"Display")
            .unwrap()
            .as_deref(),
        Some("Display")
    );
    assert_eq!(
        config
            .canonical_view_name(c"Display", c"legacy")
            .unwrap()
            .as_deref(),
        Some("Canonical")
    );
    assert_eq!(
        config
            .resolved_display_view_color_space_name(c"Display", c"legacy")
            .unwrap()
            .as_deref(),
        Some("raw")
    );
    assert_eq!(config.display_description(c"missing").unwrap(), None);
    assert!(!config.check_hdr_display_compatibility().unwrap());
    let baker = ocio_rs::Baker::create().unwrap();
    baker.set_config(&config).unwrap();
    let shared = baker.const_config().unwrap().unwrap();
    assert_eq!(
        shared.canonical_display_name(c"Display").unwrap(),
        config.canonical_display_name(c"Display").unwrap()
    );
    assert_eq!(
        shared.canonical_view_name(c"Display", c"legacy").unwrap(),
        config.canonical_view_name(c"Display", c"legacy").unwrap()
    );
    assert_eq!(
        shared
            .resolved_display_view_color_space_name(c"Display", c"legacy")
            .unwrap(),
        config
            .resolved_display_view_color_space_name(c"Display", c"legacy")
            .unwrap()
    );
    assert_eq!(shared.display_description(c"missing").unwrap(), None);
    assert_eq!(
        shared
            .num_display_view_aliases(c"Display", c"Canonical")
            .unwrap(),
        1
    );
    assert_eq!(
        shared
            .display_view_alias(c"Display", c"Canonical", 0)
            .unwrap()
            .as_deref(),
        Some("legacy")
    );
    assert_eq!(
        shared
            .display_view_alias(c"Display", c"Canonical", 9)
            .unwrap(),
        None
    );
    assert!(shared
        .has_display_view_alias(c"Display", c"Canonical", c"legacy")
        .unwrap());
    assert!(shared.use_display_aliases().unwrap());
    assert_eq!(
        shared.check_hdr_display_compatibility().unwrap(),
        config.check_hdr_display_compatibility().unwrap()
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6")]
fn display_color_space_alias_resolves_only_when_enabled() {
    let config = Config::raw().unwrap().create_editable_copy().unwrap();
    config.set_version(2, 6).unwrap();
    let display = ColorSpace::create_with_reference_space(ReferenceSpaceType::Display).unwrap();
    assert_eq!(display.reference_space_type(), ReferenceSpaceType::Display);
    display.set_name("Screen").unwrap();
    display.set_description("Reference monitor").unwrap();
    display.add_alias("old-screen").unwrap();
    config.try_add_color_space(&display).unwrap();
    config
        .add_display_view_with_aliases(
            c"Screen",
            ViewWithAliases {
                view: c"Standard",
                view_transform: c"",
                color_space: c"Screen",
                looks: c"",
                rule: c"",
                description: c"",
                aliases: &[c"old-view"],
            },
        )
        .unwrap();
    assert_ne!(
        config
            .canonical_display_name(c"old-screen")
            .unwrap()
            .as_deref(),
        Some("Screen")
    );
    assert_eq!(
        config
            .canonical_view_name(c"Screen", c"old-view")
            .unwrap()
            .as_deref(),
        Some("Standard")
    );
    let before_baker = ocio_rs::Baker::create().unwrap();
    before_baker.set_config(&config).unwrap();
    let before = before_baker.const_config().unwrap().unwrap();
    drop(before_baker);
    assert!(!before.use_display_aliases().unwrap());
    assert_ne!(
        before
            .canonical_display_name(c"old-screen")
            .unwrap()
            .as_deref(),
        Some("Screen")
    );
    assert_eq!(
        before
            .canonical_view_name(c"Screen", c"old-view")
            .unwrap()
            .as_deref(),
        Some("Standard")
    );
    config.set_use_display_aliases(true).unwrap();
    let after_baker = ocio_rs::Baker::create().unwrap();
    after_baker.set_config(&config).unwrap();
    let after = after_baker.const_config().unwrap().unwrap();
    drop(after_baker);
    assert!(after.use_display_aliases().unwrap());
    assert_eq!(
        after
            .canonical_display_name(c"old-screen")
            .unwrap()
            .as_deref(),
        Some("Screen")
    );
    assert_eq!(
        after
            .canonical_view_name(c"old-screen", c"old-view")
            .unwrap()
            .as_deref(),
        Some("Standard")
    );
    assert_eq!(
        after
            .resolved_display_view_color_space_name(c"old-screen", c"old-view")
            .unwrap()
            .as_deref(),
        Some("Screen")
    );
    assert_eq!(
        after.display_description(c"old-screen").unwrap().as_deref(),
        Some("Reference monitor")
    );
    assert!(!before.use_display_aliases().unwrap());
    assert_ne!(
        before
            .canonical_display_name(c"old-screen")
            .unwrap()
            .as_deref(),
        Some("Screen")
    );
    assert_eq!(
        config
            .display_description(c"old-screen")
            .unwrap()
            .as_deref(),
        Some("Reference monitor")
    );
    assert_eq!(
        config
            .canonical_display_name(c"old-screen")
            .unwrap()
            .as_deref(),
        Some("Screen")
    );
    assert_eq!(
        config
            .canonical_view_name(c"old-screen", c"old-view")
            .unwrap()
            .as_deref(),
        Some("Standard")
    );
    assert_eq!(
        config
            .resolved_display_view_color_space_name(c"old-screen", c"old-view")
            .unwrap()
            .as_deref(),
        Some("Screen")
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6")]
fn display_and_view_aliases_produce_the_same_cpu_result() {
    use ocio_rs::transform::DisplayViewTransform;
    use ocio_rs::{ViewTransform, ViewTransformDirection};

    let config = Config::raw().unwrap().create_editable_copy().unwrap();
    config.set_version(2, 6).unwrap();
    let source = ColorSpace::create().unwrap();
    source.set_name("Input").unwrap();
    config.try_add_color_space(&source).unwrap();
    let display = ColorSpace::create_with_reference_space(ReferenceSpaceType::Display).unwrap();
    display.set_name("Screen").unwrap();
    display.add_alias("old-screen").unwrap();
    config.try_add_color_space(&display).unwrap();
    let view_transform = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
    view_transform.set_name("ViewTransform").unwrap();
    let scale = MatrixTransform::scale(&[2.0, 1.0, 0.5, 1.0]).unwrap();
    view_transform.set_transform(Some(&scale), ViewTransformDirection::FromReference);
    config.add_view_transform(&view_transform);
    config
        .add_display_view_with_aliases(
            c"Screen",
            ViewWithAliases {
                view: c"Standard",
                view_transform: c"ViewTransform",
                color_space: c"Screen",
                looks: c"",
                rule: c"",
                description: c"",
                aliases: &[c"old-view"],
            },
        )
        .unwrap();
    let make_transform = |display: &str, view: &str| {
        let transform = DisplayViewTransform::create().unwrap();
        transform.set_src("Input").unwrap();
        transform.set_display(display).unwrap();
        transform.set_view(view).unwrap();
        transform
    };
    let canonical = make_transform("Screen", "Standard");
    let alias = make_transform("old-screen", "old-view");
    assert!(config
        .processor_from_transform(&alias, TransformDirection::Forward)
        .is_err());
    config.set_use_display_aliases(true).unwrap();
    let process = |transform: &DisplayViewTransform| {
        let mut pixel = [0.2, 0.4, 0.6, 1.0];
        config
            .processor_from_transform(transform, TransformDirection::Forward)
            .unwrap()
            .default_cpu_processor()
            .unwrap()
            .try_apply_rgba(&mut pixel)
            .unwrap();
        pixel
    };
    let expected = process(&canonical);
    let actual = process(&alias);
    for (left, right) in actual.into_iter().zip(expected) {
        assert!((left - right).abs() < 1e-6);
    }
    assert!((expected[0] - 0.4).abs() < 1e-6);
    assert!((expected[2] - 0.3).abs() < 1e-6);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6")]
fn color_interop_lookup_and_local_id_round_trip() {
    let config = Config::raw().unwrap().create_editable_copy().unwrap();
    config.set_name("test_config").unwrap();
    let space = ColorSpace::create().unwrap();
    space.set_name("test_space").unwrap();
    space.add_alias("test_alias").unwrap();
    config.try_add_color_space(&space).unwrap();
    assert_eq!(
        config
            .find_color_space_for_id(c"vendor:test_alias")
            .unwrap()
            .unwrap()
            .name()
            .as_deref(),
        Some("test_space")
    );
    assert!(config
        .find_color_space_for_id(c"missing")
        .unwrap()
        .is_none());
    assert!(config
        .find_const_color_space_for_id(c"missing")
        .unwrap()
        .is_none());
    assert!(config.find_const_color_space_for_id(c"").unwrap().is_none());
    let shared = config
        .find_const_color_space_for_id(c"vendor:test_alias")
        .unwrap()
        .unwrap();
    assert_eq!(shared.name().unwrap(), c"test_space");
    let copy = shared.create_editable_copy().unwrap();
    copy.set_name("independent").unwrap();
    assert_eq!(shared.name().unwrap(), c"test_space");
    assert!(config
        .generate_local_id_for_color_space(c"missing")
        .is_err());
    let id = std::ffi::CString::new(
        config
            .generate_local_id_for_color_space(c"test_space")
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        config
            .find_color_space_for_id(&id)
            .unwrap()
            .unwrap()
            .name()
            .as_deref(),
        Some("test_space")
    );
    drop(config);
    assert_eq!(shared.name().unwrap(), c"test_space");
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6")]
fn interop_id_lookup_uses_names_and_aliases_not_attributes() {
    let config = Config::raw().unwrap().create_editable_copy().unwrap();
    config.set_name("Studio Name").unwrap();
    let first = ColorSpace::create().unwrap();
    first.set_name("First Space").unwrap();
    first.set_interop_id("vendor:shared").unwrap();
    config.try_add_color_space(&first).unwrap();
    let second = ColorSpace::create().unwrap();
    second.set_name("Second Space").unwrap();
    second.set_interop_id("vendor:shared").unwrap();
    config.try_add_color_space(&second).unwrap();
    assert!(config
        .find_const_color_space_for_id(c"vendor:shared")
        .unwrap()
        .is_none());

    second.add_alias("vendor:shared").unwrap();
    config.try_add_color_space(&second).unwrap();
    let match_space = config
        .find_const_color_space_for_id(c"vendor:shared")
        .unwrap()
        .unwrap();
    assert_eq!(match_space.name().unwrap(), c"Second Space");
    let local_id = config
        .generate_local_id_for_color_space(c"First Space")
        .unwrap()
        .unwrap();
    let local_cstr = std::ffi::CString::new(local_id.as_str()).unwrap();
    assert_eq!(
        config
            .find_const_color_space_for_id(&local_cstr)
            .unwrap()
            .unwrap()
            .name()
            .unwrap(),
        c"First Space"
    );
    let (_, local_part) = local_id.split_once(':').unwrap();
    let wrong_namespace = std::ffi::CString::new(format!("other:{local_part}")).unwrap();
    assert!(config
        .find_const_color_space_for_id(&wrong_namespace)
        .unwrap()
        .is_none());

    let baker = ocio_rs::Baker::create().unwrap();
    baker.set_config(&config).unwrap();
    let shared = baker.const_config().unwrap().unwrap();
    assert_eq!(
        shared
            .find_const_color_space_for_id(c"vendor:shared")
            .unwrap()
            .unwrap()
            .name()
            .unwrap(),
        c"Second Space"
    );
    assert_eq!(
        shared
            .find_const_color_space_for_id(&local_cstr)
            .unwrap()
            .unwrap()
            .name()
            .unwrap(),
        c"First Space"
    );
    assert!(shared
        .find_const_color_space_for_id(&wrong_namespace)
        .unwrap()
        .is_none());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6")]
fn hmj_and_jmh_agree_under_documented_coordinate_conversion() {
    let params = [0.7347, 0.2653, 0.0, 1.0, 0.0001, -0.0770, 0.32168, 0.33767];
    let config = Config::raw().unwrap();
    let apply = |style| {
        let transform = FixedFunctionTransform::create_with_params(style, &params).unwrap();
        assert_eq!(transform.style(), style);
        assert_eq!(transform.try_style().unwrap(), style);
        let mut pixel = [0.2, 0.4, 0.6, 1.0];
        config
            .processor_from_transform(&transform, TransformDirection::Forward)
            .unwrap()
            .default_cpu_processor()
            .unwrap()
            .try_apply_rgba(&mut pixel)
            .unwrap();
        pixel
    };
    let jmh = apply(FixedFunctionStyle::AcesRgbToJmh20);
    let hmj = apply(FixedFunctionStyle::AcesRgbToHmj20);
    for (actual, expected) in
        hmj.into_iter()
            .zip([jmh[2] / 360.0, jmh[1] / 200.0, jmh[0] / 100.0, 1.0])
    {
        assert!((actual - expected).abs() < 1e-5, "{actual} != {expected}");
    }
    assert!(!metadata_id_element().unwrap().is_empty());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6")]
fn hmj_generates_a_native_gpu_shader() {
    let transform = FixedFunctionTransform::create_with_params(
        FixedFunctionStyle::AcesRgbToHmj20,
        &[0.7347, 0.2653, 0.0, 1.0, 0.0001, -0.0770, 0.32168, 0.33767],
    )
    .unwrap();
    let gpu = Config::raw()
        .unwrap()
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let mut desc = GpuShaderDesc::create().unwrap();
    desc.set_language(GpuLanguage::Glsl4_0).unwrap();
    desc.set_function_name("ocio_hmj_test").unwrap();
    gpu.try_extract_shader_info(&mut desc).unwrap();
    let shader = desc.shader_text().unwrap();
    assert!(shader.contains("ocio_hmj_test"));
    assert!(shader.contains("atan"), "{shader}");
}
