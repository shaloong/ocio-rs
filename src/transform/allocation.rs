use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{Allocation, OcioError, Result, TransformDirection};
use ocio_sys;

/// Maps values into or out of an OCIO allocation domain.
pub struct AllocationTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl AllocationTransform {
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_allocation_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn allocation(&self) -> Allocation {
        let v = unsafe { ocio_sys::ocio_allocation_transform_get_allocation(self.handle.as_ptr()) };
        match v {
            1 => Allocation::Uniform,
            2 => Allocation::Lg2,
            _ => Allocation::Unknown,
        }
    }

    pub fn set_allocation(&self, allocation: Allocation) {
        self.try_set_allocation(allocation)
            .expect("failed to set allocation mode");
    }

    /// Set the allocation mode and surface any OCIO validation error.
    pub fn try_set_allocation(&self, allocation: Allocation) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_allocation_transform_set_allocation(
                self.handle.as_ptr(),
                allocation as i32,
            );
        }
        crate::ocio_call_status()
    }

    pub fn num_vars(&self) -> i32 {
        unsafe { ocio_sys::ocio_allocation_transform_get_num_vars(self.handle.as_ptr()) }
    }

    pub fn vars(&self) -> Vec<f32> {
        let n = self.num_vars() as usize;
        if n == 0 {
            return vec![];
        }
        let mut v = vec![0.0f32; n];
        unsafe {
            ocio_sys::ocio_allocation_transform_get_vars(
                self.handle.as_ptr(),
                v.as_mut_ptr() as *mut c_void,
            );
        }
        v
    }

    /// Set allocation-domain parameters.
    pub fn set_vars(&self, vars: &[f32]) -> Result<()> {
        let num_vars = i32::try_from(vars.len()).map_err(|_| {
            OcioError::InvalidInput(
                "AllocationTransform::set_vars: too many allocation values".to_owned(),
            )
        })?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_allocation_transform_set_vars(
                self.handle.as_ptr(),
                num_vars,
                vars.as_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    pub fn direction(&self) -> TransformDirection {
        let dir =
            unsafe { ocio_sys::ocio_allocation_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_allocation_transform_create_editable_copy(self.handle.as_ptr())
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set allocation transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_allocation_transform_set_direction(
                self.handle.as_ptr(),
                direction as i32,
            );
        }
        crate::ocio_call_status()
    }

    pub fn validate(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_allocation_transform_validate(self.handle.as_ptr()) };
        crate::validation_status()
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
        t.try_set_allocation(Allocation::Uniform).unwrap();
        t.try_set_allocation(Allocation::Lg2).unwrap();
    }

    #[test]
    fn set_vars_no_crash() {
        let t = AllocationTransform::create().unwrap();
        let vars: [f32; 3] = [1.0, 2.0, 3.0];
        t.set_vars(&vars).unwrap();
    }

    #[test]
    fn direction_no_crash() {
        let t = AllocationTransform::create().unwrap();
        let _ = t.direction();
        t.try_set_direction(TransformDirection::Inverse).unwrap();
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = AllocationTransform::create().unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = AllocationTransform::create().unwrap();
        let _ = t.format_metadata();
    }

    #[test]
    fn validate_no_crash() {
        let t = AllocationTransform::create().unwrap();
        let _ = t.validate();
    }
}
