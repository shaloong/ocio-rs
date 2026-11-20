//! OCIO transform wrappers.
//!
//! Each struct in this module wraps a single OpenColorIO transform type
//! (e.g. [`ColorSpaceTransform`], [`GradingRGBCurveTransform`]) and exposes
//! safe Rust accessors for its properties.
//!
//! All transform types follow a consistent pattern:
//!
//! - **Constructor**: [`create`](std::result::Result) returns `Result<Self>`.
//! - **Getters**: Return the current property value (often infallible).
//! - **Setters**: Legacy void setters panic on error; checked `try_*` variants
//!   return `Result<()>` and surface OCIO validation errors.
//! - **Direction**: Every transform that supports forward/inverse evaluation
//!   provides `direction()` / `try_set_direction()`. Import [`TransformHandle`]
//!   for the shared fallible `try_direction()` query on editable transforms.
//! - **Editable copy**: Native copy semantics apply. In particular, copying a
//!   [`GroupTransform`] copies its container but keeps its children shared;
//!   use [`GroupTransform::create_deep_copy`] for an independent built-in graph.

mod allocation;
mod builtin;
mod cdl;
mod color_space;
mod display_view;
mod exponent;
mod exponent_with_linear;
mod exposure_contrast;
mod file;
mod fixed_function;
#[cfg(feature = "v2_5")]
mod grading_hue_curve;
mod grading_primary;
mod grading_rgb_curve;
mod grading_tone;
mod group;
mod log;
mod log_affine;
mod log_camera;
mod look_transform;
mod lut1d;
mod lut3d;
mod matrix;
mod range;

use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;

fn transform_diagnostic_text(handle: *mut c_void, is_const: bool) -> crate::Result<String> {
    crate::clear_last_error();
    let ptr = unsafe { ocio_sys::ocio_transform_diagnostic_text(handle, is_const) };
    crate::ocio_call_status()?;
    unsafe { crate::cstr_to_opt_string(ptr) }
        .ok_or_else(|| crate::OcioError::Ocio("null transform diagnostic text".into()))
}

