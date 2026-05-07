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

// --- Global utility functions ---
const char* ocio_get_version(void);
int ocio_get_version_hex(void);
int ocio_get_logging_level(void);
void ocio_set_logging_level(int level);

// --- Global config ---
void* ocio_get_current_config(void);
void ocio_set_current_config(void* config);
void ocio_clear_all_caches(void);

// --- BuiltinConfigRegistry ---
void* ocio_builtin_config_registry_get(void);
size_t ocio_builtin_config_registry_get_num_builtin_configs(void* handle);
void* ocio_builtin_config_registry_get_builtin_config_name(void* handle, size_t configIndex);
void* ocio_builtin_config_registry_get_builtin_config_ui_name(void* handle, size_t configIndex);
void* ocio_builtin_config_registry_get_builtin_config(void* handle, size_t configIndex);
void* ocio_builtin_config_registry_get_builtin_config_by_name(void* handle, const char* configName);
bool ocio_builtin_config_registry_is_builtin_config_recommended(void* handle, size_t configIndex);

// --- Config ---
void* ocio_config_create_raw(void);
void* ocio_config_create_from_file(const char* path);
void ocio_config_destroy(void* handle);

int ocio_config_get_major_version(void* handle);
void ocio_config_set_major_version(void* handle, unsigned int major);
int ocio_config_get_minor_version(void* handle);
void ocio_config_set_minor_version(void* handle, unsigned int minor);
void ocio_config_set_version(void* handle, unsigned int major, unsigned int minor);
void ocio_config_upgrade_to_latest_version(void* handle);
void ocio_config_validate(void* handle);
void* ocio_config_get_name(void* handle);
void ocio_config_set_name(void* handle, const char* name);
char ocio_config_get_family_separator(void* handle);
char ocio_config_get_default_family_separator(void* handle);
void ocio_config_set_family_separator(void* handle, char separator);
void* ocio_config_get_description(void* handle);
void ocio_config_set_description(void* handle, const char* description);
void ocio_config_serialize(void* handle, void* os);
void* ocio_config_get_cache_id(void* handle);
void* ocio_config_get_cache_id_n(void* handle, void* context);
void* ocio_config_get_current_context(void* handle);
void ocio_config_add_environment_var(void* handle, const char* name, const char* defaultValue);
int ocio_config_get_num_environment_vars(void* handle);
void* ocio_config_get_environment_var_name_by_index(void* handle, int index);
void* ocio_config_get_environment_var_default(void* handle, const char* name);
void ocio_config_clear_environment_vars(void* handle);
void ocio_config_set_environment_mode(void* handle, int mode);
int ocio_config_get_environment_mode(void* handle);
void ocio_config_load_environment(void* handle);
void* ocio_config_get_search_path(void* handle);
void ocio_config_set_search_path(void* handle, const char* path);
int ocio_config_get_num_search_paths(void* handle);
void* ocio_config_get_search_path_by_index(void* handle, int index);
void ocio_config_clear_search_paths(void* handle);
void ocio_config_add_search_path(void* handle, const char* path);
void* ocio_config_get_working_dir(void* handle);
void ocio_config_set_working_dir(void* handle, const char* dirname);
void* ocio_config_get_color_spaces(void* handle, const char* category);
int ocio_config_get_num_color_spaces(void* handle, int searchReferenceType, int visibility);
void* ocio_config_get_color_space_name_by_index(void* handle, int searchReferenceType, int visibility, int index);
int ocio_config_get_num_color_spaces_v1(void* handle);
void* ocio_config_get_color_space_name_by_index_v1(void* handle, int index);
int ocio_config_get_index_for_color_space(void* handle, const char* name);
void* ocio_config_get_color_space(void* handle, const char* name);
void* ocio_config_get_canonical_name(void* handle, const char* name);
void ocio_config_add_color_space(void* handle, void* cs);
void ocio_config_remove_color_space(void* handle, const char* name);
bool ocio_config_is_color_space_used(void* handle, const char* name);
void ocio_config_clear_color_spaces(void* handle);
void ocio_config_set_inactive_color_spaces(void* handle, const char* inactiveColorSpaces);
void* ocio_config_get_inactive_color_spaces(void* handle);
bool ocio_config_is_inactive_color_space(void* handle, const char* colorspace);
bool ocio_config_is_color_space_linear(void* handle, const char* colorSpace, int referenceSpaceType);
void* ocio_config_identify_builtin_color_space(void* handle, void* srcConfig, void* builtinConfig, const char* builtinColorSpaceName);
void ocio_config_identify_interchange_space(void* handle, void* srcInterchangeName, void* builtinInterchangeName, void* srcConfig, const char* srcColorSpaceName, void* builtinConfig, const char* builtinColorSpaceName);
void ocio_config_set_role(void* handle, const char* role, const char* colorSpaceName);
int ocio_config_get_num_roles(void* handle);
bool ocio_config_has_role(void* handle, const char* role);
void* ocio_config_get_role_name(void* handle, int index);
void* ocio_config_get_role_color_space_by_index(void* handle, int index);
void* ocio_config_get_role_color_space_by_name(void* handle, const char* roleName);
bool ocio_config_is_view_shared(void* handle, const char* dispName, const char* viewName);
void ocio_config_add_shared_view(void* handle, const char* view, const char* viewTransformName, const char* colorSpaceName, const char* looks, const char* ruleName, const char* description);
void ocio_config_remove_shared_view(void* handle, const char* view);
void ocio_config_clear_shared_views(void* handle);
void* ocio_config_get_default_display(void* handle);
int ocio_config_get_num_displays(void* handle);
void* ocio_config_get_display(void* handle, int index);
void* ocio_config_get_default_view(void* handle, const char* display);
void* ocio_config_get_default_view_v1(void* handle, const char* display, const char* colorspaceName);
int ocio_config_get_num_views(void* handle, const char* display);
void* ocio_config_get_view(void* handle, const char* display, int index);
int ocio_config_get_num_views_v1(void* handle, const char* display, const char* colorspaceName);
void* ocio_config_get_view_v1(void* handle, const char* display, const char* colorspaceName, int index);
bool ocio_config_are_views_equal(void* handle, void* first, void* second, const char* dispName, const char* viewName);
void* ocio_config_get_display_view_transform_name(void* handle, const char* display, const char* view);
void* ocio_config_get_display_view_color_space_name(void* handle, const char* display, const char* view);
void* ocio_config_get_display_view_looks(void* handle, const char* display, const char* view);
void* ocio_config_get_display_view_rule(void* handle, const char* display, const char* view);
void* ocio_config_get_display_view_description(void* handle, const char* display, const char* view);
bool ocio_config_has_view(void* handle, const char* dispName, const char* viewName);
void ocio_config_add_display_view_v1(void* handle, const char* display, const char* view, const char* colorSpaceName, const char* looks);
void ocio_config_add_display_view_v2(void* handle, const char* display, const char* view, const char* viewTransformName, const char* colorSpaceName, const char* looks, const char* ruleName, const char* description);
void ocio_config_add_display_shared_view(void* handle, const char* display, const char* sharedView);
void ocio_config_remove_display_view(void* handle, const char* display, const char* view);
void ocio_config_clear_displays(void* handle);
bool ocio_config_has_virtual_view(void* handle, const char* viewName);
bool ocio_config_is_virtual_view_shared(void* handle, const char* viewName);
void ocio_config_add_virtual_display_view(void* handle, const char* view, const char* viewTransformName, const char* colorSpaceName, const char* looks, const char* ruleName, const char* description);
void ocio_config_add_virtual_display_shared_view(void* handle, const char* sharedView);
int ocio_config_get_virtual_display_num_views(void* handle, int type);
void* ocio_config_get_virtual_display_view(void* handle, int type, int index);
bool ocio_config_are_virtual_views_equal(void* handle, void* first, void* second, const char* viewName);
void* ocio_config_get_virtual_display_view_transform_name(void* handle, const char* view);
void* ocio_config_get_virtual_display_view_color_space_name(void* handle, const char* view);
void* ocio_config_get_virtual_display_view_looks(void* handle, const char* view);
void* ocio_config_get_virtual_display_view_rule(void* handle, const char* view);
void* ocio_config_get_virtual_display_view_description(void* handle, const char* view);
void ocio_config_remove_virtual_display_view(void* handle, const char* view);
void ocio_config_clear_virtual_display(void* handle);
int ocio_config_instantiate_display_from_monitor_name(void* handle, const char* monitorName);
int ocio_config_instantiate_display_from_icc_profile(void* handle, const char* ICCProfileFilepath);
void ocio_config_set_active_displays(void* handle, const char* displays);
void* ocio_config_get_active_displays(void* handle);
int ocio_config_get_num_active_displays(void* handle);
void* ocio_config_get_active_display(void* handle, int index);
void ocio_config_add_active_display(void* handle, const char* display);
void ocio_config_remove_active_display(void* handle, const char* display);
void ocio_config_clear_active_displays(void* handle);
void ocio_config_set_active_views(void* handle, const char* views);
void* ocio_config_get_active_views(void* handle);
int ocio_config_get_num_active_views(void* handle);
void* ocio_config_get_active_view(void* handle, int index);
void ocio_config_add_active_view(void* handle, const char* view);
void ocio_config_remove_active_view(void* handle, const char* view);
void ocio_config_clear_active_views(void* handle);
int ocio_config_get_num_displays_all(void* handle);
void* ocio_config_get_display_all(void* handle, int index);
int ocio_config_get_display_all_by_name(void* handle, void* arg0);
bool ocio_config_is_display_temporary(void* handle, int index);
void ocio_config_set_display_temporary(void* handle, int index, bool isTemporary);
int ocio_config_get_num_views_v2(void* handle, int type, const char* display);
void* ocio_config_get_view_v2(void* handle, int type, const char* display, int index);
void* ocio_config_get_viewing_rules(void* handle);
void ocio_config_set_viewing_rules(void* handle, void* viewingRules);
void ocio_config_get_default_luma_coefs(void* handle, void* rgb);
void ocio_config_set_default_luma_coefs(void* handle, const double* rgb);
void* ocio_config_get_look(void* handle, const char* name);
int ocio_config_get_num_looks(void* handle);
void* ocio_config_get_look_name_by_index(void* handle, int index);
void ocio_config_add_look(void* handle, void* look);
void ocio_config_clear_looks(void* handle);
int ocio_config_get_num_view_transforms(void* handle);
void* ocio_config_get_view_transform(void* handle, const char* name);
void* ocio_config_get_view_transform_name_by_index(void* handle, int i);
void ocio_config_add_view_transform(void* handle, void* viewTransform);
void* ocio_config_get_default_scene_to_display_view_transform(void* handle);
void* ocio_config_get_default_view_transform_name(void* handle);
void ocio_config_set_default_view_transform_name(void* handle, const char* defaultName);
void ocio_config_clear_view_transforms(void* handle);
int ocio_config_get_num_named_transforms(void* handle, int visibility);
void* ocio_config_get_named_transform_name_by_index(void* handle, int visibility, int index);
int ocio_config_get_num_named_transforms_v1(void* handle);
void* ocio_config_get_named_transform_name_by_index_v1(void* handle, int index);
int ocio_config_get_index_for_named_transform(void* handle, const char* name);
void* ocio_config_get_named_transform(void* handle, const char* name);
void ocio_config_add_named_transform(void* handle, void* namedTransform);
void ocio_config_remove_named_transform(void* handle, const char* name);
void ocio_config_clear_named_transforms(void* handle);
void* ocio_config_get_file_rules(void* handle);
void ocio_config_set_file_rules(void* handle, void* fileRules);
void* ocio_config_get_color_space_from_filepath(void* handle, const char* filePath);
void* ocio_config_get_color_space_from_filepath_by_ref_type(void* handle, const char* filePath, void* ruleIndex);
bool ocio_config_filepath_only_matches_default_rule(void* handle, const char* filePath);
void* ocio_config_parse_color_space_from_string(void* handle, const char* str);
bool ocio_config_is_strict_parsing_enabled(void* handle);
void ocio_config_set_strict_parsing_enabled(void* handle, bool enabled);
void* ocio_config_get_processor(void* handle, void* context, void* srcColorSpace, void* dstColorSpace);
void* ocio_config_get_processor_v1(void* handle, void* srcColorSpace, void* dstColorSpace);
void* ocio_config_get_processor_v2(void* handle, const char* srcColorSpaceName, const char* dstColorSpaceName);
void* ocio_config_get_processor_v3(void* handle, void* context, const char* srcColorSpaceName, const char* dstColorSpaceName);
void* ocio_config_get_processor_v4(void* handle, const char* srcColorSpaceName, const char* display, const char* view, int direction);
void* ocio_config_get_processor_v5(void* handle, void* context, const char* srcColorSpaceName, const char* display, const char* view, int direction);
void* ocio_config_get_processor_v6(void* handle, void* namedTransform, int direction);
void* ocio_config_get_processor_v7(void* handle, void* context, void* namedTransform, int direction);
void* ocio_config_get_processor_v8(void* handle, const char* namedTransformName, int direction);
void* ocio_config_get_processor_v9(void* handle, void* context, const char* namedTransformName, int direction);
void* ocio_config_get_processor_v10(void* handle, void* transform);
void* ocio_config_get_processor_v11(void* handle, void* transform, int direction);
void* ocio_config_get_processor_v12(void* handle, void* context, void* transform, int direction);
void* ocio_config_get_processor_to_builtin_color_space(void* handle, void* srcConfig, const char* srcColorSpaceName, const char* builtinColorSpaceName);
void* ocio_config_get_processor_from_builtin_color_space(void* handle, const char* builtinColorSpaceName, void* srcConfig, const char* srcColorSpaceName);
void* ocio_config_get_processor_from_configs(void* handle, void* srcConfig, const char* srcColorSpaceName, void* dstConfig, const char* dstColorSpaceName);
void* ocio_config_get_processor_from_configs_v1(void* handle, void* srcContext, void* srcConfig, const char* srcColorSpaceName, void* dstContext, void* dstConfig, const char* dstColorSpaceName);
void* ocio_config_get_processor_from_configs_v2(void* handle, void* srcConfig, const char* srcColorSpaceName, const char* srcInterchangeName, void* dstConfig, const char* dstColorSpaceName, const char* dstInterchangeName);
void* ocio_config_get_processor_from_configs_v3(void* handle, void* srcContext, void* srcConfig, const char* srcColorSpaceName, const char* srcInterchangeName, void* dstContext, void* dstConfig, const char* dstColorSpaceName, const char* dstInterchangeName);
void* ocio_config_get_processor_from_configs_v4(void* handle, void* srcConfig, const char* srcColorSpaceName, void* dstConfig, const char* dstDisplay, const char* dstView, int direction);
void* ocio_config_get_processor_from_configs_v5(void* handle, void* srcContext, void* srcConfig, const char* srcColorSpaceName, void* dstContext, void* dstConfig, const char* dstDisplay, const char* dstView, int direction);
void* ocio_config_get_processor_from_configs_v6(void* handle, void* srcConfig, const char* srcColorSpaceName, const char* srcInterchangeName, void* dstConfig, const char* dstDisplay, const char* dstView, const char* dstInterchangeName, int direction);
void* ocio_config_get_processor_from_configs_v7(void* handle, void* srcContext, void* srcConfig, const char* srcColorSpaceName, const char* srcInterchangeName, void* dstContext, void* dstConfig, const char* dstDisplay, const char* dstView, const char* dstInterchangeName, int direction);
int ocio_config_get_processor_cache_flags(void* handle);
void ocio_config_set_processor_cache_flags(void* handle, int flags);
void ocio_config_clear_processor_cache(void* handle);
void ocio_config_set_config_io_proxy(void* handle, void* ciop);
void* ocio_config_get_config_io_proxy(void* handle);
bool ocio_config_is_archivable(void* handle);
void ocio_config_archive(void* handle, void* ostream);

