use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection, Interpolation, BitDepth};

pub struct Lut1DTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl Lut1DTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_lut1d_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn interpolation(&self) -> Interpolation {
        let i = unsafe { ocio_sys::ocio_lut1d_transform_get_interpolation(self.handle.as_ptr()) };
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
            ocio_sys::ocio_lut1d_transform_set_interpolation(self.handle.as_ptr(), interpolation as i32);
        }
    }

    pub fn file_output_bit_depth(&self) -> BitDepth {
        let b = unsafe { ocio_sys::ocio_lut1d_transform_get_file_output_bit_depth(self.handle.as_ptr()) };
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
            ocio_sys::ocio_lut1d_transform_set_file_output_bit_depth(
                self.handle.as_ptr(), bit_depth as i32,
            );
        }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_lut1d_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_lut1d_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }
}

impl Drop for Lut1DTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_lut1d_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_lut1d() {
        let lt = Lut1DTransform::create();
        assert!(lt.is_ok());
    }

    #[test]
    fn interpolation_no_crash() {
        let lt = Lut1DTransform::create().unwrap();
        let _ = lt.interpolation();
        lt.set_interpolation(Interpolation::Linear);
    }

    #[test]
    fn bit_depth_no_crash() {
        let lt = Lut1DTransform::create().unwrap();
        let _ = lt.file_output_bit_depth();
        lt.set_file_output_bit_depth(BitDepth::F32);
    }

    #[test]
    fn direction_no_crash() {
        let lt = Lut1DTransform::create().unwrap();
        let _ = lt.direction();
        lt.set_direction(TransformDirection::Inverse);
    }
}
