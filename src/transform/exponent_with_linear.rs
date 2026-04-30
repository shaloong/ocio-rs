use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection, NegativeStyle};

pub struct ExponentWithLinearTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl ExponentWithLinearTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_exponent_with_linear_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn value(&self) -> [f64; 4] {
        let mut vec4 = [1.0f64; 4];
        unsafe {
            ocio_sys::ocio_exponent_with_linear_transform_get_value(self.handle.as_ptr(), vec4.as_mut_ptr());
        }
        vec4
    }

    pub fn set_value(&self, vec4: &[f64; 4]) {
        unsafe {
            ocio_sys::ocio_exponent_with_linear_transform_set_value(self.handle.as_ptr(), vec4.as_ptr());
        }
    }

    pub fn negative_style(&self) -> NegativeStyle {
        let s = unsafe { ocio_sys::ocio_exponent_with_linear_transform_get_negative_style(self.handle.as_ptr()) };
        match s {
            1 => NegativeStyle::Mirror,
            2 => NegativeStyle::PassThru,
            3 => NegativeStyle::Linear,
            _ => NegativeStyle::Clamp,
        }
    }

    pub fn set_negative_style(&self, style: NegativeStyle) {
        unsafe {
            ocio_sys::ocio_exponent_with_linear_transform_set_negative_style(self.handle.as_ptr(), style as i32);
        }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_exponent_with_linear_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_exponent_with_linear_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }
}

impl Drop for ExponentWithLinearTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_exponent_with_linear_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_exponent_with_linear() {
        let t = ExponentWithLinearTransform::create();
        assert!(t.is_ok());
    }

    #[test]
    fn value_no_crash() {
        let t = ExponentWithLinearTransform::create().unwrap();
        let _ = t.value();
        t.set_value(&[1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn negative_style_no_crash() {
        let t = ExponentWithLinearTransform::create().unwrap();
        let _ = t.negative_style();
        t.set_negative_style(NegativeStyle::Mirror);
    }

    #[test]
    fn direction_no_crash() {
        let t = ExponentWithLinearTransform::create().unwrap();
        let _ = t.direction();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = ExponentWithLinearTransform::create().unwrap();
        let _ = t.create_editable_copy();
    }
}
