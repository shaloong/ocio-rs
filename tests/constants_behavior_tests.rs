use ocio_rs::constants::*;

#[test]
fn standard_names_are_available_in_stub_builds() {
    assert_eq!(OCIO_CONFIG_ENVVAR, "OCIO");
    assert_eq!(ROLE_INTERCHANGE_SCENE, "aces_interchange");
    assert_eq!(ROLE_INTERCHANGE_DISPLAY, "cie_xyz_d65_interchange");
    assert_eq!(OCIO_VIEW_USE_DISPLAY_NAME, "<USE_DISPLAY_NAME>");
    assert_eq!(METADATA_ID, "id");
    assert_eq!(
        OCIO_CONFIG_DEFAULT_NAME.to_owned() + OCIO_CONFIG_DEFAULT_FILE_EXT,
        "config.ocio"
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn standard_role_name_works_with_native_config() {
    let config = ocio_rs::Config::raw().expect("raw config");
    config.set_role(ROLE_SCENE_LINEAR, "raw").expect("set role");
    assert_eq!(
        config.role_color_space(ROLE_SCENE_LINEAR).as_deref(),
        Some("raw")
    );
}

#[test]
#[cfg(feature = "v2_6")]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn id_element_matches_native_constant() {
    let native = ocio_rs::metadata_id_element().expect("native metadata constant");
    assert_eq!(native.to_str().unwrap(), METADATA_ID_ELEMENT);
}
