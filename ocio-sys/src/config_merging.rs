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
    /// # Safety
    /// All pointers must be live handles from this ABI. Each flag must match
    /// whether its corresponding handle is a const or editable config. A
    /// non-null returned config is owned by the caller.
    pub fn ocio_config_merging_parameters_merge_config_refs(
        handle: *mut c_void,
        base: *mut c_void,
        base_const: bool,
        input: *mut c_void,
        input_const: bool,
    ) -> *mut c_void;
    /// # Safety
    /// All pointers must be live handles from this ABI. Flags identify the
    /// const or editable config and color-space handle representations. A
    /// non-null returned config is owned by the caller.
    pub fn ocio_config_merging_parameters_merge_color_space_refs(
        handle: *mut c_void,
        base: *mut c_void,
        base_const: bool,
        input: *mut c_void,
        input_const: bool,
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
    /// Retain the merger's mutable parameters, or return null for an invalid index.
    /// Release the returned handle with `ocio_config_merging_parameters_destroy`.
    /// Edits affect the merger and any sharing handles; callers must serialize
    /// access and invalidate all borrowed strings before mutating any alias.
    pub fn ocio_config_merger_params_shared(h: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_merger_add_params(h: *mut c_void, params: *mut c_void);
    pub fn ocio_config_merger_merge_configs(h: *mut c_void) -> *mut c_void;
    /// Returns an owned read-only merger handle; destroy it with
    /// `ocio_const_config_merger_destroy`.
    /// # Safety
    /// `h` must be a live editable merger handle from this ABI.
    pub fn ocio_config_merger_merge_configs_shared(h: *mut c_void) -> *mut c_void;
    /// # Safety
    /// `path` must be a valid C string. Destroy a non-null result using
    /// `ocio_const_config_merger_destroy`.
    pub fn ocio_const_config_merger_from_file(path: *const c_char) -> *mut c_void;
    /// # Safety
    /// `h` must be a live read-only merger handle from this ABI; a non-null
    /// result is an owned read-only merger handle.
    pub fn ocio_const_config_merger_merge_configs(h: *const c_void) -> *mut c_void;
    /// # Safety
    /// `h` must be null or an owned read-only merger handle, released once.
    pub fn ocio_const_config_merger_destroy(h: *mut c_void);
    /// # Safety
    /// `h` must be a live read-only merger handle from this ABI.
    pub fn ocio_const_config_merger_num_merged_configs(h: *const c_void) -> i32;
    /// # Safety
    /// `h` must be a live read-only merger handle. Destroy a non-null result
    /// with `ocio_const_config_destroy`.
    pub fn ocio_const_config_merger_merged_config(h: *const c_void, index: i32) -> *mut c_void;
    /// # Safety
    /// `h` must be a live read-only merger handle. Destroy a non-null result
    /// with `ocio_config_merger_destroy`.
    pub fn ocio_const_config_merger_editable_copy(h: *const c_void) -> *mut c_void;
    pub fn ocio_config_merger_num_merged_configs(h: *mut c_void) -> i32;
    pub fn ocio_config_merger_merged_config(h: *mut c_void, index: i32) -> *mut c_void;
    /// Retain a read-only result. Index `-1` selects the final result.
    /// Release the returned handle with `ocio_const_config_destroy`.
    pub fn ocio_config_merger_merged_const_config(h: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_config_merger_serialize(h: *mut c_void) -> *const c_char;
    pub fn ocio_config_merger_set_version(h: *mut c_void, major: u32, minor: u32);
    pub fn ocio_config_merger_major_version(h: *mut c_void) -> u32;
    pub fn ocio_config_merger_minor_version(h: *mut c_void) -> u32;
}
