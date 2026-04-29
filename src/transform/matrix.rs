use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection};

pub struct MatrixTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl MatrixTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_matrix_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn matrix(&self) -> [f64; 16] {
        let mut m = [0.0f64; 16];
        for i in 0..4 { m[i * 5] = 1.0; }
        unsafe { ocio_sys::ocio_matrix_transform_get_matrix(self.handle.as_ptr(), m.as_mut_ptr()) };
        m
    }

    pub fn set_matrix(&self, m44: &[f64; 16]) {
        unsafe { ocio_sys::ocio_matrix_transform_set_matrix(self.handle.as_ptr(), m44.as_ptr()) };
    }

    pub fn offset(&self) -> [f64; 4] {
        let mut o = [0.0f64; 4];
        unsafe { ocio_sys::ocio_matrix_transform_get_offset(self.handle.as_ptr(), o.as_mut_ptr()) };
        o
    }

    pub fn set_offset(&self, offset4: &[f64; 4]) {
        unsafe { ocio_sys::ocio_matrix_transform_set_offset(self.handle.as_ptr(), offset4.as_ptr()) };
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_matrix_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_matrix_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }
}

impl Drop for MatrixTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_matrix_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_matrix() {
        let mt = MatrixTransform::create();
        assert!(mt.is_ok());
    }

    #[test]
    fn matrix_and_offset() {
        let mt = MatrixTransform::create().unwrap();
        // Default is identity matrix
        let m = mt.matrix();
        assert_eq!(m[0], 1.0);
        assert_eq!(m[5], 1.0);
        assert_eq!(m[10], 1.0);
        assert_eq!(m[15], 1.0);
        // Default offset is zero
        let o = mt.offset();
        assert_eq!(o, [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let mt = MatrixTransform::create().unwrap();
        let _ = mt.create_editable_copy();
    }
}
