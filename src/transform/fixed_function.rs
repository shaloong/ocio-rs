use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{FixedFunctionStyle, OcioError, Result, TransformDirection};
use ocio_sys;

/// Wraps one of OCIO's fixed-function transform styles.
pub struct FixedFunctionTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl FixedFunctionTransform {
    pub fn create(style: FixedFunctionStyle) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_fixed_function_transform_create_with_params(
                style as i32,
                std::ptr::null(),
                0,
            )
        };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn create_with_params(style: FixedFunctionStyle, params: &[f64]) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_fixed_function_transform_create_with_params(
                style as i32,
                params.as_ptr(),
                params.len(),
            )
        };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn style(&self) -> FixedFunctionStyle {
        let s = unsafe {
            ocio_sys::ocio_fixed_function_transform_get_style(self.handle.as_ptr() as *mut c_void)
        };
        match s {
            1 => FixedFunctionStyle::AcesRedMod10,
            2 => FixedFunctionStyle::AcesGlow03,
            3 => FixedFunctionStyle::AcesGlow10,
            4 => FixedFunctionStyle::AcesDarkToDim10,
            5 => FixedFunctionStyle::Rec2100Surround,
            6 => FixedFunctionStyle::RgbToHsv,
            7 => FixedFunctionStyle::XyzToxyY,
            8 => FixedFunctionStyle::XyzTouvY,
            9 => FixedFunctionStyle::XyzToLuv,
            10 => FixedFunctionStyle::AcesGamutMap02,
            11 => FixedFunctionStyle::AcesGamutMap07,
            12 => FixedFunctionStyle::AcesGamutCompress13,
            13 => FixedFunctionStyle::LinToPq,
            14 => FixedFunctionStyle::LinToGammaLog,
            15 => FixedFunctionStyle::LinToDoubleLog,
            16 => FixedFunctionStyle::AcesOutputTransform20,
            17 => FixedFunctionStyle::AcesRgbToJmh20,
            18 => FixedFunctionStyle::AcesTonescaleCompress20,
            19 => FixedFunctionStyle::AcesGamutCompress20,
            20 => FixedFunctionStyle::RgbToHsyLin,
            21 => FixedFunctionStyle::RgbToHsyLog,
            22 => FixedFunctionStyle::RgbToHsyVid,
            _ => FixedFunctionStyle::AcesRedMod03,
        }
    }

    pub fn set_style(&self, style: FixedFunctionStyle) {
        unsafe {
            ocio_sys::ocio_fixed_function_transform_set_style(self.handle.as_ptr(), style as i32);
        }
    }

    pub fn num_params(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_fixed_function_transform_get_num_params(self.handle.as_ptr()) as i32
        }
    }

    pub fn params(&self) -> Vec<f64> {
        let n = self.num_params();
        if n <= 0 {
            return Vec::new();
        }
        let mut params = vec![0.0f64; n as usize];
        unsafe {
            ocio_sys::ocio_fixed_function_transform_get_params(
                self.handle.as_ptr(),
                params.as_mut_ptr() as *mut c_void,
            );
        }
        params
    }

    pub fn set_params(&self, params: &[f64]) {
        unsafe {
            ocio_sys::ocio_fixed_function_transform_set_params(
                self.handle.as_ptr(),
                params.as_ptr(),
                params.len() as usize,
            );
        }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe {
            ocio_sys::ocio_fixed_function_transform_get_direction(
                self.handle.as_ptr() as *mut c_void
            )
        };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_fixed_function_transform_set_direction(
                self.handle.as_ptr(),
                direction as i32,
            );
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe {
            ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr() as *mut c_void)
        };
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
        unsafe {
            ocio_sys::ocio_fixed_function_transform_equals(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            )
        }
    }
}

impl Drop for FixedFunctionTransform {
    fn drop(&mut self) {
        unsafe {
            ocio_sys::ocio_fixed_function_transform_destroy(self.handle.as_ptr() as *mut c_void)
        };
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
        let ft =
            FixedFunctionTransform::create_with_params(FixedFunctionStyle::Rec2100Surround, &[1.0]);
        assert!(ft.is_ok());
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let _ = ft.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let _ = ft.format_metadata();
    }

    #[test]
    fn equals_no_crash() {
        let a = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let b = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let _ = a.equals(&b);
    }
}
