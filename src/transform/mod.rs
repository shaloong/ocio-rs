mod file;
mod cdl;
mod exponent;
mod matrix;
mod log;
mod range;
mod group;
mod builtin;
mod fixed_function;
mod lut1d;
mod lut3d;
mod exposure_contrast;
mod color_space;

use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;

pub use file::FileTransform;
pub use cdl::CDLTransform;
pub use exponent::ExponentTransform;
pub use matrix::MatrixTransform;
pub use log::LogTransform;
pub use range::RangeTransform;
pub use group::GroupTransform;
pub use builtin::BuiltinTransform;
pub use fixed_function::FixedFunctionTransform;
pub use lut1d::Lut1DTransform;
pub use lut3d::Lut3DTransform;
pub use exposure_contrast::ExposureContrastTransform;
pub use color_space::ColorSpaceTransform;

pub trait TransformHandle {
    fn as_ptr(&self) -> *mut c_void;
}

impl TransformHandle for FileTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for CDLTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for ExponentTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for MatrixTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for LogTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for RangeTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for GroupTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for BuiltinTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for FixedFunctionTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for Lut1DTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for Lut3DTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for ExposureContrastTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}
impl TransformHandle for ColorSpaceTransform {
    fn as_ptr(&self) -> *mut c_void { self.handle.as_ptr() }
}

pub enum Transform {
    File(FileTransform),
    CDL(CDLTransform),
    Exponent(ExponentTransform),
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
}

impl TransformHandle for Transform {
    fn as_ptr(&self) -> *mut c_void {
        match self {
            Transform::File(t) => t.as_ptr(),
            Transform::CDL(t) => t.as_ptr(),
            Transform::Exponent(t) => t.as_ptr(),
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
        }
    }
}

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
        8 => Some(Transform::File(FileTransform { handle: nn })),
        9 => Some(Transform::FixedFunction(FixedFunctionTransform { handle: nn })),
        14 => Some(Transform::Group(GroupTransform { handle: nn })),
        17 => Some(Transform::Log(LogTransform { handle: nn })),
        19 => Some(Transform::Lut1D(Lut1DTransform { handle: nn })),
        20 => Some(Transform::Lut3D(Lut3DTransform { handle: nn })),
        21 => Some(Transform::Matrix(MatrixTransform { handle: nn })),
        22 => Some(Transform::Range(RangeTransform { handle: nn })),
        7 => Some(Transform::ExposureContrast(ExposureContrastTransform { handle: nn })),
        4 => Some(Transform::ColorSpace(ColorSpaceTransform { handle: nn })),
        _ => None,
    }
}
