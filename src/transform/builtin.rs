use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstr_from_mut, cstring, OcioError, Result, TransformDirection};

pub struct BuiltinTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl BuiltinTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_builtin_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn style(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_builtin_transform_get_style(self.handle.as_ptr())) }
    }

    pub fn set_style(&self, style: impl AsRef<str>) -> Result<()> {
        let s = cstring(style)?;
        unsafe { ocio_sys::ocio_builtin_transform_set_style(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_builtin_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_builtin_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn description(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_builtin_transform_get_description(self.handle.as_ptr())) }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
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
        let s = match crate::cstring(style) { Ok(s) => s, Err(_) => return false };
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
