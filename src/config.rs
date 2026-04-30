use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, ColorSpaceSet, FileRules, OcioError, Processor, ColorSpace, Look, Context, Result, TransformDirection, ReferenceSpaceType, SearchReferenceSpaceType, NamedTransform, ViewTransform, Interpolation};
use crate::transform::TransformHandle;

pub struct Config {
    pub(crate) handle: NonNull<c_void>,
}

impl Config {
    pub fn raw() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_config_create_raw() };
        NonNull::new(handle).map(|handle| Self { handle }).ok_or(OcioError::AllocationFailed)
    }

    pub fn from_file(path: impl AsRef<str>) -> Result<Self> {
        let path = cstring(path)?;
        let handle = unsafe { ocio_sys::ocio_config_create_from_file(path.as_ptr().cast()) };
        NonNull::new(handle).map(|handle| Self { handle }).ok_or(OcioError::AllocationFailed)
    }

    // --- Name & metadata ---

    pub fn name(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_name(self.handle.as_ptr())) }
    }

    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let name = cstring(name)?;
        unsafe { ocio_sys::ocio_config_set_name(self.handle.as_ptr(), name.as_ptr().cast()) };
        Ok(())
    }

    pub fn description(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_description(self.handle.as_ptr())) }
    }

    pub fn set_description(&self, desc: impl AsRef<str>) -> Result<()> {
        let desc = cstring(desc)?;
        unsafe { ocio_sys::ocio_config_set_description(self.handle.as_ptr(), desc.as_ptr().cast()) };
        Ok(())
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_cache_id(self.handle.as_ptr())) }
    }

    pub fn cache_id_with_context(&self, context_key: impl AsRef<str>) -> Option<String> {
        let ck = cstring(context_key).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_cache_id_n(
                self.handle.as_ptr(), ck.as_ptr().cast(),
            ))
        }
    }

    // --- Version ---

    pub fn major_version(&self) -> u32 {
        unsafe { ocio_sys::ocio_config_get_major_version(self.handle.as_ptr()) }
    }

    pub fn minor_version(&self) -> u32 {
        unsafe { ocio_sys::ocio_config_get_minor_version(self.handle.as_ptr()) }
    }

    pub fn family_separator(&self) -> char {
        let sep = unsafe { ocio_sys::ocio_config_get_family_separator(self.handle.as_ptr()) };
        sep as u8 as char
    }

    // --- Color spaces ---

    pub fn num_color_spaces(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_color_spaces(self.handle.as_ptr()) }
    }

    pub fn color_space_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_color_space_name_by_index(
                self.handle.as_ptr(), index,
            ))
        }
    }

    pub fn color_spaces(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_color_spaces(self.handle.as_ptr())) }
    }

    pub fn canonical_name(&self, name: impl AsRef<str>) -> Option<String> {
        let name = cstring(name).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_canonical_name(
                self.handle.as_ptr(), name.as_ptr().cast(),
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
                self.handle.as_ptr(), cs.as_ptr().cast(), reference as i32,
            )
        }
    }

    pub fn color_space_from_filepath(&self, file_path: impl AsRef<str>) -> Option<String> {
        let fp = cstring(file_path).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_color_space_from_filepath(
                self.handle.as_ptr(), fp.as_ptr().cast(),
            ))
        }
    }

    // --- Displays ---

    pub fn default_display(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_default_display(self.handle.as_ptr())) }
    }

    pub fn set_default_display(&self, display: impl AsRef<str>) -> Result<()> {
        let d = cstring(display)?;
        unsafe { ocio_sys::ocio_config_set_default_display(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn num_displays(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_displays(self.handle.as_ptr()) }
    }

    pub fn display(&self, index: i32) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_display(self.handle.as_ptr(), index)) }
    }

    // --- Views ---

    pub fn default_view(&self, display: impl AsRef<str>) -> Option<String> {
        let display = cstring(display).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_default_view(
                self.handle.as_ptr(), display.as_ptr().cast(),
            ))
        }
    }

    pub fn set_default_view(&self, view: impl AsRef<str>) -> Result<()> {
        let v = cstring(view)?;
        unsafe { ocio_sys::ocio_config_set_default_view(self.handle.as_ptr(), v.as_ptr().cast()) };
        Ok(())
    }

    pub fn num_views(&self, display: impl AsRef<str>) -> i32 {
        let display = match cstring(display) { Ok(d) => d, Err(_) => return 0 };
        unsafe { ocio_sys::ocio_config_get_num_views(self.handle.as_ptr(), display.as_ptr().cast()) }
    }

    pub fn view(&self, display: impl AsRef<str>, index: i32) -> Option<String> {
        let display = cstring(display).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_view(
                self.handle.as_ptr(), display.as_ptr().cast(), index,
            ))
        }
    }

    // --- Looks ---

    pub fn num_looks(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_looks(self.handle.as_ptr()) }
    }

    pub fn look_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_look_name_by_index(
                self.handle.as_ptr(), index,
            ))
        }
    }

    pub fn looks(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_looks(self.handle.as_ptr())) }
    }

    // --- Luma coefficients ---

    pub fn default_luma_coefs(&self) -> [f64; 3] {
        let mut coefs = [0.0f64; 3];
        unsafe {
            ocio_sys::ocio_config_get_default_luma_coefs(self.handle.as_ptr(), coefs.as_mut_ptr());
        }
        coefs
    }

    pub fn set_default_luma_coefs(&self, coefs: &[f64; 3]) {
        unsafe { ocio_sys::ocio_config_set_default_luma_coefs(self.handle.as_ptr(), coefs.as_ptr()) };
    }

    // --- Roles ---

    pub fn num_roles(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_roles(self.handle.as_ptr()) }
    }

    pub fn has_role(&self, role: impl AsRef<str>) -> bool {
        let role = match cstring(role) { Ok(r) => r, Err(_) => return false };
        unsafe { ocio_sys::ocio_config_has_role(self.handle.as_ptr(), role.as_ptr().cast()) }
    }

    pub fn role_name(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_role_name(self.handle.as_ptr(), index))
        }
    }

    pub fn role_color_space_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_role_color_space_by_index(
                self.handle.as_ptr(), index,
            ))
        }
    }

    pub fn role_color_space(&self, role_name: impl AsRef<str>) -> Option<String> {
        let role = cstring(role_name).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_role_color_space_by_name(
                self.handle.as_ptr(), role.as_ptr().cast(),
            ))
        }
    }

    // --- Active displays / views ---

    pub fn active_displays(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_active_displays(self.handle.as_ptr())) }
    }

    pub fn active_views(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_active_views(self.handle.as_ptr())) }
    }

    pub fn set_active_displays(&self, displays: impl AsRef<str>) -> Result<()> {
        let d = cstring(displays)?;
        unsafe { ocio_sys::ocio_config_set_active_displays(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn set_active_views(&self, views: impl AsRef<str>) -> Result<()> {
        let v = cstring(views)?;
        unsafe { ocio_sys::ocio_config_set_active_views(self.handle.as_ptr(), v.as_ptr().cast()) };
        Ok(())
    }

    // --- Display/view transform name queries ---

    pub fn display_view_transform_name(&self, display: impl AsRef<str>, view: impl AsRef<str>) -> Option<String> {
        let display = cstring(display).ok()?;
        let view = cstring(view).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_display_view_transform_name(
                self.handle.as_ptr(), display.as_ptr().cast(), view.as_ptr().cast(),
            ))
        }
    }

    pub fn display_view_color_space_name(&self, display: impl AsRef<str>, view: impl AsRef<str>) -> Option<String> {
        let display = cstring(display).ok()?;
        let view = cstring(view).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_display_view_color_space_name(
                self.handle.as_ptr(), display.as_ptr().cast(), view.as_ptr().cast(),
            ))
        }
    }

    pub fn display_view_looks(&self, display: impl AsRef<str>, view: impl AsRef<str>) -> Option<String> {
        let d = cstring(display).ok()?;
        let v = cstring(view).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_display_view_looks(
                self.handle.as_ptr(), d.as_ptr().cast(), v.as_ptr().cast(),
            ))
        }
    }

    pub fn default_scene_to_display_view_transform(&self) -> Option<crate::ViewTransform> {
        let handle = unsafe {
            ocio_sys::ocio_config_get_default_scene_to_display_view_transform(self.handle.as_ptr())
        };
        NonNull::new(handle).map(|h| crate::ViewTransform { handle: h })
    }

    // --- Processors ---

    pub fn processor(&self, src: impl AsRef<str>, dst: impl AsRef<str>) -> Result<Processor> {
        let src = cstring(src)?;
        let dst = cstring(dst)?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor(
                self.handle.as_ptr(),
                src.as_ptr().cast(),
                dst.as_ptr().cast(),
            )
        };
        NonNull::new(handle).map(|h| Processor { handle: h }).ok_or(OcioError::AllocationFailed)
    }

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
            ocio_sys::ocio_config_get_processor_display(
                self.handle.as_ptr(),
                src.as_ptr().cast(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
                direction as i32,
            )
        };
        NonNull::new(handle).map(|h| Processor { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn processor_from_transform(
        &self,
        transform: &impl TransformHandle,
        direction: TransformDirection,
    ) -> Result<Processor> {
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor_transform(
                self.handle.as_ptr(),
                transform.as_ptr(),
                direction as i32,
            )
        };
        NonNull::new(handle).map(|h| Processor { handle: h }).ok_or(OcioError::AllocationFailed)
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
            ocio_sys::ocio_config_get_processor_with_context(
                self.handle.as_ptr(),
                src.as_ptr().cast(),
                dst.as_ptr().cast(),
                context.handle.as_ptr(),
            )
        };
        NonNull::new(handle).map(|h| Processor { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    /// Create a processor from two distinct configs (cross-config conversion).
    /// This is a static method on Config in OCIO v2.
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
                src_name.as_ptr().cast(),
                dst_config.handle.as_ptr(),
                dst_name.as_ptr().cast(),
            )
        };
        NonNull::new(handle).map(|h| Processor { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn get_color_space(&self, name: impl AsRef<str>) -> Option<ColorSpace> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_color_space(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| ColorSpace { handle: h })
    }

    pub fn color_space_by_ref_type(&self, name: impl AsRef<str>, ref_type: SearchReferenceSpaceType) -> Option<ColorSpace> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_color_space_by_ref_type(
                self.handle.as_ptr(), n.as_ptr().cast(), ref_type as i32,
            )
        };
        NonNull::new(handle).map(|h| ColorSpace { handle: h })
    }

    pub fn color_space_from_filepath_by_ref_type(&self, path: impl AsRef<str>, ref_type: SearchReferenceSpaceType) -> Result<ColorSpace> {
        let fp = cstring(path)?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_color_space_from_filepath_by_ref_type(
                self.handle.as_ptr(), fp.as_ptr().cast(), ref_type as i32,
            )
        };
        NonNull::new(handle).map(|h| ColorSpace { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn get_index_for_color_space(&self, name: impl AsRef<str>) -> i32 {
        let n = cstring(name);
        match n {
            Ok(n) => unsafe {
                ocio_sys::ocio_config_get_index_for_color_space(self.handle.as_ptr(), n.as_ptr().cast())
            },
            Err(_) => -1,
        }
    }

    pub fn add_color_space(&self, cs: &ColorSpace) {
        unsafe {
            ocio_sys::ocio_config_add_color_space(self.handle.as_ptr(), cs.handle.as_ptr());
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

    pub fn get_look(&self, name: impl AsRef<str>) -> Option<Look> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_look(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| Look { handle: h })
    }

    pub fn add_look(&self, look: &Look) {
        unsafe {
            ocio_sys::ocio_config_add_look(self.handle.as_ptr(), look.handle.as_ptr());
        }
    }

    // --- Clear collections ---

    pub fn clear_color_spaces(&self) {
        unsafe { ocio_sys::ocio_config_clear_color_spaces(self.handle.as_ptr()) };
    }

    pub fn clear_looks(&self) {
        unsafe { ocio_sys::ocio_config_clear_looks(self.handle.as_ptr()) };
    }

    pub fn clear_named_transforms(&self) {
        unsafe { ocio_sys::ocio_config_clear_named_transforms(self.handle.as_ptr()) };
    }

    pub fn clear_view_transforms(&self) {
        unsafe { ocio_sys::ocio_config_clear_view_transforms(self.handle.as_ptr()) };
    }

    // --- Display/view management ---

    pub fn add_display(&self, display: impl AsRef<str>, view: impl AsRef<str>, transform_name: impl AsRef<str>, rule: impl AsRef<str>) -> Result<()> {
        let display = cstring(display)?;
        let view = cstring(view)?;
        let transform_name = cstring(transform_name)?;
        let rule = cstring(rule)?;
        unsafe {
            ocio_sys::ocio_config_add_display(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
                transform_name.as_ptr().cast(),
                rule.as_ptr().cast(),
            );
        }
        Ok(())
    }

    pub fn add_shared_view(&self, display: impl AsRef<str>, view: impl AsRef<str>, transform_name: impl AsRef<str>, rule: impl AsRef<str>) -> Result<()> {
        let display = cstring(display)?;
        let view = cstring(view)?;
        let transform_name = cstring(transform_name)?;
        let rule = cstring(rule)?;
        unsafe {
            ocio_sys::ocio_config_add_shared_view(
                self.handle.as_ptr(),
                display.as_ptr().cast(),
                view.as_ptr().cast(),
                transform_name.as_ptr().cast(),
                rule.as_ptr().cast(),
            );
        }
        Ok(())
    }

    pub fn remove_display(&self, display: impl AsRef<str>) -> Result<()> {
        let d = cstring(display)?;
        unsafe { ocio_sys::ocio_config_remove_display(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn remove_view(&self, display: impl AsRef<str>, view: impl AsRef<str>) -> Result<()> {
        let display = cstring(display)?;
        let view = cstring(view)?;
        unsafe {
            ocio_sys::ocio_config_remove_view(
                self.handle.as_ptr(), display.as_ptr().cast(), view.as_ptr().cast(),
            );
        }
        Ok(())
    }

    // --- Named transforms ---

    pub fn num_named_transforms(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_named_transforms(self.handle.as_ptr()) }
    }

    pub fn named_transform_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_named_transform_name_by_index(
                self.handle.as_ptr(), index,
            ))
        }
    }

    pub fn get_named_transform(&self, name: impl AsRef<str>) -> Option<NamedTransform> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_named_transform(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| NamedTransform { handle: h })
    }

    pub fn add_named_transform(&self, named_transform: &NamedTransform) {
        unsafe {
            ocio_sys::ocio_config_add_named_transform(self.handle.as_ptr(), named_transform.handle.as_ptr());
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
        unsafe { ocio_sys::ocio_config_get_num_view_transforms(self.handle.as_ptr()) }
    }

    pub fn view_transform_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_get_view_transform_name_by_index(
                self.handle.as_ptr(), index,
            ))
        }
    }

    pub fn get_view_transform(&self, name: impl AsRef<str>) -> Option<ViewTransform> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_view_transform(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| ViewTransform { handle: h })
    }

    pub fn add_view_transform(&self, view_transform: &ViewTransform) {
        unsafe {
            ocio_sys::ocio_config_add_view_transform(self.handle.as_ptr(), view_transform.handle.as_ptr());
        }
    }

    pub fn remove_view_transform(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe {
            ocio_sys::ocio_config_remove_view_transform(self.handle.as_ptr(), n.as_ptr().cast());
        }
        Ok(())
    }

    // --- Search paths ---

    pub fn search_path(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_search_path(self.handle.as_ptr())) }
    }

    pub fn set_search_path(&self, path: impl AsRef<str>) -> Result<()> {
        let p = cstring(path)?;
        unsafe { ocio_sys::ocio_config_set_search_path(self.handle.as_ptr(), p.as_ptr().cast()) };
        Ok(())
    }

    pub fn num_search_paths(&self) -> i32 {
        unsafe { ocio_sys::ocio_config_get_num_search_paths(self.handle.as_ptr()) }
    }

    pub fn search_path_by_index(&self, index: i32) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_search_path_by_index(self.handle.as_ptr(), index)) }
    }

    pub fn clear_search_paths(&self) {
        unsafe { ocio_sys::ocio_config_clear_search_paths(self.handle.as_ptr()) };
    }

    pub fn add_search_path(&self, path: impl AsRef<str>) -> Result<()> {
        let p = cstring(path)?;
        unsafe { ocio_sys::ocio_config_add_search_path(self.handle.as_ptr(), p.as_ptr().cast()) };
        Ok(())
    }

    // --- Strict parsing ---

    pub fn is_strict_parsing_enabled(&self) -> bool {
        unsafe { ocio_sys::ocio_config_is_strict_parsing_enabled(self.handle.as_ptr()) }
    }

    pub fn set_strict_parsing_enabled(&self, enabled: bool) {
        unsafe { ocio_sys::ocio_config_set_strict_parsing_enabled(self.handle.as_ptr(), enabled) };
    }

    // --- Roles (mutable) ---

    pub fn set_role(&self, role: impl AsRef<str>, color_space: impl AsRef<str>) -> Result<()> {
        let r = cstring(role)?;
        let cs = cstring(color_space)?;
        unsafe {
            ocio_sys::ocio_config_set_role(self.handle.as_ptr(), r.as_ptr().cast(), cs.as_ptr().cast());
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
        let err = unsafe { ocio_sys::ocio_config_validate(self.handle.as_ptr()) };
        if let Some(msg) = unsafe { cstr_to_opt_string(err) } {
            Err(OcioError::ValidationFailed(msg))
        } else {
            Ok(())
        }
    }

    // --- Serialize ---

    pub fn serialize(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_serialize(self.handle.as_ptr())) }
    }

    // --- Editable copy ---

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_config_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    // --- Context ---

    pub fn current_context(&self) -> Option<Context> {
        let handle = unsafe { ocio_sys::ocio_config_get_current_context(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Context { handle: h })
    }

    pub fn set_current_context(&self, context: &Context) {
        unsafe {
            ocio_sys::ocio_config_set_current_context(self.handle.as_ptr(), context.handle.as_ptr());
        }
    }

    // --- Clear all ---

    pub fn clear_all(&self) {
        unsafe { ocio_sys::ocio_config_clear_all(self.handle.as_ptr()) };
    }

    // --- Version setters ---

    pub fn set_major_version(&self, version: u32) {
        unsafe { ocio_sys::ocio_config_set_major_version(self.handle.as_ptr(), version) };
    }

    pub fn set_minor_version(&self, version: u32) {
        unsafe { ocio_sys::ocio_config_set_minor_version(self.handle.as_ptr(), version) };
    }

    // --- Default interpolation ---

    pub fn default_interpolation(&self) -> Interpolation {
        let i = unsafe { ocio_sys::ocio_config_get_default_interpolation(self.handle.as_ptr()) };
        match i {
            1 => Interpolation::Nearest,
            2 => Interpolation::Linear,
            3 => Interpolation::Tetrahedral,
            4 => Interpolation::Cubic,
            5 => Interpolation::Default,
            6 => Interpolation::Best,
            _ => Interpolation::Unknown,
        }
    }

    pub fn set_default_interpolation(&self, interpolation: Interpolation) {
        unsafe {
            ocio_sys::ocio_config_set_default_interpolation(self.handle.as_ptr(), interpolation as i32);
        }
    }

    // --- Working directory ---

    pub fn working_dir(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_working_dir(self.handle.as_ptr())) }
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
                    ocio_sys::ocio_config_get_color_space_set(self.handle.as_ptr(), s.as_ptr().cast())
                }
            }
            None => {
                unsafe {
                    ocio_sys::ocio_config_get_color_space_set(self.handle.as_ptr(), std::ptr::null())
                }
            }
        };
        NonNull::new(handle).map(|h| ColorSpaceSet { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    // --- FileRules ---

    pub fn file_rules(&self) -> Result<FileRules> {
        let handle = unsafe { ocio_sys::ocio_config_get_file_rules(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| FileRules { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn set_file_rules(&self, file_rules: &FileRules) {
        unsafe {
            ocio_sys::ocio_config_set_file_rules(self.handle.as_ptr(), file_rules.handle.as_ptr());
        }
    }

    // --- Environment mode ---

    pub fn set_environment_mode(&self, mode: crate::EnvironmentMode) {
        unsafe {
            ocio_sys::ocio_config_set_environment_mode(self.handle.as_ptr(), mode as i32);
        }
    }

    pub fn environment_mode(&self) -> crate::EnvironmentMode {
        let m = unsafe { ocio_sys::ocio_config_get_environment_mode(self.handle.as_ptr()) };
        match m { 1 => crate::EnvironmentMode::LoadAll, _ => crate::EnvironmentMode::LoadPredefined }
    }

    pub fn load_environment(&self) {
        unsafe { ocio_sys::ocio_config_load_environment(self.handle.as_ptr()) };
    }

    // --- Inactive color spaces ---

    pub fn inactive_color_spaces(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_inactive_color_spaces(self.handle.as_ptr())) }
    }

    pub fn set_inactive_color_spaces(&self, inactive: impl AsRef<str>) -> Result<()> {
        let s = cstring(inactive)?;
        unsafe {
            ocio_sys::ocio_config_set_inactive_color_spaces(self.handle.as_ptr(), s.as_ptr().cast());
        }
        Ok(())
    }

    // --- Archivable ---

    pub fn is_archivable(&self) -> bool {
        unsafe { ocio_sys::ocio_config_is_archivable(self.handle.as_ptr()) }
    }

    // --- Processor cache ---

    pub fn clear_processor_cache(&self) {
        unsafe { ocio_sys::ocio_config_clear_processor_cache(self.handle.as_ptr()) };
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_config_destroy(self.handle.as_ptr()) };
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
    fn processor_from_transform_no_crash() {
        let config = Config::raw().unwrap();
        let ft = crate::transform::FileTransform::create().unwrap();
        let proc = config.processor_from_transform(&ft, TransformDirection::Forward);
        let _ = proc;
    }

    #[test]
    fn get_color_space_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.get_color_space("raw");
    }

    #[test]
    fn add_remove_color_space_no_crash() {
        let config = Config::raw().unwrap();
        let cs = ColorSpace::create().unwrap();
        cs.set_name("TestCS").unwrap();
        config.add_color_space(&cs);
        let _ = config.get_index_for_color_space("TestCS");
        let _ = config.is_color_space_used("TestCS");
        let _ = config.remove_color_space("TestCS");
    }

    #[test]
    fn get_look_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.get_look("look_name");
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
        let _ = config.serialize();
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
        assert!(config.add_display("sRGB", "Film", "DisplayTransform", "srgb").is_ok());
        assert!(config.add_shared_view("sRGB", "SharedView", "TransformName", "srgb").is_ok());
        assert!(config.remove_view("sRGB", "Film").is_ok());
        assert!(config.remove_display("sRGB").is_ok());
    }

    #[test]
    fn named_transforms_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.num_named_transforms();
        let _ = config.named_transform_name_by_index(0);
        let _ = config.get_named_transform("Default");
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
        let _ = config.get_view_transform("Default");
    }

    #[test]
    fn add_remove_view_transform_no_crash() {
        let config = Config::raw().unwrap();
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        config.add_view_transform(&vt);
        assert!(config.remove_view_transform("MyViewTransform").is_ok());
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
    fn default_interpolation_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.default_interpolation();
        config.set_default_interpolation(Interpolation::Linear);
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
    fn color_space_from_filepath_by_ref_type_no_crash() {
        let config = Config::raw().unwrap();
        let _ = config.color_space_from_filepath_by_ref_type("test.jpg", SearchReferenceSpaceType::Scene);
    }

    #[test]
    fn processor_from_configs_no_crash() {
        let src_config = Config::raw().unwrap();
        let dst_config = Config::raw().unwrap();
        // In stub mode, this returns a stub processor (processed through get_processor_from_configs stub)
        let proc = Config::processor_from_configs(&src_config, "raw", &dst_config, "raw");
        let _ = proc;
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
}