// --- FileRules ---
void* ocio_file_rules_create(void);
void ocio_file_rules_destroy(void* handle);

size_t ocio_file_rules_get_num_entries(void* handle);
size_t ocio_file_rules_get_index_for_rule(void* handle, const char* ruleName);
void* ocio_file_rules_get_name(void* handle, size_t ruleIndex);
void* ocio_file_rules_get_pattern(void* handle, size_t ruleIndex);
void ocio_file_rules_set_pattern(void* handle, size_t ruleIndex, const char* pattern);
void* ocio_file_rules_get_extension(void* handle, size_t ruleIndex);
void ocio_file_rules_set_extension(void* handle, size_t ruleIndex, const char* extension);
void* ocio_file_rules_get_regex(void* handle, size_t ruleIndex);
void ocio_file_rules_set_regex(void* handle, size_t ruleIndex, const char* regex);
void* ocio_file_rules_get_color_space(void* handle, size_t ruleIndex);
void ocio_file_rules_set_color_space(void* handle, size_t ruleIndex, const char* colorSpace);
size_t ocio_file_rules_get_num_custom_keys(void* handle, size_t ruleIndex);
void* ocio_file_rules_get_custom_key_name(void* handle, size_t ruleIndex, size_t key);
void* ocio_file_rules_get_custom_key_value(void* handle, size_t ruleIndex, size_t key);
void ocio_file_rules_set_custom_key(void* handle, size_t ruleIndex, const char* key, const char* value);
void ocio_file_rules_insert_rule(void* handle, size_t ruleIndex, const char* name, const char* colorSpace, const char* pattern, const char* extension);
void ocio_file_rules_insert_rule_v1(void* handle, size_t ruleIndex, const char* name, const char* colorSpace, const char* regex);
void ocio_file_rules_insert_path_search_rule(void* handle, size_t ruleIndex);
void ocio_file_rules_set_default_rule_color_space(void* handle, const char* colorSpace);
void ocio_file_rules_remove_rule(void* handle, size_t ruleIndex);
void ocio_file_rules_increase_rule_priority(void* handle, size_t ruleIndex);
void ocio_file_rules_decrease_rule_priority(void* handle, size_t ruleIndex);
bool ocio_file_rules_is_default(void* handle);

