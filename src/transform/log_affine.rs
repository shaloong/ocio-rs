use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{Result, TransformDirection};
use ocio_sys;

/// Affine logarithmic OCIO transform with per-channel offsets and slopes.
pub struct LogAffineTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl LogAffineTransform {
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_log_affine_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn base(&self) -> Result<f64> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_log_affine_transform_get_base(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    pub fn set_base(&self, base: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_affine_transform_set_base(self.handle.as_ptr(), base);
        }
        crate::ocio_call_status()
    }

    pub fn log_side_slope_value(&self) -> Result<[f64; 3]> {
        let mut v = [0.0f64; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_affine_transform_get_log_side_slope_value(
                self.handle.as_ptr(),
                v.as_mut_ptr(),
            );
        }
        crate::ocio_call_status()?;
        Ok(v)
    }

    pub fn set_log_side_slope_value(&self, values: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_affine_transform_set_log_side_slope_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn log_side_offset_value(&self) -> Result<[f64; 3]> {
        let mut v = [0.0f64; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_affine_transform_get_log_side_offset_value(
                self.handle.as_ptr(),
                v.as_mut_ptr(),
            );
        }
        crate::ocio_call_status()?;
        Ok(v)
    }

    pub fn set_log_side_offset_value(&self, values: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_affine_transform_set_log_side_offset_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn lin_side_slope_value(&self) -> Result<[f64; 3]> {
        let mut v = [0.0f64; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_affine_transform_get_lin_side_slope_value(
                self.handle.as_ptr(),
                v.as_mut_ptr(),
            );
        }
        crate::ocio_call_status()?;
        Ok(v)
    }

    pub fn set_lin_side_slope_value(&self, values: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_affine_transform_set_lin_side_slope_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn direction(&self) -> TransformDirection {
        let dir =
            unsafe { ocio_sys::ocio_log_affine_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set log-affine transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_affine_transform_set_direction(
                self.handle.as_ptr(),
                direction as i32,
            );
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

    pub fn lin_side_offset_value(&self) -> Result<[f64; 3]> {
        let mut v = [0.0f64; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_affine_transform_get_lin_side_offset_value(
                self.handle.as_ptr(),
                v.as_mut_ptr(),
            );
        }
        crate::ocio_call_status()?;
        Ok(v)
    }

    pub fn set_lin_side_offset_value(&self, values: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_affine_transform_set_lin_side_offset_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn equals(&self, other: &Self) -> bool {
        unsafe {
            ocio_sys::ocio_log_affine_transform_equals(self.handle.as_ptr(), other.handle.as_ptr())
        }
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
        let _ = t.set_base(10.0);
        let _ = t.set_log_side_slope_value(&[1.0, 1.0, 1.0]);
        let _ = t.set_log_side_offset_value(&[0.0, 0.0, 0.0]);
        let _ = t.set_lin_side_slope_value(&[1.0, 1.0, 1.0]);
        let _ = t.set_lin_side_offset_value(&[0.0, 0.0, 0.0]);
    }

    #[test]
    fn direction_no_crash() {
        let t = LogAffineTransform::create().unwrap();
        let _ = t.direction();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = LogAffineTransform::create().unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = LogAffineTransform::create().unwrap();
        let _ = t.format_metadata();
    }

    #[test]
    fn equals_no_crash() {
        let a = LogAffineTransform::create().unwrap();
        let b = LogAffineTransform::create().unwrap();
        let _ = a.equals(&b);
    }
}
