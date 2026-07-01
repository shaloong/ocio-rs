#![allow(
    non_snake_case,
    unused_variables,
    clippy::missing_safety_doc,
    clippy::unused_unit
)]

use std::ffi::c_void;

#[repr(C)]
pub struct OcioGpuTexture2DInfo {
    pub texture_name: *const i8,
    pub sampler_name: *const i8,
    pub width: u32,
    pub height: u32,
    pub channel: i32,
    pub dimensions: i32,
    pub interpolation: i32,
    pub binding_index: u32,
}

#[repr(C)]
pub struct OcioGpuTexture3DInfo {
    pub texture_name: *const i8,
    pub sampler_name: *const i8,
    pub edge_len: u32,
    pub interpolation: i32,
    pub binding_index: u32,
}

#[repr(C)]
pub struct OcioGpuUniformInfo {
    pub name: *const i8,
    pub type_: i32,
    pub buffer_offset: usize,
    pub value_count: usize,
}

unsafe extern "C" {
    // --- Runtime ---
    pub fn ocio_runtime_is_stub() -> bool;

    // --- Global utility functions ---
    pub fn ocio_get_version() -> *const i8;
    pub fn ocio_get_version_hex() -> i32;
    pub fn ocio_get_logging_level() -> i32;
    pub fn ocio_set_logging_level(level: i32);

    // --- Global config ---
    pub fn ocio_get_current_config() -> *mut c_void;
    pub fn ocio_set_current_config(config: *mut c_void);
    pub fn ocio_clear_all_caches();

    // --- BuiltinConfigRegistry ---
    pub fn ocio_builtin_config_registry_get() -> *mut c_void;
    pub fn ocio_builtin_config_registry_get_num_builtin_configs(handle: *mut c_void) -> usize;
    pub fn ocio_builtin_config_registry_get_builtin_config_name(
        handle: *mut c_void,
        configIndex: usize,
    ) -> *mut c_void;
    pub fn ocio_builtin_config_registry_get_builtin_config_ui_name(
        handle: *mut c_void,
        configIndex: usize,
    ) -> *mut c_void;
    pub fn ocio_builtin_config_registry_get_builtin_config(
        handle: *mut c_void,
        configIndex: usize,
    ) -> *mut c_void;
    pub fn ocio_builtin_config_registry_get_builtin_config_by_name(
        handle: *mut c_void,
        configName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_builtin_config_registry_is_builtin_config_recommended(
        handle: *mut c_void,
        configIndex: usize,
    ) -> bool;
    pub fn ocio_builtin_transform_registry_get() -> *mut c_void;
    pub fn ocio_builtin_transform_registry_destroy(handle: *mut c_void);
    pub fn ocio_builtin_transform_registry_get_num_builtins(handle: *mut c_void) -> usize;
    pub fn ocio_builtin_transform_registry_get_builtin_style(
        handle: *mut c_void,
        index: usize,
    ) -> *const i8;
    pub fn ocio_builtin_transform_registry_get_builtin_description(
        handle: *mut c_void,
        index: usize,
    ) -> *const i8;
    pub fn ocio_config_io_proxy_create() -> *mut c_void;
    pub fn ocio_config_io_proxy_destroy(handle: *mut c_void);
    pub fn ocio_config_io_proxy_set_config_data(handle: *mut c_void, data: *const i8) -> ();
    pub fn ocio_config_io_proxy_get_config_data(handle: *mut c_void) -> *const i8;
    pub fn ocio_config_io_proxy_set_lut_data(
        handle: *mut c_void,
        filepath: *const i8,
        data: *const u8,
        len: usize,
        fastHash: *const i8,
    ) -> bool;
    pub fn ocio_config_io_proxy_get_lut_data_size(
        handle: *mut c_void,
        filepath: *const i8,
    ) -> usize;
    pub fn ocio_config_io_proxy_copy_lut_data(
        handle: *mut c_void,
        filepath: *const i8,
        data: *mut u8,
        len: usize,
    ) -> bool;
    pub fn ocio_config_io_proxy_get_fast_lut_file_hash(
        handle: *mut c_void,
        filepath: *const i8,
    ) -> *const i8;

    // --- Config ---
    pub fn ocio_config_raw() -> *mut c_void;
    pub fn ocio_config_from_file(path: *const i8) -> *mut c_void;
    pub fn ocio_config_create_raw() -> *mut c_void;
    pub fn ocio_config_create_from_file(path: *const i8) -> *mut c_void;
    pub fn ocio_config_create_from_builtin_config(configName: *const i8) -> *mut c_void;
    pub fn ocio_config_create_from_env() -> *mut c_void;
    pub fn ocio_config_create_from_stream(text: *const i8) -> *mut c_void;
    pub fn ocio_config_create_from_config_io_proxy(ciop: *mut c_void) -> *mut c_void;
    pub fn ocio_config_destroy(handle: *mut c_void);
    pub fn ocio_config_get_major_version(handle: *mut c_void) -> i32;
    pub fn ocio_config_set_major_version(handle: *mut c_void, major: u32) -> ();
    pub fn ocio_config_get_minor_version(handle: *mut c_void) -> i32;
    pub fn ocio_config_set_minor_version(handle: *mut c_void, minor: u32) -> ();
    pub fn ocio_config_set_version(handle: *mut c_void, major: u32, minor: u32) -> ();
    pub fn ocio_config_upgrade_to_latest_version(handle: *mut c_void) -> ();
    pub fn ocio_config_validate(handle: *mut c_void) -> ();
    pub fn ocio_config_get_name(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_set_name(handle: *mut c_void, name: *const i8) -> ();
    pub fn ocio_config_get_family_separator(handle: *mut c_void) -> i8;
    pub fn ocio_config_get_default_family_separator(handle: *mut c_void) -> i8;
    pub fn ocio_config_set_family_separator(handle: *mut c_void, separator: i8) -> ();
    pub fn ocio_config_get_description(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_set_description(handle: *mut c_void, description: *const i8) -> ();
    pub fn ocio_config_serialize(handle: *mut c_void, os: *mut c_void) -> ();
    pub fn ocio_config_serialize_to_string(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_get_cache_id(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_get_cache_id_n(handle: *mut c_void, context: *mut c_void) -> *mut c_void;
    pub fn ocio_config_get_current_context(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_add_environment_var(
        handle: *mut c_void,
        name: *const i8,
        defaultValue: *const i8,
    ) -> ();
    pub fn ocio_config_get_num_environment_vars(handle: *mut c_void) -> i32;
    pub fn ocio_config_get_environment_var_name_by_index(
        handle: *mut c_void,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_environment_var_default(
        handle: *mut c_void,
        name: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_clear_environment_vars(handle: *mut c_void) -> ();
    pub fn ocio_config_set_environment_mode(handle: *mut c_void, mode: i32) -> ();
    pub fn ocio_config_get_environment_mode(handle: *mut c_void) -> i32;
    pub fn ocio_config_load_environment(handle: *mut c_void) -> ();
    pub fn ocio_config_get_search_path(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_set_search_path(handle: *mut c_void, path: *const i8) -> ();
    pub fn ocio_config_get_num_search_paths(handle: *mut c_void) -> i32;
    pub fn ocio_config_get_search_path_by_index(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_clear_search_paths(handle: *mut c_void) -> ();
    pub fn ocio_config_add_search_path(handle: *mut c_void, path: *const i8) -> ();
    pub fn ocio_config_get_working_dir(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_set_working_dir(handle: *mut c_void, dirname: *const i8) -> ();
    pub fn ocio_config_get_color_spaces(handle: *mut c_void, category: *const i8) -> *mut c_void;
    pub fn ocio_config_get_num_color_spaces(
        handle: *mut c_void,
        searchReferenceType: i32,
        visibility: i32,
    ) -> i32;
    pub fn ocio_config_get_color_space_name_by_index(
        handle: *mut c_void,
        searchReferenceType: i32,
        visibility: i32,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_num_color_spaces_v1(handle: *mut c_void) -> i32;
    pub fn ocio_config_get_color_space_name_by_index_v1(
        handle: *mut c_void,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_index_for_color_space(handle: *mut c_void, name: *const i8) -> i32;
    pub fn ocio_config_get_color_space(handle: *mut c_void, name: *const i8) -> *mut c_void;
    pub fn ocio_config_get_canonical_name(handle: *mut c_void, name: *const i8) -> *mut c_void;
    pub fn ocio_config_add_color_space(handle: *mut c_void, cs: *mut c_void) -> ();
    pub fn ocio_config_remove_color_space(handle: *mut c_void, name: *const i8) -> ();
    pub fn ocio_config_is_color_space_used(handle: *mut c_void, name: *const i8) -> bool;
    pub fn ocio_config_clear_color_spaces(handle: *mut c_void) -> ();
    pub fn ocio_config_set_inactive_color_spaces(
        handle: *mut c_void,
        inactiveColorSpaces: *const i8,
    ) -> ();
    pub fn ocio_config_get_inactive_color_spaces(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_is_inactive_color_space(handle: *mut c_void, colorspace: *const i8) -> bool;
    pub fn ocio_config_is_color_space_linear(
        handle: *mut c_void,
        colorSpace: *const i8,
        referenceSpaceType: i32,
    ) -> bool;
    pub fn ocio_config_identify_builtin_color_space(
        handle: *mut c_void,
        srcConfig: *mut c_void,
        builtinConfig: *mut c_void,
        builtinColorSpaceName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_identify_interchange_space(
        handle: *mut c_void,
        srcInterchangeName: *mut c_void,
        builtinInterchangeName: *mut c_void,
        srcConfig: *mut c_void,
        srcColorSpaceName: *const i8,
        builtinConfig: *mut c_void,
        builtinColorSpaceName: *const i8,
    ) -> ();
    pub fn ocio_config_set_role(
        handle: *mut c_void,
        role: *const i8,
        colorSpaceName: *const i8,
    ) -> ();
    pub fn ocio_config_get_num_roles(handle: *mut c_void) -> i32;
    pub fn ocio_config_has_role(handle: *mut c_void, role: *const i8) -> bool;
    pub fn ocio_config_get_role_name(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_get_role_color_space(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_get_role_color_space_v1(
        handle: *mut c_void,
        roleName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_role_color_space_by_index(
        handle: *mut c_void,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_role_color_space_by_name(
        handle: *mut c_void,
        roleName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_is_view_shared(
        handle: *mut c_void,
        dispName: *const i8,
        viewName: *const i8,
    ) -> bool;
    pub fn ocio_config_add_shared_view(
        handle: *mut c_void,
        view: *const i8,
        viewTransformName: *const i8,
        colorSpaceName: *const i8,
        looks: *const i8,
        ruleName: *const i8,
        description: *const i8,
    ) -> ();
    pub fn ocio_config_remove_shared_view(handle: *mut c_void, view: *const i8) -> ();
    pub fn ocio_config_clear_shared_views(handle: *mut c_void) -> ();
    pub fn ocio_config_get_default_display(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_get_num_displays(handle: *mut c_void) -> i32;
    pub fn ocio_config_get_display(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_get_default_view(handle: *mut c_void, display: *const i8) -> *mut c_void;
    pub fn ocio_config_get_default_view_v1(
        handle: *mut c_void,
        display: *const i8,
        colorspaceName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_num_views(handle: *mut c_void, display: *const i8) -> i32;
    pub fn ocio_config_get_view(handle: *mut c_void, display: *const i8, index: i32)
        -> *mut c_void;
    pub fn ocio_config_get_num_views_v1(
        handle: *mut c_void,
        display: *const i8,
        colorspaceName: *const i8,
    ) -> i32;
    pub fn ocio_config_get_view_v1(
        handle: *mut c_void,
        display: *const i8,
        colorspaceName: *const i8,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_config_are_views_equal(
        handle: *mut c_void,
        first: *mut c_void,
        second: *mut c_void,
        dispName: *const i8,
        viewName: *const i8,
    ) -> bool;
    pub fn ocio_config_get_display_view_transform_name(
        handle: *mut c_void,
        display: *const i8,
        view: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_display_view_color_space_name(
        handle: *mut c_void,
        display: *const i8,
        view: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_display_view_looks(
        handle: *mut c_void,
        display: *const i8,
        view: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_display_view_rule(
        handle: *mut c_void,
        display: *const i8,
        view: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_display_view_description(
        handle: *mut c_void,
        display: *const i8,
        view: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_has_view(
        handle: *mut c_void,
        dispName: *const i8,
        viewName: *const i8,
    ) -> bool;
    pub fn ocio_config_add_display_view_v1(
        handle: *mut c_void,
        display: *const i8,
        view: *const i8,
        colorSpaceName: *const i8,
        looks: *const i8,
    ) -> ();
    pub fn ocio_config_add_display_view_v2(
        handle: *mut c_void,
        display: *const i8,
        view: *const i8,
        viewTransformName: *const i8,
        colorSpaceName: *const i8,
        looks: *const i8,
        ruleName: *const i8,
        description: *const i8,
    ) -> ();
    pub fn ocio_config_add_display_shared_view(
        handle: *mut c_void,
        display: *const i8,
        sharedView: *const i8,
    ) -> ();
    pub fn ocio_config_remove_display_view(
        handle: *mut c_void,
        display: *const i8,
        view: *const i8,
    ) -> ();
    pub fn ocio_config_clear_displays(handle: *mut c_void) -> ();
    pub fn ocio_config_has_virtual_view(handle: *mut c_void, viewName: *const i8) -> bool;
    pub fn ocio_config_is_virtual_view_shared(handle: *mut c_void, viewName: *const i8) -> bool;
    pub fn ocio_config_add_virtual_display_view(
        handle: *mut c_void,
        view: *const i8,
        viewTransformName: *const i8,
        colorSpaceName: *const i8,
        looks: *const i8,
        ruleName: *const i8,
        description: *const i8,
    ) -> ();
    pub fn ocio_config_add_virtual_display_shared_view(
        handle: *mut c_void,
        sharedView: *const i8,
    ) -> ();
    pub fn ocio_config_get_virtual_display_num_views(handle: *mut c_void, type_param: i32) -> i32;
    pub fn ocio_config_get_virtual_display_view(
        handle: *mut c_void,
        type_param: i32,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_config_are_virtual_views_equal(
        handle: *mut c_void,
        first: *mut c_void,
        second: *mut c_void,
        viewName: *const i8,
    ) -> bool;
    pub fn ocio_config_get_virtual_display_view_transform_name(
        handle: *mut c_void,
        view: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_virtual_display_view_color_space_name(
        handle: *mut c_void,
        view: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_virtual_display_view_looks(
        handle: *mut c_void,
        view: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_virtual_display_view_rule(
        handle: *mut c_void,
        view: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_virtual_display_view_description(
        handle: *mut c_void,
        view: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_remove_virtual_display_view(handle: *mut c_void, view: *const i8) -> ();
    pub fn ocio_config_clear_virtual_display(handle: *mut c_void) -> ();
    pub fn ocio_config_instantiate_display_from_monitor_name(
        handle: *mut c_void,
        monitorName: *const i8,
    ) -> i32;
    pub fn ocio_config_instantiate_display_from_icc_profile(
        handle: *mut c_void,
        ICCProfileFilepath: *const i8,
    ) -> i32;
    pub fn ocio_config_set_active_displays(handle: *mut c_void, displays: *const i8) -> ();
    pub fn ocio_config_get_active_displays(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_get_num_active_displays(handle: *mut c_void) -> i32;
    pub fn ocio_config_get_active_display(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_add_active_display(handle: *mut c_void, display: *const i8) -> ();
    pub fn ocio_config_remove_active_display(handle: *mut c_void, display: *const i8) -> ();
    pub fn ocio_config_clear_active_displays(handle: *mut c_void) -> ();
    pub fn ocio_config_set_active_views(handle: *mut c_void, views: *const i8) -> ();
    pub fn ocio_config_get_active_views(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_get_num_active_views(handle: *mut c_void) -> i32;
    pub fn ocio_config_get_active_view(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_add_active_view(handle: *mut c_void, view: *const i8) -> ();
    pub fn ocio_config_remove_active_view(handle: *mut c_void, view: *const i8) -> ();
    pub fn ocio_config_clear_active_views(handle: *mut c_void) -> ();
    pub fn ocio_config_get_num_displays_all(handle: *mut c_void) -> i32;
    pub fn ocio_config_get_display_all(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_get_display_all_by_name(handle: *mut c_void, arg0: *mut c_void) -> i32;
    pub fn ocio_config_is_display_temporary(handle: *mut c_void, index: i32) -> bool;
    pub fn ocio_config_set_display_temporary(
        handle: *mut c_void,
        index: i32,
        isTemporary: bool,
    ) -> ();
    pub fn ocio_config_get_num_views_v2(
        handle: *mut c_void,
        type_param: i32,
        display: *const i8,
    ) -> i32;
    pub fn ocio_config_get_view_v2(
        handle: *mut c_void,
        type_param: i32,
        display: *const i8,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_viewing_rules(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_set_viewing_rules(handle: *mut c_void, viewingRules: *mut c_void) -> ();
    pub fn ocio_config_get_default_luma_coefs(handle: *mut c_void, rgb: *mut c_void) -> ();
    pub fn ocio_config_set_default_luma_coefs(handle: *mut c_void, rgb: *mut c_void) -> ();
    pub fn ocio_config_get_look(handle: *mut c_void, name: *const i8) -> *mut c_void;
    pub fn ocio_config_get_num_looks(handle: *mut c_void) -> i32;
    pub fn ocio_config_get_look_name_by_index(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_add_look(handle: *mut c_void, look: *mut c_void) -> ();
    pub fn ocio_config_clear_looks(handle: *mut c_void) -> ();
    pub fn ocio_config_get_num_view_transforms(handle: *mut c_void) -> i32;
    pub fn ocio_config_get_view_transform(handle: *mut c_void, name: *const i8) -> *mut c_void;
    pub fn ocio_config_get_view_transform_name_by_index(handle: *mut c_void, i: i32)
        -> *mut c_void;
    pub fn ocio_config_add_view_transform(handle: *mut c_void, viewTransform: *mut c_void) -> ();
    pub fn ocio_config_get_default_scene_to_display_view_transform(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_config_get_default_view_transform_name(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_set_default_view_transform_name(
        handle: *mut c_void,
        defaultName: *const i8,
    ) -> ();
    pub fn ocio_config_clear_view_transforms(handle: *mut c_void) -> ();
    pub fn ocio_config_get_num_named_transforms(handle: *mut c_void, visibility: i32) -> i32;
    pub fn ocio_config_get_named_transform_name_by_index(
        handle: *mut c_void,
        visibility: i32,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_num_named_transforms_v1(handle: *mut c_void) -> i32;
    pub fn ocio_config_get_named_transform_name_by_index_v1(
        handle: *mut c_void,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_index_for_named_transform(handle: *mut c_void, name: *const i8) -> i32;
    pub fn ocio_config_get_named_transform(handle: *mut c_void, name: *const i8) -> *mut c_void;
    pub fn ocio_config_add_named_transform(handle: *mut c_void, namedTransform: *mut c_void) -> ();
    pub fn ocio_config_remove_named_transform(handle: *mut c_void, name: *const i8) -> ();
    pub fn ocio_config_clear_named_transforms(handle: *mut c_void) -> ();
    pub fn ocio_config_get_file_rules(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_set_file_rules(handle: *mut c_void, fileRules: *mut c_void) -> ();
    pub fn ocio_config_get_color_space_from_filepath(
        handle: *mut c_void,
        filePath: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_color_space_from_filepath_by_ref_type(
        handle: *mut c_void,
        filePath: *const i8,
        ruleIndex: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_config_get_color_space_from_filepath_with_rule_index(
        handle: *mut c_void,
        filePath: *const i8,
        ruleIndex: *mut usize,
    ) -> *mut c_void;
    pub fn ocio_config_filepath_only_matches_default_rule(
        handle: *mut c_void,
        filePath: *const i8,
    ) -> bool;
    pub fn ocio_config_parse_color_space_from_string(
        handle: *mut c_void,
        str: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_is_strict_parsing_enabled(handle: *mut c_void) -> bool;
    pub fn ocio_config_set_strict_parsing_enabled(handle: *mut c_void, enabled: bool) -> ();
    pub fn ocio_config_get_processor(
        handle: *mut c_void,
        context: *mut c_void,
        srcColorSpace: *mut c_void,
        dstColorSpace: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v1(
        handle: *mut c_void,
        srcColorSpace: *mut c_void,
        dstColorSpace: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v2(
        handle: *mut c_void,
        srcColorSpaceName: *const i8,
        dstColorSpaceName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v3(
        handle: *mut c_void,
        context: *mut c_void,
        srcColorSpaceName: *const i8,
        dstColorSpaceName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v4(
        handle: *mut c_void,
        srcColorSpaceName: *const i8,
        display: *const i8,
        view: *const i8,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v5(
        handle: *mut c_void,
        context: *mut c_void,
        srcColorSpaceName: *const i8,
        display: *const i8,
        view: *const i8,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v6(
        handle: *mut c_void,
        namedTransform: *mut c_void,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v7(
        handle: *mut c_void,
        context: *mut c_void,
        namedTransform: *mut c_void,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v8(
        handle: *mut c_void,
        namedTransformName: *const i8,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v9(
        handle: *mut c_void,
        context: *mut c_void,
        namedTransformName: *const i8,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v10(
        handle: *mut c_void,
        transform: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v11(
        handle: *mut c_void,
        transform: *mut c_void,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_v12(
        handle: *mut c_void,
        context: *mut c_void,
        transform: *mut c_void,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_to_builtin_color_space(
        handle: *mut c_void,
        srcConfig: *mut c_void,
        srcColorSpaceName: *const i8,
        builtinColorSpaceName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_from_builtin_color_space(
        handle: *mut c_void,
        builtinColorSpaceName: *const i8,
        srcConfig: *mut c_void,
        srcColorSpaceName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_from_configs(
        handle: *mut c_void,
        srcConfig: *mut c_void,
        srcColorSpaceName: *const i8,
        dstConfig: *mut c_void,
        dstColorSpaceName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_from_configs_v1(
        handle: *mut c_void,
        srcContext: *mut c_void,
        srcConfig: *mut c_void,
        srcColorSpaceName: *const i8,
        dstContext: *mut c_void,
        dstConfig: *mut c_void,
        dstColorSpaceName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_from_configs_v2(
        handle: *mut c_void,
        srcConfig: *mut c_void,
        srcColorSpaceName: *const i8,
        srcInterchangeName: *const i8,
        dstConfig: *mut c_void,
        dstColorSpaceName: *const i8,
        dstInterchangeName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_from_configs_v3(
        handle: *mut c_void,
        srcContext: *mut c_void,
        srcConfig: *mut c_void,
        srcColorSpaceName: *const i8,
        srcInterchangeName: *const i8,
        dstContext: *mut c_void,
        dstConfig: *mut c_void,
        dstColorSpaceName: *const i8,
        dstInterchangeName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_from_configs_v4(
        handle: *mut c_void,
        srcConfig: *mut c_void,
        srcColorSpaceName: *const i8,
        dstConfig: *mut c_void,
        dstDisplay: *const i8,
        dstView: *const i8,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_from_configs_v5(
        handle: *mut c_void,
        srcContext: *mut c_void,
        srcConfig: *mut c_void,
        srcColorSpaceName: *const i8,
        dstContext: *mut c_void,
        dstConfig: *mut c_void,
        dstDisplay: *const i8,
        dstView: *const i8,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_from_configs_v6(
        handle: *mut c_void,
        srcConfig: *mut c_void,
        srcColorSpaceName: *const i8,
        srcInterchangeName: *const i8,
        dstConfig: *mut c_void,
        dstDisplay: *const i8,
        dstView: *const i8,
        dstInterchangeName: *const i8,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_from_configs_v7(
        handle: *mut c_void,
        srcContext: *mut c_void,
        srcConfig: *mut c_void,
        srcColorSpaceName: *const i8,
        srcInterchangeName: *const i8,
        dstContext: *mut c_void,
        dstConfig: *mut c_void,
        dstDisplay: *const i8,
        dstView: *const i8,
        dstInterchangeName: *const i8,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_cache_flags(handle: *mut c_void) -> i32;
    pub fn ocio_config_set_processor_cache_flags(handle: *mut c_void, flags: i32) -> ();
    pub fn ocio_config_clear_processor_cache(handle: *mut c_void) -> ();
    pub fn ocio_config_set_config_io_proxy(handle: *mut c_void, ciop: *mut c_void) -> ();
    pub fn ocio_config_get_config_io_proxy(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_is_archivable(handle: *mut c_void) -> bool;
    pub fn ocio_config_archive(handle: *mut c_void, ostream: *mut c_void) -> ();
    pub fn ocio_config_archive_to_string(handle: *mut c_void) -> *mut c_void;

    // --- FileRules ---
    pub fn ocio_file_rules_create() -> *mut c_void;
    pub fn ocio_file_rules_destroy(handle: *mut c_void);
    pub fn ocio_file_rules_get_num_entries(handle: *mut c_void) -> usize;
    pub fn ocio_file_rules_get_index_for_rule(handle: *mut c_void, ruleName: *const i8) -> usize;
    pub fn ocio_file_rules_get_name(handle: *mut c_void, ruleIndex: usize) -> *mut c_void;
    pub fn ocio_file_rules_get_pattern(handle: *mut c_void, ruleIndex: usize) -> *mut c_void;
    pub fn ocio_file_rules_set_pattern(
        handle: *mut c_void,
        ruleIndex: usize,
        pattern: *const i8,
    ) -> ();
    pub fn ocio_file_rules_get_extension(handle: *mut c_void, ruleIndex: usize) -> *mut c_void;
    pub fn ocio_file_rules_set_extension(
        handle: *mut c_void,
        ruleIndex: usize,
        extension: *const i8,
    ) -> ();
    pub fn ocio_file_rules_get_regex(handle: *mut c_void, ruleIndex: usize) -> *mut c_void;
    pub fn ocio_file_rules_set_regex(handle: *mut c_void, ruleIndex: usize, regex: *const i8)
        -> ();
    pub fn ocio_file_rules_get_color_space(handle: *mut c_void, ruleIndex: usize) -> *mut c_void;
    pub fn ocio_file_rules_set_color_space(
        handle: *mut c_void,
        ruleIndex: usize,
        colorSpace: *const i8,
    ) -> ();
    pub fn ocio_file_rules_get_num_custom_keys(handle: *mut c_void, ruleIndex: usize) -> usize;
    pub fn ocio_file_rules_get_custom_key_name(
        handle: *mut c_void,
        ruleIndex: usize,
        key: usize,
    ) -> *mut c_void;
    pub fn ocio_file_rules_get_custom_key_value(
        handle: *mut c_void,
        ruleIndex: usize,
        key: usize,
    ) -> *mut c_void;
    pub fn ocio_file_rules_set_custom_key(
        handle: *mut c_void,
        ruleIndex: usize,
        key: *const i8,
        value: *const i8,
    ) -> ();
    pub fn ocio_file_rules_insert_rule(
        handle: *mut c_void,
        ruleIndex: usize,
        name: *const i8,
        colorSpace: *const i8,
        pattern: *const i8,
        extension: *const i8,
    ) -> ();
    pub fn ocio_file_rules_insert_rule_v1(
        handle: *mut c_void,
        ruleIndex: usize,
        name: *const i8,
        colorSpace: *const i8,
        regex: *const i8,
    ) -> ();
    pub fn ocio_file_rules_insert_path_search_rule(handle: *mut c_void, ruleIndex: usize) -> ();
    pub fn ocio_file_rules_set_default_rule_color_space(
        handle: *mut c_void,
        colorSpace: *const i8,
    ) -> ();
    pub fn ocio_file_rules_remove_rule(handle: *mut c_void, ruleIndex: usize) -> ();
    pub fn ocio_file_rules_increase_rule_priority(handle: *mut c_void, ruleIndex: usize) -> ();
    pub fn ocio_file_rules_decrease_rule_priority(handle: *mut c_void, ruleIndex: usize) -> ();
    pub fn ocio_file_rules_is_default(handle: *mut c_void) -> bool;

    // --- ViewingRules ---
    pub fn ocio_viewing_rules_create() -> *mut c_void;
    pub fn ocio_viewing_rules_create_editable_copy(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_viewing_rules_destroy(handle: *mut c_void);
    pub fn ocio_viewing_rules_get_num_entries(handle: *mut c_void) -> usize;
    pub fn ocio_viewing_rules_get_index_for_rule(
        handle: *mut c_void,
        ruleName: *const i8,
    ) -> usize;
    pub fn ocio_viewing_rules_get_name(handle: *mut c_void, ruleIndex: usize) -> *mut c_void;
    pub fn ocio_viewing_rules_get_num_color_spaces(
        handle: *mut c_void,
        ruleIndex: usize,
    ) -> usize;
    pub fn ocio_viewing_rules_get_color_space(
        handle: *mut c_void,
        ruleIndex: usize,
        colorSpaceIndex: usize,
    ) -> *mut c_void;
    pub fn ocio_viewing_rules_add_color_space(
        handle: *mut c_void,
        ruleIndex: usize,
        colorSpace: *const i8,
    ) -> ();
    pub fn ocio_viewing_rules_remove_color_space(
        handle: *mut c_void,
        ruleIndex: usize,
        colorSpaceIndex: usize,
    ) -> ();
    pub fn ocio_viewing_rules_get_num_encodings(handle: *mut c_void, ruleIndex: usize) -> usize;
    pub fn ocio_viewing_rules_get_encoding(
        handle: *mut c_void,
        ruleIndex: usize,
        encodingIndex: usize,
    ) -> *mut c_void;
    pub fn ocio_viewing_rules_add_encoding(
        handle: *mut c_void,
        ruleIndex: usize,
        encoding: *const i8,
    ) -> ();
    pub fn ocio_viewing_rules_remove_encoding(
        handle: *mut c_void,
        ruleIndex: usize,
        encodingIndex: usize,
    ) -> ();
    pub fn ocio_viewing_rules_get_num_custom_keys(
        handle: *mut c_void,
        ruleIndex: usize,
    ) -> usize;
    pub fn ocio_viewing_rules_get_custom_key_name(
        handle: *mut c_void,
        ruleIndex: usize,
        keyIndex: usize,
    ) -> *mut c_void;
    pub fn ocio_viewing_rules_get_custom_key_value(
        handle: *mut c_void,
        ruleIndex: usize,
        keyIndex: usize,
    ) -> *mut c_void;
    pub fn ocio_viewing_rules_set_custom_key(
        handle: *mut c_void,
        ruleIndex: usize,
        key: *const i8,
        value: *const i8,
    ) -> ();
    pub fn ocio_viewing_rules_insert_rule(
        handle: *mut c_void,
        ruleIndex: usize,
        ruleName: *const i8,
    ) -> ();
    pub fn ocio_viewing_rules_remove_rule(handle: *mut c_void, ruleIndex: usize) -> ();

    // --- ColorSpace ---
    pub fn ocio_color_space_create() -> *mut c_void;
    pub fn ocio_color_space_destroy(handle: *mut c_void);
    pub fn ocio_color_space_get_name(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_set_name(handle: *mut c_void, name: *const i8) -> ();
    pub fn ocio_color_space_get_num_aliases(handle: *mut c_void) -> usize;
    pub fn ocio_color_space_get_alias(handle: *mut c_void, idx: usize) -> *mut c_void;
    pub fn ocio_color_space_has_alias(handle: *mut c_void, alias: *const i8) -> bool;
    pub fn ocio_color_space_add_alias(handle: *mut c_void, alias: *const i8) -> ();
    pub fn ocio_color_space_remove_alias(handle: *mut c_void, alias: *const i8) -> ();
    pub fn ocio_color_space_clear_aliases(handle: *mut c_void) -> ();
    pub fn ocio_color_space_get_family(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_set_family(handle: *mut c_void, family: *const i8) -> ();
    pub fn ocio_color_space_get_equality_group(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_set_equality_group(handle: *mut c_void, equalityGroup: *const i8)
        -> ();
    pub fn ocio_color_space_get_description(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_set_description(handle: *mut c_void, description: *const i8) -> ();
    pub fn ocio_color_space_get_interop_id(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_set_interop_id(handle: *mut c_void, interopID: *const i8) -> ();
    pub fn ocio_color_space_set_interchange_attribute(
        handle: *mut c_void,
        attrName: *const i8,
        value: *const i8,
    ) -> ();
    pub fn ocio_color_space_get_interchange_attribute(
        handle: *mut c_void,
        attrName: *const i8,
    ) -> *const i8;
    pub fn ocio_color_space_get_num_interchange_attributes(handle: *mut c_void) -> i32;
    pub fn ocio_color_space_get_interchange_attribute_name_by_index(
        handle: *mut c_void,
        index: i32,
    ) -> *const i8;
    pub fn ocio_color_space_get_interchange_attribute_value_by_index(
        handle: *mut c_void,
        index: i32,
    ) -> *const i8;
    pub fn ocio_color_space_get_bit_depth(handle: *mut c_void) -> i32;
    pub fn ocio_color_space_set_bit_depth(handle: *mut c_void, bitDepth: i32) -> ();
    pub fn ocio_color_space_get_reference_space_type(handle: *mut c_void) -> i32;
    pub fn ocio_color_space_has_category(handle: *mut c_void, category: *const i8) -> bool;
    pub fn ocio_color_space_add_category(handle: *mut c_void, category: *const i8) -> ();
    pub fn ocio_color_space_remove_category(handle: *mut c_void, category: *const i8) -> ();
    pub fn ocio_color_space_get_num_categories(handle: *mut c_void) -> i32;
    pub fn ocio_color_space_get_category(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_color_space_clear_categories(handle: *mut c_void) -> ();
    pub fn ocio_color_space_get_encoding(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_set_encoding(handle: *mut c_void, encoding: *const i8) -> ();
    pub fn ocio_color_space_is_data(handle: *mut c_void) -> bool;
    pub fn ocio_color_space_set_is_data(handle: *mut c_void, isData: bool) -> ();
    pub fn ocio_color_space_get_allocation(handle: *mut c_void) -> i32;
    pub fn ocio_color_space_set_allocation(handle: *mut c_void, allocation: i32) -> ();
    pub fn ocio_color_space_get_allocation_num_vars(handle: *mut c_void) -> i32;
    pub fn ocio_color_space_get_allocation_vars(handle: *mut c_void, vars: *mut c_void) -> ();
    pub fn ocio_color_space_set_allocation_vars(
        handle: *mut c_void,
        numvars: i32,
        vars: *mut c_void,
    ) -> ();
    pub fn ocio_color_space_get_transform(handle: *mut c_void, dir: i32) -> *mut c_void;
    pub fn ocio_color_space_set_transform(
        handle: *mut c_void,
        transform: *mut c_void,
        dir: i32,
    ) -> ();

    // --- ColorSpaceSet ---
    pub fn ocio_color_space_set_create() -> *mut c_void;
    pub fn ocio_color_space_set_destroy(handle: *mut c_void);
    pub fn ocio_color_space_set_get_num_color_spaces(handle: *mut c_void) -> i32;
    pub fn ocio_color_space_set_get_color_space_name_by_index(
        handle: *mut c_void,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_color_space_set_get_color_space_by_index(
        handle: *mut c_void,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_color_space_set_get_color_space(
        handle: *mut c_void,
        name: *const i8,
    ) -> *mut c_void;
    pub fn ocio_color_space_set_get_color_space_index(handle: *mut c_void, name: *const i8) -> i32;
    pub fn ocio_color_space_set_has_color_space(handle: *mut c_void, name: *const i8) -> bool;
    pub fn ocio_color_space_set_add_color_space(handle: *mut c_void, cs: *mut c_void) -> ();
    pub fn ocio_color_space_set_add_color_spaces(handle: *mut c_void, cs: *mut c_void) -> ();
    pub fn ocio_color_space_set_remove_color_space(handle: *mut c_void, name: *const i8) -> ();
    pub fn ocio_color_space_set_remove_color_spaces(handle: *mut c_void, cs: *mut c_void) -> ();
    pub fn ocio_color_space_set_clear_color_spaces(handle: *mut c_void) -> ();

    // --- Look ---
    pub fn ocio_look_create() -> *mut c_void;
    pub fn ocio_look_destroy(handle: *mut c_void);
    pub fn ocio_look_get_name(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_look_set_name(handle: *mut c_void, name: *const i8) -> ();
    pub fn ocio_look_get_process_space(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_look_set_process_space(handle: *mut c_void, processSpace: *const i8) -> ();
    pub fn ocio_look_get_transform(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_look_set_transform(handle: *mut c_void, transform: *mut c_void) -> ();
    pub fn ocio_look_get_inverse_transform(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_look_set_inverse_transform(handle: *mut c_void, transform: *mut c_void) -> ();
    pub fn ocio_look_get_description(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_look_set_description(handle: *mut c_void, description: *const i8) -> ();
    pub fn ocio_look_set_interchange_attribute(
        handle: *mut c_void,
        attrName: *const i8,
        value: *const i8,
    ) -> ();
    pub fn ocio_look_get_interchange_attribute(
        handle: *mut c_void,
        attrName: *const i8,
    ) -> *const i8;
    pub fn ocio_look_get_num_interchange_attributes(handle: *mut c_void) -> i32;
    pub fn ocio_look_get_interchange_attribute_name_by_index(
        handle: *mut c_void,
        index: i32,
    ) -> *const i8;
    pub fn ocio_look_get_interchange_attribute_value_by_index(
        handle: *mut c_void,
        index: i32,
    ) -> *const i8;

    // --- NamedTransform ---
    pub fn ocio_named_transform_create() -> *mut c_void;
    pub fn ocio_named_transform_destroy(handle: *mut c_void);
    pub fn ocio_named_transform_get_name(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_named_transform_set_name(handle: *mut c_void, name: *const i8) -> ();
    pub fn ocio_named_transform_get_num_aliases(handle: *mut c_void) -> usize;
    pub fn ocio_named_transform_get_alias(handle: *mut c_void, idx: usize) -> *mut c_void;
    pub fn ocio_named_transform_has_alias(handle: *mut c_void, alias: *const i8) -> bool;
    pub fn ocio_named_transform_add_alias(handle: *mut c_void, alias: *const i8) -> ();
    pub fn ocio_named_transform_remove_alias(handle: *mut c_void, alias: *const i8) -> ();
    pub fn ocio_named_transform_clear_aliases(handle: *mut c_void) -> ();
    pub fn ocio_named_transform_get_family(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_named_transform_set_family(handle: *mut c_void, family: *const i8) -> ();
    pub fn ocio_named_transform_get_description(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_named_transform_set_description(handle: *mut c_void, description: *const i8) -> ();
    pub fn ocio_named_transform_has_category(handle: *mut c_void, category: *const i8) -> bool;
    pub fn ocio_named_transform_add_category(handle: *mut c_void, category: *const i8) -> ();
    pub fn ocio_named_transform_remove_category(handle: *mut c_void, category: *const i8) -> ();
    pub fn ocio_named_transform_get_num_categories(handle: *mut c_void) -> i32;
    pub fn ocio_named_transform_get_category(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_named_transform_clear_categories(handle: *mut c_void) -> ();
    pub fn ocio_named_transform_get_encoding(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_named_transform_set_encoding(handle: *mut c_void, encoding: *const i8) -> ();
    pub fn ocio_named_transform_get_transform(handle: *mut c_void, dir: i32) -> *mut c_void;
    pub fn ocio_named_transform_set_transform(
        handle: *mut c_void,
        transform: *mut c_void,
        dir: i32,
    ) -> ();

    // --- ViewTransform ---
    pub fn ocio_view_transform_create() -> *mut c_void;
    pub fn ocio_view_transform_create_with_reference_space(referenceSpace: i32) -> *mut c_void;
    pub fn ocio_view_transform_destroy(handle: *mut c_void);
    pub fn ocio_view_transform_create_editable_copy(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_view_transform_get_name(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_view_transform_set_name(handle: *mut c_void, name: *const i8) -> ();
    pub fn ocio_view_transform_get_family(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_view_transform_set_family(handle: *mut c_void, family: *const i8) -> ();
    pub fn ocio_view_transform_get_description(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_view_transform_set_description(handle: *mut c_void, description: *const i8) -> ();
    pub fn ocio_view_transform_set_interchange_attribute(
        handle: *mut c_void,
        attrName: *const i8,
        value: *const i8,
    ) -> ();
    pub fn ocio_view_transform_get_interchange_attribute(
        handle: *mut c_void,
        attrName: *const i8,
    ) -> *const i8;
    pub fn ocio_view_transform_get_num_interchange_attributes(handle: *mut c_void) -> i32;
    pub fn ocio_view_transform_get_interchange_attribute_name_by_index(
        handle: *mut c_void,
        index: i32,
    ) -> *const i8;
    pub fn ocio_view_transform_get_interchange_attribute_value_by_index(
        handle: *mut c_void,
        index: i32,
    ) -> *const i8;
    pub fn ocio_view_transform_has_category(handle: *mut c_void, category: *const i8) -> bool;
    pub fn ocio_view_transform_add_category(handle: *mut c_void, category: *const i8) -> ();
    pub fn ocio_view_transform_remove_category(handle: *mut c_void, category: *const i8) -> ();
    pub fn ocio_view_transform_get_num_categories(handle: *mut c_void) -> i32;
    pub fn ocio_view_transform_get_category(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_view_transform_clear_categories(handle: *mut c_void) -> ();
    pub fn ocio_view_transform_get_reference_space_type(handle: *mut c_void) -> i32;
    pub fn ocio_view_transform_get_transform(handle: *mut c_void, dir: i32) -> *mut c_void;
    pub fn ocio_view_transform_set_transform(
        handle: *mut c_void,
        transform: *mut c_void,
        dir: i32,
    ) -> ();

    // --- Transform ---
    pub fn ocio_transform_create_editable_copy(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_transform_get_transform_type(handle: *mut c_void) -> i32;

    // --- Processor ---
    pub fn ocio_processor_destroy(handle: *mut c_void);
    pub fn ocio_processor_is_no_op(handle: *mut c_void) -> bool;
    pub fn ocio_processor_has_channel_crosstalk(handle: *mut c_void) -> bool;
    pub fn ocio_processor_get_cache_id(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_processor_get_processor_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_processor_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_processor_get_num_transforms(handle: *mut c_void) -> i32;
    pub fn ocio_processor_get_transform_format_metadata(
        handle: *mut c_void,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_processor_create_group_transform(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_processor_get_dynamic_property(handle: *mut c_void, type_param: i32)
        -> *mut c_void;
    pub fn ocio_processor_has_dynamic_property(handle: *mut c_void, type_param: i32) -> bool;
    pub fn ocio_processor_is_dynamic(handle: *mut c_void) -> bool;
    pub fn ocio_processor_get_optimized_processor_v1(
        handle: *mut c_void,
        oFlags: i32,
    ) -> *mut c_void;
    pub fn ocio_processor_get_optimized_processor_v2(
        handle: *mut c_void,
        inBD: i32,
        outBD: i32,
        oFlags: i32,
    ) -> *mut c_void;
    pub fn ocio_processor_optimized_processor(handle: *mut c_void, oFlags: i32) -> *mut c_void;
    pub fn ocio_processor_get_default_gpu_processor(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_processor_get_optimized_gpu_processor(
        handle: *mut c_void,
        oFlags: i32,
    ) -> *mut c_void;
    pub fn ocio_processor_get_optimized_legacy_gpu_processor(
        handle: *mut c_void,
        oFlags: i32,
        edgelen: u32,
    ) -> *mut c_void;
    pub fn ocio_processor_get_default_cpu_processor(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_processor_get_optimized_cpu_processor(
        handle: *mut c_void,
        oFlags: i32,
    ) -> *mut c_void;

    // --- ProcessorMetadata ---
    pub fn ocio_processor_metadata_create() -> *mut c_void;
    pub fn ocio_processor_metadata_destroy(handle: *mut c_void);
    pub fn ocio_processor_metadata_get_num_files(handle: *mut c_void) -> i32;
    pub fn ocio_processor_metadata_get_file(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_processor_metadata_get_num_looks(handle: *mut c_void) -> i32;
    pub fn ocio_processor_metadata_get_look(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_processor_metadata_add_file(handle: *mut c_void, fileName: *const i8) -> ();
    pub fn ocio_processor_metadata_add_look(handle: *mut c_void, look: *const i8) -> ();

    // --- CPUProcessor ---
    pub fn ocio_cpu_processor_destroy(handle: *mut c_void);
    pub fn ocio_cpu_processor_is_no_op(handle: *mut c_void) -> bool;
    pub fn ocio_cpu_processor_is_identity(handle: *mut c_void) -> bool;
    pub fn ocio_cpu_processor_has_channel_crosstalk(handle: *mut c_void) -> bool;
    pub fn ocio_cpu_processor_get_cache_id(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_cpu_processor_get_input_bit_depth(handle: *mut c_void) -> i32;
    pub fn ocio_cpu_processor_get_output_bit_depth(handle: *mut c_void) -> i32;
    pub fn ocio_cpu_processor_get_dynamic_property(
        handle: *mut c_void,
        type_param: i32,
    ) -> *mut c_void;
    pub fn ocio_cpu_processor_has_dynamic_property(handle: *mut c_void, type_param: i32) -> bool;
    pub fn ocio_cpu_processor_is_dynamic(handle: *mut c_void) -> bool;
    pub fn ocio_cpu_processor_apply(handle: *mut c_void, imgDesc: *mut c_void) -> ();
    pub fn ocio_cpu_processor_apply_v1(handle: *mut c_void, imgDesc: *mut c_void) -> ();
    pub fn ocio_cpu_processor_apply_v2(
        handle: *mut c_void,
        srcImgDesc: *mut c_void,
        dstImgDesc: *mut c_void,
    ) -> ();
    pub fn ocio_cpu_processor_apply_rgb(handle: *mut c_void, pixel: *mut c_void) -> ();
    pub fn ocio_cpu_processor_apply_rgba(handle: *mut c_void, pixel: *mut c_void) -> ();

    // --- GPUProcessor ---
    pub fn ocio_gpu_processor_destroy(handle: *mut c_void);
    pub fn ocio_gpu_processor_is_no_op(handle: *mut c_void) -> bool;
    pub fn ocio_gpu_processor_has_channel_crosstalk(handle: *mut c_void) -> bool;
    pub fn ocio_gpu_processor_get_cache_id(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_gpu_processor_extract_gpu_shader_info(
        handle: *mut c_void,
        shaderDesc: *mut c_void,
    ) -> ();
    pub fn ocio_gpu_processor_extract_gpu_shader_info_v1(
        handle: *mut c_void,
        shaderDesc: *mut c_void,
    ) -> ();
    pub fn ocio_gpu_processor_extract_gpu_shader_info_v2(
        handle: *mut c_void,
        shaderCreator: *mut c_void,
    ) -> ();

    // --- GpuShaderDesc ---
    pub fn ocio_gpu_shader_desc_create_shader_desc() -> *mut c_void;
    pub fn ocio_gpu_shader_desc_create() -> *mut c_void;
    pub fn ocio_gpu_shader_desc_destroy(handle: *mut c_void);
    pub fn ocio_gpu_shader_desc_clone(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_gpu_shader_desc_get_num_uniforms_u32(handle: *mut c_void) -> u32;
    pub fn ocio_gpu_shader_desc_get_uniform_info(
        handle: *mut c_void,
        index: u32,
        out: *mut OcioGpuUniformInfo,
    ) -> bool;
    pub fn ocio_gpu_shader_desc_get_uniform_value_count(handle: *mut c_void, index: u32) -> usize;
    pub fn ocio_gpu_shader_desc_copy_uniform_f32_values(
        handle: *mut c_void,
        index: u32,
        values: *mut f32,
        len: usize,
    ) -> bool;
    pub fn ocio_gpu_shader_desc_copy_uniform_i32_values(
        handle: *mut c_void,
        index: u32,
        values: *mut i32,
        len: usize,
    ) -> bool;
    pub fn ocio_gpu_shader_desc_get_uniform_buffer_size_bytes(handle: *mut c_void) -> usize;
    pub fn ocio_gpu_shader_desc_get_num_textures_u32(handle: *mut c_void) -> u32;
    pub fn ocio_gpu_shader_desc_get_texture_info(
        handle: *mut c_void,
        index: u32,
        out: *mut OcioGpuTexture2DInfo,
    ) -> bool;
    pub fn ocio_gpu_shader_desc_get_texture_value_count(handle: *mut c_void, index: u32) -> usize;
    pub fn ocio_gpu_shader_desc_copy_texture_values(
        handle: *mut c_void,
        index: u32,
        values: *mut f32,
        len: usize,
    ) -> bool;
    pub fn ocio_gpu_shader_desc_get_num3d_textures_u32(handle: *mut c_void) -> u32;
    pub fn ocio_gpu_shader_desc_get3d_texture_info(
        handle: *mut c_void,
        index: u32,
        out: *mut OcioGpuTexture3DInfo,
    ) -> bool;
    pub fn ocio_gpu_shader_desc_get3d_texture_value_count(handle: *mut c_void, index: u32)
        -> usize;
    pub fn ocio_gpu_shader_desc_copy3d_texture_values(
        handle: *mut c_void,
        index: u32,
        values: *mut f32,
        len: usize,
    ) -> bool;
    pub fn ocio_gpu_shader_desc_get_num_uniforms(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_gpu_shader_desc_get_uniform(
        handle: *mut c_void,
        index: *mut c_void,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_gpu_shader_desc_get_uniform_buffer_size(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_gpu_shader_desc_get_num_textures(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_gpu_shader_desc_get_texture(
        handle: *mut c_void,
        index: *mut c_void,
        textureName: *const i8,
        samplerName: *const i8,
        width: *mut c_void,
        height: *mut c_void,
        channel: *mut c_void,
        dimensions: *mut c_void,
        interpolation: *mut c_void,
    ) -> ();
    pub fn ocio_gpu_shader_desc_get_texture_values(
        handle: *mut c_void,
        index: *mut c_void,
        values: *mut c_void,
    ) -> ();
    pub fn ocio_gpu_shader_desc_get_texture_shader_binding_index(
        handle: *mut c_void,
        index: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_gpu_shader_desc_get_num3d_textures(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_gpu_shader_desc_get3d_texture(
        handle: *mut c_void,
        index: *mut c_void,
        textureName: *const i8,
        samplerName: *const i8,
        edgelen: *mut c_void,
        interpolation: *mut c_void,
    ) -> ();
    pub fn ocio_gpu_shader_desc_get3d_texture_values(
        handle: *mut c_void,
        index: *mut c_void,
        values: *mut c_void,
    ) -> ();
    pub fn ocio_gpu_shader_desc_get3d_texture_shader_binding_index(
        handle: *mut c_void,
        index: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_gpu_shader_desc_get_shader_text(handle: *mut c_void) -> *mut c_void;

    // --- Baker ---
    pub fn ocio_baker_create() -> *mut c_void;
    pub fn ocio_baker_destroy(handle: *mut c_void);
    pub fn ocio_baker_get_config(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_set_config(handle: *mut c_void, config: *mut c_void) -> ();
    pub fn ocio_baker_get_format(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_set_format(handle: *mut c_void, formatName: *const i8) -> ();
    pub fn ocio_baker_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_get_input_space(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_set_input_space(handle: *mut c_void, inputSpace: *const i8) -> ();
    pub fn ocio_baker_get_shaper_space(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_set_shaper_space(handle: *mut c_void, shaperSpace: *const i8) -> ();
    pub fn ocio_baker_get_looks(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_set_looks(handle: *mut c_void, looks: *const i8) -> ();
    pub fn ocio_baker_get_target_space(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_set_target_space(handle: *mut c_void, targetSpace: *const i8) -> ();
    pub fn ocio_baker_get_display(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_get_view(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_set_display_view(
        handle: *mut c_void,
        display: *const i8,
        view: *const i8,
    ) -> ();
    pub fn ocio_baker_get_shaper_size(handle: *mut c_void) -> i32;
    pub fn ocio_baker_set_shaper_size(handle: *mut c_void, shapersize: i32) -> ();
    pub fn ocio_baker_get_cube_size(handle: *mut c_void) -> i32;
    pub fn ocio_baker_set_cube_size(handle: *mut c_void, cubesize: i32) -> ();
    pub fn ocio_baker_bake(handle: *mut c_void, os: *mut c_void) -> ();
    pub fn ocio_baker_bake_to_string(handle: *mut c_void) -> *mut c_void;

    // --- Context ---
    pub fn ocio_context_create() -> *mut c_void;
    pub fn ocio_context_destroy(handle: *mut c_void);
    pub fn ocio_context_get_cache_id(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_context_set_search_path(handle: *mut c_void, path: *const i8) -> ();
    pub fn ocio_context_get_search_path(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_context_get_num_search_paths(handle: *mut c_void) -> i32;
    pub fn ocio_context_get_search_path_by_index(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_context_clear_search_paths(handle: *mut c_void) -> ();
    pub fn ocio_context_add_search_path(handle: *mut c_void, path: *const i8) -> ();
    pub fn ocio_context_set_working_dir(handle: *mut c_void, dirname: *const i8) -> ();
    pub fn ocio_context_get_working_dir(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_context_set_string_var(
        handle: *mut c_void,
        name: *const i8,
        value: *const i8,
    ) -> ();
    pub fn ocio_context_get_string_var(handle: *mut c_void, name: *const i8) -> *mut c_void;
    pub fn ocio_context_get_num_string_vars(handle: *mut c_void) -> i32;
    pub fn ocio_context_get_string_var_name_by_index(
        handle: *mut c_void,
        index: i32,
    ) -> *mut c_void;
    pub fn ocio_context_get_string_var_by_index(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_context_clear_string_vars(handle: *mut c_void) -> ();
    pub fn ocio_context_add_string_vars(handle: *mut c_void, ctx: *mut c_void) -> ();
    pub fn ocio_context_set_environment_mode(handle: *mut c_void, mode: i32) -> ();
    pub fn ocio_context_get_environment_mode(handle: *mut c_void) -> i32;
    pub fn ocio_context_load_environment(handle: *mut c_void) -> ();
    pub fn ocio_context_resolve_string_var(handle: *mut c_void, string: *const i8) -> *mut c_void;
    pub fn ocio_context_resolve_string_var_v1(
        handle: *mut c_void,
        string: *const i8,
        usedContextVars: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_context_resolve_file_location(
        handle: *mut c_void,
        filename: *const i8,
    ) -> *mut c_void;
    pub fn ocio_context_resolve_file_location_v1(
        handle: *mut c_void,
        filename: *const i8,
        usedContextVars: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_context_set_config_io_proxy(handle: *mut c_void, ciop: *mut c_void) -> ();
    pub fn ocio_context_get_config_io_proxy(handle: *mut c_void) -> *mut c_void;

    // --- AllocationTransform ---
    pub fn ocio_allocation_transform_create() -> *mut c_void;
    pub fn ocio_allocation_transform_destroy(handle: *mut c_void);
    pub fn ocio_allocation_transform_create_editable_copy(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_allocation_transform_get_direction(handle: *mut c_void) -> i32;
    pub fn ocio_allocation_transform_set_direction(handle: *mut c_void, dir: i32) -> ();
    pub fn ocio_allocation_transform_validate(handle: *mut c_void) -> ();
    pub fn ocio_allocation_transform_get_allocation(handle: *mut c_void) -> i32;
    pub fn ocio_allocation_transform_set_allocation(handle: *mut c_void, allocation: i32) -> ();
    pub fn ocio_allocation_transform_get_num_vars(handle: *mut c_void) -> i32;
    pub fn ocio_allocation_transform_get_vars(handle: *mut c_void, vars: *mut c_void) -> ();
    pub fn ocio_allocation_transform_set_vars(
        handle: *mut c_void,
        numvars: i32,
        vars: *mut c_void,
    ) -> ();

    // --- BuiltinTransform ---
    pub fn ocio_builtin_transform_create() -> *mut c_void;
    pub fn ocio_builtin_transform_destroy(handle: *mut c_void);
    pub fn ocio_builtin_transform_get_style(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_builtin_transform_set_style(handle: *mut c_void, style: *const i8) -> ();
    pub fn ocio_builtin_transform_get_description(handle: *mut c_void) -> *mut c_void;

    // --- CDLTransform ---
    pub fn ocio_cdl_transform_create() -> *mut c_void;
    pub fn ocio_cdl_transform_destroy(handle: *mut c_void);
    pub fn ocio_cdl_transform_from_file(src: *const i8, cccId: *const i8) -> *mut c_void;
    pub fn ocio_cdl_transform_create_group_from_file(src: *const i8) -> *mut c_void;
    pub fn ocio_cdl_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_cdl_transform_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_cdl_transform_get_format_metadata_v2(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_cdl_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_cdl_transform_get_style(handle: *mut c_void) -> i32;
    pub fn ocio_cdl_transform_set_style(handle: *mut c_void, style: i32) -> ();
    pub fn ocio_cdl_transform_get_slope(handle: *mut c_void, rgb: *mut c_void) -> ();
    pub fn ocio_cdl_transform_set_slope(handle: *mut c_void, rgb: *mut c_void) -> ();
    pub fn ocio_cdl_transform_get_offset(handle: *mut c_void, rgb: *mut c_void) -> ();
    pub fn ocio_cdl_transform_set_offset(handle: *mut c_void, rgb: *mut c_void) -> ();
    pub fn ocio_cdl_transform_get_power(handle: *mut c_void, rgb: *mut c_void) -> ();
    pub fn ocio_cdl_transform_set_power(handle: *mut c_void, rgb: *mut c_void) -> ();
    pub fn ocio_cdl_transform_get_sop(handle: *mut c_void, vec9: *mut c_void) -> ();
    pub fn ocio_cdl_transform_set_sop(handle: *mut c_void, vec9: *mut c_void) -> ();
    pub fn ocio_cdl_transform_get_sat(handle: *mut c_void) -> f64;
    pub fn ocio_cdl_transform_set_sat(handle: *mut c_void, sat: f64) -> ();
    pub fn ocio_cdl_transform_get_sat_luma_coefs(handle: *mut c_void, rgb: *mut c_void) -> ();
    pub fn ocio_cdl_transform_get_id(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_cdl_transform_set_id(handle: *mut c_void, id: *const i8) -> ();
    pub fn ocio_cdl_transform_get_first_sop_description(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_cdl_transform_set_first_sop_description(
        handle: *mut c_void,
        description: *const i8,
    ) -> ();

    // --- ColorSpaceTransform ---
    pub fn ocio_color_space_transform_create() -> *mut c_void;
    pub fn ocio_color_space_transform_destroy(handle: *mut c_void);
    pub fn ocio_color_space_transform_create_editable_copy(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_transform_get_direction(handle: *mut c_void) -> i32;
    pub fn ocio_color_space_transform_set_direction(handle: *mut c_void, dir: i32) -> ();
    pub fn ocio_color_space_transform_validate(handle: *mut c_void) -> ();
    pub fn ocio_color_space_transform_get_src(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_transform_set_src(handle: *mut c_void, src: *const i8) -> ();
    pub fn ocio_color_space_transform_get_dst(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_transform_set_dst(handle: *mut c_void, dst: *const i8) -> ();
    pub fn ocio_color_space_transform_get_data_bypass(handle: *mut c_void) -> bool;
    pub fn ocio_color_space_transform_set_data_bypass(handle: *mut c_void, enabled: bool) -> ();

    // --- DisplayViewTransform ---
    pub fn ocio_display_view_transform_create() -> *mut c_void;
    pub fn ocio_display_view_transform_destroy(handle: *mut c_void);
    pub fn ocio_display_view_transform_create_editable_copy(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_display_view_transform_get_direction(handle: *mut c_void) -> i32;
    pub fn ocio_display_view_transform_set_direction(handle: *mut c_void, dir: i32) -> ();
    pub fn ocio_display_view_transform_validate(handle: *mut c_void) -> ();
    pub fn ocio_display_view_transform_get_src(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_display_view_transform_set_src(handle: *mut c_void, name: *const i8) -> ();
    pub fn ocio_display_view_transform_get_display(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_display_view_transform_set_display(handle: *mut c_void, display: *const i8) -> ();
    pub fn ocio_display_view_transform_get_view(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_display_view_transform_set_view(handle: *mut c_void, view: *const i8) -> ();
    pub fn ocio_display_view_transform_get_looks_bypass(handle: *mut c_void) -> bool;
    pub fn ocio_display_view_transform_set_looks_bypass(handle: *mut c_void, bypass: bool) -> ();
    pub fn ocio_display_view_transform_get_data_bypass(handle: *mut c_void) -> bool;
    pub fn ocio_display_view_transform_set_data_bypass(handle: *mut c_void, bypass: bool) -> ();

    // --- ExponentTransform ---
    pub fn ocio_exponent_transform_create() -> *mut c_void;
    pub fn ocio_exponent_transform_destroy(handle: *mut c_void);
    pub fn ocio_exponent_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_exponent_transform_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_exponent_transform_get_format_metadata_v2(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_exponent_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_exponent_transform_get_negative_style(handle: *mut c_void) -> i32;
    pub fn ocio_exponent_transform_set_negative_style(handle: *mut c_void, style: i32) -> ();

    // --- ExponentWithLinearTransform ---
    pub fn ocio_exponent_with_linear_transform_create() -> *mut c_void;
    pub fn ocio_exponent_with_linear_transform_destroy(handle: *mut c_void);
    pub fn ocio_exponent_with_linear_transform_get_format_metadata(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_exponent_with_linear_transform_get_format_metadata_v1(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_exponent_with_linear_transform_get_format_metadata_v2(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_exponent_with_linear_transform_equals(
        handle: *mut c_void,
        other: *mut c_void,
    ) -> bool;
    pub fn ocio_exponent_with_linear_transform_get_negative_style(handle: *mut c_void) -> i32;
    pub fn ocio_exponent_with_linear_transform_set_negative_style(
        handle: *mut c_void,
        style: i32,
    ) -> ();

    // --- ExposureContrastTransform ---
    pub fn ocio_exposure_contrast_transform_create() -> *mut c_void;
    pub fn ocio_exposure_contrast_transform_destroy(handle: *mut c_void);
    pub fn ocio_exposure_contrast_transform_get_format_metadata(handle: *mut c_void)
        -> *mut c_void;
    pub fn ocio_exposure_contrast_transform_get_format_metadata_v1(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_exposure_contrast_transform_get_format_metadata_v2(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_exposure_contrast_transform_equals(handle: *mut c_void, other: *mut c_void)
        -> bool;
    pub fn ocio_exposure_contrast_transform_get_style(handle: *mut c_void) -> i32;
    pub fn ocio_exposure_contrast_transform_set_style(handle: *mut c_void, style: i32) -> ();
    pub fn ocio_exposure_contrast_transform_get_exposure(handle: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_exposure(handle: *mut c_void, exposure: f64) -> ();
    pub fn ocio_exposure_contrast_transform_is_exposure_dynamic(handle: *mut c_void) -> bool;
    pub fn ocio_exposure_contrast_transform_make_exposure_dynamic(handle: *mut c_void) -> ();
    pub fn ocio_exposure_contrast_transform_make_exposure_non_dynamic(handle: *mut c_void) -> ();
    pub fn ocio_exposure_contrast_transform_get_contrast(handle: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_contrast(handle: *mut c_void, contrast: f64) -> ();
    pub fn ocio_exposure_contrast_transform_is_contrast_dynamic(handle: *mut c_void) -> bool;
    pub fn ocio_exposure_contrast_transform_make_contrast_dynamic(handle: *mut c_void) -> ();
    pub fn ocio_exposure_contrast_transform_make_contrast_non_dynamic(handle: *mut c_void) -> ();
    pub fn ocio_exposure_contrast_transform_get_gamma(handle: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_gamma(handle: *mut c_void, gamma: f64) -> ();
    pub fn ocio_exposure_contrast_transform_is_gamma_dynamic(handle: *mut c_void) -> bool;
    pub fn ocio_exposure_contrast_transform_make_gamma_dynamic(handle: *mut c_void) -> ();
    pub fn ocio_exposure_contrast_transform_make_gamma_non_dynamic(handle: *mut c_void) -> ();
    pub fn ocio_exposure_contrast_transform_get_pivot(handle: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_pivot(handle: *mut c_void, pivot: f64) -> ();
    pub fn ocio_exposure_contrast_transform_get_log_exposure_step(handle: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_log_exposure_step(
        handle: *mut c_void,
        logExposureStep: f64,
    ) -> ();
    pub fn ocio_exposure_contrast_transform_get_log_mid_gray(handle: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_log_mid_gray(
        handle: *mut c_void,
        logMidGray: f64,
    ) -> ();

    // --- FileTransform ---
    pub fn ocio_file_transform_create() -> *mut c_void;
    pub fn ocio_file_transform_destroy(handle: *mut c_void);
    pub fn ocio_file_transform_create_editable_copy(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_file_transform_get_direction(handle: *mut c_void) -> i32;
    pub fn ocio_file_transform_set_direction(handle: *mut c_void, dir: i32) -> ();
    pub fn ocio_file_transform_validate(handle: *mut c_void) -> ();
    pub fn ocio_file_transform_get_src(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_file_transform_set_src(handle: *mut c_void, src: *const i8) -> ();
    pub fn ocio_file_transform_get_ccc_id(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_file_transform_set_ccc_id(handle: *mut c_void, id: *const i8) -> ();
    pub fn ocio_file_transform_get_cdl_style(handle: *mut c_void) -> i32;
    pub fn ocio_file_transform_set_cdl_style(handle: *mut c_void, arg0: i32) -> ();
    pub fn ocio_file_transform_get_interpolation(handle: *mut c_void) -> i32;
    pub fn ocio_file_transform_set_interpolation(handle: *mut c_void, interp: i32) -> ();
    pub fn ocio_file_transform_get_num_formats() -> i32;
    pub fn ocio_file_transform_get_format_name_by_index(index: i32) -> *const i8;
    pub fn ocio_file_transform_get_format_extension_by_index(index: i32) -> *const i8;
    pub fn ocio_file_transform_is_format_extension_supported(extension: *const i8) -> bool;

    // --- FixedFunctionTransform ---
    pub fn ocio_fixed_function_transform_create() -> *mut c_void;
    pub fn ocio_fixed_function_transform_create_with_params(
        style: i32,
        params: *const f64,
        num: usize,
    ) -> *mut c_void;
    pub fn ocio_fixed_function_transform_destroy(handle: *mut c_void);
    pub fn ocio_fixed_function_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_fixed_function_transform_get_format_metadata_v1(handle: *mut c_void)
        -> *mut c_void;
    pub fn ocio_fixed_function_transform_get_format_metadata_v2(handle: *mut c_void)
        -> *mut c_void;
    pub fn ocio_fixed_function_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_fixed_function_transform_get_direction(handle: *mut c_void) -> i32;
    pub fn ocio_fixed_function_transform_set_direction(handle: *mut c_void, dir: i32) -> ();
    pub fn ocio_fixed_function_transform_get_style(handle: *mut c_void) -> i32;
    pub fn ocio_fixed_function_transform_set_style(handle: *mut c_void, style: i32) -> ();
    pub fn ocio_fixed_function_transform_get_num_params(handle: *mut c_void) -> usize;
    pub fn ocio_fixed_function_transform_get_params(handle: *mut c_void, params: *mut c_void)
        -> ();
    pub fn ocio_fixed_function_transform_set_params(
        handle: *mut c_void,
        params: *const f64,
        num: usize,
    ) -> ();

    // --- GradingPrimaryTransform ---
    pub fn ocio_grading_primary_transform_create() -> *mut c_void;
    pub fn ocio_grading_primary_transform_create_with_style(style: i32) -> *mut c_void;
    pub fn ocio_grading_primary_transform_destroy(handle: *mut c_void);
    pub fn ocio_grading_primary_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_grading_primary_transform_get_format_metadata_v1(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_grading_primary_transform_get_format_metadata_v2(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_grading_primary_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_grading_primary_transform_get_style(handle: *mut c_void) -> i32;
    pub fn ocio_grading_primary_transform_set_style(handle: *mut c_void, style: i32) -> ();
    pub fn ocio_grading_primary_transform_get_value(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_grading_primary_transform_set_value(handle: *mut c_void, values: *mut c_void)
        -> ();
    pub fn ocio_grading_primary_transform_copy_value(
        handle: *mut c_void,
        values: *mut f64,
        len: usize,
    ) -> bool;
    pub fn ocio_grading_primary_transform_set_value_from_f64(
        handle: *mut c_void,
        values: *const f64,
        len: usize,
    ) -> bool;
    pub fn ocio_grading_primary_transform_is_dynamic(handle: *mut c_void) -> bool;
    pub fn ocio_grading_primary_transform_make_dynamic(handle: *mut c_void) -> ();
    pub fn ocio_grading_primary_transform_make_non_dynamic(handle: *mut c_void) -> ();

    // --- GradingRGBCurveTransform ---
    pub fn ocio_grading_rgb_curve_transform_create() -> *mut c_void;
    pub fn ocio_grading_rgb_curve_transform_create_with_style(style: i32) -> *mut c_void;
    pub fn ocio_grading_rgb_curve_transform_destroy(handle: *mut c_void);
    pub fn ocio_grading_rgb_curve_transform_get_format_metadata(handle: *mut c_void)
        -> *mut c_void;
    pub fn ocio_grading_rgb_curve_transform_get_format_metadata_v1(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_grading_rgb_curve_transform_get_format_metadata_v2(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_grading_rgb_curve_transform_equals(handle: *mut c_void, other: *mut c_void)
        -> bool;
    pub fn ocio_grading_rgb_curve_transform_get_direction(handle: *mut c_void) -> i32;
    pub fn ocio_grading_rgb_curve_transform_set_direction(handle: *mut c_void, dir: i32) -> ();
    pub fn ocio_grading_rgb_curve_transform_get_style(handle: *mut c_void) -> i32;
    pub fn ocio_grading_rgb_curve_transform_set_style(handle: *mut c_void, style: i32) -> ();
    pub fn ocio_grading_rgb_curve_transform_get_value(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_grading_rgb_curve_transform_set_value(
        handle: *mut c_void,
        values: *mut c_void,
    ) -> ();
    pub fn ocio_grading_rgb_curve_transform_get_num_control_points(
        handle: *mut c_void,
        c: i32,
    ) -> i32;
    pub fn ocio_grading_rgb_curve_transform_get_control_point(
        handle: *mut c_void,
        c: i32,
        index: i32,
        x: *mut f32,
        y: *mut f32,
    ) -> ();
    pub fn ocio_grading_rgb_curve_transform_set_num_control_points(
        handle: *mut c_void,
        c: i32,
        num: i32,
    ) -> ();
    pub fn ocio_grading_rgb_curve_transform_set_control_point(
        handle: *mut c_void,
        c: i32,
        index: i32,
        x: f32,
        y: f32,
    ) -> ();
    pub fn ocio_grading_rgb_curve_transform_get_slope(
        handle: *mut c_void,
        c: i32,
        index: usize,
    ) -> f32;
    pub fn ocio_grading_rgb_curve_transform_set_slope(
        handle: *mut c_void,
        c: i32,
        index: usize,
        slope: f32,
    ) -> ();
    pub fn ocio_grading_rgb_curve_transform_slopes_are_default(handle: *mut c_void, c: i32)
        -> bool;
    pub fn ocio_grading_rgb_curve_transform_get_bypass_lin_to_log(handle: *mut c_void) -> bool;
    pub fn ocio_grading_rgb_curve_transform_set_bypass_lin_to_log(
        handle: *mut c_void,
        bypass: bool,
    ) -> ();
    pub fn ocio_grading_rgb_curve_transform_is_dynamic(handle: *mut c_void) -> bool;
    pub fn ocio_grading_rgb_curve_transform_make_dynamic(handle: *mut c_void) -> ();
    pub fn ocio_grading_rgb_curve_transform_make_non_dynamic(handle: *mut c_void) -> ();

    // --- GradingHueCurveTransform ---
    pub fn ocio_grading_hue_curve_transform_create() -> *mut c_void;
    pub fn ocio_grading_hue_curve_transform_create_with_style(style: i32) -> *mut c_void;
    pub fn ocio_grading_hue_curve_transform_destroy(handle: *mut c_void);
    pub fn ocio_grading_hue_curve_transform_get_format_metadata(handle: *mut c_void)
        -> *mut c_void;
    pub fn ocio_grading_hue_curve_transform_get_format_metadata_v1(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_grading_hue_curve_transform_get_format_metadata_v2(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_grading_hue_curve_transform_equals(handle: *mut c_void, other: *mut c_void)
        -> bool;
    pub fn ocio_grading_hue_curve_transform_get_direction(handle: *mut c_void) -> i32;
    pub fn ocio_grading_hue_curve_transform_set_direction(handle: *mut c_void, dir: i32) -> ();
    pub fn ocio_grading_hue_curve_transform_get_style(handle: *mut c_void) -> i32;
    pub fn ocio_grading_hue_curve_transform_set_style(handle: *mut c_void, style: i32) -> ();
    pub fn ocio_grading_hue_curve_transform_get_value(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_grading_hue_curve_transform_set_value(
        handle: *mut c_void,
        value: *mut c_void,
    ) -> ();
    pub fn ocio_grading_hue_curve_transform_get_num_control_points(
        handle: *mut c_void,
        c: i32,
    ) -> i32;
    pub fn ocio_grading_hue_curve_transform_get_control_point(
        handle: *mut c_void,
        c: i32,
        index: i32,
        x: *mut f32,
        y: *mut f32,
    ) -> ();
    pub fn ocio_grading_hue_curve_transform_set_num_control_points(
        handle: *mut c_void,
        c: i32,
        num: i32,
    ) -> ();
    pub fn ocio_grading_hue_curve_transform_set_control_point(
        handle: *mut c_void,
        c: i32,
        index: i32,
        x: f32,
        y: f32,
    ) -> ();
    pub fn ocio_grading_hue_curve_transform_get_slope(
        handle: *mut c_void,
        c: i32,
        index: usize,
    ) -> f32;
    pub fn ocio_grading_hue_curve_transform_set_slope(
        handle: *mut c_void,
        c: i32,
        index: usize,
        slope: f32,
    ) -> ();
    pub fn ocio_grading_hue_curve_transform_slopes_are_default(handle: *mut c_void, c: i32)
        -> bool;
    pub fn ocio_grading_hue_curve_transform_get_rgb_to_hsy(handle: *mut c_void) -> i32;
    pub fn ocio_grading_hue_curve_transform_set_rgb_to_hsy(handle: *mut c_void, style: i32) -> ();
    pub fn ocio_grading_hue_curve_transform_is_dynamic(handle: *mut c_void) -> bool;
    pub fn ocio_grading_hue_curve_transform_make_dynamic(handle: *mut c_void) -> ();
    pub fn ocio_grading_hue_curve_transform_make_non_dynamic(handle: *mut c_void) -> ();

    // --- GradingToneTransform ---
    pub fn ocio_grading_tone_transform_create() -> *mut c_void;
    pub fn ocio_grading_tone_transform_create_with_style(style: i32) -> *mut c_void;
    pub fn ocio_grading_tone_transform_destroy(handle: *mut c_void);
    pub fn ocio_grading_tone_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_grading_tone_transform_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_grading_tone_transform_get_format_metadata_v2(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_grading_tone_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_grading_tone_transform_get_style(handle: *mut c_void) -> i32;
    pub fn ocio_grading_tone_transform_set_style(handle: *mut c_void, style: i32) -> ();
    pub fn ocio_grading_tone_transform_get_value(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_grading_tone_transform_set_value(handle: *mut c_void, values: *mut c_void) -> ();
    pub fn ocio_grading_tone_transform_copy_value(
        handle: *mut c_void,
        values: *mut f64,
        len: usize,
    ) -> bool;
    pub fn ocio_grading_tone_transform_set_value_from_f64(
        handle: *mut c_void,
        values: *const f64,
        len: usize,
    ) -> bool;
    pub fn ocio_grading_tone_transform_is_dynamic(handle: *mut c_void) -> bool;
    pub fn ocio_grading_tone_transform_make_dynamic(handle: *mut c_void) -> ();
    pub fn ocio_grading_tone_transform_make_non_dynamic(handle: *mut c_void) -> ();

    // --- GroupTransform ---
    pub fn ocio_group_transform_create() -> *mut c_void;
    pub fn ocio_group_transform_destroy(handle: *mut c_void);
    pub fn ocio_group_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_group_transform_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_group_transform_get_format_metadata_v2(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_group_transform_get_transform(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_group_transform_get_transform_v1(handle: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_group_transform_get_num_transforms(handle: *mut c_void) -> i32;
    pub fn ocio_group_transform_append_transform(handle: *mut c_void, transform: *mut c_void)
        -> ();
    pub fn ocio_group_transform_prepend_transform(
        handle: *mut c_void,
        transform: *mut c_void,
    ) -> ();
    pub fn ocio_group_transform_write(
        handle: *mut c_void,
        config: *mut c_void,
        formatName: *const i8,
        os: *mut c_void,
    ) -> ();
    pub fn ocio_group_transform_write_to_string(
        handle: *mut c_void,
        config: *mut c_void,
        formatName: *const i8,
    ) -> *mut c_void;
    pub fn ocio_group_transform_get_num_write_formats() -> i32;
    pub fn ocio_group_transform_get_format_name_by_index(index: i32) -> *const i8;
    pub fn ocio_group_transform_get_format_extension_by_index(index: i32) -> *const i8;

    // --- LogAffineTransform ---
    pub fn ocio_log_affine_transform_create() -> *mut c_void;
    pub fn ocio_log_affine_transform_destroy(handle: *mut c_void);
    pub fn ocio_log_affine_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_log_affine_transform_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_log_affine_transform_get_format_metadata_v2(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_log_affine_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_log_affine_transform_get_base(handle: *mut c_void) -> f64;
    pub fn ocio_log_affine_transform_set_base(handle: *mut c_void, base: f64) -> ();

    // --- LogCameraTransform ---
    pub fn ocio_log_camera_transform_create() -> *mut c_void;
    pub fn ocio_log_camera_transform_create_with_lin_side_break(values: *const f64) -> *mut c_void;
    pub fn ocio_log_camera_transform_destroy(handle: *mut c_void);
    pub fn ocio_log_camera_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_log_camera_transform_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_log_camera_transform_get_format_metadata_v2(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_log_camera_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_log_camera_transform_get_direction(handle: *mut c_void) -> i32;
    pub fn ocio_log_camera_transform_set_direction(handle: *mut c_void, dir: i32) -> ();
    pub fn ocio_log_camera_transform_get_base(handle: *mut c_void) -> f64;
    pub fn ocio_log_camera_transform_set_base(handle: *mut c_void, base: f64) -> ();
    pub fn ocio_log_camera_transform_get_log_side_slope_value(
        handle: *mut c_void,
        values: *mut f64,
    ) -> ();
    pub fn ocio_log_camera_transform_set_log_side_slope_value(
        handle: *mut c_void,
        values: *const f64,
    ) -> ();
    pub fn ocio_log_camera_transform_get_log_side_offset_value(
        handle: *mut c_void,
        values: *mut f64,
    ) -> ();
    pub fn ocio_log_camera_transform_set_log_side_offset_value(
        handle: *mut c_void,
        values: *const f64,
    ) -> ();
    pub fn ocio_log_camera_transform_get_lin_side_slope_value(
        handle: *mut c_void,
        values: *mut f64,
    ) -> ();
    pub fn ocio_log_camera_transform_set_lin_side_slope_value(
        handle: *mut c_void,
        values: *const f64,
    ) -> ();
    pub fn ocio_log_camera_transform_get_lin_side_offset_value(
        handle: *mut c_void,
        values: *mut f64,
    ) -> ();
    pub fn ocio_log_camera_transform_set_lin_side_offset_value(
        handle: *mut c_void,
        values: *const f64,
    ) -> ();
    pub fn ocio_log_camera_transform_get_lin_side_break_value(
        handle: *mut c_void,
        values: *mut f64,
    ) -> ();
    pub fn ocio_log_camera_transform_set_lin_side_break_value(
        handle: *mut c_void,
        values: *const f64,
    ) -> ();
    pub fn ocio_log_camera_transform_get_linear_slope_value(
        handle: *mut c_void,
        values: *mut f64,
    ) -> bool;
    pub fn ocio_log_camera_transform_set_linear_slope_value(
        handle: *mut c_void,
        values: *const f64,
    ) -> ();
    pub fn ocio_log_camera_transform_unset_linear_slope_value(handle: *mut c_void) -> ();

    // --- LogTransform ---
    pub fn ocio_log_transform_create() -> *mut c_void;
    pub fn ocio_log_transform_destroy(handle: *mut c_void);
    pub fn ocio_log_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_log_transform_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_log_transform_get_format_metadata_v2(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_log_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_log_transform_get_base(handle: *mut c_void) -> f64;
    pub fn ocio_log_transform_set_base(handle: *mut c_void, val: f64) -> ();

    // --- LookTransform ---
    pub fn ocio_look_transform_create() -> *mut c_void;
    pub fn ocio_look_transform_destroy(handle: *mut c_void);
    pub fn ocio_look_transform_create_editable_copy(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_look_transform_get_direction(handle: *mut c_void) -> i32;
    pub fn ocio_look_transform_set_direction(handle: *mut c_void, dir: i32) -> ();
    pub fn ocio_look_transform_validate(handle: *mut c_void) -> ();
    pub fn ocio_look_transform_get_src(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_look_transform_set_src(handle: *mut c_void, src: *const i8) -> ();
    pub fn ocio_look_transform_get_dst(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_look_transform_set_dst(handle: *mut c_void, dst: *const i8) -> ();
    pub fn ocio_look_transform_get_looks(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_look_transform_set_looks(handle: *mut c_void, looks: *const i8) -> ();
    pub fn ocio_look_transform_get_skip_color_space_conversion(handle: *mut c_void) -> bool;
    pub fn ocio_look_transform_set_skip_color_space_conversion(
        handle: *mut c_void,
        skip: bool,
    ) -> ();

    // --- Lut1DTransform ---
    pub fn ocio_lut1d_transform_create() -> *mut c_void;
    pub fn ocio_lut1d_transform_destroy(handle: *mut c_void);
    pub fn ocio_lut1d_transform_get_file_output_bit_depth(handle: *mut c_void) -> i32;
    pub fn ocio_lut1d_transform_set_file_output_bit_depth(handle: *mut c_void, bitDepth: i32)
        -> ();
    pub fn ocio_lut1d_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_lut1d_transform_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_lut1d_transform_get_format_metadata_v2(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_lut1d_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_lut1d_transform_get_length(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_lut1d_transform_set_length(handle: *mut c_void, length: *mut c_void) -> ();
    pub fn ocio_lut1d_transform_get_length_u64(handle: *mut c_void) -> u64;
    pub fn ocio_lut1d_transform_set_length_u64(handle: *mut c_void, length: u64) -> ();
    pub fn ocio_lut1d_transform_get_value(
        handle: *mut c_void,
        index: *mut c_void,
        r: *mut c_void,
        g: *mut c_void,
        b: *mut c_void,
    ) -> ();
    pub fn ocio_lut1d_transform_set_value(
        handle: *mut c_void,
        index: *mut c_void,
        r: f32,
        g: f32,
        b: f32,
    ) -> ();
    pub fn ocio_lut1d_transform_get_input_half_domain(handle: *mut c_void) -> bool;
    pub fn ocio_lut1d_transform_set_input_half_domain(
        handle: *mut c_void,
        isHalfDomain: bool,
    ) -> ();
    pub fn ocio_lut1d_transform_get_output_raw_halfs(handle: *mut c_void) -> bool;
    pub fn ocio_lut1d_transform_set_output_raw_halfs(handle: *mut c_void, isRawHalfs: bool) -> ();
    pub fn ocio_lut1d_transform_get_hue_adjust(handle: *mut c_void) -> i32;
    pub fn ocio_lut1d_transform_set_hue_adjust(handle: *mut c_void, algo: i32) -> ();
    pub fn ocio_lut1d_transform_get_interpolation(handle: *mut c_void) -> i32;
    pub fn ocio_lut1d_transform_set_interpolation(handle: *mut c_void, algo: i32) -> ();

    // --- Lut3DTransform ---
    pub fn ocio_lut3d_transform_create() -> *mut c_void;
    pub fn ocio_lut3d_transform_destroy(handle: *mut c_void);
    pub fn ocio_lut3d_transform_get_file_output_bit_depth(handle: *mut c_void) -> i32;
    pub fn ocio_lut3d_transform_set_file_output_bit_depth(handle: *mut c_void, bitDepth: i32)
        -> ();
    pub fn ocio_lut3d_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_lut3d_transform_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_lut3d_transform_get_format_metadata_v2(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_lut3d_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_lut3d_transform_get_grid_size(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_lut3d_transform_set_grid_size(handle: *mut c_void, gridSize: *mut c_void) -> ();
    pub fn ocio_lut3d_transform_get_grid_size_u64(handle: *mut c_void) -> u64;
    pub fn ocio_lut3d_transform_set_grid_size_u64(handle: *mut c_void, gridSize: u64) -> ();
    pub fn ocio_lut3d_transform_get_value(
        handle: *mut c_void,
        indexR: *mut c_void,
        indexG: *mut c_void,
        indexB: *mut c_void,
        r: *mut c_void,
        g: *mut c_void,
        b: *mut c_void,
    ) -> ();
    pub fn ocio_lut3d_transform_set_value(
        handle: *mut c_void,
        indexR: *mut c_void,
        indexG: *mut c_void,
        indexB: *mut c_void,
        r: f32,
        g: f32,
        b: f32,
    ) -> ();
    pub fn ocio_lut3d_transform_get_interpolation(handle: *mut c_void) -> i32;
    pub fn ocio_lut3d_transform_set_interpolation(handle: *mut c_void, algo: i32) -> ();

    // --- MatrixTransform ---
    pub fn ocio_matrix_transform_create() -> *mut c_void;
    pub fn ocio_matrix_transform_destroy(handle: *mut c_void);
    pub fn ocio_matrix_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_matrix_transform_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_matrix_transform_get_format_metadata_v2(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_matrix_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_matrix_transform_get_matrix(handle: *mut c_void, m44: *mut c_void) -> ();
    pub fn ocio_matrix_transform_set_matrix(handle: *mut c_void, m44: *mut c_void) -> ();
    pub fn ocio_matrix_transform_get_offset(handle: *mut c_void, offset4: *mut c_void) -> ();
    pub fn ocio_matrix_transform_set_offset(handle: *mut c_void, offset4: *mut c_void) -> ();
    pub fn ocio_matrix_transform_get_file_input_bit_depth(handle: *mut c_void) -> i32;
    pub fn ocio_matrix_transform_set_file_input_bit_depth(handle: *mut c_void, bitDepth: i32)
        -> ();
    pub fn ocio_matrix_transform_get_file_output_bit_depth(handle: *mut c_void) -> i32;
    pub fn ocio_matrix_transform_set_file_output_bit_depth(
        handle: *mut c_void,
        bitDepth: i32,
    ) -> ();

    // --- RangeTransform ---
    pub fn ocio_range_transform_create() -> *mut c_void;
    pub fn ocio_range_transform_destroy(handle: *mut c_void);
    pub fn ocio_range_transform_get_format_metadata(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_range_transform_get_style(handle: *mut c_void) -> i32;
    pub fn ocio_range_transform_set_style(handle: *mut c_void, style: i32) -> ();
    pub fn ocio_range_transform_get_format_metadata_v1(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_range_transform_get_format_metadata_v2(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_range_transform_equals(handle: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_range_transform_get_file_input_bit_depth(handle: *mut c_void) -> i32;
    pub fn ocio_range_transform_set_file_input_bit_depth(handle: *mut c_void, bitDepth: i32) -> ();
    pub fn ocio_range_transform_get_file_output_bit_depth(handle: *mut c_void) -> i32;
    pub fn ocio_range_transform_set_file_output_bit_depth(handle: *mut c_void, bitDepth: i32)
        -> ();
    pub fn ocio_range_transform_get_min_in_value(handle: *mut c_void) -> f64;
    pub fn ocio_range_transform_set_min_in_value(handle: *mut c_void, val: f64) -> ();
    pub fn ocio_range_transform_has_min_in_value(handle: *mut c_void) -> bool;
    pub fn ocio_range_transform_unset_min_in_value(handle: *mut c_void) -> ();
    pub fn ocio_range_transform_set_max_in_value(handle: *mut c_void, val: f64) -> ();
    pub fn ocio_range_transform_get_max_in_value(handle: *mut c_void) -> f64;
    pub fn ocio_range_transform_has_max_in_value(handle: *mut c_void) -> bool;
    pub fn ocio_range_transform_unset_max_in_value(handle: *mut c_void) -> ();
    pub fn ocio_range_transform_set_min_out_value(handle: *mut c_void, val: f64) -> ();
    pub fn ocio_range_transform_get_min_out_value(handle: *mut c_void) -> f64;
    pub fn ocio_range_transform_has_min_out_value(handle: *mut c_void) -> bool;
    pub fn ocio_range_transform_unset_min_out_value(handle: *mut c_void) -> ();
    pub fn ocio_range_transform_set_max_out_value(handle: *mut c_void, val: f64) -> ();
    pub fn ocio_range_transform_get_max_out_value(handle: *mut c_void) -> f64;
    pub fn ocio_range_transform_has_max_out_value(handle: *mut c_void) -> bool;
    pub fn ocio_range_transform_unset_max_out_value(handle: *mut c_void) -> ();

    // --- DynamicProperty ---
    pub fn ocio_dynamic_property_destroy(handle: *mut c_void);
    pub fn ocio_dynamic_property_get_type(handle: *mut c_void) -> i32;
}

// Additional real C ABI declarations kept out of the generated block while the bridge is consolidated.
unsafe extern "C" {
    pub fn ocio_baker_create_editable_copy(baker: *mut c_void) -> *mut c_void;

    pub fn ocio_baker_get_format_extension_by_index(index: i32) -> *const i8;

    pub fn ocio_baker_get_format_name_by_index(index: i32) -> *const i8;

    pub fn ocio_baker_get_num_formats() -> i32;

    pub fn ocio_builtin_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_builtin_transform_get_num_styles() -> i32;

    pub fn ocio_builtin_transform_get_style_by_index(index: i32) -> *const i8;

    pub fn ocio_builtin_transform_is_valid_style(style: *const i8) -> bool;

    pub fn ocio_builtin_transform_set_direction(transform: *mut c_void, direction: i32) -> ();

    pub fn ocio_cdl_transform_create_from_file(src: *const i8, cccId: *const i8) -> *mut c_void;

    pub fn ocio_cdl_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_cdl_transform_set_direction(transform: *mut c_void, direction: i32) -> ();

    pub fn ocio_color_space_create_editable_copy(colorSpace: *mut c_void) -> *mut c_void;

    pub fn ocio_color_space_is_transform_defined(colorSpace: *mut c_void, direction: i32) -> bool;

    pub fn ocio_color_space_set_create_editable_copy(set: *mut c_void) -> *mut c_void;

    pub fn ocio_config_add_display(
        config: *mut c_void,
        display: *const i8,
        view: *const i8,
        transformName: *const i8,
        rule: *const i8,
    ) -> ();

    pub fn ocio_config_create_editable_copy(config: *mut c_void) -> *mut c_void;

    pub fn ocio_context_create_editable_copy(context: *mut c_void) -> *mut c_void;

    pub fn ocio_cpu_processor_apply_rgb_packed(
        cpu_processor: *mut c_void,
        rgb: *mut c_void,
        bitDepth: i32,
        numPixels: i64,
        stride: i64,
    ) -> ();

    pub fn ocio_cpu_processor_apply_rgb_pixels(
        cpu_processor: *mut c_void,
        rgb: *mut f32,
        numPixels: i64,
        stride: i64,
    ) -> ();

    pub fn ocio_cpu_processor_apply_rgba_packed(
        cpu_processor: *mut c_void,
        rgba: *mut c_void,
        bitDepth: i32,
        numPixels: i64,
        stride: i64,
    ) -> ();

    pub fn ocio_cpu_processor_apply_rgba_pixels(
        cpu_processor: *mut c_void,
        rgba: *mut f32,
        numPixels: i64,
        stride: i64,
    ) -> ();

    pub fn ocio_dynamic_property_double_get_value(prop: *mut c_void) -> f64;

    pub fn ocio_dynamic_property_double_set_value(prop: *mut c_void, value: f64) -> ();

    pub fn ocio_dynamic_property_grading_hue_curve_get_control_point(
        prop: *mut c_void,
        curveType: i32,
        index: i32,
        x: *mut f32,
        y: *mut f32,
    ) -> ();

    pub fn ocio_dynamic_property_grading_hue_curve_get_num_control_points(
        prop: *mut c_void,
        curveType: i32,
    ) -> i32;

    pub fn ocio_dynamic_property_grading_hue_curve_get_slope(
        prop: *mut c_void,
        curveType: i32,
        index: i32,
    ) -> f32;

    pub fn ocio_dynamic_property_grading_hue_curve_set_control_point(
        prop: *mut c_void,
        curveType: i32,
        index: i32,
        x: f32,
        y: f32,
    ) -> ();

    pub fn ocio_dynamic_property_grading_hue_curve_set_num_control_points(
        prop: *mut c_void,
        curveType: i32,
        num: i32,
    ) -> ();

    pub fn ocio_dynamic_property_grading_hue_curve_set_slope(
        prop: *mut c_void,
        curveType: i32,
        index: i32,
        slope: f32,
    ) -> ();

    pub fn ocio_dynamic_property_grading_hue_curve_slopes_are_default(
        prop: *mut c_void,
        curveType: i32,
    ) -> bool;

    pub fn ocio_dynamic_property_grading_primary_get_value(
        prop: *mut c_void,
        values: *mut f64,
    ) -> ();

    pub fn ocio_dynamic_property_grading_primary_set_value(
        prop: *mut c_void,
        values: *const f64,
    ) -> ();

    pub fn ocio_dynamic_property_grading_rgb_curve_get_control_point(
        prop: *mut c_void,
        curveType: i32,
        index: i32,
        x: *mut f32,
        y: *mut f32,
    ) -> ();

    pub fn ocio_dynamic_property_grading_rgb_curve_get_num_control_points(
        prop: *mut c_void,
        curveType: i32,
    ) -> i32;

    pub fn ocio_dynamic_property_grading_rgb_curve_get_slope(
        prop: *mut c_void,
        curveType: i32,
        index: i32,
    ) -> f32;

    pub fn ocio_dynamic_property_grading_rgb_curve_set_control_point(
        prop: *mut c_void,
        curveType: i32,
        index: i32,
        x: f32,
        y: f32,
    ) -> ();

    pub fn ocio_dynamic_property_grading_rgb_curve_set_num_control_points(
        prop: *mut c_void,
        curveType: i32,
        num: i32,
    ) -> ();

    pub fn ocio_dynamic_property_grading_rgb_curve_set_slope(
        prop: *mut c_void,
        curveType: i32,
        index: i32,
        slope: f32,
    ) -> ();

    pub fn ocio_dynamic_property_grading_rgb_curve_slopes_are_default(
        prop: *mut c_void,
        curveType: i32,
    ) -> bool;

    pub fn ocio_dynamic_property_grading_tone_get_value(prop: *mut c_void, values: *mut f64) -> ();

    pub fn ocio_dynamic_property_grading_tone_set_value(
        prop: *mut c_void,
        values: *const f64,
    ) -> ();

    pub fn ocio_exponent_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_exponent_transform_get_value(transform: *mut c_void, vec4: *mut f64) -> ();

    pub fn ocio_exponent_transform_set_direction(transform: *mut c_void, direction: i32) -> ();

    pub fn ocio_exponent_transform_set_value(transform: *mut c_void, vec4: *const f64) -> ();

    pub fn ocio_exponent_with_linear_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_exponent_with_linear_transform_get_gamma(
        transform: *mut c_void,
        vec4: *mut f64,
    ) -> ();

    pub fn ocio_exponent_with_linear_transform_get_offset(
        transform: *mut c_void,
        vec4: *mut f64,
    ) -> ();

    pub fn ocio_exponent_with_linear_transform_set_direction(
        transform: *mut c_void,
        direction: i32,
    ) -> ();

    pub fn ocio_exponent_with_linear_transform_set_gamma(
        transform: *mut c_void,
        vec4: *const f64,
    ) -> ();

    pub fn ocio_exponent_with_linear_transform_set_offset(
        transform: *mut c_void,
        vec4: *const f64,
    ) -> ();

    pub fn ocio_exposure_contrast_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_exposure_contrast_transform_set_direction(
        transform: *mut c_void,
        direction: i32,
    ) -> ();

    pub fn ocio_file_rules_create_editable_copy(rules: *mut c_void) -> *mut c_void;

    pub fn ocio_format_metadata_add_attribute(
        metadata: *mut c_void,
        name: *const i8,
        value: *const i8,
    ) -> ();

    pub fn ocio_format_metadata_add_child_element(
        metadata: *mut c_void,
        name: *const i8,
        value: *const i8,
    ) -> ();

    pub fn ocio_format_metadata_clear(metadata: *mut c_void) -> ();

    pub fn ocio_format_metadata_destroy(handle: *mut c_void) -> ();

    pub fn ocio_format_metadata_get_attribute_name(metadata: *mut c_void, i: i32) -> *const i8;

    pub fn ocio_format_metadata_get_attribute_value_by_index(
        metadata: *mut c_void,
        i: i32,
    ) -> *const i8;

    pub fn ocio_format_metadata_get_attribute_value(
        metadata: *mut c_void,
        name: *const i8,
    ) -> *const i8;

    pub fn ocio_format_metadata_get_child_element(metadata: *mut c_void, i: i32) -> *mut c_void;

    pub fn ocio_format_metadata_get_element_name(metadata: *mut c_void) -> *const i8;

    pub fn ocio_format_metadata_get_element_value(metadata: *mut c_void) -> *const i8;

    pub fn ocio_format_metadata_get_id(metadata: *mut c_void) -> *const i8;

    pub fn ocio_format_metadata_get_name(metadata: *mut c_void) -> *const i8;

    pub fn ocio_format_metadata_get_num_attributes(metadata: *mut c_void) -> i32;

    pub fn ocio_format_metadata_get_num_children_elements(metadata: *mut c_void) -> i32;

    pub fn ocio_format_metadata_set_element_name(metadata: *mut c_void, name: *const i8) -> ();

    pub fn ocio_format_metadata_set_element_value(metadata: *mut c_void, value: *const i8) -> ();

    pub fn ocio_format_metadata_set_id(metadata: *mut c_void, id: *const i8) -> ();

    pub fn ocio_format_metadata_set_name(metadata: *mut c_void, name: *const i8) -> ();

    pub fn ocio_gpu_shader_desc_finalize(shader_desc: *mut c_void) -> ();

    pub fn ocio_gpu_shader_desc_get_cache_id(desc: *mut c_void) -> *const i8;

    pub fn ocio_gpu_shader_desc_get_function_name(shader_desc: *mut c_void) -> *const i8;

    pub fn ocio_gpu_shader_desc_get_language(shader_desc: *mut c_void) -> i32;

    pub fn ocio_gpu_shader_desc_get_pixel_name(shader_desc: *mut c_void) -> *const i8;

    pub fn ocio_gpu_shader_desc_get_resource_prefix(shader_desc: *mut c_void) -> *const i8;

    pub fn ocio_gpu_shader_desc_get_texture_max_height(desc: *mut c_void, index: i32) -> u32;

    pub fn ocio_gpu_shader_desc_get_texture_max_width(desc: *mut c_void, index: i32) -> u32;

    pub fn ocio_gpu_shader_desc_get_texture_uid(desc: *mut c_void, index: i32) -> *const i8;

    pub fn ocio_gpu_shader_desc_set_function_name(shader_desc: *mut c_void, name: *const i8) -> ();

    pub fn ocio_gpu_shader_desc_set_language(shader_desc: *mut c_void, language: i32) -> ();

    pub fn ocio_gpu_shader_desc_set_pixel_name(shader_desc: *mut c_void, name: *const i8) -> ();

    pub fn ocio_gpu_shader_desc_set_resource_prefix(
        shader_desc: *mut c_void,
        prefix: *const i8,
    ) -> ();

    pub fn ocio_grading_primary_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_grading_primary_transform_set_direction(
        transform: *mut c_void,
        direction: i32,
    ) -> ();

    pub fn ocio_grading_tone_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_grading_tone_transform_set_direction(transform: *mut c_void, direction: i32) -> ();

    pub fn ocio_group_transform_clear_transforms(transform: *mut c_void) -> ();

    pub fn ocio_group_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_group_transform_remove_transform(transform: *mut c_void, index: u64) -> ();

    pub fn ocio_group_transform_set_direction(transform: *mut c_void, direction: i32) -> ();

    pub fn ocio_log_affine_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_log_affine_transform_get_lin_side_offset_value(
        transform: *mut c_void,
        values: *mut f64,
    ) -> ();

    pub fn ocio_log_affine_transform_get_lin_side_slope_value(
        transform: *mut c_void,
        values: *mut f64,
    ) -> ();

    pub fn ocio_log_affine_transform_get_log_side_offset_value(
        transform: *mut c_void,
        values: *mut f64,
    ) -> ();

    pub fn ocio_log_affine_transform_get_log_side_slope_value(
        transform: *mut c_void,
        values: *mut f64,
    ) -> ();

    pub fn ocio_log_affine_transform_set_direction(transform: *mut c_void, direction: i32) -> ();

    pub fn ocio_log_affine_transform_set_lin_side_offset_value(
        transform: *mut c_void,
        values: *const f64,
    ) -> ();

    pub fn ocio_log_affine_transform_set_lin_side_slope_value(
        transform: *mut c_void,
        values: *const f64,
    ) -> ();

    pub fn ocio_log_affine_transform_set_log_side_offset_value(
        transform: *mut c_void,
        values: *const f64,
    ) -> ();

    pub fn ocio_log_affine_transform_set_log_side_slope_value(
        transform: *mut c_void,
        values: *const f64,
    ) -> ();

    pub fn ocio_log_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_log_transform_set_direction(transform: *mut c_void, direction: i32) -> ();

    pub fn ocio_look_create_editable_copy(look: *mut c_void) -> *mut c_void;

    pub fn ocio_lut1d_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_lut1d_transform_get_values(transform: *mut c_void, data: *mut f64) -> ();

    pub fn ocio_lut1d_transform_set_direction(transform: *mut c_void, direction: i32) -> ();

    pub fn ocio_lut1d_transform_set_values(transform: *mut c_void, data: *const f64) -> ();

    pub fn ocio_lut3d_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_lut3d_transform_get_values(transform: *mut c_void, data: *mut f64) -> ();

    pub fn ocio_lut3d_transform_set_direction(transform: *mut c_void, direction: i32) -> ();

    pub fn ocio_lut3d_transform_set_values(transform: *mut c_void, data: *const f64) -> ();

    pub fn ocio_matrix_transform_create_fit(
        oldMin4: *const f64,
        oldMax4: *const f64,
        newMin4: *const f64,
        newMax4: *const f64,
    ) -> *mut c_void;

    pub fn ocio_matrix_transform_create_identity() -> *mut c_void;

    pub fn ocio_matrix_transform_identity(m44: *mut f64, offset4: *mut f64) -> ();

    pub fn ocio_matrix_transform_create_sat(sat: f64, luma: *const f64) -> *mut c_void;

    pub fn ocio_matrix_transform_create_scale(scale: *const f64) -> *mut c_void;

    pub fn ocio_matrix_transform_create_view(channels: *mut i32, luma: *const f64) -> *mut c_void;

    pub fn ocio_matrix_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_matrix_transform_set_direction(transform: *mut c_void, direction: i32) -> ();

    pub fn ocio_named_transform_create_editable_copy(namedTransform: *mut c_void) -> *mut c_void;

    pub fn ocio_range_transform_get_direction(transform: *mut c_void) -> i32;

    pub fn ocio_range_transform_set_direction(transform: *mut c_void, direction: i32) -> ();

    pub fn ocio_transform_get_format_metadata(transform: *mut c_void) -> *mut c_void;
}
