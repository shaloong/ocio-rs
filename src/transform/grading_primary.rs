use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{grading::GradingPrimary, GradingStyle, OcioError, Result, TransformDirection};
use ocio_sys;

/// OCIO primary grading transform for lift/gamma/gain-style controls.
pub struct GradingPrimaryTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl GradingPrimaryTransform {
    /// Create a primary grading transform with the given style (alias for [`Self::create`]).
    pub fn create_with_style(style: GradingStyle) -> Result<Self> {
        Self::create(style)
    }

    /// Create a new primary grading transform with the given style.
    pub fn create(style: GradingStyle) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_grading_primary_transform_create_with_style(style as i32) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an independent, editable copy of this transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the current grading style (Log, Lin, or Video).
    pub fn style(&self) -> GradingStyle {
        let v = unsafe { ocio_sys::ocio_grading_primary_transform_get_style(self.handle.as_ptr()) };
        match v {
            1 => GradingStyle::Lin,
            2 => GradingStyle::Video,
            _ => GradingStyle::Log,
        }
    }

    /// Set the grading style, panicking on failure.
    pub fn set_style(&self, style: GradingStyle) {
        self.try_set_style(style)
            .expect("failed to set primary grading style");
    }

    /// Set the grading style and surface any OCIO validation error.
    pub fn try_set_style(&self, style: GradingStyle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_primary_transform_set_style(self.handle.as_ptr(), style as i32);
        }
        crate::ocio_call_status()
    }

    /// Return the current primary grading control values.
    pub fn value(&self) -> GradingPrimary {
        let mut flat = [0.0f64; 34];
        let copied = unsafe {
            ocio_sys::ocio_grading_primary_transform_copy_value(
                self.handle.as_ptr(),
                flat.as_mut_ptr(),
                flat.len(),
            )
        };
        if !copied {
            return GradingPrimary::new(self.style());
        }
        GradingPrimary::from_flat_array(&flat)
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer value()")]
    pub fn copy_value(&self) -> GradingPrimary {
        self.value()
    }

    /// Set the primary grading control values, panicking on failure.
    pub fn set_value(&self, value: &GradingPrimary) {
        self.try_set_value(value)
            .expect("failed to set primary grading value");
    }

    /// Replace the primary grading controls and surface any OCIO validation error.
    pub fn try_set_value(&self, value: &GradingPrimary) -> Result<()> {
        let flat = value.to_flat_array();
        crate::clear_last_error();
        let accepted = unsafe {
            ocio_sys::ocio_grading_primary_transform_set_value_from_f64(
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
                "GradingPrimaryTransform::set_value was rejected".to_owned(),
            ))
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer set_value(&GradingPrimary)"
    )]
    pub fn set_value_from_f64(&self, value: &GradingPrimary) {
        self.set_value(value);
    }

    /// Return `true` if this transform is dynamic (values can be updated at runtime).
    pub fn is_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_grading_primary_transform_is_dynamic(self.handle.as_ptr()) }
    }

    /// Make this transform dynamic, panicking on failure.
    pub fn make_dynamic(&self) {
        self.try_make_dynamic()
            .expect("failed to make primary grading dynamic");
    }

    /// Make this transform dynamic and surface any OCIO validation error.
    pub fn try_make_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_primary_transform_make_dynamic(self.handle.as_ptr());
        }
        crate::ocio_call_status()
    }

    /// Make this transform non-dynamic, panicking on failure.
    pub fn make_non_dynamic(&self) {
        self.try_make_non_dynamic()
            .expect("failed to make primary grading non-dynamic");
    }

    /// Make this transform non-dynamic and surface any OCIO validation error.
    pub fn try_make_non_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_primary_transform_make_non_dynamic(self.handle.as_ptr());
        }
        crate::ocio_call_status()
    }

    /// Return the current transform direction.
    pub fn direction(&self) -> TransformDirection {
        let dir =
            unsafe { ocio_sys::ocio_grading_primary_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the transform direction, panicking on failure.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set primary grading direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_primary_transform_set_direction(
                self.handle.as_ptr(),
                direction as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Return the format metadata associated with this transform, if any.
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

    /// Return `true` if this transform is equal to `other`.
    pub fn equals(&self, other: &Self) -> bool {
        unsafe {
            ocio_sys::ocio_grading_primary_transform_equals(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            )
        }
    }
}

impl Drop for GradingPrimaryTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_grading_primary_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_grading_primary() {
        let t = GradingPrimaryTransform::create(GradingStyle::Log);
        assert!(t.is_ok());
    }

    #[test]
    fn grading_primary_methods_no_crash() {
        let t = GradingPrimaryTransform::create(GradingStyle::Lin).unwrap();
        let _ = t.style();
        let _ = t.value();
        let _ = t.is_dynamic();
        let _ = t.direction();
    }

    #[test]
    fn set_style_no_crash() {
        let t = GradingPrimaryTransform::create(GradingStyle::Log).unwrap();
        t.try_set_style(GradingStyle::Lin).unwrap();
        t.try_set_style(GradingStyle::Video).unwrap();
    }

    #[test]
    fn set_value_no_crash() {
        let t = GradingPrimaryTransform::create(GradingStyle::Log).unwrap();
        let v = GradingPrimary::new(GradingStyle::Log);
        t.try_set_value(&v).unwrap();
    }

    #[test]
    fn make_dynamic_no_crash() {
        let t = GradingPrimaryTransform::create(GradingStyle::Log).unwrap();
        t.try_make_dynamic().unwrap();
        t.try_make_non_dynamic().unwrap();
    }

    #[test]
    fn direction_no_crash() {
        let t = GradingPrimaryTransform::create(GradingStyle::Log).unwrap();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = GradingPrimaryTransform::create(GradingStyle::Log).unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = GradingPrimaryTransform::create(GradingStyle::Log).unwrap();
        let _ = t.format_metadata();
    }
}
