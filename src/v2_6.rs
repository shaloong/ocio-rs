//! OpenColorIO 2.6 extensions to the existing safe API.
use std::ffi::{c_char, CStr};
use std::ptr::NonNull;

use crate::{ColorSpace, Config, Result};

/// Borrowed arguments for authoring a display or shared view with aliases.
pub struct ViewWithAliases<'a> {
    /// Native view argument.
    pub view: &'a CStr,
    /// Native view transform argument.
    pub view_transform: &'a CStr,
    /// Native color space argument.
    pub color_space: &'a CStr,
    /// Native looks argument.
    pub looks: &'a CStr,
    /// Native rule argument.
    pub rule: &'a CStr,
    /// Native description argument.
    pub description: &'a CStr,
    /// Alias names. Each alias is passed directly to the native authoring API.
    pub aliases: &'a [&'a CStr],
}

unsafe fn owned_text(value: *const c_char) -> Option<String> {
    if value.is_null() {
        return None;
    }
    let value = unsafe { CStr::from_ptr(value) };
    (!value.is_empty()).then(|| value.to_string_lossy().into_owned())
}

/// Native metadata element name for a CLF ID, absent in a stub build.
pub fn metadata_id_element() -> Option<&'static CStr> {
    let value = unsafe { ocio_sys::ocio_metadata_id_element() };
    // The native constant points at immutable static storage.
    (!value.is_null()).then(|| unsafe { CStr::from_ptr(value) })
}

impl Config {
    /// Canonical display name.
    pub fn canonical_display_name(&self, display: &CStr) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_canonical_display_name(self.handle.as_ptr(), display.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { owned_text(value) })
    }

    /// Canonical view name.
    pub fn canonical_view_name(&self, display: &CStr, view: &CStr) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_canonical_view_name(
                self.handle.as_ptr(),
                display.as_ptr(),
                view.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { owned_text(value) })
    }

    /// Resolved display view color space name.
    pub fn resolved_display_view_color_space_name(
        &self,
        display: &CStr,
        view: &CStr,
    ) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_resolved_display_view_color_space_name(
                self.handle.as_ptr(),
                display.as_ptr(),
                view.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { owned_text(value) })
    }

    /// Display description.
    pub fn display_description(&self, display: &CStr) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_display_description(self.handle.as_ptr(), display.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(unsafe { owned_text(value) })
    }

    /// Display view alias.
    pub fn display_view_alias(
        &self,
        display: &CStr,
        view: &CStr,
        index: i32,
    ) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_display_view_alias(
                self.handle.as_ptr(),
                display.as_ptr(),
                view.as_ptr(),
                index,
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { owned_text(value) })
    }

    /// Generate an ASWF local Color Interop ID using the native config.
    pub fn generate_local_id_for_color_space(&self, name: &CStr) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_generate_local_id_for_color_space(
                self.handle.as_ptr(),
                name.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { owned_text(value) })
    }

    /// Find an equivalent color space in a built-in config. Reuse `builtin` to retain the native fingerprint cache.
    pub fn locate_builtin_color_space(
        &self,
        name: &CStr,
        builtin: &Config,
    ) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_locate_builtin_color_space(
                self.handle.as_ptr(),
                name.as_ptr(),
                builtin.handle.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { owned_text(value) })
    }

    /// Num display view aliases.
    pub fn num_display_view_aliases(&self, display: &CStr, view: &CStr) -> Result<i32> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_num_display_view_aliases(
                self.handle.as_ptr(),
                display.as_ptr(),
                view.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Has display view alias.
    pub fn has_display_view_alias(
        &self,
        display: &CStr,
        view: &CStr,
        alias: &CStr,
    ) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_has_display_view_alias(
                self.handle.as_ptr(),
                display.as_ptr(),
                view.as_ptr(),
                alias.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Use display aliases.
    pub fn use_display_aliases(&self) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_config_use_display_aliases(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set use display aliases.
    pub fn set_use_display_aliases(&self, enabled: bool) -> Result<()> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_config_set_use_display_aliases(self.handle.as_ptr(), enabled) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Check the native HDR display compatibility requirement defined by OCIO 2.6.
    pub fn check_hdr_display_compatibility(&self) -> Result<bool> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_config_check_hdr_display_compatibility(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Look up a Color Interop ID and return an independent editable copy of the match.
    pub fn find_color_space_for_id(&self, id: &CStr) -> Result<Option<ColorSpace>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_find_color_space_for_id(self.handle.as_ptr(), id.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(value).map(|handle| ColorSpace { handle }))
    }

    /// Add a shared view and its aliases in one native call.
    pub fn add_shared_view_with_aliases(&self, view: ViewWithAliases<'_>) -> Result<()> {
        let aliases: Vec<_> = view.aliases.iter().map(|alias| alias.as_ptr()).collect();
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_shared_view_with_aliases(
                self.handle.as_ptr(),
                view.view.as_ptr(),
                view.view_transform.as_ptr(),
                view.color_space.as_ptr(),
                view.looks.as_ptr(),
                view.rule.as_ptr(),
                view.description.as_ptr(),
                aliases.as_ptr(),
                aliases.len(),
            )
        };
        crate::ocio_call_status()
    }

    /// Add a display view and its aliases in one native call.
    pub fn add_display_view_with_aliases(
        &self,
        display: &CStr,
        view: ViewWithAliases<'_>,
    ) -> Result<()> {
        let aliases: Vec<_> = view.aliases.iter().map(|alias| alias.as_ptr()).collect();
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_display_view_with_aliases(
                self.handle.as_ptr(),
                display.as_ptr(),
                view.view.as_ptr(),
                view.view_transform.as_ptr(),
                view.color_space.as_ptr(),
                view.looks.as_ptr(),
                view.rule.as_ptr(),
                view.description.as_ptr(),
                aliases.as_ptr(),
                aliases.len(),
            )
        };
        crate::ocio_call_status()
    }
}
