use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection};
use super::{TransformHandle, Transform, transform_from_raw_handle};

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
        transform_from_raw_handle(handle)
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

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
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
    use crate::transform::{FileTransform, CDLTransform};

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

    #[test]
    fn create_editable_copy_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let _ = gt.create_editable_copy();
    }
}
