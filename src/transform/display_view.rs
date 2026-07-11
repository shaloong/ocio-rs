use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstr_to_opt_string, cstring, OcioError, Result, TransformDirection};
use ocio_sys;

/// Converts through an OCIO display/view pipeline.
pub struct DisplayViewTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl DisplayViewTransform {
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_display_view_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn src(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_display_view_transform_get_src(
                self.handle.as_ptr(),
            ))
        }
    }

    pub fn set_src(&self, src: impl AsRef<str>) -> Result<()> {
        let s = cstring(src)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_display_view_transform_set_src(self.handle.as_ptr(), s.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    pub fn display(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_display_view_transform_get_display(
                self.handle.as_ptr(),
            ))
        }
    }

    pub fn set_display(&self, display: impl AsRef<str>) -> Result<()> {
        let d = cstring(display)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_display_view_transform_set_display(
                self.handle.as_ptr(),
                d.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    pub fn view(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_display_view_transform_get_view(
                self.handle.as_ptr(),
            ))
        }
    }

    pub fn set_view(&self, view: impl AsRef<str>) -> Result<()> {
        let v = cstring(view)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_display_view_transform_set_view(self.handle.as_ptr(), v.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    pub fn looks_bypass(&self) -> bool {
        unsafe { ocio_sys::ocio_display_view_transform_get_looks_bypass(self.handle.as_ptr()) }
    }

    pub fn set_looks_bypass(&self, bypass: bool) {
        self.try_set_looks_bypass(bypass)
            .expect("failed to set display view looks bypass");
    }

    /// Set looks bypass behavior and surface any OCIO validation error.
    pub fn try_set_looks_bypass(&self, bypass: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_display_view_transform_set_looks_bypass(self.handle.as_ptr(), bypass)
        };
        crate::ocio_call_status()
    }

    pub fn direction(&self) -> TransformDirection {
        let dir =
            unsafe { ocio_sys::ocio_display_view_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set display view transform direction");
    }

    /// Set evaluation direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_display_view_transform_set_direction(
                self.handle.as_ptr(),
                direction as i32,
            );
        }
        crate::ocio_call_status()
    }

    pub fn data_bypass(&self) -> bool {
        unsafe { ocio_sys::ocio_display_view_transform_get_data_bypass(self.handle.as_ptr()) }
    }

    pub fn set_data_bypass(&self, bypass: bool) {
        self.try_set_data_bypass(bypass)
            .expect("failed to set display view data bypass");
    }

    /// Set data bypass behavior and surface any OCIO validation error.
    pub fn try_set_data_bypass(&self, bypass: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_display_view_transform_set_data_bypass(self.handle.as_ptr(), bypass)
        };
        crate::ocio_call_status()
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_display_view_transform_create_editable_copy(self.handle.as_ptr())
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }

    pub fn validate(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_display_view_transform_validate(self.handle.as_ptr()) };
        crate::validation_status()
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
        t.try_set_direction(TransformDirection::Inverse).unwrap();
    }

    #[test]
    fn looks_bypass_no_crash() {
        let t = DisplayViewTransform::create().unwrap();
        let _ = t.looks_bypass();
        t.try_set_looks_bypass(true).unwrap();
    }

    #[test]
    fn data_bypass_no_crash() {
        let t = DisplayViewTransform::create().unwrap();
        let _ = t.data_bypass();
        t.try_set_data_bypass(true).unwrap();
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

    #[test]
    fn validate_no_crash() {
        let t = DisplayViewTransform::create().unwrap();
        let _ = t.validate();
    }
}
