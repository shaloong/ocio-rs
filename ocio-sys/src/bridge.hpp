#pragma once

#include <stddef.h>
#include <stdbool.h>

#ifndef BEGIN_TRY
#define BEGIN_TRY try {
#endif

#ifndef END_TRY
#define END_TRY(return_value) \
  } catch (...) {             \
    return (return_value);    \
  }
#endif

#ifndef END_TRY_VOID
#define END_TRY_VOID \
  } catch (...) {}
#endif

extern "C" {

// --- Runtime ---
bool ocio_runtime_is_stub(void);

// --- BuiltinConfigRegistry ---
void* ocio_builtin_config_registry_get(void);
int ocio_builtin_config_registry_get_num_builtin_configs(void* registry);
const char* ocio_builtin_config_registry_get_config_name(void* registry, int index);
const char* ocio_builtin_config_registry_get_config_ui_name(void* registry, int index);
bool ocio_builtin_config_registry_is_config_recommended(void* registry, int index);
void* ocio_builtin_config_registry_get_config_by_index(void* registry, int index);
void* ocio_builtin_config_registry_get_config_by_name(void* registry, const char* name);

// --- Global utility functions ---
const char* ocio_get_version(void);
int ocio_get_version_hex(void);
int ocio_get_logging_level(void);
void ocio_set_logging_level(int level);
void ocio_set_logging_level_to_override(int level);

// --- Config ---
void* ocio_config_create_raw(void);
void* ocio_config_create_from_file(const char* path);
void ocio_config_destroy(void* handle);

// Config: name & metadata
const char* ocio_config_get_name(void* config);
void ocio_config_set_name(void* config, const char* name);
const char* ocio_config_get_description(void* config);
void ocio_config_set_description(void* config, const char* desc);
const char* ocio_config_get_cache_id(void* config);

// Config: version
unsigned int ocio_config_get_major_version(void* config);
unsigned int ocio_config_get_minor_version(void* config);
char ocio_config_get_family_separator(void* config);

// Config: color spaces
int ocio_config_get_num_color_spaces(void* config);
const char* ocio_config_get_color_space_name_by_index(void* config, int index);
const char* ocio_config_get_canonical_name(void* config, const char* name);
bool ocio_config_is_color_space_linear(void* config, const char* colorSpace, int referenceSpaceType);
const char* ocio_config_get_color_space_from_filepath(void* config, const char* filePath);

// Config: displays
const char* ocio_config_get_default_display(void* config);
int ocio_config_get_num_displays(void* config);
const char* ocio_config_get_display(void* config, int index);

// Config: views
const char* ocio_config_get_default_view(void* config, const char* display);
int ocio_config_get_num_views(void* config, const char* display);
const char* ocio_config_get_view(void* config, const char* display, int index);

// Config: looks
int ocio_config_get_num_looks(void* config);
const char* ocio_config_get_look_name_by_index(void* config, int index);

// Config: luma coefs
void ocio_config_get_default_luma_coefs(void* config, double* rgb);

// Config: roles
int ocio_config_get_num_roles(void* config);
bool ocio_config_has_role(void* config, const char* role);
const char* ocio_config_get_role_name(void* config, int index);
const char* ocio_config_get_role_color_space_by_index(void* config, int index);
const char* ocio_config_get_role_color_space_by_name(void* config, const char* roleName);

// Config: active displays / views
const char* ocio_config_get_active_displays(void* config);
const char* ocio_config_get_active_views(void* config);

// Config: search paths & strict parsing
const char* ocio_config_get_search_path(void* config);
void ocio_config_set_search_path(void* config, const char* path);
bool ocio_config_is_strict_parsing_enabled(void* config);
void ocio_config_set_strict_parsing_enabled(void* config, bool enabled);

// Config: roles (mutable)
void ocio_config_set_role(void* config, const char* role, const char* colorSpace);

// Config: family separator
void ocio_config_set_family_separator(void* config, char separator);

// Config: validate & serialize
const char* ocio_config_validate(void* config);
const char* ocio_config_serialize(void* config);

// Config: editable copy
void* ocio_config_create_editable_copy(void* config);

// Config: context
void* ocio_config_get_current_context(void* config);
void ocio_config_set_current_context(void* config, void* context);

// Config: processors
void* ocio_config_get_processor(
    void* config, const char* src, const char* dst);
void* ocio_config_get_processor_display(
    void* config, const char* src, const char* display, const char* view, int direction);

// --- Processor ---
void* ocio_processor_get_default_cpu_processor(void* processor);
void* ocio_processor_get_optimized_cpu_processor(void* processor, unsigned long flags);
bool ocio_processor_is_no_op(void* processor);
bool ocio_processor_has_channel_crosstalk(void* processor);
const char* ocio_processor_get_cache_id(void* processor);
void ocio_processor_apply_rgba(void* processor, float* rgba, size_t len);
int ocio_processor_get_num_transforms(void* processor);
void* ocio_processor_create_group_transform(void* processor);
void ocio_processor_destroy(void* handle);

// --- CPUProcessor ---
void ocio_cpu_processor_apply_rgba(void* cpu_processor, float* rgba);
void ocio_cpu_processor_apply_rgb(void* cpu_processor, float* rgb);
bool ocio_cpu_processor_is_no_op(void* cpu_processor);
bool ocio_cpu_processor_has_channel_crosstalk(void* cpu_processor);
const char* ocio_cpu_processor_get_cache_id(void* cpu_processor);
int ocio_cpu_processor_get_input_bit_depth(void* cpu_processor);
int ocio_cpu_processor_get_output_bit_depth(void* cpu_processor);
bool ocio_cpu_processor_is_identity(void* cpu_processor);
void ocio_cpu_processor_destroy(void* handle);

// --- GPUProcessor ---
void* ocio_processor_get_default_gpu_processor(void* processor);
void* ocio_processor_get_optimized_gpu_processor(void* processor, unsigned long flags);
bool ocio_gpu_processor_is_no_op(void* gpu_processor);
bool ocio_gpu_processor_has_channel_crosstalk(void* gpu_processor);
const char* ocio_gpu_processor_get_cache_id(void* gpu_processor);
void ocio_gpu_processor_extract_shader_info(void* gpu_processor, void* shader_desc);
void ocio_gpu_processor_destroy(void* handle);

// --- GpuShaderDesc ---
void* ocio_gpu_shader_desc_create(void);
const char* ocio_gpu_shader_desc_get_shader_text(void* shader_desc);
int ocio_gpu_shader_desc_get_language(void* shader_desc);
void ocio_gpu_shader_desc_set_language(void* shader_desc, int language);
const char* ocio_gpu_shader_desc_get_function_name(void* shader_desc);
void ocio_gpu_shader_desc_set_function_name(void* shader_desc, const char* name);
const char* ocio_gpu_shader_desc_get_pixel_name(void* shader_desc);
void ocio_gpu_shader_desc_set_pixel_name(void* shader_desc, const char* name);
const char* ocio_gpu_shader_desc_get_resource_prefix(void* shader_desc);
void ocio_gpu_shader_desc_set_resource_prefix(void* shader_desc, const char* prefix);
unsigned int ocio_gpu_shader_desc_get_num_textures(void* shader_desc);
void ocio_gpu_shader_desc_get_texture(
    void* shader_desc, unsigned int index,
    const char** textureName, const char** samplerName,
    unsigned int* width, unsigned int* height,
    int* channel, int* dimensions, int* interpolation);
const float* ocio_gpu_shader_desc_get_texture_values(void* shader_desc, unsigned int index);
void ocio_gpu_shader_desc_finalize(void* shader_desc);
void ocio_gpu_shader_desc_destroy(void* handle);

// --- Transform base ---
int ocio_transform_get_direction(void* transform);
void ocio_transform_set_direction(void* transform, int direction);
int ocio_transform_get_transform_type(void* transform);
void* ocio_transform_create_editable_copy(void* transform);
void ocio_transform_destroy(void* handle);

// --- FileTransform ---
void* ocio_file_transform_create(void);
const char* ocio_file_transform_get_src(void* transform);
void ocio_file_transform_set_src(void* transform, const char* src);
const char* ocio_file_transform_get_ccc_id(void* transform);
void ocio_file_transform_set_ccc_id(void* transform, const char* id);
int ocio_file_transform_get_interpolation(void* transform);
void ocio_file_transform_set_interpolation(void* transform, int interp);
int ocio_file_transform_get_cdl_style(void* transform);
void ocio_file_transform_set_cdl_style(void* transform, int style);
int ocio_file_transform_get_direction(void* transform);
void ocio_file_transform_set_direction(void* transform, int direction);
void ocio_file_transform_destroy(void* handle);

// --- CDLTransform ---
void* ocio_cdl_transform_create(void);
void* ocio_cdl_transform_create_from_file(const char* src, const char* cccId);
void ocio_cdl_transform_get_slope(void* transform, double* rgb);
void ocio_cdl_transform_set_slope(void* transform, const double* rgb);
void ocio_cdl_transform_get_offset(void* transform, double* rgb);
void ocio_cdl_transform_set_offset(void* transform, const double* rgb);
void ocio_cdl_transform_get_power(void* transform, double* rgb);
void ocio_cdl_transform_set_power(void* transform, const double* rgb);
double ocio_cdl_transform_get_sat(void* transform);
void ocio_cdl_transform_set_sat(void* transform, double sat);
void ocio_cdl_transform_get_sat_luma_coefs(void* transform, double* rgb);
void ocio_cdl_transform_set_sat_luma_coefs(void* transform, const double* rgb);
int ocio_cdl_transform_get_style(void* transform);
void ocio_cdl_transform_set_style(void* transform, int style);
const char* ocio_cdl_transform_get_id(void* transform);
void ocio_cdl_transform_set_id(void* transform, const char* id);
int ocio_cdl_transform_get_direction(void* transform);
void ocio_cdl_transform_set_direction(void* transform, int direction);
void ocio_cdl_transform_destroy(void* handle);

// --- ExponentTransform ---
void* ocio_exponent_transform_create(void);
void ocio_exponent_transform_get_value(void* transform, double* vec4);
void ocio_exponent_transform_set_value(void* transform, const double* vec4);
int ocio_exponent_transform_get_negative_style(void* transform);
void ocio_exponent_transform_set_negative_style(void* transform, int style);
int ocio_exponent_transform_get_direction(void* transform);
void ocio_exponent_transform_set_direction(void* transform, int direction);
void ocio_exponent_transform_destroy(void* handle);

// --- ExponentWithLinearTransform ---
void* ocio_exponent_with_linear_transform_create(void);
void ocio_exponent_with_linear_transform_get_gamma(void* transform, double* vec4);
void ocio_exponent_with_linear_transform_set_gamma(void* transform, const double* vec4);
void ocio_exponent_with_linear_transform_get_offset(void* transform, double* vec4);
void ocio_exponent_with_linear_transform_set_offset(void* transform, const double* vec4);
int ocio_exponent_with_linear_transform_get_negative_style(void* transform);
void ocio_exponent_with_linear_transform_set_negative_style(void* transform, int style);
int ocio_exponent_with_linear_transform_get_direction(void* transform);
void ocio_exponent_with_linear_transform_set_direction(void* transform, int direction);
void ocio_exponent_with_linear_transform_destroy(void* handle);

// --- MatrixTransform ---
void* ocio_matrix_transform_create(void);
void ocio_matrix_transform_get_matrix(void* transform, double* m44);
void ocio_matrix_transform_set_matrix(void* transform, const double* m44);
void ocio_matrix_transform_get_offset(void* transform, double* offset4);
void ocio_matrix_transform_set_offset(void* transform, const double* offset4);
int ocio_matrix_transform_get_direction(void* transform);
void ocio_matrix_transform_set_direction(void* transform, int direction);
void ocio_matrix_transform_destroy(void* handle);

// --- LogTransform ---
void* ocio_log_transform_create(void);
double ocio_log_transform_get_base(void* transform);
void ocio_log_transform_set_base(void* transform, double base);
int ocio_log_transform_get_direction(void* transform);
void ocio_log_transform_set_direction(void* transform, int direction);
void ocio_log_transform_destroy(void* handle);

// --- RangeTransform ---
void* ocio_range_transform_create(void);
int ocio_range_transform_get_style(void* transform);
void ocio_range_transform_set_style(void* transform, int style);
double ocio_range_transform_get_min_in_value(void* transform);
void ocio_range_transform_set_min_in_value(void* transform, double value);
double ocio_range_transform_get_max_in_value(void* transform);
void ocio_range_transform_set_max_in_value(void* transform, double value);
double ocio_range_transform_get_min_out_value(void* transform);
void ocio_range_transform_set_min_out_value(void* transform, double value);
double ocio_range_transform_get_max_out_value(void* transform);
void ocio_range_transform_set_max_out_value(void* transform, double value);
int ocio_range_transform_get_direction(void* transform);
void ocio_range_transform_set_direction(void* transform, int direction);
void ocio_range_transform_destroy(void* handle);

// --- GroupTransform ---
void* ocio_group_transform_create(void);
int ocio_group_transform_get_num_transforms(void* transform);
void* ocio_group_transform_get_transform(void* transform, int index);
void ocio_group_transform_append_transform(void* transform, void* child);
void ocio_group_transform_prepend_transform(void* transform, void* child);
int ocio_group_transform_get_direction(void* transform);
void ocio_group_transform_set_direction(void* transform, int direction);
void ocio_group_transform_destroy(void* handle);

// --- BuiltinTransform ---
void* ocio_builtin_transform_create(void);
const char* ocio_builtin_transform_get_style(void* transform);
void ocio_builtin_transform_set_style(void* transform, const char* style);
int ocio_builtin_transform_get_direction(void* transform);
void ocio_builtin_transform_set_direction(void* transform, int direction);
void ocio_builtin_transform_destroy(void* handle);

// --- FixedFunctionTransform ---
void* ocio_fixed_function_transform_create(int style);
void* ocio_fixed_function_transform_create_with_params(int style, const double* params, int num_params);
int ocio_fixed_function_transform_get_style(void* transform);
void ocio_fixed_function_transform_set_style(void* transform, int style);
int ocio_fixed_function_transform_get_num_params(void* transform);
void ocio_fixed_function_transform_get_params(void* transform, double* params);
void ocio_fixed_function_transform_set_params(void* transform, const double* params, int num_params);
int ocio_fixed_function_transform_get_direction(void* transform);
void ocio_fixed_function_transform_set_direction(void* transform, int direction);
void ocio_fixed_function_transform_destroy(void* handle);

// --- Lut1DTransform ---
void* ocio_lut1d_transform_create(void);
int ocio_lut1d_transform_get_interpolation(void* transform);
void ocio_lut1d_transform_set_interpolation(void* transform, int interpolation);
int ocio_lut1d_transform_get_file_output_bit_depth(void* transform);
void ocio_lut1d_transform_set_file_output_bit_depth(void* transform, int bit_depth);
int ocio_lut1d_transform_get_direction(void* transform);
void ocio_lut1d_transform_set_direction(void* transform, int direction);
void ocio_lut1d_transform_destroy(void* handle);

// --- Lut3DTransform ---
void* ocio_lut3d_transform_create(void);
int ocio_lut3d_transform_get_interpolation(void* transform);
void ocio_lut3d_transform_set_interpolation(void* transform, int interpolation);
int ocio_lut3d_transform_get_file_output_bit_depth(void* transform);
void ocio_lut3d_transform_set_file_output_bit_depth(void* transform, int bit_depth);
int ocio_lut3d_transform_get_direction(void* transform);
void ocio_lut3d_transform_set_direction(void* transform, int direction);
void ocio_lut3d_transform_destroy(void* handle);

// --- Baker ---
void* ocio_baker_create(void);
void* ocio_baker_create_editable_copy(void* baker);
void ocio_baker_set_config(void* baker, void* config);
void* ocio_baker_get_config(void* baker);
const char* ocio_baker_get_format(void* baker);
void ocio_baker_set_format(void* baker, const char* formatName);
const char* ocio_baker_get_input_space(void* baker);
void ocio_baker_set_input_space(void* baker, const char* inputSpace);
const char* ocio_baker_get_shaper_space(void* baker);
void ocio_baker_set_shaper_space(void* baker, const char* shaperSpace);
const char* ocio_baker_get_looks(void* baker);
void ocio_baker_set_looks(void* baker, const char* looks);
const char* ocio_baker_get_target_space(void* baker);
void ocio_baker_set_target_space(void* baker, const char* targetSpace);
const char* ocio_baker_get_display(void* baker);
const char* ocio_baker_get_view(void* baker);
void ocio_baker_set_display_view(void* baker, const char* display, const char* view);
int ocio_baker_get_shaper_size(void* baker);
void ocio_baker_set_shaper_size(void* baker, int size);
int ocio_baker_get_cube_size(void* baker);
void ocio_baker_set_cube_size(void* baker, int size);
void ocio_baker_bake(void* baker, const char* outputPath);
void ocio_baker_destroy(void* handle);

// --- Context ---
void* ocio_context_create(void);
void* ocio_context_create_editable_copy(void* context);
const char* ocio_context_get_cache_id(void* context);
const char* ocio_context_get_search_path(void* context);
void ocio_context_set_search_path(void* context, const char* path);
int ocio_context_get_num_search_paths(void* context);
const char* ocio_context_get_search_path_by_index(void* context, int index);
const char* ocio_context_get_working_dir(void* context);
void ocio_context_set_working_dir(void* context, const char* dirname);
const char* ocio_context_get_string_var(void* context, const char* name);
void ocio_context_set_string_var(void* context, const char* name, const char* value);
int ocio_context_get_num_string_vars(void* context);
const char* ocio_context_get_string_var_name_by_index(void* context, int index);
const char* ocio_context_get_string_var_by_index(void* context, int index);
const char* ocio_context_resolve_string_var(void* context, const char* string);
const char* ocio_context_resolve_file_location(void* context, const char* filename);
void ocio_context_clear_string_vars(void* context);
void ocio_context_set_environment_mode(void* context, int mode);
int ocio_context_get_environment_mode(void* context);
void ocio_context_load_environment(void* context);
void ocio_context_destroy(void* handle);

// --- ColorSpace ---
void* ocio_color_space_create(void);
void* ocio_color_space_create_editable_copy(void* colorSpace);
const char* ocio_color_space_get_name(void* colorSpace);
void ocio_color_space_set_name(void* colorSpace, const char* name);
const char* ocio_color_space_get_family(void* colorSpace);
void ocio_color_space_set_family(void* colorSpace, const char* family);
const char* ocio_color_space_get_equality_group(void* colorSpace);
void ocio_color_space_set_equality_group(void* colorSpace, const char* group);
const char* ocio_color_space_get_description(void* colorSpace);
void ocio_color_space_set_description(void* colorSpace, const char* description);
int ocio_color_space_get_bit_depth(void* colorSpace);
void ocio_color_space_set_bit_depth(void* colorSpace, int bitDepth);
int ocio_color_space_get_reference_space_type(void* colorSpace);
bool ocio_color_space_is_data(void* colorSpace);
void ocio_color_space_set_is_data(void* colorSpace, bool isData);
int ocio_color_space_get_allocation(void* colorSpace);
void ocio_color_space_set_allocation(void* colorSpace, int allocation);
int ocio_color_space_get_allocation_num_vars(void* colorSpace);
void ocio_color_space_get_allocation_vars(void* colorSpace, float* vars);
void ocio_color_space_set_allocation_vars(void* colorSpace, int numVars, const float* vars);
void* ocio_color_space_get_transform(void* colorSpace, int direction);
void ocio_color_space_set_transform(void* colorSpace, const void* transform, int direction);
const char* ocio_color_space_get_encoding(void* colorSpace);
void ocio_color_space_set_encoding(void* colorSpace, const char* encoding);
void ocio_color_space_destroy(void* handle);

// --- Config: ColorSpace by object ---
void* ocio_config_get_color_space(void* config, const char* name);
int ocio_config_get_index_for_color_space(void* config, const char* name);
void ocio_config_add_color_space(void* config, void* colorSpace);
void ocio_config_remove_color_space(void* config, const char* name);
bool ocio_config_is_color_space_used(void* config, const char* name);

// --- Config: Looks by object ---
void* ocio_config_get_look(void* config, const char* name);
void ocio_config_add_look(void* config, void* look);

// --- Config: processor from transform ---
void* ocio_config_get_processor_transform(void* config, void* transform, int direction);

// --- Look ---
void* ocio_look_create(void);
void* ocio_look_create_editable_copy(void* look);
const char* ocio_look_get_name(void* look);
void ocio_look_set_name(void* look, const char* name);
const char* ocio_look_get_process_space(void* look);
void ocio_look_set_process_space(void* look, const char* processSpace);
void* ocio_look_get_transform(void* look);
void ocio_look_set_transform(void* look, const void* transform);
int ocio_look_get_direction(void* look);
void ocio_look_set_direction(void* look, int direction);
void ocio_look_destroy(void* handle);

// --- ViewTransform ---
void* ocio_view_transform_create(int referenceSpace);
const char* ocio_view_transform_get_src(void* viewTransform);
void ocio_view_transform_set_src(void* viewTransform, const char* src);
const char* ocio_view_transform_get_display(void* viewTransform);
void ocio_view_transform_set_display(void* viewTransform, const char* display);
const char* ocio_view_transform_get_view(void* viewTransform);
void ocio_view_transform_set_view(void* viewTransform, const char* view);
bool ocio_view_transform_get_looks_bypass(void* viewTransform);
void ocio_view_transform_set_looks_bypass(void* viewTransform, bool bypass);
const char* ocio_view_transform_get_rule(void* viewTransform);
void ocio_view_transform_set_rule(void* viewTransform, const char* rule);
void* ocio_view_transform_get_transform(void* viewTransform);
void ocio_view_transform_set_transform(void* viewTransform, const void* transform);
int ocio_view_transform_get_direction(void* viewTransform);
void ocio_view_transform_set_direction(void* viewTransform, int direction);
void ocio_view_transform_destroy(void* handle);

// --- NamedTransform ---
void* ocio_named_transform_create(void);
void* ocio_named_transform_create_editable_copy(void* namedTransform);
const char* ocio_named_transform_get_name(void* namedTransform);
void ocio_named_transform_set_name(void* namedTransform, const char* name);
const char* ocio_named_transform_get_family(void* namedTransform);
void ocio_named_transform_set_family(void* namedTransform, const char* family);
const char* ocio_named_transform_get_description(void* namedTransform);
void ocio_named_transform_set_description(void* namedTransform, const char* description);
const char* ocio_named_transform_get_encoding(void* namedTransform);
void ocio_named_transform_set_encoding(void* namedTransform, const char* encoding);
void* ocio_named_transform_get_transform(void* namedTransform, int direction);
void ocio_named_transform_set_transform(void* namedTransform, const void* transform, int direction);
void ocio_named_transform_destroy(void* handle);

// --- DynamicProperty ---
void* ocio_processor_get_dynamic_property(void* processor, int propertyType);
int ocio_dynamic_property_get_type(void* prop);
double ocio_dynamic_property_double_get_value(void* prop);
void ocio_dynamic_property_double_set_value(void* prop, double value);
void ocio_dynamic_property_destroy(void* handle);

// --- ExposureContrastTransform ---
void* ocio_exposure_contrast_transform_create(void);
double ocio_exposure_contrast_transform_get_exposure(void* transform);
void ocio_exposure_contrast_transform_set_exposure(void* transform, double exposure);
double ocio_exposure_contrast_transform_get_contrast(void* transform);
void ocio_exposure_contrast_transform_set_contrast(void* transform, double contrast);
double ocio_exposure_contrast_transform_get_gamma(void* transform);
void ocio_exposure_contrast_transform_set_gamma(void* transform, double gamma);
double ocio_exposure_contrast_transform_get_pivot(void* transform);
void ocio_exposure_contrast_transform_set_pivot(void* transform, double pivot);
int ocio_exposure_contrast_transform_get_style(void* transform);
void ocio_exposure_contrast_transform_set_style(void* transform, int style);
bool ocio_exposure_contrast_transform_is_exposure_dynamic(void* transform);
void ocio_exposure_contrast_transform_make_exposure_dynamic(void* transform);
bool ocio_exposure_contrast_transform_is_contrast_dynamic(void* transform);
void ocio_exposure_contrast_transform_make_contrast_dynamic(void* transform);
bool ocio_exposure_contrast_transform_is_gamma_dynamic(void* transform);
void ocio_exposure_contrast_transform_make_gamma_dynamic(void* transform);
int ocio_exposure_contrast_transform_get_direction(void* transform);
void ocio_exposure_contrast_transform_set_direction(void* transform, int direction);
void ocio_exposure_contrast_transform_destroy(void* handle);

// --- ColorSpaceTransform ---
void* ocio_color_space_transform_create(void);
const char* ocio_color_space_transform_get_src(void* transform);
void ocio_color_space_transform_set_src(void* transform, const char* src);
const char* ocio_color_space_transform_get_dst(void* transform);
void ocio_color_space_transform_set_dst(void* transform, const char* dst);
bool ocio_color_space_transform_get_data_bypass(void* transform);
void ocio_color_space_transform_set_data_bypass(void* transform, bool bypass);
int ocio_color_space_transform_get_direction(void* transform);
void ocio_color_space_transform_set_direction(void* transform, int direction);
void ocio_color_space_transform_destroy(void* handle);

// --- LookTransform ---
void* ocio_look_transform_create(void);
const char* ocio_look_transform_get_src(void* transform);
void ocio_look_transform_set_src(void* transform, const char* src);
const char* ocio_look_transform_get_dst(void* transform);
void ocio_look_transform_set_dst(void* transform, const char* dst);
const char* ocio_look_transform_get_looks(void* transform);
void ocio_look_transform_set_looks(void* transform, const char* looks);
int ocio_look_transform_get_direction(void* transform);
void ocio_look_transform_set_direction(void* transform, int direction);
void ocio_look_transform_destroy(void* handle);

// --- DisplayViewTransform ---
void* ocio_display_view_transform_create(void);
const char* ocio_display_view_transform_get_src(void* transform);
void ocio_display_view_transform_set_src(void* transform, const char* src);
const char* ocio_display_view_transform_get_display(void* transform);
void ocio_display_view_transform_set_display(void* transform, const char* display);
const char* ocio_display_view_transform_get_view(void* transform);
void ocio_display_view_transform_set_view(void* transform, const char* view);
bool ocio_display_view_transform_get_looks_bypass(void* transform);
void ocio_display_view_transform_set_looks_bypass(void* transform, bool bypass);
int ocio_display_view_transform_get_direction(void* transform);
void ocio_display_view_transform_set_direction(void* transform, int direction);
void ocio_display_view_transform_destroy(void* handle);

// --- GradingPrimaryTransform ---
void* ocio_grading_primary_transform_create(int style);
int ocio_grading_primary_transform_get_style(void* transform);
void ocio_grading_primary_transform_set_style(void* transform, int style);
void ocio_grading_primary_transform_get_value(void* transform, double* values);
void ocio_grading_primary_transform_set_value(void* transform, const double* values);
bool ocio_grading_primary_transform_is_dynamic(void* transform);
void ocio_grading_primary_transform_make_dynamic(void* transform);
void ocio_grading_primary_transform_make_non_dynamic(void* transform);
int ocio_grading_primary_transform_get_direction(void* transform);
void ocio_grading_primary_transform_set_direction(void* transform, int direction);
void ocio_grading_primary_transform_destroy(void* handle);

// --- GradingToneTransform ---
void* ocio_grading_tone_transform_create(int style);
int ocio_grading_tone_transform_get_style(void* transform);
void ocio_grading_tone_transform_set_style(void* transform, int style);
void ocio_grading_tone_transform_get_value(void* transform, double* values);
void ocio_grading_tone_transform_set_value(void* transform, const double* values);
bool ocio_grading_tone_transform_is_dynamic(void* transform);
void ocio_grading_tone_transform_make_dynamic(void* transform);
void ocio_grading_tone_transform_make_non_dynamic(void* transform);
int ocio_grading_tone_transform_get_direction(void* transform);
void ocio_grading_tone_transform_set_direction(void* transform, int direction);
void ocio_grading_tone_transform_destroy(void* handle);

// --- GradingRGBCurveTransform ---
void* ocio_grading_rgb_curve_transform_create(int style);
int ocio_grading_rgb_curve_transform_get_style(void* transform);
void ocio_grading_rgb_curve_transform_set_style(void* transform, int style);
int ocio_grading_rgb_curve_transform_get_num_control_points(void* transform, int curveType);
void ocio_grading_rgb_curve_transform_get_control_point(void* transform, int curveType, int index, float* x, float* y);
void ocio_grading_rgb_curve_transform_set_num_control_points(void* transform, int curveType, int num);
void ocio_grading_rgb_curve_transform_set_control_point(void* transform, int curveType, int index, float x, float y);
float ocio_grading_rgb_curve_transform_get_slope(void* transform, int curveType, int index);
void ocio_grading_rgb_curve_transform_set_slope(void* transform, int curveType, int index, float slope);
bool ocio_grading_rgb_curve_transform_slopes_are_default(void* transform, int curveType);
bool ocio_grading_rgb_curve_transform_get_bypass_lin_to_log(void* transform);
void ocio_grading_rgb_curve_transform_set_bypass_lin_to_log(void* transform, bool bypass);
bool ocio_grading_rgb_curve_transform_is_dynamic(void* transform);
void ocio_grading_rgb_curve_transform_make_dynamic(void* transform);
void ocio_grading_rgb_curve_transform_make_non_dynamic(void* transform);
int ocio_grading_rgb_curve_transform_get_direction(void* transform);
void ocio_grading_rgb_curve_transform_set_direction(void* transform, int direction);
void ocio_grading_rgb_curve_transform_destroy(void* handle);

// --- GradingHueCurveTransform ---
void* ocio_grading_hue_curve_transform_create(int style);
int ocio_grading_hue_curve_transform_get_style(void* transform);
void ocio_grading_hue_curve_transform_set_style(void* transform, int style);
int ocio_grading_hue_curve_transform_get_num_control_points(void* transform, int curveType);
void ocio_grading_hue_curve_transform_get_control_point(void* transform, int curveType, int index, float* x, float* y);
void ocio_grading_hue_curve_transform_set_num_control_points(void* transform, int curveType, int num);
void ocio_grading_hue_curve_transform_set_control_point(void* transform, int curveType, int index, float x, float y);
float ocio_grading_hue_curve_transform_get_slope(void* transform, int curveType, int index);
void ocio_grading_hue_curve_transform_set_slope(void* transform, int curveType, int index, float slope);
bool ocio_grading_hue_curve_transform_slopes_are_default(void* transform, int curveType);
bool ocio_grading_hue_curve_transform_get_bypass_lin_to_log(void* transform);
void ocio_grading_hue_curve_transform_set_bypass_lin_to_log(void* transform, bool bypass);
bool ocio_grading_hue_curve_transform_is_dynamic(void* transform);
void ocio_grading_hue_curve_transform_make_dynamic(void* transform);
void ocio_grading_hue_curve_transform_make_non_dynamic(void* transform);
int ocio_grading_hue_curve_transform_get_direction(void* transform);
void ocio_grading_hue_curve_transform_set_direction(void* transform, int direction);
void ocio_grading_hue_curve_transform_destroy(void* handle);

// --- AllocationTransform ---
void* ocio_allocation_transform_create(void);
int ocio_allocation_transform_get_allocation(void* transform);
void ocio_allocation_transform_set_allocation(void* transform, int allocation);
int ocio_allocation_transform_get_num_vars(void* transform);
void ocio_allocation_transform_get_vars(void* transform, float* vars);
void ocio_allocation_transform_set_vars(void* transform, int numvars, const float* vars);
int ocio_allocation_transform_get_direction(void* transform);
void ocio_allocation_transform_set_direction(void* transform, int direction);
void ocio_allocation_transform_destroy(void* handle);

// --- LogAffineTransform ---
void* ocio_log_affine_transform_create(void);
double ocio_log_affine_transform_get_base(void* transform);
void ocio_log_affine_transform_set_base(void* transform, double base);
void ocio_log_affine_transform_get_log_side_slope_value(void* transform, double* values);
void ocio_log_affine_transform_set_log_side_slope_value(void* transform, const double* values);
void ocio_log_affine_transform_get_log_side_offset_value(void* transform, double* values);
void ocio_log_affine_transform_set_log_side_offset_value(void* transform, const double* values);
void ocio_log_affine_transform_get_lin_side_slope_value(void* transform, double* values);
void ocio_log_affine_transform_set_lin_side_slope_value(void* transform, const double* values);
void ocio_log_affine_transform_get_lin_side_offset_value(void* transform, double* values);
void ocio_log_affine_transform_set_lin_side_offset_value(void* transform, const double* values);
void ocio_log_affine_transform_destroy(void* handle);

// --- LogCameraTransform ---
void* ocio_log_camera_transform_create(const double* linSideBreakValues);
double ocio_log_camera_transform_get_base(void* transform);
void ocio_log_camera_transform_set_base(void* transform, double base);
void ocio_log_camera_transform_get_log_side_slope_value(void* transform, double* values);
void ocio_log_camera_transform_set_log_side_slope_value(void* transform, const double* values);
void ocio_log_camera_transform_get_log_side_offset_value(void* transform, double* values);
void ocio_log_camera_transform_set_log_side_offset_value(void* transform, const double* values);
void ocio_log_camera_transform_get_lin_side_slope_value(void* transform, double* values);
void ocio_log_camera_transform_set_lin_side_slope_value(void* transform, const double* values);
void ocio_log_camera_transform_get_lin_side_offset_value(void* transform, double* values);
void ocio_log_camera_transform_set_lin_side_offset_value(void* transform, const double* values);
void ocio_log_camera_transform_get_lin_side_break_value(void* transform, double* values);
void ocio_log_camera_transform_set_lin_side_break_value(void* transform, const double* values);
bool ocio_log_camera_transform_get_linear_slope_value(void* transform, double* values);
void ocio_log_camera_transform_set_linear_slope_value(void* transform, const double* values);
void ocio_log_camera_transform_unset_linear_slope_value(void* transform);
void ocio_log_camera_transform_destroy(void* handle);

// --- Config: active display/view setters ---
void ocio_config_set_active_displays(void* config, const char* displays);
void ocio_config_set_active_views(void* config, const char* views);

// --- Config: display/view transform name queries ---
const char* ocio_config_get_display_view_transform_name(void* config, const char* display, const char* view);
const char* ocio_config_get_display_view_color_space_name(void* config, const char* display, const char* view);

// --- Config: clear collections ---
void ocio_config_clear_color_spaces(void* config);
void ocio_config_clear_looks(void* config);

// --- Config: default luma setter ---
void ocio_config_set_default_luma_coefs(void* config, const double* rgb);

// --- Config: display/view management ---
void ocio_config_add_display(void* config, const char* display, const char* view, const char* transformName, const char* rule);
void ocio_config_add_shared_view(void* config, const char* display, const char* view, const char* transformName, const char* rule);
void ocio_config_remove_display(void* config, const char* display);
void ocio_config_remove_view(void* config, const char* display, const char* view);

// --- Config: named transforms ---
int ocio_config_get_num_named_transforms(void* config);
const char* ocio_config_get_named_transform_name_by_index(void* config, int index);
void* ocio_config_get_named_transform(void* config, const char* name);
void ocio_config_add_named_transform(void* config, void* namedTransform);
void ocio_config_remove_named_transform(void* config, const char* name);

// --- Config: view transforms ---
int ocio_config_get_num_view_transforms(void* config);
const char* ocio_config_get_view_transform_name_by_index(void* config, int index);
void* ocio_config_get_view_transform(void* config, const char* name);
void ocio_config_add_view_transform(void* config, void* viewTransform);
void ocio_config_remove_view_transform(void* config, const char* name);

// --- ColorSpace: aliases ---
int ocio_color_space_get_num_aliases(void* colorSpace);
const char* ocio_color_space_get_alias(void* colorSpace, int index);
void ocio_color_space_add_alias(void* colorSpace, const char* alias);
void ocio_color_space_remove_alias(void* colorSpace, const char* alias);
void ocio_color_space_clear_aliases(void* colorSpace);

// --- ColorSpace: inactive ---
bool ocio_color_space_is_inactive(void* colorSpace);
void ocio_color_space_set_inactive(void* colorSpace, bool inactive);

// --- Look: aliases ---
int ocio_look_get_num_aliases(void* look);
const char* ocio_look_get_alias(void* look, int index);
void ocio_look_add_alias(void* look, const char* alias);
void ocio_look_remove_alias(void* look, const char* alias);
void ocio_look_clear_aliases(void* look);

// --- Look: inactive ---
bool ocio_look_is_inactive(void* look);
void ocio_look_set_inactive(void* look, bool inactive);

// --- NamedTransform: aliases ---
int ocio_named_transform_get_num_aliases(void* namedTransform);
const char* ocio_named_transform_get_alias(void* namedTransform, int index);
void ocio_named_transform_add_alias(void* namedTransform, const char* alias);
void ocio_named_transform_remove_alias(void* namedTransform, const char* alias);
void ocio_named_transform_clear_aliases(void* namedTransform);

// --- NamedTransform: inactive & category ---
bool ocio_named_transform_is_inactive(void* namedTransform);
void ocio_named_transform_set_inactive(void* namedTransform, bool inactive);
const char* ocio_named_transform_get_category(void* namedTransform);
void ocio_named_transform_set_category(void* namedTransform, const char* category);

// --- ViewTransform: aliases ---
int ocio_view_transform_get_num_aliases(void* viewTransform);
const char* ocio_view_transform_get_alias(void* viewTransform, int index);
void ocio_view_transform_add_alias(void* viewTransform, const char* alias);
void ocio_view_transform_remove_alias(void* viewTransform, const char* alias);
void ocio_view_transform_clear_aliases(void* viewTransform);

// --- ViewTransform: inactive, category, description ---
bool ocio_view_transform_is_inactive(void* viewTransform);
void ocio_view_transform_set_inactive(void* viewTransform, bool inactive);
const char* ocio_view_transform_get_category(void* viewTransform);
void ocio_view_transform_set_category(void* viewTransform, const char* category);
const char* ocio_view_transform_get_description(void* viewTransform);
void ocio_view_transform_set_description(void* viewTransform, const char* description);

// --- ViewTransform: editable copy ---
void* ocio_view_transform_create_editable_copy(void* viewTransform);

// --- FileRules ---
void* ocio_file_rules_create(void);
void* ocio_file_rules_create_editable_copy(void* rules);
unsigned long long ocio_file_rules_get_num_entries(void* rules);
unsigned long long ocio_file_rules_get_index_for_rule(void* rules, const char* ruleName);
const char* ocio_file_rules_get_name(void* rules, unsigned long long ruleIndex);
const char* ocio_file_rules_get_pattern(void* rules, unsigned long long ruleIndex);
void ocio_file_rules_set_pattern(void* rules, unsigned long long ruleIndex, const char* pattern);
const char* ocio_file_rules_get_extension(void* rules, unsigned long long ruleIndex);
void ocio_file_rules_set_extension(void* rules, unsigned long long ruleIndex, const char* extension);
const char* ocio_file_rules_get_regex(void* rules, unsigned long long ruleIndex);
void ocio_file_rules_set_regex(void* rules, unsigned long long ruleIndex, const char* regex);
const char* ocio_file_rules_get_color_space(void* rules, unsigned long long ruleIndex);
void ocio_file_rules_set_color_space(void* rules, unsigned long long ruleIndex, const char* colorSpace);
unsigned long long ocio_file_rules_get_num_custom_keys(void* rules, unsigned long long ruleIndex);
const char* ocio_file_rules_get_custom_key_name(void* rules, unsigned long long ruleIndex, unsigned long long key);
const char* ocio_file_rules_get_custom_key_value(void* rules, unsigned long long ruleIndex, unsigned long long key);
void ocio_file_rules_set_custom_key(void* rules, unsigned long long ruleIndex, const char* key, const char* value);
void ocio_file_rules_insert_rule(void* rules, unsigned long long ruleIndex, const char* name, const char* colorSpace, const char* pattern, const char* extension);
void ocio_file_rules_insert_rule_regex(void* rules, unsigned long long ruleIndex, const char* name, const char* colorSpace, const char* regex);
void ocio_file_rules_insert_path_search_rule(void* rules, unsigned long long ruleIndex);
void ocio_file_rules_set_default_rule_color_space(void* rules, const char* colorSpace);
void ocio_file_rules_remove_rule(void* rules, unsigned long long ruleIndex);
void ocio_file_rules_increase_rule_priority(void* rules, unsigned long long ruleIndex);
void ocio_file_rules_decrease_rule_priority(void* rules, unsigned long long ruleIndex);
bool ocio_file_rules_is_default(void* rules);
void ocio_file_rules_destroy(void* handle);

// --- Config: FileRules ---
void* ocio_config_get_file_rules(void* config);

// --- Lut1DTransform: data ---
unsigned long long ocio_lut1d_transform_get_length(void* transform);
void ocio_lut1d_transform_get_values(void* transform, double* data);
void ocio_lut1d_transform_set_length(void* transform, unsigned long long len);
void ocio_lut1d_transform_set_values(void* transform, const double* data);

// --- Lut3DTransform: data ---
unsigned long long ocio_lut3d_transform_get_grid_size(void* transform);
void ocio_lut3d_transform_get_values(void* transform, double* data);
void ocio_lut3d_transform_set_grid_size(void* transform, unsigned long long size);
void ocio_lut3d_transform_set_values(void* transform, const double* data);

// --- GroupTransform: remove/clear ---
void ocio_group_transform_remove_transform(void* transform, unsigned long long index);
void ocio_group_transform_clear_transforms(void* transform);

// --- Config: clear all, version setters, interpolation, working dir ---
void ocio_config_clear_all(void* config);
void ocio_config_set_major_version(void* config, unsigned int version);
void ocio_config_set_minor_version(void* config, unsigned int version);
int ocio_config_get_default_interpolation(void* config);
void ocio_config_set_default_interpolation(void* config, int interpolation);
const char* ocio_config_get_working_dir(void* config);
void ocio_config_set_working_dir(void* config, const char* dirName);

// --- ColorSpace: visibility + set_reference_space_type ---
int ocio_color_space_get_visibility(void* colorSpace);
void ocio_color_space_set_visibility(void* colorSpace, int visibility);
void ocio_color_space_set_reference_space_type(void* colorSpace, int referenceSpace);

// --- LookTransform: skip color space conversion ---
bool ocio_look_transform_get_skip_color_space_conversion(void* transform);
void ocio_look_transform_set_skip_color_space_conversion(void* transform, bool skip);

// --- Global: processor cache flags ---
int ocio_get_processor_cache_flags(void);
void ocio_set_processor_cache_flags(int flags);

// --- CPUProcessor: batch pixel processing ---
void ocio_cpu_processor_apply_rgba_pixels(void* cpu_processor, float* rgba, long numPixels, long stride);
void ocio_cpu_processor_apply_rgb_pixels(void* cpu_processor, float* rgb, long numPixels, long stride);

// --- Processor: batch pixel processing ---
void ocio_processor_apply_rgba_pixels(void* processor, float* rgba, long numPixels, long stride);

// --- Baker: format metadata ---
void* ocio_baker_get_format_metadata(void* baker);

// --- ColorSpaceSet ---
void* ocio_color_space_set_create(void);
void* ocio_color_space_set_create_editable_copy(void* set);
int ocio_color_space_set_get_num_color_spaces(void* set);
const char* ocio_color_space_set_get_color_space_name_by_index(void* set, int index);
void* ocio_color_space_set_get_color_space_by_index(void* set, int index);
void* ocio_color_space_set_get_color_space(void* set, const char* name);
int ocio_color_space_set_get_color_space_index(void* set, const char* name);
bool ocio_color_space_set_has_color_space(void* set, const char* name);
void ocio_color_space_set_destroy(void* handle);

// --- Config: ColorSpaceSet ---
void* ocio_config_get_color_space_set(void* config, const char* search);

// --- Transform: format metadata ---
void* ocio_transform_get_format_metadata(void* transform);

// --- FormatMetadata ---
const char* ocio_format_metadata_get_element_name(void* metadata);
void ocio_format_metadata_set_element_name(void* metadata, const char* name);
const char* ocio_format_metadata_get_element_value(void* metadata);
void ocio_format_metadata_set_element_value(void* metadata, const char* value);
int ocio_format_metadata_get_num_attributes(void* metadata);
const char* ocio_format_metadata_get_attribute_name(void* metadata, int i);
const char* ocio_format_metadata_get_attribute_value_by_index(void* metadata, int i);
const char* ocio_format_metadata_get_attribute_value(void* metadata, const char* name);
void ocio_format_metadata_add_attribute(void* metadata, const char* name, const char* value);
int ocio_format_metadata_get_num_children_elements(void* metadata);
void* ocio_format_metadata_get_child_element(void* metadata, int i);
void ocio_format_metadata_add_child_element(void* metadata, const char* name, const char* value);
void ocio_format_metadata_clear(void* metadata);
const char* ocio_format_metadata_get_name(void* metadata);
void ocio_format_metadata_set_name(void* metadata, const char* name);
const char* ocio_format_metadata_get_id(void* metadata);
void ocio_format_metadata_set_id(void* metadata, const char* id);
void ocio_format_metadata_destroy(void* handle);

// --- BuiltinTransform: description ---
const char* ocio_builtin_transform_get_description(void* transform);

// --- DisplayViewTransform: data bypass ---
bool ocio_display_view_transform_get_data_bypass(void* transform);
void ocio_display_view_transform_set_data_bypass(void* transform, bool bypass);

// --- ExposureContrastTransform: log & dynamic ---
double ocio_exposure_contrast_transform_get_log_exposure_step(void* transform);
void ocio_exposure_contrast_transform_set_log_exposure_step(void* transform, double step);
double ocio_exposure_contrast_transform_get_log_mid_gray(void* transform);
void ocio_exposure_contrast_transform_set_log_mid_gray(void* transform, double mid_gray);
void ocio_exposure_contrast_transform_make_exposure_non_dynamic(void* transform);
void ocio_exposure_contrast_transform_make_contrast_non_dynamic(void* transform);
void ocio_exposure_contrast_transform_make_gamma_non_dynamic(void* transform);

// --- LogAffineTransform: direction ---
int ocio_log_affine_transform_get_direction(void* transform);
void ocio_log_affine_transform_set_direction(void* transform, int direction);

// --- LogCameraTransform: direction ---
int ocio_log_camera_transform_get_direction(void* transform);
void ocio_log_camera_transform_set_direction(void* transform, int direction);

// --- Lut1DTransform: half domain, raw halfs, hue adjust ---
bool ocio_lut1d_transform_get_input_half_domain(void* transform);
void ocio_lut1d_transform_set_input_half_domain(void* transform, bool half_domain);
bool ocio_lut1d_transform_get_output_raw_halfs(void* transform);
void ocio_lut1d_transform_set_output_raw_halfs(void* transform, bool raw_halfs);
int ocio_lut1d_transform_get_hue_adjust(void* transform);
void ocio_lut1d_transform_set_hue_adjust(void* transform, int hue_adjust);

// --- MatrixTransform: bit depth & static helpers ---
int ocio_matrix_transform_get_file_input_bit_depth(void* transform);
void ocio_matrix_transform_set_file_input_bit_depth(void* transform, int bit_depth);
int ocio_matrix_transform_get_file_output_bit_depth(void* transform);
void ocio_matrix_transform_set_file_output_bit_depth(void* transform, int bit_depth);
void* ocio_matrix_transform_create_fit(const char* inputColorSpace, const char* outputColorSpace);
void* ocio_matrix_transform_create_identity(void);
void* ocio_matrix_transform_create_sat(double sat, const double* luma);
void* ocio_matrix_transform_create_scale(const double* scale);
void* ocio_matrix_transform_create_view(int* channels, const char* gamma);

// --- RangeTransform: has/unset value & bit depth ---
bool ocio_range_transform_has_min_in_value(void* transform);
void ocio_range_transform_unset_min_in_value(void* transform);
bool ocio_range_transform_has_max_in_value(void* transform);
void ocio_range_transform_unset_max_in_value(void* transform);
bool ocio_range_transform_has_min_out_value(void* transform);
void ocio_range_transform_unset_min_out_value(void* transform);
bool ocio_range_transform_has_max_out_value(void* transform);
void ocio_range_transform_unset_max_out_value(void* transform);
int ocio_range_transform_get_file_input_bit_depth(void* transform);
void ocio_range_transform_set_file_input_bit_depth(void* transform, int bit_depth);
int ocio_range_transform_get_file_output_bit_depth(void* transform);
void ocio_range_transform_set_file_output_bit_depth(void* transform, int bit_depth);

// --- CDLTransform: SOP & description ---
void ocio_cdl_transform_get_sop(void* transform, double* vec9);
void ocio_cdl_transform_set_sop(void* transform, const double* vec9);
const char* ocio_cdl_transform_get_first_sop_description(void* transform);
void ocio_cdl_transform_set_first_sop_description(void* transform, const char* description);

}
