use std::ffi::c_void;

unsafe extern "C" {
    // --- Runtime ---
    pub fn ocio_runtime_is_stub() -> bool;

    // --- BuiltinConfigRegistry ---
    pub fn ocio_builtin_config_registry_get() -> *mut c_void;
    pub fn ocio_builtin_config_registry_get_num_builtin_configs(registry: *mut c_void) -> i32;
    pub fn ocio_builtin_config_registry_get_config_name(registry: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_builtin_config_registry_get_config_ui_name(registry: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_builtin_config_registry_is_config_recommended(registry: *mut c_void, index: i32) -> bool;
    pub fn ocio_builtin_config_registry_get_config_by_index(registry: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_builtin_config_registry_get_config_by_name(registry: *mut c_void, name: *const i8) -> *mut c_void;

    // --- Global utility functions ---
    pub fn ocio_get_version() -> *const i8;
    pub fn ocio_get_version_hex() -> i32;
    pub fn ocio_get_logging_level() -> i32;
    pub fn ocio_set_logging_level(level: i32);
    pub fn ocio_set_logging_level_to_override(level: i32);

    // --- Global config ---
    pub fn ocio_get_current_config() -> *mut c_void;
    pub fn ocio_set_current_config(config: *mut c_void);
    pub fn ocio_clear_all_caches();

    // --- Config: construction ---
    pub fn ocio_config_create_raw() -> *mut c_void;
    pub fn ocio_config_create_from_file(path: *const i8) -> *mut c_void;
    pub fn ocio_config_destroy(handle: *mut c_void);

    // --- Config: name & metadata ---
    pub fn ocio_config_get_name(config: *mut c_void) -> *const i8;
    pub fn ocio_config_set_name(config: *mut c_void, name: *const i8);
    pub fn ocio_config_get_description(config: *mut c_void) -> *const i8;
    pub fn ocio_config_set_description(config: *mut c_void, desc: *const i8);
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

    // --- Config: search paths & strict parsing ---
    pub fn ocio_config_get_search_path(config: *mut c_void) -> *const i8;
    pub fn ocio_config_set_search_path(config: *mut c_void, path: *const i8);
    pub fn ocio_config_is_strict_parsing_enabled(config: *mut c_void) -> bool;
    pub fn ocio_config_set_strict_parsing_enabled(config: *mut c_void, enabled: bool);

    // --- Config: roles (mutable) ---
    pub fn ocio_config_set_role(config: *mut c_void, role: *const i8, colorSpace: *const i8);

    // --- Config: family separator ---
    pub fn ocio_config_set_family_separator(config: *mut c_void, separator: i8);

    // --- Config: validate & serialize ---
    pub fn ocio_config_validate(config: *mut c_void) -> *const i8;
    pub fn ocio_config_serialize(config: *mut c_void) -> *const i8;

    // --- Config: editable copy ---
    pub fn ocio_config_create_editable_copy(config: *mut c_void) -> *mut c_void;

    // --- Config: context ---
    pub fn ocio_config_get_current_context(config: *mut c_void) -> *mut c_void;
    pub fn ocio_config_set_current_context(config: *mut c_void, context: *mut c_void);

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
    pub fn ocio_processor_get_num_transforms(processor: *mut c_void) -> i32;
    pub fn ocio_processor_create_group_transform(processor: *mut c_void) -> *mut c_void;
    pub fn ocio_processor_destroy(handle: *mut c_void);

    // --- Processor: bit-depth CPU/GPU processor access ---
    pub fn ocio_processor_get_default_cpu_processor_bitdepth(processor: *mut c_void, inBitDepth: i32, outBitDepth: i32) -> *mut c_void;
    pub fn ocio_processor_get_optimized_cpu_processor_bitdepth(processor: *mut c_void, inBitDepth: i32, outBitDepth: i32, flags: u64) -> *mut c_void;
    pub fn ocio_processor_get_default_gpu_processor_bitdepth(processor: *mut c_void, inBitDepth: i32, outBitDepth: i32) -> *mut c_void;
    pub fn ocio_processor_get_optimized_gpu_processor_bitdepth(processor: *mut c_void, inBitDepth: i32, outBitDepth: i32, flags: u64) -> *mut c_void;

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

    // --- CPUProcessor: packed pixel processing ---
    pub fn ocio_cpu_processor_apply_rgba_packed(cpu_processor: *mut c_void, rgba: *mut c_void, bitDepth: i32, numPixels: i64, stride: i64);
    pub fn ocio_cpu_processor_apply_rgb_packed(cpu_processor: *mut c_void, rgb: *mut c_void, bitDepth: i32, numPixels: i64, stride: i64);

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
    pub fn ocio_file_transform_get_cdl_style(transform: *mut c_void) -> i32;
    pub fn ocio_file_transform_set_cdl_style(transform: *mut c_void, style: i32);
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
    pub fn ocio_cdl_transform_set_sat_luma_coefs(transform: *mut c_void, rgb: *const f64);
    pub fn ocio_cdl_transform_get_style(transform: *mut c_void) -> i32;
    pub fn ocio_cdl_transform_set_style(transform: *mut c_void, style: i32);
    pub fn ocio_cdl_transform_get_id(transform: *mut c_void) -> *const i8;
    pub fn ocio_cdl_transform_set_id(transform: *mut c_void, id: *const i8);
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

    // --- ExponentWithLinearTransform ---
    pub fn ocio_exponent_with_linear_transform_create() -> *mut c_void;
    pub fn ocio_exponent_with_linear_transform_get_gamma(transform: *mut c_void, vec4: *mut f64);
    pub fn ocio_exponent_with_linear_transform_set_gamma(transform: *mut c_void, vec4: *const f64);
    pub fn ocio_exponent_with_linear_transform_get_offset(transform: *mut c_void, vec4: *mut f64);
    pub fn ocio_exponent_with_linear_transform_set_offset(transform: *mut c_void, vec4: *const f64);
    pub fn ocio_exponent_with_linear_transform_get_negative_style(transform: *mut c_void) -> i32;
    pub fn ocio_exponent_with_linear_transform_set_negative_style(transform: *mut c_void, style: i32);
    pub fn ocio_exponent_with_linear_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_exponent_with_linear_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_exponent_with_linear_transform_destroy(handle: *mut c_void);

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
    pub fn ocio_baker_get_num_formats() -> i32;
    pub fn ocio_baker_get_format_name_by_index(index: i32) -> *const i8;
    pub fn ocio_baker_get_format_extension_by_index(index: i32) -> *const i8;
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
    pub fn ocio_color_space_get_category(colorSpace: *mut c_void) -> *const i8;
    pub fn ocio_color_space_set_category(colorSpace: *mut c_void, category: *const i8);
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
    pub fn ocio_look_get_description(look: *mut c_void) -> *const i8;
    pub fn ocio_look_set_description(look: *mut c_void, description: *const i8);
    pub fn ocio_look_get_transform(look: *mut c_void) -> *mut c_void;
    pub fn ocio_look_set_transform(look: *mut c_void, transform: *const c_void);
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

    // --- ColorSpaceTransform ---
    pub fn ocio_color_space_transform_create() -> *mut c_void;
    pub fn ocio_color_space_transform_get_src(transform: *mut c_void) -> *const i8;
    pub fn ocio_color_space_transform_set_src(transform: *mut c_void, src: *const i8);
    pub fn ocio_color_space_transform_get_dst(transform: *mut c_void) -> *const i8;
    pub fn ocio_color_space_transform_set_dst(transform: *mut c_void, dst: *const i8);
    pub fn ocio_color_space_transform_get_data_bypass(transform: *mut c_void) -> bool;
    pub fn ocio_color_space_transform_set_data_bypass(transform: *mut c_void, bypass: bool);
    pub fn ocio_color_space_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_color_space_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_color_space_transform_destroy(handle: *mut c_void);

    // --- LookTransform ---
    pub fn ocio_look_transform_create() -> *mut c_void;
    pub fn ocio_look_transform_get_src(transform: *mut c_void) -> *const i8;
    pub fn ocio_look_transform_set_src(transform: *mut c_void, src: *const i8);
    pub fn ocio_look_transform_get_dst(transform: *mut c_void) -> *const i8;
    pub fn ocio_look_transform_set_dst(transform: *mut c_void, dst: *const i8);
    pub fn ocio_look_transform_get_looks(transform: *mut c_void) -> *const i8;
    pub fn ocio_look_transform_set_looks(transform: *mut c_void, looks: *const i8);
    pub fn ocio_look_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_look_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_look_transform_destroy(handle: *mut c_void);

    // --- DisplayViewTransform ---
    pub fn ocio_display_view_transform_create() -> *mut c_void;
    pub fn ocio_display_view_transform_get_src(transform: *mut c_void) -> *const i8;
    pub fn ocio_display_view_transform_set_src(transform: *mut c_void, src: *const i8);
    pub fn ocio_display_view_transform_get_display(transform: *mut c_void) -> *const i8;
    pub fn ocio_display_view_transform_set_display(transform: *mut c_void, display: *const i8);
    pub fn ocio_display_view_transform_get_view(transform: *mut c_void) -> *const i8;
    pub fn ocio_display_view_transform_set_view(transform: *mut c_void, view: *const i8);
    pub fn ocio_display_view_transform_get_looks_bypass(transform: *mut c_void) -> bool;
    pub fn ocio_display_view_transform_set_looks_bypass(transform: *mut c_void, bypass: bool);
    pub fn ocio_display_view_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_display_view_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_display_view_transform_destroy(handle: *mut c_void);

    // --- GradingPrimaryTransform ---
    pub fn ocio_grading_primary_transform_create(style: i32) -> *mut c_void;
    pub fn ocio_grading_primary_transform_get_style(transform: *mut c_void) -> i32;
    pub fn ocio_grading_primary_transform_set_style(transform: *mut c_void, style: i32);
    pub fn ocio_grading_primary_transform_get_value(transform: *mut c_void, values: *mut f64);
    pub fn ocio_grading_primary_transform_set_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_grading_primary_transform_is_dynamic(transform: *mut c_void) -> bool;
    pub fn ocio_grading_primary_transform_make_dynamic(transform: *mut c_void);
    pub fn ocio_grading_primary_transform_make_non_dynamic(transform: *mut c_void);
    pub fn ocio_grading_primary_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_grading_primary_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_grading_primary_transform_destroy(handle: *mut c_void);

    // --- GradingToneTransform ---
    pub fn ocio_grading_tone_transform_create(style: i32) -> *mut c_void;
    pub fn ocio_grading_tone_transform_get_style(transform: *mut c_void) -> i32;
    pub fn ocio_grading_tone_transform_set_style(transform: *mut c_void, style: i32);
    pub fn ocio_grading_tone_transform_get_value(transform: *mut c_void, values: *mut f64);
    pub fn ocio_grading_tone_transform_set_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_grading_tone_transform_is_dynamic(transform: *mut c_void) -> bool;
    pub fn ocio_grading_tone_transform_make_dynamic(transform: *mut c_void);
    pub fn ocio_grading_tone_transform_make_non_dynamic(transform: *mut c_void);
    pub fn ocio_grading_tone_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_grading_tone_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_grading_tone_transform_destroy(handle: *mut c_void);

    // --- GradingRGBCurveTransform ---
    pub fn ocio_grading_rgb_curve_transform_create(style: i32) -> *mut c_void;
    pub fn ocio_grading_rgb_curve_transform_get_style(transform: *mut c_void) -> i32;
    pub fn ocio_grading_rgb_curve_transform_set_style(transform: *mut c_void, style: i32);
    pub fn ocio_grading_rgb_curve_transform_get_num_control_points(transform: *mut c_void, curveType: i32) -> i32;
    pub fn ocio_grading_rgb_curve_transform_get_control_point(transform: *mut c_void, curveType: i32, index: i32, x: *mut f32, y: *mut f32);
    pub fn ocio_grading_rgb_curve_transform_set_num_control_points(transform: *mut c_void, curveType: i32, num: i32);
    pub fn ocio_grading_rgb_curve_transform_set_control_point(transform: *mut c_void, curveType: i32, index: i32, x: f32, y: f32);
    pub fn ocio_grading_rgb_curve_transform_get_slope(transform: *mut c_void, curveType: i32, index: i32) -> f32;
    pub fn ocio_grading_rgb_curve_transform_set_slope(transform: *mut c_void, curveType: i32, index: i32, slope: f32);
    pub fn ocio_grading_rgb_curve_transform_slopes_are_default(transform: *mut c_void, curveType: i32) -> bool;
    pub fn ocio_grading_rgb_curve_transform_get_bypass_lin_to_log(transform: *mut c_void) -> bool;
    pub fn ocio_grading_rgb_curve_transform_set_bypass_lin_to_log(transform: *mut c_void, bypass: bool);
    pub fn ocio_grading_rgb_curve_transform_is_dynamic(transform: *mut c_void) -> bool;
    pub fn ocio_grading_rgb_curve_transform_make_dynamic(transform: *mut c_void);
    pub fn ocio_grading_rgb_curve_transform_make_non_dynamic(transform: *mut c_void);
    pub fn ocio_grading_rgb_curve_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_grading_rgb_curve_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_grading_rgb_curve_transform_destroy(handle: *mut c_void);

    // --- GradingHueCurveTransform ---
    pub fn ocio_grading_hue_curve_transform_create(style: i32) -> *mut c_void;
    pub fn ocio_grading_hue_curve_transform_get_style(transform: *mut c_void) -> i32;
    pub fn ocio_grading_hue_curve_transform_set_style(transform: *mut c_void, style: i32);
    pub fn ocio_grading_hue_curve_transform_get_num_control_points(transform: *mut c_void, curveType: i32) -> i32;
    pub fn ocio_grading_hue_curve_transform_get_control_point(transform: *mut c_void, curveType: i32, index: i32, x: *mut f32, y: *mut f32);
    pub fn ocio_grading_hue_curve_transform_set_num_control_points(transform: *mut c_void, curveType: i32, num: i32);
    pub fn ocio_grading_hue_curve_transform_set_control_point(transform: *mut c_void, curveType: i32, index: i32, x: f32, y: f32);
    pub fn ocio_grading_hue_curve_transform_get_slope(transform: *mut c_void, curveType: i32, index: i32) -> f32;
    pub fn ocio_grading_hue_curve_transform_set_slope(transform: *mut c_void, curveType: i32, index: i32, slope: f32);
    pub fn ocio_grading_hue_curve_transform_slopes_are_default(transform: *mut c_void, curveType: i32) -> bool;
    pub fn ocio_grading_hue_curve_transform_get_bypass_lin_to_log(transform: *mut c_void) -> bool;
    pub fn ocio_grading_hue_curve_transform_set_bypass_lin_to_log(transform: *mut c_void, bypass: bool);
    pub fn ocio_grading_hue_curve_transform_is_dynamic(transform: *mut c_void) -> bool;
    pub fn ocio_grading_hue_curve_transform_make_dynamic(transform: *mut c_void);
    pub fn ocio_grading_hue_curve_transform_make_non_dynamic(transform: *mut c_void);
    pub fn ocio_grading_hue_curve_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_grading_hue_curve_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_grading_hue_curve_transform_destroy(handle: *mut c_void);

    // --- AllocationTransform ---
    pub fn ocio_allocation_transform_create() -> *mut c_void;
    pub fn ocio_allocation_transform_get_allocation(transform: *mut c_void) -> i32;
    pub fn ocio_allocation_transform_set_allocation(transform: *mut c_void, allocation: i32);
    pub fn ocio_allocation_transform_get_num_vars(transform: *mut c_void) -> i32;
    pub fn ocio_allocation_transform_get_vars(transform: *mut c_void, vars: *mut f32);
    pub fn ocio_allocation_transform_set_vars(transform: *mut c_void, numvars: i32, vars: *const f32);
    pub fn ocio_allocation_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_allocation_transform_set_direction(transform: *mut c_void, direction: i32);
    pub fn ocio_allocation_transform_destroy(handle: *mut c_void);

    // --- LogAffineTransform ---
    pub fn ocio_log_affine_transform_create() -> *mut c_void;
    pub fn ocio_log_affine_transform_get_base(transform: *mut c_void) -> f64;
    pub fn ocio_log_affine_transform_set_base(transform: *mut c_void, base: f64);
    pub fn ocio_log_affine_transform_get_log_side_slope_value(transform: *mut c_void, values: *mut f64);
    pub fn ocio_log_affine_transform_set_log_side_slope_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_log_affine_transform_get_log_side_offset_value(transform: *mut c_void, values: *mut f64);
    pub fn ocio_log_affine_transform_set_log_side_offset_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_log_affine_transform_get_lin_side_slope_value(transform: *mut c_void, values: *mut f64);
    pub fn ocio_log_affine_transform_set_lin_side_slope_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_log_affine_transform_get_lin_side_offset_value(transform: *mut c_void, values: *mut f64);
    pub fn ocio_log_affine_transform_set_lin_side_offset_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_log_affine_transform_destroy(handle: *mut c_void);

    // --- LogCameraTransform ---
    pub fn ocio_log_camera_transform_create(linSideBreakValues: *const f64) -> *mut c_void;
    pub fn ocio_log_camera_transform_get_base(transform: *mut c_void) -> f64;
    pub fn ocio_log_camera_transform_set_base(transform: *mut c_void, base: f64);
    pub fn ocio_log_camera_transform_get_log_side_slope_value(transform: *mut c_void, values: *mut f64);
    pub fn ocio_log_camera_transform_set_log_side_slope_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_log_camera_transform_get_log_side_offset_value(transform: *mut c_void, values: *mut f64);
    pub fn ocio_log_camera_transform_set_log_side_offset_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_log_camera_transform_get_lin_side_slope_value(transform: *mut c_void, values: *mut f64);
    pub fn ocio_log_camera_transform_set_lin_side_slope_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_log_camera_transform_get_lin_side_offset_value(transform: *mut c_void, values: *mut f64);
    pub fn ocio_log_camera_transform_set_lin_side_offset_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_log_camera_transform_get_lin_side_break_value(transform: *mut c_void, values: *mut f64);
    pub fn ocio_log_camera_transform_set_lin_side_break_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_log_camera_transform_get_linear_slope_value(transform: *mut c_void, values: *mut f64) -> bool;
    pub fn ocio_log_camera_transform_set_linear_slope_value(transform: *mut c_void, values: *const f64);
    pub fn ocio_log_camera_transform_unset_linear_slope_value(transform: *mut c_void);
    pub fn ocio_log_camera_transform_destroy(handle: *mut c_void);

    // --- Config: active display/view setters ---
    pub fn ocio_config_set_active_displays(config: *mut c_void, displays: *const i8);
    pub fn ocio_config_set_active_views(config: *mut c_void, views: *const i8);

    // --- Config: display/view transform name queries ---
    pub fn ocio_config_get_display_view_transform_name(config: *mut c_void, display: *const i8, view: *const i8) -> *const i8;
    pub fn ocio_config_get_display_view_color_space_name(config: *mut c_void, display: *const i8, view: *const i8) -> *const i8;

    // --- Config: clear collections ---
    pub fn ocio_config_clear_color_spaces(config: *mut c_void);
    pub fn ocio_config_clear_looks(config: *mut c_void);

    // --- Config: default luma setter ---
    pub fn ocio_config_set_default_luma_coefs(config: *mut c_void, rgb: *const f64);

    // --- Config: display/view management ---
    pub fn ocio_config_add_display(config: *mut c_void, display: *const i8, view: *const i8, transformName: *const i8, rule: *const i8);
    pub fn ocio_config_add_shared_view(config: *mut c_void, display: *const i8, view: *const i8, transformName: *const i8, rule: *const i8);
    pub fn ocio_config_remove_display(config: *mut c_void, display: *const i8);
    pub fn ocio_config_remove_view(config: *mut c_void, display: *const i8, view: *const i8);

    // --- Config: named transforms ---
    pub fn ocio_config_get_num_named_transforms(config: *mut c_void) -> i32;
    pub fn ocio_config_get_named_transform_name_by_index(config: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_config_get_named_transform(config: *mut c_void, name: *const i8) -> *mut c_void;
    pub fn ocio_config_add_named_transform(config: *mut c_void, namedTransform: *mut c_void);
    pub fn ocio_config_remove_named_transform(config: *mut c_void, name: *const i8);

    // --- Config: view transforms ---
    pub fn ocio_config_get_num_view_transforms(config: *mut c_void) -> i32;
    pub fn ocio_config_get_view_transform_name_by_index(config: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_config_get_view_transform(config: *mut c_void, name: *const i8) -> *mut c_void;
    pub fn ocio_config_add_view_transform(config: *mut c_void, viewTransform: *mut c_void);
    pub fn ocio_config_remove_view_transform(config: *mut c_void, name: *const i8);

    // --- ColorSpace: aliases ---
    pub fn ocio_color_space_get_num_aliases(colorSpace: *mut c_void) -> i32;
    pub fn ocio_color_space_get_alias(colorSpace: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_color_space_add_alias(colorSpace: *mut c_void, alias: *const i8);
    pub fn ocio_color_space_remove_alias(colorSpace: *mut c_void, alias: *const i8);
    pub fn ocio_color_space_clear_aliases(colorSpace: *mut c_void);

    // --- ColorSpace: inactive ---
    pub fn ocio_color_space_is_inactive(colorSpace: *mut c_void) -> bool;
    pub fn ocio_color_space_set_inactive(colorSpace: *mut c_void, inactive: bool);

    // --- Look: aliases ---
    pub fn ocio_look_get_num_aliases(look: *mut c_void) -> i32;
    pub fn ocio_look_get_alias(look: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_look_add_alias(look: *mut c_void, alias: *const i8);
    pub fn ocio_look_remove_alias(look: *mut c_void, alias: *const i8);
    pub fn ocio_look_clear_aliases(look: *mut c_void);

    // --- Look: inactive ---
    pub fn ocio_look_is_inactive(look: *mut c_void) -> bool;
    pub fn ocio_look_set_inactive(look: *mut c_void, inactive: bool);

    // --- NamedTransform: aliases ---
    pub fn ocio_named_transform_get_num_aliases(namedTransform: *mut c_void) -> i32;
    pub fn ocio_named_transform_get_alias(namedTransform: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_named_transform_add_alias(namedTransform: *mut c_void, alias: *const i8);
    pub fn ocio_named_transform_remove_alias(namedTransform: *mut c_void, alias: *const i8);
    pub fn ocio_named_transform_clear_aliases(namedTransform: *mut c_void);

    // --- NamedTransform: inactive & category ---
    pub fn ocio_named_transform_is_inactive(namedTransform: *mut c_void) -> bool;
    pub fn ocio_named_transform_set_inactive(namedTransform: *mut c_void, inactive: bool);
    pub fn ocio_named_transform_get_category(namedTransform: *mut c_void) -> *const i8;
    pub fn ocio_named_transform_set_category(namedTransform: *mut c_void, category: *const i8);

    // --- ViewTransform: aliases ---
    pub fn ocio_view_transform_get_num_aliases(viewTransform: *mut c_void) -> i32;
    pub fn ocio_view_transform_get_alias(viewTransform: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_view_transform_add_alias(viewTransform: *mut c_void, alias: *const i8);
    pub fn ocio_view_transform_remove_alias(viewTransform: *mut c_void, alias: *const i8);
    pub fn ocio_view_transform_clear_aliases(viewTransform: *mut c_void);

    // --- ViewTransform: inactive, category, description ---
    pub fn ocio_view_transform_is_inactive(viewTransform: *mut c_void) -> bool;
    pub fn ocio_view_transform_set_inactive(viewTransform: *mut c_void, inactive: bool);
    pub fn ocio_view_transform_get_category(viewTransform: *mut c_void) -> *const i8;
    pub fn ocio_view_transform_set_category(viewTransform: *mut c_void, category: *const i8);
    pub fn ocio_view_transform_get_description(viewTransform: *mut c_void) -> *const i8;
    pub fn ocio_view_transform_set_description(viewTransform: *mut c_void, description: *const i8);

    // --- ViewTransform: editable copy ---
    pub fn ocio_view_transform_create_editable_copy(viewTransform: *mut c_void) -> *mut c_void;

    // --- FileRules ---
    pub fn ocio_file_rules_create() -> *mut c_void;
    pub fn ocio_file_rules_create_editable_copy(rules: *mut c_void) -> *mut c_void;
    pub fn ocio_file_rules_get_num_entries(rules: *mut c_void) -> u64;
    pub fn ocio_file_rules_get_index_for_rule(rules: *mut c_void, ruleName: *const i8) -> u64;
    pub fn ocio_file_rules_get_name(rules: *mut c_void, ruleIndex: u64) -> *const i8;
    pub fn ocio_file_rules_get_pattern(rules: *mut c_void, ruleIndex: u64) -> *const i8;
    pub fn ocio_file_rules_set_pattern(rules: *mut c_void, ruleIndex: u64, pattern: *const i8);
    pub fn ocio_file_rules_get_extension(rules: *mut c_void, ruleIndex: u64) -> *const i8;
    pub fn ocio_file_rules_set_extension(rules: *mut c_void, ruleIndex: u64, extension: *const i8);
    pub fn ocio_file_rules_get_regex(rules: *mut c_void, ruleIndex: u64) -> *const i8;
    pub fn ocio_file_rules_set_regex(rules: *mut c_void, ruleIndex: u64, regex: *const i8);
    pub fn ocio_file_rules_get_color_space(rules: *mut c_void, ruleIndex: u64) -> *const i8;
    pub fn ocio_file_rules_set_color_space(rules: *mut c_void, ruleIndex: u64, colorSpace: *const i8);
    pub fn ocio_file_rules_get_num_custom_keys(rules: *mut c_void, ruleIndex: u64) -> u64;
    pub fn ocio_file_rules_get_custom_key_name(rules: *mut c_void, ruleIndex: u64, key: u64) -> *const i8;
    pub fn ocio_file_rules_get_custom_key_value(rules: *mut c_void, ruleIndex: u64, key: u64) -> *const i8;
    pub fn ocio_file_rules_set_custom_key(rules: *mut c_void, ruleIndex: u64, key: *const i8, value: *const i8);
    pub fn ocio_file_rules_insert_rule(rules: *mut c_void, ruleIndex: u64, name: *const i8, colorSpace: *const i8, pattern: *const i8, extension: *const i8);
    pub fn ocio_file_rules_insert_rule_regex(rules: *mut c_void, ruleIndex: u64, name: *const i8, colorSpace: *const i8, regex: *const i8);
    pub fn ocio_file_rules_insert_path_search_rule(rules: *mut c_void, ruleIndex: u64);
    pub fn ocio_file_rules_set_default_rule_color_space(rules: *mut c_void, colorSpace: *const i8);
    pub fn ocio_file_rules_remove_rule(rules: *mut c_void, ruleIndex: u64);
    pub fn ocio_file_rules_increase_rule_priority(rules: *mut c_void, ruleIndex: u64);
    pub fn ocio_file_rules_decrease_rule_priority(rules: *mut c_void, ruleIndex: u64);
    pub fn ocio_file_rules_is_default(rules: *mut c_void) -> bool;
    pub fn ocio_file_rules_destroy(handle: *mut c_void);

    // --- Config: FileRules ---
    pub fn ocio_config_get_file_rules(config: *mut c_void) -> *mut c_void;

    // --- Lut1DTransform: data ---
    pub fn ocio_lut1d_transform_get_length(transform: *mut c_void) -> u64;
    pub fn ocio_lut1d_transform_get_values(transform: *mut c_void, data: *mut f64);
    pub fn ocio_lut1d_transform_set_length(transform: *mut c_void, len: u64);
    pub fn ocio_lut1d_transform_set_values(transform: *mut c_void, data: *const f64);

    // --- Lut3DTransform: data ---
    pub fn ocio_lut3d_transform_get_grid_size(transform: *mut c_void) -> u64;
    pub fn ocio_lut3d_transform_get_values(transform: *mut c_void, data: *mut f64);
    pub fn ocio_lut3d_transform_set_grid_size(transform: *mut c_void, size: u64);
    pub fn ocio_lut3d_transform_set_values(transform: *mut c_void, data: *const f64);

    // --- GroupTransform: remove/clear ---
    pub fn ocio_group_transform_remove_transform(transform: *mut c_void, index: u64);
    pub fn ocio_group_transform_clear_transforms(transform: *mut c_void);

    // --- Config: clear all, version setters, interpolation, working dir ---
    pub fn ocio_config_clear_all(config: *mut c_void);
    pub fn ocio_config_set_major_version(config: *mut c_void, version: u32);
    pub fn ocio_config_set_minor_version(config: *mut c_void, version: u32);
    pub fn ocio_config_get_default_interpolation(config: *mut c_void) -> i32;
    pub fn ocio_config_set_default_interpolation(config: *mut c_void, interpolation: i32);
    pub fn ocio_config_get_working_dir(config: *mut c_void) -> *const i8;
    pub fn ocio_config_set_working_dir(config: *mut c_void, dirName: *const i8);

    // --- Config: inactive color spaces, archivable, processor cache, file rules ---
    pub fn ocio_config_set_inactive_color_spaces(config: *mut c_void, inactive: *const i8);
    pub fn ocio_config_is_archivable(config: *mut c_void) -> bool;
    pub fn ocio_config_clear_processor_cache(config: *mut c_void);
    pub fn ocio_config_set_file_rules(config: *mut c_void, fileRules: *mut c_void);

    // --- ColorSpace: visibility + set_reference_space_type ---
    pub fn ocio_color_space_get_visibility(colorSpace: *mut c_void) -> i32;
    pub fn ocio_color_space_set_visibility(colorSpace: *mut c_void, visibility: i32);
    pub fn ocio_color_space_set_reference_space_type(colorSpace: *mut c_void, referenceSpace: i32);

    // --- LookTransform: skip color space conversion ---
    pub fn ocio_look_transform_get_skip_color_space_conversion(transform: *mut c_void) -> bool;
    pub fn ocio_look_transform_set_skip_color_space_conversion(transform: *mut c_void, skip: bool);

    // --- Global: processor cache flags ---
    pub fn ocio_get_processor_cache_flags() -> i32;
    pub fn ocio_set_processor_cache_flags(flags: i32);

    // --- CPUProcessor: batch pixel processing ---
    pub fn ocio_cpu_processor_apply_rgba_pixels(cpu_processor: *mut c_void, rgba: *mut f32, numPixels: i64, stride: i64);
    pub fn ocio_cpu_processor_apply_rgb_pixels(cpu_processor: *mut c_void, rgb: *mut f32, numPixels: i64, stride: i64);

    // --- Processor: batch pixel processing ---
    pub fn ocio_processor_apply_rgba_pixels(processor: *mut c_void, rgba: *mut f32, numPixels: i64, stride: i64);

    // --- Baker: format metadata ---
    pub fn ocio_baker_get_format_metadata(baker: *mut c_void) -> *mut c_void;

    // --- ColorSpaceSet ---
    pub fn ocio_color_space_set_create() -> *mut c_void;
    pub fn ocio_color_space_set_create_editable_copy(set: *mut c_void) -> *mut c_void;
    pub fn ocio_color_space_set_get_num_color_spaces(set: *mut c_void) -> i32;
    pub fn ocio_color_space_set_get_color_space_name_by_index(set: *mut c_void, index: i32) -> *const i8;
    pub fn ocio_color_space_set_get_color_space_by_index(set: *mut c_void, index: i32) -> *mut c_void;
    pub fn ocio_color_space_set_get_color_space(set: *mut c_void, name: *const i8) -> *mut c_void;
    pub fn ocio_color_space_set_get_color_space_index(set: *mut c_void, name: *const i8) -> i32;
    pub fn ocio_color_space_set_has_color_space(set: *mut c_void, name: *const i8) -> bool;
    pub fn ocio_color_space_set_destroy(handle: *mut c_void);

    // --- Config: ColorSpaceSet ---
    pub fn ocio_config_get_color_space_set(config: *mut c_void, search: *const i8) -> *mut c_void;

    // --- Transform: format metadata ---
    pub fn ocio_transform_get_format_metadata(transform: *mut c_void) -> *mut c_void;

    // --- FormatMetadata ---
    pub fn ocio_format_metadata_get_element_name(metadata: *mut c_void) -> *const i8;
    pub fn ocio_format_metadata_set_element_name(metadata: *mut c_void, name: *const i8);
    pub fn ocio_format_metadata_get_element_value(metadata: *mut c_void) -> *const i8;
    pub fn ocio_format_metadata_set_element_value(metadata: *mut c_void, value: *const i8);
    pub fn ocio_format_metadata_get_num_attributes(metadata: *mut c_void) -> i32;
    pub fn ocio_format_metadata_get_attribute_name(metadata: *mut c_void, i: i32) -> *const i8;
    pub fn ocio_format_metadata_get_attribute_value_by_index(metadata: *mut c_void, i: i32) -> *const i8;
    pub fn ocio_format_metadata_get_attribute_value(metadata: *mut c_void, name: *const i8) -> *const i8;
    pub fn ocio_format_metadata_add_attribute(metadata: *mut c_void, name: *const i8, value: *const i8);
    pub fn ocio_format_metadata_get_num_children_elements(metadata: *mut c_void) -> i32;
    pub fn ocio_format_metadata_get_child_element(metadata: *mut c_void, i: i32) -> *mut c_void;
    pub fn ocio_format_metadata_add_child_element(metadata: *mut c_void, name: *const i8, value: *const i8);
    pub fn ocio_format_metadata_clear(metadata: *mut c_void);
    pub fn ocio_format_metadata_get_name(metadata: *mut c_void) -> *const i8;
    pub fn ocio_format_metadata_set_name(metadata: *mut c_void, name: *const i8);
    pub fn ocio_format_metadata_get_id(metadata: *mut c_void) -> *const i8;
    pub fn ocio_format_metadata_set_id(metadata: *mut c_void, id: *const i8);
    pub fn ocio_format_metadata_destroy(handle: *mut c_void);

    // --- BuiltinTransform: description ---
    pub fn ocio_builtin_transform_get_description(transform: *mut c_void) -> *const i8;

    // --- DisplayViewTransform: data bypass ---
    pub fn ocio_display_view_transform_get_data_bypass(transform: *mut c_void) -> bool;
    pub fn ocio_display_view_transform_set_data_bypass(transform: *mut c_void, bypass: bool);

    // --- ExposureContrastTransform: log & non-dynamic ---
    pub fn ocio_exposure_contrast_transform_get_log_exposure_step(transform: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_log_exposure_step(transform: *mut c_void, step: f64);
    pub fn ocio_exposure_contrast_transform_get_log_mid_gray(transform: *mut c_void) -> f64;
    pub fn ocio_exposure_contrast_transform_set_log_mid_gray(transform: *mut c_void, mid_gray: f64);
    pub fn ocio_exposure_contrast_transform_make_exposure_non_dynamic(transform: *mut c_void);
    pub fn ocio_exposure_contrast_transform_make_contrast_non_dynamic(transform: *mut c_void);
    pub fn ocio_exposure_contrast_transform_make_gamma_non_dynamic(transform: *mut c_void);

    // --- LogAffineTransform: direction ---
    pub fn ocio_log_affine_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_log_affine_transform_set_direction(transform: *mut c_void, direction: i32);

    // --- LogCameraTransform: direction ---
    pub fn ocio_log_camera_transform_get_direction(transform: *mut c_void) -> i32;
    pub fn ocio_log_camera_transform_set_direction(transform: *mut c_void, direction: i32);

    // --- Lut1DTransform: half domain, raw halfs, hue adjust ---
    pub fn ocio_lut1d_transform_get_input_half_domain(transform: *mut c_void) -> bool;
    pub fn ocio_lut1d_transform_set_input_half_domain(transform: *mut c_void, half_domain: bool);
    pub fn ocio_lut1d_transform_get_output_raw_halfs(transform: *mut c_void) -> bool;
    pub fn ocio_lut1d_transform_set_output_raw_halfs(transform: *mut c_void, raw_halfs: bool);
    pub fn ocio_lut1d_transform_get_hue_adjust(transform: *mut c_void) -> i32;
    pub fn ocio_lut1d_transform_set_hue_adjust(transform: *mut c_void, hue_adjust: i32);

    // --- MatrixTransform: bit depth & static helpers ---
    pub fn ocio_matrix_transform_get_file_input_bit_depth(transform: *mut c_void) -> i32;
    pub fn ocio_matrix_transform_set_file_input_bit_depth(transform: *mut c_void, bit_depth: i32);
    pub fn ocio_matrix_transform_get_file_output_bit_depth(transform: *mut c_void) -> i32;
    pub fn ocio_matrix_transform_set_file_output_bit_depth(transform: *mut c_void, bit_depth: i32);
    pub fn ocio_matrix_transform_create_fit(inputColorSpace: *const i8, outputColorSpace: *const i8) -> *mut c_void;
    pub fn ocio_matrix_transform_create_identity() -> *mut c_void;
    pub fn ocio_matrix_transform_create_sat(sat: f64, luma: *const f64) -> *mut c_void;
    pub fn ocio_matrix_transform_create_scale(scale: *const f64) -> *mut c_void;
    pub fn ocio_matrix_transform_create_view(channels: *mut i32, gamma: *const i8) -> *mut c_void;

    // --- RangeTransform: has/unset value & bit depth ---
    pub fn ocio_range_transform_has_min_in_value(transform: *mut c_void) -> bool;
    pub fn ocio_range_transform_unset_min_in_value(transform: *mut c_void);
    pub fn ocio_range_transform_has_max_in_value(transform: *mut c_void) -> bool;
    pub fn ocio_range_transform_unset_max_in_value(transform: *mut c_void);
    pub fn ocio_range_transform_has_min_out_value(transform: *mut c_void) -> bool;
    pub fn ocio_range_transform_unset_min_out_value(transform: *mut c_void);
    pub fn ocio_range_transform_has_max_out_value(transform: *mut c_void) -> bool;
    pub fn ocio_range_transform_unset_max_out_value(transform: *mut c_void);
    pub fn ocio_range_transform_get_file_input_bit_depth(transform: *mut c_void) -> i32;
    pub fn ocio_range_transform_set_file_input_bit_depth(transform: *mut c_void, bit_depth: i32);
    pub fn ocio_range_transform_get_file_output_bit_depth(transform: *mut c_void) -> i32;
    pub fn ocio_range_transform_set_file_output_bit_depth(transform: *mut c_void, bit_depth: i32);

    // --- CDLTransform: SOP & description ---
    pub fn ocio_cdl_transform_get_sop(transform: *mut c_void, vec9: *mut f64);
    pub fn ocio_cdl_transform_set_sop(transform: *mut c_void, vec9: *const f64);
    pub fn ocio_cdl_transform_get_first_sop_description(transform: *mut c_void) -> *const i8;
    pub fn ocio_cdl_transform_set_first_sop_description(transform: *mut c_void, description: *const i8);
}
