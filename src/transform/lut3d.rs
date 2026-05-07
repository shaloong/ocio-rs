use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection, Interpolation, BitDepth};

pub struct Lut3DTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl Lut3DTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_lut3d_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn interpolation(&self) -> Interpolation {
        let i = unsafe { ocio_sys::ocio_lut3d_transform_get_interpolation(self.handle.as_ptr()) };
        match i {
            1 => Interpolation::Nearest,
            2 => Interpolation::Linear,
            3 => Interpolation::Tetrahedral,
            4 => Interpolation::Cubic,
            5 => Interpolation::Default,
            6 => Interpolation::Best,
            _ => Interpolation::Unknown,
        }
    }

    pub fn set_interpolation(&self, interpolation: Interpolation) {
        unsafe {
            ocio_sys::ocio_lut3d_transform_set_interpolation(self.handle.as_ptr(), interpolation as i32);
        }
    }

    pub fn file_output_bit_depth(&self) -> BitDepth {
        let b = unsafe { ocio_sys::ocio_lut3d_transform_get_file_output_bit_depth(self.handle.as_ptr()) };
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
            ocio_sys::ocio_lut3d_transform_set_file_output_bit_depth(
                self.handle.as_ptr(), bit_depth as i32,
            );
        }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_lut3d_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_lut3d_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn grid_size(&self) -> u64 {
        unsafe { ocio_sys::ocio_lut3d_transform_get_grid_size(self.handle.as_ptr()) as u64 }
    }

    pub fn set_grid_size(&self, size: u64) {
        unsafe { ocio_sys::ocio_lut3d_transform_set_grid_size(self.handle.as_ptr(), std::ptr::null_mut()) };
    }

    pub fn values(&self) -> Vec<f64> {
        let gs = self.grid_size() as usize;
        let len = gs.max(1);
        let mut data = vec![0.0f64; len * len * len * 3];
        unsafe { ocio_sys::ocio_lut3d_transform_get_values(self.handle.as_ptr(), data.as_mut_ptr()) };
        data
    }

    pub fn set_values(&self, data: &[f64]) {
        unsafe { ocio_sys::ocio_lut3d_transform_set_values(self.handle.as_ptr(), data.as_ptr()) };
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }
}

impl Drop for Lut3DTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_lut3d_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_lut3d() {
        let lt = Lut3DTransform::create();
        assert!(lt.is_ok());
    }

    #[test]
    fn interpolation_no_crash() {
        let lt = Lut3DTransform::create().unwrap();
        let _ = lt.interpolation();
        lt.set_interpolation(Interpolation::Tetrahedral);
    }

    #[test]
    fn bit_depth_no_crash() {
        let lt = Lut3DTransform::create().unwrap();
        let _ = lt.file_output_bit_depth();
        lt.set_file_output_bit_depth(BitDepth::F32);
    }

    #[test]
    fn direction_no_crash() {
        let lt = Lut3DTransform::create().unwrap();
        let _ = lt.direction();
        lt.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let lt = Lut3DTransform::create().unwrap();
        let _ = lt.create_editable_copy();
    }

    #[test]
    fn values_no_crash() {
        let t = Lut3DTransform::create().unwrap();
        let _ = t.grid_size();
        t.set_grid_size(10);
        let v = t.values();
        t.set_values(&v);
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = Lut3DTransform::create().unwrap();
        let _ = t.format_metadata();
    }
}
