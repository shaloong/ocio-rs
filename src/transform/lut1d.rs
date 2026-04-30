use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection, Interpolation, BitDepth, Lut1DHueAdjust};

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

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn length(&self) -> u64 {
        unsafe { ocio_sys::ocio_lut1d_transform_get_length(self.handle.as_ptr()) }
    }

    pub fn set_length(&self, len: u64) {
        unsafe { ocio_sys::ocio_lut1d_transform_set_length(self.handle.as_ptr(), len) };
    }

    pub fn values(&self) -> Vec<f64> {
        let len = self.length() as usize;
        let mut data = vec![0.0f64; len.max(1) * 3];
        unsafe { ocio_sys::ocio_lut1d_transform_get_values(self.handle.as_ptr(), data.as_mut_ptr()) };
        data
    }

    pub fn set_values(&self, data: &[f64]) {
        unsafe { ocio_sys::ocio_lut1d_transform_set_values(self.handle.as_ptr(), data.as_ptr()) };
    }

    pub fn input_half_domain(&self) -> bool {
        unsafe { ocio_sys::ocio_lut1d_transform_get_input_half_domain(self.handle.as_ptr()) }
    }

    pub fn set_input_half_domain(&self, half_domain: bool) {
        unsafe { ocio_sys::ocio_lut1d_transform_set_input_half_domain(self.handle.as_ptr(), half_domain) };
    }

    pub fn output_raw_halfs(&self) -> bool {
        unsafe { ocio_sys::ocio_lut1d_transform_get_output_raw_halfs(self.handle.as_ptr()) }
    }

    pub fn set_output_raw_halfs(&self, raw_halfs: bool) {
        unsafe { ocio_sys::ocio_lut1d_transform_set_output_raw_halfs(self.handle.as_ptr(), raw_halfs) };
    }

    pub fn hue_adjust(&self) -> Lut1DHueAdjust {
        match unsafe { ocio_sys::ocio_lut1d_transform_get_hue_adjust(self.handle.as_ptr()) } {
            1 => Lut1DHueAdjust::Dw3,
            2 => Lut1DHueAdjust::Wypn,
            _ => Lut1DHueAdjust::None_,
        }
    }

    pub fn set_hue_adjust(&self, hue_adjust: Lut1DHueAdjust) {
        unsafe { ocio_sys::ocio_lut1d_transform_set_hue_adjust(self.handle.as_ptr(), hue_adjust as i32) };
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
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

    #[test]
    fn create_editable_copy_no_crash() {
        let lt = Lut1DTransform::create().unwrap();
        let _ = lt.create_editable_copy();
    }

    #[test]
    fn values_no_crash() {
        let t = Lut1DTransform::create().unwrap();
        let _ = t.length();
        t.set_length(32);
        let v = t.values();
        t.set_values(&v);
    }

    #[test]
    fn half_domain_no_crash() {
        let t = Lut1DTransform::create().unwrap();
        let _ = t.input_half_domain();
        t.set_input_half_domain(true);
        let _ = t.output_raw_halfs();
        t.set_output_raw_halfs(true);
    }

    #[test]
    fn hue_adjust_no_crash() {
        let t = Lut1DTransform::create().unwrap();
        let _ = t.hue_adjust();
        t.set_hue_adjust(Lut1DHueAdjust::Dw3);
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = Lut1DTransform::create().unwrap();
        let _ = t.format_metadata();
    }
}
