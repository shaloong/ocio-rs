use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result};

pub struct LogAffineTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl LogAffineTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_log_affine_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn base(&self) -> f64 {
        unsafe { ocio_sys::ocio_log_affine_transform_get_base(self.handle.as_ptr()) }
    }

    pub fn set_base(&self, base: f64) {
        unsafe { ocio_sys::ocio_log_affine_transform_set_base(self.handle.as_ptr(), base); }
    }

    pub fn log_side_slope_value(&self) -> [f64; 3] {
        let mut v = [0.0f64; 3];
        unsafe { ocio_sys::ocio_log_affine_transform_get_log_side_slope_value(self.handle.as_ptr(), v.as_mut_ptr()); }
        v
    }

    pub fn set_log_side_slope_value(&self, values: &[f64; 3]) {
        unsafe { ocio_sys::ocio_log_affine_transform_set_log_side_slope_value(self.handle.as_ptr(), values.as_ptr()); }
    }

    pub fn log_side_offset_value(&self) -> [f64; 3] {
        let mut v = [0.0f64; 3];
        unsafe { ocio_sys::ocio_log_affine_transform_get_log_side_offset_value(self.handle.as_ptr(), v.as_mut_ptr()); }
        v
    }

    pub fn set_log_side_offset_value(&self, values: &[f64; 3]) {
        unsafe { ocio_sys::ocio_log_affine_transform_set_log_side_offset_value(self.handle.as_ptr(), values.as_ptr()); }
    }

    pub fn lin_side_slope_value(&self) -> [f64; 3] {
        let mut v = [0.0f64; 3];
        unsafe { ocio_sys::ocio_log_affine_transform_get_lin_side_slope_value(self.handle.as_ptr(), v.as_mut_ptr()); }
        v
    }

    pub fn set_lin_side_slope_value(&self, values: &[f64; 3]) {
        unsafe { ocio_sys::ocio_log_affine_transform_set_lin_side_slope_value(self.handle.as_ptr(), values.as_ptr()); }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn lin_side_offset_value(&self) -> [f64; 3] {
        let mut v = [0.0f64; 3];
        unsafe { ocio_sys::ocio_log_affine_transform_get_lin_side_offset_value(self.handle.as_ptr(), v.as_mut_ptr()); }
        v
    }

    pub fn set_lin_side_offset_value(&self, values: &[f64; 3]) {
        unsafe { ocio_sys::ocio_log_affine_transform_set_lin_side_offset_value(self.handle.as_ptr(), values.as_ptr()); }
    }
}

impl Drop for LogAffineTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_log_affine_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_log_affine_transform() {
        let t = LogAffineTransform::create();
        assert!(t.is_ok());
    }

    #[test]
    fn log_affine_transform_methods_no_crash() {
        let t = LogAffineTransform::create().unwrap();
        let _ = t.base();
        let _ = t.log_side_slope_value();
        let _ = t.log_side_offset_value();
        let _ = t.lin_side_slope_value();
        let _ = t.lin_side_offset_value();
    }

    #[test]
    fn set_values_no_crash() {
        let t = LogAffineTransform::create().unwrap();
        t.set_base(10.0);
        t.set_log_side_slope_value(&[1.0, 1.0, 1.0]);
        t.set_log_side_offset_value(&[0.0, 0.0, 0.0]);
        t.set_lin_side_slope_value(&[1.0, 1.0, 1.0]);
        t.set_lin_side_offset_value(&[0.0, 0.0, 0.0]);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = LogAffineTransform::create().unwrap();
        let _ = t.create_editable_copy();
    }
}
