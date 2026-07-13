use std::ffi::c_void;
use std::ptr::NonNull;

use crate::transform::TransformHandle;
use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, ColorSpace, ColorSpaceSet, ConfigIOProxy, Context,
    FileRules, Look, NamedTransform, OcioError, Processor, ReferenceSpaceType, Result,
    SearchReferenceSpaceType, TransformDirection, ViewTransform, ViewingRules,
};
use ocio_sys;

/// An OpenColorIO configuration.
///
/// `Config` is the central entry point for the crate. It holds color-space
/// definitions, display/view mappings, file rules, looks, roles, and the
/// metadata needed to build [`Processor`] pipelines.
///
/// A config can be loaded from a file, an environment variable, in-memory text,
/// or one of OCIO's built-in presets:
///
/// ```rust,no_run
/// # use ocio_rs::Config;
/// # fn example() -> ocio_rs::Result<()> {
/// // From a .ocio file
/// let config = Config::from_file("config.ocio")?;
///
/// // From the OCIO environment variable
/// let config = Config::from_env()?;
///
/// // From in-memory YAML text
/// let config = Config::from_stream("ociofile_version: 2")?;
///
/// // From a built-in preset
/// let config = Config::create_from_builtin_config("default")?;
/// # Ok(())
/// # }
/// ```
///
/// Once loaded, use the config to query color-space metadata, enumerate
/// displays and views, and create [`Processor`] instances for color-space
/// conversions or display-view pipelines.
///
/// [`Processor`]: crate::Processor
pub struct Config {
    pub(crate) handle: NonNull<c_void>,
}

impl Config {
    fn processor_handle_result(handle: *mut c_void) -> Result<Processor> {
        crate::handle_result(handle).map(|handle| Processor { handle })
    }

