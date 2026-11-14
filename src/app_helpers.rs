//! Native OpenColorIO color-space menus.

use std::{
    ffi::{c_char, c_void, CStr},
    ptr::NonNull,
};

use crate::{Config, Processor, Result, SearchReferenceSpaceType, TransformDirection};

unsafe fn text<'a>(ptr: *const c_char) -> &'a CStr {
    if ptr.is_null() {
        c""
    } else {
        unsafe { CStr::from_ptr(ptr) }
    }
}

/// Mutable native options used to construct a color-space menu.
pub struct ColorSpaceMenuParameters {
    handle: NonNull<c_void>,
}

impl ColorSpaceMenuParameters {
    /// Create options for a configuration. The native parameters retain the config.
    pub fn create(config: &Config) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_color_space_menu_parameters_create(config.handle.as_ptr()) };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }

    /// Replace the retained configuration.
    pub fn set_config(&mut self, config: &Config) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_set_config(
                self.handle.as_ptr(),
                config.handle.as_ptr(),
            )
        }
    }

    /// Return an independent editable copy of the retained native configuration.
    pub fn config(&self) -> Result<Config> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_color_space_menu_parameters_get_config(self.handle.as_ptr()) };
        Ok(Config {
            handle: crate::handle_result(handle)?,
        })
    }

    /// Return the native role setting.
    pub fn role(&self) -> &CStr {
        unsafe {
            text(ocio_sys::ocio_color_space_menu_parameters_get_role(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Update the native role setting.
    pub fn set_role(&mut self, value: &CStr) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_set_role(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        }
    }

    /// Return the native include color spaces setting.
    pub fn include_color_spaces(&self) -> bool {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_get_include_color_spaces(
                self.handle.as_ptr(),
            )
        }
    }

    /// Update the native include color spaces setting.
    pub fn set_include_color_spaces(&mut self, value: bool) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_set_include_color_spaces(
                self.handle.as_ptr(),
                value,
            )
        }
    }

    /// Return the native include named transforms setting.
    pub fn include_named_transforms(&self) -> bool {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_get_include_named_transforms(
                self.handle.as_ptr(),
            )
        }
    }

    /// Update the native include named transforms setting.
    pub fn set_include_named_transforms(&mut self, value: bool) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_set_include_named_transforms(
                self.handle.as_ptr(),
                value,
            )
        }
    }

    #[cfg(feature = "v2_5")]
    /// Return the native treat no category as any setting.
    pub fn treat_no_category_as_any(&self) -> bool {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_get_treat_no_category_as_any(
                self.handle.as_ptr(),
            )
        }
    }

    #[cfg(feature = "v2_5")]
    /// Update the native treat no category as any setting.
    pub fn set_treat_no_category_as_any(&mut self, value: bool) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_set_treat_no_category_as_any(
                self.handle.as_ptr(),
                value,
            )
        }
    }

    /// Return the native app categories setting.
    pub fn app_categories(&self) -> &CStr {
        unsafe {
            text(
                ocio_sys::ocio_color_space_menu_parameters_get_app_categories(self.handle.as_ptr()),
            )
        }
    }

    /// Update the native app categories setting.
    pub fn set_app_categories(&mut self, value: &CStr) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_set_app_categories(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        }
    }

    /// Return the native encodings setting.
    pub fn encodings(&self) -> &CStr {
        unsafe {
            text(ocio_sys::ocio_color_space_menu_parameters_get_encodings(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Update the native encodings setting.
    pub fn set_encodings(&mut self, value: &CStr) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_set_encodings(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        }
    }

    /// Return the native user categories setting.
    pub fn user_categories(&self) -> &CStr {
        unsafe {
            text(
                ocio_sys::ocio_color_space_menu_parameters_get_user_categories(
                    self.handle.as_ptr(),
                ),
            )
        }
    }

    /// Update the native user categories setting.
    pub fn set_user_categories(&mut self, value: &CStr) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_set_user_categories(
                self.handle.as_ptr(),
                value.as_ptr(),
            )
        }
    }

    /// Return the native include roles setting.
    pub fn include_roles(&self) -> bool {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_get_include_roles(self.handle.as_ptr())
        }
    }

    /// Update the native include roles setting.
    pub fn set_include_roles(&mut self, value: bool) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_set_include_roles(
                self.handle.as_ptr(),
                value,
            )
        }
    }

    /// Return the native search reference space type setting.
    pub fn search_reference_space_type(&self) -> SearchReferenceSpaceType {
        match unsafe {
            ocio_sys::ocio_color_space_menu_parameters_get_search_reference_space_type(
                self.handle.as_ptr(),
            )
        } {
            0 => SearchReferenceSpaceType::Scene,
            1 => SearchReferenceSpaceType::Display,
            _ => SearchReferenceSpaceType::All,
        }
    }

    /// Update the native search reference space type setting.
    pub fn set_search_reference_space_type(&mut self, value: SearchReferenceSpaceType) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_set_search_reference_space_type(
                self.handle.as_ptr(),
                value as i32,
            )
        }
    }

    /// Include an additional color space or named transform by name.
    pub fn add_color_space(&mut self, name: &CStr) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_add_color_space(
                self.handle.as_ptr(),
                name.as_ptr(),
            )
        }
    }

    /// Count explicitly added entries before native menu construction.
    pub fn num_added_color_spaces(&self) -> usize {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_get_num_added_color_spaces(
                self.handle.as_ptr(),
            )
        }
    }

    /// Return an explicitly added name, or an empty string for an invalid index.
    pub fn added_color_space(&self, index: usize) -> &CStr {
        unsafe {
            text(
                ocio_sys::ocio_color_space_menu_parameters_get_added_color_space(
                    self.handle.as_ptr(),
                    index,
                ),
            )
        }
    }

    /// Clear explicitly added entries.
    pub fn clear_added_color_spaces(&mut self) {
        unsafe {
            ocio_sys::ocio_color_space_menu_parameters_clear_added_color_spaces(
                self.handle.as_ptr(),
            )
        }
    }
}

