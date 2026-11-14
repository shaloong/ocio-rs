use ocio_rs::{ColorSpace, ColorSpaceMenuHelper, ColorSpaceMenuParameters, Config};

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn mixing_manager_retains_config_and_slider_round_trips() {
    let config = Config::raw().unwrap();
    let mut manager = ocio_rs::MixingColorSpaceManager::create(&config).unwrap();
    manager.refresh(&config).unwrap();
    drop(config);
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

    let slider = manager.slider_with_edges(0.1, 0.9);
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
    copy.set_name("changed copy").unwrap();
    assert_eq!(config.name().as_deref(), Some("menu source"));
    parameters.add_color_space(c"nonexistent-menu-space");
    assert!(ColorSpaceMenuHelper::create(&parameters).is_err());
}
