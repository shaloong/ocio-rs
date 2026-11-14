use crate::Result;
use std::{ffi::c_void, ptr::NonNull};

/// Type of the monotonic or periodic native grading spline.
#[cfg(feature = "v2_5")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum BSplineType {
    /// RGB master and component spline.
    Standard = 0,
    /// Diagonal spline.
    Diagonal = 1,
    /// Periodic hue-to-hue spline.
    HueHue = 2,
    /// Periodic spline with identity 1.
    PeriodicOne = 3,
    /// Periodic spline with identity 0.
    PeriodicZero = 4,
    /// Horizontal spline with identity 1.
    HorizontalOne = 5,
}

/// An independently owned native OCIO grading spline.
pub struct GradingBSplineCurve {
    pub(crate) handle: NonNull<c_void>,
}
impl GradingBSplineCurve {
    /// Create a native spline with the specified number of points.
    pub fn create(count: usize) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_grading_bspline_curve_create(count) };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Create a spline with an explicit native spline type.
    #[cfg(feature = "v2_5")]
    pub fn create_with_spline_type(count: usize, kind: BSplineType) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_grading_bspline_curve_create_with_spline_type(count, kind as i32)
        };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Create the native spline appropriate for a hue-curve type.
    #[cfg(feature = "v2_5")]
    pub fn create_with_hue_type(count: usize, kind: crate::HueCurveType) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_grading_bspline_curve_create_with_hue_type(count, kind as i32)
        };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Create an independent editable native copy.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_grading_bspline_curve_create_editable_copy(self.handle.as_ptr())
        };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Number of control points.
    pub fn num_control_points(&self) -> Result<usize> {
        crate::clear_last_error();
        let count = unsafe {
            ocio_sys::ocio_grading_bspline_curve_get_num_control_points(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(count)
    }
    /// Resize the curve according to native OCIO validation.
    pub fn set_num_control_points(&mut self, count: usize) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_bspline_curve_set_num_control_points(self.handle.as_ptr(), count)
        };
        crate::ocio_call_status()
    }
    /// Read the coordinates of a control point.
    pub fn control_point(&self, index: usize) -> Result<(f32, f32)> {
        let (mut x, mut y) = (0., 0.);
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_bspline_curve_get_control_point(
                self.handle.as_ptr(),
                index,
                &mut x,
                &mut y,
            )
        };
        crate::ocio_call_status()?;
        Ok((x, y))
    }
    /// Update the coordinates of a control point.
    pub fn set_control_point(&mut self, index: usize, x: f32, y: f32) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_bspline_curve_set_control_point(
                self.handle.as_ptr(),
                index,
                x,
                y,
            )
        };
        crate::ocio_call_status()
    }
    /// Read the native slope at a control point.
    pub fn slope(&self, index: usize) -> Result<f32> {
        crate::clear_last_error();
        let slope =
            unsafe { ocio_sys::ocio_grading_bspline_curve_get_slope(self.handle.as_ptr(), index) };
        crate::ocio_call_status()?;
        Ok(slope)
    }
    /// Set the native slope at a control point.
    pub fn set_slope(&mut self, index: usize, slope: f32) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_bspline_curve_set_slope(self.handle.as_ptr(), index, slope)
        };
        crate::ocio_call_status()
    }
    /// Whether all native slopes use their defaults.
    pub fn slopes_are_default(&self) -> Result<bool> {
        crate::clear_last_error();
        let result = unsafe {
            ocio_sys::ocio_grading_bspline_curve_slopes_are_default(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(result)
    }
    /// Ask OCIO to validate curve ordering and bounds.
    pub fn validate(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_grading_bspline_curve_validate(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }
    /// Compare native curve values.
    pub fn equals(&self, other: &Self) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_grading_bspline_curve_equals(self.handle.as_ptr(), other.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(value)
    }
    /// Read the native spline type.
    #[cfg(feature = "v2_5")]
    pub fn spline_type(&self) -> Result<BSplineType> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_grading_bspline_curve_get_spline_type(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        match value {
            0 => Ok(BSplineType::Standard),
            1 => Ok(BSplineType::Diagonal),
            2 => Ok(BSplineType::HueHue),
            3 => Ok(BSplineType::PeriodicOne),
            4 => Ok(BSplineType::PeriodicZero),
            5 => Ok(BSplineType::HorizontalOne),
            _ => Err(crate::OcioError::Ocio(format!(
                "Unknown spline type: {value}"
            ))),
        }
    }
    /// Set the native spline type.
    #[cfg(feature = "v2_5")]
    pub fn set_spline_type(&mut self, kind: BSplineType) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_bspline_curve_set_spline_type(self.handle.as_ptr(), kind as i32)
        };
        crate::ocio_call_status()
    }
}
impl Drop for GradingBSplineCurve {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_grading_bspline_curve_destroy(self.handle.as_ptr()) }
    }
}
/// An independently owned native RGB grading curve set.
pub struct GradingRGBCurve {
    handle: NonNull<c_void>,
}
impl GradingRGBCurve {
    /// Create the native default curves for a grading style.
    pub fn create(style: crate::GradingStyle) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_grading_rgb_curve_object_create(style as i32) };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Get an independent copy of a transform's curve set.
    pub fn from_transform(transform: &crate::transform::GradingRGBCurveTransform) -> Result<Self> {
        use crate::transform::TransformHandle;
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_grading_rgb_curve_object_from_transform(transform.as_ptr()) };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Set a transform's native value in one validated update.
    pub fn set_transform(
        &self,
        transform: &crate::transform::GradingRGBCurveTransform,
    ) -> Result<()> {
        use crate::transform::TransformHandle;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_rgb_curve_object_set_transform(
                self.handle.as_ptr(),
                transform.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }
    /// Create an independent editable copy.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_grading_rgb_curve_object_create_editable_copy(self.handle.as_ptr())
        };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Access a native curve; the returned handle keeps it alive independently.
    pub fn curve(&mut self, kind: crate::RGBCurveType) -> Result<GradingBSplineCurve> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_grading_rgb_curve_object_get_curve(self.handle.as_ptr(), kind as i32)
        };
        Ok(GradingBSplineCurve {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Check native identity semantics.
    pub fn is_identity(&self) -> Result<bool> {
        crate::clear_last_error();
        let result =
            unsafe { ocio_sys::ocio_grading_rgb_curve_object_is_identity(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(result)
    }
    /// Validate all constituent curves.
    pub fn validate(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_grading_rgb_curve_object_validate(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }
    /// Compare all native curve values.
    pub fn equals(&self, other: &Self) -> Result<bool> {
        crate::clear_last_error();
        let result = unsafe {
            ocio_sys::ocio_grading_rgb_curve_object_equals(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(result)
    }
}
impl Drop for GradingRGBCurve {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_grading_rgb_curve_object_destroy(self.handle.as_ptr()) }
    }
}

#[cfg(feature = "v2_5")]
/// An independently owned native HUE grading curve set.
pub struct GradingHueCurve {
    handle: NonNull<c_void>,
}
#[cfg(feature = "v2_5")]
impl GradingHueCurve {
    /// Return the native spline type used for a hue-curve family.
    pub fn b_spline_type_for_hue_curve_type(kind: crate::HueCurveType) -> Result<BSplineType> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_grading_hue_curve_object_get_b_spline_type_for_hue_curve_type(
                kind as i32,
            )
        };
        crate::ocio_call_status()?;
        match value {
            0 => Ok(BSplineType::Standard),
            1 => Ok(BSplineType::Diagonal),
            2 => Ok(BSplineType::HueHue),
            3 => Ok(BSplineType::PeriodicOne),
            4 => Ok(BSplineType::PeriodicZero),
            5 => Ok(BSplineType::HorizontalOne),
            _ => Err(crate::OcioError::Ocio(format!(
                "Unknown spline type: {value}"
            ))),
        }
    }
    /// Create the native default curves for a grading style.
    pub fn create(style: crate::GradingStyle) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_grading_hue_curve_object_create(style as i32) };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Get an independent copy of a transform's curve set.
    pub fn from_transform(transform: &crate::transform::GradingHueCurveTransform) -> Result<Self> {
        use crate::transform::TransformHandle;
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_grading_hue_curve_object_from_transform(transform.as_ptr()) };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Set a transform's native value in one validated update.
    pub fn set_transform(
        &self,
        transform: &crate::transform::GradingHueCurveTransform,
    ) -> Result<()> {
        use crate::transform::TransformHandle;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_hue_curve_object_set_transform(
                self.handle.as_ptr(),
                transform.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }
    /// Create an independent editable copy.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_grading_hue_curve_object_create_editable_copy(self.handle.as_ptr())
        };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Access a native curve; the returned handle keeps it alive independently.
    pub fn curve(&mut self, kind: crate::HueCurveType) -> Result<GradingBSplineCurve> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_grading_hue_curve_object_get_curve(self.handle.as_ptr(), kind as i32)
        };
        Ok(GradingBSplineCurve {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Check native identity semantics.
    pub fn is_identity(&self) -> Result<bool> {
        crate::clear_last_error();
        let result =
            unsafe { ocio_sys::ocio_grading_hue_curve_object_is_identity(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(result)
    }
    /// Validate all constituent curves.
    pub fn validate(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_grading_hue_curve_object_validate(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }
    /// Compare all native curve values.
    pub fn equals(&self, other: &Self) -> Result<bool> {
        crate::clear_last_error();
        let result = unsafe {
            ocio_sys::ocio_grading_hue_curve_object_equals(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(result)
    }
    /// Check whether native curve-only drawing is enabled.
    pub fn draw_curve_only(&self) -> Result<bool> {
        crate::clear_last_error();
        let result = unsafe {
            ocio_sys::ocio_grading_hue_curve_object_get_draw_curve_only(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(result)
    }
    /// Toggle the native curve-only drawing mode.
    pub fn set_draw_curve_only(&mut self, value: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_hue_curve_object_set_draw_curve_only(self.handle.as_ptr(), value)
        };
        crate::ocio_call_status()
    }
}
#[cfg(feature = "v2_5")]
impl Drop for GradingHueCurve {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_grading_hue_curve_object_destroy(self.handle.as_ptr()) }
    }
}
