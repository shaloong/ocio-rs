use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{BitDepth, OcioError, Result, TransformDirection};
use ocio_sys;

/// A 4x4 matrix transform plus RGBA offset.
///
/// Matrix transforms are commonly used for channel scaling, saturation,
/// luminance views, and fitted range remapping.
pub struct MatrixTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl MatrixTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_matrix_transform_create() };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn matrix(&self) -> [f64; 16] {
        let mut m = [0.0f64; 16];
        for i in 0..4 {
            m[i * 5] = 1.0;
        }
        unsafe {
            ocio_sys::ocio_matrix_transform_get_matrix(
                self.handle.as_ptr(),
                m.as_mut_ptr() as *mut c_void,
            )
        };
        m
    }

    pub fn set_matrix(&self, m44: &[f64; 16]) {
        unsafe {
            ocio_sys::ocio_matrix_transform_set_matrix(
                self.handle.as_ptr(),
                m44.as_ptr() as *mut c_void,
            )
        };
    }

    pub fn offset(&self) -> [f64; 4] {
        let mut o = [0.0f64; 4];
        unsafe {
            ocio_sys::ocio_matrix_transform_get_offset(
                self.handle.as_ptr(),
                o.as_mut_ptr() as *mut c_void,
            )
        };
        o
    }

    pub fn set_offset(&self, offset4: &[f64; 4]) {
        unsafe {
            ocio_sys::ocio_matrix_transform_set_offset(
                self.handle.as_ptr(),
                offset4.as_ptr() as *mut c_void,
            )
        };
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe {
            ocio_sys::ocio_matrix_transform_get_direction(self.handle.as_ptr() as *mut c_void)
        };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_matrix_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn file_input_bit_depth(&self) -> BitDepth {
        let b = unsafe {
            ocio_sys::ocio_matrix_transform_get_file_input_bit_depth(
                self.handle.as_ptr() as *mut c_void
            )
        };
        match b {
            1 => BitDepth::Uint8,
            2 => BitDepth::Uint10,
            3 => BitDepth::Uint12,
            4 => BitDepth::Uint14,
            5 => BitDepth::Uint16,
            6 => BitDepth::Uint32,
            7 => BitDepth::F16,
            8 => BitDepth::F32,
            _ => BitDepth::Unknown,
        }
    }

    pub fn set_file_input_bit_depth(&self, bit_depth: BitDepth) {
        unsafe {
            ocio_sys::ocio_matrix_transform_set_file_input_bit_depth(
                self.handle.as_ptr(),
                bit_depth as i32,
            )
        };
    }

    pub fn file_output_bit_depth(&self) -> BitDepth {
        let b = unsafe {
            ocio_sys::ocio_matrix_transform_get_file_output_bit_depth(
                self.handle.as_ptr() as *mut c_void
            )
        };
        match b {
            1 => BitDepth::Uint8,
            2 => BitDepth::Uint10,
            3 => BitDepth::Uint12,
            4 => BitDepth::Uint14,
            5 => BitDepth::Uint16,
            6 => BitDepth::Uint32,
            7 => BitDepth::F16,
            8 => BitDepth::F32,
            _ => BitDepth::Unknown,
        }
    }

    pub fn set_file_output_bit_depth(&self, bit_depth: BitDepth) {
        unsafe {
            ocio_sys::ocio_matrix_transform_set_file_output_bit_depth(
                self.handle.as_ptr(),
                bit_depth as i32,
            )
        };
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle =
            unsafe { ocio_sys::ocio_matrix_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer format_metadata()")]
    pub fn format_metadata_v1(&self) -> Option<crate::FormatMetadata> {
        self.format_metadata()
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer format_metadata()")]
    pub fn format_metadata_v2(&self) -> Option<crate::FormatMetadata> {
        self.format_metadata()
    }

    pub fn equals(&self, other: &Self) -> bool {
        unsafe {
            ocio_sys::ocio_matrix_transform_equals(self.handle.as_ptr(), other.handle.as_ptr())
        }
    }
}

impl MatrixTransform {
    pub fn fit(
        old_min: &[f64; 4],
        old_max: &[f64; 4],
        new_min: &[f64; 4],
        new_max: &[f64; 4],
    ) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_matrix_transform_create_fit(
                old_min.as_ptr(),
                old_max.as_ptr(),
                new_min.as_ptr(),
                new_max.as_ptr(),
            )
        };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn identity() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_matrix_transform_create_identity() };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn sat(sat: f64, luma: &[f64; 3]) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_matrix_transform_create_sat(sat, luma.as_ptr()) };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn scale(scale: &[f64; 4]) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_matrix_transform_create_scale(scale.as_ptr()) };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn view(channels: &mut [i32; 4], luma: &[f64; 3]) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_matrix_transform_create_view(channels.as_mut_ptr(), luma.as_ptr())
        };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }
}

impl Drop for MatrixTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_matrix_transform_destroy(self.handle.as_ptr() as *mut c_void) };
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

    #[test]
    fn bit_depth_no_crash() {
        let mt = MatrixTransform::create().unwrap();
        let _ = mt.file_input_bit_depth();
        mt.set_file_input_bit_depth(BitDepth::F32);
        let _ = mt.file_output_bit_depth();
        mt.set_file_output_bit_depth(BitDepth::F32);
    }

    #[test]
    fn static_helpers_no_crash() {
        let _ = MatrixTransform::identity();
        let _ = MatrixTransform::fit(
            &[0.0, 0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0, 1.0],
            &[0.0, 0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0, 1.0],
        );
        let _ = MatrixTransform::sat(1.5, &[0.2126, 0.7152, 0.0722]);
        let _ = MatrixTransform::scale(&[1.0, 1.0, 1.0, 1.0]);
        let mut channels = [0i32; 4];
        let _ = MatrixTransform::view(&mut channels, &[0.2126, 0.7152, 0.0722]);
    }

    #[test]
    fn format_metadata_no_crash() {
        let mt = MatrixTransform::create().unwrap();
        let _ = mt.format_metadata();
    }

    #[test]
    fn equals_no_crash() {
        let a = MatrixTransform::create().unwrap();
        let b = MatrixTransform::create().unwrap();
        let _ = a.equals(&b);
    }
}
