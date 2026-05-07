use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection};

pub struct LogCameraTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl LogCameraTransform {
    pub fn create(lin_side_break_values: &[f64; 3]) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_log_camera_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn base(&self) -> f64 {
        unsafe { ocio_sys::ocio_log_camera_transform_get_base(self.handle.as_ptr()) }
    }

    pub fn set_base(&self, base: f64) {
        unsafe { ocio_sys::ocio_log_camera_transform_set_base(self.handle.as_ptr(), base); }
    }

    pub fn log_side_slope_value(&self) -> [f64; 3] {
        let mut v = [0.0f64; 3];
        unsafe { ocio_sys::ocio_log_camera_transform_get_log_side_slope_value(self.handle.as_ptr(), v.as_mut_ptr()); }
        v
    }

    pub fn set_log_side_slope_value(&self, values: &[f64; 3]) {
        unsafe { ocio_sys::ocio_log_camera_transform_set_log_side_slope_value(self.handle.as_ptr(), values.as_ptr()); }
    }

    pub fn log_side_offset_value(&self) -> [f64; 3] {
        let mut v = [0.0f64; 3];
        unsafe { ocio_sys::ocio_log_camera_transform_get_log_side_offset_value(self.handle.as_ptr(), v.as_mut_ptr()); }
        v
    }

    pub fn set_log_side_offset_value(&self, values: &[f64; 3]) {
        unsafe { ocio_sys::ocio_log_camera_transform_set_log_side_offset_value(self.handle.as_ptr(), values.as_ptr()); }
    }

    pub fn lin_side_slope_value(&self) -> [f64; 3] {
        let mut v = [0.0f64; 3];
        unsafe { ocio_sys::ocio_log_camera_transform_get_lin_side_slope_value(self.handle.as_ptr(), v.as_mut_ptr()); }
        v
    }

    pub fn set_lin_side_slope_value(&self, values: &[f64; 3]) {
        unsafe { ocio_sys::ocio_log_camera_transform_set_lin_side_slope_value(self.handle.as_ptr(), values.as_ptr()); }
    }

    pub fn lin_side_offset_value(&self) -> [f64; 3] {
        let mut v = [0.0f64; 3];
        unsafe { ocio_sys::ocio_log_camera_transform_get_lin_side_offset_value(self.handle.as_ptr(), v.as_mut_ptr()); }
        v
    }

    pub fn set_lin_side_offset_value(&self, values: &[f64; 3]) {
        unsafe { ocio_sys::ocio_log_camera_transform_set_lin_side_offset_value(self.handle.as_ptr(), values.as_ptr()); }
    }

    pub fn lin_side_break_value(&self) -> [f64; 3] {
        let mut v = [0.0f64; 3];
        unsafe { ocio_sys::ocio_log_camera_transform_get_lin_side_break_value(self.handle.as_ptr(), v.as_mut_ptr()); }
        v
    }

    pub fn set_lin_side_break_value(&self, values: &[f64; 3]) {
        unsafe { ocio_sys::ocio_log_camera_transform_set_lin_side_break_value(self.handle.as_ptr(), values.as_ptr()); }
    }

    pub fn linear_slope_value(&self) -> Option<[f64; 3]> {
        let mut v = [0.0f64; 3];
        let ok = unsafe { ocio_sys::ocio_log_camera_transform_get_linear_slope_value(self.handle.as_ptr(), v.as_mut_ptr()) };
        if ok { Some(v) } else { None }
    }

    pub fn set_linear_slope_value(&self, values: &[f64; 3]) {
        unsafe { ocio_sys::ocio_log_camera_transform_set_linear_slope_value(self.handle.as_ptr(), values.as_ptr()); }
    }

    pub fn unset_linear_slope_value(&self) {
        unsafe { ocio_sys::ocio_log_camera_transform_unset_linear_slope_value(self.handle.as_ptr()); }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_log_camera_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_log_camera_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }
}

impl Drop for LogCameraTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_log_camera_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_log_camera_transform() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]);
        assert!(t.is_ok());
    }

    #[test]
    fn log_camera_transform_methods_no_crash() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        let _ = t.base();
        let _ = t.log_side_slope_value();
        let _ = t.log_side_offset_value();
        let _ = t.lin_side_slope_value();
        let _ = t.lin_side_offset_value();
        let _ = t.lin_side_break_value();
        let _ = t.linear_slope_value();
    }

    #[test]
    fn set_values_no_crash() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        t.set_base(10.0);
        t.set_log_side_slope_value(&[1.0, 1.0, 1.0]);
        t.set_log_side_offset_value(&[0.0, 0.0, 0.0]);
        t.set_lin_side_slope_value(&[1.0, 1.0, 1.0]);
        t.set_lin_side_offset_value(&[0.0, 0.0, 0.0]);
        t.set_lin_side_break_value(&[0.01, 0.01, 0.01]);
        t.set_linear_slope_value(&[1.0, 1.0, 1.0]);
        t.unset_linear_slope_value();
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn direction_no_crash() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        let _ = t.direction();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        let _ = t.format_metadata();
    }
}