// --- ColorSpace ---
void* ocio_color_space_create(void);
void ocio_color_space_destroy(void* handle);

void* ocio_color_space_get_name(void* handle);
void ocio_color_space_set_name(void* handle, const char* name);
size_t ocio_color_space_get_num_aliases(void* handle);
void* ocio_color_space_get_alias(void* handle, size_t idx);
bool ocio_color_space_has_alias(void* handle, const char* alias);
void ocio_color_space_add_alias(void* handle, const char* alias);
void ocio_color_space_remove_alias(void* handle, const char* alias);
void ocio_color_space_clear_aliases(void* handle);
void* ocio_color_space_get_family(void* handle);
void ocio_color_space_set_family(void* handle, const char* family);
void* ocio_color_space_get_equality_group(void* handle);
void ocio_color_space_set_equality_group(void* handle, const char* equalityGroup);
void* ocio_color_space_get_description(void* handle);
void ocio_color_space_set_description(void* handle, const char* description);
void* ocio_color_space_get_interop_id(void* handle);
void ocio_color_space_set_interop_id(void* handle, const char* interopID);
void ocio_color_space_set_interchange_attribute(void* handle, const char* attrName, void* value);
int ocio_color_space_get_bit_depth(void* handle);
void ocio_color_space_set_bit_depth(void* handle, int bitDepth);
int ocio_color_space_get_reference_space_type(void* handle);
bool ocio_color_space_has_category(void* handle, const char* category);
void ocio_color_space_add_category(void* handle, const char* category);
void ocio_color_space_remove_category(void* handle, const char* category);
int ocio_color_space_get_num_categories(void* handle);
void* ocio_color_space_get_category(void* handle, int index);
void ocio_color_space_clear_categories(void* handle);
void* ocio_color_space_get_encoding(void* handle);
void ocio_color_space_set_encoding(void* handle, const char* encoding);
bool ocio_color_space_is_data(void* handle);
void ocio_color_space_set_is_data(void* handle, bool isData);
int ocio_color_space_get_allocation(void* handle);
void ocio_color_space_set_allocation(void* handle, int allocation);
int ocio_color_space_get_allocation_num_vars(void* handle);
void ocio_color_space_get_allocation_vars(void* handle, void* vars);
void ocio_color_space_set_allocation_vars(void* handle, int numvars, const float* vars);
void* ocio_color_space_get_transform(void* handle, int dir);
void ocio_color_space_set_transform(void* handle, void* transform, int dir);

