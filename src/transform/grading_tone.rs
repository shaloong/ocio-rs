use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{grading::GradingTone, GradingStyle, OcioError, Result, TransformDirection};
use ocio_sys;

/// OCIO grading transform that applies tone-region contrast adjustments.
pub struct GradingToneTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl GradingToneTransform {
    pub fn create_with_style(style: GradingStyle) -> Result<Self> {
        Self::create(style)
    }

    pub fn create(style: GradingStyle) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_grading_tone_transform_create_with_style(style as i32) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn style(&self) -> GradingStyle {
        let v = unsafe { ocio_sys::ocio_grading_tone_transform_get_style(self.handle.as_ptr()) };
        match v {
            1 => GradingStyle::Lin,
            2 => GradingStyle::Video,
            _ => GradingStyle::Log,
        }
    }

    pub fn set_style(&self, style: GradingStyle) {
        self.try_set_style(style)
            .expect("failed to set grading tone style");
    }

    /// Set the grading style and surface any OCIO validation error.
    pub fn try_set_style(&self, style: GradingStyle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_tone_transform_set_style(self.handle.as_ptr(), style as i32);
        }
        crate::ocio_call_status()
    }

    pub fn value(&self) -> GradingTone {
        let mut flat = [0.0f64; 31];
        let copied = unsafe {
            ocio_sys::ocio_grading_tone_transform_copy_value(
                self.handle.as_ptr(),
                flat.as_mut_ptr(),
                flat.len(),
            )
        };
        if !copied {
            return GradingTone::new(self.style());
        }
        GradingTone::from_flat_array(&flat)
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer value()")]
    pub fn copy_value(&self) -> GradingTone {
        self.value()
    }

    pub fn set_value(&self, value: &GradingTone) {
        self.try_set_value(value)
            .expect("failed to set grading tone value");
    }

    /// Replace the tone grading controls and surface any OCIO validation error.
    pub fn try_set_value(&self, value: &GradingTone) -> Result<()> {
        let flat = value.to_flat_array();
        crate::clear_last_error();
        let accepted = unsafe {
            ocio_sys::ocio_grading_tone_transform_set_value_from_f64(
                self.handle.as_ptr(),
                flat.as_ptr(),
                flat.len(),
            )
        };
        if accepted {
            crate::ocio_call_status()
        } else {
            crate::ocio_call_status()?;
            Err(OcioError::Ocio(
                "GradingToneTransform::set_value was rejected".to_owned(),
            ))
        }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer set_value(&GradingTone)")]
    pub fn set_value_from_f64(&self, value: &GradingTone) {
        self.set_value(value);
    }

    pub fn is_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_grading_tone_transform_is_dynamic(self.handle.as_ptr()) }
    }

    pub fn make_dynamic(&self) {
        self.try_make_dynamic()
            .expect("failed to make grading tone dynamic");
    }

    /// Make this transform dynamic and surface any OCIO validation error.
    pub fn try_make_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_tone_transform_make_dynamic(self.handle.as_ptr());
        }
        crate::ocio_call_status()
    }

    pub fn make_non_dynamic(&self) {
        self.try_make_non_dynamic()
            .expect("failed to make grading tone non-dynamic");
    }

    /// Make this transform non-dynamic and surface any OCIO validation error.
    pub fn try_make_non_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_tone_transform_make_non_dynamic(self.handle.as_ptr());
        }
        crate::ocio_call_status()
    }

    pub fn direction(&self) -> TransformDirection {
        let dir =
            unsafe { ocio_sys::ocio_grading_tone_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_grading_tone_transform_set_direction(
                self.handle.as_ptr(),
                direction as i32,
            );
        }
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

    pub fn equals(&self, other: &Self) -> bool {
        unsafe {
            ocio_sys::ocio_grading_tone_transform_equals(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            )
        }
    }
}

impl Drop for GradingToneTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_grading_tone_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_grading_tone() {
        let t = GradingToneTransform::create(GradingStyle::Log);
        assert!(t.is_ok());
    }

    #[test]
    fn grading_tone_methods_no_crash() {
        let t = GradingToneTransform::create(GradingStyle::Lin).unwrap();
        let _ = t.style();
        let _ = t.value();
        let _ = t.is_dynamic();
        let _ = t.direction();
    }

    #[test]
    fn set_style_no_crash() {
        let t = GradingToneTransform::create(GradingStyle::Log).unwrap();
        t.try_set_style(GradingStyle::Lin).unwrap();
        t.try_set_style(GradingStyle::Video).unwrap();
    }

    #[test]
    fn set_value_no_crash() {
        if crate::is_stub_build() {
            return;
        }
        let t = GradingToneTransform::create(GradingStyle::Log).unwrap();
        let v = GradingTone::new(GradingStyle::Log);
        t.try_set_value(&v).unwrap();
    }

    #[test]
    fn make_dynamic_no_crash() {
        let t = GradingToneTransform::create(GradingStyle::Log).unwrap();
        t.try_make_dynamic().unwrap();
        t.try_make_non_dynamic().unwrap();
    }

    #[test]
    fn direction_no_crash() {
        let t = GradingToneTransform::create(GradingStyle::Log).unwrap();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = GradingToneTransform::create(GradingStyle::Log).unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = GradingToneTransform::create(GradingStyle::Log).unwrap();
        let _ = t.format_metadata();
    }
}
