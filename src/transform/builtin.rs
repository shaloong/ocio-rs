use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstr_to_opt_string, cstring, OcioError, Result, TransformDirection};
use ocio_sys;

/// Wraps one of OCIO's named built-in transform styles.
pub struct BuiltinTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl BuiltinTransform {
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_builtin_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn style(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_builtin_transform_get_style(
                self.handle.as_ptr(),
            ))
        }
    }

    pub fn set_style(&self, style: impl AsRef<str>) -> Result<()> {
        let s = cstring(style)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_builtin_transform_set_style(self.handle.as_ptr(), s.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_builtin_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set builtin transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_builtin_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
        crate::ocio_call_status()
    }

    pub fn description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_builtin_transform_get_description(
                self.handle.as_ptr(),
            ))
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }

    // --- Static methods ---

    pub fn num_builtin_styles() -> i32 {
        unsafe { ocio_sys::ocio_builtin_transform_get_num_styles() }
    }

    pub fn builtin_style(index: i32) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_builtin_transform_get_style_by_index(index)) }
    }

    pub fn is_valid_builtin_style(style: impl AsRef<str>) -> bool {
        let s = match crate::cstring(style) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ocio_sys::ocio_builtin_transform_is_valid_style(s.as_ptr().cast()) }
    }
}

impl Drop for BuiltinTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_builtin_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_builtin() {
        let bt = BuiltinTransform::create();
        assert!(bt.is_ok());
    }

    #[test]
    fn style_no_crash() {
        let bt = BuiltinTransform::create().unwrap();
        let _ = bt.style();
        assert!(bt.set_style("ACEScct_to_ACES2065-1").is_ok());
        let invalid = bt.set_style("not-a-real-builtin-style");
        if crate::is_stub_build() {
            assert!(invalid.is_ok());
        } else {
            assert!(invalid.is_err());
        }
    }

    #[test]
    fn direction_no_crash() {
        let bt = BuiltinTransform::create().unwrap();
        let _ = bt.direction();
        bt.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn description_no_crash() {
        let bt = BuiltinTransform::create().unwrap();
        let _ = bt.description();
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let bt = BuiltinTransform::create().unwrap();
        let _ = bt.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let bt = BuiltinTransform::create().unwrap();
        let _ = bt.format_metadata();
    }

    #[test]
    fn num_builtin_styles_no_crash() {
        let n = BuiltinTransform::num_builtin_styles();
        assert!(n >= 0);
    }

    #[test]
    fn builtin_style_no_crash() {
        let n = BuiltinTransform::num_builtin_styles();
        if n > 0 {
            let style = BuiltinTransform::builtin_style(0);
            // In real mode we get a style, in stub mode we get None
            let _ = style;
        }
    }

    #[test]
    fn is_valid_builtin_style_no_crash() {
        let valid = BuiltinTransform::is_valid_builtin_style("ACEScct_to_ACES2065-1");
        let _ = valid;
        let invalid = BuiltinTransform::is_valid_builtin_style("nonexistent_style");
        let _ = invalid;
    }
}
