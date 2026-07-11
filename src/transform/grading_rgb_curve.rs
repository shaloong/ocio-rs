use std::ffi::c_void;
use std::ptr::NonNull;

use crate::grading::{GradingCurvePoint, GradingRGBCurveValue};
use crate::{GradingStyle, OcioError, RGBCurveType, Result, TransformDirection};
use ocio_sys;

/// OCIO grading transform that applies independent RGB curve adjustments.
pub struct GradingRGBCurveTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl GradingRGBCurveTransform {
    fn read_curve(&self, curve_type: RGBCurveType) -> Result<Vec<GradingCurvePoint>> {
        let count = self.num_control_points(curve_type)?.max(0) as usize;
        (0..count)
            .map(|index| {
                let index = index as i32;
                let (x, y) = self.control_point(curve_type, index)?;
                let slope = self.slope(curve_type, index)?;
                Ok(GradingCurvePoint { x, y, slope })
            })
            .collect()
    }

    fn require_non_negative_index(index: i32, operation: &'static str) -> Result<()> {
        if index < 0 {
            Err(OcioError::InvalidInput(format!(
                "{operation}: index must be non-negative"
            )))
        } else {
            Ok(())
        }
    }

    /// Create an RGB curve grading transform with the given style (alias for [`Self::create`]).
    pub fn create_with_style(style: GradingStyle) -> Result<Self> {
        Self::create(style)
    }