/// Shared immutable native transform, which can be processed without copying.
pub struct ConstTransform {
    pub(crate) handle: NonNull<c_void>,
}
impl ConstTransform {
    /// Count children of a native group without copying it. A non-group
    /// transform returns an error.
    pub fn group_child_count(&self) -> crate::Result<i32> {
        crate::clear_last_error();
        let count =
            unsafe { ocio_sys::ocio_const_transform_group_child_count(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(count)
    }

    /// Retain one native group child without copying the group or transform.
    /// Invalid indices and non-group handles return errors. The returned
    /// child can outlive the group.
    pub fn group_child(&self, index: i32) -> crate::Result<Option<Self>> {
        Self::from_parent(self.handle.as_ptr(), 12, index)
    }

    /// Return OCIO's diagnostic representation, which may include file paths.
    /// This text is not a stable interchange format.
    pub fn diagnostic_text(&self) -> crate::Result<String> {
        transform_diagnostic_text(self.handle.as_ptr(), true)
    }
    pub(crate) fn from_parent(
        parent: *mut c_void,
        kind: i32,
        direction: i32,
    ) -> crate::Result<Option<Self>> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_get_const_child_transform(parent, kind, direction) };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| Self { handle }))
    }
    /// Version-independent discriminant corresponding to [`crate::TransformType`].
    pub fn type_tag(&self) -> crate::Result<i32> {
        crate::clear_last_error();
        let result = unsafe { ocio_sys::ocio_const_transform_get_type(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(result)
    }
    /// Query the native direction, rejecting unrecognized values.
    pub fn try_direction(&self) -> crate::Result<crate::TransformDirection> {
        crate::clear_last_error();
        let raw = unsafe { ocio_sys::ocio_const_transform_get_direction(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        direction_from_raw(raw)
    }
    /// Validate this transform using OCIO without creating an editable copy.
    ///
    /// This checks the transform's parameters; it does not build a processor
    /// or resolve external files.
    pub fn validate(&self) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_const_transform_validate(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    /// Create an editable copy using the concrete transform's native semantics.
    /// A copied group shares its children with the original. For an independent
    /// built-in group graph, use [`GroupTransform::create_deep_copy`].
    pub fn create_editable_copy(&self) -> crate::Result<Transform> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_const_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        transform_from_raw_handle(handle)
            .ok_or_else(|| crate::OcioError::Ocio("unsupported or null transform copy".into()))
    }

    /// Access existing low-level getters without copying the native transform.
    ///
    /// This creates a temporary typed FFI handle sharing the same native
    /// object; it is destroyed when `read` returns or unwinds. Use `type_tag`
    /// to select getters for the concrete transform type. An unsupported or
    /// inconsistent native type returns an error. Errors from FFI calls inside
    /// `read` must be handled by the callback itself.
    ///
    /// # Safety
    /// Call only FFI operations corresponding to native `const` methods. Do
    /// not mutate the transform or its descendants, and do not access mutable
    /// aliases concurrently. The handle, descendant handles and any pointers
    /// or references derived from them must not escape `read`. Native getters
    /// may have additional pointer, buffer, and synchronization requirements.
    /// In particular, `ocio_transform_get_format_metadata` selects the native
    /// mutable overload; use [`Self::with_readonly_metadata`] instead.
    pub unsafe fn with_readonly_raw_handle<R>(
        &self,
        read: impl FnOnce(*mut c_void) -> R,
    ) -> crate::Result<R> {
        struct RawHandle(NonNull<c_void>);
        impl Drop for RawHandle {
            fn drop(&mut self) {
                unsafe { ocio_sys::ocio_transform_destroy(self.0.as_ptr()) }
            }
        }

        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_const_transform_create_readonly_handle(self.handle.as_ptr()) };
        let handle = RawHandle(crate::handle_result(handle)?);
        Ok(read(handle.0.as_ptr()))
    }

    /// Inspect native const metadata without copying the transform or metadata.
    /// The callback receives `None` for transform types without metadata.
    ///
    /// # Safety
    /// Do not mutate this transform or its metadata through any alias while
    /// `read` runs. Neither borrowed metadata nor any child handles, pointers,
    /// or references derived from it may escape the callback.
    pub unsafe fn with_readonly_metadata<R>(
        &self,
        read: impl FnOnce(Option<&crate::ConstFormatMetadata>) -> R,
    ) -> crate::Result<R> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_const_transform_get_const_metadata(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        let metadata = NonNull::new(handle).map(|handle| crate::ConstFormatMetadata { handle });
        Ok(read(metadata.as_ref()))
    }
}
impl Drop for ConstTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_const_transform_destroy(self.handle.as_ptr()) }
    }
}

pub use allocation::AllocationTransform;
pub use builtin::BuiltinTransform;
pub use cdl::CDLTransform;
pub use color_space::ColorSpaceTransform;
pub use display_view::DisplayViewTransform;
pub use exponent::ExponentTransform;
pub use exponent_with_linear::ExponentWithLinearTransform;
pub use exposure_contrast::ExposureContrastTransform;
pub use file::FileTransform;
pub use fixed_function::FixedFunctionTransform;
#[cfg(feature = "v2_5")]
pub use grading_hue_curve::GradingHueCurveTransform;
pub use grading_primary::GradingPrimaryTransform;
pub use grading_rgb_curve::GradingRGBCurveTransform;
pub use grading_tone::GradingToneTransform;
pub use group::GroupTransform;
pub use log::LogTransform;
pub use log_affine::LogAffineTransform;
pub use log_camera::LogCameraTransform;
pub use look_transform::LookTransform;
pub use lut1d::Lut1DTransform;
pub use lut3d::Lut3DTransform;
pub use matrix::MatrixTransform;
pub use range::RangeTransform;

