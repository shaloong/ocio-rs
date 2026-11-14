use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ocio_config_canonical_display_name(
        handle: *mut c_void,
        display: *const c_char,
    ) -> *const c_char;
    pub fn ocio_config_canonical_view_name(
        handle: *mut c_void,
        display: *const c_char,
        view: *const c_char,
    ) -> *const c_char;
    pub fn ocio_config_resolved_display_view_color_space_name(
        handle: *mut c_void,
        display: *const c_char,
        view: *const c_char,
    ) -> *const c_char;
    pub fn ocio_config_display_description(
        handle: *mut c_void,
        display: *const c_char,
    ) -> *const c_char;
    pub fn ocio_config_display_view_alias(
        handle: *mut c_void,
        display: *const c_char,
        view: *const c_char,
        index: i32,
    ) -> *const c_char;
    pub fn ocio_config_generate_local_id_for_color_space(
        handle: *mut c_void,
        name: *const c_char,
    ) -> *const c_char;
    pub fn ocio_config_locate_builtin_color_space(
        handle: *mut c_void,
        name: *const c_char,
        builtin: *mut c_void,
    ) -> *const c_char;
    pub fn ocio_config_num_display_view_aliases(
        handle: *mut c_void,
        display: *const c_char,
        view: *const c_char,
    ) -> i32;
    pub fn ocio_config_has_display_view_alias(
        handle: *mut c_void,
        display: *const c_char,
        view: *const c_char,
        alias: *const c_char,
    ) -> bool;
    pub fn ocio_config_use_display_aliases(handle: *mut c_void) -> bool;
    pub fn ocio_config_set_use_display_aliases(handle: *mut c_void, enabled: bool);
    pub fn ocio_config_check_hdr_display_compatibility(handle: *mut c_void) -> bool;
    pub fn ocio_config_find_color_space_for_id(
        handle: *mut c_void,
        id: *const c_char,
    ) -> *mut c_void;
    pub fn ocio_config_add_shared_view_with_aliases(
        handle: *mut c_void,
        view: *const c_char,
        view_transform: *const c_char,
        color_space: *const c_char,
        looks: *const c_char,
        rule: *const c_char,
        description: *const c_char,
        aliases: *const *const c_char,
        count: usize,
    );
    pub fn ocio_config_add_display_view_with_aliases(
        handle: *mut c_void,
        display: *const c_char,
        view: *const c_char,
        view_transform: *const c_char,
        color_space: *const c_char,
        looks: *const c_char,
        rule: *const c_char,
        description: *const c_char,
        aliases: *const *const c_char,
        count: usize,
    );
    pub fn ocio_metadata_id_element() -> *const c_char;
}
