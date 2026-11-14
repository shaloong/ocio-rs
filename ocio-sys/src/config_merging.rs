use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ocio_config_merging_parameters_create() -> *mut c_void;
    pub fn ocio_config_merging_parameters_destroy(handle: *mut c_void);
    pub fn ocio_config_merging_parameters_create_editable_copy(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_config_merging_parameters_get_base_config_name(
        handle: *mut c_void,
    ) -> *const c_char;
    pub fn ocio_config_merging_parameters_set_base_config_name(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_get_input_config_name(
        handle: *mut c_void,
    ) -> *const c_char;
    pub fn ocio_config_merging_parameters_set_input_config_name(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_get_output_name(handle: *mut c_void) -> *const c_char;
    pub fn ocio_config_merging_parameters_set_output_name(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_get_input_family_prefix(
        handle: *mut c_void,
    ) -> *const c_char;
    pub fn ocio_config_merging_parameters_set_input_family_prefix(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_get_base_family_prefix(
        handle: *mut c_void,
    ) -> *const c_char;
    pub fn ocio_config_merging_parameters_set_base_family_prefix(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_get_name(handle: *mut c_void) -> *const c_char;
    pub fn ocio_config_merging_parameters_set_name(handle: *mut c_void, value: *const c_char);
    pub fn ocio_config_merging_parameters_get_description(handle: *mut c_void) -> *const c_char;
    pub fn ocio_config_merging_parameters_set_description(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_get_search_path(handle: *mut c_void) -> *const c_char;
    pub fn ocio_config_merging_parameters_set_search_path(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_get_active_displays(handle: *mut c_void)
        -> *const c_char;
    pub fn ocio_config_merging_parameters_set_active_displays(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_get_active_views(handle: *mut c_void) -> *const c_char;
    pub fn ocio_config_merging_parameters_set_active_views(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_get_inactive_color_spaces(
        handle: *mut c_void,
    ) -> *const c_char;
    pub fn ocio_config_merging_parameters_set_inactive_color_spaces(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_is_input_first(handle: *mut c_void) -> bool;
    pub fn ocio_config_merging_parameters_set_input_first(handle: *mut c_void, value: bool);
    pub fn ocio_config_merging_parameters_is_error_on_conflict(handle: *mut c_void) -> bool;
    pub fn ocio_config_merging_parameters_set_error_on_conflict(handle: *mut c_void, value: bool);
    pub fn ocio_config_merging_parameters_is_avoid_duplicates(handle: *mut c_void) -> bool;
    pub fn ocio_config_merging_parameters_set_avoid_duplicates(handle: *mut c_void, value: bool);
    pub fn ocio_config_merging_parameters_is_adjust_input_reference_space(
        handle: *mut c_void,
    ) -> bool;
    pub fn ocio_config_merging_parameters_set_adjust_input_reference_space(
        handle: *mut c_void,
        value: bool,
    );
    pub fn ocio_config_merging_parameters_get_default_strategy(handle: *mut c_void) -> i32;
    pub fn ocio_config_merging_parameters_set_default_strategy(handle: *mut c_void, value: i32);
    pub fn ocio_config_merging_parameters_get_roles(handle: *mut c_void) -> i32;
    pub fn ocio_config_merging_parameters_set_roles(handle: *mut c_void, value: i32);
    pub fn ocio_config_merging_parameters_get_file_rules(handle: *mut c_void) -> i32;
    pub fn ocio_config_merging_parameters_set_file_rules(handle: *mut c_void, value: i32);
    pub fn ocio_config_merging_parameters_get_display_views(handle: *mut c_void) -> i32;
    pub fn ocio_config_merging_parameters_set_display_views(handle: *mut c_void, value: i32);
    pub fn ocio_config_merging_parameters_get_view_transforms(handle: *mut c_void) -> i32;
    pub fn ocio_config_merging_parameters_set_view_transforms(handle: *mut c_void, value: i32);
    pub fn ocio_config_merging_parameters_get_looks(handle: *mut c_void) -> i32;
    pub fn ocio_config_merging_parameters_set_looks(handle: *mut c_void, value: i32);
    pub fn ocio_config_merging_parameters_get_color_spaces(handle: *mut c_void) -> i32;
    pub fn ocio_config_merging_parameters_set_color_spaces(handle: *mut c_void, value: i32);
    pub fn ocio_config_merging_parameters_get_named_transforms(handle: *mut c_void) -> i32;
    pub fn ocio_config_merging_parameters_set_named_transforms(handle: *mut c_void, value: i32);
    pub fn ocio_config_merging_parameters_add_search_path(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_add_environment_var(
        handle: *mut c_void,
        name: *const c_char,
        value: *const c_char,
    );
    pub fn ocio_config_merging_parameters_get_num_environment_vars(handle: *mut c_void) -> i32;
    pub fn ocio_config_merging_parameters_get_environment_var(
        handle: *mut c_void,
        index: i32,
    ) -> *const c_char;
    pub fn ocio_config_merging_parameters_get_environment_var_value(
        handle: *mut c_void,
        index: i32,
    ) -> *const c_char;
    pub fn ocio_config_merging_parameters_merge_configs(
        handle: *mut c_void,
        base: *mut c_void,
        input: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_config_merging_parameters_merge_color_space(
        handle: *mut c_void,
        base: *mut c_void,
        input: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_config_merger_create() -> *mut c_void;
    pub fn ocio_config_merger_from_file(path: *const c_char) -> *mut c_void;
    pub fn ocio_config_merger_create_editable_copy(h: *mut c_void) -> *mut c_void;
    pub fn ocio_config_merger_destroy(h: *mut c_void);
    pub fn ocio_config_merger_set_search_path(h: *mut c_void, v: *const c_char);
    pub fn ocio_config_merger_add_search_path(h: *mut c_void, v: *const c_char);
    pub fn ocio_config_merger_num_search_paths(h: *mut c_void) -> i32;
    pub fn ocio_config_merger_search_path(h: *mut c_void, index: i32) -> *const c_char;
    pub fn ocio_config_merger_set_working_dir(h: *mut c_void, v: *const c_char);
    pub fn ocio_config_merger_working_dir(h: *mut c_void) -> *const c_char;
    pub fn ocio_config_merger_num_params(h: *mut c_void) -> i32;
    pub fn ocio_config_merger_params(h: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_merger_add_params(h: *mut c_void, params: *mut c_void);
    pub fn ocio_config_merger_merge_configs(h: *mut c_void) -> *mut c_void;
    pub fn ocio_config_merger_num_merged_configs(h: *mut c_void) -> i32;
    pub fn ocio_config_merger_merged_config(h: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_merger_serialize(h: *mut c_void) -> *const c_char;
    pub fn ocio_config_merger_set_version(h: *mut c_void, major: u32, minor: u32);
    pub fn ocio_config_merger_major_version(h: *mut c_void) -> u32;
    pub fn ocio_config_merger_minor_version(h: *mut c_void) -> u32;
}