// --- ColorSpaceSet ---
void* ocio_color_space_set_create(void);
void ocio_color_space_set_destroy(void* handle);

int ocio_color_space_set_get_num_color_spaces(void* handle);
void* ocio_color_space_set_get_color_space_name_by_index(void* handle, int index);
void* ocio_color_space_set_get_color_space_by_index(void* handle, int index);
void* ocio_color_space_set_get_color_space(void* handle, const char* name);
int ocio_color_space_set_get_color_space_index(void* handle, const char* name);
bool ocio_color_space_set_has_color_space(void* handle, const char* name);
void ocio_color_space_set_add_color_space(void* handle, void* cs);
void ocio_color_space_set_add_color_spaces(void* handle, void* cs);
void ocio_color_space_set_remove_color_space(void* handle, const char* name);
void ocio_color_space_set_remove_color_spaces(void* handle, void* cs);
void ocio_color_space_set_clear_color_spaces(void* handle);

// --- Look ---
void* ocio_look_create(void);
void ocio_look_destroy(void* handle);

void* ocio_look_get_name(void* handle);
void ocio_look_set_name(void* handle, const char* name);
void* ocio_look_get_process_space(void* handle);
void ocio_look_set_process_space(void* handle, const char* processSpace);
void* ocio_look_get_transform(void* handle);
void ocio_look_set_transform(void* handle, void* transform);
void* ocio_look_get_inverse_transform(void* handle);
void ocio_look_set_inverse_transform(void* handle, void* transform);
void* ocio_look_get_description(void* handle);
void ocio_look_set_description(void* handle, const char* description);
void ocio_look_set_interchange_attribute(void* handle, const char* attrName, void* value);

// --- NamedTransform ---
void* ocio_named_transform_create(void);
void ocio_named_transform_destroy(void* handle);

void* ocio_named_transform_get_name(void* handle);
void ocio_named_transform_set_name(void* handle, const char* name);
size_t ocio_named_transform_get_num_aliases(void* handle);
void* ocio_named_transform_get_alias(void* handle, size_t idx);
bool ocio_named_transform_has_alias(void* handle, const char* alias);
void ocio_named_transform_add_alias(void* handle, const char* alias);
void ocio_named_transform_remove_alias(void* handle, const char* alias);
void ocio_named_transform_clear_aliases(void* handle);
void* ocio_named_transform_get_family(void* handle);
void ocio_named_transform_set_family(void* handle, const char* family);
void* ocio_named_transform_get_description(void* handle);
void ocio_named_transform_set_description(void* handle, const char* description);
bool ocio_named_transform_has_category(void* handle, const char* category);
void ocio_named_transform_add_category(void* handle, const char* category);
void ocio_named_transform_remove_category(void* handle, const char* category);
int ocio_named_transform_get_num_categories(void* handle);
void* ocio_named_transform_get_category(void* handle, int index);
void ocio_named_transform_clear_categories(void* handle);
void* ocio_named_transform_get_encoding(void* handle);
void ocio_named_transform_set_encoding(void* handle, const char* encoding);
void* ocio_named_transform_get_transform(void* handle, int dir);
void ocio_named_transform_set_transform(void* handle, void* transform, int dir);

// --- ViewTransform ---
void* ocio_view_transform_create(void);
void ocio_view_transform_destroy(void* handle);

void* ocio_view_transform_get_name(void* handle);
void ocio_view_transform_set_name(void* handle, const char* name);
void* ocio_view_transform_get_family(void* handle);
void ocio_view_transform_set_family(void* handle, const char* family);
void* ocio_view_transform_get_description(void* handle);
void ocio_view_transform_set_description(void* handle, const char* description);
void ocio_view_transform_set_interchange_attribute(void* handle, const char* attrName, void* value);
bool ocio_view_transform_has_category(void* handle, const char* category);
void ocio_view_transform_add_category(void* handle, const char* category);
void ocio_view_transform_remove_category(void* handle, const char* category);
int ocio_view_transform_get_num_categories(void* handle);
void* ocio_view_transform_get_category(void* handle, int index);
void ocio_view_transform_clear_categories(void* handle);
int ocio_view_transform_get_reference_space_type(void* handle);
void* ocio_view_transform_get_transform(void* handle, int dir);
void ocio_view_transform_set_transform(void* handle, void* transform, int dir);

// --- Processor ---
void* ocio_processor_create(void);
void ocio_processor_destroy(void* handle);

bool ocio_processor_is_no_op(void* handle);
bool ocio_processor_has_channel_crosstalk(void* handle);
void* ocio_processor_get_cache_id(void* handle);
void* ocio_processor_get_processor_metadata(void* handle);
void* ocio_processor_get_format_metadata(void* handle);
int ocio_processor_get_num_transforms(void* handle);
void* ocio_processor_get_transform_format_metadata(void* handle, int index);
void* ocio_processor_create_group_transform(void* handle);
void* ocio_processor_get_dynamic_property(void* handle, int type);
bool ocio_processor_has_dynamic_property(void* handle, int type);
bool ocio_processor_is_dynamic(void* handle);
void* ocio_processor_get_optimized_processor_v1(void* handle, int oFlags);
void* ocio_processor_get_optimized_processor_v2(void* handle, int inBD, int outBD, int oFlags);
void* ocio_processor_get_default_gpu_processor(void* handle);
void* ocio_processor_get_optimized_gpu_processor(void* handle, int oFlags);
void* ocio_processor_get_optimized_legacy_gpu_processor(void* handle, int oFlags, void* edgelen);
void* ocio_processor_get_default_cpu_processor(void* handle);
void* ocio_processor_get_optimized_cpu_processor(void* handle, int oFlags);
void* ocio_processor_get_optimized_cpu_processor_v1(void* handle, int inBitDepth, int outBitDepth, int oFlags);

// --- CPUProcessor ---
void* ocio_cpu_processor_create(void);
void ocio_cpu_processor_destroy(void* handle);

bool ocio_cpu_processor_is_no_op(void* handle);
bool ocio_cpu_processor_is_identity(void* handle);
bool ocio_cpu_processor_has_channel_crosstalk(void* handle);
void* ocio_cpu_processor_get_cache_id(void* handle);
int ocio_cpu_processor_get_input_bit_depth(void* handle);
int ocio_cpu_processor_get_output_bit_depth(void* handle);
void* ocio_cpu_processor_get_dynamic_property(void* handle, int type);
bool ocio_cpu_processor_has_dynamic_property(void* handle, int type);
bool ocio_cpu_processor_is_dynamic(void* handle);
void ocio_cpu_processor_apply_v1(void* handle, void* imgDesc);
void ocio_cpu_processor_apply_v2(void* handle, void* srcImgDesc, void* dstImgDesc);
void ocio_cpu_processor_apply_rgb(void* handle, void* pixel);
void ocio_cpu_processor_apply_rgba(void* handle, void* pixel);

