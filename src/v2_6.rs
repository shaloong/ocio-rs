//! OpenColorIO 2.6 extensions to the existing safe API.
use std::ffi::{c_char, CStr};
use std::ptr::NonNull;

use crate::{ColorSpace, Config, ConfigRef, Result};

fn locate_builtin_color_space_refs(
    source: ConfigRef<'_>,
    name: &CStr,
    builtin: ConfigRef<'_>,
) -> Result<Option<String>> {
    let source_copy = source.isolated_copy()?;
    let builtin_copy = builtin.isolated_copy()?;
    let source = source_copy.as_ref().map_or(source, ConfigRef::Editable);
    let builtin = builtin_copy.as_ref().map_or(builtin, ConfigRef::Editable);
    let (source, source_const) = source.raw();
    let (builtin, builtin_const) = builtin.raw();
    crate::clear_last_error();
    let value = unsafe {
        ocio_sys::ocio_config_locate_builtin_color_space_refs(
            source,
            source_const,
            name.as_ptr(),
            builtin,
            builtin_const,
        )
    };
    crate::ocio_call_status()?;
    Ok(unsafe { owned_text(value) })
}

impl crate::ConstConfig {
    fn v26_display_text(
        &self,
        field: i32,
        display: &CStr,
        view: &CStr,
        index: i32,
    ) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_const_config_v26_display_text(
                self.handle.as_ptr(),
                field,
                display.as_ptr(),
                view.as_ptr(),
                index,
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { owned_text(value) })
    }

    fn v26_display_number(
        &self,
        field: i32,
        display: &CStr,
        view: &CStr,
        alias: &CStr,
    ) -> Result<i32> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_const_config_v26_display_number(
                self.handle.as_ptr(),
                field,
                display.as_ptr(),
                view.as_ptr(),
                alias.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Resolve a display name or enabled alias to its canonical name.
    pub fn canonical_display_name(&self, display: &CStr) -> Result<Option<String>> {
        self.v26_display_text(0, display, c"", 0)
    }
    /// Resolve a view name or alias to its canonical name.
    pub fn canonical_view_name(&self, display: &CStr, view: &CStr) -> Result<Option<String>> {
        self.v26_display_text(1, display, view, 0)
    }
    /// Return the resolved display-view color space, including alias handling.
    pub fn resolved_display_view_color_space_name(
        &self,
        display: &CStr,
        view: &CStr,
    ) -> Result<Option<String>> {
        self.v26_display_text(2, display, view, 0)
    }
    /// Return the native display description.
    pub fn display_description(&self, display: &CStr) -> Result<Option<String>> {
        self.v26_display_text(3, display, c"", 0)
    }
    /// Return a view alias at the given index.
    pub fn display_view_alias(
        &self,
        display: &CStr,
        view: &CStr,
        index: i32,
    ) -> Result<Option<String>> {
        self.v26_display_text(4, display, view, index)
    }
    /// Return the number of aliases for the view.
    pub fn num_display_view_aliases(&self, display: &CStr, view: &CStr) -> Result<i32> {
        self.v26_display_number(0, display, view, c"")
    }
    /// Return whether a view has a particular alias.
    pub fn has_display_view_alias(
        &self,
        display: &CStr,
        view: &CStr,
        alias: &CStr,
    ) -> Result<bool> {
        Ok(self.v26_display_number(1, display, view, alias)? != 0)
    }
    /// Return whether display color-space aliases are enabled.
    pub fn use_display_aliases(&self) -> Result<bool> {
        Ok(self.v26_display_number(2, c"", c"", c"")? != 0)
    }

    /// Check OCIO's 2.6 HDR-display compatibility requirement.
    pub fn check_hdr_display_compatibility(&self) -> Result<bool> {
        Ok(self.v26_display_number(3, c"", c"", c"")? != 0)
    }

    /// Find an equivalent space in a built-in config without copying either config.
    /// Reuse the built-in config to retain OCIO's fingerprint cache.
    /// Read-only configs are copied because OCIO's matching heuristics temporarily
    /// change their cache flags. For repeated matching, create editable copies once.
    pub fn locate_builtin_color_space<'a>(
        &self,
        name: &CStr,
        builtin: impl Into<ConfigRef<'a>>,
    ) -> Result<Option<String>> {
        locate_builtin_color_space_refs(self.into(), name, builtin.into())
    }
    /// Generate a local Color Interop ID without copying the configuration.
    ///
    /// The result is owned because OCIO constructs a new string. Missing
    /// color spaces and unsuitable config names are returned as native errors.
    pub fn generate_local_id_for_color_space(&self, name: &CStr) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_const_config_generate_local_id_for_color_space(
                self.handle.as_ptr(),
                name.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { owned_text(value) })
    }

    /// Look up a Color Interop ID without copying the immutable native match.
    ///
    /// The returned handle retains shared ownership and may outlive this config.
    /// OCIO searches color-space names and aliases, not `interop_id` attributes;
    /// it applies the native namespace and local-ID fallback rules.
    pub fn find_const_color_space_for_id(
        &self,
        id: &CStr,
    ) -> Result<Option<crate::ConstColorSpace>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_const_config_find_color_space_for_id(self.handle.as_ptr(), id.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(value).map(|handle| crate::ConstColorSpace { handle }))
    }
}

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

    /// Find an equivalent space using an editable or shared read-only built-in config.
    /// A read-only built-in config is copied to isolate OCIO's cache-flag changes.
    pub fn locate_builtin_color_space_ref<'a>(
        &self,
        name: &CStr,
        builtin: impl Into<ConfigRef<'a>>,
    ) -> Result<Option<String>> {
        locate_builtin_color_space_refs(self.into(), name, builtin.into())
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
        unsafe { ocio_sys::ocio_config_set_use_display_aliases(self.handle.as_ptr(), enabled) };
        crate::ocio_call_status()
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
    /// OCIO searches names and aliases, not `interop_id` attributes; namespace
    /// and local-ID fallbacks follow the native rules.
    pub fn find_color_space_for_id(&self, id: &CStr) -> Result<Option<ColorSpace>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_find_color_space_for_id(self.handle.as_ptr(), id.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(value).map(|handle| ColorSpace { handle }))
    }

    /// Look up a Color Interop ID and share OCIO's immutable matching object.
    ///
    /// The returned handle retains native shared ownership and may outlive this
    /// config. It avoids the editable deep copy made by
    /// [`Self::find_color_space_for_id`], but allocates an ownership handle.
    /// Matching uses names and aliases, not `interop_id` attributes.
    pub fn find_const_color_space_for_id(
        &self,
        id: &CStr,
    ) -> Result<Option<crate::ConstColorSpace>> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_config_find_const_color_space_for_id(self.handle.as_ptr(), id.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(value).map(|handle| crate::ConstColorSpace { handle }))
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
