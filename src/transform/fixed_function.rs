use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection, FixedFunctionStyle};

pub struct FixedFunctionTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl FixedFunctionTransform {
    pub fn create(style: FixedFunctionStyle) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_fixed_function_transform_create(style as i32) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn create_with_params(style: FixedFunctionStyle, params: &[f64]) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_fixed_function_transform_create_with_params(
                style as i32, params.as_ptr(), params.len() as i32,
            )
        };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn style(&self) -> FixedFunctionStyle {
        let s = unsafe { ocio_sys::ocio_fixed_function_transform_get_style(self.handle.as_ptr()) };
        match s {
            1 => FixedFunctionStyle::AcesRedMod10,
            2 => FixedFunctionStyle::AcesGlow03,
            3 => FixedFunctionStyle::AcesGlow10,
            4 => FixedFunctionStyle::AcesGamutCompress13,
            5 => FixedFunctionStyle::AcesGamutCompress20,
            6 => FixedFunctionStyle::Rec2100Surround,
            7 => FixedFunctionStyle::RgbToHsv,
            8 => FixedFunctionStyle::XyzToxyY,
            9 => FixedFunctionStyle::XyzTouvY,
            10 => FixedFunctionStyle::XyzToLuv,
            _ => FixedFunctionStyle::AcesRedMod03,
        }
    }

    pub fn set_style(&self, style: FixedFunctionStyle) {
        unsafe {
            ocio_sys::ocio_fixed_function_transform_set_style(self.handle.as_ptr(), style as i32);
        }
    }

    pub fn num_params(&self) -> i32 {
        unsafe { ocio_sys::ocio_fixed_function_transform_get_num_params(self.handle.as_ptr()) }
    }

    pub fn params(&self) -> Vec<f64> {
        let n = self.num_params();
        if n <= 0 {
            return Vec::new();
        }
        let mut params = vec![0.0f64; n as usize];
        unsafe {
            ocio_sys::ocio_fixed_function_transform_get_params(
                self.handle.as_ptr(), params.as_mut_ptr(),
            );
        }
        params
    }

    pub fn set_params(&self, params: &[f64]) {
        unsafe {
            ocio_sys::ocio_fixed_function_transform_set_params(
                self.handle.as_ptr(), params.as_ptr(), params.len() as i32,
            );
        }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_fixed_function_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_fixed_function_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }
}

impl Drop for FixedFunctionTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_fixed_function_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_fixed_function() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03);
        assert!(ft.is_ok());
    }

    #[test]
    fn style_no_crash() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let _ = ft.style();
        ft.set_style(FixedFunctionStyle::RgbToHsv);
    }

    #[test]
    fn params_no_crash() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let _ = ft.num_params();
        let _ = ft.params();
        ft.set_params(&[1.0, 2.0, 3.0]);
    }

    #[test]
    fn direction_no_crash() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let _ = ft.direction();
        ft.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_with_params() {
        let ft = FixedFunctionTransform::create_with_params(
            FixedFunctionStyle::AcesRedMod03, &[1.0, 2.0],
        );
        assert!(ft.is_ok());
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let _ = ft.create_editable_copy();
    }
}
