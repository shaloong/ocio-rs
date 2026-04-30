use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{OcioError, Result, TransformDirection, RangeStyle, BitDepth};

pub struct RangeTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl RangeTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_range_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn style(&self) -> RangeStyle {
        let s = unsafe { ocio_sys::ocio_range_transform_get_style(self.handle.as_ptr()) };
        match s { 1 => RangeStyle::Clamp, _ => RangeStyle::NoClamp }
    }

    pub fn set_style(&self, style: RangeStyle) {
        unsafe { ocio_sys::ocio_range_transform_set_style(self.handle.as_ptr(), style as i32) };
    }

    pub fn min_in_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_min_in_value(self.handle.as_ptr()) }
    }

    pub fn set_min_in_value(&self, value: f64) {
        unsafe { ocio_sys::ocio_range_transform_set_min_in_value(self.handle.as_ptr(), value) };
    }

    pub fn max_in_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_max_in_value(self.handle.as_ptr()) }
    }

    pub fn set_max_in_value(&self, value: f64) {
        unsafe { ocio_sys::ocio_range_transform_set_max_in_value(self.handle.as_ptr(), value) };
    }

    pub fn min_out_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_min_out_value(self.handle.as_ptr()) }
    }

    pub fn set_min_out_value(&self, value: f64) {
        unsafe { ocio_sys::ocio_range_transform_set_min_out_value(self.handle.as_ptr(), value) };
    }

    pub fn max_out_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_max_out_value(self.handle.as_ptr()) }
    }

    pub fn set_max_out_value(&self, value: f64) {
        unsafe { ocio_sys::ocio_range_transform_set_max_out_value(self.handle.as_ptr(), value) };
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_range_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_range_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn has_min_in_value(&self) -> bool {
        unsafe { ocio_sys::ocio_range_transform_has_min_in_value(self.handle.as_ptr()) }
    }

    pub fn unset_min_in_value(&self) {
        unsafe { ocio_sys::ocio_range_transform_unset_min_in_value(self.handle.as_ptr()) };
    }

    pub fn has_max_in_value(&self) -> bool {
        unsafe { ocio_sys::ocio_range_transform_has_max_in_value(self.handle.as_ptr()) }
    }

    pub fn unset_max_in_value(&self) {
        unsafe { ocio_sys::ocio_range_transform_unset_max_in_value(self.handle.as_ptr()) };
    }

    pub fn has_min_out_value(&self) -> bool {
        unsafe { ocio_sys::ocio_range_transform_has_min_out_value(self.handle.as_ptr()) }
    }

    pub fn unset_min_out_value(&self) {
        unsafe { ocio_sys::ocio_range_transform_unset_min_out_value(self.handle.as_ptr()) };
    }

    pub fn has_max_out_value(&self) -> bool {
        unsafe { ocio_sys::ocio_range_transform_has_max_out_value(self.handle.as_ptr()) }
    }

    pub fn unset_max_out_value(&self) {
        unsafe { ocio_sys::ocio_range_transform_unset_max_out_value(self.handle.as_ptr()) };
    }

    pub fn file_input_bit_depth(&self) -> BitDepth {
        let b = unsafe { ocio_sys::ocio_range_transform_get_file_input_bit_depth(self.handle.as_ptr()) };
        match b {
            1 => BitDepth::Uint8, 2 => BitDepth::Uint10, 3 => BitDepth::Uint12,
            4 => BitDepth::Uint14, 5 => BitDepth::Uint16, 6 => BitDepth::Uint32,
            7 => BitDepth::F16, 8 => BitDepth::F32, _ => BitDepth::Unknown,
        }
    }

    pub fn set_file_input_bit_depth(&self, bit_depth: BitDepth) {
        unsafe { ocio_sys::ocio_range_transform_set_file_input_bit_depth(self.handle.as_ptr(), bit_depth as i32) };
    }

    pub fn file_output_bit_depth(&self) -> BitDepth {
        let b = unsafe { ocio_sys::ocio_range_transform_get_file_output_bit_depth(self.handle.as_ptr()) };
        match b {
            1 => BitDepth::Uint8, 2 => BitDepth::Uint10, 3 => BitDepth::Uint12,
            4 => BitDepth::Uint14, 5 => BitDepth::Uint16, 6 => BitDepth::Uint32,
            7 => BitDepth::F16, 8 => BitDepth::F32, _ => BitDepth::Unknown,
        }
    }

    pub fn set_file_output_bit_depth(&self, bit_depth: BitDepth) {
        unsafe { ocio_sys::ocio_range_transform_set_file_output_bit_depth(self.handle.as_ptr(), bit_depth as i32) };
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }
}

impl Drop for RangeTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_range_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_range() {
        let rt = RangeTransform::create();
        assert!(rt.is_ok());
    }

    #[test]
    fn range_values_no_crash() {
        let rt = RangeTransform::create().unwrap();
        let _ = rt.min_in_value();
        let _ = rt.max_in_value();
        let _ = rt.min_out_value();
        let _ = rt.max_out_value();
        rt.set_min_in_value(0.1);
        rt.set_max_in_value(0.9);
        rt.set_min_out_value(0.05);
        rt.set_max_out_value(0.95);
    }

    #[test]
    fn has_unset_no_crash() {
        let rt = RangeTransform::create().unwrap();
        let _ = rt.has_min_in_value();
        let _ = rt.has_max_in_value();
        let _ = rt.has_min_out_value();
        let _ = rt.has_max_out_value();
        rt.unset_min_in_value();
        rt.unset_max_in_value();
        rt.unset_min_out_value();
        rt.unset_max_out_value();
    }

    #[test]
    fn bit_depth_no_crash() {
        let rt = RangeTransform::create().unwrap();
        let _ = rt.file_input_bit_depth();
        rt.set_file_input_bit_depth(BitDepth::F32);
        let _ = rt.file_output_bit_depth();
        rt.set_file_output_bit_depth(BitDepth::F32);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let rt = RangeTransform::create().unwrap();
        let _ = rt.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let rt = RangeTransform::create().unwrap();
        let _ = rt.format_metadata();
    }
}
