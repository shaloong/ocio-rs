use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{grading::GradingPrimary, GradingStyle, OcioError, Result, TransformDirection};

pub struct GradingPrimaryTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl GradingPrimaryTransform {
    pub fn create(style: GradingStyle) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_grading_primary_transform_create(style as i32) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn style(&self) -> GradingStyle {
        let v = unsafe { ocio_sys::ocio_grading_primary_transform_get_style(self.handle.as_ptr()) };
        match v { 1 => GradingStyle::Lin, 2 => GradingStyle::Video, _ => GradingStyle::Log }
    }

    pub fn set_style(&self, style: GradingStyle) {
        unsafe { ocio_sys::ocio_grading_primary_transform_set_style(self.handle.as_ptr(), style as i32); }
    }

    pub fn value(&self) -> GradingPrimary {
        let mut flat = [0.0f64; 34];
        unsafe { ocio_sys::ocio_grading_primary_transform_get_value(self.handle.as_ptr(), flat.as_mut_ptr()); }
        GradingPrimary::from_flat_array(&flat)
    }

    pub fn set_value(&self, value: &GradingPrimary) {
        let flat = value.to_flat_array();
        unsafe { ocio_sys::ocio_grading_primary_transform_set_value(self.handle.as_ptr(), flat.as_ptr()); }
    }

    pub fn is_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_grading_primary_transform_is_dynamic(self.handle.as_ptr()) }
    }

    pub fn make_dynamic(&self) {
        unsafe { ocio_sys::ocio_grading_primary_transform_make_dynamic(self.handle.as_ptr()); }
    }

    pub fn make_non_dynamic(&self) {
        unsafe { ocio_sys::ocio_grading_primary_transform_make_non_dynamic(self.handle.as_ptr()); }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_grading_primary_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe { ocio_sys::ocio_grading_primary_transform_set_direction(self.handle.as_ptr(), direction as i32); }
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
        t.set_style(GradingStyle::Lin);
        t.set_style(GradingStyle::Video);
    }

    #[test]
    fn set_value_no_crash() {
        let t = GradingPrimaryTransform::create(GradingStyle::Log).unwrap();
        let v = GradingPrimary::new(GradingStyle::Log);
        t.set_value(&v);
    }

    #[test]
    fn make_dynamic_no_crash() {
        let t = GradingPrimaryTransform::create(GradingStyle::Log).unwrap();
        t.make_dynamic();
        t.make_non_dynamic();
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
}
