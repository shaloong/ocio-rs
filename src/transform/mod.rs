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

use std::ffi::c_void;

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
        }
    }
}