// --- GPUProcessor ---
void* ocio_gpu_processor_create(void);
void ocio_gpu_processor_destroy(void* handle);

bool ocio_gpu_processor_is_no_op(void* handle);
bool ocio_gpu_processor_has_channel_crosstalk(void* handle);
void* ocio_gpu_processor_get_cache_id(void* handle);
void ocio_gpu_processor_extract_gpu_shader_info_v1(void* handle, void* shaderDesc);
void ocio_gpu_processor_extract_gpu_shader_info_v2(void* handle, void* shaderCreator);

// --- GpuShaderDesc ---
void* ocio_gpu_shader_desc_create(void);
void ocio_gpu_shader_desc_destroy(void* handle);

void* ocio_gpu_shader_desc_clone(void* handle);
void* ocio_gpu_shader_desc_get_num_uniforms(void* handle);
void* ocio_gpu_shader_desc_get_uniform(void* handle, void* index, void* data);
void* ocio_gpu_shader_desc_get_uniform_buffer_size(void* handle);
void* ocio_gpu_shader_desc_get_num_textures(void* handle);
void ocio_gpu_shader_desc_get_texture(void* handle, void* index, const char* textureName, const char* samplerName, void* width, void* height, void* channel, void* dimensions, void* interpolation);
void ocio_gpu_shader_desc_get_texture_values(void* handle, void* index, const float* values);
void* ocio_gpu_shader_desc_get_texture_shader_binding_index(void* handle, void* index);
void* ocio_gpu_shader_desc_get_num3d_textures(void* handle);
void ocio_gpu_shader_desc_get3d_texture(void* handle, void* index, const char* textureName, const char* samplerName, void* edgelen, void* interpolation);
void ocio_gpu_shader_desc_get3d_texture_values(void* handle, void* index, const float* values);
void* ocio_gpu_shader_desc_get3d_texture_shader_binding_index(void* handle, void* index);
void* ocio_gpu_shader_desc_get_shader_text(void* handle);

// --- Baker ---
void* ocio_baker_create(void);
void ocio_baker_destroy(void* handle);

void* ocio_baker_get_config(void* handle);
void ocio_baker_set_config(void* handle, void* config);
void* ocio_baker_get_format(void* handle);
void ocio_baker_set_format(void* handle, const char* formatName);
void* ocio_baker_get_format_metadata(void* handle);
void* ocio_baker_get_format_metadata_v1(void* handle);
void* ocio_baker_get_input_space(void* handle);
void ocio_baker_set_input_space(void* handle, const char* inputSpace);
void* ocio_baker_get_shaper_space(void* handle);
void ocio_baker_set_shaper_space(void* handle, const char* shaperSpace);
void* ocio_baker_get_looks(void* handle);
void ocio_baker_set_looks(void* handle, const char* looks);
void* ocio_baker_get_target_space(void* handle);
void ocio_baker_set_target_space(void* handle, const char* targetSpace);
void* ocio_baker_get_display(void* handle);
void* ocio_baker_get_view(void* handle);
void ocio_baker_set_display_view(void* handle, const char* display, const char* view);
int ocio_baker_get_shaper_size(void* handle);
void ocio_baker_set_shaper_size(void* handle, int shapersize);
int ocio_baker_get_cube_size(void* handle);
void ocio_baker_set_cube_size(void* handle, int cubesize);
void ocio_baker_bake(void* handle, void* os);

// --- Context ---
void* ocio_context_create(void);
void ocio_context_destroy(void* handle);

void* ocio_context_get_cache_id(void* handle);
void ocio_context_set_search_path(void* handle, const char* path);
void* ocio_context_get_search_path(void* handle);
int ocio_context_get_num_search_paths(void* handle);
void* ocio_context_get_search_path_by_index(void* handle, int index);
void ocio_context_clear_search_paths(void* handle);
void ocio_context_add_search_path(void* handle, const char* path);
void ocio_context_set_working_dir(void* handle, const char* dirname);
void* ocio_context_get_working_dir(void* handle);
void ocio_context_set_string_var(void* handle, const char* name, const char* value);
void* ocio_context_get_string_var(void* handle, const char* name);
int ocio_context_get_num_string_vars(void* handle);
void* ocio_context_get_string_var_name_by_index(void* handle, int index);
void* ocio_context_get_string_var_by_index(void* handle, int index);
void ocio_context_clear_string_vars(void* handle);
void ocio_context_add_string_vars(void* handle, void* ctx);
void ocio_context_set_environment_mode(void* handle, int mode);
int ocio_context_get_environment_mode(void* handle);
void ocio_context_load_environment(void* handle);
void* ocio_context_resolve_string_var(void* handle, const char* string);
void* ocio_context_resolve_string_var_v1(void* handle, const char* string, void* usedContextVars);
void* ocio_context_resolve_file_location(void* handle, const char* filename);
void* ocio_context_resolve_file_location_v1(void* handle, const char* filename, void* usedContextVars);
void ocio_context_set_config_io_proxy(void* handle, void* ciop);
void* ocio_context_get_config_io_proxy(void* handle);

// --- AllocationTransform ---
void* ocio_allocation_transform_create(void);
void ocio_allocation_transform_destroy(void* handle);

int ocio_allocation_transform_get_direction(void* handle);
void ocio_allocation_transform_set_direction(void* handle, int dir);
void ocio_allocation_transform_validate(void* handle);
int ocio_allocation_transform_get_allocation(void* handle);
void ocio_allocation_transform_set_allocation(void* handle, int allocation);
int ocio_allocation_transform_get_num_vars(void* handle);
void ocio_allocation_transform_get_vars(void* handle, void* vars);
void ocio_allocation_transform_set_vars(void* handle, int numvars, const float* vars);

// --- BuiltinTransform ---
void* ocio_builtin_transform_create(void);
void ocio_builtin_transform_destroy(void* handle);

void* ocio_builtin_transform_get_style(void* handle);
void ocio_builtin_transform_set_style(void* handle, const char* style);
void* ocio_builtin_transform_get_description(void* handle);

// --- CDLTransform ---
void* ocio_cdl_transform_create(void);
void ocio_cdl_transform_destroy(void* handle);

