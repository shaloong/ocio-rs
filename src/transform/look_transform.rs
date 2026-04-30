use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, OcioError, Result, TransformDirection};

pub struct LookTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl LookTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_look_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn src(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_look_transform_get_src(self.handle.as_ptr())) }
    }

    pub fn set_src(&self, src: impl AsRef<str>) -> Result<()> {
        let s = cstring(src)?;
        unsafe { ocio_sys::ocio_look_transform_set_src(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn dst(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_look_transform_get_dst(self.handle.as_ptr())) }
    }

    pub fn set_dst(&self, dst: impl AsRef<str>) -> Result<()> {
        let d = cstring(dst)?;
        unsafe { ocio_sys::ocio_look_transform_set_dst(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn looks(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_look_transform_get_looks(self.handle.as_ptr())) }
    }

    pub fn set_looks(&self, looks: impl AsRef<str>) -> Result<()> {
        let l = cstring(looks)?;
        unsafe { ocio_sys::ocio_look_transform_set_looks(self.handle.as_ptr(), l.as_ptr().cast()) };
        Ok(())
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_look_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_look_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }

    pub fn skip_color_space_conversion(&self) -> bool {
        unsafe { ocio_sys::ocio_look_transform_get_skip_color_space_conversion(self.handle.as_ptr()) }
    }

    pub fn set_skip_color_space_conversion(&self, skip: bool) {
        unsafe {
            ocio_sys::ocio_look_transform_set_skip_color_space_conversion(self.handle.as_ptr(), skip);
        }
    }
}

impl Drop for LookTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_look_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_look_transform() {
        let t = LookTransform::create();
        assert!(t.is_ok());
    }

    #[test]
    fn look_transform_methods_no_crash() {
        let t = LookTransform::create().unwrap();
        let _ = t.src();
        let _ = t.dst();
        let _ = t.looks();
        let _ = t.direction();
    }

    #[test]
    fn set_src_dst_looks_no_crash() {
        let t = LookTransform::create().unwrap();
        assert!(t.set_src("ACEScg").is_ok());
        assert!(t.set_dst("Output").is_ok());
        assert!(t.set_looks("look1,look2").is_ok());
    }

    #[test]
    fn direction_no_crash() {
        let t = LookTransform::create().unwrap();
        let _ = t.direction();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = LookTransform::create().unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn skip_cs_conversion_no_crash() {
        let t = LookTransform::create().unwrap();
        let _ = t.skip_color_space_conversion();
        t.set_skip_color_space_conversion(true);
        t.set_skip_color_space_conversion(false);
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = LookTransform::create().unwrap();
        let _ = t.format_metadata();
    }
}
