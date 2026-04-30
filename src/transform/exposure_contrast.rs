use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection, ExposureContrastStyle};

pub struct ExposureContrastTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl ExposureContrastTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_exposure_contrast_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn exposure(&self) -> f64 {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_get_exposure(self.handle.as_ptr()) }
    }

    pub fn set_exposure(&self, exposure: f64) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_set_exposure(self.handle.as_ptr(), exposure) };
    }

    pub fn contrast(&self) -> f64 {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_get_contrast(self.handle.as_ptr()) }
    }

    pub fn set_contrast(&self, contrast: f64) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_set_contrast(self.handle.as_ptr(), contrast) };
    }

    pub fn gamma(&self) -> f64 {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_get_gamma(self.handle.as_ptr()) }
    }

    pub fn set_gamma(&self, gamma: f64) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_set_gamma(self.handle.as_ptr(), gamma) };
    }

    pub fn pivot(&self) -> f64 {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_get_pivot(self.handle.as_ptr()) }
    }

    pub fn set_pivot(&self, pivot: f64) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_set_pivot(self.handle.as_ptr(), pivot) };
    }

    pub fn style(&self) -> ExposureContrastStyle {
        let s = unsafe { ocio_sys::ocio_exposure_contrast_transform_get_style(self.handle.as_ptr()) };
        match s {
            1 => ExposureContrastStyle::Video,
            2 => ExposureContrastStyle::Logarithmic,
            _ => ExposureContrastStyle::Linear,
        }
    }

    pub fn set_style(&self, style: ExposureContrastStyle) {
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_set_style(self.handle.as_ptr(), style as i32);
        }
    }

    pub fn is_exposure_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_is_exposure_dynamic(self.handle.as_ptr()) }
    }

    pub fn make_exposure_dynamic(&self) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_make_exposure_dynamic(self.handle.as_ptr()) };
    }

    pub fn is_contrast_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_is_contrast_dynamic(self.handle.as_ptr()) }
    }

    pub fn make_contrast_dynamic(&self) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_make_contrast_dynamic(self.handle.as_ptr()) };
    }

    pub fn make_exposure_non_dynamic(&self) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_make_exposure_non_dynamic(self.handle.as_ptr()) };
    }

    pub fn make_contrast_non_dynamic(&self) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_make_contrast_non_dynamic(self.handle.as_ptr()) };
    }

    pub fn is_gamma_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_is_gamma_dynamic(self.handle.as_ptr()) }
    }

    pub fn make_gamma_dynamic(&self) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_make_gamma_dynamic(self.handle.as_ptr()) };
    }

    pub fn make_gamma_non_dynamic(&self) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_make_gamma_non_dynamic(self.handle.as_ptr()) };
    }

    pub fn log_exposure_step(&self) -> f64 {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_get_log_exposure_step(self.handle.as_ptr()) }
    }

    pub fn set_log_exposure_step(&self, step: f64) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_set_log_exposure_step(self.handle.as_ptr(), step) };
    }

    pub fn log_mid_gray(&self) -> f64 {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_get_log_mid_gray(self.handle.as_ptr()) }
    }

    pub fn set_log_mid_gray(&self, mid_gray: f64) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_set_log_mid_gray(self.handle.as_ptr(), mid_gray) };
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_exposure_contrast_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }
}

impl Drop for ExposureContrastTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_exposure_contrast() {
        let t = ExposureContrastTransform::create();
        assert!(t.is_ok());
    }

    #[test]
    fn exposure_contrast_methods_no_crash() {
        let t = ExposureContrastTransform::create().unwrap();
        let _ = t.exposure();
        let _ = t.contrast();
        let _ = t.gamma();
        let _ = t.pivot();
        let _ = t.style();
        let _ = t.is_exposure_dynamic();
        let _ = t.is_contrast_dynamic();
        let _ = t.is_gamma_dynamic();
    }

    #[test]
    fn set_values_no_crash() {
        let t = ExposureContrastTransform::create().unwrap();
        t.set_exposure(1.5);
        t.set_contrast(1.2);
        t.set_gamma(1.1);
        t.set_pivot(0.18);
        t.set_style(ExposureContrastStyle::Video);
    }

    #[test]
    fn make_dynamic_no_crash() {
        let t = ExposureContrastTransform::create().unwrap();
        t.make_exposure_dynamic();
        t.make_contrast_dynamic();
        t.make_gamma_dynamic();
        t.make_exposure_non_dynamic();
        t.make_contrast_non_dynamic();
        t.make_gamma_non_dynamic();
        let _ = t.log_exposure_step();
        t.set_log_exposure_step(0.088);
        let _ = t.log_mid_gray();
        t.set_log_mid_gray(0.18);
    }

    #[test]
    fn direction_no_crash() {
        let t = ExposureContrastTransform::create().unwrap();
        let _ = t.direction();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = ExposureContrastTransform::create().unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = ExposureContrastTransform::create().unwrap();
        let _ = t.format_metadata();
    }
}