void* ocio_cdl_transform_get_format_metadata_v1(void* handle);
void* ocio_cdl_transform_get_format_metadata_v2(void* handle);
bool ocio_cdl_transform_equals(void* handle, void* other);
int ocio_cdl_transform_get_style(void* handle);
void ocio_cdl_transform_set_style(void* handle, int style);
void ocio_cdl_transform_get_slope(void* handle, void* rgb);
void ocio_cdl_transform_set_slope(void* handle, const double* rgb);
void ocio_cdl_transform_get_offset(void* handle, void* rgb);
void ocio_cdl_transform_set_offset(void* handle, const double* rgb);
void ocio_cdl_transform_get_power(void* handle, void* rgb);
void ocio_cdl_transform_set_power(void* handle, const double* rgb);
void ocio_cdl_transform_get_sop(void* handle, void* vec9);
void ocio_cdl_transform_set_sop(void* handle, const double* vec9);
double ocio_cdl_transform_get_sat(void* handle);
void ocio_cdl_transform_set_sat(void* handle, double sat);
void ocio_cdl_transform_get_sat_luma_coefs(void* handle, void* rgb);
void* ocio_cdl_transform_get_id(void* handle);
void ocio_cdl_transform_set_id(void* handle, const char* id);
void* ocio_cdl_transform_get_first_sop_description(void* handle);
void ocio_cdl_transform_set_first_sop_description(void* handle, const char* description);

// --- ColorSpaceTransform ---
void* ocio_color_space_transform_create(void);
void ocio_color_space_transform_destroy(void* handle);

int ocio_color_space_transform_get_direction(void* handle);
void ocio_color_space_transform_set_direction(void* handle, int dir);
void ocio_color_space_transform_validate(void* handle);
void* ocio_color_space_transform_get_src(void* handle);
void ocio_color_space_transform_set_src(void* handle, const char* src);
void* ocio_color_space_transform_get_dst(void* handle);
void ocio_color_space_transform_set_dst(void* handle, const char* dst);
bool ocio_color_space_transform_get_data_bypass(void* handle);
void ocio_color_space_transform_set_data_bypass(void* handle, bool enabled);

// --- DisplayViewTransform ---
void* ocio_display_view_transform_create(void);
void ocio_display_view_transform_destroy(void* handle);

int ocio_display_view_transform_get_direction(void* handle);
void ocio_display_view_transform_set_direction(void* handle, int dir);
void ocio_display_view_transform_validate(void* handle);
void* ocio_display_view_transform_get_src(void* handle);
void ocio_display_view_transform_set_src(void* handle, const char* name);
void* ocio_display_view_transform_get_display(void* handle);
void ocio_display_view_transform_set_display(void* handle, const char* display);
void* ocio_display_view_transform_get_view(void* handle);
void ocio_display_view_transform_set_view(void* handle, const char* view);
bool ocio_display_view_transform_get_looks_bypass(void* handle);
void ocio_display_view_transform_set_looks_bypass(void* handle, bool bypass);
bool ocio_display_view_transform_get_data_bypass(void* handle);
void ocio_display_view_transform_set_data_bypass(void* handle, bool bypass);

// --- ExponentTransform ---
void* ocio_exponent_transform_create(void);
void ocio_exponent_transform_destroy(void* handle);

void* ocio_exponent_transform_get_format_metadata_v1(void* handle);
void* ocio_exponent_transform_get_format_metadata_v2(void* handle);
bool ocio_exponent_transform_equals(void* handle, void* other);
int ocio_exponent_transform_get_negative_style(void* handle);
void ocio_exponent_transform_set_negative_style(void* handle, int style);

// --- ExponentWithLinearTransform ---
void* ocio_exponent_with_linear_transform_create(void);
void ocio_exponent_with_linear_transform_destroy(void* handle);

void* ocio_exponent_with_linear_transform_get_format_metadata_v1(void* handle);
void* ocio_exponent_with_linear_transform_get_format_metadata_v2(void* handle);
bool ocio_exponent_with_linear_transform_equals(void* handle, void* other);
int ocio_exponent_with_linear_transform_get_negative_style(void* handle);
void ocio_exponent_with_linear_transform_set_negative_style(void* handle, int style);

// --- ExposureContrastTransform ---
void* ocio_exposure_contrast_transform_create(void);
void ocio_exposure_contrast_transform_destroy(void* handle);

void* ocio_exposure_contrast_transform_get_format_metadata_v1(void* handle);
void* ocio_exposure_contrast_transform_get_format_metadata_v2(void* handle);
bool ocio_exposure_contrast_transform_equals(void* handle, void* other);
int ocio_exposure_contrast_transform_get_style(void* handle);
void ocio_exposure_contrast_transform_set_style(void* handle, int style);
double ocio_exposure_contrast_transform_get_exposure(void* handle);
void ocio_exposure_contrast_transform_set_exposure(void* handle, double exposure);
bool ocio_exposure_contrast_transform_is_exposure_dynamic(void* handle);
void ocio_exposure_contrast_transform_make_exposure_dynamic(void* handle);
void ocio_exposure_contrast_transform_make_exposure_non_dynamic(void* handle);
double ocio_exposure_contrast_transform_get_contrast(void* handle);
void ocio_exposure_contrast_transform_set_contrast(void* handle, double contrast);
bool ocio_exposure_contrast_transform_is_contrast_dynamic(void* handle);
void ocio_exposure_contrast_transform_make_contrast_dynamic(void* handle);
void ocio_exposure_contrast_transform_make_contrast_non_dynamic(void* handle);
double ocio_exposure_contrast_transform_get_gamma(void* handle);
void ocio_exposure_contrast_transform_set_gamma(void* handle, double gamma);
bool ocio_exposure_contrast_transform_is_gamma_dynamic(void* handle);
void ocio_exposure_contrast_transform_make_gamma_dynamic(void* handle);
void ocio_exposure_contrast_transform_make_gamma_non_dynamic(void* handle);
double ocio_exposure_contrast_transform_get_pivot(void* handle);
void ocio_exposure_contrast_transform_set_pivot(void* handle, double pivot);
double ocio_exposure_contrast_transform_get_log_exposure_step(void* handle);
void ocio_exposure_contrast_transform_set_log_exposure_step(void* handle, double logExposureStep);
double ocio_exposure_contrast_transform_get_log_mid_gray(void* handle);
void ocio_exposure_contrast_transform_set_log_mid_gray(void* handle, double logMidGray);

// --- FileTransform ---
void* ocio_file_transform_create(void);
void ocio_file_transform_destroy(void* handle);

int ocio_file_transform_get_direction(void* handle);
void ocio_file_transform_set_direction(void* handle, int dir);
void ocio_file_transform_validate(void* handle);
void* ocio_file_transform_get_src(void* handle);
void ocio_file_transform_set_src(void* handle, const char* src);
void* ocio_file_transform_get_ccc_id(void* handle);
void ocio_file_transform_set_ccc_id(void* handle, const char* id);
int ocio_file_transform_get_cdl_style(void* handle);
void ocio_file_transform_set_cdl_style(void* handle, int arg0);
int ocio_file_transform_get_interpolation(void* handle);
void ocio_file_transform_set_interpolation(void* handle, int interp);

