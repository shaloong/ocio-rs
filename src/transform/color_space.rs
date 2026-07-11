use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstr_to_opt_string, cstring, OcioError, Result, TransformDirection};
use ocio_sys;

/// Converts between two named color spaces inside a config.
pub struct ColorSpaceTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl ColorSpaceTransform {
    /// Create a color-space transform with default source and destination.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_color_space_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the source color space name.
    pub fn src(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_transform_get_src(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the source color space name.
    pub fn set_src(&self, src: impl AsRef<str>) -> Result<()> {
        let s = cstring(src)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_transform_set_src(self.handle.as_ptr(), s.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Return the destination color space name.
    pub fn dst(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_transform_get_dst(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the destination color space name.
    pub fn set_dst(&self, dst: impl AsRef<str>) -> Result<()> {
        let d = cstring(dst)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_transform_set_dst(self.handle.as_ptr(), d.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Return whether data channel bypass is enabled.
    pub fn data_bypass(&self) -> bool {
        unsafe { ocio_sys::ocio_color_space_transform_get_data_bypass(self.handle.as_ptr()) }
    }

    /// Set data bypass behavior, panicking on validation error.
    pub fn set_data_bypass(&self, bypass: bool) {
        self.try_set_data_bypass(bypass)
            .expect("failed to set color space data bypass");
    }

    /// Set data bypass behavior and surface any OCIO validation error.
    pub fn try_set_data_bypass(&self, bypass: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_transform_set_data_bypass(self.handle.as_ptr(), bypass)
        };
        crate::ocio_call_status()
    }

    /// Return the evaluation direction.
    pub fn direction(&self) -> TransformDirection {
        let dir =
            unsafe { ocio_sys::ocio_color_space_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the evaluation direction, panicking on validation error.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set color space transform direction");
    }

    /// Set evaluation direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_transform_set_direction(
                self.handle.as_ptr(),
                direction as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Create an editable copy that is independent from the original transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_color_space_transform_create_editable_copy(self.handle.as_ptr())
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return format metadata attached to the transform, when available.
    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }

    /// Validate the transform configuration and return any errors.
    pub fn validate(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_transform_validate(self.handle.as_ptr()) };
        crate::validation_status()
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
        t.try_set_data_bypass(true).unwrap();
    }

    #[test]
    fn direction_no_crash() {
        let t = ColorSpaceTransform::create().unwrap();
        let _ = t.direction();
        t.try_set_direction(TransformDirection::Inverse).unwrap();
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = ColorSpaceTransform::create().unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = ColorSpaceTransform::create().unwrap();
        let _ = t.format_metadata();
    }

    #[test]
    fn validate_no_crash() {
        let t = ColorSpaceTransform::create().unwrap();
        let _ = t.validate();
    }
}
