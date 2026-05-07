use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstr_from_mut, cstring, OcioError, Result, TransformDirection};

pub struct DisplayViewTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl DisplayViewTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_display_view_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn src(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_display_view_transform_get_src(self.handle.as_ptr())) }
    }

    pub fn set_src(&self, src: impl AsRef<str>) -> Result<()> {
        let s = cstring(src)?;
        unsafe { ocio_sys::ocio_display_view_transform_set_src(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn display(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_display_view_transform_get_display(self.handle.as_ptr())) }
    }

    pub fn set_display(&self, display: impl AsRef<str>) -> Result<()> {
        let d = cstring(display)?;
        unsafe { ocio_sys::ocio_display_view_transform_set_display(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn view(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_display_view_transform_get_view(self.handle.as_ptr())) }
    }

    pub fn set_view(&self, view: impl AsRef<str>) -> Result<()> {
        let v = cstring(view)?;
        unsafe { ocio_sys::ocio_display_view_transform_set_view(self.handle.as_ptr(), v.as_ptr().cast()) };
        Ok(())
    }

    pub fn looks_bypass(&self) -> bool {
        unsafe { ocio_sys::ocio_display_view_transform_get_looks_bypass(self.handle.as_ptr()) }
    }

    pub fn set_looks_bypass(&self, bypass: bool) {
        unsafe { ocio_sys::ocio_display_view_transform_set_looks_bypass(self.handle.as_ptr(), bypass) };
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_display_view_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_display_view_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn data_bypass(&self) -> bool {
        unsafe { ocio_sys::ocio_display_view_transform_get_data_bypass(self.handle.as_ptr()) }
    }

    pub fn set_data_bypass(&self, bypass: bool) {
        unsafe { ocio_sys::ocio_display_view_transform_set_data_bypass(self.handle.as_ptr(), bypass) };
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }
}

impl Drop for DisplayViewTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_display_view_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_display_view_transform() {
        let t = DisplayViewTransform::create();
        assert!(t.is_ok());
    }

    #[test]
    fn set_src_display_view_no_crash() {
        let t = DisplayViewTransform::create().unwrap();
        assert!(t.set_src("src").is_ok());
        let _ = t.src();
        assert!(t.set_display("display").is_ok());
        let _ = t.display();
        assert!(t.set_view("view").is_ok());
        let _ = t.view();
    }

    #[test]
    fn direction_no_crash() {
        let t = DisplayViewTransform::create().unwrap();
        let _ = t.direction();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn looks_bypass_no_crash() {
        let t = DisplayViewTransform::create().unwrap();
        let _ = t.looks_bypass();
        t.set_looks_bypass(true);
    }

    #[test]
    fn data_bypass_no_crash() {
        let t = DisplayViewTransform::create().unwrap();
        let _ = t.data_bypass();
        t.set_data_bypass(true);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = DisplayViewTransform::create().unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = DisplayViewTransform::create().unwrap();
        let _ = t.format_metadata();
    }
}