mod sealed {
    pub trait Sealed {}
}

macro_rules! seal_transform_handles {
    ($($ty:ty),+ $(,)?) => { $(impl sealed::Sealed for $ty {})+ };
}
seal_transform_handles!(
    FileTransform,
    CDLTransform,
    ExponentTransform,
    ExponentWithLinearTransform,
    MatrixTransform,
    LogTransform,
    RangeTransform,
    GroupTransform,
    BuiltinTransform,
    FixedFunctionTransform,
    Lut1DTransform,
    Lut3DTransform,
    ExposureContrastTransform,
    ColorSpaceTransform,
    LookTransform,
    DisplayViewTransform,
    AllocationTransform,
    LogAffineTransform,
    LogCameraTransform,
    GradingRGBCurveTransform,
    GradingPrimaryTransform,
    GradingToneTransform,
    Transform,
);
#[cfg(feature = "v2_5")]
seal_transform_handles!(GradingHueCurveTransform);

fn direction_from_raw(raw: i32) -> crate::Result<crate::TransformDirection> {
    match raw {
        0 => Ok(crate::TransformDirection::Forward),
        1 => Ok(crate::TransformDirection::Inverse),
        _ => Err(crate::OcioError::ValidationFailed(format!(
            "unknown OCIO transform direction {raw}"
        ))),
    }
}

pub(crate) fn checked_native_count(count: i32, operation: &'static str) -> crate::Result<usize> {
    usize::try_from(count).map_err(|_| {
        crate::OcioError::ValidationFailed(format!("{operation}: negative native count {count}"))
    })
}

pub(crate) fn checked_curve_count(count: usize) -> crate::Result<i32> {
    i32::try_from(count)
        .map_err(|_| crate::OcioError::InvalidInput("too many control points".into()))
}

pub(crate) fn reserve_native_items<T>(
    count: usize,
    operation: &'static str,
) -> crate::Result<Vec<T>> {
    let mut result = Vec::new();
    result.try_reserve_exact(count).map_err(|error| {
        crate::OcioError::ValidationFailed(format!(
            "{operation}: cannot allocate {count} items: {error}"
        ))
    })?;
    Ok(result)
}

#[cfg(test)]
#[test]
fn direction_conversion_rejects_unrecognized_native_values() {
    assert_eq!(
        direction_from_raw(0).unwrap(),
        crate::TransformDirection::Forward
    );
    assert_eq!(
        direction_from_raw(1).unwrap(),
        crate::TransformDirection::Inverse
    );
    assert!(direction_from_raw(-1).is_err());
    assert!(direction_from_raw(2).is_err());
}

#[cfg(test)]
#[test]
fn native_collection_count_validation_rejects_negative_and_capacity_overflow() {
    assert!(checked_native_count(-1, "test").is_err());
    assert_eq!(checked_native_count(0, "test").unwrap(), 0);
    assert!(reserve_native_items::<u64>(usize::MAX, "test").is_err());
    assert_eq!(checked_curve_count(i32::MAX as usize).unwrap(), i32::MAX);
    assert!(checked_curve_count(i32::MAX as usize + 1).is_err());
}

/// A native transform handle supplied by this crate.
///
/// This trait is sealed: arbitrary raw pointers cannot satisfy the internal
/// bridge ownership and type invariants. Use the built-in transform wrappers or
/// [`Transform`] in generic code.
///
/// ```compile_fail
/// use ocio_rs::transform::TransformHandle;
/// struct Forged;
/// impl TransformHandle for Forged {
///     fn as_ptr(&self) -> *mut std::ffi::c_void { std::ptr::null_mut() }
/// }
/// ```
pub trait TransformHandle: sealed::Sealed {
    fn as_ptr(&self) -> *mut c_void;

