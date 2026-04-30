use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{GradingStyle, OcioError, RGBCurveType, Result, TransformDirection};

pub struct GradingHueCurveTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl GradingHueCurveTransform {
    pub fn create(style: GradingStyle) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_grading_hue_curve_transform_create(style as i32) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn style(&self) -> GradingStyle {
        let v = unsafe { ocio_sys::ocio_grading_hue_curve_transform_get_style(self.handle.as_ptr()) };
        match v { 1 => GradingStyle::Lin, 2 => GradingStyle::Video, _ => GradingStyle::Log }
    }

    pub fn set_style(&self, style: GradingStyle) {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_set_style(self.handle.as_ptr(), style as i32); }
    }

    pub fn num_control_points(&self, curve_type: RGBCurveType) -> i32 {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_get_num_control_points(self.handle.as_ptr(), curve_type as i32) }
    }

    pub fn control_point(&self, curve_type: RGBCurveType, index: i32) -> (f32, f32) {
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_get_control_point(
                self.handle.as_ptr(), curve_type as i32, index, &mut x, &mut y
            );
        }
        (x, y)
    }

    pub fn set_num_control_points(&self, curve_type: RGBCurveType, num: i32) {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_set_num_control_points(self.handle.as_ptr(), curve_type as i32, num); }
    }

    pub fn set_control_point(&self, curve_type: RGBCurveType, index: i32, x: f32, y: f32) {
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_set_control_point(
                self.handle.as_ptr(), curve_type as i32, index, x, y
            );
        }
    }

    pub fn slope(&self, curve_type: RGBCurveType, index: i32) -> f32 {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_get_slope(self.handle.as_ptr(), curve_type as i32, index) }
    }

    pub fn set_slope(&self, curve_type: RGBCurveType, index: i32, slope: f32) {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_set_slope(self.handle.as_ptr(), curve_type as i32, index, slope); }
    }

    pub fn slopes_are_default(&self, curve_type: RGBCurveType) -> bool {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_slopes_are_default(self.handle.as_ptr(), curve_type as i32) }
    }

    pub fn bypass_lin_to_log(&self) -> bool {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_get_bypass_lin_to_log(self.handle.as_ptr()) }
    }

    pub fn set_bypass_lin_to_log(&self, bypass: bool) {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_set_bypass_lin_to_log(self.handle.as_ptr(), bypass); }
    }

    pub fn is_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_is_dynamic(self.handle.as_ptr()) }
    }

    pub fn make_dynamic(&self) {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_make_dynamic(self.handle.as_ptr()); }
    }

    pub fn make_non_dynamic(&self) {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_make_non_dynamic(self.handle.as_ptr()); }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_grading_hue_curve_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_set_direction(self.handle.as_ptr(), direction as i32); }
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }
}

impl Drop for GradingHueCurveTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_grading_hue_curve() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log);
        assert!(t.is_ok());
    }

    #[test]
    fn grading_hue_curve_methods_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Lin).unwrap();
        let _ = t.style();
        let _ = t.num_control_points(RGBCurveType::Red);
        let _ = t.control_point(RGBCurveType::Red, 0);
        let _ = t.slope(RGBCurveType::Red, 0);
        let _ = t.slopes_are_default(RGBCurveType::Red);
        let _ = t.bypass_lin_to_log();
        let _ = t.is_dynamic();
        let _ = t.direction();
    }

    #[test]
    fn set_style_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        t.set_style(GradingStyle::Lin);
    }

    #[test]
    fn set_curve_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        t.set_num_control_points(RGBCurveType::Red, 2);
        t.set_control_point(RGBCurveType::Red, 0, 0.0, 0.0);
        t.set_control_point(RGBCurveType::Red, 1, 1.0, 1.0);
        t.set_slope(RGBCurveType::Red, 0, 1.0);
    }

    #[test]
    fn bypass_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        t.set_bypass_lin_to_log(true);
        t.set_bypass_lin_to_log(false);
    }

    #[test]
    fn make_dynamic_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        t.make_dynamic();
        t.make_non_dynamic();
    }

    #[test]
    fn direction_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        let _ = t.format_metadata();
    }
}
