use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{BitDepth, Interpolation, Lut1DHueAdjust, OcioError, Result, TransformDirection};
use ocio_sys;

fn required_value_count(length: u64) -> Result<usize> {
    let count = length
        .checked_mul(3)
        .ok_or_else(|| OcioError::InvalidInput("LUT1D value count overflowed".to_owned()))?;
    usize::try_from(count)
        .map_err(|_| OcioError::InvalidInput("LUT1D value count does not fit usize".to_owned()))
}

fn validate_ocio_size(length: u64, api: &str) -> Result<()> {
    if length > std::os::raw::c_ulong::MAX as u64 {
        return Err(OcioError::InvalidInput(format!(
            "{api}: length exceeds OCIO unsigned long range"
        )));
    }
    Ok(())
}

/// One-dimensional LUT transform with optional half-domain and hue adjustment.
pub struct Lut1DTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl Lut1DTransform {
    /// Create a new identity 1D LUT transform.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_lut1d_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the current interpolation mode.
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

    /// Set the interpolation mode.
    pub fn set_interpolation(&self, interpolation: Interpolation) {
        self.try_set_interpolation(interpolation)
            .expect("failed to set LUT1D interpolation");
    }

    /// Set the interpolation mode and surface any OCIO validation error.
    pub fn try_set_interpolation(&self, interpolation: Interpolation) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut1d_transform_set_interpolation(
                self.handle.as_ptr(),
                interpolation as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Return the bit depth declared for file-based output serialization.
    pub fn file_output_bit_depth(&self) -> BitDepth {
        let b = unsafe {
            ocio_sys::ocio_lut1d_transform_get_file_output_bit_depth(self.handle.as_ptr())
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

    /// Set the serialized output bit depth.
    pub fn set_file_output_bit_depth(&self, bit_depth: BitDepth) {
        self.try_set_file_output_bit_depth(bit_depth)
            .expect("failed to set LUT1D file output bit depth");
    }

    /// Set the serialized output bit depth and surface any OCIO validation error.
    pub fn try_set_file_output_bit_depth(&self, bit_depth: BitDepth) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut1d_transform_set_file_output_bit_depth(
                self.handle.as_ptr(),
                bit_depth as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Return the transform direction.
    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_lut1d_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the transform direction.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set LUT1D transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut1d_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
        crate::ocio_call_status()
    }

    /// Create an independent copy of this transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the LUT length (number of entries).
    pub fn length(&self) -> u64 {
        unsafe { ocio_sys::ocio_lut1d_transform_get_length_u64(self.handle.as_ptr()) }
    }

    /// Return the LUT length (number of entries).
    pub fn length_u64(&self) -> u64 {
        self.length()
    }

    /// Resize the LUT, returning an error when OCIO rejects the requested size.
    pub fn set_length(&self, len: u64) -> Result<()> {
        validate_ocio_size(len, "Lut1DTransform::set_length")?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_lut1d_transform_set_length_u64(self.handle.as_ptr(), len) };
        crate::ocio_call_status()
    }

    /// Resize the LUT, returning an error when OCIO rejects the requested size.
    pub fn set_length_u64(&self, len: u64) -> Result<()> {
        self.set_length(len)
    }

    /// Return all RGB LUT values as a flat `f64` vector.
    pub fn try_values(&self) -> Result<Vec<f64>> {
        let mut data = vec![0.0f64; required_value_count(self.length())?];
        if data.is_empty() {
            return Ok(data);
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut1d_transform_get_values(self.handle.as_ptr(), data.as_mut_ptr())
        };
        crate::ocio_call_status()?;
        Ok(data)
    }

    /// Return all RGB LUT values as a flat `f64` vector.
    pub fn values(&self) -> Vec<f64> {
        self.try_values().unwrap_or_default()
    }

    /// Replace every RGB LUT entry.
    ///
    /// `data` must contain exactly `length() * 3` values in index order.
    pub fn set_values(&self, data: &[f64]) -> Result<()> {
        let expected = required_value_count(self.length())?;
        if data.len() != expected {
            return Err(OcioError::InvalidInput(format!(
                "Lut1DTransform::set_values: expected {expected} values, got {}",
                data.len()
            )));
        }
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_lut1d_transform_set_values(self.handle.as_ptr(), data.as_ptr()) };
        crate::ocio_call_status()
    }

    /// Return a single RGB LUT entry by index, preserving OCIO failures.
    ///
    /// `Ok(None)` means `index` is outside the current LUT length.
    pub fn try_value(&self, index: u64) -> Result<Option<[f32; 3]>> {
        if index >= self.length() {
            return Ok(None);
        }
        let mut value = [0.0f32; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut1d_transform_get_value(
                self.handle.as_ptr(),
                index as *mut c_void,
                (&mut value[0] as *mut f32).cast(),
                (&mut value[1] as *mut f32).cast(),
                (&mut value[2] as *mut f32).cast(),
            );
        }
        crate::ocio_call_status()?;
        Ok(Some(value))
    }

    /// Return a single RGB LUT entry by index, or `None` if out of range.
    pub fn value(&self, index: u64) -> Option<[f32; 3]> {
        self.try_value(index).ok().flatten()
    }

    /// Set one RGB LUT entry.
    pub fn set_value(&self, index: u64, value: [f32; 3]) -> Result<()> {
        if index >= self.length() {
            return Err(OcioError::InvalidInput(format!(
                "Lut1DTransform::set_value: index {index} is outside LUT length {}",
                self.length()
            )));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut1d_transform_set_value(
                self.handle.as_ptr(),
                index as *mut c_void,
                value[0],
                value[1],
                value[2],
            );
        }
        crate::ocio_call_status()
    }

    /// Return whether the input half-domain flag is enabled.
    pub fn input_half_domain(&self) -> bool {
        unsafe { ocio_sys::ocio_lut1d_transform_get_input_half_domain(self.handle.as_ptr()) }
    }

    /// Set the input half-domain flag.
    pub fn set_input_half_domain(&self, half_domain: bool) {
        self.try_set_input_half_domain(half_domain)
            .expect("failed to set LUT1D input half domain");
    }

    /// Set the input half-domain flag and surface any OCIO validation error.
    pub fn try_set_input_half_domain(&self, half_domain: bool) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut1d_transform_set_input_half_domain(self.handle.as_ptr(), half_domain);
        }
        crate::ocio_call_status()
    }

    /// Return whether the output raw halfs flag is enabled.
    pub fn output_raw_halfs(&self) -> bool {
        unsafe { ocio_sys::ocio_lut1d_transform_get_output_raw_halfs(self.handle.as_ptr()) }
    }

    /// Set the output raw halfs flag.
    pub fn set_output_raw_halfs(&self, raw_halfs: bool) {
        self.try_set_output_raw_halfs(raw_halfs)
            .expect("failed to set LUT1D output raw halfs");
    }

    /// Set the output-raw-halfs flag and surface any OCIO validation error.
    pub fn try_set_output_raw_halfs(&self, raw_halfs: bool) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut1d_transform_set_output_raw_halfs(self.handle.as_ptr(), raw_halfs);
        }
        crate::ocio_call_status()
    }

    /// Return the hue-adjust algorithm.
    pub fn hue_adjust(&self) -> Lut1DHueAdjust {
        match unsafe { ocio_sys::ocio_lut1d_transform_get_hue_adjust(self.handle.as_ptr()) } {
            1 => Lut1DHueAdjust::Dw3,
            2 => Lut1DHueAdjust::Wypn,
            _ => Lut1DHueAdjust::None_,
        }
    }

    /// Set the hue-adjust algorithm.
    pub fn set_hue_adjust(&self, hue_adjust: Lut1DHueAdjust) {
        self.try_set_hue_adjust(hue_adjust)
            .expect("failed to set LUT1D hue adjust");
    }

    /// Set the hue-adjust algorithm and surface any OCIO validation error.
    pub fn try_set_hue_adjust(&self, hue_adjust: Lut1DHueAdjust) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut1d_transform_set_hue_adjust(self.handle.as_ptr(), hue_adjust as i32);
        }
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
            ocio_sys::ocio_lut1d_transform_equals(self.handle.as_ptr(), other.handle.as_ptr())
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
        lt.try_set_interpolation(Interpolation::Linear).unwrap();
    }

    #[test]
    fn bit_depth_no_crash() {
        let lt = Lut1DTransform::create().unwrap();
        let _ = lt.file_output_bit_depth();
        lt.try_set_file_output_bit_depth(BitDepth::F32).unwrap();
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
        t.set_length(32).unwrap();
        let v = t.values();
        t.set_values(&v).unwrap();
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
