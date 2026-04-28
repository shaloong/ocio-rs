use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection};

pub struct GroupTransform {
    handle: NonNull<c_void>,
}

impl GroupTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_group_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn num_transforms(&self) -> i32 {
        unsafe { ocio_sys::ocio_group_transform_get_num_transforms(self.handle.as_ptr()) }
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
        assert_eq!(gt.num_transforms(), 0);
    }
}