    /// Return OCIO's diagnostic representation, which may include file paths.
    /// This text is not a stable interchange format.
    fn diagnostic_text(&self) -> crate::Result<String> {
        transform_diagnostic_text(self.as_ptr(), false)
    }

    /// Return the version-independent tag corresponding to [`crate::TransformType`].
    fn try_type_tag(&self) -> crate::Result<i32> {
        let tag = unsafe { ocio_sys::ocio_transform_get_transform_type(self.as_ptr()) };
        if tag < 0 {
            Err(crate::OcioError::Ocio(
                "native transform type unavailable".into(),
            ))
        } else {
            Ok(tag)
        }
    }

    /// Change the native evaluation direction without modifying the stored parameters.
    fn try_set_direction(&mut self, direction: crate::TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_transform_set_direction(self.as_ptr(), direction as i32) };
        crate::ocio_call_status()
    }

    /// Validate the native transform without copying it or building a processor.
    fn try_validate(&self) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_transform_validate(self.as_ptr()) };
        crate::ocio_call_status()
    }

    /// Create a native editable copy, preserving the concrete transform type.
    /// A copied group shares its children with the original. For an independent
    /// built-in group graph, use [`GroupTransform::create_deep_copy`].
    fn try_create_editable_copy(&self) -> crate::Result<Transform> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.as_ptr()) };
        crate::ocio_call_status()?;
        transform_from_raw_handle(handle)
            .ok_or_else(|| crate::OcioError::Ocio("unsupported or null transform copy".into()))
    }

    /// Query the native direction with validation, for any supported transform.
    fn try_direction(&self) -> crate::Result<crate::TransformDirection> {
        crate::clear_last_error();
        let raw = unsafe { ocio_sys::ocio_transform_get_direction(self.as_ptr()) };
        crate::ocio_call_status()?;
        direction_from_raw(raw)
    }
}

