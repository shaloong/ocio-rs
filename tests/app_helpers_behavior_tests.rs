use ocio_rs::{ColorSpace, ColorSpaceMenuHelper, ColorSpaceMenuParameters, Config};

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn system_monitors_checks_bounds_and_keeps_snapshot() {
    let monitors = ocio_rs::SystemMonitors::get().unwrap();
    let count = monitors.num_monitors();
    assert!(monitors.monitor_name(count).is_err());
    assert!(monitors.profile_filepath(count).is_err());
    assert!(monitors.monitor_name(usize::MAX).is_err());
    assert!(monitors.profile_filepath(usize::MAX).is_err());
    if count > 0 {
        assert!(!monitors.monitor_name(0).unwrap().is_empty());
        let _ = monitors.profile_filepath(0).unwrap();
    }
    #[cfg(target_os = "linux")]
    assert!(!monitors.is_supported());
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn ociom_controller_round_trip_and_execute() {
    use ocio_rs::{ConfigMerger, ConfigMergingParameters};
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/ociom");
    let dir = std::ffi::CString::new(fixture.to_str().unwrap()).unwrap();
    let mut params = ConfigMergingParameters::create().unwrap();
    params.set_base_config_name(c"base.ocio").unwrap();
    params.set_input_config_name(c"input.ocio").unwrap();
    let params_text = params.serialize().unwrap();
    assert!(params_text.contains("base.ocio"));
    assert!(params_text.contains("input.ocio"));
    params.set_adjust_input_reference_space(false).unwrap();
    let mut merger = ConfigMerger::create().unwrap();
    merger.set_working_dir(&dir).unwrap();
    merger.set_search_path(c".").unwrap();
    merger.add_params(&params).unwrap();
    assert_eq!(merger.num_params().unwrap(), 1);
    assert!(merger.params(1).unwrap().is_none());
    assert_eq!(
        merger
            .params(0)
            .unwrap()
            .unwrap()
            .base_config_name()
            .unwrap(),
        c"base.ocio"
    );
    let serialized = merger.serialize().unwrap();
    assert!(serialized.contains("base.ocio"));
    let temp = std::env::temp_dir().join(format!("ocio-rs-ociom-{}.ociom", std::process::id()));
    std::fs::write(&temp, &serialized).unwrap();
    let path = std::ffi::CString::new(temp.to_str().unwrap()).unwrap();
    let parsed = ConfigMerger::from_file(&path).unwrap();
    std::fs::remove_file(&temp).unwrap();
    assert_eq!(parsed.num_params().unwrap(), 1);
    assert_eq!(
        parsed
            .params(0)
            .unwrap()
            .unwrap()
            .input_config_name()
            .unwrap(),
        c"input.ocio"
    );
    assert!(merger.merged_config(-1).unwrap().is_none());
    let output = merger.merge_configs().unwrap();
    assert_eq!(output.num_merged_configs().unwrap(), 1);
    let merged = output.merged_config(-1).unwrap().unwrap();
    let retained = output.merged_const_config(-1).unwrap().unwrap();
    assert!(merged.color_space("extra").is_some());
    assert!(output.merged_config(2).unwrap().is_none());
    assert!(output.merged_config(-2).unwrap().is_none());
    assert!(output.merged_config(i32::MIN).unwrap().is_none());
    assert!(output.merged_const_config(-2).unwrap().is_none());
    assert!(output.merged_const_config(1).unwrap().is_none());
    assert_eq!(
        retained.cache_id().unwrap(),
        output
            .merged_const_config(0)
            .unwrap()
            .unwrap()
            .cache_id()
            .unwrap()
    );
    drop(output);
    assert!(merged.color_space("extra").is_some());
    assert!(retained.try_const_color_space(c"extra").unwrap().is_some());
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn ociom_raw_shared_parameters_update_the_merger() {
    unsafe {
        let merger = ocio_sys::ocio_config_merger_create();
        let original = ocio_sys::ocio_config_merging_parameters_create();
        assert!(!merger.is_null() && !original.is_null());
        ocio_sys::ocio_config_merger_add_params(merger, original);
        assert!(ocio_sys::ocio_config_merger_params_shared(merger, -1).is_null());
        let shared = ocio_sys::ocio_config_merger_params_shared(merger, 0);
        assert!(!shared.is_null());
        ocio_sys::ocio_config_merging_parameters_set_name(shared, c"shared edit".as_ptr());
        let copied = ocio_sys::ocio_config_merger_params(merger, 0);
        assert!(!copied.is_null());
        ocio_sys::ocio_config_merging_parameters_destroy(original);
        ocio_sys::ocio_config_merger_destroy(merger);
        for handle in [shared, copied] {
            let name = ocio_sys::ocio_config_merging_parameters_get_name(handle);
            assert!(!name.is_null());
            assert_eq!(std::ffi::CStr::from_ptr(name), c"shared edit");
            ocio_sys::ocio_config_merging_parameters_destroy(handle);
        }
    }
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn config_merging_preserves_inputs_and_applies_native_conflict_policy() {
    use ocio_rs::{ConfigMergingParameters, MergeStrategy};
    let base = Config::raw().unwrap().create_editable_copy().unwrap();
    let input = base.create_editable_copy().unwrap();
    let space = ColorSpace::create().unwrap();
    space.set_name("new-space").unwrap();
    input.try_add_color_space(&space).unwrap();
    let mut options = ConfigMergingParameters::create().unwrap();
    options.set_name(c"merged").unwrap();
    options.set_adjust_input_reference_space(false).unwrap();
    options
        .set_default_strategy(MergeStrategy::PreferBase)
        .unwrap();
    options
        .set_color_spaces(MergeStrategy::PreferInput)
        .unwrap();
    options
        .add_environment_var(c"MERGE_TEST", c"default")
        .unwrap();
    let copy = options.create_editable_copy().unwrap();
    assert_eq!(copy.color_spaces().unwrap(), MergeStrategy::PreferInput);
    assert_eq!(copy.environment_var(0).unwrap(), c"MERGE_TEST");
    assert_eq!(copy.environment_var_value(0).unwrap(), c"default");
    let merged = copy.merge_configs(&base, &input).unwrap();
    assert_eq!(merged.name().as_deref(), Some("merged"));
    assert!(merged.color_space("new-space").is_some());
    let merged = copy.merge_color_space(&base, &space).unwrap();
    assert!(merged.color_space("new-space").is_some());
    assert!(base.color_space("new-space").is_none());
    assert_ne!(input.name().as_deref(), Some("merged"));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn file_backed_display_helpers_author_process_and_remove() {
    use ocio_rs::{Context, DisplayViewFromFile, DynamicPropertyType, TransformDirection};
    let mut config = Config::from_stream(
        "ocio_profile_version: 2\nroles: {default: linear, scene_linear: linear}\n\
         displays: {Monitor: [!<View> {name: Raw, colorspace: linear}]}\n\
         colorspaces: [!<ColorSpace> {name: linear, bitdepth: 32f, isdata: false}]\n",
    )
    .unwrap();
    let file = std::ffi::CString::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/clf/matrix_3x4_example.clf"
    ))
    .unwrap();
    assert!(config
        .add_color_space_from_file(c"Invalid", &file, c"", c"missing")
        .is_err());
    config
        .add_color_space_from_file(c"File Input", &file, c"", c"linear")
        .unwrap();
    assert!(config.color_space("File Input").is_some());
    config
        .add_display_view_from_file(DisplayViewFromFile {
            display: c"Extra",
            view: c"File View",
            looks: c"",
            color_space: c"File Display",
            family: c"Output",
            description: c"Test output",
            categories: c"",
            file: &file,
            connection_space: c"linear",
        })
        .unwrap();
    assert!(config.color_space("File Display").is_some());
    let context = Context::create().unwrap();
    let mut output = [[0.25, 0.5, 0.75, 1.0]; 2];
    for (rgba, ctx) in output.iter_mut().zip([None, Some(&context)]) {
        let cpu = config
            .application_display_processor(
                ctx,
                c"linear",
                c"Extra",
                c"File View",
                None,
                TransformDirection::Forward,
            )
            .unwrap()
            .default_cpu_processor()
            .unwrap();
        assert!(cpu.has_dynamic_property_kind(DynamicPropertyType::Exposure));
        cpu.try_apply_rgba(rgba).unwrap();
    }
    assert_eq!(output[0], output[1]);
    assert_ne!(output[0], [0.25, 0.5, 0.75, 1.0]);
    config
        .remove_display_view_and_unused_color_space(c"Extra", c"File View")
        .unwrap();
    assert!(config.color_space("File Display").is_none());
    assert!(config.color_space("File Input").is_some());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn identity_viewing_helper_preserves_pixels_with_dynamic_adjustments() {
    let cpu = Config::raw()
        .unwrap()
        .display_view_helper_identity_processor()
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    assert!(cpu.has_dynamic_property_kind(ocio_rs::DynamicPropertyType::Exposure));
    assert!(cpu.has_dynamic_property_kind(ocio_rs::DynamicPropertyType::Contrast));
    assert!(cpu.has_dynamic_property_kind(ocio_rs::DynamicPropertyType::Gamma));
    let mut rgba = [0.25, 0.5, 0.75, 0.5];
    cpu.try_apply_rgba(&mut rgba).unwrap();
    assert_eq!(rgba, [0.25, 0.5, 0.75, 0.5]);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn legacy_viewing_pipeline_copies_stages_and_processes_pixels() {
    use ocio_rs::transform::{
        DisplayViewTransform, ExposureContrastTransform, MatrixTransform, Transform,
    };
    use ocio_rs::LegacyViewingStage;
    use ocio_rs::{Context, LegacyViewingPipeline};
    let config = Config::from_stream(
        "ocio_profile_version: 2\nroles: {default: linear, scene_linear: linear}\n\
         displays: {Monitor: [!<View> {name: Raw, colorspace: linear}]}\n\
         colorspaces: [!<ColorSpace> {name: linear, bitdepth: 32f, isdata: false}]\n",
    )
    .unwrap();
    let mut pipeline = LegacyViewingPipeline::create().unwrap();
    assert!(pipeline.display_view_transform().unwrap().is_none());
    assert!(pipeline.processor(&config).is_err());
    let display = DisplayViewTransform::create().unwrap();
    display.set_src("linear").unwrap();
    display.set_display("Monitor").unwrap();
    display.set_view("Raw").unwrap();
    pipeline.set_display_view_transform(Some(&display));
    let retained_display = pipeline
        .const_stage(LegacyViewingStage::DisplayView)
        .unwrap()
        .unwrap();
    assert!(pipeline
        .serialize()
        .unwrap()
        .contains("DisplayViewTransform"));
    display.set_display("changed").unwrap();
    assert_eq!(
        pipeline
            .display_view_transform()
            .unwrap()
            .unwrap()
            .display()
            .as_deref(),
        Some("Monitor")
    );
    let stage = Transform::Matrix(MatrixTransform::scale(&[2.0, 1.0, 0.5, 1.0]).unwrap());
    pipeline.set_display_cc(Some(&stage));
    pipeline.set_linear_cc(Some(&stage));
    pipeline.set_color_timing_cc(Some(&stage));
    pipeline.set_channel_view(Some(&stage));
    let retained_correction = pipeline
        .const_stage(LegacyViewingStage::DisplayCorrection)
        .unwrap()
        .unwrap();
    assert!(pipeline
        .set_const_stage(LegacyViewingStage::DisplayView, Some(&retained_correction))
        .is_err());
    assert!(pipeline
        .const_stage(LegacyViewingStage::DisplayView)
        .unwrap()
        .is_some());
    drop(stage);
    assert!(matches!(
        pipeline.display_cc().unwrap(),
        Some(Transform::Matrix(_))
    ));
    assert!(matches!(
        pipeline.linear_cc().unwrap(),
        Some(Transform::Matrix(_))
    ));
    assert!(matches!(
        pipeline.color_timing_cc().unwrap(),
        Some(Transform::Matrix(_))
    ));
    assert!(matches!(
        pipeline.channel_view().unwrap(),
        Some(Transform::Matrix(_))
    ));
    pipeline.set_linear_cc(None);
    pipeline.set_color_timing_cc(None);
    pipeline.set_channel_view(None);
    assert!(pipeline.linear_cc().unwrap().is_none());
    assert!(pipeline.color_timing_cc().unwrap().is_none());
    assert!(pipeline.channel_view().unwrap().is_none());
    pipeline.set_looks_override(c"").unwrap();
    pipeline.set_looks_override_enabled(true).unwrap();
    assert!(pipeline.looks_override_enabled().unwrap());
    assert_eq!(pipeline.looks_override().unwrap(), c"");
    let const_config = ocio_rs::ColorSpaceMenuParameters::create(&config)
        .unwrap()
        .const_config()
        .unwrap();
    let const_context = const_config.try_const_current_context().unwrap().unwrap();
    for processor in [
        pipeline.processor(&config).unwrap(),
        pipeline
            .processor_with_context(&config, &Context::create().unwrap())
            .unwrap(),
        pipeline
            .processor_with_const_config(&const_config, None)
            .unwrap(),
        pipeline
            .processor_with_const_config(&const_config, Some(&const_context))
            .unwrap(),
    ] {
        let mut rgba = [0.25, 0.5, 0.75, 1.0];
        processor
            .default_cpu_processor()
            .unwrap()
            .try_apply_rgba(&mut rgba)
            .unwrap();
        assert_eq!(rgba, [0.5, 0.5, 0.375, 1.0]);
    }
    drop(pipeline);

    let mut pipeline = LegacyViewingPipeline::create().unwrap();
    pipeline
        .set_const_stage(LegacyViewingStage::DisplayView, Some(&retained_display))
        .unwrap();
    pipeline
        .set_const_stage(
            LegacyViewingStage::DisplayCorrection,
            Some(&retained_correction),
        )
        .unwrap();
    assert!(pipeline
        .const_stage(LegacyViewingStage::LinearCorrection)
        .unwrap()
        .is_none());
    let mut rgba = [0.25, 0.5, 0.75, 1.0];
    pipeline
        .processor(&config)
        .unwrap()
        .default_cpu_processor()
        .unwrap()
        .try_apply_rgba(&mut rgba)
        .unwrap();
    assert_eq!(rgba, [0.5, 0.5, 0.375, 1.0]);
    pipeline
        .set_const_stage(LegacyViewingStage::DisplayCorrection, None)
        .unwrap();
    assert!(pipeline
        .const_stage(LegacyViewingStage::DisplayCorrection)
        .unwrap()
        .is_none());
    retained_correction.validate().unwrap();

    let exposure = ExposureContrastTransform::create().unwrap();
    exposure.try_set_exposure(0.5).unwrap();
    exposure.try_make_exposure_dynamic().unwrap();
    pipeline.set_linear_cc(Some(&Transform::ExposureContrast(exposure)));
    let first = pipeline
        .processor_with_const_config(&const_config, None)
        .unwrap();
    let second = pipeline
        .processor_with_const_config(&const_config, Some(&const_context))
        .unwrap();
    let kind = ocio_rs::DynamicPropertyType::Exposure;
    first
        .dynamic_property(kind)
        .unwrap()
        .set_double_value(2.0)
        .unwrap();
    assert_eq!(
        second
            .dynamic_property(kind)
            .unwrap()
            .double_value()
            .unwrap(),
        0.5
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn mixing_manager_retains_config_and_slider_round_trips() {
    let config = Config::raw().unwrap();
    let mut manager = ocio_rs::MixingColorSpaceManager::create(&config).unwrap();
    manager.refresh(&config).unwrap();
    drop(config);
    let manager_text = manager.serialize().unwrap();
    assert!(manager_text.contains("slider"));
    assert!(manager.num_mixing_spaces() > 0);
    assert!(manager.num_mixing_encodings() > 0);
    assert!(manager.set_selected_mixing_space_idx(usize::MAX).is_err());
    assert!(manager
        .set_selected_mixing_encoding_idx(usize::MAX)
        .is_err());
    let space = manager.mixing_space_ui_name(0).unwrap().to_owned();
    manager.set_selected_mixing_space(&space).unwrap();
    assert_eq!(manager.selected_mixing_space_idx(), 0);
    let encoding = manager.mixing_encoding_name(0).unwrap().to_owned();
    manager.set_selected_mixing_encoding(&encoding).unwrap();
    assert_eq!(manager.selected_mixing_encoding_idx(), 0);
    manager.set_selected_mixing_encoding_idx(1).unwrap();
    let cpu = manager
        .processor(c"raw", c"", c"", ocio_rs::TransformDirection::Forward)
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    let mut red = [1.0, 0.0, 0.0, 0.5];
    cpu.try_apply_rgba(&mut red).unwrap();
    assert_eq!(red, [0.0, 1.0, 1.0, 0.5]);
    manager.set_selected_mixing_encoding_idx(0).unwrap();

    let slider = manager.slider_with_edges(0.1, 0.9);
    let slider_text = slider.serialize().unwrap();
    assert!(slider_text.contains("minEdge"));
    assert!(slider_text.contains("maxEdge"));
    assert!(manager_text.contains("slider"));
    // Native getters report perceptual edges for non-uniform spaces.
    assert!((slider.slider_to_mixing(0.0) - 0.1).abs() < 1e-5);
    assert!((slider.slider_to_mixing(1.0) - 0.9).abs() < 1e-5);
    let value = 0.35;
    assert!((slider.mixing_to_slider(slider.slider_to_mixing(value)) - value).abs() < 1e-5);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn native_menu_filters_and_borrows_immutable_entries() {
    let config = Config::raw().unwrap().create_editable_copy().unwrap();
    let space = ColorSpace::create().unwrap();
    space.set_name("Menu Input").unwrap();
    space.set_family("Input/Camera").unwrap();
    space.set_description("Camera input space").unwrap();
    space.add_category("ocio_rs_camera_test").unwrap();
    config.try_add_color_space(&space).unwrap();

    let mut parameters = ColorSpaceMenuParameters::create(&config).unwrap();
    parameters.set_app_categories(c"ocio_rs_camera_test");
    let parameters_text = parameters.serialize().unwrap();
    assert!(parameters_text.contains("ocio_rs_camera_test"));
    assert_eq!(parameters.app_categories(), c"ocio_rs_camera_test");
    #[cfg(feature = "v2_5")]
    {
        parameters.set_treat_no_category_as_any(false);
        assert!(!parameters.treat_no_category_as_any());
    }
    parameters.add_color_space(c"raw");
    assert_eq!(parameters.num_added_color_spaces(), 1);
    assert_eq!(parameters.added_color_space(0), c"raw");
    let menu = ColorSpaceMenuHelper::create(&parameters).unwrap();
    let menu_text = menu.serialize().unwrap();
    assert!(menu_text.contains("Menu Input"));
    assert!(parameters_text.contains("ocio_rs_camera_test"));
    parameters.clear_added_color_spaces();
    drop(parameters);
    drop(config);

    let index = menu.index_from_name(c"Menu Input").unwrap();
    assert_eq!(menu.name(index), c"Menu Input");
    assert_eq!(menu.description(index), c"Camera input space");
    assert_eq!(menu.family(index), c"Input/Camera");
    assert_eq!(menu.num_hierarchy_levels(index), 2);
    assert_eq!(menu.hierarchy_level(index, 1), c"Camera");
    assert_eq!(menu.index_from_ui_name(menu.ui_name(index)), Some(index));
    assert_eq!(
        menu.name_from_ui_name(menu.ui_name(index)),
        menu.name(index)
    );
    assert_eq!(
        menu.ui_name_from_name(menu.name(index)),
        menu.ui_name(index)
    );
    assert!(menu.index_from_name(c"raw").is_some());
    assert_eq!(menu.index_from_name(c"absent"), None);
    assert_eq!(menu.name(menu.num_color_spaces()), c"");
    assert_eq!(menu.name(index).as_ptr(), menu.name(index).as_ptr());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn menu_configuration_is_an_editable_copy_and_invalid_entries_error() {
    let config = Config::raw().unwrap().create_editable_copy().unwrap();
    config.set_name("menu source").unwrap();
    let mut parameters = ColorSpaceMenuParameters::create(&config).unwrap();
    let copy = parameters.config().unwrap();
    let retained = parameters.const_config().unwrap();
    copy.set_name("changed copy").unwrap();
    assert_eq!(config.name().as_deref(), Some("menu source"));
    config.set_name("same native config").unwrap();
    assert_eq!(
        retained.name().unwrap().as_deref(),
        Some("same native config")
    );
    parameters.add_color_space(c"nonexistent-menu-space");
    assert!(ColorSpaceMenuHelper::create(&parameters).is_err());
    drop(parameters);
    drop(config);
    assert_eq!(
        retained.name().unwrap().as_deref(),
        Some("same native config")
    );
}
