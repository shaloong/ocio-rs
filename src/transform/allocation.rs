use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection, Allocation};

pub struct AllocationTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl AllocationTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_allocation_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn allocation(&self) -> Allocation {
        let v = unsafe { ocio_sys::ocio_allocation_transform_get_allocation(self.handle.as_ptr()) };
        match v { 1 => Allocation::Uniform, 2 => Allocation::Lg2, _ => Allocation::Unknown }
    }

    pub fn set_allocation(&self, allocation: Allocation) {
        unsafe { ocio_sys::ocio_allocation_transform_set_allocation(self.handle.as_ptr(), allocation as i32); }
    }

    pub fn num_vars(&self) -> i32 {
        unsafe { ocio_sys::ocio_allocation_transform_get_num_vars(self.handle.as_ptr()) }
    }

    pub fn vars(&self) -> Vec<f32> {
        let n = self.num_vars() as usize;
        if n == 0 { return vec![]; }
        let mut v = vec![0.0f32; n];
        unsafe { ocio_sys::ocio_allocation_transform_get_vars(self.handle.as_ptr(), v.as_mut_ptr()); }
        v
    }

    pub fn set_vars(&self, vars: &[f32]) {
        unsafe {
            ocio_sys::ocio_allocation_transform_set_vars(
                self.handle.as_ptr(), vars.len() as i32, vars.as_ptr()
            );
        }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_allocation_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_allocation_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }
}

impl Drop for AllocationTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_allocation_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_allocation_transform() {
        let t = AllocationTransform::create();
        assert!(t.is_ok());
    }

    #[test]
    fn allocation_transform_methods_no_crash() {
        let t = AllocationTransform::create().unwrap();
        let _ = t.allocation();
        let _ = t.num_vars();
        let _ = t.vars();
        let _ = t.direction();
    }

    #[test]
    fn set_allocation_no_crash() {
        let t = AllocationTransform::create().unwrap();
        t.set_allocation(Allocation::Uniform);
        t.set_allocation(Allocation::Lg2);
    }

    #[test]
    fn set_vars_no_crash() {
        let t = AllocationTransform::create().unwrap();
        let vars: [f32; 3] = [1.0, 2.0, 3.0];
        t.set_vars(&vars);
    }

    #[test]
    fn direction_no_crash() {
        let t = AllocationTransform::create().unwrap();
        let _ = t.direction();
        t.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = AllocationTransform::create().unwrap();
        let _ = t.create_editable_copy();
    }
}
