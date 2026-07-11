use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{OcioError, Result, TransformDirection};
use ocio_sys;

/// Logarithmic OCIO transform with configurable base and direction.
pub struct LogTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl LogTransform {
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_log_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn base(&self) -> f64 {
        unsafe { ocio_sys::ocio_log_transform_get_base(self.handle.as_ptr()) }
    }

    pub fn set_base(&self, base: f64) {
        self.try_set_base(base)
            .expect("failed to set logarithm base");
    }

    /// Set the logarithm base and surface any OCIO validation error.
    pub fn try_set_base(&self, base: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_log_transform_set_base(self.handle.as_ptr(), base) };
        crate::ocio_call_status()
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_log_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set log transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
        crate::ocio_call_status()
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

    #[deprecated(since = "0.2.0", note = "compat alias; prefer format_metadata()")]
    pub fn format_metadata_v1(&self) -> Option<crate::FormatMetadata> {
        self.format_metadata()
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer format_metadata()")]
    pub fn format_metadata_v2(&self) -> Option<crate::FormatMetadata> {
        self.format_metadata()
    }

    pub fn equals(&self, other: &Self) -> bool {
        unsafe { ocio_sys::ocio_log_transform_equals(self.handle.as_ptr(), other.handle.as_ptr()) }
    }
}

impl Drop for LogTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_log_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_log() {
        let lt = LogTransform::create();
        assert!(lt.is_ok());
    }

    #[test]
    fn base_no_crash() {
        let lt = LogTransform::create().unwrap();
        let _ = lt.base();
        lt.try_set_base(10.0).unwrap();
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let lt = LogTransform::create().unwrap();
        let _ = lt.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let lt = LogTransform::create().unwrap();
        let _ = lt.format_metadata();
    }

    #[test]
    fn equals_no_crash() {
        let a = LogTransform::create().unwrap();
        let b = LogTransform::create().unwrap();
        let _ = a.equals(&b);
    }
}
