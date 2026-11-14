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
    config.set_use_display_aliases(true).unwrap();
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
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6")]
fn hmj_and_jmh_agree_under_documented_coordinate_conversion() {
    let params = [0.7347, 0.2653, 0.0, 1.0, 0.0001, -0.0770, 0.32168, 0.33767];
    let config = Config::raw().unwrap();
    let apply = |style| {
        let transform = FixedFunctionTransform::create_with_params(style, &params).unwrap();
        assert_eq!(transform.style(), style);
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
