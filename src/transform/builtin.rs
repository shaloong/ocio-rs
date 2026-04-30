use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, OcioError, Result, TransformDirection};

pub struct BuiltinTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl BuiltinTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_builtin_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn style(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_builtin_transform_get_style(self.handle.as_ptr())) }
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
        unsafe { cstr_to_opt_string(ocio_sys::ocio_builtin_transform_get_description(self.handle.as_ptr())) }
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
}
