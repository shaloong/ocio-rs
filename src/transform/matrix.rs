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
    /// Create a new identity matrix transform.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_matrix_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the current 4x4 matrix in row-major order.
    pub fn try_matrix(&self) -> Result<[f64; 16]> {
        let mut m = [0.0f64; 16];
        for i in 0..4 {
            m[i * 5] = 1.0;
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_matrix_transform_get_matrix(
                self.handle.as_ptr(),
                m.as_mut_ptr() as *mut c_void,
            )
        };
        crate::ocio_call_status()?;
        Ok(m)
    }

    /// Return the current 4x4 matrix in row-major order.
    pub fn matrix(&self) -> [f64; 16] {
        self.try_matrix().unwrap_or_else(|_| {
            let mut m = [0.0f64; 16];
            for i in 0..4 {
                m[i * 5] = 1.0;
            }
            m
        })
    }

    /// Replace the current 4x4 matrix in row-major order.
    pub fn set_matrix(&self, m44: &[f64; 16]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_matrix_transform_set_matrix(
                self.handle.as_ptr(),
                m44.as_ptr() as *mut c_void,
            )
        };
        crate::ocio_call_status()
    }

    /// Return the current RGBA offset vector.
    pub fn try_offset(&self) -> Result<[f64; 4]> {
        let mut o = [0.0f64; 4];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_matrix_transform_get_offset(
                self.handle.as_ptr(),
                o.as_mut_ptr() as *mut c_void,
            )
        };
        crate::ocio_call_status()?;
        Ok(o)
    }

    /// Return the current RGBA offset vector.
    pub fn offset(&self) -> [f64; 4] {
        self.try_offset().unwrap_or([0.0; 4])
    }

    /// Replace the current RGBA offset vector.
    pub fn set_offset(&self, offset4: &[f64; 4]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_matrix_transform_set_offset(
                self.handle.as_ptr(),
                offset4.as_ptr() as *mut c_void,
            )
        };
        crate::ocio_call_status()
    }

    /// Return the transform direction used when this op is evaluated.
    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe {
            ocio_sys::ocio_matrix_transform_get_direction(self.handle.as_ptr() as *mut c_void)
        };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the transform direction used when this op is evaluated.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set matrix transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_matrix_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
        crate::ocio_call_status()
    }

    /// Create an editable copy that is independent from the original transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the bit depth declared for file-based input serialization.
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

    /// Set the bit depth declared for file-based input serialization.
    pub fn set_file_input_bit_depth(&self, bit_depth: BitDepth) {
        self.try_set_file_input_bit_depth(bit_depth)
            .expect("failed to set matrix file input bit depth");
    }

    /// Set the serialized input bit depth and surface any OCIO validation error.
    pub fn try_set_file_input_bit_depth(&self, bit_depth: BitDepth) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_matrix_transform_set_file_input_bit_depth(
                self.handle.as_ptr(),
                bit_depth as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// Return the bit depth declared for file-based output serialization.
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

    /// Set the bit depth declared for file-based output serialization.
    pub fn set_file_output_bit_depth(&self, bit_depth: BitDepth) {
        self.try_set_file_output_bit_depth(bit_depth)
            .expect("failed to set matrix file output bit depth");
    }

    /// Set the serialized output bit depth and surface any OCIO validation error.
    pub fn try_set_file_output_bit_depth(&self, bit_depth: BitDepth) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_matrix_transform_set_file_output_bit_depth(
                self.handle.as_ptr(),
                bit_depth as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// Return format metadata attached to the transform, when available.
    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
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

    /// Return whether this transform is equivalent to `other`.
    pub fn equals(&self, other: &Self) -> bool {
        unsafe {
            ocio_sys::ocio_matrix_transform_equals(self.handle.as_ptr(), other.handle.as_ptr())
        }
    }
}

impl MatrixTransform {
    /// Create a matrix transform that remaps the inclusive range
    /// `old_min..old_max` into `new_min..new_max`.
    pub fn fit(
        old_min: &[f64; 4],
        old_max: &[f64; 4],
        new_min: &[f64; 4],
        new_max: &[f64; 4],
    ) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_matrix_transform_create_fit(
                old_min.as_ptr(),
                old_max.as_ptr(),
                new_min.as_ptr(),
                new_max.as_ptr(),
            )
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an identity matrix transform.
    pub fn identity() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_matrix_transform_create_identity() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create a saturation matrix using the provided luma coefficients.
    pub fn sat(sat: f64, luma: &[f64; 3]) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_matrix_transform_create_sat(sat, luma.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create a per-channel scale matrix.
    pub fn scale(scale: &[f64; 4]) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_matrix_transform_create_scale(scale.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create a matrix that remaps channels according to a canonical OCIO view mask.
    pub fn view(channels: &mut [i32; 4], luma: &[f64; 3]) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_matrix_transform_create_view(channels.as_mut_ptr(), luma.as_ptr())
        };
        crate::handle_result(handle).map(|handle| Self { handle })
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
        mt.try_set_file_input_bit_depth(BitDepth::F32).unwrap();
        let _ = mt.file_output_bit_depth();
        mt.try_set_file_output_bit_depth(BitDepth::F32).unwrap();
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
