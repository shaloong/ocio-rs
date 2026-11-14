use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ocio_color_space_menu_parameters_create(config: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_menu_parameters_destroy(handle: *mut c_void);
    pub fn ocio_color_space_menu_parameters_set_config(handle: *mut c_void, config: *mut c_void);
    pub fn ocio_color_space_menu_parameters_get_config(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_menu_parameters_get_role(handle: *mut c_void) -> *const c_char;
    pub fn ocio_color_space_menu_parameters_set_role(handle: *mut c_void, value: *const c_char);
    pub fn ocio_color_space_menu_parameters_get_include_color_spaces(handle: *mut c_void) -> bool;
    pub fn ocio_color_space_menu_parameters_set_include_color_spaces(
        handle: *mut c_void,
        value: bool,
    );
    pub fn ocio_color_space_menu_parameters_get_include_named_transforms(
        handle: *mut c_void,
    ) -> bool;
    pub fn ocio_color_space_menu_parameters_set_include_named_transforms(
        handle: *mut c_void,
        value: bool,
    );
    #[cfg(feature = "v2_5")]
    pub fn ocio_color_space_menu_parameters_get_treat_no_category_as_any(
        handle: *mut c_void,
    ) -> bool;
    #[cfg(feature = "v2_5")]
    pub fn ocio_color_space_menu_parameters_set_treat_no_category_as_any(
        handle: *mut c_void,
        value: bool,
    );
    pub fn ocio_color_space_menu_parameters_get_app_categories(
        handle: *mut c_void,
    ) -> *const c_char;
    pub fn ocio_color_space_menu_parameters_set_app_categories(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_color_space_menu_parameters_get_encodings(handle: *mut c_void) -> *const c_char;
    pub fn ocio_color_space_menu_parameters_set_encodings(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_color_space_menu_parameters_get_user_categories(
        handle: *mut c_void,
    ) -> *const c_char;
    pub fn ocio_color_space_menu_parameters_set_user_categories(
        handle: *mut c_void,
        value: *const c_char,
    );
    pub fn ocio_color_space_menu_parameters_get_include_roles(handle: *mut c_void) -> bool;
    pub fn ocio_color_space_menu_parameters_set_include_roles(handle: *mut c_void, value: bool);
    pub fn ocio_color_space_menu_parameters_get_search_reference_space_type(
        handle: *mut c_void,
    ) -> i32;
    pub fn ocio_color_space_menu_parameters_set_search_reference_space_type(
        handle: *mut c_void,
        value: i32,
    );
    pub fn ocio_color_space_menu_parameters_add_color_space(
        handle: *mut c_void,
        name: *const c_char,
    );
    pub fn ocio_color_space_menu_parameters_get_num_added_color_spaces(
        handle: *mut c_void,
    ) -> usize;
    pub fn ocio_color_space_menu_parameters_get_added_color_space(
        handle: *mut c_void,
        index: usize,
    ) -> *const c_char;
    pub fn ocio_color_space_menu_parameters_clear_added_color_spaces(handle: *mut c_void);
    pub fn ocio_color_space_menu_helper_create(parameters: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_menu_helper_destroy(handle: *mut c_void);
    pub fn ocio_color_space_menu_helper_get_num_color_spaces(handle: *mut c_void) -> usize;
    pub fn ocio_color_space_menu_helper_get_name(
        handle: *mut c_void,
        index: usize,
    ) -> *const c_char;
    pub fn ocio_color_space_menu_helper_get_ui_name(
        handle: *mut c_void,
        index: usize,
    ) -> *const c_char;
    pub fn ocio_color_space_menu_helper_get_index_from_name(
        handle: *mut c_void,
        name: *const c_char,
    ) -> usize;
    pub fn ocio_color_space_menu_helper_get_index_from_ui_name(
        handle: *mut c_void,
        name: *const c_char,
    ) -> usize;
    pub fn ocio_color_space_menu_helper_get_description(
        handle: *mut c_void,
        index: usize,
    ) -> *const c_char;
    pub fn ocio_color_space_menu_helper_get_family(
        handle: *mut c_void,
        index: usize,
    ) -> *const c_char;
    pub fn ocio_color_space_menu_helper_get_num_hierarchy_levels(
        handle: *mut c_void,
        index: usize,
    ) -> usize;
    pub fn ocio_color_space_menu_helper_get_hierarchy_level(
        handle: *mut c_void,
        index: usize,
        level: usize,
    ) -> *const c_char;
    pub fn ocio_color_space_menu_helper_get_name_from_ui_name(
        handle: *mut c_void,
        name: *const c_char,
    ) -> *const c_char;
    pub fn ocio_color_space_menu_helper_get_ui_name_from_name(
        handle: *mut c_void,
        name: *const c_char,
    ) -> *const c_char;
    pub fn ocio_mixing_color_space_manager_create(config: *mut c_void) -> *mut c_void;
    pub fn ocio_mixing_color_space_manager_destroy(handle: *mut c_void);
    pub fn ocio_mixing_color_space_manager_get_num_mixing_spaces(handle: *mut c_void) -> usize;
    pub fn ocio_mixing_color_space_manager_get_mixing_space_ui_name(
        handle: *mut c_void,
        index: usize,
    ) -> *const c_char;
    pub fn ocio_mixing_color_space_manager_get_selected_mixing_space_idx(
        handle: *mut c_void,
    ) -> usize;
    pub fn ocio_mixing_color_space_manager_set_selected_mixing_space_idx(
        handle: *mut c_void,
        index: usize,
    );
    pub fn ocio_mixing_color_space_manager_set_selected_mixing_space(
        handle: *mut c_void,
        name: *const c_char,
    );
    pub fn ocio_mixing_color_space_manager_is_perceptually_uniform(handle: *mut c_void) -> bool;
    pub fn ocio_mixing_color_space_manager_get_num_mixing_encodings(handle: *mut c_void) -> usize;
    pub fn ocio_mixing_color_space_manager_get_mixing_encoding_name(
        handle: *mut c_void,
        index: usize,
    ) -> *const c_char;
    pub fn ocio_mixing_color_space_manager_get_selected_mixing_encoding_idx(
        handle: *mut c_void,
    ) -> usize;
    pub fn ocio_mixing_color_space_manager_set_selected_mixing_encoding_idx(
        handle: *mut c_void,
        index: usize,
    );
    pub fn ocio_mixing_color_space_manager_set_selected_mixing_encoding(
        handle: *mut c_void,
        name: *const c_char,
    );
    pub fn ocio_mixing_color_space_manager_refresh(handle: *mut c_void, config: *mut c_void);
    pub fn ocio_mixing_color_space_manager_get_processor(
        handle: *mut c_void,
        working: *const c_char,
        display: *const c_char,
        view: *const c_char,
        direction: i32,
    ) -> *mut c_void;
    pub fn ocio_mixing_color_space_manager_get_slider(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_mixing_color_space_manager_get_slider_with_edges(
        handle: *mut c_void,
        min: f32,
        max: f32,
    ) -> *mut c_void;
    pub fn ocio_mixing_slider_set_slider_min_edge(handle: *mut c_void, value: f32);
    pub fn ocio_mixing_slider_set_slider_max_edge(handle: *mut c_void, value: f32);
    pub fn ocio_mixing_slider_get_slider_min_edge(handle: *mut c_void) -> f32;
    pub fn ocio_mixing_slider_get_slider_max_edge(handle: *mut c_void) -> f32;
    pub fn ocio_mixing_slider_slider_to_mixing(handle: *mut c_void, value: f32) -> f32;
    pub fn ocio_mixing_slider_mixing_to_slider(handle: *mut c_void, value: f32) -> f32;
    pub fn ocio_legacy_viewing_pipeline_create() -> *mut c_void;
    pub fn ocio_legacy_viewing_pipeline_destroy(handle: *mut c_void);
    pub fn ocio_legacy_viewing_pipeline_get_display_view_transform(
        handle: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_legacy_viewing_pipeline_set_display_view_transform(
        handle: *mut c_void,
        transform: *mut c_void,
    );
    pub fn ocio_legacy_viewing_pipeline_get_linear_cc(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_legacy_viewing_pipeline_set_linear_cc(handle: *mut c_void, transform: *mut c_void);
    pub fn ocio_legacy_viewing_pipeline_get_color_timing_cc(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_legacy_viewing_pipeline_set_color_timing_cc(
        handle: *mut c_void,
        transform: *mut c_void,
    );
    pub fn ocio_legacy_viewing_pipeline_get_channel_view(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_legacy_viewing_pipeline_set_channel_view(
        handle: *mut c_void,
        transform: *mut c_void,
    );
    pub fn ocio_legacy_viewing_pipeline_get_display_cc(handle: *mut c_void) -> *mut c_void;
    pub fn ocio_legacy_viewing_pipeline_set_display_cc(handle: *mut c_void, transform: *mut c_void);
    pub fn ocio_legacy_viewing_pipeline_get_looks_override(handle: *mut c_void) -> *const c_char;
    pub fn ocio_legacy_viewing_pipeline_set_looks_override(
        handle: *mut c_void,
        looks: *const c_char,
    );
    pub fn ocio_legacy_viewing_pipeline_get_looks_override_enabled(handle: *mut c_void) -> bool;
    pub fn ocio_legacy_viewing_pipeline_set_looks_override_enabled(
        handle: *mut c_void,
        enabled: bool,
    );
    pub fn ocio_legacy_viewing_pipeline_get_processor(
        handle: *mut c_void,
        config: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_legacy_viewing_pipeline_get_processor_with_context(
        handle: *mut c_void,
        config: *mut c_void,
        context: *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_config_get_display_view_helper_identity_processor(
        handle: *mut c_void,
    ) -> *mut c_void;
}
