use std::ffi::c_void;
use std::ptr::NonNull;

use crate::grading::{GradingCurvePoint, GradingRGBCurveValue};
use crate::{GradingStyle, OcioError, RGBCurveType, Result, TransformDirection};
use ocio_sys;

pub struct GradingRGBCurveTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl GradingRGBCurveTransform {
    pub fn create_with_style(style: GradingStyle) -> Result<Self> {
        Self::create(style)
    }

    pub fn create(style: GradingStyle) -> Result<Self> {
        let handle =
            unsafe { ocio_sys::ocio_grading_rgb_curve_transform_create_with_style(style as i32) };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn style(&self) -> GradingStyle {
        let v =
            unsafe { ocio_sys::ocio_grading_rgb_curve_transform_get_style(self.handle.as_ptr()) };
        match v {
            1 => GradingStyle::Lin,
            2 => GradingStyle::Video,
            _ => GradingStyle::Log,
        }
    }

    pub fn set_style(&self, style: GradingStyle) {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_style(
                self.handle.as_ptr(),
                style as i32,
            );
        }
    }

    pub fn value(&self) -> GradingRGBCurveValue {
        fn read_curve(
            transform: &GradingRGBCurveTransform,
            curve_type: RGBCurveType,
        ) -> Vec<GradingCurvePoint> {
            let count = transform.num_control_points(curve_type).max(0) as usize;
            (0..count)
                .map(|index| {
                    let index = index as i32;
                    let (x, y) = transform.control_point(curve_type, index);
                    let slope = transform.slope(curve_type, index);
                    GradingCurvePoint { x, y, slope }
                })
                .collect()
        }

        GradingRGBCurveValue {
            red: read_curve(self, RGBCurveType::Red),
            green: read_curve(self, RGBCurveType::Green),
            blue: read_curve(self, RGBCurveType::Blue),
            master: read_curve(self, RGBCurveType::Master),
        }
    }

    pub fn set_value(&self, value: &GradingRGBCurveValue) {
        fn write_curve(
            transform: &GradingRGBCurveTransform,
            curve_type: RGBCurveType,
            points: &[GradingCurvePoint],
        ) {
            transform.set_num_control_points(curve_type, points.len() as i32);
            for (index, point) in points.iter().enumerate() {
                let index = index as i32;
                transform.set_control_point(curve_type, index, point.x, point.y);
                transform.set_slope(curve_type, index, point.slope);
            }
        }

        write_curve(self, RGBCurveType::Red, &value.red);
        write_curve(self, RGBCurveType::Green, &value.green);
        write_curve(self, RGBCurveType::Blue, &value.blue);
        write_curve(self, RGBCurveType::Master, &value.master);
    }

    #[deprecated(
        since = "0.2.0",
        note = "prefer value() to read a safe Rust snapshot of the grading curve"
    )]
    pub fn raw_value_handle(&self) -> *mut c_void {
        unsafe { ocio_sys::ocio_grading_rgb_curve_transform_get_value(self.handle.as_ptr()) }
    }

    /// # Safety
    /// `values` must point to a valid OCIO grading-RGB-curve value object for the active ABI.
    #[deprecated(
        since = "0.2.0",
        note = "prefer set_value(&GradingRGBCurveValue) instead of passing a raw OCIO handle"
    )]
    pub unsafe fn set_value_raw(&self, values: *mut c_void) {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_value(self.handle.as_ptr(), values);
        }
    }

    pub fn num_control_points(&self, curve_type: RGBCurveType) -> i32 {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_get_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        }
    }

    pub fn control_point(&self, curve_type: RGBCurveType, index: i32) -> (f32, f32) {
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_get_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                &mut x,
                &mut y,
            );
        }
        (x, y)
    }

    pub fn set_num_control_points(&self, curve_type: RGBCurveType, num: i32) {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
                num,
            );
        }
    }

    pub fn set_control_point(&self, curve_type: RGBCurveType, index: i32, x: f32, y: f32) {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                x,
                y,
            );
        }
    }

    pub fn slope(&self, curve_type: RGBCurveType, index: i32) -> f32 {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_get_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index as usize,
            )
        }
    }

    pub fn set_slope(&self, curve_type: RGBCurveType, index: i32, slope: f32) {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index as usize,
                slope,
            );
        }
    }

    pub fn slopes_are_default(&self, curve_type: RGBCurveType) -> bool {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_slopes_are_default(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        }
    }

    pub fn bypass_lin_to_log(&self) -> bool {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_get_bypass_lin_to_log(self.handle.as_ptr())
        }
    }

    pub fn set_bypass_lin_to_log(&self, bypass: bool) {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_bypass_lin_to_log(
                self.handle.as_ptr(),
                bypass,
            );
        }
    }

    pub fn is_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_grading_rgb_curve_transform_is_dynamic(self.handle.as_ptr()) }
    }

    pub fn make_dynamic(&self) {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_make_dynamic(self.handle.as_ptr());
        }
    }

    pub fn make_non_dynamic(&self) {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_make_non_dynamic(self.handle.as_ptr());
        }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_get_direction(self.handle.as_ptr())
        };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_direction(
                self.handle.as_ptr(),
                direction as i32,
            );
        }
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_get_format_metadata(self.handle.as_ptr())
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
            ocio_sys::ocio_grading_rgb_curve_transform_equals(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            )
        }
    }
}

impl Drop for GradingRGBCurveTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_grading_rgb_curve_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_grading_rgb_curve() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log);
        assert!(t.is_ok());
    }

    #[test]
    fn grading_rgb_curve_methods_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Lin).unwrap();
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
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        t.set_style(GradingStyle::Lin);
    }

    #[test]
    fn set_curve_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        t.set_num_control_points(RGBCurveType::Red, 2);
        t.set_control_point(RGBCurveType::Red, 0, 0.0, 0.0);
        t.set_control_point(RGBCurveType::Red, 1, 1.0, 1.0);
        t.set_slope(RGBCurveType::Red, 0, 1.0);
    }

    #[test]
    fn value_round_trip_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        let value = crate::grading::GradingRGBCurveValue {
            red: vec![
                crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0),
                crate::grading::GradingCurvePoint::new(1.0, 1.0, 1.0),
            ],
            green: vec![crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0)],
            blue: vec![crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0)],
            master: vec![crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0)],
        };
        t.set_value(&value);
        let _ = t.value();
    }

    #[test]
    fn bypass_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        t.set_bypass_lin_to_log(true);
        t.set_bypass_lin_to_log(false);
    }

    #[test]
    fn make_dynamic_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        t.make_dynamic();
        t.make_non_dynamic();
    }

    #[test]
    fn direction_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        let _ = t.format_metadata();
    }
}