    /// Create a config from one of OCIO's built-in configuration presets.
    ///
    /// Use `BuiltinConfigRegistry` to enumerate the preset names exposed by the
    /// linked OCIO build.
    pub fn create_from_builtin_config(config_name: impl AsRef<str>) -> Result<Self> {
        let config_name = cstring(config_name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_create_from_builtin_config(config_name.as_ptr().cast())
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an empty editable config using OCIO defaults.
    pub fn raw() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_config_create_raw() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Load a config from an `.ocio` file on disk.
    pub fn from_file(path: impl AsRef<str>) -> Result<Self> {
        let path = cstring(path)?;
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_config_create_from_file(path.as_ptr().cast()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Load a config from the `OCIO` environment variable.
    ///
    /// In real OCIO mode this mirrors `OCIO::Config::CreateFromEnv`.
    pub fn from_env() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_config_create_from_env() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Parse a config from in-memory OCIO text.
    ///
    /// This is useful for tests, generated configs, or editor tooling that
    /// wants to validate config text before writing it to disk.
    pub fn from_stream(text: impl AsRef<str>) -> Result<Self> {
        let text = cstring(text)?;
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_config_create_from_stream(text.as_ptr().cast()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create a config from an in-memory `ConfigIOProxy`.
    pub fn from_config_io_proxy(proxy: &ConfigIOProxy) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_config_create_from_config_io_proxy(proxy.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    // --- Name & metadata ---

    /// Return the config name, if one has been authored.
    pub fn name(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_name(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    /// Set the config name used in serialized metadata.
    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let name = cstring(name)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_set_name(self.handle.as_ptr(), name.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Return the config description, if present.
    pub fn description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_description(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    /// Set the config description stored in serialized metadata.
    pub fn set_description(&self, desc: impl AsRef<str>) -> Result<()> {
        let desc = cstring(desc)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_description(self.handle.as_ptr(), desc.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Return OCIO's cache identifier for the config's current authored state.
    pub fn cache_id(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_cache_id(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    /// Return a cache identifier specialized for a concrete OCIO context.
    pub fn cache_id_for_context(&self, context: &Context) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_cache_id_n(
                self.handle.as_ptr(),
                context.handle.as_ptr(),
            ))
        }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer cache_id_for_context()")]
    pub fn cache_id_with_context(&self, context: &Context) -> Option<String> {
        self.cache_id_for_context(context)
    }

    // --- Version ---

    /// Return the major version of this config profile.
    pub fn major_version(&self) -> u32 {
        unsafe { ocio_sys::ocio_config_get_major_version(self.handle.as_ptr()) as u32 }
    }

    /// Return the minor version of this config profile.
    pub fn minor_version(&self) -> u32 {
        unsafe { ocio_sys::ocio_config_get_minor_version(self.handle.as_ptr()) as u32 }
    }

    /// Set the authored config version.
    ///
    /// OCIO rejects unsupported major/minor combinations. For example, asking
    /// for a minor version that does not exist for the current major returns an
    /// error instead of silently clamping.
    pub fn set_version(&self, major: u32, minor: u32) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_set_version(self.handle.as_ptr(), major, minor) };
        crate::ocio_call_status()
    }

    /// Upgrade this config to the newest OCIO profile version it supports.
    pub fn upgrade_to_latest_version(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_upgrade_to_latest_version(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    /// Return the character used to separate family hierarchy levels in color-space names.
    pub fn family_separator(&self) -> char {
        let sep = unsafe {
            ocio_sys::ocio_config_get_family_separator(self.handle.as_ptr() as *mut c_void)
        };
        sep as u8 as char
    }

    // --- Color spaces ---

    /// Return the total number of color spaces registered in this config.
    pub fn num_color_spaces(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_num_color_spaces_v1(self.handle.as_ptr() as *mut c_void)
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_color_spaces()")]
    pub fn get_num_color_spaces_v1(&self) -> i32 {
        self.num_color_spaces()
    }

    /// Return the name of the color space at the given index, or `None` if out of range.
    pub fn color_space_name_by_index(&self, index: i32) -> Option<String> {
        self.try_color_space_name_by_index(index).ok().flatten()
    }

    /// Return a color-space name by index, preserving bridge failures.
    pub fn try_color_space_name_by_index(&self, index: i32) -> Result<Option<String>> {
        crate::clear_last_error();
        let name = unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_color_space_name_by_index_v1(
                self.handle.as_ptr(),
                index,
            ))
        };
        crate::ocio_call_status()?;
        Ok(name)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer color_space_name_by_index()"
    )]
    pub fn get_color_space_name_by_index_v1(&self, index: i32) -> Option<String> {
        self.color_space_name_by_index(index)
    }

    /// Return a comma-separated list of all color-space names in this config.
    pub fn color_spaces(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_color_spaces(
                self.handle.as_ptr(),
                std::ptr::null(),
            ))
        }
    }

    /// Return the canonical name for the given color-space name, or `None` if not found.
    pub fn canonical_name(&self, name: impl AsRef<str>) -> Option<String> {
        let name = cstring(name).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_canonical_name(
                self.handle.as_ptr(),
                name.as_ptr().cast(),
            ))
        }
    }

    /// Test whether a named color space is linear with respect to a reference space type.
    pub fn is_color_space_linear(
        &self,
        color_space: impl AsRef<str>,
        reference: ReferenceSpaceType,
    ) -> bool {
        let cs = match cstring(color_space) {
            Ok(cs) => cs,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_config_is_color_space_linear(
                self.handle.as_ptr(),
                cs.as_ptr().cast(),
                reference as i32,
            )
        }
    }

    /// # Safety
    /// `src_config` and `builtin_config` must be valid OCIO config pointers for the active ABI.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO config-pointer entry point; prefer built-in config and color-space APIs unless you must interoperate with external OCIO ABI objects"
    )]
    pub unsafe fn identify_builtin_color_space(
        &self,
        src_config: *mut c_void,
        builtin_config: *mut c_void,
        builtin_color_space_name: impl AsRef<str>,
    ) -> Option<String> {
        let builtin_color_space_name = cstring(builtin_color_space_name).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_identify_builtin_color_space(
                self.handle.as_ptr(),
                src_config,
                builtin_config,
                builtin_color_space_name.as_ptr().cast(),
            ))
        }
    }

    /// # Safety
    /// `src_interchange_name`, `builtin_interchange_name`, `src_config`, and `builtin_config`
    /// must be valid pointers accepted by the OCIO ABI.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO interchange/config pointer entry point; prefer higher-level config APIs unless you must interoperate with external OCIO ABI objects"
    )]
    pub unsafe fn identify_interchange_space(
        &self,
        src_interchange_name: *mut c_void,
        builtin_interchange_name: *mut c_void,
        src_config: *mut c_void,
        src_color_space_name: impl AsRef<str>,
        builtin_config: *mut c_void,
        builtin_color_space_name: impl AsRef<str>,
    ) {
        let src_color_space_name = match cstring(src_color_space_name) {
            Ok(v) => v,
            Err(_) => return,
        };
        let builtin_color_space_name = match cstring(builtin_color_space_name) {
            Ok(v) => v,
            Err(_) => return,
        };
        unsafe {
            ocio_sys::ocio_config_identify_interchange_space(
                self.handle.as_ptr(),
                src_interchange_name,
                builtin_interchange_name,
                src_config,
                src_color_space_name.as_ptr().cast(),
                builtin_config,
                builtin_color_space_name.as_ptr().cast(),
            );
        }
    }

    /// Guess the color space for a file path using the config's file-rule patterns.
    pub fn color_space_from_filepath(&self, file_path: impl AsRef<str>) -> Option<String> {
        let fp = cstring(file_path).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_color_space_from_filepath(
                self.handle.as_ptr(),
                fp.as_ptr().cast(),
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer color_space_from_filepath_with_rule_index() or color_space_from_filepath()"
    )]
    pub fn get_color_space_from_filepath_by_ref_type(
        &self,
        file_path: impl AsRef<str>,
    ) -> Option<String> {
        self.color_space_from_filepath_with_rule_index(file_path)
            .map(|(color_space, _rule_index)| color_space)
    }

    /// Parse a color-space name from a free-form text string (e.g. a file path or description).
    pub fn parse_color_space_from_string(&self, text: impl AsRef<str>) -> Option<String> {
        let text = cstring(text).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_parse_color_space_from_string(
                self.handle.as_ptr(),
                text.as_ptr().cast(),
            ))
        }
    }

    // --- Displays ---

    /// Return the default display name, or `None` if no displays are configured.
    pub fn default_display(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_default_display(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; OpenColorIO does not expose a setDefaultDisplay() mutator, prefer set_active_displays()"
    )]
    pub fn set_default_display(&self, display: impl AsRef<str>) -> Result<()> {
        self.set_active_displays(display)
    }

    /// Return the total number of displays registered in this config.
    pub fn num_displays(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_displays(self.handle.as_ptr() as *mut c_void) }
    }

    /// Return the display name at the given index, or `None` if out of range.
    pub fn display(&self, index: i32) -> Option<String> {
        self.try_display(index).ok().flatten()
    }

    /// Return a display name by index, preserving bridge failures.
    pub fn try_display(&self, index: i32) -> Result<Option<String>> {
        crate::clear_last_error();
        let display = unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_display(
                self.handle.as_ptr(),
                index,
            ))
        };
        crate::ocio_call_status()?;
        Ok(display)
    }

    // --- Views ---

    /// Return the default view name for the given display, or `None` if not found.
    pub fn default_view(&self, display: impl AsRef<str>) -> Option<String> {
        let display = cstring(display).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_default_view(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
            ))
        }
    }

    /// Return the default view for a display, filtered by color-space name.
    pub fn default_view_with_color_space(
        &self,
        display: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
    ) -> Option<String> {
        let display = cstring(display).ok()?;
        let color_space_name = cstring(color_space_name).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_default_view_v1(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                color_space_name.as_ptr().cast(),
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat overload; prefer default_view_with_color_space()"
    )]
    pub fn get_default_view_v1(
        &self,
        display: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
    ) -> Option<String> {
        self.default_view_with_color_space(display, color_space_name)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; OpenColorIO does not expose a setDefaultView() mutator, prefer set_active_views()"
    )]
    pub fn set_default_view(&self, view: impl AsRef<str>) -> Result<()> {
        self.set_active_views(view)
    }

    /// Return the number of views registered for the given display.
    pub fn num_views(&self, display: impl AsRef<str>) -> i32 {
        let display = match cstring(display) {
            Ok(d) => d,
            Err(_) => return 0,
        };
        unsafe {
            ocio_sys::ocio_config_get_num_views(self.handle.as_ptr(), display.as_ptr().cast())
        }
    }

    /// Return the number of views for a display, filtered by color-space name.
    pub fn num_views_with_color_space(
        &self,
        display: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
    ) -> i32 {
        let display = match cstring(display) {
            Ok(d) => d,
            Err(_) => return 0,
        };
        let color_space_name = match cstring(color_space_name) {
            Ok(v) => v,
            Err(_) => return 0,
        };
        unsafe {
            ocio_sys::ocio_config_get_num_views_v1(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                color_space_name.as_ptr().cast(),
            )
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat overload; prefer num_views_with_color_space()"
    )]
    pub fn get_num_views_v1(
        &self,
        display: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
    ) -> i32 {
        self.num_views_with_color_space(display, color_space_name)
    }

    /// Return the view name at the given index for the specified display.
    pub fn view(&self, display: impl AsRef<str>, index: i32) -> Option<String> {
        self.try_view(display, index).ok().flatten()
    }

    /// Return a view name by display and index, preserving bridge failures.
    pub fn try_view(&self, display: impl AsRef<str>, index: i32) -> Result<Option<String>> {
        let display = cstring(display)?;
        crate::clear_last_error();
        let view = unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_view(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                index,
            ))
        };
        crate::ocio_call_status()?;
        Ok(view)
    }

    /// Return the view name at a given index for a display, filtered by color-space name.
    pub fn view_with_color_space(
        &self,
        display: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
        index: i32,
    ) -> Option<String> {
        self.try_view_with_color_space(display, color_space_name, index)
            .ok()
            .flatten()
    }

    /// Return the view name at a given index for a display and color-space name.
    ///
    /// Unlike [`Self::view_with_color_space`], this preserves invalid input and
    /// OCIO query failures as [`OcioError`].
    pub fn try_view_with_color_space(
        &self,
        display: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
        index: i32,
    ) -> Result<Option<String>> {
        let display = cstring(display)?;
        let color_space_name = cstring(color_space_name)?;
        crate::clear_last_error();
        let view = unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_view_v1(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                color_space_name.as_ptr().cast(),
                index,
            ))
        };
        crate::ocio_call_status()?;
        Ok(view)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat overload; prefer view_with_color_space()"
    )]
    pub fn get_view_v1(
        &self,
        display: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
        index: i32,
    ) -> Option<String> {
        self.view_with_color_space(display, color_space_name, index)
    }

    /// Return whether a given view is a shared view for a specific display.
    pub fn is_view_shared(&self, display: impl AsRef<str>, view: impl AsRef<str>) -> bool {
        let display = match cstring(display) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let view = match cstring(view) {
            Ok(v) => v,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_config_is_view_shared(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
            )
        }
    }

    /// # Safety
    /// `first` and `second` must be valid pointers to OCIO view descriptors for the active ABI.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO view-descriptor entry point; prefer higher-level display/view APIs unless you must interoperate with external OCIO ABI objects"
    )]
    pub unsafe fn are_views_equal(
        &self,
        first: *mut c_void,
        second: *mut c_void,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
    ) -> bool {
        let display = match cstring(display) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let view = match cstring(view) {
            Ok(v) => v,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_config_are_views_equal(
                self.handle.as_ptr(),
                first,
                second,
                display.as_ptr().cast(),
                view.as_ptr().cast(),
            )
        }
    }

    // --- Looks ---

    /// Return the total number of looks registered in the config.
    pub fn num_looks(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_looks(self.handle.as_ptr() as *mut c_void) }
    }

    /// Return the look name at a given index, or `None` if out of range.
    pub fn look_name_by_index(&self, index: i32) -> Option<String> {
        self.try_look_name_by_index(index).ok().flatten()
    }

    /// Return a look name by index, preserving bridge failures.
    pub fn try_look_name_by_index(&self, index: i32) -> Result<Option<String>> {
        crate::clear_last_error();
        let name = unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_look_name_by_index(
                self.handle.as_ptr(),
                index,
            ))
        };
        crate::ocio_call_status()?;
        Ok(name)
    }

    /// Return a comma-separated string of all look names.
    pub fn looks(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_look(
                self.handle.as_ptr(),
                std::ptr::null(),
            ))
        }
    }

    // --- Luma coefficients ---

    /// Return the default luminance coefficients used by display transforms.
    pub fn default_luma_coefs(&self) -> Result<[f64; 3]> {
        let mut coefs = [0.0f64; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_get_default_luma_coefs(
                self.handle.as_ptr(),
                coefs.as_mut_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()?;
        Ok(coefs)
    }

    /// Set the default luminance coefficients used by display transforms.
    pub fn set_default_luma_coefs(&self, coefs: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_default_luma_coefs(
                self.handle.as_ptr(),
                coefs.as_ptr() as *mut c_void,
            )
        };
        crate::ocio_call_status()
    }

    // --- Roles ---

    /// Return the total number of roles defined in the config.
    pub fn num_roles(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_roles(self.handle.as_ptr() as *mut c_void) }
    }

    /// Return whether a role with the given name exists.
    pub fn has_role(&self, role: impl AsRef<str>) -> bool {
        let role = match cstring(role) {
            Ok(r) => r,
            Err(_) => return false,
        };
        unsafe { ocio_sys::ocio_config_has_role(self.handle.as_ptr(), role.as_ptr().cast()) }
    }

    /// Return the role name at a given index.
    pub fn role_name(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_role_name(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Return the color-space name bound to the role at a given index.
    pub fn role_color_space_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_role_color_space_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Look up the color space currently bound to a role name.
    pub fn role_color_space(&self, role_name: impl AsRef<str>) -> Option<String> {
        let role = cstring(role_name).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_role_color_space_by_name(
                self.handle.as_ptr(),
                role.as_ptr().cast(),
            ))
        }
    }

    // --- Active displays / views ---

    /// Return the comma-separated string of active display names.
    pub fn active_displays(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_active_displays(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Return the comma-separated string of active view names.
    pub fn active_views(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_active_views(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    /// Replace the active displays list from a comma-separated string.
    pub fn set_active_displays(&self, displays: impl AsRef<str>) -> Result<()> {
        let d = cstring(displays)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_active_displays(self.handle.as_ptr(), d.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Replace the active views list from a comma-separated string.
    pub fn set_active_views(&self, views: impl AsRef<str>) -> Result<()> {
        let v = cstring(views)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_set_active_views(self.handle.as_ptr(), v.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    // --- Display/view transform name queries ---

    /// Return the display-view transform name for the given display/view pair.
    pub fn display_view_transform_name(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
    ) -> Option<String> {
        let display = cstring(display).ok()?;
        let view = cstring(view).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_display_view_transform_name(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
            ))
        }
    }

    /// Return the color-space name associated with the given display/view pair.
    pub fn display_view_color_space_name(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
    ) -> Option<String> {
        let display = cstring(display).ok()?;
        let view = cstring(view).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_display_view_color_space_name(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
            ))
        }
    }

    /// Return the looks string associated with a display/view pair.
    pub fn display_view_looks(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
    ) -> Option<String> {
        let d = cstring(display).ok()?;
        let v = cstring(view).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_display_view_looks(
                self.handle.as_ptr(),
                d.as_ptr().cast(),
                v.as_ptr().cast(),
            ))
        }
    }

    /// Return the rule name associated with a display/view pair.
    pub fn display_view_rule(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
    ) -> Option<String> {
        let display = cstring(display).ok()?;
        let view = cstring(view).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_display_view_rule(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer display_view_rule()")]
    pub fn get_display_view_rule(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
    ) -> Option<String> {
        self.display_view_rule(display, view)
    }

    /// Return the description string for a display/view pair.
    pub fn display_view_description(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
    ) -> Option<String> {
        let display = cstring(display).ok()?;
        let view = cstring(view).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_display_view_description(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer display_view_description()"
    )]
    pub fn get_display_view_description(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
    ) -> Option<String> {
        self.display_view_description(display, view)
    }

    /// Return whether the given view exists for the specified display.
    pub fn has_view(&self, display: impl AsRef<str>, view: impl AsRef<str>) -> bool {
        let display = match cstring(display) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let view = match cstring(view) {
            Ok(v) => v,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_config_has_view(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
            )
        }
    }

    /// Return the default scene-to-display view-transform object.
    pub fn default_scene_to_display_view_transform(&self) -> Option<crate::ViewTransform> {
        let handle = unsafe {
            ocio_sys::ocio_config_get_default_scene_to_display_view_transform(
                self.handle.as_ptr() as *mut c_void
            )
        };
        NonNull::new(handle).map(|h| crate::ViewTransform { handle: h })
    }

    /// Return the name of the default view transform.
    pub fn default_view_transform_name(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_default_view_transform_name(
                self.handle.as_ptr(),
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer default_view_transform_name()"
    )]
    pub fn get_default_view_transform_name(&self) -> Option<String> {
        self.default_view_transform_name()
    }

    /// Set the name of the default view transform.
    pub fn set_default_view_transform_name(&self, default_name: impl AsRef<str>) -> Result<()> {
        let default_name = cstring(default_name)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_default_view_transform_name(
                self.handle.as_ptr(),
                default_name.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    // --- Processors ---

    /// Create a processor between two authored color-space names.
    pub fn processor(&self, src: impl AsRef<str>, dst: impl AsRef<str>) -> Result<Processor> {
        let src = cstring(src)?;
        let dst = cstring(dst)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v2(
                self.handle.as_ptr(),
                src.as_ptr().cast(),
                dst.as_ptr().cast(),
            )
        };
        Self::processor_handle_result(handle)
    }

    /// Create a processor between two `ColorSpace` object handles.
    pub fn processor_from_color_spaces(
        &self,
        src_color_space: &ColorSpace,
        dst_color_space: &ColorSpace,
    ) -> Result<Processor> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v1(
                self.handle.as_ptr(),
                src_color_space.handle.as_ptr(),
                dst_color_space.handle.as_ptr(),
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_color_spaces()"
    )]
    pub fn get_processor_v1(
        &self,
        src_color_space: &ColorSpace,
        dst_color_space: &ColorSpace,
    ) -> Result<Processor> {
        self.processor_from_color_spaces(src_color_space, dst_color_space)
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer processor()")]
    pub fn get_processor_v2(
        &self,
        src: impl AsRef<str>,
        dst: impl AsRef<str>,
    ) -> Result<Processor> {
        self.processor(src, dst)
    }

    /// Create a processor that resolves through a display/view pair.
    pub fn processor_display(
        &self,
        src: impl AsRef<str>,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        let src = cstring(src)?;
        let display = cstring(display)?;
        let view = cstring(view)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v4(
                self.handle.as_ptr(),
                src.as_ptr().cast(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer processor_display()")]
    pub fn get_processor_v4(
        &self,
        src: impl AsRef<str>,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        self.processor_display(src, display, view, direction)
    }

    /// Create a processor from a transform with an explicit direction.
    pub fn processor_from_transform(
        &self,
        transform: &impl TransformHandle,
        direction: TransformDirection,
    ) -> Result<Processor> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v11(
                self.handle.as_ptr(),
                transform.as_ptr(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    /// Create a processor from a transform using OCIO's default transform direction.
    pub fn processor_from_transform_default_direction(
        &self,
        transform: &impl TransformHandle,
    ) -> Result<Processor> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v10(self.handle.as_ptr(), transform.as_ptr())
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_transform_default_direction()"
    )]
    pub fn get_processor_v10(&self, transform: &impl TransformHandle) -> Result<Processor> {
        self.processor_from_transform_default_direction(transform)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_transform()"
    )]
    pub fn get_processor_v11(
        &self,
        transform: &impl TransformHandle,
        direction: TransformDirection,
    ) -> Result<Processor> {
        self.processor_from_transform(transform, direction)
    }

    /// Create a processor between two color spaces using an explicit context for lookups.
    pub fn processor_with_context(
        &self,
        src: impl AsRef<str>,
        dst: impl AsRef<str>,
        context: &crate::Context,
    ) -> Result<Processor> {
        let src = cstring(src)?;
        let dst = cstring(dst)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v3(
                self.handle.as_ptr(),
                context.handle.as_ptr(),
                src.as_ptr().cast(),
                dst.as_ptr().cast(),
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_with_context()"
    )]
    pub fn get_processor_v3(
        &self,
        src: impl AsRef<str>,
        dst: impl AsRef<str>,
        context: &crate::Context,
    ) -> Result<Processor> {
        self.processor_with_context(src, dst, context)
    }

    /// Create a processor that resolves through a display/view pair with an explicit context.
    pub fn processor_display_with_context(
        &self,
        src: impl AsRef<str>,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
        direction: TransformDirection,
        context: &crate::Context,
    ) -> Result<Processor> {
        let src = cstring(src)?;
        let display = cstring(display)?;
        let view = cstring(view)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v5(
                self.handle.as_ptr(),
                context.handle.as_ptr(),
                src.as_ptr().cast(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_display_with_context()"
    )]
    pub fn get_processor_v5(
        &self,
        src: impl AsRef<str>,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
        direction: TransformDirection,
        context: &crate::Context,
    ) -> Result<Processor> {
        self.processor_display_with_context(src, display, view, direction, context)
    }

    /// Create a processor from a transform and an explicit context.
    pub fn processor_from_transform_with_context(
        &self,
        context: &crate::Context,
        transform: &impl TransformHandle,
        direction: TransformDirection,
    ) -> Result<Processor> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v12(
                self.handle.as_ptr(),
                context.handle.as_ptr(),
                transform.as_ptr(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_transform_with_context()"
    )]
    pub fn get_processor_v12(
        &self,
        context: &crate::Context,
        transform: &impl TransformHandle,
        direction: TransformDirection,
    ) -> Result<Processor> {
        self.processor_from_transform_with_context(context, transform, direction)
    }

    /// Create a processor from a named-transform object.
    pub fn processor_named_transform(
        &self,
        named_transform: &NamedTransform,
        direction: TransformDirection,
    ) -> Result<Processor> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v6(
                self.handle.as_ptr(),
                named_transform.handle.as_ptr(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_named_transform()"
    )]
    pub fn get_processor_v6(
        &self,
        named_transform: &NamedTransform,
        direction: TransformDirection,
    ) -> Result<Processor> {
        self.processor_named_transform(named_transform, direction)
    }

    /// Create a processor from a named-transform object with an explicit context.
    pub fn processor_named_transform_with_context(
        &self,
        context: &crate::Context,
        named_transform: &NamedTransform,
        direction: TransformDirection,
    ) -> Result<Processor> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v7(
                self.handle.as_ptr(),
                context.handle.as_ptr(),
                named_transform.handle.as_ptr(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_named_transform_with_context()"
    )]
    pub fn get_processor_v7(
        &self,
        context: &crate::Context,
        named_transform: &NamedTransform,
        direction: TransformDirection,
    ) -> Result<Processor> {
        self.processor_named_transform_with_context(context, named_transform, direction)
    }

    /// Create a processor from a named-transform name.
    pub fn processor_named_transform_name(
        &self,
        named_transform_name: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        let named_transform_name = cstring(named_transform_name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v8(
                self.handle.as_ptr(),
                named_transform_name.as_ptr().cast(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_named_transform_name()"
    )]
    pub fn get_processor_v8(
        &self,
        named_transform_name: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        self.processor_named_transform_name(named_transform_name, direction)
    }

    /// Create a processor from a named-transform name with an explicit context.
    pub fn processor_named_transform_name_with_context(
        &self,
        context: &crate::Context,
        named_transform_name: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        let named_transform_name = cstring(named_transform_name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v9(
                self.handle.as_ptr(),
                context.handle.as_ptr(),
                named_transform_name.as_ptr().cast(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_named_transform_name_with_context()"
    )]
    pub fn get_processor_v9(
        &self,
        context: &crate::Context,
        named_transform_name: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        self.processor_named_transform_name_with_context(context, named_transform_name, direction)
    }

    /// Create a processor from a source config color space to a built-in color space.
    pub fn processor_to_builtin_color_space(
        &self,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        builtin_color_space_name: impl AsRef<str>,
    ) -> Result<Processor> {
        let src_color_space_name = cstring(src_color_space_name)?;
        let builtin_color_space_name = cstring(builtin_color_space_name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_to_builtin_color_space(
                self.handle.as_ptr(),
                src_config.handle.as_ptr(),
                src_color_space_name.as_ptr().cast(),
                builtin_color_space_name.as_ptr().cast(),
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_to_builtin_color_space()"
    )]
    pub fn get_processor_to_builtin_color_space(
        &self,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        builtin_color_space_name: impl AsRef<str>,
    ) -> Result<Processor> {
        self.processor_to_builtin_color_space(
            src_config,
            src_color_space_name,
            builtin_color_space_name,
        )
    }

    /// Create a processor from a built-in color space to a source config color space.
    pub fn processor_from_builtin_color_space(
        &self,
        builtin_color_space_name: impl AsRef<str>,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
    ) -> Result<Processor> {
        let builtin_color_space_name = cstring(builtin_color_space_name)?;
        let src_color_space_name = cstring(src_color_space_name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_from_builtin_color_space(
                self.handle.as_ptr(),
                builtin_color_space_name.as_ptr().cast(),
                src_config.handle.as_ptr(),
                src_color_space_name.as_ptr().cast(),
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_builtin_color_space()"
    )]
    pub fn get_processor_from_builtin_color_space(
        &self,
        builtin_color_space_name: impl AsRef<str>,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
    ) -> Result<Processor> {
        self.processor_from_builtin_color_space(
            builtin_color_space_name,
            src_config,
            src_color_space_name,
        )
    }

    /// Create a processor between two color spaces in two different configs.
    pub fn processor_from_configs(
        src_config: &Config,
        src_name: impl AsRef<str>,
        dst_config: &Config,
        dst_name: impl AsRef<str>,
    ) -> Result<Processor> {
        let src_name = cstring(src_name)?;
        let dst_name = cstring(dst_name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_from_configs(
                src_config.handle.as_ptr(),
                src_config.handle.as_ptr(),
                src_name.as_ptr().cast(),
                dst_config.handle.as_ptr(),
                dst_name.as_ptr().cast(),
            )
        };
        Self::processor_handle_result(handle)
    }

    /// Create a processor between two configs with explicit context objects.
    pub fn processor_from_configs_with_contexts(
        &self,
        src_context: &Context,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        dst_context: &Context,
        dst_config: &Config,
        dst_color_space_name: impl AsRef<str>,
    ) -> Result<Processor> {
        let src_color_space_name = cstring(src_color_space_name)?;
        let dst_color_space_name = cstring(dst_color_space_name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_from_configs_v1(
                self.handle.as_ptr(),
                src_context.handle.as_ptr(),
                src_config.handle.as_ptr(),
                src_color_space_name.as_ptr().cast(),
                dst_context.handle.as_ptr(),
                dst_config.handle.as_ptr(),
                dst_color_space_name.as_ptr().cast(),
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_configs_with_contexts()"
    )]
    pub fn get_processor_from_configs_v1(
        &self,
        src_context: &Context,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        dst_context: &Context,
        dst_config: &Config,
        dst_color_space_name: impl AsRef<str>,
    ) -> Result<Processor> {
        self.processor_from_configs_with_contexts(
            src_context,
            src_config,
            src_color_space_name,
            dst_context,
            dst_config,
            dst_color_space_name,
        )
    }

    /// Create a processor between two configs using interchange color spaces for matching.
    pub fn processor_from_configs_with_interchange(
        &self,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        src_interchange_name: impl AsRef<str>,
        dst_config: &Config,
        dst_color_space_name: impl AsRef<str>,
        dst_interchange_name: impl AsRef<str>,
    ) -> Result<Processor> {
        let src_color_space_name = cstring(src_color_space_name)?;
        let src_interchange_name = cstring(src_interchange_name)?;
        let dst_color_space_name = cstring(dst_color_space_name)?;
        let dst_interchange_name = cstring(dst_interchange_name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_from_configs_v2(
                self.handle.as_ptr(),
                src_config.handle.as_ptr(),
                src_color_space_name.as_ptr().cast(),
                src_interchange_name.as_ptr().cast(),
                dst_config.handle.as_ptr(),
                dst_color_space_name.as_ptr().cast(),
                dst_interchange_name.as_ptr().cast(),
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_configs_with_interchange()"
    )]
    pub fn get_processor_from_configs_v2(
        &self,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        src_interchange_name: impl AsRef<str>,
        dst_config: &Config,
        dst_color_space_name: impl AsRef<str>,
        dst_interchange_name: impl AsRef<str>,
    ) -> Result<Processor> {
        self.processor_from_configs_with_interchange(
            src_config,
            src_color_space_name,
            src_interchange_name,
            dst_config,
            dst_color_space_name,
            dst_interchange_name,
        )
    }

    /// Create a processor between two configs with explicit contexts and interchange spaces.
    #[allow(clippy::too_many_arguments)]
    pub fn processor_from_configs_with_contexts_and_interchange(
        &self,
        src_context: &Context,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        src_interchange_name: impl AsRef<str>,
        dst_context: &Context,
        dst_config: &Config,
        dst_color_space_name: impl AsRef<str>,
        dst_interchange_name: impl AsRef<str>,
    ) -> Result<Processor> {
        let src_color_space_name = cstring(src_color_space_name)?;
        let src_interchange_name = cstring(src_interchange_name)?;
        let dst_color_space_name = cstring(dst_color_space_name)?;
        let dst_interchange_name = cstring(dst_interchange_name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_from_configs_v3(
                self.handle.as_ptr(),
                src_context.handle.as_ptr(),
                src_config.handle.as_ptr(),
                src_color_space_name.as_ptr().cast(),
                src_interchange_name.as_ptr().cast(),
                dst_context.handle.as_ptr(),
                dst_config.handle.as_ptr(),
                dst_color_space_name.as_ptr().cast(),
                dst_interchange_name.as_ptr().cast(),
            )
        };
        Self::processor_handle_result(handle)
    }

    #[allow(clippy::too_many_arguments)]
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_configs_with_contexts_and_interchange()"
    )]
    pub fn get_processor_from_configs_v3(
        &self,
        src_context: &Context,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        src_interchange_name: impl AsRef<str>,
        dst_context: &Context,
        dst_config: &Config,
        dst_color_space_name: impl AsRef<str>,
        dst_interchange_name: impl AsRef<str>,
    ) -> Result<Processor> {
        self.processor_from_configs_with_contexts_and_interchange(
            src_context,
            src_config,
            src_color_space_name,
            src_interchange_name,
            dst_context,
            dst_config,
            dst_color_space_name,
            dst_interchange_name,
        )
    }

    /// Create a processor from a source color space to a destination display/view pair across configs.
    pub fn processor_from_configs_to_display(
        &self,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        dst_config: &Config,
        dst_display: impl AsRef<str>,
        dst_view: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        let src_color_space_name = cstring(src_color_space_name)?;
        let dst_display = cstring(dst_display)?;
        let dst_view = cstring(dst_view)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_from_configs_v4(
                self.handle.as_ptr(),
                src_config.handle.as_ptr(),
                src_color_space_name.as_ptr().cast(),
                dst_config.handle.as_ptr(),
                dst_display.as_ptr().cast(),
                dst_view.as_ptr().cast(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_configs_to_display()"
    )]
    pub fn get_processor_from_configs_v4(
        &self,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        dst_config: &Config,
        dst_display: impl AsRef<str>,
        dst_view: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        self.processor_from_configs_to_display(
            src_config,
            src_color_space_name,
            dst_config,
            dst_display,
            dst_view,
            direction,
        )
    }

    /// Create a processor to a display/view pair with explicit context objects for both configs.
    #[allow(clippy::too_many_arguments)]
    pub fn processor_from_configs_to_display_with_contexts(
        &self,
        src_context: &Context,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        dst_context: &Context,
        dst_config: &Config,
        dst_display: impl AsRef<str>,
        dst_view: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        let src_color_space_name = cstring(src_color_space_name)?;
        let dst_display = cstring(dst_display)?;
        let dst_view = cstring(dst_view)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_from_configs_v5(
                self.handle.as_ptr(),
                src_context.handle.as_ptr(),
                src_config.handle.as_ptr(),
                src_color_space_name.as_ptr().cast(),
                dst_context.handle.as_ptr(),
                dst_config.handle.as_ptr(),
                dst_display.as_ptr().cast(),
                dst_view.as_ptr().cast(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    #[allow(clippy::too_many_arguments)]
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_configs_to_display_with_contexts()"
    )]
    pub fn get_processor_from_configs_v5(
        &self,
        src_context: &Context,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        dst_context: &Context,
        dst_config: &Config,
        dst_display: impl AsRef<str>,
        dst_view: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        self.processor_from_configs_to_display_with_contexts(
            src_context,
            src_config,
            src_color_space_name,
            dst_context,
            dst_config,
            dst_display,
            dst_view,
            direction,
        )
    }

    /// Create a processor to a display/view pair with interchange spaces for matching.
    #[allow(clippy::too_many_arguments)]
    pub fn processor_from_configs_to_display_with_interchange(
        &self,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        src_interchange_name: impl AsRef<str>,
        dst_config: &Config,
        dst_display: impl AsRef<str>,
        dst_view: impl AsRef<str>,
        dst_interchange_name: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        let src_color_space_name = cstring(src_color_space_name)?;
        let src_interchange_name = cstring(src_interchange_name)?;
        let dst_display = cstring(dst_display)?;
        let dst_view = cstring(dst_view)?;
        let dst_interchange_name = cstring(dst_interchange_name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_from_configs_v6(
                self.handle.as_ptr(),
                src_config.handle.as_ptr(),
                src_color_space_name.as_ptr().cast(),
                src_interchange_name.as_ptr().cast(),
                dst_config.handle.as_ptr(),
                dst_display.as_ptr().cast(),
                dst_view.as_ptr().cast(),
                dst_interchange_name.as_ptr().cast(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    #[allow(clippy::too_many_arguments)]
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_configs_to_display_with_interchange()"
    )]
    pub fn get_processor_from_configs_v6(
        &self,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        src_interchange_name: impl AsRef<str>,
        dst_config: &Config,
        dst_display: impl AsRef<str>,
        dst_view: impl AsRef<str>,
        dst_interchange_name: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        self.processor_from_configs_to_display_with_interchange(
            src_config,
            src_color_space_name,
            src_interchange_name,
            dst_config,
            dst_display,
            dst_view,
            dst_interchange_name,
            direction,
        )
    }

    /// Create a processor to a display/view pair with contexts and interchange spaces.
    #[allow(clippy::too_many_arguments)]
    pub fn processor_from_configs_to_display_with_contexts_and_interchange(
        &self,
        src_context: &Context,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        src_interchange_name: impl AsRef<str>,
        dst_context: &Context,
        dst_config: &Config,
        dst_display: impl AsRef<str>,
        dst_view: impl AsRef<str>,
        dst_interchange_name: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        let src_color_space_name = cstring(src_color_space_name)?;
        let src_interchange_name = cstring(src_interchange_name)?;
        let dst_display = cstring(dst_display)?;
        let dst_view = cstring(dst_view)?;
        let dst_interchange_name = cstring(dst_interchange_name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_from_configs_v7(
                self.handle.as_ptr(),
                src_context.handle.as_ptr(),
                src_config.handle.as_ptr(),
                src_color_space_name.as_ptr().cast(),
                src_interchange_name.as_ptr().cast(),
                dst_context.handle.as_ptr(),
                dst_config.handle.as_ptr(),
                dst_display.as_ptr().cast(),
                dst_view.as_ptr().cast(),
                dst_interchange_name.as_ptr().cast(),
                direction as i32,
            )
        };
        Self::processor_handle_result(handle)
    }

    #[allow(clippy::too_many_arguments)]
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_configs_to_display_with_contexts_and_interchange()"
    )]
    pub fn get_processor_from_configs_v7(
        &self,
        src_context: &Context,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        src_interchange_name: impl AsRef<str>,
        dst_context: &Context,
        dst_config: &Config,
        dst_display: impl AsRef<str>,
        dst_view: impl AsRef<str>,
        dst_interchange_name: impl AsRef<str>,
        direction: TransformDirection,
    ) -> Result<Processor> {
        self.processor_from_configs_to_display_with_contexts_and_interchange(
            src_context,
            src_config,
            src_color_space_name,
            src_interchange_name,
            dst_context,
            dst_config,
            dst_display,
            dst_view,
            dst_interchange_name,
            direction,
        )
    }

    /// Look up a color space by authored name.
    pub fn color_space(&self, name: impl AsRef<str>) -> Option<ColorSpace> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_color_space(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| ColorSpace { handle: h })
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer color_space()")]
    pub fn get_color_space(&self, name: impl AsRef<str>) -> Option<ColorSpace> {
        self.color_space(name)
    }

    /// Look up a color space by name, filtered by reference space type (scene, display, or all).
    pub fn color_space_by_ref_type(
        &self,
        name: impl AsRef<str>,
        ref_type: SearchReferenceSpaceType,
    ) -> Option<ColorSpace> {
        let color_space = self.color_space(name)?;
        match ref_type {
            SearchReferenceSpaceType::All => Some(color_space),
            SearchReferenceSpaceType::Scene
                if color_space.reference_space_type() == crate::ReferenceSpaceType::Scene =>
            {
                Some(color_space)
            }
            SearchReferenceSpaceType::Display
                if color_space.reference_space_type() == crate::ReferenceSpaceType::Display =>
            {
                Some(color_space)
            }
            _ => None,
        }
    }

    /// Infer a color space from a file path and also return the matched rule index.
    pub fn color_space_from_filepath_with_rule_index(
        &self,
        path: impl AsRef<str>,
    ) -> Option<(String, usize)> {
        let path = cstring(path).ok()?;
        let mut rule_index = 0usize;
        let color_space = unsafe {
            cstr_from_mut(
                ocio_sys::ocio_config_get_color_space_from_filepath_with_rule_index(
                    self.handle.as_ptr(),
                    path.as_ptr().cast(),
                    &mut rule_index,
                ),
            )
        }?;
        Some((color_space, rule_index))
    }

    /// Return the index of the named color space in the config, or -1 if not found.
    pub fn color_space_index(&self, name: impl AsRef<str>) -> i32 {
        let n = cstring(name);
        match n {
            Ok(n) => unsafe {
                ocio_sys::ocio_config_get_index_for_color_space(
                    self.handle.as_ptr(),
                    n.as_ptr().cast(),
                )
            },
            Err(_) => -1,
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer color_space_index()")]
    pub fn get_index_for_color_space(&self, name: impl AsRef<str>) -> i32 {
        self.color_space_index(name)
    }

    /// Add a color space to the config; panics on failure.
    pub fn add_color_space(&self, cs: &ColorSpace) {
        self.try_add_color_space(cs)
            .expect("failed to add color space");
    }

    /// Add a color space to the config, returning an error on failure.
    pub fn try_add_color_space(&self, cs: &ColorSpace) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_color_space(
                self.handle.as_ptr(),
                cs.handle.as_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    /// Remove a named color space from the config.
    pub fn remove_color_space(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_remove_color_space(self.handle.as_ptr(), n.as_ptr().cast());
        }
        crate::ocio_call_status()
    }

    /// Return whether a named color space is referenced by any other part of the config.
    pub fn is_color_space_used(&self, name: impl AsRef<str>) -> bool {
        let n = cstring(name);
        match n {
            Ok(n) => unsafe {
                ocio_sys::ocio_config_is_color_space_used(self.handle.as_ptr(), n.as_ptr().cast())
            },
            Err(_) => false,
        }
    }

    /// Look up a `Look` by its authored name.
    pub fn look(&self, name: impl AsRef<str>) -> Option<Look> {
        let n = cstring(name).ok()?;
        let handle =
            unsafe { ocio_sys::ocio_config_get_look(self.handle.as_ptr(), n.as_ptr().cast()) };
        NonNull::new(handle).map(|h| Look { handle: h })
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer look()")]
    pub fn get_look(&self, name: impl AsRef<str>) -> Option<Look> {
        self.look(name)
    }

    pub fn add_look(&self, look: &Look) {
        self.try_add_look(look).expect("failed to add look");
    }

    pub fn try_add_look(&self, look: &Look) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_look(
                self.handle.as_ptr(),
                look.handle.as_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    // --- Clear collections ---

    pub fn clear_color_spaces(&self) {
        self.try_clear_color_spaces()
            .expect("failed to clear color spaces");
    }

    /// Clear all color spaces and surface any OCIO validation error.
    pub fn try_clear_color_spaces(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_clear_color_spaces(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()
    }

    pub fn clear_looks(&self) {
        self.try_clear_looks().expect("failed to clear looks");
    }

    /// Clear all looks and surface any OCIO validation error.
    pub fn try_clear_looks(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_clear_looks(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()
    }

    pub fn clear_named_transforms(&self) {
        self.try_clear_named_transforms()
            .expect("failed to clear named transforms");
    }

    /// Clear all named transforms and surface any OCIO validation error.
    pub fn try_clear_named_transforms(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_clear_named_transforms(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()
    }

    pub fn clear_view_transforms(&self) {
        self.try_clear_view_transforms()
            .expect("failed to clear view transforms");
    }

    /// Clear all view transforms and surface any OCIO validation error.
    pub fn try_clear_view_transforms(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_clear_view_transforms(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()
    }

    // --- Display/view management ---

    pub fn add_display(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
        transform_name: impl AsRef<str>,
        rule: impl AsRef<str>,
    ) -> Result<()> {
        let display = cstring(display)?;
        let view = cstring(view)?;
        let transform_name = cstring(transform_name)?;
        let rule = cstring(rule)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_display_view_v1(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
                transform_name.as_ptr().cast(),
                rule.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer add_display() for basic display/view wiring"
    )]
    pub fn add_display_view_v1(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
        looks: impl AsRef<str>,
    ) -> Result<()> {
        let display = cstring(display)?;
        let view = cstring(view)?;
        let color_space_name = cstring(color_space_name)?;
        let looks = cstring(looks)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_display_view_v1(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
                color_space_name.as_ptr().cast(),
                looks.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_display_view_detailed(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
        view_transform_name: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
        looks: impl AsRef<str>,
        rule_name: impl AsRef<str>,
        description: impl AsRef<str>,
    ) -> Result<()> {
        let display = cstring(display)?;
        let view = cstring(view)?;
        let view_transform_name = cstring(view_transform_name)?;
        let color_space_name = cstring(color_space_name)?;
        let looks = cstring(looks)?;
        let rule_name = cstring(rule_name)?;
        let description = cstring(description)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_display_view_v2(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
                view_transform_name.as_ptr().cast(),
                color_space_name.as_ptr().cast(),
                looks.as_ptr().cast(),
                rule_name.as_ptr().cast(),
                description.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    #[allow(clippy::too_many_arguments)]
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer add_display_view_detailed()"
    )]
    pub fn add_display_view_v2(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
        view_transform_name: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
        looks: impl AsRef<str>,
        rule_name: impl AsRef<str>,
        description: impl AsRef<str>,
    ) -> Result<()> {
        self.add_display_view_detailed(
            display,
            view,
            view_transform_name,
            color_space_name,
            looks,
            rule_name,
            description,
        )
    }

    pub fn add_shared_view(
        &self,
        view: impl AsRef<str>,
        view_transform_name: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
        looks: impl AsRef<str>,
        rule_name: impl AsRef<str>,
        description: impl AsRef<str>,
    ) -> Result<()> {
        let view = cstring(view)?;
        let view_transform_name = cstring(view_transform_name)?;
        let color_space_name = cstring(color_space_name)?;
        let looks = cstring(looks)?;
        let rule_name = cstring(rule_name)?;
        let description = cstring(description)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_shared_view(
                self.handle.as_ptr(),
                view.as_ptr().cast(),
                view_transform_name.as_ptr().cast(),
                color_space_name.as_ptr().cast(),
                looks.as_ptr().cast(),
                rule_name.as_ptr().cast(),
                description.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn remove_shared_view(&self, view: impl AsRef<str>) -> Result<()> {
        let view = cstring(view)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_remove_shared_view(self.handle.as_ptr(), view.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    pub fn clear_shared_views(&self) {
        self.try_clear_shared_views()
            .expect("failed to clear shared views");
    }

    /// Clear every shared view and surface any OCIO validation error.
    pub fn try_clear_shared_views(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_clear_shared_views(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    pub fn remove_view(&self, display: impl AsRef<str>, view: impl AsRef<str>) -> Result<()> {
        let display = cstring(display)?;
        let view = cstring(view)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_remove_display_view(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn remove_display_view(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
    ) -> Result<()> {
        self.remove_view(display, view)
    }

    pub fn add_display_shared_view(
        &self,
        display: impl AsRef<str>,
        shared_view: impl AsRef<str>,
    ) -> Result<()> {
        let display = cstring(display)?;
        let shared_view = cstring(shared_view)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_display_shared_view(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                shared_view.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn clear_displays(&self) {
        self.try_clear_displays().expect("failed to clear displays");
    }

    /// Clear every display and surface any OCIO validation error.
    pub fn try_clear_displays(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_clear_displays(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    pub fn has_virtual_view(&self, view_name: impl AsRef<str>) -> bool {
        let view_name = match cstring(view_name) {
            Ok(v) => v,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_config_has_virtual_view(self.handle.as_ptr(), view_name.as_ptr().cast())
        }
    }

    pub fn is_virtual_view_shared(&self, view_name: impl AsRef<str>) -> bool {
        let view_name = match cstring(view_name) {
            Ok(v) => v,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_config_is_virtual_view_shared(
                self.handle.as_ptr(),
                view_name.as_ptr().cast(),
            )
        }
    }

    pub fn add_virtual_display_view(
        &self,
        view: impl AsRef<str>,
        view_transform_name: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
        looks: impl AsRef<str>,
        rule_name: impl AsRef<str>,
        description: impl AsRef<str>,
    ) -> Result<()> {
        let view = cstring(view)?;
        let view_transform_name = cstring(view_transform_name)?;
        let color_space_name = cstring(color_space_name)?;
        let looks = cstring(looks)?;
        let rule_name = cstring(rule_name)?;
        let description = cstring(description)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_virtual_display_view(
                self.handle.as_ptr(),
                view.as_ptr().cast(),
                view_transform_name.as_ptr().cast(),
                color_space_name.as_ptr().cast(),
                looks.as_ptr().cast(),
                rule_name.as_ptr().cast(),
                description.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn add_virtual_display_shared_view(&self, shared_view: impl AsRef<str>) -> Result<()> {
        let shared_view = cstring(shared_view)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_virtual_display_shared_view(
                self.handle.as_ptr(),
                shared_view.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn virtual_display_num_views(&self, reference_space: SearchReferenceSpaceType) -> i32 {
        self.try_virtual_display_num_views(reference_space)
            .unwrap_or(0)
    }

    /// Return the virtual-display view count, preserving bridge failures.
    pub fn try_virtual_display_num_views(
        &self,
        reference_space: SearchReferenceSpaceType,
    ) -> Result<i32> {
        crate::clear_last_error();
        let count = unsafe {
            ocio_sys::ocio_config_get_virtual_display_num_views(
                self.handle.as_ptr(),
                reference_space as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(count)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer virtual_display_num_views()"
    )]
    pub fn get_virtual_display_num_views(&self, reference_space: SearchReferenceSpaceType) -> i32 {
        self.virtual_display_num_views(reference_space)
    }

    pub fn virtual_display_view(
        &self,
        reference_space: SearchReferenceSpaceType,
        index: i32,
    ) -> Option<String> {
        self.try_virtual_display_view(reference_space, index)
            .ok()
            .flatten()
    }

    /// Return a virtual-display view by index, preserving bridge failures.
    pub fn try_virtual_display_view(
        &self,
        reference_space: SearchReferenceSpaceType,
        index: i32,
    ) -> Result<Option<String>> {
        crate::clear_last_error();
        let view = unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_virtual_display_view(
                self.handle.as_ptr(),
                reference_space as i32,
                index,
            ))
        };
        crate::ocio_call_status()?;
        Ok(view)
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer virtual_display_view()")]
    pub fn get_virtual_display_view(
        &self,
        reference_space: SearchReferenceSpaceType,
        index: i32,
    ) -> Option<String> {
        self.virtual_display_view(reference_space, index)
    }

    /// # Safety
    /// `first` and `second` must be valid pointers to OCIO virtual-view descriptors.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO virtual-view entry point; prefer higher-level virtual display APIs unless you must interoperate with external OCIO ABI objects"
    )]
    pub unsafe fn are_virtual_views_equal(
        &self,
        first: *mut c_void,
        second: *mut c_void,
        view_name: impl AsRef<str>,
    ) -> bool {
        let view_name = match cstring(view_name) {
            Ok(v) => v,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_config_are_virtual_views_equal(
                self.handle.as_ptr(),
                first,
                second,
                view_name.as_ptr().cast(),
            )
        }
    }

    pub fn virtual_display_view_transform_name(&self, view: impl AsRef<str>) -> Option<String> {
        let view = cstring(view).ok()?;
        unsafe {
            cstr_from_mut(
                ocio_sys::ocio_config_get_virtual_display_view_transform_name(
                    self.handle.as_ptr(),
                    view.as_ptr().cast(),
                ),
            )
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer virtual_display_view_transform_name()"
    )]
    pub fn get_virtual_display_view_transform_name(&self, view: impl AsRef<str>) -> Option<String> {
        self.virtual_display_view_transform_name(view)
    }

    pub fn virtual_display_view_color_space_name(&self, view: impl AsRef<str>) -> Option<String> {
        let view = cstring(view).ok()?;
        unsafe {
            cstr_from_mut(
                ocio_sys::ocio_config_get_virtual_display_view_color_space_name(
                    self.handle.as_ptr(),
                    view.as_ptr().cast(),
                ),
            )
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer virtual_display_view_color_space_name()"
    )]
    pub fn get_virtual_display_view_color_space_name(
        &self,
        view: impl AsRef<str>,
    ) -> Option<String> {
        self.virtual_display_view_color_space_name(view)
    }

    pub fn virtual_display_view_looks(&self, view: impl AsRef<str>) -> Option<String> {
        let view = cstring(view).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_virtual_display_view_looks(
                self.handle.as_ptr(),
                view.as_ptr().cast(),
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer virtual_display_view_looks()"
    )]
    pub fn get_virtual_display_view_looks(&self, view: impl AsRef<str>) -> Option<String> {
        self.virtual_display_view_looks(view)
    }

    pub fn virtual_display_view_rule(&self, view: impl AsRef<str>) -> Option<String> {
        let view = cstring(view).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_virtual_display_view_rule(
                self.handle.as_ptr(),
                view.as_ptr().cast(),
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer virtual_display_view_rule()"
    )]
    pub fn get_virtual_display_view_rule(&self, view: impl AsRef<str>) -> Option<String> {
        self.virtual_display_view_rule(view)
    }

    pub fn virtual_display_view_description(&self, view: impl AsRef<str>) -> Option<String> {
        let view = cstring(view).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_virtual_display_view_description(
                self.handle.as_ptr(),
                view.as_ptr().cast(),
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer virtual_display_view_description()"
    )]
    pub fn get_virtual_display_view_description(&self, view: impl AsRef<str>) -> Option<String> {
        self.virtual_display_view_description(view)
    }

    pub fn remove_virtual_display_view(&self, view: impl AsRef<str>) -> Result<()> {
        let view = cstring(view)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_remove_virtual_display_view(
                self.handle.as_ptr(),
                view.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn clear_virtual_display(&self) {
        self.try_clear_virtual_display()
            .expect("failed to clear virtual display");
    }

    /// Clear all virtual-display views and surface any OCIO validation error.
    pub fn try_clear_virtual_display(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_clear_virtual_display(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    pub fn instantiate_display_from_monitor_name(&self, monitor_name: impl AsRef<str>) -> i32 {
        let monitor_name = match cstring(monitor_name) {
            Ok(v) => v,
            Err(_) => return -1,
        };
        unsafe {
            ocio_sys::ocio_config_instantiate_display_from_monitor_name(
                self.handle.as_ptr(),
                monitor_name.as_ptr().cast(),
            )
        }
    }

    pub fn instantiate_display_from_icc_profile(
        &self,
        icc_profile_filepath: impl AsRef<str>,
    ) -> i32 {
        let icc_profile_filepath = match cstring(icc_profile_filepath) {
            Ok(v) => v,
            Err(_) => return -1,
        };
        unsafe {
            ocio_sys::ocio_config_instantiate_display_from_icc_profile(
                self.handle.as_ptr(),
                icc_profile_filepath.as_ptr().cast(),
            )
        }
    }

    // --- Named transforms ---

    /// Return the total number of named transforms in the config.
    pub fn num_named_transforms(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_num_named_transforms_v1(self.handle.as_ptr() as *mut c_void)
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_named_transforms()")]
    pub fn get_num_named_transforms_v1(&self) -> i32 {
        self.num_named_transforms()
    }

    /// Return the named-transform name at a given index.
    pub fn named_transform_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_named_transform_name_by_index_v1(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer named_transform_name_by_index()"
    )]
    pub fn get_named_transform_name_by_index_v1(&self, index: i32) -> Option<String> {
        self.named_transform_name_by_index(index)
    }

    /// Look up a `NamedTransform` by name.
    pub fn named_transform(&self, name: impl AsRef<str>) -> Option<NamedTransform> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_named_transform(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| NamedTransform { handle: h })
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer named_transform()")]
    pub fn get_named_transform(&self, name: impl AsRef<str>) -> Option<NamedTransform> {
        self.named_transform(name)
    }

    /// Return the index of a named transform, or -1 if not found.
    pub fn named_transform_index(&self, name: impl AsRef<str>) -> i32 {
        let name = match cstring(name) {
            Ok(v) => v,
            Err(_) => return -1,
        };
        unsafe {
            ocio_sys::ocio_config_get_index_for_named_transform(
                self.handle.as_ptr(),
                name.as_ptr().cast(),
            )
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer named_transform_index()")]
    pub fn get_index_for_named_transform(&self, name: impl AsRef<str>) -> i32 {
        self.named_transform_index(name)
    }

    /// Add a named transform to the config; panics on failure.
    pub fn add_named_transform(&self, named_transform: &NamedTransform) {
        self.try_add_named_transform(named_transform)
            .expect("failed to add named transform");
    }

    /// Add a named transform to the config, returning an error on failure.
    pub fn try_add_named_transform(&self, named_transform: &NamedTransform) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_named_transform(
                self.handle.as_ptr(),
                named_transform.handle.as_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    /// Remove a named transform by name.
    pub fn remove_named_transform(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_remove_named_transform(self.handle.as_ptr(), n.as_ptr().cast());
        }
        crate::ocio_call_status()
    }

    // --- View transforms ---

    /// Return the total number of view transforms in the config.
    pub fn num_view_transforms(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_num_view_transforms(self.handle.as_ptr() as *mut c_void)
        }
    }

    /// Return the view-transform name at a given index.
    pub fn view_transform_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_view_transform_name_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Look up a `ViewTransform` by name.
    pub fn view_transform(&self, name: impl AsRef<str>) -> Option<ViewTransform> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_view_transform(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| ViewTransform { handle: h })
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer view_transform()")]
    pub fn get_view_transform(&self, name: impl AsRef<str>) -> Option<ViewTransform> {
        self.view_transform(name)
    }

    /// Add a view transform to the config; panics on failure.
    pub fn add_view_transform(&self, view_transform: &ViewTransform) {
        self.try_add_view_transform(view_transform)
            .expect("failed to add view transform");
    }

    /// Add a view transform to the config, returning an error on failure.
    pub fn try_add_view_transform(&self, view_transform: &ViewTransform) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_view_transform(
                self.handle.as_ptr(),
                view_transform.handle.as_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    // --- Search paths ---

    /// Return the serialized search-path string used for file resolution.
    pub fn search_path(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_search_path(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    /// Replace the config search-path list from a serialized search-path string.
    pub fn set_search_path(&self, path: impl AsRef<str>) -> Result<()> {
        let p = cstring(path)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_set_search_path(self.handle.as_ptr(), p.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Return the number of individual search-path entries.
    pub fn num_search_paths(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_search_paths(self.handle.as_ptr() as *mut c_void) }
    }

    /// Return the search-path string at a given index.
    pub fn search_path_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_search_path_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Remove all configured search-path entries.
    #[deprecated(
        since = "0.2.0",
        note = "discarded OCIO errors; prefer try_clear_search_paths()"
    )]
    pub fn clear_search_paths(&self) {
        let _ = self.try_clear_search_paths();
    }

    /// Try to remove all configured search-path entries.
    pub fn try_clear_search_paths(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_clear_search_paths(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()
    }

    /// Append one search-path entry to the config.
    pub fn add_search_path(&self, path: impl AsRef<str>) -> Result<()> {
        let p = cstring(path)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_add_search_path(self.handle.as_ptr(), p.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    // --- Strict parsing ---

    pub fn is_strict_parsing_enabled(&self) -> bool {
        unsafe {
            ocio_sys::ocio_config_is_strict_parsing_enabled(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn set_strict_parsing_enabled(&self, enabled: bool) {
        self.try_set_strict_parsing_enabled(enabled)
            .expect("failed to set strict parsing enabled");
    }

    /// Enable or disable strict parsing and surface any OCIO validation error.
    pub fn try_set_strict_parsing_enabled(&self, enabled: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_set_strict_parsing_enabled(self.handle.as_ptr(), enabled) };
        crate::ocio_call_status()
    }

    // --- Roles (mutable) ---

    pub fn set_role(&self, role: impl AsRef<str>, color_space: impl AsRef<str>) -> Result<()> {
        let r = cstring(role)?;
        let cs = cstring(color_space)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_role(
                self.handle.as_ptr(),
                r.as_ptr().cast(),
                cs.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    // --- Family separator ---

    pub fn set_family_separator(&self, separator: char) {
        self.try_set_family_separator(separator)
            .expect("failed to set family separator");
    }

    /// Set the family separator character and surface any OCIO validation error.
    pub fn try_set_family_separator(&self, separator: char) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_family_separator(self.handle.as_ptr(), separator as i8);
        }
        crate::ocio_call_status()
    }

    // --- Validate ---

    /// Ask OCIO to validate the config in its current authored state.
    pub fn validate(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_validate(self.handle.as_ptr()) };
        crate::validation_status()
    }

    // --- Serialize ---

    /// Serialize the config to OCIO YAML text.
    ///
    /// Returns `Ok(None)` in stub builds where no real OCIO serializer is linked.
    pub fn serialize(&self) -> Result<Option<String>> {
        self.serialize_to_string()
    }

    /// Serialize the config to OCIO YAML text.
    ///
    /// Returns `Ok(None)` in stub builds where no real OCIO serializer is linked.
    ///
    /// Returns an error when OCIO cannot serialize this config.
    pub fn serialize_to_string(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let text = unsafe {
            cstr_from_mut(ocio_sys::ocio_config_serialize_to_string(
                self.handle.as_ptr(),
            ))
        };
        crate::ocio_call_status()?;
        Ok(text)
    }

    // --- Editable copy ---

    /// Create an editable clone of the config.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    // --- Context ---

    /// Return the current context associated with this config, if available.
    ///
    /// This compatibility helper returns `None` both when no context is
    /// available and when OCIO reports an error. Use [`Self::try_current_context`]
    /// when those cases must be distinguished.
    pub fn current_context(&self) -> Option<Context> {
        self.try_current_context().ok().flatten()
    }

    /// Try to get the current context associated with this config.
    ///
    /// The returned context owns an independent OCIO shared reference and may
    /// outlive this `Config` wrapper.
    pub fn try_current_context(&self) -> Result<Option<Context>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_current_context(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| Context { handle }))
    }

    // --- Clear all ---

    pub fn clear_all(&self) {
        self.try_clear_all()
            .expect("failed to clear all config collections");
    }

    /// Clear all config collections and surface any OCIO validation error.
    pub fn try_clear_all(&self) -> Result<()> {
        self.try_clear_color_spaces()?;
        self.try_clear_looks()?;
        self.try_clear_named_transforms()?;
        self.try_clear_view_transforms()?;
        self.try_clear_shared_views()?;
        self.try_clear_displays()?;
        self.try_clear_active_displays()?;
        self.try_clear_active_views()?;
        Ok(())
    }

    // --- Version setters ---

    /// Set the authored config major version.
    ///
    /// OCIO validates that the requested major version is supported and will
    /// update the minor version to the newest supported value for that major.
    pub fn set_major_version(&self, version: u32) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_set_major_version(self.handle.as_ptr(), version) };
        crate::ocio_call_status()
    }

    /// Set the authored config minor version for the current major version.
    ///
    /// OCIO rejects minor versions that are unsupported for the current major.
    pub fn set_minor_version(&self, version: u32) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_set_minor_version(self.handle.as_ptr(), version) };
        crate::ocio_call_status()
    }

    // --- Working directory ---

    /// Return the config's working directory.
    pub fn working_dir(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_working_dir(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    /// Set the config's working directory.
    pub fn set_working_dir(&self, dir_name: impl AsRef<str>) -> Result<()> {
        let d = cstring(dir_name)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_set_working_dir(self.handle.as_ptr(), d.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    // --- ColorSpaceSet ---

    /// Return a `ColorSpaceSet` filtered by an optional search string.
    pub fn color_space_set<S: AsRef<str>>(&self, search: Option<S>) -> Result<ColorSpaceSet> {
        crate::clear_last_error();
        let handle = match search {
            Some(ref s) => {
                let s = cstring(s.as_ref())?;
                unsafe {
                    ocio_sys::ocio_config_get_color_spaces(self.handle.as_ptr(), s.as_ptr().cast())
                }
            }
            None => unsafe {
                ocio_sys::ocio_config_get_color_spaces(self.handle.as_ptr(), std::ptr::null())
            },
        };
        crate::handle_result(handle).map(|handle| ColorSpaceSet { handle })
    }

    // --- FileRules ---

    /// Return the editable file-rules object attached to this config.
    pub fn file_rules(&self) -> Result<FileRules> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_config_get_file_rules(self.handle.as_ptr() as *mut c_void) };
        crate::handle_result(handle).map(|handle| FileRules { handle })
    }

    /// Attach a file-rules object to this config.
    pub fn set_file_rules(&self, file_rules: &FileRules) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_file_rules(
                self.handle.as_ptr(),
                file_rules.handle.as_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    // --- Environment mode ---

    /// Select whether OCIO imports only declared variables or the full process environment.
    pub fn set_environment_mode(&self, mode: crate::EnvironmentMode) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_environment_mode(self.handle.as_ptr(), mode as i32);
        }
        crate::ocio_call_status()
    }

    /// Return the current environment mode.
    pub fn environment_mode(&self) -> crate::EnvironmentMode {
        let m = unsafe {
            ocio_sys::ocio_config_get_environment_mode(self.handle.as_ptr() as *mut c_void)
        };
        match m {
            1 => crate::EnvironmentMode::LoadAll,
            _ => crate::EnvironmentMode::LoadPredefined,
        }
    }

    /// Refresh this config's context variables from the process environment.
    pub fn load_environment(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_load_environment(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()
    }

    // --- Inactive color spaces ---

    /// Return the comma-separated list of inactive color-space names.
    pub fn inactive_color_spaces(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_inactive_color_spaces(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Set the comma-separated list of inactive color-space names.
    pub fn set_inactive_color_spaces(&self, inactive: impl AsRef<str>) -> Result<()> {
        let s = cstring(inactive)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_inactive_color_spaces(
                self.handle.as_ptr(),
                s.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    /// Test whether a given color space name is in the inactive set.
    pub fn is_inactive_color_space(&self, color_space: impl AsRef<str>) -> bool {
        let color_space = match cstring(color_space) {
            Ok(v) => v,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_config_is_inactive_color_space(
                self.handle.as_ptr(),
                color_space.as_ptr().cast(),
            )
        }
    }

    // --- Archivable ---

    /// Return whether the config can be serialized as an OCIO archive.
    pub fn is_archivable(&self) -> bool {
        unsafe { ocio_sys::ocio_config_is_archivable(self.handle.as_ptr() as *mut c_void) }
    }

    // --- Processor cache ---

    #[deprecated(
        since = "0.2.0",
        note = "discarded OCIO errors; prefer try_clear_processor_cache()"
    )]
    pub fn clear_processor_cache(&self) {
        let _ = self.try_clear_processor_cache();
    }

    /// Try to invalidate this config's processor cache.
    pub fn try_clear_processor_cache(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_clear_processor_cache(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()
    }

    // --- v2.5.1: Environment variables ---

    /// Add an environment-variable declaration with a name and default value.
    pub fn add_environment_var(
        &self,
        name: impl AsRef<str>,
        default_val: impl AsRef<str>,
    ) -> Result<()> {
        let n = cstring(name)?;
        let v = cstring(default_val)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_environment_var(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
                v.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Return the number of environment-variable declarations.
    pub fn num_environment_vars(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_num_environment_vars(self.handle.as_ptr() as *mut c_void)
        }
    }

    /// Return the environment-variable name at a given index.
    pub fn environment_var_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_environment_var_name_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Return the default value for a named environment variable.
    pub fn environment_var_default(&self, name: impl AsRef<str>) -> Option<String> {
        let n = cstring(name).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_environment_var_default(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
            ))
        }
    }

    /// Remove every environment-variable declaration authored on this config.
    pub fn clear_environment_vars(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_clear_environment_vars(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()
    }

    // --- v2.5.1: Active display/view management ---

    /// Append a single display name to the active displays list.
    pub fn add_active_display(&self, display: impl AsRef<str>) -> Result<()> {
        let d = cstring(display)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_add_active_display(self.handle.as_ptr(), d.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Return the active display name at a given index.
    pub fn active_display(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_active_display(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer active_display()")]
    pub fn get_active_display(&self, index: i32) -> Option<String> {
        self.active_display(index)
    }

    /// Remove a single display name from the active displays list.
    pub fn remove_active_display(&self, display: impl AsRef<str>) -> Result<()> {
        let display = cstring(display)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_remove_active_display(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Append a single view name to the active views list.
    pub fn add_active_view(&self, view: impl AsRef<str>) -> Result<()> {
        let v = cstring(view)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_add_active_view(self.handle.as_ptr(), v.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Return the active view name at a given index.
    pub fn active_view(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_active_view(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer active_view()")]
    pub fn get_active_view(&self, index: i32) -> Option<String> {
        self.active_view(index)
    }

    /// Remove a single view name from the active views list.
    pub fn remove_active_view(&self, view: impl AsRef<str>) -> Result<()> {
        let view = cstring(view)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_remove_active_view(self.handle.as_ptr(), view.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    #[deprecated(
        since = "0.2.0",
        note = "discarded OCIO errors; prefer try_clear_active_displays()"
    )]
    pub fn clear_active_displays(&self) {
        let _ = self.try_clear_active_displays();
    }

    /// Try to clear all active display overrides.
    pub fn try_clear_active_displays(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_clear_active_displays(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()
    }

    #[deprecated(
        since = "0.2.0",
        note = "discarded OCIO errors; prefer try_clear_active_views()"
    )]
    pub fn clear_active_views(&self) {
        let _ = self.try_clear_active_views();
    }

    /// Try to clear all active view overrides.
    pub fn try_clear_active_views(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_clear_active_views(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()
    }

    /// Return the count of active displays.
    pub fn num_active_displays(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_num_active_displays(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn num_active_views(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_active_views(self.handle.as_ptr() as *mut c_void) }
    }

    /// Return the total number of all displays (including inactive).
    pub fn num_displays_all(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_displays_all(self.handle.as_ptr()) }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_displays_all()")]
    pub fn get_num_displays_all(&self) -> i32 {
        self.num_displays_all()
    }

    /// Return the display name at a given index from all displays (including inactive).
    pub fn display_all(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_display_all(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer display_all()")]
    pub fn get_display_all(&self, index: i32) -> Option<String> {
        self.display_all(index)
    }

    pub fn display_all_index(&self, display: impl AsRef<str>) -> i32 {
        let display = match cstring(display) {
            Ok(v) => v,
            Err(_) => return -1,
        };
        unsafe {
            ocio_sys::ocio_config_get_display_all_by_name(
                self.handle.as_ptr(),
                display.as_ptr() as *mut c_void,
            )
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer display_all_index()")]
    pub fn get_display_all_by_name(&self, display: impl AsRef<str>) -> i32 {
        self.display_all_index(display)
    }

    /// Return whether the display at a given index is marked as temporary.
    pub fn is_display_temporary(&self, index: i32) -> bool {
        unsafe { ocio_sys::ocio_config_is_display_temporary(self.handle.as_ptr(), index) }
    }

    /// Mark the display at `index` as temporary or persistent.
    ///
    /// Returns an error when `index` does not identify an existing display.
    pub fn set_display_temporary(&self, index: i32, temporary: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_display_temporary(self.handle.as_ptr(), index, temporary)
        };
        crate::ocio_call_status()
    }

    /// Return the number of views for a display, filtered by reference space type.
    pub fn num_views_by_reference_space(
        &self,
        reference_space: SearchReferenceSpaceType,
        display: impl AsRef<str>,
    ) -> i32 {
        let display = match cstring(display) {
            Ok(v) => v,
            Err(_) => return 0,
        };
        unsafe {
            ocio_sys::ocio_config_get_num_views_v2(
                self.handle.as_ptr(),
                reference_space as i32,
                display.as_ptr().cast(),
            )
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat overload; prefer num_views_by_reference_space()"
    )]
    pub fn get_num_views_v2(
        &self,
        reference_space: SearchReferenceSpaceType,
        display: impl AsRef<str>,
    ) -> i32 {
        self.num_views_by_reference_space(reference_space, display)
    }

    /// Return the view name at a given index for a display, filtered by reference space type.
    pub fn view_by_reference_space(
        &self,
        reference_space: SearchReferenceSpaceType,
        display: impl AsRef<str>,
        index: i32,
    ) -> Option<String> {
        self.try_view_by_reference_space(reference_space, display, index)
            .ok()
            .flatten()
    }

    /// Return the view name at a given index for a display and reference space type.
    ///
    /// Unlike [`Self::view_by_reference_space`], this preserves invalid input and
    /// OCIO query failures as [`OcioError`].
    pub fn try_view_by_reference_space(
        &self,
        reference_space: SearchReferenceSpaceType,
        display: impl AsRef<str>,
        index: i32,
    ) -> Result<Option<String>> {
        let display = cstring(display)?;
        crate::clear_last_error();
        let view = unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_view_v2(
                self.handle.as_ptr(),
                reference_space as i32,
                display.as_ptr().cast(),
                index,
            ))
        };
        crate::ocio_call_status()?;
        Ok(view)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat overload; prefer view_by_reference_space()"
    )]
    pub fn get_view_v2(
        &self,
        reference_space: SearchReferenceSpaceType,
        display: impl AsRef<str>,
        index: i32,
    ) -> Option<String> {
        self.view_by_reference_space(reference_space, display, index)
    }

    /// Return the editable viewing-rules object attached to this config, if any.
    pub fn viewing_rules(&self) -> Option<ViewingRules> {
        self.try_viewing_rules().ok().flatten()
    }

    /// Return the editable viewing-rules object attached to this config.
    ///
    /// `Ok(None)` means the config has no viewing-rules object. OCIO bridge
    /// failures are returned separately.
    pub fn try_viewing_rules(&self) -> Result<Option<ViewingRules>> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_config_get_viewing_rules(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| ViewingRules { handle }))
    }

    /// Attach a viewing-rules object to this config.
    pub fn set_viewing_rules_object(&self, viewing_rules: &ViewingRules) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_viewing_rules(
                self.handle.as_ptr(),
                viewing_rules.handle.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Return a borrowed raw OCIO viewing-rules handle.
    ///
    /// # Safety
    /// The pointer is owned by this config and must not be freed. It is valid
    /// only while this config remains alive and has not replaced its viewing
    /// rules. Prefer [`Self::viewing_rules`] for a typed wrapper.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO viewing-rules handle; prefer higher-level Config APIs where possible"
    )]
    pub unsafe fn get_viewing_rules(&self) -> *mut c_void {
        unsafe { ocio_sys::ocio_config_get_viewing_rules(self.handle.as_ptr()) }
    }

    /// # Safety
    /// `viewing_rules` must be a valid OCIO viewing-rules pointer for the
    /// active ABI and must remain valid for as long as OCIO retains it. Prefer
    /// [`Self::set_viewing_rules_object`] for a typed wrapper.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO viewing-rules handle; prefer higher-level Config APIs where possible"
    )]
    pub unsafe fn set_viewing_rules(&self, viewing_rules: *mut c_void) {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_set_viewing_rules(self.handle.as_ptr(), viewing_rules) };
        let _ = crate::ocio_call_status();
    }

    // --- v2.5.1: Misc utilities ---

    /// Archive the config to OCIO's textual archive representation.
    ///
    /// Returns `Ok(None)` in stub builds where no real OCIO archiver is linked.
    pub fn archive(&self) -> Result<Option<String>> {
        self.archive_to_string()
    }

    /// Archive the config to OCIO's textual archive representation.
    ///
    /// Returns `Ok(None)` in stub builds where no real OCIO archiver is linked.
    ///
    /// Returns an error when OCIO cannot archive this config.
    pub fn archive_to_string(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let text = unsafe {
            cstr_from_mut(ocio_sys::ocio_config_archive_to_string(
                self.handle.as_ptr(),
            ))
        };
        crate::ocio_call_status()?;
        Ok(text)
    }

    /// Return the attached typed config IO proxy when it originated from a Rust-managed proxy.
    pub fn config_io_proxy_object(&self) -> Option<ConfigIOProxy> {
        self.try_config_io_proxy_object().ok().flatten()
    }

    /// Return the attached typed config IO proxy, preserving bridge errors.
    ///
    /// `Ok(None)` means this config has no proxy attached.
    pub fn try_config_io_proxy_object(&self) -> Result<Option<ConfigIOProxy>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_get_config_io_proxy(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| ConfigIOProxy { handle }))
    }

    /// Attach a typed config IO proxy used to serve the config and LUT assets.
    pub fn set_config_io_proxy_object(&self, proxy: &ConfigIOProxy) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_config_io_proxy(self.handle.as_ptr(), proxy.handle.as_ptr())
        };
        crate::ocio_call_status()
    }

    /// Return the default family separator character for this OCIO version.
    pub fn default_family_separator(&self) -> char {
        unsafe {
            ocio_sys::ocio_config_get_default_family_separator(self.handle.as_ptr() as *mut c_void)
                as u8 as char
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO config-IO proxy handle; prefer file/path based Config APIs where possible"
    )]
    /// Returns a borrowed raw OCIO config-IO proxy handle.
    ///
    /// The pointer is owned by OCIO and must not be freed. It is only valid
    /// while this config remains alive and continues to reference the proxy.
    pub fn config_io_proxy(&self) -> *mut std::ffi::c_void {
        unsafe { ocio_sys::ocio_config_get_config_io_proxy(self.handle.as_ptr() as *mut c_void) }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer config_io_proxy()")]
    pub fn get_config_io_proxy(&self) -> *mut std::ffi::c_void {
        unsafe { ocio_sys::ocio_config_get_config_io_proxy(self.handle.as_ptr() as *mut c_void) }
    }

    /// # Safety
    /// The caller must pass a valid OCIO config-IO proxy pointer for the
    /// active ABI and keep it alive for as long as OCIO may use it. Prefer
    /// [`Self::set_config_io_proxy_object`] for a typed wrapper.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO config-IO proxy handle; prefer file/path based Config APIs where possible"
    )]
    pub unsafe fn set_config_io_proxy(&self, proxy: *mut std::ffi::c_void) {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_config_set_config_io_proxy(self.handle.as_ptr(), proxy) };
        let _ = crate::ocio_call_status();
    }

    /// Return whether a file path matches only the default file rule.
    pub fn filepath_only_matches_default_rule(&self, filepath: impl AsRef<str>) -> bool {
        let fp = match cstring(filepath) {
            Ok(f) => f,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_config_filepath_only_matches_default_rule(
                self.handle.as_ptr(),
                fp.as_ptr().cast(),
            )
        }
    }

    // --- v2.5.1: Processor cache flags ---

    /// Return the current processor-cache behavior flags.
    pub fn processor_cache_flags(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_processor_cache_flags(self.handle.as_ptr() as *mut c_void)
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "discarded OCIO errors; prefer try_set_processor_cache_flags()"
    )]
    pub fn set_processor_cache_flags(&self, flags: i32) {
        let _ = self.try_set_processor_cache_flags(flags);
    }

    /// Try to set OCIO's processor-cache behavior flags for this config.
    pub fn try_set_processor_cache_flags(&self, flags: i32) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_set_processor_cache_flags(
                self.handle.as_ptr() as *mut c_void,
                flags,
            )
        };
        crate::ocio_call_status()
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_config_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn create_raw_config() {
        let cfg = Config::raw();
        assert!(cfg.is_ok());
    }

    #[test]
    fn create_config_from_env_no_crash() {
        let _guard = env_lock();
        let path = "tests/data/configs/context_test1/config.ocio";
        let prev = std::env::var_os("OCIO");
        unsafe {
            std::env::set_var("OCIO", path);
        }

        let cfg = Config::from_env();

        match prev {
            Some(value) => unsafe { std::env::set_var("OCIO", value) },
            None => unsafe { std::env::remove_var("OCIO") },
        }

        if crate::is_stub_build() {
            let _ = cfg;
        } else {
            assert!(cfg.is_ok());
        }
    }

    #[test]
    fn create_config_from_stream_no_crash() {
        let text =
            fs::read_to_string("tests/data/configs/context_test1/config.ocio").expect("read ocio");
        let cfg = Config::from_stream(text);
        if crate::is_stub_build() {
            let _ = cfg;
        } else {
            assert!(cfg.is_ok());
        }
    }

    #[test]
    fn config_metadata_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.name();
        let _ = config.description();
        let _ = config.cache_id();
    }

    #[test]
    fn config_version() {
        let config = Config::raw().unwrap();
        let _ = config.major_version();
        let _ = config.minor_version();
        let _ = config.family_separator();
    }

    #[test]
    fn config_color_spaces() {
        let config = Config::raw().unwrap();
        // In stub mode, num_color_spaces is 0
        assert!(config.num_color_spaces() >= 0);
    }

    #[test]
    fn config_displays_views() {
        let config = Config::raw().unwrap();
        // In stub mode these return 0/None, but shouldn't crash
        let _ = config.num_displays();
        let _ = config.default_display();
        let _ = config.default_view("sRGB");
    }

    #[test]
    fn config_looks() {
        let config = Config::raw().unwrap();
        assert!(config.num_looks() >= 0);
        let _ = config.look_name_by_index(0);
    }

    #[test]
    fn config_luma_coefs() {
        let config = Config::raw().unwrap();
        let coefs = config.default_luma_coefs().unwrap();
        // Stub mode returns zeros
        assert_eq!(coefs.len(), 3);
    }

    #[test]
    fn config_roles() {
        let config = Config::raw().unwrap();
        assert!(config.num_roles() >= 0);
        let _ = config.has_role("default");
    }

    #[test]
    fn config_active() {
        let config = Config::raw().unwrap();
        let _ = config.active_displays();
        let _ = config.active_views();
    }

    #[test]
    fn create_processor() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw");
        assert!(proc.is_ok());
    }

    #[test]
    fn create_processor_display() {
        let config = Config::raw().unwrap();
        // In stub mode this returns error since stub config has no displays
        let proc = config.processor_display("raw", "sRGB", "Film", TransformDirection::Forward);
        // Just check it doesn't crash
        let _ = proc;
    }

    #[test]
    fn config_view_named_wrappers_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.default_view("sRGB");
        let _ = config.num_views("sRGB");
        let _ = config.view("sRGB", 0);
        let _ = config.default_view_with_color_space("sRGB", "raw");
        let _ = config.num_views_with_color_space("sRGB", "raw");
        let _ = config.view_with_color_space("sRGB", "raw", 0);
        let _ = config.num_views_by_reference_space(SearchReferenceSpaceType::Scene, "sRGB");
        let _ = config.view_by_reference_space(SearchReferenceSpaceType::Scene, "sRGB", 0);
        let _ = config.virtual_display_num_views(SearchReferenceSpaceType::Scene);
        let _ = config.virtual_display_view(SearchReferenceSpaceType::Scene, 0);
    }

    #[test]
    fn processor_from_transform_no_crash() {
        let config = Config::raw().unwrap();
        let ft = crate::transform::FileTransform::create().unwrap();
        let proc = config.processor_from_transform(&ft, TransformDirection::Forward);
        let _ = proc;
    }

    #[test]
    fn config_lookup_named_wrappers_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.color_space("raw");
        let _ = config.color_space_index("raw");
        let _ = config.look("look_name");
        let _ = config.named_transform("Default");
        let _ = config.named_transform_index("Default");
        let _ = config.view_transform("Default");
    }

    #[test]
    fn add_remove_color_space_no_crash() {
        let config = Config::raw().unwrap();
        let cs = ColorSpace::create().unwrap();
        cs.set_name("TestCS").unwrap();
        config.add_color_space(&cs);
        let _ = config.color_space_index("TestCS");
        let _ = config.is_color_space_used("TestCS");
        let _ = config.remove_color_space("TestCS");
    }

    #[test]
    fn get_look_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.look("look_name");
    }

    #[test]
    fn add_look_no_crash() {
        let config = Config::raw().unwrap();
        let look = Look::create().unwrap();
        look.set_name("MyLook").unwrap();
        config.add_look(&look);
    }

    #[test]
    fn search_path_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.search_path();
        assert!(config.set_search_path("/path/to/ocio").is_ok());
    }

    #[test]
    fn strict_parsing_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.is_strict_parsing_enabled();
        config.set_strict_parsing_enabled(true);
        config.set_strict_parsing_enabled(false);
    }

    #[test]
    fn set_role_no_crash() {
        let config = Config::raw().unwrap();
        assert!(config.set_role("default", "raw").is_ok());
    }

    #[test]
    fn set_family_separator_no_crash() {
        let config = Config::raw().unwrap();
        config.set_family_separator('|');
    }

    #[test]
    fn validate_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.validate();
    }

    #[test]
    fn serialize_no_crash() {
        let config = Config::raw().unwrap();
        let serialized = config.serialize().unwrap();
        if crate::is_stub_build() {
            assert!(serialized.is_none());
        } else {
            let serialized = serialized.expect("real OCIO config should serialize");
            assert!(
                !serialized.trim().is_empty(),
                "real OCIO config serialization should not be empty"
            );
            assert!(
                serialized.contains("ocio_profile_version"),
                "serialized config should look like OCIO YAML"
            );
        }
    }

    #[test]
    fn archive_no_crash() {
        let config = Config::raw().unwrap();
        if crate::is_stub_build() {
            assert!(config.archive().unwrap().is_none());
        } else if config.is_archivable() {
            let archived = config
                .archive()
                .unwrap()
                .expect("real archivable config should archive");
            assert!(
                !archived.trim().is_empty(),
                "real OCIO config archive should not be empty"
            );
        } else {
            assert!(matches!(config.archive(), Err(OcioError::Ocio(_))));
        }
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.create_editable_copy();
    }

    #[test]
    fn fail_on_missing_file() {
        if crate::is_stub_build() {
            let cfg = Config::from_file("tests/missing_config.ocio");
            assert!(cfg.is_err());
        }
    }

    #[test]
    fn set_active_displays_views_no_crash() {
        let config = Config::raw().unwrap();
        assert!(config.set_active_displays("sRGB").is_ok());
        assert!(config.set_active_views("Film,Log").is_ok());
    }

    #[test]
    fn display_view_transform_name_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.display_view_transform_name("sRGB", "Film");
        let _ = config.display_view_color_space_name("sRGB", "Film");
        let _ = config.display_view_rule("sRGB", "Film");
        let _ = config.display_view_description("sRGB", "Film");
    }

    #[test]
    fn set_default_luma_coefs_no_crash() {
        let config = Config::raw().unwrap();
        config
            .set_default_luma_coefs(&[0.2126, 0.7152, 0.0722])
            .unwrap();
    }

    #[test]
    fn clear_color_spaces_looks_no_crash() {
        let config = Config::raw().unwrap();
        config.try_clear_color_spaces().unwrap();
        config.try_clear_looks().unwrap();
    }

    #[test]
    fn display_management_no_crash() {
        let config = Config::raw().unwrap();
        assert!(config
            .add_display("sRGB", "Film", "DisplayTransform", "srgb")
            .is_ok());
        assert!(config
            .add_shared_view("SharedView", "TransformName", "srgb", "", "", "")
            .is_ok());
        assert!(config.remove_view("sRGB", "Film").is_ok());
    }

    #[test]
    fn named_transforms_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.num_named_transforms();
        let _ = config.named_transform_name_by_index(0);
        let _ = config.named_transform("Default");
        let _ = config.named_transform_index("Default");
    }

    #[test]
    fn add_remove_named_transform_no_crash() {
        let config = Config::raw().unwrap();
        let nt = NamedTransform::create().unwrap();
        nt.set_name("MyNamedTransform").unwrap();
        let identity = crate::transform::MatrixTransform::identity().unwrap();
        nt.set_transform(&identity, crate::TransformDirection::Forward);
        config.add_named_transform(&nt);
        assert!(config.remove_named_transform("MyNamedTransform").is_ok());
    }

    #[test]
    fn view_transforms_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.num_view_transforms();
        let _ = config.view_transform_name_by_index(0);
        let _ = config.view_transform("Default");
    }

    #[test]
    fn clear_all_no_crash() {
        let config = Config::raw().unwrap();
        config.clear_all();
    }

    #[test]
    fn version_setters_no_crash() {
        let config = Config::raw().unwrap();
        assert!(config.set_major_version(2).is_ok());
        assert!(config.set_minor_version(1).is_ok());
    }

    #[test]
    fn color_space_set_no_crash() {
        let config = Config::raw().unwrap();
        let set = config.color_space_set(Some(""));
        assert!(set.is_ok());
        if let Ok(set) = set {
            let _ = set.num_color_spaces();
            let _ = set.has_color_space("raw");
        }
    }

    #[test]
    fn working_dir_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.working_dir();
        assert!(config.set_working_dir("/path/to/working").is_ok());
    }

    #[test]
    fn file_rules_set_no_crash() {
        let config = Config::raw().unwrap();
        // Stub mode creates a default FileRules, real mode gets from config
        if let Ok(rules) = config.file_rules() {
            config.set_file_rules(&rules).unwrap();
        }
    }

    #[test]
    fn inactive_color_spaces_no_crash() {
        let config = Config::raw().unwrap();
        assert!(config.set_inactive_color_spaces("inactive_cs").is_ok());
    }

    #[test]
    fn is_archivable_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.is_archivable();
    }

    #[test]
    fn clear_processor_cache_no_crash() {
        let config = Config::raw().unwrap();
        assert!(config.try_clear_processor_cache().is_ok());
    }

    #[test]
    fn config_num_search_paths_no_crash() {
        let config = Config::raw().unwrap();
        let n = config.num_search_paths();
        assert!(n >= 0);
        let _ = config.search_path_by_index(0);
    }

    #[test]
    fn default_display_view_compat_aliases_no_crash() {
        let config = Config::raw().unwrap();
        #[allow(deprecated)]
        {
            assert!(config.set_default_display("sRGB").is_ok());
            assert!(config.set_default_view("Film").is_ok());
        }
        let _ = config.default_view_transform_name();
    }

    #[test]
    fn get_inactive_color_spaces_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.inactive_color_spaces();
    }

    #[test]
    fn color_space_by_ref_type_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.color_space_by_ref_type("raw", SearchReferenceSpaceType::Scene);
        let _ = config.color_space_by_ref_type("raw", SearchReferenceSpaceType::Display);
        let _ = config.color_space_by_ref_type("raw", SearchReferenceSpaceType::All);
    }

    #[test]
    fn color_space_from_filepath_with_rule_index_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.color_space_from_filepath_with_rule_index("test.jpg");
    }

    #[test]
    #[allow(deprecated)]
    fn color_space_from_filepath_compat_alias_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.get_color_space_from_filepath_by_ref_type("test.jpg");
    }

    #[test]
    fn processor_from_configs_no_crash() {
        let src_config = Config::raw().unwrap();
        let dst_config = Config::raw().unwrap();
        let proc = Config::processor_from_configs(&src_config, "raw", &dst_config, "raw");
        let _ = proc;
    }

    #[test]
    #[allow(deprecated)]
    fn processor_from_configs_overloads_no_crash() {
        let config = Config::raw().unwrap();
        let src_config = Config::raw().unwrap();
        let dst_config = Config::raw().unwrap();
        let src_ctx = src_config.current_context();
        let dst_ctx = dst_config.current_context();

        let _ =
            config.get_processor_from_configs_v2(&src_config, "raw", "", &dst_config, "raw", "");
        let _ = config.get_processor_from_configs_v4(
            &src_config,
            "raw",
            &dst_config,
            "sRGB",
            "Film",
            TransformDirection::Forward,
        );
        let _ = config.get_processor_from_configs_v6(
            &src_config,
            "raw",
            "",
            &dst_config,
            "sRGB",
            "Film",
            "",
            TransformDirection::Forward,
        );

        if let (Some(src_ctx), Some(dst_ctx)) = (src_ctx, dst_ctx) {
            let _ = config.get_processor_from_configs_v1(
                &src_ctx,
                &src_config,
                "raw",
                &dst_ctx,
                &dst_config,
                "raw",
            );
            let _ = config.get_processor_from_configs_v3(
                &src_ctx,
                &src_config,
                "raw",
                "",
                &dst_ctx,
                &dst_config,
                "raw",
                "",
            );
            let _ = config.get_processor_from_configs_v5(
                &src_ctx,
                &src_config,
                "raw",
                &dst_ctx,
                &dst_config,
                "sRGB",
                "Film",
                TransformDirection::Forward,
            );
            let _ = config.get_processor_from_configs_v7(
                &src_ctx,
                &src_config,
                "raw",
                "",
                &dst_ctx,
                &dst_config,
                "sRGB",
                "Film",
                "",
                TransformDirection::Forward,
            );
        }
    }

    #[test]
    fn processor_from_configs_named_wrappers_no_crash() {
        let config = Config::raw().unwrap();
        let src_config = Config::raw().unwrap();
        let dst_config = Config::raw().unwrap();
        let src_ctx = src_config.current_context();
        let dst_ctx = dst_config.current_context();

        let _ = config.processor_from_configs_with_interchange(
            &src_config,
            "raw",
            "",
            &dst_config,
            "raw",
            "",
        );
        let _ = config.processor_from_configs_to_display(
            &src_config,
            "raw",
            &dst_config,
            "sRGB",
            "Film",
            TransformDirection::Forward,
        );
        let _ = config.processor_from_configs_to_display_with_interchange(
            &src_config,
            "raw",
            "",
            &dst_config,
            "sRGB",
            "Film",
            "",
            TransformDirection::Forward,
        );

        if let (Some(src_ctx), Some(dst_ctx)) = (src_ctx, dst_ctx) {
            let _ = config.processor_from_configs_with_contexts(
                &src_ctx,
                &src_config,
                "raw",
                &dst_ctx,
                &dst_config,
                "raw",
            );
            let _ = config.processor_from_configs_with_contexts_and_interchange(
                &src_ctx,
                &src_config,
                "raw",
                "",
                &dst_ctx,
                &dst_config,
                "raw",
                "",
            );
            let _ = config.processor_from_configs_to_display_with_contexts(
                &src_ctx,
                &src_config,
                "raw",
                &dst_ctx,
                &dst_config,
                "sRGB",
                "Film",
                TransformDirection::Forward,
            );
            let _ = config.processor_from_configs_to_display_with_contexts_and_interchange(
                &src_ctx,
                &src_config,
                "raw",
                "",
                &dst_ctx,
                &dst_config,
                "sRGB",
                "Film",
                "",
                TransformDirection::Forward,
            );
        }
    }

    #[test]
    fn active_and_all_display_named_wrappers_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.num_active_displays();
        let _ = config.num_active_views();
        let _ = config.active_display(0);
        let _ = config.active_view(0);
        let _ = config.num_displays_all();
        let _ = config.display_all(0);
        let _ = config.display_all_index("sRGB");
    }

    #[test]
    fn processor_with_context_no_crash() {
        let config = Config::raw().unwrap();
        if let Some(ctx) = config.current_context() {
            let proc = config.processor_with_context("raw", "raw", &ctx);
            let _ = proc;
        }
    }

    #[test]
    fn processor_overload_named_wrappers_no_crash() {
        let config = Config::raw().unwrap();
        let ft = crate::transform::FileTransform::create().unwrap();
        let src_cs = crate::ColorSpace::create().unwrap();
        let dst_cs = crate::ColorSpace::create().unwrap();
        let nt = configured_named_transform_for_processor_tests();
        config.add_named_transform(&nt);

        let _ = config.processor("raw", "raw");
        let _ = config.processor_from_color_spaces(&src_cs, &dst_cs);
        let _ = config.processor_display("raw", "sRGB", "Film", TransformDirection::Forward);
        let _ = config.processor_from_transform_default_direction(&ft);
        let _ = config.processor_from_transform(&ft, TransformDirection::Forward);
        let _ = config.processor_named_transform(&nt, TransformDirection::Forward);
        let _ = config.processor_named_transform_name(
            "UnitCompatNamedTransform",
            TransformDirection::Forward,
        );
        let _ = config.processor_to_builtin_color_space(&config, "raw", "ACES2065-1");
        let _ = config.processor_from_builtin_color_space("ACES2065-1", &config, "raw");

        if let Some(ctx) = config.current_context() {
            let _ = config.processor_with_context("raw", "raw", &ctx);
            let _ = config.processor_display_with_context(
                "raw",
                "sRGB",
                "Film",
                TransformDirection::Forward,
                &ctx,
            );
            let _ = config.processor_from_transform_with_context(
                &ctx,
                &ft,
                TransformDirection::Forward,
            );
            let _ = config.processor_named_transform_with_context(
                &ctx,
                &nt,
                TransformDirection::Forward,
            );
            let _ = config.processor_named_transform_name_with_context(
                &ctx,
                "UnitCompatNamedTransform",
                TransformDirection::Forward,
            );
        }
    }

    #[test]
    #[allow(deprecated)]
    fn processor_basic_compat_aliases_no_crash() {
        let config = Config::raw().unwrap();
        let src_cs = crate::ColorSpace::create().unwrap();
        let dst_cs = crate::ColorSpace::create().unwrap();

        let _ = config.get_processor_v1(&src_cs, &dst_cs);
        let _ = config.get_processor_v2("raw", "raw");
        let _ = config.get_processor_v4("raw", "sRGB", "Film", TransformDirection::Forward);
    }

    #[test]
    #[allow(deprecated)]
    fn processor_context_compat_aliases_no_crash() {
        let config = Config::raw().unwrap();
        if let Some(ctx) = config.current_context() {
            let _ = config.get_processor_v3("raw", "raw", &ctx);
            let _ =
                config.get_processor_v5("raw", "sRGB", "Film", TransformDirection::Forward, &ctx);
        }
    }

    #[test]
    #[allow(deprecated)]
    fn processor_transform_compat_aliases_no_crash() {
        let config = Config::raw().unwrap();
        let ft = crate::transform::FileTransform::create().unwrap();

        let _ = config.get_processor_v10(&ft);
        let _ = config.get_processor_v11(&ft, TransformDirection::Forward);

        if let Some(ctx) = config.current_context() {
            let _ = config.get_processor_v12(&ctx, &ft, TransformDirection::Forward);
        }
    }

    #[test]
    #[allow(deprecated)]
    fn processor_named_transform_compat_aliases_no_crash() {
        let config = Config::raw().unwrap();
        let nt = configured_named_transform_for_processor_tests();
        config.add_named_transform(&nt);

        let _ = config.get_processor_v6(&nt, TransformDirection::Forward);
        let _ = config.get_processor_v8("UnitCompatNamedTransform", TransformDirection::Forward);

        if let Some(ctx) = config.current_context() {
            let _ = config.get_processor_v7(&ctx, &nt, TransformDirection::Forward);
            let _ = config.get_processor_v9(
                &ctx,
                "UnitCompatNamedTransform",
                TransformDirection::Forward,
            );
        }
    }

    #[test]
    #[allow(deprecated)]
    fn processor_builtin_color_space_compat_aliases_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.get_processor_to_builtin_color_space(&config, "raw", "ACES2065-1");
        let _ = config.get_processor_from_builtin_color_space("ACES2065-1", &config, "raw");
    }

    fn configured_named_transform_for_processor_tests() -> NamedTransform {
        let nt = NamedTransform::create().unwrap();
        nt.set_name("UnitCompatNamedTransform").unwrap();

        let forward = crate::transform::MatrixTransform::scale(&[1.0, 1.0, 1.0, 1.0]).unwrap();
        let inverse = crate::transform::MatrixTransform::scale(&[1.0, 1.0, 1.0, 1.0]).unwrap();
        nt.set_transform(&forward, TransformDirection::Forward);
        nt.set_transform(&inverse, TransformDirection::Inverse);
        nt
    }

    #[test]
    fn cache_id_with_context_no_crash() {
        let config = Config::raw().unwrap();
        if let Some(ctx) = config.current_context() {
            let _ = config.cache_id_for_context(&ctx);
            #[allow(deprecated)]
            let _ = config.cache_id_with_context(&ctx);
        }
    }

    #[test]
    fn search_paths_no_crash() {
        let config = Config::raw().unwrap();
        assert!(config.try_clear_search_paths().is_ok());
        assert!(config.add_search_path("/some/path").is_ok());
    }

    #[test]
    fn display_view_looks_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.display_view_looks("sRGB", "Film");
    }

    #[test]
    fn default_scene_to_display_view_transform_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.default_scene_to_display_view_transform();
    }

    #[test]
    fn color_spaces_looks_string_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.color_spaces();
        let _ = config.looks();
    }

    #[test]
    fn clear_named_view_transforms_no_crash() {
        let config = Config::raw().unwrap();
        config.try_clear_named_transforms().unwrap();
        config.try_clear_view_transforms().unwrap();
    }

    #[test]
    fn environment_mode_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.environment_mode();
        config
            .set_environment_mode(crate::EnvironmentMode::LoadAll)
            .unwrap();
        config
            .set_environment_mode(crate::EnvironmentMode::LoadPredefined)
            .unwrap();
        config.load_environment().unwrap();
    }

    #[test]
    #[allow(deprecated)]
    fn config_version_aliases_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.get_num_color_spaces_v1();
        let _ = config.get_color_space_name_by_index_v1(0);
        assert!(config.set_version(2, 5).is_ok());
        config.upgrade_to_latest_version().unwrap();
    }

    #[test]
    fn config_named_enumeration_wrappers_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.num_color_spaces();
        let _ = config.color_space_name_by_index(0);
        let _ = config.num_named_transforms();
        let _ = config.named_transform_name_by_index(0);
    }

    #[test]
    #[allow(deprecated)]
    fn config_io_proxy_no_crash() {
        let config = Config::raw().unwrap();
        unsafe { config.set_config_io_proxy(std::ptr::null_mut()) };
        let _ = config.config_io_proxy();
    }

    #[test]
    fn config_io_proxy_object_no_crash() {
        if crate::is_stub_build() {
            return;
        }

        let config = Config::raw().unwrap();
        let proxy = ConfigIOProxy::create().unwrap();
        proxy.set_config_data("ocio_profile_version: 2\nroles:\n  default: raw\ncolorspaces:\n  - !<ColorSpace> {name: raw, isdata: true}\n").unwrap();
        config.set_config_io_proxy_object(&proxy).unwrap();
        let _ = config.config_io_proxy_object();
    }

    #[test]
    fn create_from_config_io_proxy_real_behavior() {
        if crate::is_stub_build() {
            return;
        }

        let proxy = ConfigIOProxy::create().expect("config io proxy");
        let config_text = std::fs::read_to_string("tests/data/configs/context_test1/config.ocio")
            .expect("config text");
        let lut = std::fs::read("tests/data/configs/context_test1/lut1.clf").expect("lut");
        proxy
            .set_config_data(&config_text)
            .expect("set config data");
        proxy
            .set_lut_data("E:/virtual/context/lut1.clf", &lut, "lut1-hash")
            .expect("set lut data");

        let config = Config::from_config_io_proxy(&proxy).expect("config from proxy");
        config
            .set_working_dir("E:/virtual/context")
            .expect("working dir");

        let processor = config
            .processor("plain_lut1_cs", "reference")
            .expect("processor");
        let cpu = processor.optimized_cpu_processor(0).expect("cpu processor");
        let mut pixel = [1.0f32, 1.0, 1.0, 1.0];
        cpu.apply_rgba(&mut pixel);
        assert!((pixel[0] - 5.0).abs() < 1e-5);
        assert!((pixel[1] - 5.0).abs() < 1e-5);
        assert!((pixel[2] - 5.0).abs() < 1e-5);
    }

    #[test]
    #[allow(deprecated)]
    fn config_io_proxy_compat_alias_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.get_config_io_proxy();
    }

    #[test]
    #[allow(deprecated)]
    fn active_display_view_management_v251_no_crash() {
        let config = Config::raw().unwrap();
        assert!(config.add_active_display("sRGB").is_ok());
        assert!(config.add_active_view("Film").is_ok());
        let _ = config.num_active_displays();
        let _ = config.num_active_views();
        let _ = config.get_active_display(0);
        let _ = config.get_active_view(0);
        assert!(config.remove_active_display("sRGB").is_ok());
        assert!(config.remove_active_view("Film").is_ok());
        config.clear_active_displays();
        config.clear_active_views();
    }

    #[test]
    #[allow(deprecated)]
    fn display_view_v2_and_view_transform_aliases_no_crash() {
        let config = Config::raw().unwrap();
        let vt = crate::ViewTransform::create(crate::ReferenceSpaceType::Scene).unwrap();
        assert!(vt.set_name("MyViewTransform").is_ok());
        let identity = crate::transform::MatrixTransform::identity().unwrap();
        vt.set_transform(
            Some(&identity),
            crate::ViewTransformDirection::FromReference,
        );
        config.add_view_transform(&vt);

        assert!(config
            .set_default_view_transform_name("MyViewTransform")
            .is_ok());
        let _ = config.get_default_view_transform_name();
        assert!(config
            .add_display_view_v2("sRGB", "Film", "MyViewTransform", "raw", "", "", "",)
            .is_ok());
        let _ = config.get_num_views_v2(crate::SearchReferenceSpaceType::Scene, "sRGB");
        let _ = config.get_view_v2(crate::SearchReferenceSpaceType::Scene, "sRGB", 0);
    }

    #[test]
    fn view_transforms_named_wrappers_no_crash() {
        let config = Config::raw().unwrap();
        let vt = crate::ViewTransform::create(crate::ReferenceSpaceType::Scene).unwrap();
        assert!(vt.set_name("MyViewTransform").is_ok());
        let identity = crate::transform::MatrixTransform::identity().unwrap();
        vt.set_transform(
            Some(&identity),
            crate::ViewTransformDirection::FromReference,
        );
        config.add_view_transform(&vt);

        assert!(config
            .add_display_view_detailed("sRGB", "Film", "MyViewTransform", "raw", "", "", "")
            .is_ok());
        let _ = config.virtual_display_num_views(crate::SearchReferenceSpaceType::Scene);
        let _ = config.virtual_display_view(crate::SearchReferenceSpaceType::Scene, 0);
    }

    #[test]
    fn virtual_display_named_metadata_no_crash() {
        let config = Config::raw().unwrap();
        assert!(config
            .add_shared_view("SharedView", "MyViewTransform", "raw", "", "", "")
            .is_ok());
        assert!(config.add_display_shared_view("sRGB", "SharedView").is_ok());
        assert!(config.add_virtual_display_shared_view("SharedView").is_ok());
        assert!(config
            .add_virtual_display_view("VirtualFilm", "MyViewTransform", "raw", "", "", "")
            .is_ok());
        let _ = config.virtual_display_num_views(crate::SearchReferenceSpaceType::Scene);
        let _ = config.virtual_display_view(crate::SearchReferenceSpaceType::Scene, 0);
        let _ = config.virtual_display_view_transform_name("VirtualFilm");
        let _ = config.virtual_display_view_color_space_name("VirtualFilm");
        let _ = config.virtual_display_view_looks("VirtualFilm");
        let _ = config.virtual_display_view_rule("VirtualFilm");
        let _ = config.virtual_display_view_description("VirtualFilm");
        assert!(config.remove_virtual_display_view("VirtualFilm").is_ok());
        config.clear_virtual_display();
    }

    #[test]
    #[allow(deprecated)]
    fn virtual_display_management_no_crash() {
        let config = Config::raw().unwrap();
        assert!(config
            .add_shared_view("SharedView", "MyViewTransform", "raw", "", "", "")
            .is_ok());
        assert!(config.add_display_shared_view("sRGB", "SharedView").is_ok());
        assert!(config.add_virtual_display_shared_view("SharedView").is_ok());
        assert!(config
            .add_virtual_display_view("VirtualFilm", "MyViewTransform", "raw", "", "", "")
            .is_ok());
        let _ = config.get_virtual_display_num_views(crate::SearchReferenceSpaceType::Scene);
        let _ = config.get_virtual_display_view(crate::SearchReferenceSpaceType::Scene, 0);
        let _ = config.get_virtual_display_view_transform_name("VirtualFilm");
        let _ = config.get_virtual_display_view_color_space_name("VirtualFilm");
        let _ = config.get_virtual_display_view_looks("VirtualFilm");
        let _ = config.get_virtual_display_view_rule("VirtualFilm");
        let _ = config.get_virtual_display_view_description("VirtualFilm");
        assert!(config.remove_virtual_display_view("VirtualFilm").is_ok());
        config.clear_virtual_display();
    }

    #[test]
    #[allow(deprecated)]
    fn viewing_rules_pointer_round_trip_no_crash() {
        let config = Config::raw().unwrap();
        let ptr = unsafe { config.get_viewing_rules() };
        if !ptr.is_null() {
            unsafe { config.set_viewing_rules(ptr) };
        }
    }

    #[test]
    #[allow(deprecated)]
    fn builtin_config_entry_points_no_crash() {
        if let Ok(registry) = crate::BuiltinConfigRegistry::get() {
            if registry.num_builtin_configs() > 0 {
                if let Some(name) = registry.config_name(0) {
                    let config = Config::create_from_builtin_config(&name).unwrap();
                    let _ = config.validate();
                    let _ = config.get_num_displays_all();
                    let _ = config.get_display_all(0);
                }
            }
        }
    }
}
