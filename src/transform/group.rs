use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection};
use super::{TransformHandle, Transform};
use super::{FileTransform, CDLTransform, ExponentTransform, MatrixTransform, LogTransform, RangeTransform, BuiltinTransform, FixedFunctionTransform, Lut1DTransform, Lut3DTransform};

pub struct GroupTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl GroupTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_group_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn num_transforms(&self) -> i32 {
        unsafe { ocio_sys::ocio_group_transform_get_num_transforms(self.handle.as_ptr()) }
    }

    pub fn append_transform(&self, child: &impl TransformHandle) {
        unsafe {
            ocio_sys::ocio_group_transform_append_transform(self.handle.as_ptr(), child.as_ptr());
        }
    }

    pub fn prepend_transform(&self, child: &impl TransformHandle) {
        unsafe {
            ocio_sys::ocio_group_transform_prepend_transform(self.handle.as_ptr(), child.as_ptr());
        }
    }

    pub fn get_transform(&self, index: i32) -> Option<Transform> {
        let handle = unsafe {
            ocio_sys::ocio_group_transform_get_transform(self.handle.as_ptr(), index)
        };
        if handle.is_null() {
            return None;
        }
        let type_tag = unsafe { ocio_sys::ocio_transform_get_transform_type(handle) };
        match type_tag {
            1 => Some(Transform::Builtin(BuiltinTransform { handle: NonNull::new(handle).unwrap() })),
            2 => Some(Transform::CDL(CDLTransform { handle: NonNull::new(handle).unwrap() })),
            5 => Some(Transform::Exponent(ExponentTransform { handle: NonNull::new(handle).unwrap() })),
            8 => Some(Transform::File(FileTransform { handle: NonNull::new(handle).unwrap() })),
            9 => Some(Transform::FixedFunction(FixedFunctionTransform { handle: NonNull::new(handle).unwrap() })),
            14 => Some(Transform::Group(GroupTransform { handle: NonNull::new(handle).unwrap() })),
            19 => Some(Transform::Lut1D(Lut1DTransform { handle: NonNull::new(handle).unwrap() })),
            20 => Some(Transform::Lut3D(Lut3DTransform { handle: NonNull::new(handle).unwrap() })),
            17 => Some(Transform::Log(LogTransform { handle: NonNull::new(handle).unwrap() })),
            21 => Some(Transform::Matrix(MatrixTransform { handle: NonNull::new(handle).unwrap() })),
            22 => Some(Transform::Range(RangeTransform { handle: NonNull::new(handle).unwrap() })),
            _ => None,
        }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_group_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_group_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }
}

impl Drop for GroupTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_group_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_group() {
        let gt = GroupTransform::create();
        assert!(gt.is_ok());
    }

    #[test]
    fn num_transforms() {
        let gt = GroupTransform::create().unwrap();
        let _ = gt.num_transforms();
    }

    #[test]
    fn append_transform_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        gt.append_transform(&ft);
    }

    #[test]
    fn prepend_transform_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        gt.prepend_transform(&ft);
    }

    #[test]
    fn get_transform_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        gt.append_transform(&ft);
        let _ = gt.get_transform(0);
    }

    #[test]
    fn get_transform_out_of_range() {
        let gt = GroupTransform::create().unwrap();
        assert!(gt.get_transform(0).is_none());
    }

    #[test]
    fn append_multiple_transforms_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        let ct = CDLTransform::create().unwrap();
        gt.append_transform(&ft);
        gt.append_transform(&ct);
    }

    #[test]
    fn append_via_enum_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let t = Transform::File(FileTransform::create().unwrap());
        gt.append_transform(&t);
    }
}
