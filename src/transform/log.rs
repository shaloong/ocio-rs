use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection};

pub struct LogTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl LogTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_log_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn base(&self) -> f64 {
        unsafe { ocio_sys::ocio_log_transform_get_base(self.handle.as_ptr()) }
    }

    pub fn set_base(&self, base: f64) {
        unsafe { ocio_sys::ocio_log_transform_set_base(self.handle.as_ptr(), base) };
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_log_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_log_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
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
        lt.set_base(10.0);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let lt = LogTransform::create().unwrap();
        let _ = lt.create_editable_copy();
    }
}