impl Drop for ColorSpaceMenuParameters {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_color_space_menu_parameters_destroy(self.handle.as_ptr()) }
    }
}

/// Read-only native color-space menu. Strings borrow the menu's storage.
pub struct ColorSpaceMenuHelper {
    handle: NonNull<c_void>,
}

impl ColorSpaceMenuHelper {
    /// Build a native menu. Upstream evaluates filtering and hierarchy once.
    pub fn create(parameters: &ColorSpaceMenuParameters) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_color_space_menu_helper_create(parameters.handle.as_ptr()) };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }

    /// Native menu num color spaces.
    pub fn num_color_spaces(&self) -> usize {
        unsafe { ocio_sys::ocio_color_space_menu_helper_get_num_color_spaces(self.handle.as_ptr()) }
    }

    /// Native menu name. Returns an empty string if not found.
    pub fn name(&self, index: usize) -> &CStr {
        unsafe {
            text(ocio_sys::ocio_color_space_menu_helper_get_name(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Native menu ui name. Returns an empty string if not found.
    pub fn ui_name(&self, index: usize) -> &CStr {
        unsafe {
            text(ocio_sys::ocio_color_space_menu_helper_get_ui_name(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Native menu index from name.
    pub fn index_from_name(&self, name: &CStr) -> Option<usize> {
        let index = unsafe {
            ocio_sys::ocio_color_space_menu_helper_get_index_from_name(
                self.handle.as_ptr(),
                name.as_ptr(),
            )
        };
        (index != usize::MAX).then_some(index)
    }

    /// Native menu index from ui name.
    pub fn index_from_ui_name(&self, name: &CStr) -> Option<usize> {
        let index = unsafe {
            ocio_sys::ocio_color_space_menu_helper_get_index_from_ui_name(
                self.handle.as_ptr(),
                name.as_ptr(),
            )
        };
        (index != usize::MAX).then_some(index)
    }

    /// Native menu description. Returns an empty string if not found.
    pub fn description(&self, index: usize) -> &CStr {
        unsafe {
            text(ocio_sys::ocio_color_space_menu_helper_get_description(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Native menu family. Returns an empty string if not found.
    pub fn family(&self, index: usize) -> &CStr {
        unsafe {
            text(ocio_sys::ocio_color_space_menu_helper_get_family(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Native menu num hierarchy levels.
    pub fn num_hierarchy_levels(&self, index: usize) -> usize {
        unsafe {
            ocio_sys::ocio_color_space_menu_helper_get_num_hierarchy_levels(
                self.handle.as_ptr(),
                index,
            )
        }
    }

    /// Native menu hierarchy level. Returns an empty string if not found.
    pub fn hierarchy_level(&self, index: usize, level: usize) -> &CStr {
        unsafe {
            text(ocio_sys::ocio_color_space_menu_helper_get_hierarchy_level(
                self.handle.as_ptr(),
                index,
                level,
            ))
        }
    }

    /// Native menu name from ui name. Returns an empty string if not found.
    pub fn name_from_ui_name(&self, name: &CStr) -> &CStr {
        unsafe {
            text(
                ocio_sys::ocio_color_space_menu_helper_get_name_from_ui_name(
                    self.handle.as_ptr(),
                    name.as_ptr(),
                ),
            )
        }
    }

    /// Native menu ui name from name. Returns an empty string if not found.
    pub fn ui_name_from_name(&self, name: &CStr) -> &CStr {
        unsafe {
            text(
                ocio_sys::ocio_color_space_menu_helper_get_ui_name_from_name(
                    self.handle.as_ptr(),
                    name.as_ptr(),
                ),
            )
        }
    }
}

impl Drop for ColorSpaceMenuHelper {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_color_space_menu_helper_destroy(self.handle.as_ptr()) }
    }
}

/// Native mixing space and encoding selection.
pub struct MixingColorSpaceManager {
    handle: NonNull<c_void>,
}

impl MixingColorSpaceManager {
    /// Create using the native implementation.
    pub fn create(config: &Config) -> Result<Self> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_mixing_color_space_manager_create(config.handle.as_ptr()) };
        Ok(Self {
            handle: crate::handle_result(value)?,
        })
    }

    /// Num mixing spaces using the native implementation.
    pub fn num_mixing_spaces(&self) -> usize {
        unsafe {
            ocio_sys::ocio_mixing_color_space_manager_get_num_mixing_spaces(self.handle.as_ptr())
        }
    }

    /// Mixing space ui name using the native implementation.
    pub fn mixing_space_ui_name(&self, index: usize) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_mixing_color_space_manager_get_mixing_space_ui_name(
                self.handle.as_ptr(),
                index,
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Selected mixing space idx using the native implementation.
    pub fn selected_mixing_space_idx(&self) -> usize {
        unsafe {
            ocio_sys::ocio_mixing_color_space_manager_get_selected_mixing_space_idx(
                self.handle.as_ptr(),
            )
        }
    }

    /// Set selected mixing space idx using the native implementation.
    pub fn set_selected_mixing_space_idx(&mut self, index: usize) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_mixing_color_space_manager_set_selected_mixing_space_idx(
                self.handle.as_ptr(),
                index,
            )
        };
        crate::ocio_call_status()
    }

    /// Set selected mixing space using the native implementation.
    pub fn set_selected_mixing_space(&mut self, name: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_mixing_color_space_manager_set_selected_mixing_space(
                self.handle.as_ptr(),
                name.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Is perceptually uniform using the native implementation.
    pub fn is_perceptually_uniform(&self) -> bool {
        unsafe {
            ocio_sys::ocio_mixing_color_space_manager_is_perceptually_uniform(self.handle.as_ptr())
        }
    }

    /// Num mixing encodings using the native implementation.
    pub fn num_mixing_encodings(&self) -> usize {
        unsafe {
            ocio_sys::ocio_mixing_color_space_manager_get_num_mixing_encodings(self.handle.as_ptr())
        }
    }

    /// Mixing encoding name using the native implementation.
    pub fn mixing_encoding_name(&self, index: usize) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_mixing_color_space_manager_get_mixing_encoding_name(
                self.handle.as_ptr(),
                index,
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { text(value) })
    }

    /// Selected mixing encoding idx using the native implementation.
    pub fn selected_mixing_encoding_idx(&self) -> usize {
        unsafe {
            ocio_sys::ocio_mixing_color_space_manager_get_selected_mixing_encoding_idx(
                self.handle.as_ptr(),
            )
        }
    }

    /// Set selected mixing encoding idx using the native implementation.
    pub fn set_selected_mixing_encoding_idx(&mut self, index: usize) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_mixing_color_space_manager_set_selected_mixing_encoding_idx(
                self.handle.as_ptr(),
                index,
            )
        };
        crate::ocio_call_status()
    }

    /// Set selected mixing encoding using the native implementation.
    pub fn set_selected_mixing_encoding(&mut self, name: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_mixing_color_space_manager_set_selected_mixing_encoding(
                self.handle.as_ptr(),
                name.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Refresh using the native implementation.
    pub fn refresh(&mut self, config: &Config) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_mixing_color_space_manager_refresh(
                self.handle.as_ptr(),
                config.handle.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Processor using the native implementation.
    pub fn processor(
        &self,
        working: &CStr,
        display: &CStr,
        view: &CStr,
        direction: TransformDirection,
    ) -> Result<Processor> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_mixing_color_space_manager_get_processor(
                self.handle.as_ptr(),
                working.as_ptr(),
                display.as_ptr(),
                view.as_ptr(),
                direction as i32,
            )
        };
        Ok(Processor {
            handle: crate::handle_result(value)?,
        })
    }

    /// Slider using the native implementation.
    pub fn slider(&mut self) -> MixingSlider<'_> {
        MixingSlider {
            handle: NonNull::new(unsafe {
                ocio_sys::ocio_mixing_color_space_manager_get_slider(self.handle.as_ptr())
            })
            .expect("native manager owns a slider"),
            owner: std::marker::PhantomData,
        }
    }

    /// Slider with edges using the native implementation.
    pub fn slider_with_edges(&mut self, min: f32, max: f32) -> MixingSlider<'_> {
        MixingSlider {
            handle: NonNull::new(unsafe {
                ocio_sys::ocio_mixing_color_space_manager_get_slider_with_edges(
                    self.handle.as_ptr(),
                    min,
                    max,
                )
            })
            .expect("native manager owns a slider"),
            owner: std::marker::PhantomData,
        }
    }
}

impl Drop for MixingColorSpaceManager {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_mixing_color_space_manager_destroy(self.handle.as_ptr()) }
    }
}

/// An exclusive borrow of the native manager's slider, without an allocation.
pub struct MixingSlider<'a> {
    handle: NonNull<c_void>,
    owner: std::marker::PhantomData<&'a mut MixingColorSpaceManager>,
}

impl MixingSlider<'_> {
    /// Set slider min edge using the native implementation.
    pub fn set_slider_min_edge(&mut self, value: f32) {
        unsafe { ocio_sys::ocio_mixing_slider_set_slider_min_edge(self.handle.as_ptr(), value) }
    }

    /// Set slider max edge using the native implementation.
    pub fn set_slider_max_edge(&mut self, value: f32) {
        unsafe { ocio_sys::ocio_mixing_slider_set_slider_max_edge(self.handle.as_ptr(), value) }
    }

    /// Slider min edge using the native implementation.
    pub fn slider_min_edge(&self) -> f32 {
        unsafe { ocio_sys::ocio_mixing_slider_get_slider_min_edge(self.handle.as_ptr()) }
    }

    /// Slider max edge using the native implementation.
    pub fn slider_max_edge(&self) -> f32 {
        unsafe { ocio_sys::ocio_mixing_slider_get_slider_max_edge(self.handle.as_ptr()) }
    }

    /// Slider to mixing using the native implementation.
    pub fn slider_to_mixing(&self, value: f32) -> f32 {
        unsafe { ocio_sys::ocio_mixing_slider_slider_to_mixing(self.handle.as_ptr(), value) }
    }

    /// Mixing to slider using the native implementation.
    pub fn mixing_to_slider(&self, value: f32) -> f32 {
        unsafe { ocio_sys::ocio_mixing_slider_mixing_to_slider(self.handle.as_ptr(), value) }
    }
}
