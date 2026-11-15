use crate::Result;
use std::{
    ffi::{c_void, CStr},
    ptr::NonNull,
};

fn serialize_curve(handle: NonNull<c_void>, kind: i32) -> Result<String> {
    crate::clear_last_error();
    let ptr = unsafe { ocio_sys::ocio_grading_curve_serialize(handle.as_ptr(), kind) };
    crate::ocio_call_status()?;
    if ptr.is_null() {
        return Err(crate::OcioError::Ocio(
            "null grading curve serialization".into(),
        ));
    }
    Ok(unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned())
}

/// Borrowed read-only spline owned by a grading curve set.
///
/// This view does not copy the spline. Other handles to the same native spline
/// can still edit it; do not access native aliases concurrently.
///
/// ```compile_fail
/// # use ocio_rs::{GradingRGBCurve, GradingStyle, RGBCurveType};
/// let parent = GradingRGBCurve::create(GradingStyle::Lin).unwrap();
/// let view = parent.curve_ref(RGBCurveType::Red).unwrap();
/// drop(parent);
/// let _ = view.num_control_points();
/// ```
pub struct ConstGradingBSplineCurve<'a> {
    handle: NonNull<c_void>,
    owner: std::marker::PhantomData<&'a ()>,
}

/// Read-only native shared spline that can outlive its original curve set.
///
/// This retains the native spline without copying it. Mutable aliases can still
/// change its contents; native concurrent access requires synchronization.
pub struct SharedGradingBSplineCurve {
    owner: NonNull<c_void>,
    data: NonNull<c_void>,
}
impl SharedGradingBSplineCurve {
    fn from_parent(parent: NonNull<c_void>, kind: i32, hue: bool) -> Result<Self> {
        crate::clear_last_error();
        let owner =
            unsafe { ocio_sys::ocio_grading_curve_const_shared(parent.as_ptr(), kind, hue) };
        let owner = crate::handle_result(owner)?;
        let data = unsafe { ocio_sys::ocio_grading_spline_shared_data(owner.as_ptr()) };
        let Some(data) = NonNull::new(data.cast_mut()) else {
            unsafe { ocio_sys::ocio_grading_spline_shared_destroy(owner.as_ptr()) };
            return Err(crate::OcioError::Ocio("null shared grading spline".into()));
        };
        Ok(Self { owner, data })
    }
    /// Borrow this shared spline without another allocation or reference increment.
    pub fn view(&self) -> ConstGradingBSplineCurve<'_> {
        ConstGradingBSplineCurve {
            handle: self.data,
            owner: std::marker::PhantomData,
        }
    }
}
impl Drop for SharedGradingBSplineCurve {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_grading_spline_shared_destroy(self.owner.as_ptr()) }
    }
}
impl ConstGradingBSplineCurve<'_> {
    /// Return OCIO's diagnostic stream representation, not a stable interchange format.
    pub fn serialize(&self) -> Result<String> {
        serialize_curve(self.handle, 1)
    }
    /// Make an independent editable native copy.
    pub fn create_editable_copy(&self) -> Result<GradingBSplineCurve> {
        crate::clear_last_error();
        let ptr = unsafe { ocio_sys::ocio_grading_spline_const_copy(self.handle.as_ptr()) };
        Ok(GradingBSplineCurve {
            handle: crate::handle_result(ptr)?,
        })
    }
    /// Number of native control points.
    pub fn num_control_points(&self) -> Result<usize> {
        crate::clear_last_error();
        let n = unsafe { ocio_sys::ocio_grading_spline_const_count(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(n)
    }
    /// Copy one native control point.
    pub fn control_point(&self, index: usize) -> Result<(f32, f32)> {
        let (mut x, mut y) = (0.0, 0.0);
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_grading_spline_const_point(self.handle.as_ptr(), index, &mut x, &mut y)
        };
        crate::ocio_call_status()?;
        Ok((x, y))
    }
    /// Read one native slope.
    pub fn slope(&self, index: usize) -> Result<f32> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_grading_spline_const_slope(self.handle.as_ptr(), index) };
        crate::ocio_call_status()?;
        Ok(value)
    }
    /// Whether native slopes use their defaults.
    pub fn slopes_are_default(&self) -> Result<bool> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_grading_spline_const_slopes_default(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
    }
    /// Validate the native spline.
    pub fn validate(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_grading_spline_const_validate(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }
    /// Read the native spline type.
    #[cfg(feature = "v2_5")]
    pub fn spline_type(&self) -> Result<BSplineType> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_grading_spline_const_type(self.handle.as_ptr()) };
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
}

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
    /// Return OCIO's diagnostic stream representation, not a stable interchange format.
    pub fn serialize(&self) -> Result<String> {
        serialize_curve(self.handle, 0)
    }
    fn create_points_impl(
        points: &[[f32; 2]],
        kind: i32,
        mode: i32,
        validate: bool,
    ) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            let create = if validate {
                ocio_sys::ocio_grading_bspline_curve_create_from_points
            } else {
                ocio_sys::ocio_grading_bspline_curve_create_from_points_unvalidated
            };
            create(points.as_ptr(), points.len(), kind, mode)
        };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }

    /// Construct and validate a native spline from coordinates with native default slopes.
    pub fn create_from_points(points: &[[f32; 2]]) -> Result<Self> {
        Self::create_points_impl(points, 0, 0, true)
    }
    /// Construct like OCIO's point-list overload without validating the curve.
    /// Call [`Self::validate`] before using it for color processing.
    pub fn create_from_points_unvalidated(points: &[[f32; 2]]) -> Result<Self> {
        Self::create_points_impl(points, 0, 0, false)
    }

    /// Construct from coordinates with a specific spline type.
    #[cfg(feature = "v2_5")]
    pub fn create_from_points_with_spline_type(
        points: &[[f32; 2]],
        kind: BSplineType,
    ) -> Result<Self> {
        Self::create_points_impl(points, kind as i32, 1, true)
    }
    /// Construct with a spline type without validating the supplied points.
    #[cfg(feature = "v2_5")]
    pub fn create_from_points_with_spline_type_unvalidated(
        points: &[[f32; 2]],
        kind: BSplineType,
    ) -> Result<Self> {
        Self::create_points_impl(points, kind as i32, 1, false)
    }

    /// Construct from coordinates with the native type for a hue curve.
    #[cfg(feature = "v2_5")]
    pub fn create_from_points_with_hue_type(
        points: &[[f32; 2]],
        kind: crate::HueCurveType,
    ) -> Result<Self> {
        Self::create_points_impl(points, kind as i32, 2, true)
    }
    /// Construct with a hue curve type without validating the supplied points.
    #[cfg(feature = "v2_5")]
    pub fn create_from_points_with_hue_type_unvalidated(
        points: &[[f32; 2]],
        kind: crate::HueCurveType,
    ) -> Result<Self> {
        Self::create_points_impl(points, kind as i32, 2, false)
    }
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
    /// Use OCIO's native equality semantics. OCIO 2.4 does not compare slopes;
    /// newer versions also compare slopes and spline type.
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
    pub(crate) handle: NonNull<c_void>,
}
impl GradingRGBCurve {
    /// Return OCIO's diagnostic stream representation, not a stable interchange format.
    pub fn serialize(&self) -> Result<String> {
        serialize_curve(self.handle, 2)
    }
    /// Construct from read-only shared splines in OCIO enum order.
    /// OCIO copies the supplied splines into the new curve set.
    pub fn create_from_shared_curves(curves: [&SharedGradingBSplineCurve; 4]) -> Result<Self> {
        let pointers = curves.map(|curve| curve.owner.as_ptr());
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_grading_rgb_curve_object_create_from_shared_curves(pointers.as_ptr())
        };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Retain a read-only native child independently of this curve set.
    pub fn curve_shared(&self, kind: crate::RGBCurveType) -> Result<SharedGradingBSplineCurve> {
        SharedGradingBSplineCurve::from_parent(self.handle, kind as i32, false)
    }
    /// Borrow a read-only native child without copying it.
    pub fn curve_ref(&self, kind: crate::RGBCurveType) -> Result<ConstGradingBSplineCurve<'_>> {
        crate::clear_last_error();
        let ptr = unsafe {
            ocio_sys::ocio_grading_curve_const_view(self.handle.as_ptr(), kind as i32, false)
        };
        Ok(ConstGradingBSplineCurve {
            handle: crate::handle_result(ptr.cast_mut())?,
            owner: std::marker::PhantomData,
        })
    }
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
    /// Use OCIO's native equality semantics, which vary by OCIO version.
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
    pub(crate) handle: NonNull<c_void>,
}
#[cfg(feature = "v2_5")]
impl GradingHueCurve {
    /// Return OCIO's diagnostic stream representation, not a stable interchange format.
    pub fn serialize(&self) -> Result<String> {
        serialize_curve(self.handle, 3)
    }
    /// Construct from read-only shared splines in OCIO enum order.
    /// OCIO copies and validates the supplied splines.
    pub fn create_from_shared_curves(curves: [&SharedGradingBSplineCurve; 8]) -> Result<Self> {
        let pointers = curves.map(|curve| curve.owner.as_ptr());
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_grading_hue_curve_object_create_from_shared_curves(pointers.as_ptr())
        };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Retain a read-only native child independently of this curve set.
    pub fn curve_shared(&self, kind: crate::HueCurveType) -> Result<SharedGradingBSplineCurve> {
        SharedGradingBSplineCurve::from_parent(self.handle, kind as i32, true)
    }
    /// Borrow a read-only native child without copying it.
    pub fn curve_ref(&self, kind: crate::HueCurveType) -> Result<ConstGradingBSplineCurve<'_>> {
        crate::clear_last_error();
        let ptr = unsafe {
            ocio_sys::ocio_grading_curve_const_view(self.handle.as_ptr(), kind as i32, true)
        };
        Ok(ConstGradingBSplineCurve {
            handle: crate::handle_result(ptr.cast_mut())?,
            owner: std::marker::PhantomData,
        })
    }
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
    /// Use OCIO's native equality semantics, which vary by OCIO version.
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
impl GradingRGBCurve {
    /// Construct a native curve set from 4 splines in OCIO enum order.
    pub fn create_from_curves(curves: [&GradingBSplineCurve; 4]) -> Result<Self> {
        let pointers = curves.map(|curve| curve.handle.as_ptr());
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_grading_rgb_curve_object_create_from_curves(pointers.as_ptr())
        };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
}

#[cfg(feature = "v2_5")]
impl GradingHueCurve {
    /// Construct a native curve set from 8 splines in OCIO enum order.
    pub fn create_from_curves(curves: [&GradingBSplineCurve; 8]) -> Result<Self> {
        let pointers = curves.map(|curve| curve.handle.as_ptr());
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_grading_hue_curve_object_create_from_curves(pointers.as_ptr())
        };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
}
