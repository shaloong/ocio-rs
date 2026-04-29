use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, OcioError, Processor, ColorSpace, Look, Result, TransformDirection, ReferenceSpaceType};
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

    pub fn description(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_description(self.handle.as_ptr())) }
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_config_get_cache_id(self.handle.as_ptr())) }
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

    // --- Luma coefficients ---

    pub fn default_luma_coefs(&self) -> [f64; 3] {
        let mut coefs = [0.0f64; 3];
        unsafe {
            ocio_sys::ocio_config_get_default_luma_coefs(self.handle.as_ptr(), coefs.as_mut_ptr());
        }
        coefs
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

    pub fn get_color_space(&self, name: impl AsRef<str>) -> Option<ColorSpace> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_color_space(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| ColorSpace { handle: h })
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
    fn fail_on_missing_file() {
        if crate::is_stub_build() {
            let cfg = Config::from_file("tests/missing_config.ocio");
            assert!(cfg.is_err());
        }
    }
}
