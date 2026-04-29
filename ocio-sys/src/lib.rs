use std::ffi::c_void;

unsafe extern "C" {
    // --- Runtime ---
    pub fn ocio_runtime_is_stub() -> bool;

    // --- Config: construction ---
    pub fn ocio_config_create_raw() -> *mut c_void;
    pub fn ocio_config_create_from_file(path: *const i8) -> *mut c_void;
    pub fn ocio_config_destroy(handle: *mut c_void);

    // --- Config: name & metadata ---
    pub fn ocio_config_get_name(config: *mut c_void) -> *const i8;
    pub fn ocio_config_get_description(config: *mut c_void) -> *const i8;
    pub fn ocio_config_get_cache_id(config: *mut c_void) -> *const i8;

    // --- Config: version ---
    pub fn ocio_config_get_major_version(config: *mut c_void) -> u32;
    pub fn ocio_config_get_minor_version(config: *mut c_void) -> u32;
    pub fn ocio_config_get_family_separator(config: *mut c_void) -> i8;

    // --- Config: color spaces ---
    pub fn ocio_config_get_num_color_spaces(config: *mut c_void) -> i32;
    pub fn ocio_config_get_color_space_name_by_index(config: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_config_get_canonical_name(config: *mut c_void, name: *const i8) -> *const i8;
    pub fn ocio_config_is_color_space_linear(
        config: *mut c_void, colorSpace: *const i8, referenceSpaceType: i32,
    ) -> bool;
    pub fn ocio_config_get_color_space_from_filepath(
        config: *mut c_void, filePath: *const i8,
    ) -> *const i8;

    // --- Config: displays ---
    pub fn ocio_config_get_default_display(config: *mut c_void) -> *const i8;
    pub fn ocio_config_get_num_displays(config: *mut c_void) -> i32;
    pub fn ocio_config_get_display(config: *mut c_void, index: i32) -> *const i8;

    // --- Config: views ---
    pub fn ocio_config_get_default_view(
        config: *mut c_void, display: *const i8,
    ) -> *const i8;
    pub fn ocio_config_get_num_views(config: *mut c_void, display: *const i8) -> i32;
    pub fn ocio_config_get_view(
        config: *mut c_void, display: *const i8, index: i32,
    ) -> *const i8;

    // --- Config: looks ---
    pub fn ocio_config_get_num_looks(config: *mut c_void) -> i32;
    pub fn ocio_config_get_look_name_by_index(config: *mut c_void, index: i32) -> *const i8;

    // --- Config: luma coefs ---
    pub fn ocio_config_get_default_luma_coefs(config: *mut c_void, rgb: *mut f64);

    // --- Config: roles ---
    pub fn ocio_config_get_num_roles(config: *mut c_void) -> i32;
    pub fn ocio_config_has_role(config: *mut c_void, role: *const i8) -> bool;
    pub fn ocio_config_get_role_name(config: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_config_get_role_color_space_by_index(
        config: *mut c_void, index: i32,
    ) -> *const i8;
    pub fn ocio_config_get_role_color_space_by_name(
        config: *mut c_void, roleName: *const i8,
    ) -> *const i8;

    // --- Config: active displays / views ---
    pub fn ocio_config_get_active_displays(config: *mut c_void) -> *const i8;
    pub fn ocio_config_get_active_views(config: *mut c_void) -> *const i8;

    // --- Config: processors ---
    pub fn ocio_config_get_processor(
        config: *mut c_void, src: *const i8, dst: *const i8,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_display(
        config: *mut c_void, src: *const i8, display: *const i8, view: *const i8, direction: i32,
    ) -> *mut c_void;
    pub fn ocio_config_get_processor_transform(
        config: *mut c_void, transform: *mut c_void, direction: i32,
    ) -> *mut c_void;

    // --- Config: ColorSpace object ---
    pub fn ocio_config_get_color_space(config: *mut c_void, name: *const i8) -> *mut c_void;
    pub fn ocio_config_get_index_for_color_space(config: *mut c_void, name: *const i8) -> i32;
    pub fn ocio_config_add_color_space(config: *mut c_void, colorSpace: *mut c_void);
    pub fn ocio_config_remove_color_space(config: *mut c_void, name: *const i8);
    pub fn ocio_config_is_color_space_used(config: *mut c_void, name: *const i8) -> bool;

    // --- Config: Looks by object ---
    pub fn ocio_config_get_look(config: *mut c_void, name: *const i8) -> *mut c_void;
    pub fn ocio_config_add_look(config: *mut c_void, look: *mut c_void);

    // --- Processor ---
    pub fn ocio_processor_get_default_cpu_processor(processor: *mut c_void) -> *mut c_void;
    pub fn ocio_processor_get_optimized_cpu_processor(
        processor: *mut c_void, flags: u64,
    ) -> *mut c_void;
    pub fn ocio_processor_is_no_op(processor: *mut c_void) -> bool;
    pub fn ocio_processor_has_channel_crosstalk(processor: *mut c_void) -> bool;
    pub fn ocio_processor_get_cache_id(processor: *mut c_void) -> *const i8;
    pub fn ocio_processor_apply_rgba(processor: *mut c_void, rgba: *mut f32, len: usize);
    pub fn ocio_processor_destroy(handle: *mut c_void);

    // --- CPUProcessor ---
    pub fn ocio_cpu_processor_apply_rgba(cpu_processor: *mut c_void, rgba: *mut f32);
    pub fn ocio_cpu_processor_apply_rgb(cpu_processor: *mut c_void, rgb: *mut f32);
    pub fn ocio_cpu_processor_is_no_op(cpu_processor: *mut c_void) -> bool;
    pub fn ocio_cpu_processor_has_channel_crosstalk(cpu_processor: *mut c_void) -> bool;
    pub fn ocio_cpu_processor_get_cache_id(cpu_processor: *mut c_void) -> *const i8;
    pub fn ocio_cpu_processor_get_input_bit_depth(cpu_processor: *mut c_void) -> i32;
    pub fn ocio_cpu_processor_get_output_bit_depth(cpu_processor: *mut c_void) -> i32;
    pub fn ocio_cpu_processor_is_identity(cpu_processor: *mut c_void) -> bool;
    pub fn ocio_cpu_processor_destroy(handle: *mut c_void);

    // --- GPUProcessor ---
    pub fn ocio_processor_get_default_gpu_processor(processor: *mut c_void) -> *mut c_void;
    pub fn ocio_processor_get_optimized_gpu_processor(
        processor: *mut c_void, flags: u64,
    ) -> *mut c_void;
    pub fn ocio_gpu_processor_is_no_op(gpu_processor: *mut c_void) -> bool;
    pub fn ocio_gpu_processor_has_channel_crosstalk(gpu_processor: *mut c_void) -> bool;
    pub fn ocio_gpu_processor_get_cache_id(gpu_processor: *mut c_void) -> *const i8;
    pub fn ocio_gpu_processor_extract_shader_info(
        gpu_processor: *mut c_void, shader_desc: *mut c_void,
    );
    pub fn ocio_gpu_processor_destroy(handle: *mut c_void);

    // --- GpuShaderDesc ---
    pub fn ocio_gpu_shader_desc_create() -> *mut c_void;
    pub fn ocio_gpu_shader_desc_get_shader_text(shader_desc: *mut c_void) -> *const i8;
    pub fn ocio_gpu_shader_desc_get_num_textures(shader_desc: *mut c_void) -> u32;
    pub fn ocio_gpu_shader_desc_get_texture(
        shader_desc: *mut c_void, index: u32,
        textureName: *mut *const i8, samplerName: *mut *const i8,
        width: *mut u32, height: *mut u32,
        channel: *mut i32, dimensions: *mut i32, interpolation: *mut i32,
    );
    pub fn ocio_gpu_shader_desc_get_texture_values(
        shader_desc: *mut c_void, index: u32,
    ) -> *const f32;
    pub fn ocio_gpu_shader_desc_get_language(shader_desc: *mut c_void) -> i32;
    pub fn ocio_gpu_shader_desc_set_language(shader_desc: *mut c_void, language: i32);
    pub fn ocio_gpu_shader_desc_get_function_name(shader_desc: *mut c_void) -> *const i8;
    pub fn ocio_gpu_shader_desc_set_function_name(shader_desc: *mut c_void, name: *const i8);
    pub fn ocio_gpu_shader_desc_get_pixel_name(shader_desc: *mut c_void) -> *const i8;
    pub fn ocio_gpu_shader_desc_set_pixel_name(shader_desc: *mut c_void, name: *const i8);
    pub fn ocio_gpu_shader_desc_get_resource_prefix(shader_desc: *mut c_void) -> *const i8;
    pub fn ocio_gpu_shader_desc_set_resource_prefix(shader_desc: *mut c_void, prefix: *const i8);
    pub fn ocio_gpu_shader_desc_finalize(shader_desc: *mut c_void);
    pub fn ocio_gpu_shader_desc_destroy(handle: *mut c_void);

    // --- Transform base ---
    pub fn ocio_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_transform_get_transform_type(transform: *mut c_void) -> i32;
    pub fn ocio_transform_create_editable_copy(transform: *mut c_void) -> *mut c_void;
    pub fn ocio_transform_destroy(handle: *mut c_void);

    // --- FileTransform ---
    pub fn ocio_file_transform_create() -> *mut c_void;
    pub fn ocio_file_transform_get_src(transform: *mut c_void) -> *const i8;
    pub fn ocio_file_transform_set_src(transform: *mut c_void, src: *const i8);
    pub fn ocio_file_transform_get_ccc_id(transform: *mut c_void) -> *const i8;
    pub fn ocio_file_transform_set_ccc_id(transform: *mut c_void, id: *const i8);
    pub fn ocio_file_transform_get_interpolation(transform: *mut c_void) -> i32;
    pub fn ocio_file_transform_set_interpolation(transform: *mut c_void, interp: i32);
    pub fn ocio_file_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_file_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_file_transform_destroy(handle: *mut c_void);

    // --- CDLTransform ---
    pub fn ocio_cdl_transform_create() -> *mut c_void;
    pub fn ocio_cdl_transform_create_from_file(src: *const i8, cccId: *const i8) -> *mut c_void;
    pub fn ocio_cdl_transform_get_slope(transform: *mut c_void, rgb: *mut f64);
    pub fn ocio_cdl_transform_set_slope(transform: *mut c_void, rgb: *const f64);
    pub fn ocio_cdl_transform_get_offset(transform: *mut c_void, rgb: *mut f64);
    pub fn ocio_cdl_transform_set_offset(transform: *mut c_void, rgb: *const f64);
    pub fn ocio_cdl_transform_get_power(transform: *mut c_void, rgb: *mut f64);
    pub fn ocio_cdl_transform_set_power(transform: *mut c_void, rgb: *const f64);
    pub fn ocio_cdl_transform_get_sat(transform: *mut c_void) -> f64;
    pub fn ocio_cdl_transform_set_sat(transform: *mut c_void, sat: f64);
    pub fn ocio_cdl_transform_get_sat_luma_coefs(transform: *mut c_void, rgb: *mut f64);
    pub fn ocio_cdl_transform_get_style(transform: *mut c_void) -> i32;
    pub fn ocio_cdl_transform_set_style(transform: *mut c_void, style: i32);
    pub fn ocio_cdl_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_cdl_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_cdl_transform_destroy(handle: *mut c_void);

    // --- ExponentTransform ---
    pub fn ocio_exponent_transform_create() -> *mut c_void;
    pub fn ocio_exponent_transform_get_value(transform: *mut c_void, vec4: *mut f64);
    pub fn ocio_exponent_transform_set_value(transform: *mut c_void, vec4: *const f64);
    pub fn ocio_exponent_transform_get_negative_style(transform: *mut c_void) -> i32;
    pub fn ocio_exponent_transform_set_negative_style(transform: *mut c_void, style: i32);
    pub fn ocio_exponent_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_exponent_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_exponent_transform_destroy(handle: *mut c_void);

    // --- MatrixTransform ---
    pub fn ocio_matrix_transform_create() -> *mut c_void;
    pub fn ocio_matrix_transform_get_matrix(transform: *mut c_void, m44: *mut f64);
    pub fn ocio_matrix_transform_set_matrix(transform: *mut c_void, m44: *const f64);
    pub fn ocio_matrix_transform_get_offset(transform: *mut c_void, offset4: *mut f64);
    pub fn ocio_matrix_transform_set_offset(transform: *mut c_void, offset4: *const f64);
    pub fn ocio_matrix_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_matrix_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_matrix_transform_destroy(handle: *mut c_void);

    // --- LogTransform ---
    pub fn ocio_log_transform_create() -> *mut c_void;
    pub fn ocio_log_transform_get_base(transform: *mut c_void) -> f64;
    pub fn ocio_log_transform_set_base(transform: *mut c_void, base: f64);
    pub fn ocio_log_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_log_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_log_transform_destroy(handle: *mut c_void);

    // --- RangeTransform ---
    pub fn ocio_range_transform_create() -> *mut c_void;
    pub fn ocio_range_transform_get_style(transform: *mut c_void) -> i32;
    pub fn ocio_range_transform_set_style(transform: *mut c_void, style: i32);
    pub fn ocio_range_transform_get_min_in_value(transform: *mut c_void) -> f64;
    pub fn ocio_range_transform_set_min_in_value(transform: *mut c_void, value: f64);
    pub fn ocio_range_transform_get_max_in_value(transform: *mut c_void) -> f64;
    pub fn ocio_range_transform_set_max_in_value(transform: *mut c_void, value: f64);
    pub fn ocio_range_transform_get_min_out_value(transform: *mut c_void) -> f64;
    pub fn ocio_range_transform_set_min_out_value(transform: *mut c_void, value: f64);
    pub fn ocio_range_transform_get_max_out_value(transform: *mut c_void) -> f64;
    pub fn ocio_range_transform_set_max_out_value(transform: *mut c_void, value: f64);
    pub fn ocio_range_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_range_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_range_transform_destroy(handle: *mut c_void);

    // --- GroupTransform ---
    pub fn ocio_group_transform_create() -> *mut c_void;
    pub fn ocio_group_transform_get_num_transforms(transform: *mut c_void) -> i32;
    pub fn ocio_group_transform_get_transform(transform: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_group_transform_append_transform(transform: *mut c_void, child: *mut c_void);
    pub fn ocio_group_transform_prepend_transform(transform: *mut c_void, child: *mut c_void);
    pub fn ocio_group_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_group_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_group_transform_destroy(handle: *mut c_void);

    // --- BuiltinTransform ---
    pub fn ocio_builtin_transform_create() -> *mut c_void;
    pub fn ocio_builtin_transform_get_style(transform: *mut c_void) -> *const i8;
    pub fn ocio_builtin_transform_set_style(transform: *mut c_void, style: *const i8);
    pub fn ocio_builtin_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_builtin_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_builtin_transform_destroy(handle: *mut c_void);

    // --- FixedFunctionTransform ---
    pub fn ocio_fixed_function_transform_create(style: i32) -> *mut c_void;
    pub fn ocio_fixed_function_transform_create_with_params(
        style: i32, params: *const f64, num_params: i32,
    ) -> *mut c_void;
    pub fn ocio_fixed_function_transform_get_style(transform: *mut c_void) -> i32;
    pub fn ocio_fixed_function_transform_set_style(transform: *mut c_void, style: i32);
    pub fn ocio_fixed_function_transform_get_num_params(transform: *mut c_void) -> i32;
    pub fn ocio_fixed_function_transform_get_params(transform: *mut c_void, params: *mut f64);
    pub fn ocio_fixed_function_transform_set_params(
        transform: *mut c_void, params: *const f64, num_params: i32,
    );
    pub fn ocio_fixed_function_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_fixed_function_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_fixed_function_transform_destroy(handle: *mut c_void);

    // --- Lut1DTransform ---
    pub fn ocio_lut1d_transform_create() -> *mut c_void;
    pub fn ocio_lut1d_transform_get_interpolation(transform: *mut c_void) -> i32;
    pub fn ocio_lut1d_transform_set_interpolation(transform: *mut c_void, interpolation: i32);
    pub fn ocio_lut1d_transform_get_file_output_bit_depth(transform: *mut c_void) -> i32;
    pub fn ocio_lut1d_transform_set_file_output_bit_depth(transform: *mut c_void, bit_depth: i32);
    pub fn ocio_lut1d_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_lut1d_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_lut1d_transform_destroy(handle: *mut c_void);

    // --- Lut3DTransform ---
    pub fn ocio_lut3d_transform_create() -> *mut c_void;
    pub fn ocio_lut3d_transform_get_interpolation(transform: *mut c_void) -> i32;
    pub fn ocio_lut3d_transform_set_interpolation(transform: *mut c_void, interpolation: i32);
    pub fn ocio_lut3d_transform_get_file_output_bit_depth(transform: *mut c_void) -> i32;
    pub fn ocio_lut3d_transform_set_file_output_bit_depth(transform: *mut c_void, bit_depth: i32);
    pub fn ocio_lut3d_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_lut3d_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_lut3d_transform_destroy(handle: *mut c_void);

    // --- Baker ---
    pub fn ocio_baker_create() -> *mut c_void;
    pub fn ocio_baker_create_editable_copy(baker: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_set_config(baker: *mut c_void, config: *mut c_void);
    pub fn ocio_baker_get_config(baker: *mut c_void) -> *mut c_void;
    pub fn ocio_baker_get_format(baker: *mut c_void) -> *const i8;
    pub fn ocio_baker_set_format(baker: *mut c_void, formatName: *const i8);
    pub fn ocio_baker_get_input_space(baker: *mut c_void) -> *const i8;
    pub fn ocio_baker_set_input_space(baker: *mut c_void, inputSpace: *const i8);
    pub fn ocio_baker_get_shaper_space(baker: *mut c_void) -> *const i8;
    pub fn ocio_baker_set_shaper_space(baker: *mut c_void, shaperSpace: *const i8);
    pub fn ocio_baker_get_looks(baker: *mut c_void) -> *const i8;
    pub fn ocio_baker_set_looks(baker: *mut c_void, looks: *const i8);
    pub fn ocio_baker_get_target_space(baker: *mut c_void) -> *const i8;
    pub fn ocio_baker_set_target_space(baker: *mut c_void, targetSpace: *const i8);
    pub fn ocio_baker_get_display(baker: *mut c_void) -> *const i8;
    pub fn ocio_baker_get_view(baker: *mut c_void) -> *const i8;
    pub fn ocio_baker_set_display_view(
        baker: *mut c_void, display: *const i8, view: *const i8,
    );
    pub fn ocio_baker_get_shaper_size(baker: *mut c_void) -> i32;
    pub fn ocio_baker_set_shaper_size(baker: *mut c_void, size: i32);
    pub fn ocio_baker_get_cube_size(baker: *mut c_void) -> i32;
    pub fn ocio_baker_set_cube_size(baker: *mut c_void, size: i32);
    pub fn ocio_baker_bake(baker: *mut c_void, outputPath: *const i8);
    pub fn ocio_baker_destroy(handle: *mut c_void);

    // --- Context ---
    pub fn ocio_context_create() -> *mut c_void;
    pub fn ocio_context_create_editable_copy(context: *mut c_void) -> *mut c_void;
    pub fn ocio_context_get_cache_id(context: *mut c_void) -> *const i8;
    pub fn ocio_context_get_search_path(context: *mut c_void) -> *const i8;
    pub fn ocio_context_set_search_path(context: *mut c_void, path: *const i8);
    pub fn ocio_context_get_num_search_paths(context: *mut c_void) -> i32;
    pub fn ocio_context_get_search_path_by_index(context: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_context_get_working_dir(context: *mut c_void) -> *const i8;
    pub fn ocio_context_set_working_dir(context: *mut c_void, dirname: *const i8);
    pub fn ocio_context_get_string_var(context: *mut c_void, name: *const i8) -> *const i8;
    pub fn ocio_context_set_string_var(context: *mut c_void, name: *const i8, value: *const i8);
    pub fn ocio_context_get_num_string_vars(context: *mut c_void) -> i32;
    pub fn ocio_context_get_string_var_name_by_index(
        context: *mut c_void, index: i32,
    ) -> *const i8;
    pub fn ocio_context_get_string_var_by_index(context: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_context_resolve_string_var(context: *mut c_void, string: *const i8) -> *const i8;
    pub fn ocio_context_resolve_file_location(
        context: *mut c_void, filename: *const i8,
    ) -> *const i8;
    pub fn ocio_context_clear_string_vars(context: *mut c_void);
    pub fn ocio_context_set_environment_mode(context: *mut c_void, mode: i32);
    pub fn ocio_context_get_environment_mode(context: *mut c_void) -> i32;
    pub fn ocio_context_load_environment(context: *mut c_void);
    pub fn ocio_context_destroy(handle: *mut c_void);

    // --- ColorSpace ---
    pub fn ocio_color_space_create() -> *mut c_void;
    pub fn ocio_color_space_create_editable_copy(colorSpace: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_get_name(colorSpace: *mut c_void) -> *const i8;
    pub fn ocio_color_space_set_name(colorSpace: *mut c_void, name: *const i8);
    pub fn ocio_color_space_get_family(colorSpace: *mut c_void) -> *const i8;
    pub fn ocio_color_space_set_family(colorSpace: *mut c_void, family: *const i8);
    pub fn ocio_color_space_get_equality_group(colorSpace: *mut c_void) -> *const i8;
    pub fn ocio_color_space_set_equality_group(colorSpace: *mut c_void, group: *const i8);
    pub fn ocio_color_space_get_description(colorSpace: *mut c_void) -> *const i8;
    pub fn ocio_color_space_set_description(colorSpace: *mut c_void, description: *const i8);
    pub fn ocio_color_space_get_bit_depth(colorSpace: *mut c_void) -> i32;
    pub fn ocio_color_space_set_bit_depth(colorSpace: *mut c_void, bitDepth: i32);
    pub fn ocio_color_space_get_reference_space_type(colorSpace: *mut c_void) -> i32;
    pub fn ocio_color_space_is_data(colorSpace: *mut c_void) -> bool;
    pub fn ocio_color_space_set_is_data(colorSpace: *mut c_void, isData: bool);
    pub fn ocio_color_space_get_allocation(colorSpace: *mut c_void) -> i32;
    pub fn ocio_color_space_set_allocation(colorSpace: *mut c_void, allocation: i32);
    pub fn ocio_color_space_get_allocation_num_vars(colorSpace: *mut c_void) -> i32;
    pub fn ocio_color_space_get_allocation_vars(colorSpace: *mut c_void, vars: *mut f32);
    pub fn ocio_color_space_set_allocation_vars(
        colorSpace: *mut c_void, numVars: i32, vars: *const f32,
    );
    pub fn ocio_color_space_get_transform(
        colorSpace: *mut c_void, direction: i32,
    ) -> *mut c_void;
    pub fn ocio_color_space_set_transform(
        colorSpace: *mut c_void, transform: *const c_void, direction: i32,
    );
    pub fn ocio_color_space_get_encoding(colorSpace: *mut c_void) -> *const i8;
    pub fn ocio_color_space_set_encoding(colorSpace: *mut c_void, encoding: *const i8);
    pub fn ocio_color_space_destroy(handle: *mut c_void);

    // --- Look ---
    pub fn ocio_look_create() -> *mut c_void;
    pub fn ocio_look_create_editable_copy(look: *mut c_void) -> *mut c_void;
    pub fn ocio_look_get_name(look: *mut c_void) -> *const i8;
    pub fn ocio_look_set_name(look: *mut c_void, name: *const i8);
    pub fn ocio_look_get_process_space(look: *mut c_void) -> *const i8;
    pub fn ocio_look_set_process_space(look: *mut c_void, processSpace: *const i8);
    pub fn ocio_look_get_direction(look: *mut c_void) -> i32;
    pub fn ocio_look_set_direction(look: *mut c_void, direction: i32);
    pub fn ocio_look_destroy(handle: *mut c_void);

    // --- ViewTransform ---
    pub fn ocio_view_transform_create(referenceSpace: i32) -> *mut c_void;
    pub fn ocio_view_transform_get_src(viewTransform: *mut c_void) -> *const i8;
    pub fn ocio_view_transform_set_src(viewTransform: *mut c_void, src: *const i8);
    pub fn ocio_view_transform_get_display(viewTransform: *mut c_void) -> *const i8;
    pub fn ocio_view_transform_set_display(viewTransform: *mut c_void, display: *const i8);
    pub fn ocio_view_transform_get_view(viewTransform: *mut c_void) -> *const i8;
    pub fn ocio_view_transform_set_view(viewTransform: *mut c_void, view: *const i8);
    pub fn ocio_view_transform_get_looks_bypass(viewTransform: *mut c_void) -> bool;
    pub fn ocio_view_transform_set_looks_bypass(viewTransform: *mut c_void, bypass: bool);
    pub fn ocio_view_transform_get_rule(viewTransform: *mut c_void) -> *const i8;
    pub fn ocio_view_transform_set_rule(viewTransform: *mut c_void, rule: *const i8);
    pub fn ocio_view_transform_get_transform(viewTransform: *mut c_void) -> *mut c_void;
    pub fn ocio_view_transform_set_transform(viewTransform: *mut c_void, transform: *const c_void);
    pub fn ocio_view_transform_get_direction(viewTransform: *mut c_void) -> i32;
    pub fn ocio_view_transform_set_direction(viewTransform: *mut c_void, direction: i32);
    pub fn ocio_view_transform_destroy(handle: *mut c_void);

    // --- NamedTransform ---
    pub fn ocio_named_transform_create() -> *mut c_void;
    pub fn ocio_named_transform_create_editable_copy(namedTransform: *mut c_void) -> *mut c_void;
    pub fn ocio_named_transform_get_name(namedTransform: *mut c_void) -> *const i8;
    pub fn ocio_named_transform_set_name(namedTransform: *mut c_void, name: *const i8);
    pub fn ocio_named_transform_get_family(namedTransform: *mut c_void) -> *const i8;
    pub fn ocio_named_transform_set_family(namedTransform: *mut c_void, family: *const i8);
    pub fn ocio_named_transform_get_description(namedTransform: *mut c_void) -> *const i8;
    pub fn ocio_named_transform_set_description(namedTransform: *mut c_void, description: *const i8);
    pub fn ocio_named_transform_get_encoding(namedTransform: *mut c_void) -> *const i8;
    pub fn ocio_named_transform_set_encoding(namedTransform: *mut c_void, encoding: *const i8);
    pub fn ocio_named_transform_get_transform(namedTransform: *mut c_void, direction: i32) -> *mut c_void;
    pub fn ocio_named_transform_set_transform(namedTransform: *mut c_void, transform: *const c_void, direction: i32);
    pub fn ocio_named_transform_destroy(handle: *mut c_void);

    // --- DynamicProperty ---
    pub fn ocio_processor_get_dynamic_property(processor: *mut c_void, propertyType: i32) -> *mut c_void;
    pub fn ocio_dynamic_property_get_type(prop: *mut c_void) -> i32;
    pub fn ocio_dynamic_property_double_get_value(prop: *mut c_void) -> f64;
    pub fn ocio_dynamic_property_double_set_value(prop: *mut c_void, value: f64);
    pub fn ocio_dynamic_property_destroy(handle: *mut c_void);

    // --- ExposureContrastTransform ---
    pub fn ocio_exposure_contrast_transform_create() -> *mut c_void;
    pub fn ocio_exposure_contrast_transform_get_exposure(transform: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_exposure(transform: *mut c_void, exposure: f64);
    pub fn ocio_exposure_contrast_transform_get_contrast(transform: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_contrast(transform: *mut c_void, contrast: f64);
    pub fn ocio_exposure_contrast_transform_get_gamma(transform: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_gamma(transform: *mut c_void, gamma: f64);
    pub fn ocio_exposure_contrast_transform_get_pivot(transform: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_pivot(transform: *mut c_void, pivot: f64);
    pub fn ocio_exposure_contrast_transform_get_style(transform: *mut c_void) -> i32;
    pub fn ocio_exposure_contrast_transform_set_style(transform: *mut c_void, style: i32);
    pub fn ocio_exposure_contrast_transform_is_exposure_dynamic(transform: *mut c_void) -> bool;
    pub fn ocio_exposure_contrast_transform_make_exposure_dynamic(transform: *mut c_void);
    pub fn ocio_exposure_contrast_transform_is_contrast_dynamic(transform: *mut c_void) -> bool;
    pub fn ocio_exposure_contrast_transform_make_contrast_dynamic(transform: *mut c_void);
    pub fn ocio_exposure_contrast_transform_is_gamma_dynamic(transform: *mut c_void) -> bool;
    pub fn ocio_exposure_contrast_transform_make_gamma_dynamic(transform: *mut c_void);
    pub fn ocio_exposure_contrast_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_exposure_contrast_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_exposure_contrast_transform_destroy(handle: *mut c_void);
}