impl TransformHandle for FileTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for CDLTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for ExponentTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for ExponentWithLinearTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for MatrixTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for LogTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for RangeTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for GroupTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for BuiltinTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for FixedFunctionTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for Lut1DTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for Lut3DTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for ExposureContrastTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for ColorSpaceTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for LookTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for DisplayViewTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for AllocationTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for LogAffineTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for LogCameraTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for GradingRGBCurveTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
#[cfg(feature = "v2_5")]
impl TransformHandle for GradingHueCurveTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for GradingPrimaryTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}
impl TransformHandle for GradingToneTransform {
    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

pub enum Transform {
    File(FileTransform),
    CDL(CDLTransform),
    Exponent(ExponentTransform),
    ExponentWithLinear(ExponentWithLinearTransform),
    Matrix(MatrixTransform),
    Log(LogTransform),
    Range(RangeTransform),
    Group(GroupTransform),
    Builtin(BuiltinTransform),
    FixedFunction(FixedFunctionTransform),
    Lut1D(Lut1DTransform),
    Lut3D(Lut3DTransform),
    ExposureContrast(ExposureContrastTransform),
    ColorSpace(ColorSpaceTransform),
    Look(LookTransform),
    DisplayView(DisplayViewTransform),
    Allocation(AllocationTransform),
    LogAffine(LogAffineTransform),
    LogCamera(LogCameraTransform),
    GradingPrimary(GradingPrimaryTransform),
    GradingRGBCurve(GradingRGBCurveTransform),
    #[cfg(feature = "v2_5")]
    GradingHueCurve(GradingHueCurveTransform),
    GradingTone(GradingToneTransform),
}

impl TransformHandle for Transform {
    fn as_ptr(&self) -> *mut c_void {
        match self {
            Transform::File(t) => t.as_ptr(),
            Transform::CDL(t) => t.as_ptr(),
            Transform::Exponent(t) => t.as_ptr(),
            Transform::ExponentWithLinear(t) => t.as_ptr(),
            Transform::Matrix(t) => t.as_ptr(),
            Transform::Log(t) => t.as_ptr(),
            Transform::Range(t) => t.as_ptr(),
            Transform::Group(t) => t.as_ptr(),
            Transform::Builtin(t) => t.as_ptr(),
            Transform::FixedFunction(t) => t.as_ptr(),
            Transform::Lut1D(t) => t.as_ptr(),
            Transform::Lut3D(t) => t.as_ptr(),
            Transform::ExposureContrast(t) => t.as_ptr(),
            Transform::ColorSpace(t) => t.as_ptr(),
            Transform::Look(t) => t.as_ptr(),
            Transform::DisplayView(t) => t.as_ptr(),
            Transform::Allocation(t) => t.as_ptr(),
            Transform::LogAffine(t) => t.as_ptr(),
            Transform::LogCamera(t) => t.as_ptr(),
            Transform::GradingPrimary(t) => t.as_ptr(),
            Transform::GradingRGBCurve(t) => t.as_ptr(),
            #[cfg(feature = "v2_5")]
            Transform::GradingHueCurve(t) => t.as_ptr(),
            Transform::GradingTone(t) => t.as_ptr(),
        }
    }
}

/// Convert an owned bridge transform handle into its typed Rust wrapper.
///
/// Callers transfer ownership of `handle` to this function. Unknown future OCIO
/// type tags are rejected after releasing the handle rather than leaking it.
pub(crate) fn transform_from_raw_handle(handle: *mut c_void) -> Option<Transform> {
    if handle.is_null() {
        return None;
    }
    let type_tag = unsafe { ocio_sys::ocio_transform_get_transform_type(handle) };
    let nn = NonNull::new(handle).unwrap();
    match type_tag {
        1 => Some(Transform::Builtin(BuiltinTransform { handle: nn })),
        2 => Some(Transform::CDL(CDLTransform { handle: nn })),
        5 => Some(Transform::Exponent(ExponentTransform { handle: nn })),
        6 => Some(Transform::ExponentWithLinear(ExponentWithLinearTransform {
            handle: nn,
        })),
        8 => Some(Transform::File(FileTransform { handle: nn })),
        9 => Some(Transform::FixedFunction(FixedFunctionTransform {
            handle: nn,
        })),
        14 => Some(Transform::Group(GroupTransform { handle: nn })),
        17 => Some(Transform::Log(LogTransform { handle: nn })),
        19 => Some(Transform::Lut1D(Lut1DTransform { handle: nn })),
        20 => Some(Transform::Lut3D(Lut3DTransform { handle: nn })),
        21 => Some(Transform::Matrix(MatrixTransform { handle: nn })),
        22 => Some(Transform::Range(RangeTransform { handle: nn })),
        7 => Some(Transform::ExposureContrast(ExposureContrastTransform {
            handle: nn,
        })),
        3 => Some(Transform::ColorSpace(ColorSpaceTransform { handle: nn })),
        18 => Some(Transform::Look(LookTransform { handle: nn })),
        4 => Some(Transform::DisplayView(DisplayViewTransform { handle: nn })),
        0 => Some(Transform::Allocation(AllocationTransform { handle: nn })),
        15 => Some(Transform::LogAffine(LogAffineTransform { handle: nn })),
        16 => Some(Transform::LogCamera(LogCameraTransform { handle: nn })),
        11 => Some(Transform::GradingPrimary(GradingPrimaryTransform {
            handle: nn,
        })),
        #[cfg(feature = "v2_5")]
        10 => Some(Transform::GradingHueCurve(GradingHueCurveTransform {
            handle: nn,
        })),
        12 => Some(Transform::GradingRGBCurve(GradingRGBCurveTransform {
            handle: nn,
        })),
        13 => Some(Transform::GradingTone(GradingToneTransform { handle: nn })),
        _ => {
            unsafe { ocio_sys::ocio_transform_destroy(handle) };
            None
        }
    }
}
