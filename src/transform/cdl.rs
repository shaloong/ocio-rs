use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstring, OcioError, Result, TransformDirection, CDLStyle};

pub struct CDLTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl CDLTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_cdl_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn create_from_file(src: impl AsRef<str>, ccc_id: impl AsRef<str>) -> Result<Self> {
        let src = cstring(src)?;
        let ccc_id = cstring(ccc_id)?;
        let handle = unsafe {
            ocio_sys::ocio_cdl_transform_create_from_file(src.as_ptr().cast(), ccc_id.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn slope(&self) -> [f64; 3] {
        let mut rgb = [1.0f64; 3];
        unsafe { ocio_sys::ocio_cdl_transform_get_slope(self.handle.as_ptr(), rgb.as_mut_ptr()) };
        rgb
    }

    pub fn set_slope(&self, rgb: &[f64; 3]) {
        unsafe { ocio_sys::ocio_cdl_transform_set_slope(self.handle.as_ptr(), rgb.as_ptr()) };
    }

    pub fn offset(&self) -> [f64; 3] {
        let mut rgb = [0.0f64; 3];
        unsafe { ocio_sys::ocio_cdl_transform_get_offset(self.handle.as_ptr(), rgb.as_mut_ptr()) };
        rgb
    }

    pub fn set_offset(&self, rgb: &[f64; 3]) {
        unsafe { ocio_sys::ocio_cdl_transform_set_offset(self.handle.as_ptr(), rgb.as_ptr()) };
    }

    pub fn power_(&self) -> [f64; 3] {
        let mut rgb = [1.0f64; 3];
        unsafe { ocio_sys::ocio_cdl_transform_get_power(self.handle.as_ptr(), rgb.as_mut_ptr()) };
        rgb
    }

    pub fn set_power(&self, rgb: &[f64; 3]) {
        unsafe { ocio_sys::ocio_cdl_transform_set_power(self.handle.as_ptr(), rgb.as_ptr()) };
    }

    pub fn sat(&self) -> f64 {
        unsafe { ocio_sys::ocio_cdl_transform_get_sat(self.handle.as_ptr()) }
    }

    pub fn set_sat(&self, sat: f64) {
        unsafe { ocio_sys::ocio_cdl_transform_set_sat(self.handle.as_ptr(), sat) };
    }

    pub fn sat_luma_coefs(&self) -> [f64; 3] {
        let mut rgb = [0.0f64; 3];
        unsafe {
            ocio_sys::ocio_cdl_transform_get_sat_luma_coefs(self.handle.as_ptr(), rgb.as_mut_ptr());
        }
        rgb
    }

    pub fn style(&self) -> CDLStyle {
        let s = unsafe { ocio_sys::ocio_cdl_transform_get_style(self.handle.as_ptr()) };
        match s { 1 => CDLStyle::NoClamp, _ => CDLStyle::Asc }
    }

    pub fn set_style(&self, style: CDLStyle) {
        unsafe { ocio_sys::ocio_cdl_transform_set_style(self.handle.as_ptr(), style as i32) };
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_cdl_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_cdl_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }
}

impl Drop for CDLTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_cdl_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_cdl() {
        let cdl = CDLTransform::create();
        assert!(cdl.is_ok());
    }

    #[test]
    fn slope_offset_power_no_crash() {
        let cdl = CDLTransform::create().unwrap();
        let _ = cdl.slope();
        let _ = cdl.offset();
        let _ = cdl.power_();
        cdl.set_slope(&[1.2, 1.0, 0.9]);
        cdl.set_offset(&[0.1, 0.0, -0.1]);
        cdl.set_power(&[1.1, 1.0, 0.95]);
    }

    #[test]
    fn saturation_no_crash() {
        let cdl = CDLTransform::create().unwrap();
        let _ = cdl.sat();
        cdl.set_sat(1.5);
    }

    #[test]
    fn style_no_crash() {
        let cdl = CDLTransform::create().unwrap();
        let _ = cdl.style();
        cdl.set_style(CDLStyle::NoClamp);
    }
}
