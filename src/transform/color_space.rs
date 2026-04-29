use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, OcioError, Result, TransformDirection};

pub struct ColorSpaceTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl ColorSpaceTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_color_space_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn src(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_transform_get_src(self.handle.as_ptr())) }
    }

    pub fn set_src(&self, src: impl AsRef<str>) -> Result<()> {
        let s = cstring(src)?;
        unsafe { ocio_sys::ocio_color_space_transform_set_src(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn dst(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_transform_get_dst(self.handle.as_ptr())) }
    }

    pub fn set_dst(&self, dst: impl AsRef<str>) -> Result<()> {
        let d = cstring(dst)?;
        unsafe { ocio_sys::ocio_color_space_transform_set_dst(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn data_bypass(&self) -> bool {
        unsafe { ocio_sys::ocio_color_space_transform_get_data_bypass(self.handle.as_ptr()) }
    }

    pub fn set_data_bypass(&self, bypass: bool) {
        unsafe { ocio_sys::ocio_color_space_transform_set_data_bypass(self.handle.as_ptr(), bypass) };
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_color_space_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_color_space_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }
}

impl Drop for ColorSpaceTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_color_space_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_color_space_transform() {
        let t = ColorSpaceTransform::create();
        assert!(t.is_ok());
    }

    #[test]
    fn color_space_transform_methods_no_crash() {
        let t = ColorSpaceTransform::create().unwrap();
        let _ = t.src();
        let _ = t.dst();
        let _ = t.data_bypass();
        let _ = t.direction();
    }

    #[test]
    fn set_src_dst_no_crash() {
        let t = ColorSpaceTransform::create().unwrap();
        assert!(t.set_src("ACEScg").is_ok());
        assert!(t.set_dst("Output").is_ok());
    }

    #[test]
    fn set_data_bypass_no_crash() {
        let t = ColorSpaceTransform::create().unwrap();
        t.set_data_bypass(true);
    }

    #[test]
    fn direction_no_crash() {
        let t = ColorSpaceTransform::create().unwrap();
        let _ = t.direction();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = ColorSpaceTransform::create().unwrap();
        let _ = t.create_editable_copy();
    }
}
