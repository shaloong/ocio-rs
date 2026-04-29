use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection, RangeStyle};

pub struct RangeTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl RangeTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_range_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn style(&self) -> RangeStyle {
        let s = unsafe { ocio_sys::ocio_range_transform_get_style(self.handle.as_ptr()) };
        match s { 1 => RangeStyle::Clamp, _ => RangeStyle::NoClamp }
    }

    pub fn set_style(&self, style: RangeStyle) {
        unsafe { ocio_sys::ocio_range_transform_set_style(self.handle.as_ptr(), style as i32) };
    }

    pub fn min_in_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_min_in_value(self.handle.as_ptr()) }
    }

    pub fn set_min_in_value(&self, value: f64) {
        unsafe { ocio_sys::ocio_range_transform_set_min_in_value(self.handle.as_ptr(), value) };
    }

    pub fn max_in_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_max_in_value(self.handle.as_ptr()) }
    }

    pub fn set_max_in_value(&self, value: f64) {
        unsafe { ocio_sys::ocio_range_transform_set_max_in_value(self.handle.as_ptr(), value) };
    }

    pub fn min_out_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_min_out_value(self.handle.as_ptr()) }
    }

    pub fn set_min_out_value(&self, value: f64) {
        unsafe { ocio_sys::ocio_range_transform_set_min_out_value(self.handle.as_ptr(), value) };
    }

    pub fn max_out_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_max_out_value(self.handle.as_ptr()) }
    }

    pub fn set_max_out_value(&self, value: f64) {
        unsafe { ocio_sys::ocio_range_transform_set_max_out_value(self.handle.as_ptr(), value) };
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_range_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_range_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }
}

impl Drop for RangeTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_range_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_range() {
        let rt = RangeTransform::create();
        assert!(rt.is_ok());
    }

    #[test]
    fn range_values_no_crash() {
        let rt = RangeTransform::create().unwrap();
        let _ = rt.min_in_value();
        let _ = rt.max_in_value();
        let _ = rt.min_out_value();
        let _ = rt.max_out_value();
        rt.set_min_in_value(0.1);
        rt.set_max_in_value(0.9);
        rt.set_min_out_value(0.05);
        rt.set_max_out_value(0.95);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let rt = RangeTransform::create().unwrap();
        let _ = rt.create_editable_copy();
    }
}