    /// Create a new RGB curve grading transform with the given style.
    pub fn create(style: GradingStyle) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_grading_rgb_curve_transform_create_with_style(style as i32) };
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
        let v =
            unsafe { ocio_sys::ocio_grading_rgb_curve_transform_get_style(self.handle.as_ptr()) };
        match v {
            1 => GradingStyle::Lin,
            2 => GradingStyle::Video,
            _ => GradingStyle::Log,
        }
    }

    /// Set the grading style, panicking on failure.
    pub fn set_style(&self, style: GradingStyle) {
        self.try_set_style(style)
            .expect("failed to set grading rgb curve style");
    }

    /// Set the grading style and surface any OCIO validation error.
    pub fn try_set_style(&self, style: GradingStyle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_style(
                self.handle.as_ptr(),
                style as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Return the current RGB curve grading values.
    pub fn value(&self) -> Result<GradingRGBCurveValue> {
        Ok(GradingRGBCurveValue {
            red: self.read_curve(RGBCurveType::Red)?,
            green: self.read_curve(RGBCurveType::Green)?,
            blue: self.read_curve(RGBCurveType::Blue)?,
            master: self.read_curve(RGBCurveType::Master)?,
        })
    }

    /// Set the RGB curve grading values.
    pub fn set_value(&self, value: &GradingRGBCurveValue) -> Result<()> {
        fn write_curve(
            transform: &GradingRGBCurveTransform,
            curve_type: RGBCurveType,
            points: &[GradingCurvePoint],
        ) -> Result<()> {
            transform.set_num_control_points(curve_type, points.len() as i32)?;
            for (index, point) in points.iter().enumerate() {
                let index = index as i32;
                transform.set_control_point(curve_type, index, point.x, point.y)?;
                transform.set_slope(curve_type, index, point.slope)?;
            }
            Ok(())
        }

        write_curve(self, RGBCurveType::Red, &value.red)?;
        write_curve(self, RGBCurveType::Green, &value.green)?;
        write_curve(self, RGBCurveType::Blue, &value.blue)?;
        write_curve(self, RGBCurveType::Master, &value.master)
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

    /// Return the number of control points for the given curve type.
    pub fn num_control_points(&self, curve_type: RGBCurveType) -> Result<i32> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_get_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return the (x, y) position of the control point at `index` for the given curve type.
    pub fn control_point(&self, curve_type: RGBCurveType, index: i32) -> Result<(f32, f32)> {
        Self::require_non_negative_index(index, "GradingRGBCurveTransform::control_point")?;
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_get_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                &mut x,
                &mut y,
            );
        }
        crate::ocio_call_status()?;
        Ok((x, y))
    }

    /// Set the number of control points for the given curve type.
    pub fn set_num_control_points(&self, curve_type: RGBCurveType, num: i32) -> Result<()> {
        if num < 0 {
            return Err(OcioError::InvalidInput(
                "GradingRGBCurveTransform::set_num_control_points: num must be non-negative"
                    .to_string(),
            ));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
                num,
            );
        }
        crate::ocio_call_status()
    }

    /// Set the (x, y) position of the control point at `index` for the given curve type.
    pub fn set_control_point(
        &self,
        curve_type: RGBCurveType,
        index: i32,
        x: f32,
        y: f32,
    ) -> Result<()> {
        Self::require_non_negative_index(index, "GradingRGBCurveTransform::set_control_point")?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                x,
                y,
            );
        }
        crate::ocio_call_status()
    }

    /// Return the slope of the control point at `index` for the given curve type.
    pub fn slope(&self, curve_type: RGBCurveType, index: i32) -> Result<f32> {
        Self::require_non_negative_index(index, "GradingRGBCurveTransform::slope")?;
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_get_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index as usize,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set the slope of the control point at `index` for the given curve type.
    pub fn set_slope(&self, curve_type: RGBCurveType, index: i32, slope: f32) -> Result<()> {
        Self::require_non_negative_index(index, "GradingRGBCurveTransform::set_slope")?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index as usize,
                slope,
            );
        }
        crate::ocio_call_status()
    }

    /// Return `true` if all slopes for the given curve type are at their default values.
    pub fn slopes_are_default(&self, curve_type: RGBCurveType) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_slopes_are_default(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return `true` if the linear-to-log conversion is bypassed.
    pub fn bypass_lin_to_log(&self) -> bool {
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_get_bypass_lin_to_log(self.handle.as_ptr())
        }
    }

    /// Set the bypass-lin-to-log flag, panicking on failure.
    pub fn set_bypass_lin_to_log(&self, bypass: bool) {
        self.try_set_bypass_lin_to_log(bypass)
            .expect("failed to set grading rgb curve bypass lin to log");
    }

    /// Set the bypass-lin-to-log flag and surface any OCIO validation error.
    pub fn try_set_bypass_lin_to_log(&self, bypass: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_bypass_lin_to_log(
                self.handle.as_ptr(),
                bypass,
            );
        }
        crate::ocio_call_status()
    }

    /// Return `true` if this transform is dynamic (values can be updated at runtime).
    pub fn is_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_grading_rgb_curve_transform_is_dynamic(self.handle.as_ptr()) }
    }

    /// Make this transform dynamic, panicking on failure.
    pub fn make_dynamic(&self) {
        self.try_make_dynamic()
            .expect("failed to make grading rgb curve dynamic");
    }

    /// Make this transform dynamic and surface any OCIO validation error.
    pub fn try_make_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_make_dynamic(self.handle.as_ptr());
        }
        crate::ocio_call_status()
    }

    /// Make this transform non-dynamic, panicking on failure.
    pub fn make_non_dynamic(&self) {
        self.try_make_non_dynamic()
            .expect("failed to make grading rgb curve non-dynamic");
    }

    /// Make this transform non-dynamic and surface any OCIO validation error.
    pub fn try_make_non_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_make_non_dynamic(self.handle.as_ptr());
        }
        crate::ocio_call_status()
    }

    /// Return the current transform direction.
    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_get_direction(self.handle.as_ptr())
        };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the transform direction, panicking on failure.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set grading rgb curve direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_transform_set_direction(
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
        t.try_set_style(GradingStyle::Lin).unwrap();
    }

    #[test]
    fn set_curve_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        let _ = t.set_num_control_points(RGBCurveType::Red, 2);
        let _ = t.set_control_point(RGBCurveType::Red, 0, 0.0, 0.0);
        let _ = t.set_control_point(RGBCurveType::Red, 1, 1.0, 1.0);
        let _ = t.set_slope(RGBCurveType::Red, 0, 1.0);
    }

    #[test]
    fn value_round_trip_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        let value = crate::grading::GradingRGBCurveValue {
            red: vec![
                crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0),
                crate::grading::GradingCurvePoint::new(1.0, 1.0, 1.0),
            ],
            green: vec![
                crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0),
                crate::grading::GradingCurvePoint::new(1.0, 1.0, 1.0),
            ],
            blue: vec![
                crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0),
                crate::grading::GradingCurvePoint::new(1.0, 1.0, 1.0),
            ],
            master: vec![
                crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0),
                crate::grading::GradingCurvePoint::new(1.0, 1.0, 1.0),
            ],
        };
        let _ = t.set_value(&value);
        let _ = t.value();
    }

    #[test]
    fn bypass_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        t.try_set_bypass_lin_to_log(true).unwrap();
        t.try_set_bypass_lin_to_log(false).unwrap();
    }

    #[test]
    fn make_dynamic_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        t.try_make_dynamic().unwrap();
        t.try_make_non_dynamic().unwrap();
    }

    #[test]
    fn direction_no_crash() {
        let t = GradingRGBCurveTransform::create(GradingStyle::Log).unwrap();
        t.try_set_direction(TransformDirection::Inverse).unwrap();
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
