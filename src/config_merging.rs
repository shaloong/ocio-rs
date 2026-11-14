//! Native OCIO 2.5.1+ configuration merging.
use crate::{ColorSpace, Config, OcioError, Result};
use std::{
    ffi::{c_char, c_void, CStr},
    ptr::NonNull,
};

unsafe fn text<'a>(value: *const c_char) -> &'a CStr {
    if value.is_null() {
        c""
    } else {
        unsafe { CStr::from_ptr(value) }
    }
}

/// Conflict policy used by OpenColorIO when merging a configuration section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MergeStrategy {
    PreferInput = 0,
    PreferBase = 1,
    InputOnly = 2,
    BaseOnly = 3,
    Remove = 4,
    Unspecified = 5,
}
impl MergeStrategy {
    fn from_native(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Self::PreferInput),
            1 => Ok(Self::PreferBase),
            2 => Ok(Self::InputOnly),
            3 => Ok(Self::BaseOnly),
            4 => Ok(Self::Remove),
            5 => Ok(Self::Unspecified),
            _ => Err(OcioError::Ocio(format!("Unknown merge strategy: {value}"))),
        }
    }
}

/// Owned native configuration merge parameters.
pub struct ConfigMergingParameters {
    handle: NonNull<c_void>,
}
impl ConfigMergingParameters {
    /// Create via the native merge API.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_merging_parameters_create() };
        Ok(Self {
            handle: crate::handle_result(value)?,
        })
    }

    /// Create editable copy via the native merge API.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_create_editable_copy(self.handle.as_ptr())
        };
        Ok(Self {
            handle: crate::handle_result(value)?,
        })
    }

    /// Base config name via the native merge API.
    pub fn base_config_name(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_base_config_name(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Set base config name via the native merge API.
    pub fn set_base_config_name(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_base_config_name(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Input config name via the native merge API.
    pub fn input_config_name(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_input_config_name(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Set input config name via the native merge API.
    pub fn set_input_config_name(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_input_config_name(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Output name via the native merge API.
    pub fn output_name(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_output_name(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Set output name via the native merge API.
    pub fn set_output_name(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_output_name(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Input family prefix via the native merge API.
    pub fn input_family_prefix(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_input_family_prefix(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Set input family prefix via the native merge API.
    pub fn set_input_family_prefix(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_input_family_prefix(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Base family prefix via the native merge API.
    pub fn base_family_prefix(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_base_family_prefix(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Set base family prefix via the native merge API.
    pub fn set_base_family_prefix(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_base_family_prefix(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Name via the native merge API.
    pub fn name(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_config_merging_parameters_get_name(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Set name via the native merge API.
    pub fn set_name(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_name(self.handle.as_ptr(), value.as_ptr())
        };
        crate::ocio_call_status()
    }

    /// Description via the native merge API.
    pub fn description(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_description(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Set description via the native merge API.
    pub fn set_description(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_description(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Search path via the native merge API.
    pub fn search_path(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_search_path(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Set search path via the native merge API.
    pub fn set_search_path(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_search_path(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Active displays via the native merge API.
    pub fn active_displays(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_active_displays(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Set active displays via the native merge API.
    pub fn set_active_displays(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_active_displays(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Active views via the native merge API.
    pub fn active_views(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_active_views(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Set active views via the native merge API.
    pub fn set_active_views(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_active_views(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Inactive color spaces via the native merge API.
    pub fn inactive_color_spaces(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_inactive_color_spaces(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Set inactive color spaces via the native merge API.
    pub fn set_inactive_color_spaces(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_inactive_color_spaces(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Is input first via the native merge API.
    pub fn is_input_first(&self) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_is_input_first(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set input first via the native merge API.
    pub fn set_input_first(&mut self, value: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_input_first(self.handle.as_ptr(), value)
        };
        crate::ocio_call_status()
    }

    /// Is error on conflict via the native merge API.
    pub fn is_error_on_conflict(&self) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_is_error_on_conflict(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set error on conflict via the native merge API.
    pub fn set_error_on_conflict(&mut self, value: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_error_on_conflict(
                self.handle.as_ptr(),
                value,
            )
        };
        crate::ocio_call_status()
    }

    /// Is avoid duplicates via the native merge API.
    pub fn is_avoid_duplicates(&self) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_is_avoid_duplicates(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set avoid duplicates via the native merge API.
    pub fn set_avoid_duplicates(&mut self, value: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_avoid_duplicates(
                self.handle.as_ptr(),
                value,
            )
        };
        crate::ocio_call_status()
    }

    /// Is adjust input reference space via the native merge API.
    pub fn is_adjust_input_reference_space(&self) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_is_adjust_input_reference_space(
                self.handle.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set adjust input reference space via the native merge API.
    pub fn set_adjust_input_reference_space(&mut self, value: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_adjust_input_reference_space(
                self.handle.as_ptr(),
                value,
            )
        };
        crate::ocio_call_status()
    }

    /// Default strategy via the native merge API.
    pub fn default_strategy(&self) -> Result<MergeStrategy> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_default_strategy(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        MergeStrategy::from_native(value)
    }

    /// Set default strategy via the native merge API.
    pub fn set_default_strategy(&mut self, value: MergeStrategy) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_default_strategy(
                self.handle.as_ptr(),
                value as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// Roles via the native merge API.
    pub fn roles(&self) -> Result<MergeStrategy> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_config_merging_parameters_get_roles(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        MergeStrategy::from_native(value)
    }

    /// Set roles via the native merge API.
    pub fn set_roles(&mut self, value: MergeStrategy) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_roles(self.handle.as_ptr(), value as i32)
        };
        crate::ocio_call_status()
    }

    /// File rules via the native merge API.
    pub fn file_rules(&self) -> Result<MergeStrategy> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_file_rules(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        MergeStrategy::from_native(value)
    }

    /// Set file rules via the native merge API.
    pub fn set_file_rules(&mut self, value: MergeStrategy) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_file_rules(
                self.handle.as_ptr(),
                value as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// Display views via the native merge API.
    pub fn display_views(&self) -> Result<MergeStrategy> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_display_views(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        MergeStrategy::from_native(value)
    }

    /// Set display views via the native merge API.
    pub fn set_display_views(&mut self, value: MergeStrategy) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_display_views(
                self.handle.as_ptr(),
                value as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// View transforms via the native merge API.
    pub fn view_transforms(&self) -> Result<MergeStrategy> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_view_transforms(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        MergeStrategy::from_native(value)
    }

    /// Set view transforms via the native merge API.
    pub fn set_view_transforms(&mut self, value: MergeStrategy) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_view_transforms(
                self.handle.as_ptr(),
                value as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// Looks via the native merge API.
    pub fn looks(&self) -> Result<MergeStrategy> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_config_merging_parameters_get_looks(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        MergeStrategy::from_native(value)
    }

    /// Set looks via the native merge API.
    pub fn set_looks(&mut self, value: MergeStrategy) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_looks(self.handle.as_ptr(), value as i32)
        };
        crate::ocio_call_status()
    }

    /// Color spaces via the native merge API.
    pub fn color_spaces(&self) -> Result<MergeStrategy> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_color_spaces(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        MergeStrategy::from_native(value)
    }

    /// Set color spaces via the native merge API.
    pub fn set_color_spaces(&mut self, value: MergeStrategy) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_color_spaces(
                self.handle.as_ptr(),
                value as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// Named transforms via the native merge API.
    pub fn named_transforms(&self) -> Result<MergeStrategy> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_named_transforms(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        MergeStrategy::from_native(value)
    }

    /// Set named transforms via the native merge API.
    pub fn set_named_transforms(&mut self, value: MergeStrategy) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_set_named_transforms(
                self.handle.as_ptr(),
                value as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// Add search path via the native merge API.
    pub fn add_search_path(&mut self, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_add_search_path(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Add environment var via the native merge API.
    pub fn add_environment_var(&mut self, name: &CStr, value: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merging_parameters_add_environment_var(
                self.handle.as_ptr(),
                name.as_ptr(),
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Num environment vars via the native merge API.
    pub fn num_environment_vars(&self) -> Result<i32> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_num_environment_vars(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Environment var via the native merge API.
    pub fn environment_var(&self, index: i32) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_environment_var(
                self.handle.as_ptr(),
                index,
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Environment var value via the native merge API.
    pub fn environment_var_value(&self, index: i32) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_get_environment_var_value(
                self.handle.as_ptr(),
                index,
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Merge configs via the native merge API.
    pub fn merge_configs(&self, base: &Config, input: &Config) -> Result<Config> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_merge_configs(
                self.handle.as_ptr(),
                base.handle.as_ptr(),
                input.handle.as_ptr(),
            )
        };
        Ok(Config {
            handle: crate::handle_result(value)?,
        })
    }

    /// Merge color space via the native merge API.
    pub fn merge_color_space(&self, base: &Config, input: &ColorSpace) -> Result<Config> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_merging_parameters_merge_color_space(
                self.handle.as_ptr(),
                base.handle.as_ptr(),
                input.handle.as_ptr(),
            )
        };
        Ok(Config {
            handle: crate::handle_result(value)?,
        })
    }
}
impl Drop for ConfigMergingParameters {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_config_merging_parameters_destroy(self.handle.as_ptr()) }
    }
}

/// An owned OCIOM merge controller. Returned params and configs are editable copies.
pub struct ConfigMerger {
    handle: NonNull<c_void>,
}
impl ConfigMerger {
    /// create using native OCIO.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_merger_create() };
        crate::ocio_call_status()?;
        Ok(Self {
            handle: crate::handle_result(value)?,
        })
    }

    /// from file using native OCIO.
    pub fn from_file(path: &CStr) -> Result<Self> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_merger_from_file(path.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(Self {
            handle: crate::handle_result(value)?,
        })
    }

    /// create editable copy using native OCIO.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_config_merger_create_editable_copy(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(Self {
            handle: crate::handle_result(value)?,
        })
    }

    /// set search path using native OCIO.
    pub fn set_search_path(&mut self, v: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_merger_set_search_path(self.handle.as_ptr(), v.as_ptr()) };
        crate::ocio_call_status()
    }

    /// add search path using native OCIO.
    pub fn add_search_path(&mut self, v: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_merger_add_search_path(self.handle.as_ptr(), v.as_ptr()) };
        crate::ocio_call_status()
    }

    /// num search paths using native OCIO.
    pub fn num_search_paths(&self) -> Result<i32> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_merger_num_search_paths(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// search path using native OCIO.
    pub fn search_path(&self, index: i32) -> Result<&CStr> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_config_merger_search_path(self.handle.as_ptr(), index) };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// set working dir using native OCIO.
    pub fn set_working_dir(&mut self, v: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_merger_set_working_dir(self.handle.as_ptr(), v.as_ptr()) };
        crate::ocio_call_status()
    }

    /// working dir using native OCIO.
    pub fn working_dir(&self) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_merger_working_dir(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// num params using native OCIO.
    pub fn num_params(&self) -> Result<i32> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_merger_num_params(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// params using native OCIO.
    pub fn params(&self, index: i32) -> Result<Option<ConfigMergingParameters>> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_merger_params(self.handle.as_ptr(), index) };
        crate::ocio_call_status()?;
        Ok(NonNull::new(value).map(|handle| ConfigMergingParameters { handle }))
    }

    /// add params using native OCIO.
    pub fn add_params(&mut self, params: &ConfigMergingParameters) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_merger_add_params(self.handle.as_ptr(), params.handle.as_ptr())
        };
        crate::ocio_call_status()
    }

    /// merge configs using native OCIO.
    pub fn merge_configs(&self) -> Result<Self> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_merger_merge_configs(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(Self {
            handle: crate::handle_result(value)?,
        })
    }

    /// num merged configs using native OCIO.
    pub fn num_merged_configs(&self) -> Result<i32> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_config_merger_num_merged_configs(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// merged config using native OCIO.
    pub fn merged_config(&self, index: i32) -> Result<Option<Config>> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_config_merger_merged_config(self.handle.as_ptr(), index) };
        crate::ocio_call_status()?;
        Ok(NonNull::new(value).map(|handle| Config { handle }))
    }

    /// serialize using native OCIO.
    pub fn serialize(&self) -> Result<String> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_merger_serialize(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) }.to_string_lossy().into_owned())
    }

    /// set version using native OCIO.
    pub fn set_version(&mut self, major: u32, minor: u32) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_merger_set_version(self.handle.as_ptr(), major, minor) };
        crate::ocio_call_status()
    }

    /// major version using native OCIO.
    pub fn major_version(&self) -> Result<u32> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_merger_major_version(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// minor version using native OCIO.
    pub fn minor_version(&self) -> Result<u32> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_merger_minor_version(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
    }
}
impl Drop for ConfigMerger {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_config_merger_destroy(self.handle.as_ptr()) }
    }
}
