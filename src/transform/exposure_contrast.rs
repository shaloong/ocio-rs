use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{ExposureContrastStyle, OcioError, Result, TransformDirection};
use ocio_sys;

/// Exposure/contrast/gamma style transform with optional dynamic controls.
pub struct ExposureContrastTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl ExposureContrastTransform {
    /// Create a new exposure-contrast transform.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_exposure_contrast_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the current exposure value.
    pub fn exposure(&self) -> f64 {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_get_exposure(self.handle.as_ptr()) }
    }

    /// Set the exposure value.
    pub fn set_exposure(&self, exposure: f64) {
        self.try_set_exposure(exposure)
            .expect("failed to set exposure");
    }

    /// Set exposure and surface any OCIO validation error.
    pub fn try_set_exposure(&self, exposure: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_set_exposure(self.handle.as_ptr(), exposure)
        };
        crate::ocio_call_status()
    }

    /// Return the current contrast value.
    pub fn contrast(&self) -> f64 {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_get_contrast(self.handle.as_ptr()) }
    }

    /// Set the contrast value.
    pub fn set_contrast(&self, contrast: f64) {
        self.try_set_contrast(contrast)
            .expect("failed to set contrast");
    }

    /// Set contrast and surface any OCIO validation error.
    pub fn try_set_contrast(&self, contrast: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_set_contrast(self.handle.as_ptr(), contrast)
        };
        crate::ocio_call_status()
    }

    /// Return the current gamma value.
    pub fn gamma(&self) -> f64 {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_get_gamma(self.handle.as_ptr()) }
    }

    /// Set the gamma value.
    pub fn set_gamma(&self, gamma: f64) {
        self.try_set_gamma(gamma).expect("failed to set gamma");
    }

    /// Set gamma and surface any OCIO validation error.
    pub fn try_set_gamma(&self, gamma: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_set_gamma(self.handle.as_ptr(), gamma)
        };
        crate::ocio_call_status()
    }

    /// Return the current pivot value.
    pub fn pivot(&self) -> f64 {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_get_pivot(self.handle.as_ptr()) }
    }

    /// Set the pivot value.
    pub fn set_pivot(&self, pivot: f64) {
        self.try_set_pivot(pivot).expect("failed to set pivot");
    }

    /// Set pivot and surface any OCIO validation error.
    pub fn try_set_pivot(&self, pivot: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_set_pivot(self.handle.as_ptr(), pivot)
        };
        crate::ocio_call_status()
    }

    /// Return the current exposure-contrast style.
    pub fn style(&self) -> ExposureContrastStyle {
        let s =
            unsafe { ocio_sys::ocio_exposure_contrast_transform_get_style(self.handle.as_ptr()) };
        match s {
            1 => ExposureContrastStyle::Video,
            2 => ExposureContrastStyle::Logarithmic,
            _ => ExposureContrastStyle::Linear,
        }
    }

    /// Set the exposure-contrast style.
    pub fn set_style(&self, style: ExposureContrastStyle) {
        self.try_set_style(style)
            .expect("failed to set exposure contrast style");
    }

    /// Set the exposure/contrast style and surface any OCIO validation error.
    pub fn try_set_style(&self, style: ExposureContrastStyle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_set_style(
                self.handle.as_ptr(),
                style as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Return whether exposure is marked as dynamic.
    pub fn is_exposure_dynamic(&self) -> bool {
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_is_exposure_dynamic(self.handle.as_ptr())
        }
    }

    /// Mark exposure as dynamic.
    pub fn make_exposure_dynamic(&self) {
        self.try_make_exposure_dynamic()
            .expect("failed to make exposure dynamic");
    }

    /// Mark exposure as dynamic and surface any OCIO validation error.
    pub fn try_make_exposure_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_make_exposure_dynamic(self.handle.as_ptr())
        };
        crate::ocio_call_status()
    }

    /// Return whether contrast is marked as dynamic.
    pub fn is_contrast_dynamic(&self) -> bool {
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_is_contrast_dynamic(self.handle.as_ptr())
        }
    }

    /// Mark contrast as dynamic.
    pub fn make_contrast_dynamic(&self) {
        self.try_make_contrast_dynamic()
            .expect("failed to make contrast dynamic");
    }

    /// Mark contrast as dynamic and surface any OCIO validation error.
    pub fn try_make_contrast_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_make_contrast_dynamic(self.handle.as_ptr())
        };
        crate::ocio_call_status()
    }

    /// Mark exposure as non-dynamic.
    pub fn make_exposure_non_dynamic(&self) {
        self.try_make_exposure_non_dynamic()
            .expect("failed to make exposure non-dynamic");
    }

    /// Mark exposure as non-dynamic and surface any OCIO validation error.
    pub fn try_make_exposure_non_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_make_exposure_non_dynamic(
                self.handle.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Mark contrast as non-dynamic.
    pub fn make_contrast_non_dynamic(&self) {
        self.try_make_contrast_non_dynamic()
            .expect("failed to make contrast non-dynamic");
    }

    /// Mark contrast as non-dynamic and surface any OCIO validation error.
    pub fn try_make_contrast_non_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_make_contrast_non_dynamic(
                self.handle.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Return whether gamma is marked as dynamic.
    pub fn is_gamma_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_is_gamma_dynamic(self.handle.as_ptr()) }
    }

    /// Mark gamma as dynamic.
    pub fn make_gamma_dynamic(&self) {
        self.try_make_gamma_dynamic()
            .expect("failed to make gamma dynamic");
    }

    /// Mark gamma as dynamic and surface any OCIO validation error.
    pub fn try_make_gamma_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_make_gamma_dynamic(self.handle.as_ptr())
        };
        crate::ocio_call_status()
    }

    /// Mark gamma as non-dynamic.
    pub fn make_gamma_non_dynamic(&self) {
        self.try_make_gamma_non_dynamic()
            .expect("failed to make gamma non-dynamic");
    }

    /// Mark gamma as non-dynamic and surface any OCIO validation error.
    pub fn try_make_gamma_non_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_make_gamma_non_dynamic(self.handle.as_ptr())
        };
        crate::ocio_call_status()
    }

    /// Return the log exposure step value.
    pub fn log_exposure_step(&self) -> f64 {
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_get_log_exposure_step(self.handle.as_ptr())
        }
    }

    /// Set the log exposure step value.
    pub fn set_log_exposure_step(&self, step: f64) {
        self.try_set_log_exposure_step(step)
            .expect("failed to set log exposure step");
    }

    /// Set the log exposure step and surface any OCIO validation error.
    pub fn try_set_log_exposure_step(&self, step: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_set_log_exposure_step(
                self.handle.as_ptr(),
                step,
            )
        };
        crate::ocio_call_status()
    }

    /// Return the log mid-gray value.
    pub fn log_mid_gray(&self) -> f64 {
        unsafe { ocio_sys::ocio_exposure_contrast_transform_get_log_mid_gray(self.handle.as_ptr()) }
    }

    /// Set the log mid-gray value.
    pub fn set_log_mid_gray(&self, mid_gray: f64) {
        self.try_set_log_mid_gray(mid_gray)
            .expect("failed to set log mid gray");
    }

    /// Set the log mid-gray value and surface any OCIO validation error.
    pub fn try_set_log_mid_gray(&self, mid_gray: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_set_log_mid_gray(
                self.handle.as_ptr(),
                mid_gray,
            )
        };
        crate::ocio_call_status()
    }

    /// Return the transform direction.
    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe {
            ocio_sys::ocio_exposure_contrast_transform_get_direction(self.handle.as_ptr())
        };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the transform direction.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set exposure-contrast transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_set_direction(
                self.handle.as_ptr(),
                direction as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Create an independent copy of this transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return format metadata attached to the transform, when available.
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

    /// Return whether this transform is equivalent to `other`.
    pub fn equals(&self, other: &Self) -> bool {
        unsafe {
            ocio_sys::ocio_exposure_contrast_transform_equals(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            )
        }
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
        t.try_make_exposure_dynamic().unwrap();
        t.try_make_contrast_dynamic().unwrap();
        t.try_make_gamma_dynamic().unwrap();
        t.try_make_exposure_non_dynamic().unwrap();
        t.try_make_contrast_non_dynamic().unwrap();
        t.try_make_gamma_non_dynamic().unwrap();
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

    #[test]
    fn equals_no_crash() {
        let a = ExposureContrastTransform::create().unwrap();
        let b = ExposureContrastTransform::create().unwrap();
        let _ = a.equals(&b);
    }
}
