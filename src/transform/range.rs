use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{BitDepth, OcioError, RangeStyle, Result, TransformDirection};
use ocio_sys;

/// Clamps and rescales numeric ranges between input and output domains.
pub struct RangeTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl RangeTransform {
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_range_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn style(&self) -> RangeStyle {
        let s = unsafe { ocio_sys::ocio_range_transform_get_style(self.handle.as_ptr()) };
        match s {
            1 => RangeStyle::Clamp,
            _ => RangeStyle::NoClamp,
        }
    }

    pub fn set_style(&self, style: RangeStyle) {
        self.try_set_style(style)
            .expect("failed to set range style");
    }

    /// Set the range style and surface any OCIO validation error.
    pub fn try_set_style(&self, style: RangeStyle) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_range_transform_set_style(self.handle.as_ptr(), style as i32) };
        crate::ocio_call_status()
    }

    pub fn min_in_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_min_in_value(self.handle.as_ptr()) }
    }

    pub fn set_min_in_value(&self, value: f64) {
        self.try_set_min_in_value(value)
            .expect("failed to set range minimum input value");
    }

    /// Set the minimum input endpoint and surface any OCIO validation error.
    pub fn try_set_min_in_value(&self, value: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_range_transform_set_min_in_value(self.handle.as_ptr(), value) };
        crate::ocio_call_status()
    }

    pub fn max_in_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_max_in_value(self.handle.as_ptr()) }
    }

    pub fn set_max_in_value(&self, value: f64) {
        self.try_set_max_in_value(value)
            .expect("failed to set range maximum input value");
    }

    /// Set the maximum input endpoint and surface any OCIO validation error.
    pub fn try_set_max_in_value(&self, value: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_range_transform_set_max_in_value(self.handle.as_ptr(), value) };
        crate::ocio_call_status()
    }

    pub fn min_out_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_min_out_value(self.handle.as_ptr()) }
    }

    pub fn set_min_out_value(&self, value: f64) {
        self.try_set_min_out_value(value)
            .expect("failed to set range minimum output value");
    }

    /// Set the minimum output endpoint and surface any OCIO validation error.
    pub fn try_set_min_out_value(&self, value: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_range_transform_set_min_out_value(self.handle.as_ptr(), value) };
        crate::ocio_call_status()
    }

    pub fn max_out_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_range_transform_get_max_out_value(self.handle.as_ptr()) }
    }

    pub fn set_max_out_value(&self, value: f64) {
        self.try_set_max_out_value(value)
            .expect("failed to set range maximum output value");
    }

    /// Set the maximum output endpoint and surface any OCIO validation error.
    pub fn try_set_max_out_value(&self, value: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_range_transform_set_max_out_value(self.handle.as_ptr(), value) };
        crate::ocio_call_status()
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_range_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set range transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_range_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
        crate::ocio_call_status()
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn has_min_in_value(&self) -> bool {
        unsafe { ocio_sys::ocio_range_transform_has_min_in_value(self.handle.as_ptr()) }
    }

    pub fn unset_min_in_value(&self) {
        self.try_unset_min_in_value()
            .expect("failed to unset range minimum input value");
    }

    /// Unset the minimum input endpoint and surface any OCIO validation error.
    pub fn try_unset_min_in_value(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_range_transform_unset_min_in_value(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    pub fn has_max_in_value(&self) -> bool {
        unsafe { ocio_sys::ocio_range_transform_has_max_in_value(self.handle.as_ptr()) }
    }

    pub fn unset_max_in_value(&self) {
        self.try_unset_max_in_value()
            .expect("failed to unset range maximum input value");
    }

    /// Unset the maximum input endpoint and surface any OCIO validation error.
    pub fn try_unset_max_in_value(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_range_transform_unset_max_in_value(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    pub fn has_min_out_value(&self) -> bool {
        unsafe { ocio_sys::ocio_range_transform_has_min_out_value(self.handle.as_ptr()) }
    }

    pub fn unset_min_out_value(&self) {
        self.try_unset_min_out_value()
            .expect("failed to unset range minimum output value");
    }

    /// Unset the minimum output endpoint and surface any OCIO validation error.
    pub fn try_unset_min_out_value(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_range_transform_unset_min_out_value(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    pub fn has_max_out_value(&self) -> bool {
        unsafe { ocio_sys::ocio_range_transform_has_max_out_value(self.handle.as_ptr()) }
    }

    pub fn unset_max_out_value(&self) {
        self.try_unset_max_out_value()
            .expect("failed to unset range maximum output value");
    }

    /// Unset the maximum output endpoint and surface any OCIO validation error.
    pub fn try_unset_max_out_value(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_range_transform_unset_max_out_value(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    pub fn file_input_bit_depth(&self) -> BitDepth {
        let b = unsafe {
            ocio_sys::ocio_range_transform_get_file_input_bit_depth(self.handle.as_ptr())
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
        self.try_set_file_input_bit_depth(bit_depth)
            .expect("failed to set range file input bit depth");
    }

    /// Set the serialized input bit depth and surface any OCIO validation error.
    pub fn try_set_file_input_bit_depth(&self, bit_depth: BitDepth) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_range_transform_set_file_input_bit_depth(
                self.handle.as_ptr(),
                bit_depth as i32,
            )
        };
        crate::ocio_call_status()
    }

    pub fn file_output_bit_depth(&self) -> BitDepth {
        let b = unsafe {
            ocio_sys::ocio_range_transform_get_file_output_bit_depth(self.handle.as_ptr())
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
        self.try_set_file_output_bit_depth(bit_depth)
            .expect("failed to set range file output bit depth");
    }

    /// Set the serialized output bit depth and surface any OCIO validation error.
    pub fn try_set_file_output_bit_depth(&self, bit_depth: BitDepth) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_range_transform_set_file_output_bit_depth(
                self.handle.as_ptr(),
                bit_depth as i32,
            )
        };
        crate::ocio_call_status()
    }

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

    pub fn equals(&self, other: &Self) -> bool {
        unsafe {
            ocio_sys::ocio_range_transform_equals(self.handle.as_ptr(), other.handle.as_ptr())
        }
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
        rt.try_set_min_in_value(0.1).unwrap();
        rt.try_set_max_in_value(0.9).unwrap();
        rt.try_set_min_out_value(0.05).unwrap();
        rt.try_set_max_out_value(0.95).unwrap();
    }

    #[test]
    fn has_unset_no_crash() {
        let rt = RangeTransform::create().unwrap();
        let _ = rt.has_min_in_value();
        let _ = rt.has_max_in_value();
        let _ = rt.has_min_out_value();
        let _ = rt.has_max_out_value();
        rt.try_unset_min_in_value().unwrap();
        rt.try_unset_max_in_value().unwrap();
        rt.try_unset_min_out_value().unwrap();
        rt.try_unset_max_out_value().unwrap();
    }

    #[test]
    fn bit_depth_no_crash() {
        let rt = RangeTransform::create().unwrap();
        let _ = rt.file_input_bit_depth();
        rt.try_set_file_input_bit_depth(BitDepth::F32).unwrap();
        let _ = rt.file_output_bit_depth();
        rt.try_set_file_output_bit_depth(BitDepth::F32).unwrap();
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

    #[test]
    fn equals_no_crash() {
        let a = RangeTransform::create().unwrap();
        let b = RangeTransform::create().unwrap();
        let _ = a.equals(&b);
    }
}