// --- FixedFunctionTransform ---
void* ocio_fixed_function_transform_create(void);
void ocio_fixed_function_transform_destroy(void* handle);

void* ocio_fixed_function_transform_get_format_metadata_v1(void* handle);
void* ocio_fixed_function_transform_get_format_metadata_v2(void* handle);
bool ocio_fixed_function_transform_equals(void* handle, void* other);
int ocio_fixed_function_transform_get_style(void* handle);
void ocio_fixed_function_transform_set_style(void* handle, int style);
size_t ocio_fixed_function_transform_get_num_params(void* handle);
void ocio_fixed_function_transform_get_params(void* handle, void* params);
void ocio_fixed_function_transform_set_params(void* handle, const double* params, size_t num);

// --- GradingPrimaryTransform ---
void* ocio_grading_primary_transform_create(void);
void ocio_grading_primary_transform_destroy(void* handle);

void* ocio_grading_primary_transform_get_format_metadata_v1(void* handle);
void* ocio_grading_primary_transform_get_format_metadata_v2(void* handle);
bool ocio_grading_primary_transform_equals(void* handle, void* other);
int ocio_grading_primary_transform_get_style(void* handle);
void ocio_grading_primary_transform_set_style(void* handle, int style);
void* ocio_grading_primary_transform_get_value(void* handle);
void ocio_grading_primary_transform_set_value(void* handle, void* values);
bool ocio_grading_primary_transform_is_dynamic(void* handle);
void ocio_grading_primary_transform_make_dynamic(void* handle);
void ocio_grading_primary_transform_make_non_dynamic(void* handle);

// --- GradingRGBCurveTransform ---
void* ocio_grading_rgb_curve_transform_create(void);
void ocio_grading_rgb_curve_transform_destroy(void* handle);

void* ocio_grading_rgb_curve_transform_get_format_metadata_v1(void* handle);
void* ocio_grading_rgb_curve_transform_get_format_metadata_v2(void* handle);
bool ocio_grading_rgb_curve_transform_equals(void* handle, void* other);
int ocio_grading_rgb_curve_transform_get_style(void* handle);
void ocio_grading_rgb_curve_transform_set_style(void* handle, int style);
void* ocio_grading_rgb_curve_transform_get_value(void* handle);
void ocio_grading_rgb_curve_transform_set_value(void* handle, void* values);
float ocio_grading_rgb_curve_transform_get_slope(void* handle, int c, size_t index);
void ocio_grading_rgb_curve_transform_set_slope(void* handle, int c, size_t index, float slope);
bool ocio_grading_rgb_curve_transform_slopes_are_default(void* handle, int c);
bool ocio_grading_rgb_curve_transform_get_bypass_lin_to_log(void* handle);
void ocio_grading_rgb_curve_transform_set_bypass_lin_to_log(void* handle, bool bypass);
bool ocio_grading_rgb_curve_transform_is_dynamic(void* handle);
void ocio_grading_rgb_curve_transform_make_dynamic(void* handle);
void ocio_grading_rgb_curve_transform_make_non_dynamic(void* handle);

// --- GradingHueCurveTransform ---
void* ocio_grading_hue_curve_transform_create(void);
void ocio_grading_hue_curve_transform_destroy(void* handle);

void* ocio_grading_hue_curve_transform_get_format_metadata_v1(void* handle);
void* ocio_grading_hue_curve_transform_get_format_metadata_v2(void* handle);
bool ocio_grading_hue_curve_transform_equals(void* handle, void* other);
int ocio_grading_hue_curve_transform_get_style(void* handle);
void ocio_grading_hue_curve_transform_set_style(void* handle, int style);
void* ocio_grading_hue_curve_transform_get_value(void* handle);
void ocio_grading_hue_curve_transform_set_value(void* handle, void* value);
float ocio_grading_hue_curve_transform_get_slope(void* handle, int c, size_t index);
void ocio_grading_hue_curve_transform_set_slope(void* handle, int c, size_t index, float slope);
bool ocio_grading_hue_curve_transform_slopes_are_default(void* handle, int c);
int ocio_grading_hue_curve_transform_get_rgb_to_hsy(void* handle);
void ocio_grading_hue_curve_transform_set_rgb_to_hsy(void* handle, int style);
bool ocio_grading_hue_curve_transform_is_dynamic(void* handle);
void ocio_grading_hue_curve_transform_make_dynamic(void* handle);
void ocio_grading_hue_curve_transform_make_non_dynamic(void* handle);

// --- GradingToneTransform ---
void* ocio_grading_tone_transform_create(void);
void ocio_grading_tone_transform_destroy(void* handle);

void* ocio_grading_tone_transform_get_format_metadata_v1(void* handle);
void* ocio_grading_tone_transform_get_format_metadata_v2(void* handle);
bool ocio_grading_tone_transform_equals(void* handle, void* other);
int ocio_grading_tone_transform_get_style(void* handle);
void ocio_grading_tone_transform_set_style(void* handle, int style);
void* ocio_grading_tone_transform_get_value(void* handle);
void ocio_grading_tone_transform_set_value(void* handle, void* values);
bool ocio_grading_tone_transform_is_dynamic(void* handle);
void ocio_grading_tone_transform_make_dynamic(void* handle);
void ocio_grading_tone_transform_make_non_dynamic(void* handle);

// --- GroupTransform ---
void* ocio_group_transform_create(void);
void ocio_group_transform_destroy(void* handle);

void* ocio_group_transform_get_format_metadata_v1(void* handle);
void* ocio_group_transform_get_format_metadata_v2(void* handle);
void* ocio_group_transform_get_transform(void* handle, int index);
void* ocio_group_transform_get_transform_v1(void* handle, int index);
int ocio_group_transform_get_num_transforms(void* handle);
void ocio_group_transform_append_transform(void* handle, void* transform);
void ocio_group_transform_prepend_transform(void* handle, void* transform);
void ocio_group_transform_write(void* handle, void* config, const char* formatName, void* os);

// --- LogAffineTransform ---
void* ocio_log_affine_transform_create(void);
void ocio_log_affine_transform_destroy(void* handle);

void* ocio_log_affine_transform_get_format_metadata_v1(void* handle);
void* ocio_log_affine_transform_get_format_metadata_v2(void* handle);
bool ocio_log_affine_transform_equals(void* handle, void* other);
double ocio_log_affine_transform_get_base(void* handle);
void ocio_log_affine_transform_set_base(void* handle, double base);

// --- LogCameraTransform ---
void* ocio_log_camera_transform_create(void);
void ocio_log_camera_transform_destroy(void* handle);

void* ocio_log_camera_transform_get_format_metadata_v1(void* handle);
void* ocio_log_camera_transform_get_format_metadata_v2(void* handle);
bool ocio_log_camera_transform_equals(void* handle, void* other);
double ocio_log_camera_transform_get_base(void* handle);
void ocio_log_camera_transform_set_base(void* handle, double base);
void ocio_log_camera_transform_unset_linear_slope_value(void* handle);

