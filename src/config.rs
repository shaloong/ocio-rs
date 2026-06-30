use std::ffi::c_void;
use std::ptr::NonNull;

use crate::transform::TransformHandle;
use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, ColorSpace, ColorSpaceSet, Context, FileRules,
    Look, NamedTransform, OcioError, Processor, ReferenceSpaceType, Result,
    SearchReferenceSpaceType, TransformDirection, ViewTransform,
};
use ocio_sys;

/// An OpenColorIO configuration.
///
/// `Config` is the main entry point for color spaces, displays, views, file
/// rules, and processor creation.
pub struct Config {
    pub(crate) handle: NonNull<c_void>,
}

impl Config {
    /// Create a config from one of OCIO's built-in configuration presets.
    ///
    /// Use `BuiltinConfigRegistry` to enumerate the preset names exposed by the
    /// linked OCIO build.
    pub fn create_from_builtin_config(config_name: impl AsRef<str>) -> Result<Self> {
        let config_name = cstring(config_name)?;
        let handle = unsafe {
            ocio_sys::ocio_config_create_from_builtin_config(config_name.as_ptr().cast())
        };
        NonNull::new(handle)
            .map(|handle| Self { handle })
            .ok_or(OcioError::AllocationFailed)
    }

    /// Create an empty editable config using OCIO defaults.
    pub fn raw() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_config_create_raw() };
        NonNull::new(handle)
            .map(|handle| Self { handle })
            .ok_or(OcioError::AllocationFailed)
    }

    /// Load a config from an `.ocio` file on disk.
    pub fn from_file(path: impl AsRef<str>) -> Result<Self> {
        let path = cstring(path)?;
        let handle = unsafe { ocio_sys::ocio_config_create_from_file(path.as_ptr().cast()) };
        NonNull::new(handle)
            .map(|handle| Self { handle })
            .ok_or(OcioError::AllocationFailed)
    }

    // --- Name & metadata ---

    pub fn name(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_name(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let name = cstring(name)?;
        unsafe { ocio_sys::ocio_config_set_name(self.handle.as_ptr(), name.as_ptr().cast()) };
        Ok(())
    }

    pub fn description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_description(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    pub fn set_description(&self, desc: impl AsRef<str>) -> Result<()> {
        let desc = cstring(desc)?;
        unsafe {
            ocio_sys::ocio_config_set_description(self.handle.as_ptr(), desc.as_ptr().cast())
        };
        Ok(())
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_cache_id(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    pub fn cache_id_with_context(&self, context_key: impl AsRef<str>) -> Option<String> {
        let ck = cstring(context_key).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_cache_id_n(
                self.handle.as_ptr(),
                ck.as_ptr() as *mut c_void,
            ))
        }
    }

    // --- Version ---

    pub fn major_version(&self) -> u32 {
        unsafe { ocio_sys::ocio_config_get_major_version(self.handle.as_ptr()) as u32 }
    }

    pub fn minor_version(&self) -> u32 {
        unsafe { ocio_sys::ocio_config_get_minor_version(self.handle.as_ptr()) as u32 }
    }

    pub fn set_version(&self, major: u32, minor: u32) {
        unsafe { ocio_sys::ocio_config_set_version(self.handle.as_ptr(), major, minor) };
    }

    pub fn upgrade_to_latest_version(&self) {
        unsafe { ocio_sys::ocio_config_upgrade_to_latest_version(self.handle.as_ptr()) };
    }

    pub fn family_separator(&self) -> char {
        let sep = unsafe {
            ocio_sys::ocio_config_get_family_separator(self.handle.as_ptr() as *mut c_void)
        };
        sep as u8 as char
    }

    // --- Color spaces ---

    pub fn num_color_spaces(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_num_color_spaces_v1(self.handle.as_ptr() as *mut c_void)
        }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_color_spaces()")]
    pub fn get_num_color_spaces_v1(&self) -> i32 {
        self.num_color_spaces()
    }

    pub fn color_space_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_color_space_name_by_index_v1(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer color_space_name_by_index()"
    )]
    pub fn get_color_space_name_by_index_v1(&self, index: i32) -> Option<String> {
        self.color_space_name_by_index(index)
    }

    pub fn color_spaces(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_color_spaces(
                self.handle.as_ptr(),
                std::ptr::null(),
            ))
        }
    }

    pub fn canonical_name(&self, name: impl AsRef<str>) -> Option<String> {
        let name = cstring(name).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_canonical_name(
                self.handle.as_ptr(),
                name.as_ptr().cast(),
            ))
        }
    }

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

    pub fn color_space_from_filepath(&self, file_path: impl AsRef<str>) -> Option<String> {
        let fp = cstring(file_path).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_color_space_from_filepath(
                self.handle.as_ptr(),
                fp.as_ptr().cast(),
            ))
        }
    }

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

    pub fn default_display(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_default_display(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn set_default_display(&self, display: impl AsRef<str>) -> Result<()> {
        let d = cstring(display)?;
        unsafe {
            ocio_sys::ocio_config_set_active_displays(self.handle.as_ptr(), d.as_ptr().cast())
        };
        Ok(())
    }

    pub fn num_displays(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_displays(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn display(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_display(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    // --- Views ---

    pub fn default_view(&self, display: impl AsRef<str>) -> Option<String> {
        let display = cstring(display).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_default_view(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
            ))
        }
    }

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

    pub fn set_default_view(&self, view: impl AsRef<str>) -> Result<()> {
        let v = cstring(view)?;
        unsafe { ocio_sys::ocio_config_set_active_views(self.handle.as_ptr(), v.as_ptr().cast()) };
        Ok(())
    }

    pub fn num_views(&self, display: impl AsRef<str>) -> i32 {
        let display = match cstring(display) {
            Ok(d) => d,
            Err(_) => return 0,
        };
        unsafe {
            ocio_sys::ocio_config_get_num_views(self.handle.as_ptr(), display.as_ptr().cast())
        }
    }

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

    pub fn view(&self, display: impl AsRef<str>, index: i32) -> Option<String> {
        let display = cstring(display).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_view(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                index,
            ))
        }
    }

    pub fn view_with_color_space(
        &self,
        display: impl AsRef<str>,
        color_space_name: impl AsRef<str>,
        index: i32,
    ) -> Option<String> {
        let display = cstring(display).ok()?;
        let color_space_name = cstring(color_space_name).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_view_v1(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                color_space_name.as_ptr().cast(),
                index,
            ))
        }
    }

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

    pub fn num_looks(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_looks(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn look_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_look_name_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    pub fn looks(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_look(
                self.handle.as_ptr(),
                std::ptr::null(),
            ))
        }
    }

    // --- Luma coefficients ---

    pub fn default_luma_coefs(&self) -> [f64; 3] {
        let mut coefs = [0.0f64; 3];
        unsafe {
            ocio_sys::ocio_config_get_default_luma_coefs(
                self.handle.as_ptr(),
                coefs.as_mut_ptr() as *mut c_void,
            );
        }
        coefs
    }

    pub fn set_default_luma_coefs(&self, coefs: &[f64; 3]) {
        unsafe {
            ocio_sys::ocio_config_set_default_luma_coefs(
                self.handle.as_ptr(),
                coefs.as_ptr() as *mut c_void,
            )
        };
    }

    // --- Roles ---

    pub fn num_roles(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_roles(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn has_role(&self, role: impl AsRef<str>) -> bool {
        let role = match cstring(role) {
            Ok(r) => r,
            Err(_) => return false,
        };
        unsafe { ocio_sys::ocio_config_has_role(self.handle.as_ptr(), role.as_ptr().cast()) }
    }

    pub fn role_name(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_role_name(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    pub fn role_color_space_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_role_color_space_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

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

    pub fn active_displays(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_active_displays(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn active_views(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_active_views(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    pub fn set_active_displays(&self, displays: impl AsRef<str>) -> Result<()> {
        let d = cstring(displays)?;
        unsafe {
            ocio_sys::ocio_config_set_active_displays(self.handle.as_ptr(), d.as_ptr().cast())
        };
        Ok(())
    }

    pub fn set_active_views(&self, views: impl AsRef<str>) -> Result<()> {
        let v = cstring(views)?;
        unsafe { ocio_sys::ocio_config_set_active_views(self.handle.as_ptr(), v.as_ptr().cast()) };
        Ok(())
    }

    // --- Display/view transform name queries ---

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

    #[deprecated(since = "0.2.0", note = "compat alias; prefer display_view_rule()")]
    pub fn get_display_view_rule(
        &self,
        display: impl AsRef<str>,
        view: impl AsRef<str>,
    ) -> Option<String> {
        self.display_view_rule(display, view)
    }

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

    pub fn default_scene_to_display_view_transform(&self) -> Option<crate::ViewTransform> {
        let handle = unsafe {
            ocio_sys::ocio_config_get_default_scene_to_display_view_transform(
                self.handle.as_ptr() as *mut c_void
            )
        };
        NonNull::new(handle).map(|h| crate::ViewTransform { handle: h })
    }

    pub fn default_view_transform_name(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_default_view_transform_name(
                self.handle.as_ptr(),
            ))
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer default_view_transform_name()"
    )]
    pub fn get_default_view_transform_name(&self) -> Option<String> {
        self.default_view_transform_name()
    }

    pub fn set_default_view_transform_name(&self, default_name: impl AsRef<str>) -> Result<()> {
        let default_name = cstring(default_name)?;
        unsafe {
            ocio_sys::ocio_config_set_default_view_transform_name(
                self.handle.as_ptr(),
                default_name.as_ptr().cast(),
            );
        }
        Ok(())
    }

    // --- Processors ---

    /// Create a processor between two named color spaces or named transforms.
    pub fn processor(&self, src: impl AsRef<str>, dst: impl AsRef<str>) -> Result<Processor> {
        let src = cstring(src)?;
        let dst = cstring(dst)?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v2(
                self.handle.as_ptr(),
                src.as_ptr().cast(),
                dst.as_ptr().cast(),
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn processor_from_color_spaces(
        &self,
        src_color_space: &ColorSpace,
        dst_color_space: &ColorSpace,
    ) -> Result<Processor> {
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v1(
                self.handle.as_ptr(),
                src_color_space.handle.as_ptr(),
                dst_color_space.handle.as_ptr(),
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v4(
                self.handle.as_ptr(),
                src.as_ptr().cast(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
                direction as i32,
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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

    pub fn processor_from_transform(
        &self,
        transform: &impl TransformHandle,
        direction: TransformDirection,
    ) -> Result<Processor> {
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v11(
                self.handle.as_ptr(),
                transform.as_ptr(),
                direction as i32,
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    /// Create a processor from a transform using OCIO's default transform direction.
    pub fn processor_from_transform_default_direction(
        &self,
        transform: &impl TransformHandle,
    ) -> Result<Processor> {
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v10(self.handle.as_ptr(), transform.as_ptr())
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer processor_from_transform_default_direction()"
    )]
    pub fn get_processor_v10(&self, transform: &impl TransformHandle) -> Result<Processor> {
        self.processor_from_transform_default_direction(transform)
    }

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

    pub fn processor_with_context(
        &self,
        src: impl AsRef<str>,
        dst: impl AsRef<str>,
        context: &crate::Context,
    ) -> Result<Processor> {
        let src = cstring(src)?;
        let dst = cstring(dst)?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v3(
                self.handle.as_ptr(),
                context.handle.as_ptr(),
                src.as_ptr().cast(),
                dst.as_ptr().cast(),
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v12(
                self.handle.as_ptr(),
                context.handle.as_ptr(),
                transform.as_ptr(),
                direction as i32,
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v6(
                self.handle.as_ptr(),
                named_transform.handle.as_ptr(),
                direction as i32,
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v7(
                self.handle.as_ptr(),
                context.handle.as_ptr(),
                named_transform.handle.as_ptr(),
                direction as i32,
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v8(
                self.handle.as_ptr(),
                named_transform_name.as_ptr().cast(),
                direction as i32,
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_v9(
                self.handle.as_ptr(),
                context.handle.as_ptr(),
                named_transform_name.as_ptr().cast(),
                direction as i32,
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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

    pub fn processor_to_builtin_color_space(
        &self,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
        builtin_color_space_name: impl AsRef<str>,
    ) -> Result<Processor> {
        let src_color_space_name = cstring(src_color_space_name)?;
        let builtin_color_space_name = cstring(builtin_color_space_name)?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_to_builtin_color_space(
                self.handle.as_ptr(),
                src_config.handle.as_ptr(),
                src_color_space_name.as_ptr().cast(),
                builtin_color_space_name.as_ptr().cast(),
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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

    pub fn processor_from_builtin_color_space(
        &self,
        builtin_color_space_name: impl AsRef<str>,
        src_config: &Config,
        src_color_space_name: impl AsRef<str>,
    ) -> Result<Processor> {
        let builtin_color_space_name = cstring(builtin_color_space_name)?;
        let src_color_space_name = cstring(src_color_space_name)?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_from_builtin_color_space(
                self.handle.as_ptr(),
                builtin_color_space_name.as_ptr().cast(),
                src_config.handle.as_ptr(),
                src_color_space_name.as_ptr().cast(),
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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

    pub fn processor_from_configs(
        src_config: &Config,
        src_name: impl AsRef<str>,
        dst_config: &Config,
        dst_name: impl AsRef<str>,
    ) -> Result<Processor> {
        let src_name = cstring(src_name)?;
        let dst_name = cstring(dst_name)?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_from_configs(
                src_config.handle.as_ptr(),
                src_config.handle.as_ptr(),
                src_name.as_ptr().cast(),
                dst_config.handle.as_ptr(),
                dst_name.as_ptr().cast(),
            )
        };
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    #[allow(clippy::too_many_arguments)]
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
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

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
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    #[allow(clippy::too_many_arguments)]
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
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    #[allow(clippy::too_many_arguments)]
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
        NonNull::new(handle)
            .map(|h| Processor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    #[allow(clippy::too_many_arguments)]
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

    pub fn color_space(&self, name: impl AsRef<str>) -> Option<ColorSpace> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_color_space(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| ColorSpace { handle: h })
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer color_space()")]
    pub fn get_color_space(&self, name: impl AsRef<str>) -> Option<ColorSpace> {
        self.color_space(name)
    }

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

    #[deprecated(since = "0.2.0", note = "compat alias; prefer color_space_index()")]
    pub fn get_index_for_color_space(&self, name: impl AsRef<str>) -> i32 {
        self.color_space_index(name)
    }

    pub fn add_color_space(&self, cs: &ColorSpace) {
        unsafe {
            ocio_sys::ocio_config_add_color_space(
                self.handle.as_ptr(),
                cs.handle.as_ptr() as *mut c_void,
            );
        }
    }

    pub fn remove_color_space(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe {
            ocio_sys::ocio_config_remove_color_space(self.handle.as_ptr(), n.as_ptr().cast());
        }
        Ok(())
    }

    pub fn is_color_space_used(&self, name: impl AsRef<str>) -> bool {
        let n = cstring(name);
        match n {
            Ok(n) => unsafe {
                ocio_sys::ocio_config_is_color_space_used(self.handle.as_ptr(), n.as_ptr().cast())
            },
            Err(_) => false,
        }
    }

    pub fn look(&self, name: impl AsRef<str>) -> Option<Look> {
        let n = cstring(name).ok()?;
        let handle =
            unsafe { ocio_sys::ocio_config_get_look(self.handle.as_ptr(), n.as_ptr().cast()) };
        NonNull::new(handle).map(|h| Look { handle: h })
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer look()")]
    pub fn get_look(&self, name: impl AsRef<str>) -> Option<Look> {
        self.look(name)
    }

    pub fn add_look(&self, look: &Look) {
        unsafe {
            ocio_sys::ocio_config_add_look(
                self.handle.as_ptr(),
                look.handle.as_ptr() as *mut c_void,
            );
        }
    }

    // --- Clear collections ---

    pub fn clear_color_spaces(&self) {
        unsafe { ocio_sys::ocio_config_clear_color_spaces(self.handle.as_ptr() as *mut c_void) };
    }

    pub fn clear_looks(&self) {
        unsafe { ocio_sys::ocio_config_clear_looks(self.handle.as_ptr() as *mut c_void) };
    }

    pub fn clear_named_transforms(&self) {
        unsafe {
            ocio_sys::ocio_config_clear_named_transforms(self.handle.as_ptr() as *mut c_void)
        };
    }

    pub fn clear_view_transforms(&self) {
        unsafe { ocio_sys::ocio_config_clear_view_transforms(self.handle.as_ptr() as *mut c_void) };
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
        unsafe {
            ocio_sys::ocio_config_add_display_view_v1(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
                transform_name.as_ptr().cast(),
                rule.as_ptr().cast(),
            );
        }
        Ok(())
    }

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
        unsafe {
            ocio_sys::ocio_config_add_display_view_v1(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
                color_space_name.as_ptr().cast(),
                looks.as_ptr().cast(),
            );
        }
        Ok(())
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
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
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
        Ok(())
    }

    pub fn remove_shared_view(&self, view: impl AsRef<str>) -> Result<()> {
        let view = cstring(view)?;
        unsafe {
            ocio_sys::ocio_config_remove_shared_view(self.handle.as_ptr(), view.as_ptr().cast())
        };
        Ok(())
    }

    pub fn clear_shared_views(&self) {
        unsafe { ocio_sys::ocio_config_clear_shared_views(self.handle.as_ptr()) };
    }

    pub fn remove_view(&self, display: impl AsRef<str>, view: impl AsRef<str>) -> Result<()> {
        let display = cstring(display)?;
        let view = cstring(view)?;
        unsafe {
            ocio_sys::ocio_config_remove_display_view(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
            );
        }
        Ok(())
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
        unsafe {
            ocio_sys::ocio_config_add_display_shared_view(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                shared_view.as_ptr().cast(),
            );
        }
        Ok(())
    }

    pub fn clear_displays(&self) {
        unsafe { ocio_sys::ocio_config_clear_displays(self.handle.as_ptr()) };
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
        Ok(())
    }

    pub fn add_virtual_display_shared_view(&self, shared_view: impl AsRef<str>) -> Result<()> {
        let shared_view = cstring(shared_view)?;
        unsafe {
            ocio_sys::ocio_config_add_virtual_display_shared_view(
                self.handle.as_ptr(),
                shared_view.as_ptr().cast(),
            );
        }
        Ok(())
    }

    pub fn virtual_display_num_views(&self, reference_space: SearchReferenceSpaceType) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_virtual_display_num_views(
                self.handle.as_ptr(),
                reference_space as i32,
            )
        }
    }

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
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_virtual_display_view(
                self.handle.as_ptr(),
                reference_space as i32,
                index,
            ))
        }
    }

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

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer virtual_display_view_description()"
    )]
    pub fn get_virtual_display_view_description(&self, view: impl AsRef<str>) -> Option<String> {
        self.virtual_display_view_description(view)
    }

    pub fn remove_virtual_display_view(&self, view: impl AsRef<str>) -> Result<()> {
        let view = cstring(view)?;
        unsafe {
            ocio_sys::ocio_config_remove_virtual_display_view(
                self.handle.as_ptr(),
                view.as_ptr().cast(),
            );
        }
        Ok(())
    }

    pub fn clear_virtual_display(&self) {
        unsafe { ocio_sys::ocio_config_clear_virtual_display(self.handle.as_ptr()) };
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

    pub fn num_named_transforms(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_num_named_transforms_v1(self.handle.as_ptr() as *mut c_void)
        }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_named_transforms()")]
    pub fn get_num_named_transforms_v1(&self) -> i32 {
        self.num_named_transforms()
    }

    pub fn named_transform_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_named_transform_name_by_index_v1(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer named_transform_name_by_index()"
    )]
    pub fn get_named_transform_name_by_index_v1(&self, index: i32) -> Option<String> {
        self.named_transform_name_by_index(index)
    }

    pub fn named_transform(&self, name: impl AsRef<str>) -> Option<NamedTransform> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_named_transform(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| NamedTransform { handle: h })
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer named_transform()")]
    pub fn get_named_transform(&self, name: impl AsRef<str>) -> Option<NamedTransform> {
        self.named_transform(name)
    }

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

    #[deprecated(since = "0.2.0", note = "compat alias; prefer named_transform_index()")]
    pub fn get_index_for_named_transform(&self, name: impl AsRef<str>) -> i32 {
        self.named_transform_index(name)
    }

    pub fn add_named_transform(&self, named_transform: &NamedTransform) {
        unsafe {
            ocio_sys::ocio_config_add_named_transform(
                self.handle.as_ptr(),
                named_transform.handle.as_ptr() as *mut c_void,
            );
        }
    }

    pub fn remove_named_transform(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe {
            ocio_sys::ocio_config_remove_named_transform(self.handle.as_ptr(), n.as_ptr().cast());
        }
        Ok(())
    }

    // --- View transforms ---

    pub fn num_view_transforms(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_num_view_transforms(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn view_transform_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_view_transform_name_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    pub fn view_transform(&self, name: impl AsRef<str>) -> Option<ViewTransform> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_view_transform(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| ViewTransform { handle: h })
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer view_transform()")]
    pub fn get_view_transform(&self, name: impl AsRef<str>) -> Option<ViewTransform> {
        self.view_transform(name)
    }

    pub fn add_view_transform(&self, view_transform: &ViewTransform) {
        unsafe {
            ocio_sys::ocio_config_add_view_transform(
                self.handle.as_ptr(),
                view_transform.handle.as_ptr() as *mut c_void,
            );
        }
    }

    // --- Search paths ---

    pub fn search_path(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_search_path(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    pub fn set_search_path(&self, path: impl AsRef<str>) -> Result<()> {
        let p = cstring(path)?;
        unsafe { ocio_sys::ocio_config_set_search_path(self.handle.as_ptr(), p.as_ptr().cast()) };
        Ok(())
    }

    pub fn num_search_paths(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_search_paths(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn search_path_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_search_path_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    pub fn clear_search_paths(&self) {
        unsafe { ocio_sys::ocio_config_clear_search_paths(self.handle.as_ptr() as *mut c_void) };
    }

    pub fn add_search_path(&self, path: impl AsRef<str>) -> Result<()> {
        let p = cstring(path)?;
        unsafe { ocio_sys::ocio_config_add_search_path(self.handle.as_ptr(), p.as_ptr().cast()) };
        Ok(())
    }

    // --- Strict parsing ---

    pub fn is_strict_parsing_enabled(&self) -> bool {
        unsafe {
            ocio_sys::ocio_config_is_strict_parsing_enabled(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn set_strict_parsing_enabled(&self, enabled: bool) {
        unsafe { ocio_sys::ocio_config_set_strict_parsing_enabled(self.handle.as_ptr(), enabled) };
    }

    // --- Roles (mutable) ---

    pub fn set_role(&self, role: impl AsRef<str>, color_space: impl AsRef<str>) -> Result<()> {
        let r = cstring(role)?;
        let cs = cstring(color_space)?;
        unsafe {
            ocio_sys::ocio_config_set_role(
                self.handle.as_ptr(),
                r.as_ptr().cast(),
                cs.as_ptr().cast(),
            );
        }
        Ok(())
    }

    // --- Family separator ---

    pub fn set_family_separator(&self, separator: char) {
        unsafe {
            ocio_sys::ocio_config_set_family_separator(self.handle.as_ptr(), separator as i8);
        }
    }

    // --- Validate ---

    pub fn validate(&self) -> Result<()> {
        unsafe { ocio_sys::ocio_config_validate(self.handle.as_ptr()) };
        Ok(()) // v2.5.1: validate() returns void
    }

    // --- Serialize ---

    /// Serialize the config to OCIO YAML text.
    ///
    /// Returns `None` in stub builds where no real OCIO serializer is linked.
    pub fn serialize(&self) -> Option<String> {
        self.serialize_to_string()
    }

    /// Serialize the config to OCIO YAML text.
    ///
    /// Returns `None` in stub builds where no real OCIO serializer is linked.
    pub fn serialize_to_string(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_serialize_to_string(
                self.handle.as_ptr(),
            ))
        }
    }

    // --- Editable copy ---

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_config_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    // --- Context ---

    /// Return the current context associated with this config, if available.
    pub fn current_context(&self) -> Option<Context> {
        let handle = unsafe {
            ocio_sys::ocio_config_get_current_context(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(handle).map(|h| Context { handle: h })
    }

    // --- Clear all ---

    pub fn clear_all(&self) {
        self.clear_color_spaces();
        self.clear_looks();
        self.clear_named_transforms();
        self.clear_view_transforms();
        unsafe {
            ocio_sys::ocio_config_clear_shared_views(self.handle.as_ptr());
            ocio_sys::ocio_config_clear_displays(self.handle.as_ptr());
            ocio_sys::ocio_config_clear_active_displays(self.handle.as_ptr());
            ocio_sys::ocio_config_clear_active_views(self.handle.as_ptr());
        }
    }

    // --- Version setters ---

    pub fn set_major_version(&self, version: u32) {
        unsafe { ocio_sys::ocio_config_set_major_version(self.handle.as_ptr(), version) };
    }

    pub fn set_minor_version(&self, version: u32) {
        unsafe { ocio_sys::ocio_config_set_minor_version(self.handle.as_ptr(), version) };
    }

    // --- Working directory ---

    pub fn working_dir(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_working_dir(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    pub fn set_working_dir(&self, dir_name: impl AsRef<str>) -> Result<()> {
        let d = cstring(dir_name)?;
        unsafe { ocio_sys::ocio_config_set_working_dir(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    // --- ColorSpaceSet ---

    pub fn color_space_set<S: AsRef<str>>(&self, search: Option<S>) -> Result<ColorSpaceSet> {
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
        NonNull::new(handle)
            .map(|h| ColorSpaceSet { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    // --- FileRules ---

    pub fn file_rules(&self) -> Result<FileRules> {
        let handle =
            unsafe { ocio_sys::ocio_config_get_file_rules(self.handle.as_ptr() as *mut c_void) };
        NonNull::new(handle)
            .map(|h| FileRules { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn set_file_rules(&self, file_rules: &FileRules) {
        unsafe {
            ocio_sys::ocio_config_set_file_rules(
                self.handle.as_ptr(),
                file_rules.handle.as_ptr() as *mut c_void,
            );
        }
    }

    // --- Environment mode ---

    pub fn set_environment_mode(&self, mode: crate::EnvironmentMode) {
        unsafe {
            ocio_sys::ocio_config_set_environment_mode(self.handle.as_ptr(), mode as i32);
        }
    }

    pub fn environment_mode(&self) -> crate::EnvironmentMode {
        let m = unsafe {
            ocio_sys::ocio_config_get_environment_mode(self.handle.as_ptr() as *mut c_void)
        };
        match m {
            1 => crate::EnvironmentMode::LoadAll,
            _ => crate::EnvironmentMode::LoadPredefined,
        }
    }

    pub fn load_environment(&self) {
        unsafe { ocio_sys::ocio_config_load_environment(self.handle.as_ptr() as *mut c_void) };
    }

    // --- Inactive color spaces ---

    pub fn inactive_color_spaces(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_inactive_color_spaces(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn set_inactive_color_spaces(&self, inactive: impl AsRef<str>) -> Result<()> {
        let s = cstring(inactive)?;
        unsafe {
            ocio_sys::ocio_config_set_inactive_color_spaces(
                self.handle.as_ptr(),
                s.as_ptr().cast(),
            );
        }
        Ok(())
    }

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

    pub fn is_archivable(&self) -> bool {
        unsafe { ocio_sys::ocio_config_is_archivable(self.handle.as_ptr() as *mut c_void) }
    }

    // --- Processor cache ---

    pub fn clear_processor_cache(&self) {
        unsafe { ocio_sys::ocio_config_clear_processor_cache(self.handle.as_ptr() as *mut c_void) };
    }

    // --- v2.5.1: Environment variables ---

    pub fn add_environment_var(
        &self,
        name: impl AsRef<str>,
        default_val: impl AsRef<str>,
    ) -> Result<()> {
        let n = cstring(name)?;
        let v = cstring(default_val)?;
        unsafe {
            ocio_sys::ocio_config_add_environment_var(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
                v.as_ptr().cast(),
            )
        };
        Ok(())
    }

    pub fn num_environment_vars(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_num_environment_vars(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn environment_var_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_environment_var_name_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    pub fn environment_var_default(&self, name: impl AsRef<str>) -> Option<String> {
        let n = cstring(name).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_environment_var_default(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
            ))
        }
    }

    pub fn clear_environment_vars(&self) {
        unsafe {
            ocio_sys::ocio_config_clear_environment_vars(self.handle.as_ptr() as *mut c_void)
        };
    }

    // --- v2.5.1: Active display/view management ---

    pub fn add_active_display(&self, display: impl AsRef<str>) -> Result<()> {
        let d = cstring(display)?;
        unsafe {
            ocio_sys::ocio_config_add_active_display(self.handle.as_ptr(), d.as_ptr().cast())
        };
        Ok(())
    }

    pub fn active_display(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_active_display(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer active_display()")]
    pub fn get_active_display(&self, index: i32) -> Option<String> {
        self.active_display(index)
    }

    pub fn remove_active_display(&self, display: impl AsRef<str>) -> Result<()> {
        let display = cstring(display)?;
        unsafe {
            ocio_sys::ocio_config_remove_active_display(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
            )
        };
        Ok(())
    }

    pub fn add_active_view(&self, view: impl AsRef<str>) -> Result<()> {
        let v = cstring(view)?;
        unsafe { ocio_sys::ocio_config_add_active_view(self.handle.as_ptr(), v.as_ptr().cast()) };
        Ok(())
    }

    pub fn active_view(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_active_view(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer active_view()")]
    pub fn get_active_view(&self, index: i32) -> Option<String> {
        self.active_view(index)
    }

    pub fn remove_active_view(&self, view: impl AsRef<str>) -> Result<()> {
        let view = cstring(view)?;
        unsafe {
            ocio_sys::ocio_config_remove_active_view(self.handle.as_ptr(), view.as_ptr().cast())
        };
        Ok(())
    }

    pub fn clear_active_displays(&self) {
        unsafe { ocio_sys::ocio_config_clear_active_displays(self.handle.as_ptr() as *mut c_void) };
    }

    pub fn clear_active_views(&self) {
        unsafe { ocio_sys::ocio_config_clear_active_views(self.handle.as_ptr() as *mut c_void) };
    }

    pub fn num_active_displays(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_num_active_displays(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn num_active_views(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_active_views(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn num_displays_all(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_displays_all(self.handle.as_ptr()) }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_displays_all()")]
    pub fn get_num_displays_all(&self) -> i32 {
        self.num_displays_all()
    }

    pub fn display_all(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_display_all(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

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

    #[deprecated(since = "0.2.0", note = "compat alias; prefer display_all_index()")]
    pub fn get_display_all_by_name(&self, display: impl AsRef<str>) -> i32 {
        self.display_all_index(display)
    }

    pub fn is_display_temporary(&self, index: i32) -> bool {
        unsafe { ocio_sys::ocio_config_is_display_temporary(self.handle.as_ptr(), index) }
    }

    pub fn set_display_temporary(&self, index: i32, temporary: bool) {
        unsafe {
            ocio_sys::ocio_config_set_display_temporary(self.handle.as_ptr(), index, temporary)
        };
    }

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

    pub fn view_by_reference_space(
        &self,
        reference_space: SearchReferenceSpaceType,
        display: impl AsRef<str>,
        index: i32,
    ) -> Option<String> {
        let display = cstring(display).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_get_view_v2(
                self.handle.as_ptr(),
                reference_space as i32,
                display.as_ptr().cast(),
                index,
            ))
        }
    }

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

    /// # Safety
    /// The returned pointer is owned by OCIO; `set_viewing_rules` requires a valid OCIO viewing-rules pointer.
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO viewing-rules handle; prefer higher-level Config APIs where possible"
    )]
    pub unsafe fn get_viewing_rules(&self) -> *mut c_void {
        unsafe { ocio_sys::ocio_config_get_viewing_rules(self.handle.as_ptr()) }
    }

    /// # Safety
    /// `viewing_rules` must be a valid OCIO viewing-rules pointer for the active ABI.
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO viewing-rules handle; prefer higher-level Config APIs where possible"
    )]
    pub unsafe fn set_viewing_rules(&self, viewing_rules: *mut c_void) {
        unsafe { ocio_sys::ocio_config_set_viewing_rules(self.handle.as_ptr(), viewing_rules) };
    }

    // --- v2.5.1: Misc utilities ---

    /// Archive the config to OCIO's textual archive representation.
    ///
    /// Returns `None` in stub builds where no real OCIO archiver is linked.
    pub fn archive(&self) -> Option<String> {
        self.archive_to_string()
    }

    /// Archive the config to OCIO's textual archive representation.
    ///
    /// Returns `None` in stub builds where no real OCIO archiver is linked.
    pub fn archive_to_string(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_config_archive_to_string(
                self.handle.as_ptr(),
            ))
        }
    }

    pub fn default_family_separator(&self) -> char {
        unsafe {
            ocio_sys::ocio_config_get_default_family_separator(self.handle.as_ptr() as *mut c_void)
                as u8 as char
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO config-IO proxy handle; prefer file/path based Config APIs where possible"
    )]
    pub fn config_io_proxy(&self) -> *mut std::ffi::c_void {
        unsafe { ocio_sys::ocio_config_get_config_io_proxy(self.handle.as_ptr() as *mut c_void) }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer config_io_proxy()")]
    pub fn get_config_io_proxy(&self) -> *mut std::ffi::c_void {
        unsafe { ocio_sys::ocio_config_get_config_io_proxy(self.handle.as_ptr() as *mut c_void) }
    }

    /// # Safety
    /// The caller must pass a valid OCIO config-IO proxy pointer for the active ABI.
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO config-IO proxy handle; prefer file/path based Config APIs where possible"
    )]
    pub unsafe fn set_config_io_proxy(&self, proxy: *mut std::ffi::c_void) {
        unsafe { ocio_sys::ocio_config_set_config_io_proxy(self.handle.as_ptr(), proxy) };
    }

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

    pub fn processor_cache_flags(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_config_get_processor_cache_flags(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn set_processor_cache_flags(&self, flags: i32) {
        unsafe {
            ocio_sys::ocio_config_set_processor_cache_flags(
                self.handle.as_ptr() as *mut c_void,
                flags,
            )
        };
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

    #[test]
    fn create_raw_config() {
        let cfg = Config::raw();
        assert!(cfg.is_ok());
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
        let coefs = config.default_luma_coefs();
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
        let serialized = config.serialize();
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
        let archived = config.archive();
        if crate::is_stub_build() {
            assert!(archived.is_none());
        } else if config.is_archivable() {
            let archived = archived.expect("real archivable config should archive");
            assert!(
                !archived.trim().is_empty(),
                "real OCIO config archive should not be empty"
            );
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
        config.set_default_luma_coefs(&[0.2126, 0.7152, 0.0722]);
    }

    #[test]
    fn clear_color_spaces_looks_no_crash() {
        let config = Config::raw().unwrap();
        config.clear_color_spaces();
        config.clear_looks();
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
        config.set_major_version(2);
        config.set_minor_version(1);
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
            config.set_file_rules(&rules);
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
        config.clear_processor_cache();
    }

    #[test]
    fn config_num_search_paths_no_crash() {
        let config = Config::raw().unwrap();
        let n = config.num_search_paths();
        assert!(n >= 0);
        let _ = config.search_path_by_index(0);
    }

    #[test]
    fn set_default_display_view_no_crash() {
        let config = Config::raw().unwrap();
        assert!(config.set_default_display("sRGB").is_ok());
        assert!(config.set_default_view("Film").is_ok());
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
        let nt = NamedTransform::create().unwrap();

        let _ = config.processor_from_transform_default_direction(&ft);
        let _ = config.processor_named_transform(&nt, TransformDirection::Forward);
        let _ = config.processor_named_transform_name("Default", TransformDirection::Forward);

        if let Some(ctx) = config.current_context() {
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
                "Default",
                TransformDirection::Forward,
            );
        }
    }

    #[test]
    fn cache_id_with_context_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.cache_id_with_context("context_key");
    }

    #[test]
    fn search_paths_no_crash() {
        let config = Config::raw().unwrap();
        config.clear_search_paths();
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
        config.clear_named_transforms();
        config.clear_view_transforms();
    }

    #[test]
    fn environment_mode_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.environment_mode();
        config.set_environment_mode(crate::EnvironmentMode::LoadAll);
        config.set_environment_mode(crate::EnvironmentMode::LoadPredefined);
        config.load_environment();
    }

    #[test]
    #[allow(deprecated)]
    fn config_version_aliases_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.get_num_color_spaces_v1();
        let _ = config.get_color_space_name_by_index_v1(0);
        config.set_version(2, 5);
        config.upgrade_to_latest_version();
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
