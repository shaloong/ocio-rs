use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection, NegativeStyle};

pub struct ExponentTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl ExponentTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_exponent_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn value(&self) -> [f64; 4] {
        let mut v = [1.0f64; 4];
        unsafe { ocio_sys::ocio_exponent_transform_get_value(self.handle.as_ptr(), v.as_mut_ptr()) };
        v
    }

    pub fn set_value(&self, vec4: &[f64; 4]) {
        unsafe { ocio_sys::ocio_exponent_transform_set_value(self.handle.as_ptr(), vec4.as_ptr()) };
    }

    pub fn negative_style(&self) -> NegativeStyle {
        let s = unsafe { ocio_sys::ocio_exponent_transform_get_negative_style(self.handle.as_ptr()) };
        match s {
            1 => NegativeStyle::Mirror,
            2 => NegativeStyle::PassThru,
            3 => NegativeStyle::Linear,
            _ => NegativeStyle::Clamp,
        }
    }

    pub fn set_negative_style(&self, style: NegativeStyle) {
        unsafe {
            ocio_sys::ocio_exponent_transform_set_negative_style(self.handle.as_ptr(), style as i32);
        }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_exponent_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_exponent_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }
}

impl Drop for ExponentTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_exponent_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_exponent() {
        let et = ExponentTransform::create();
        assert!(et.is_ok());
    }

    #[test]
    fn value_no_crash() {
        let et = ExponentTransform::create().unwrap();
        let _ = et.value();
        et.set_value(&[2.2, 2.2, 2.2, 1.0]);
    }

    #[test]
    fn negative_style_no_crash() {
        let et = ExponentTransform::create().unwrap();
        let _ = et.negative_style();
        et.set_negative_style(NegativeStyle::Mirror);
    }
}
