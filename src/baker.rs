use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, Config, FormatMetadata, OcioError, Result};

pub struct Baker {
    handle: NonNull<c_void>,
}

impl Baker {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_baker_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_baker_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn set_config(&self, config: &Config) {
        unsafe {
            ocio_sys::ocio_baker_set_config(self.handle.as_ptr(), config.handle.as_ptr());
        }
    }

    pub fn config(&self) -> Result<Config> {
        let handle = unsafe { ocio_sys::ocio_baker_get_config(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Config { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn format(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_baker_get_format(self.handle.as_ptr())) }
    }

    pub fn set_format(&self, format_name: impl AsRef<str>) -> Result<()> {
        let name = cstring(format_name)?;
        unsafe { ocio_sys::ocio_baker_set_format(self.handle.as_ptr(), name.as_ptr().cast()) };
        Ok(())
    }

    pub fn input_space(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_baker_get_input_space(self.handle.as_ptr())) }
    }

    pub fn set_input_space(&self, space: impl AsRef<str>) -> Result<()> {
        let s = cstring(space)?;
        unsafe { ocio_sys::ocio_baker_set_input_space(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn shaper_space(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_baker_get_shaper_space(self.handle.as_ptr())) }
    }

    pub fn set_shaper_space(&self, space: impl AsRef<str>) -> Result<()> {
        let s = cstring(space)?;
        unsafe { ocio_sys::ocio_baker_set_shaper_space(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn looks(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_baker_get_looks(self.handle.as_ptr())) }
    }

    pub fn set_looks(&self, looks: impl AsRef<str>) -> Result<()> {
        let s = cstring(looks)?;
        unsafe { ocio_sys::ocio_baker_set_looks(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn target_space(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_baker_get_target_space(self.handle.as_ptr())) }
    }

    pub fn set_target_space(&self, space: impl AsRef<str>) -> Result<()> {
        let s = cstring(space)?;
        unsafe { ocio_sys::ocio_baker_set_target_space(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn display(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_baker_get_display(self.handle.as_ptr())) }
    }

    pub fn view(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_baker_get_view(self.handle.as_ptr())) }
    }

    pub fn set_display_view(&self, display: impl AsRef<str>, view: impl AsRef<str>) -> Result<()> {
        let d = cstring(display)?;
        let v = cstring(view)?;
        unsafe {
            ocio_sys::ocio_baker_set_display_view(
                self.handle.as_ptr(), d.as_ptr().cast(), v.as_ptr().cast(),
            );
        }
        Ok(())
    }

    pub fn shaper_size(&self) -> i32 {
        unsafe { ocio_sys::ocio_baker_get_shaper_size(self.handle.as_ptr()) }
    }

    pub fn set_shaper_size(&self, size: i32) {
        unsafe { ocio_sys::ocio_baker_set_shaper_size(self.handle.as_ptr(), size) };
    }

    pub fn cube_size(&self) -> i32 {
        unsafe { ocio_sys::ocio_baker_get_cube_size(self.handle.as_ptr()) }
    }

    pub fn set_cube_size(&self, size: i32) {
        unsafe { ocio_sys::ocio_baker_set_cube_size(self.handle.as_ptr(), size) };
    }

    pub fn bake(&self, output_path: impl AsRef<str>) -> Result<()> {
        let path = cstring(output_path)?;
        unsafe { ocio_sys::ocio_baker_bake(self.handle.as_ptr(), path.as_ptr().cast()) };
        Ok(())
    }

    pub fn format_metadata(&self) -> Option<FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_baker_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| FormatMetadata { handle: h })
    }
}

impl Drop for Baker {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_baker_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_baker() {
        let baker = Baker::create();
        assert!(baker.is_ok());
    }

    #[test]
    fn baker_methods_no_crash() {
        let baker = Baker::create().unwrap();
        let _ = baker.format();
        let _ = baker.input_space();
        let _ = baker.target_space();
        let _ = baker.shaper_size();
        let _ = baker.cube_size();
    }

    #[test]
    fn format_metadata_no_crash() {
        let baker = Baker::create().unwrap();
        let md = baker.format_metadata();
        assert!(md.is_some());
    }
}