// --- LogTransform ---
void* ocio_log_transform_create(void);
void ocio_log_transform_destroy(void* handle);

void* ocio_log_transform_get_format_metadata_v1(void* handle);
void* ocio_log_transform_get_format_metadata_v2(void* handle);
bool ocio_log_transform_equals(void* handle, void* other);
double ocio_log_transform_get_base(void* handle);
void ocio_log_transform_set_base(void* handle, double val);

// --- LookTransform ---
void* ocio_look_transform_create(void);
void ocio_look_transform_destroy(void* handle);

int ocio_look_transform_get_direction(void* handle);
void ocio_look_transform_set_direction(void* handle, int dir);
void ocio_look_transform_validate(void* handle);
void* ocio_look_transform_get_src(void* handle);
void ocio_look_transform_set_src(void* handle, const char* src);
void* ocio_look_transform_get_dst(void* handle);
void ocio_look_transform_set_dst(void* handle, const char* dst);
void* ocio_look_transform_get_looks(void* handle);
void ocio_look_transform_set_looks(void* handle, const char* looks);
bool ocio_look_transform_get_skip_color_space_conversion(void* handle);
void ocio_look_transform_set_skip_color_space_conversion(void* handle, bool skip);

// --- Lut1DTransform ---
void* ocio_lut1d_transform_create(void);
void ocio_lut1d_transform_destroy(void* handle);

int ocio_lut1d_transform_get_file_output_bit_depth(void* handle);
void ocio_lut1d_transform_set_file_output_bit_depth(void* handle, int bitDepth);
void* ocio_lut1d_transform_get_format_metadata_v1(void* handle);
void* ocio_lut1d_transform_get_format_metadata_v2(void* handle);
bool ocio_lut1d_transform_equals(void* handle, void* other);
void* ocio_lut1d_transform_get_length(void* handle);
void ocio_lut1d_transform_set_length(void* handle, void* length);
void ocio_lut1d_transform_get_value(void* handle, void* index, void* r, void* g, void* b);
void ocio_lut1d_transform_set_value(void* handle, void* index, float r, float g, float b);
bool ocio_lut1d_transform_get_input_half_domain(void* handle);
void ocio_lut1d_transform_set_input_half_domain(void* handle, bool isHalfDomain);
bool ocio_lut1d_transform_get_output_raw_halfs(void* handle);
void ocio_lut1d_transform_set_output_raw_halfs(void* handle, bool isRawHalfs);
int ocio_lut1d_transform_get_hue_adjust(void* handle);
void ocio_lut1d_transform_set_hue_adjust(void* handle, int algo);
int ocio_lut1d_transform_get_interpolation(void* handle);
void ocio_lut1d_transform_set_interpolation(void* handle, int algo);

// --- Lut3DTransform ---
void* ocio_lut3d_transform_create(void);
void ocio_lut3d_transform_destroy(void* handle);

int ocio_lut3d_transform_get_file_output_bit_depth(void* handle);
void ocio_lut3d_transform_set_file_output_bit_depth(void* handle, int bitDepth);
void* ocio_lut3d_transform_get_format_metadata_v1(void* handle);
void* ocio_lut3d_transform_get_format_metadata_v2(void* handle);
bool ocio_lut3d_transform_equals(void* handle, void* other);
void* ocio_lut3d_transform_get_grid_size(void* handle);
void ocio_lut3d_transform_set_grid_size(void* handle, void* gridSize);
void ocio_lut3d_transform_get_value(void* handle, void* indexR, void* indexG, void* indexB, void* r, void* g, void* b);
void ocio_lut3d_transform_set_value(void* handle, void* indexR, void* indexG, void* indexB, float r, float g, float b);
int ocio_lut3d_transform_get_interpolation(void* handle);
void ocio_lut3d_transform_set_interpolation(void* handle, int algo);

// --- MatrixTransform ---
void* ocio_matrix_transform_create(void);
void ocio_matrix_transform_destroy(void* handle);

void* ocio_matrix_transform_get_format_metadata_v1(void* handle);
void* ocio_matrix_transform_get_format_metadata_v2(void* handle);
bool ocio_matrix_transform_equals(void* handle, void* other);
void ocio_matrix_transform_get_matrix(void* handle, void* m44);
void ocio_matrix_transform_set_matrix(void* handle, const double* m44);
void ocio_matrix_transform_get_offset(void* handle, void* offset4);
void ocio_matrix_transform_set_offset(void* handle, const double* offset4);
int ocio_matrix_transform_get_file_input_bit_depth(void* handle);
void ocio_matrix_transform_set_file_input_bit_depth(void* handle, int bitDepth);
int ocio_matrix_transform_get_file_output_bit_depth(void* handle);
void ocio_matrix_transform_set_file_output_bit_depth(void* handle, int bitDepth);

// --- RangeTransform ---
void* ocio_range_transform_create(void);
void ocio_range_transform_destroy(void* handle);

int ocio_range_transform_get_style(void* handle);
void ocio_range_transform_set_style(void* handle, int style);
void* ocio_range_transform_get_format_metadata_v1(void* handle);
void* ocio_range_transform_get_format_metadata_v2(void* handle);
bool ocio_range_transform_equals(void* handle, void* other);
int ocio_range_transform_get_file_input_bit_depth(void* handle);
void ocio_range_transform_set_file_input_bit_depth(void* handle, int bitDepth);
int ocio_range_transform_get_file_output_bit_depth(void* handle);
void ocio_range_transform_set_file_output_bit_depth(void* handle, int bitDepth);
double ocio_range_transform_get_min_in_value(void* handle);
void ocio_range_transform_set_min_in_value(void* handle, double val);
bool ocio_range_transform_has_min_in_value(void* handle);
void ocio_range_transform_unset_min_in_value(void* handle);
void ocio_range_transform_set_max_in_value(void* handle, double val);
double ocio_range_transform_get_max_in_value(void* handle);
bool ocio_range_transform_has_max_in_value(void* handle);
void ocio_range_transform_unset_max_in_value(void* handle);
void ocio_range_transform_set_min_out_value(void* handle, double val);
double ocio_range_transform_get_min_out_value(void* handle);
bool ocio_range_transform_has_min_out_value(void* handle);
void ocio_range_transform_unset_min_out_value(void* handle);
void ocio_range_transform_set_max_out_value(void* handle, double val);
double ocio_range_transform_get_max_out_value(void* handle);
bool ocio_range_transform_has_max_out_value(void* handle);
void ocio_range_transform_unset_max_out_value(void* handle);

// --- DynamicProperty ---
void ocio_dynamic_property_destroy(void* handle);
int ocio_dynamic_property_get_type(void* handle);

}  // extern "C"
