use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstr_to_opt_string, cstring, OcioError, Result, TransformDirection};
use ocio_sys;

/// Applies an OCIO look string between source and destination color spaces.
pub struct LookTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl LookTransform {
    /// Create a look transform with default settings.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_look_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the source color space name.
    pub fn src(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_look_transform_get_src(self.handle.as_ptr())) }
    }

    /// Set the source color space name.
    pub fn set_src(&self, src: impl AsRef<str>) -> Result<()> {
        let s = cstring(src)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_look_transform_set_src(self.handle.as_ptr(), s.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Return the destination color space name.
    pub fn dst(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_look_transform_get_dst(self.handle.as_ptr())) }
    }

    /// Set the destination color space name.
    pub fn set_dst(&self, dst: impl AsRef<str>) -> Result<()> {
        let d = cstring(dst)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_look_transform_set_dst(self.handle.as_ptr(), d.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Return the comma-separated look names.
    pub fn looks(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_look_transform_get_looks(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the comma-separated look names.
    pub fn set_looks(&self, looks: impl AsRef<str>) -> Result<()> {
        let l = cstring(looks)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_look_transform_set_looks(self.handle.as_ptr(), l.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Return the evaluation direction.
    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_look_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the evaluation direction, panicking on validation error.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set look transform direction");
    }

    /// Set evaluation direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_look_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
        crate::ocio_call_status()
    }

    /// Create an editable copy that is independent from the original transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_look_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return format metadata attached to the transform, when available.
    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }

    /// Return whether color-space conversion is skipped for this look.
    pub fn skip_color_space_conversion(&self) -> bool {
        unsafe {
            ocio_sys::ocio_look_transform_get_skip_color_space_conversion(self.handle.as_ptr())
        }
    }

    /// Set whether to skip color-space conversion, panicking on validation error.
    pub fn set_skip_color_space_conversion(&self, skip: bool) {
        self.try_set_skip_color_space_conversion(skip)
            .expect("failed to set look color space conversion bypass");
    }

    /// Set color-space conversion bypass and surface any OCIO validation error.
    pub fn try_set_skip_color_space_conversion(&self, skip: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_look_transform_set_skip_color_space_conversion(
                self.handle.as_ptr(),
                skip,
            );
        }
        crate::ocio_call_status()
    }

    /// Validate the transform configuration and return any errors.
    pub fn validate(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_look_transform_validate(self.handle.as_ptr()) };
        crate::validation_status()
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
        t.try_set_direction(TransformDirection::Inverse).unwrap();
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

    #[test]
    fn validate_no_crash() {
        let t = LookTransform::create().unwrap();
        let _ = t.validate();
    }
}
