use std::ffi::c_void;
use std::ptr::NonNull;

use crate::grading::{GradingCurvePoint, GradingHueCurveValue};
use crate::{GradingStyle, HSYTransformStyle, HueCurveType, OcioError, Result, TransformDirection};
use ocio_sys;

/// OCIO grading transform that applies hue-dependent curve adjustments.
pub struct GradingHueCurveTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl GradingHueCurveTransform {
    fn read_curve(&self, curve_type: HueCurveType) -> Result<Vec<GradingCurvePoint>> {
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

    /// Create a hue curve grading transform with the given style (alias for [`Self::create`]).
    pub fn create_with_style(style: GradingStyle) -> Result<Self> {
        Self::create(style)
    }

    /// Create a new hue curve grading transform with the given style.
    pub fn create(style: GradingStyle) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_grading_hue_curve_transform_create_with_style(style as i32) };
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
            unsafe { ocio_sys::ocio_grading_hue_curve_transform_get_style(self.handle.as_ptr()) };
        match v {
            1 => GradingStyle::Lin,
            2 => GradingStyle::Video,
            _ => GradingStyle::Log,
        }
    }

    /// Set the grading style, panicking on failure.
    pub fn set_style(&self, style: GradingStyle) {
        self.try_set_style(style)
            .expect("failed to set grading hue curve style");
    }

    /// Set the grading style and surface any OCIO validation error.
    pub fn try_set_style(&self, style: GradingStyle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_set_style(
                self.handle.as_ptr(),
                style as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Return the current hue curve grading values.
    pub fn value(&self) -> Result<GradingHueCurveValue> {
        Ok(GradingHueCurveValue {
            hue_hue: self.read_curve(HueCurveType::HueHue)?,
            hue_sat: self.read_curve(HueCurveType::HueSat)?,
            hue_lum: self.read_curve(HueCurveType::HueLum)?,
            lum_sat: self.read_curve(HueCurveType::LumSat)?,
        })
    }

    /// Set the hue curve grading values.
    pub fn set_value(&self, value: &GradingHueCurveValue) -> Result<()> {
        fn write_curve(
            transform: &GradingHueCurveTransform,
            curve_type: HueCurveType,
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

        write_curve(self, HueCurveType::HueHue, &value.hue_hue)?;
        write_curve(self, HueCurveType::HueSat, &value.hue_sat)?;
        write_curve(self, HueCurveType::HueLum, &value.hue_lum)?;
        write_curve(self, HueCurveType::LumSat, &value.lum_sat)
    }

    #[deprecated(
        since = "0.2.0",
        note = "prefer value() to read a safe Rust snapshot of the grading curve"
    )]
    /// Returns an owned legacy OCIO grading-curve handle.
    ///
    /// The caller must eventually release a non-null handle with
    /// [`ocio_sys::ocio_grading_hue_curve_destroy`]. The returned handle owns
    /// an independent OCIO value snapshot and remains valid after this
    /// transform is dropped.
    pub fn raw_value_handle(&self) -> *mut c_void {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_get_value(self.handle.as_ptr()) }
    }

    /// # Safety
    /// `values` must point to a valid OCIO grading-hue-curve value object for
    /// the active ABI. It remains owned by the caller; this method borrows it
    /// only for the duration of the call and never destroys it.
    #[deprecated(
        since = "0.2.0",
        note = "prefer set_value(&GradingHueCurveValue) instead of passing a raw OCIO handle"
    )]
    pub unsafe fn set_value_raw(&self, values: *mut c_void) {
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_set_value(self.handle.as_ptr(), values);
        }
    }

    /// Return the number of control points for the given curve type.
    pub fn num_control_points(&self, curve_type: HueCurveType) -> Result<i32> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_get_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return the (x, y) position of the control point at `index` for the given curve type.
    pub fn control_point(&self, curve_type: HueCurveType, index: i32) -> Result<(f32, f32)> {
        Self::require_non_negative_index(index, "GradingHueCurveTransform::control_point")?;
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_get_control_point(
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
    pub fn set_num_control_points(&self, curve_type: HueCurveType, num: i32) -> Result<()> {
        if num < 0 {
            return Err(OcioError::InvalidInput(
                "GradingHueCurveTransform::set_num_control_points: num must be non-negative"
                    .to_string(),
            ));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_set_num_control_points(
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
        curve_type: HueCurveType,
        index: i32,
        x: f32,
        y: f32,
    ) -> Result<()> {
        Self::require_non_negative_index(index, "GradingHueCurveTransform::set_control_point")?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_set_control_point(
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
    pub fn slope(&self, curve_type: HueCurveType, index: i32) -> Result<f32> {
        Self::require_non_negative_index(index, "GradingHueCurveTransform::slope")?;
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_get_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index as usize,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set the slope of the control point at `index` for the given curve type.
    pub fn set_slope(&self, curve_type: HueCurveType, index: i32, slope: f32) -> Result<()> {
        Self::require_non_negative_index(index, "GradingHueCurveTransform::set_slope")?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_set_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index as usize,
                slope,
            );
        }
        crate::ocio_call_status()
    }

    /// Return `true` if all slopes for the given curve type are at their default values.
    pub fn slopes_are_default(&self, curve_type: HueCurveType) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_slopes_are_default(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return the current RGB-to-HSY conversion style.
    pub fn rgb_to_hsy(&self) -> HSYTransformStyle {
        let style = unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_get_rgb_to_hsy(self.handle.as_ptr())
        };
        match style {
            0 => HSYTransformStyle::None,
            _ => HSYTransformStyle::Default,
        }
    }

    /// Set the RGB-to-HSY conversion style, panicking on failure.
    pub fn set_rgb_to_hsy(&self, style: HSYTransformStyle) {
        self.try_set_rgb_to_hsy(style)
            .expect("failed to set grading hue curve rgb to hsy");
    }

    /// Set the RGB-to-HSY conversion style and surface any OCIO validation error.
    pub fn try_set_rgb_to_hsy(&self, style: HSYTransformStyle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_set_rgb_to_hsy(
                self.handle.as_ptr(),
                style as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Return `true` if this transform is dynamic (values can be updated at runtime).
    pub fn is_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_grading_hue_curve_transform_is_dynamic(self.handle.as_ptr()) }
    }

    /// Make this transform dynamic, panicking on failure.
    pub fn make_dynamic(&self) {
        self.try_make_dynamic()
            .expect("failed to make grading hue curve dynamic");
    }

    /// Make this transform dynamic and surface any OCIO validation error.
    pub fn try_make_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_make_dynamic(self.handle.as_ptr());
        }
        crate::ocio_call_status()
    }

    /// Make this transform non-dynamic, panicking on failure.
    pub fn make_non_dynamic(&self) {
        self.try_make_non_dynamic()
            .expect("failed to make grading hue curve non-dynamic");
    }

    /// Make this transform non-dynamic and surface any OCIO validation error.
    pub fn try_make_non_dynamic(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_make_non_dynamic(self.handle.as_ptr());
        }
        crate::ocio_call_status()
    }

    /// Return the current transform direction.
    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_get_direction(self.handle.as_ptr())
        };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the transform direction, panicking on failure.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set grading hue curve direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_hue_curve_transform_set_direction(
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
            ocio_sys::ocio_grading_hue_curve_transform_equals(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            )
        }
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
        let _ = t.num_control_points(HueCurveType::HueHue);
        let _ = t.control_point(HueCurveType::HueHue, 0);
        let _ = t.slope(HueCurveType::HueHue, 0);
        let _ = t.slopes_are_default(HueCurveType::HueHue);
        let _ = t.rgb_to_hsy();
        let _ = t.is_dynamic();
        let _ = t.direction();
    }

    #[test]
    fn set_style_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        t.try_set_style(GradingStyle::Lin).unwrap();
    }

    #[test]
    fn set_curve_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        let _ = t.set_num_control_points(HueCurveType::HueHue, 2);
        let _ = t.set_control_point(HueCurveType::HueHue, 0, 0.0, 0.0);
        let _ = t.set_control_point(HueCurveType::HueHue, 1, 1.0, 1.0);
        let _ = t.set_slope(HueCurveType::HueHue, 0, 1.0);
    }

    #[test]
    fn value_round_trip_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        let value = crate::grading::GradingHueCurveValue {
            hue_hue: vec![
                crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0),
                crate::grading::GradingCurvePoint::new(1.0, 1.0, 1.0),
            ],
            hue_sat: vec![
                crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0),
                crate::grading::GradingCurvePoint::new(1.0, 1.0, 1.0),
            ],
            hue_lum: vec![
                crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0),
                crate::grading::GradingCurvePoint::new(1.0, 1.0, 1.0),
            ],
            lum_sat: vec![
                crate::grading::GradingCurvePoint::new(0.0, 0.0, 1.0),
                crate::grading::GradingCurvePoint::new(1.0, 1.0, 1.0),
            ],
        };
        let _ = t.set_value(&value);
        let _ = t.value();
    }

    #[test]
    fn rgb_to_hsy_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        t.try_set_rgb_to_hsy(HSYTransformStyle::None).unwrap();
        t.try_set_rgb_to_hsy(HSYTransformStyle::Default).unwrap();
    }

    #[test]
    fn make_dynamic_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        t.try_make_dynamic().unwrap();
        t.try_make_non_dynamic().unwrap();
    }

    #[test]
    fn direction_no_crash() {
        let t = GradingHueCurveTransform::create(GradingStyle::Log).unwrap();
        t.try_set_direction(TransformDirection::Inverse).unwrap();
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
