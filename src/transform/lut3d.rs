use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{BitDepth, Interpolation, OcioError, Result, TransformDirection};
use ocio_sys;

fn required_value_count(grid_size: u64) -> Result<usize> {
    let count = grid_size
        .checked_mul(grid_size)
        .and_then(|count| count.checked_mul(grid_size))
        .and_then(|count| count.checked_mul(3))
        .ok_or_else(|| OcioError::InvalidInput("LUT3D value count overflowed".to_owned()))?;
    usize::try_from(count)
        .map_err(|_| OcioError::InvalidInput("LUT3D value count does not fit usize".to_owned()))
}

fn validate_ocio_size(grid_size: u64, api: &str) -> Result<()> {
    if grid_size > std::os::raw::c_ulong::MAX as u64 {
        return Err(OcioError::InvalidInput(format!(
            "{api}: grid size exceeds OCIO unsigned long range"
        )));
    }
    Ok(())
}

/// Three-dimensional LUT transform with configurable interpolation.
pub struct Lut3DTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl Lut3DTransform {
    /// Create a new identity 3D LUT transform.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_lut3d_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the current interpolation mode.
    pub fn interpolation(&self) -> Interpolation {
        let i = unsafe { ocio_sys::ocio_lut3d_transform_get_interpolation(self.handle.as_ptr()) };
        match i {
            1 => Interpolation::Nearest,
            2 => Interpolation::Linear,
            3 => Interpolation::Tetrahedral,
            4 => Interpolation::Cubic,
            254 => Interpolation::Default,
            255 => Interpolation::Best,
            _ => Interpolation::Unknown,
        }
    }

    /// Set the interpolation mode.
    pub fn set_interpolation(&self, interpolation: Interpolation) {
        self.try_set_interpolation(interpolation)
            .expect("failed to set LUT3D interpolation");
    }

    /// Set the interpolation mode and surface any OCIO validation error.
    pub fn try_set_interpolation(&self, interpolation: Interpolation) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut3d_transform_set_interpolation(
                self.handle.as_ptr(),
                interpolation as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Return the bit depth declared for file-based output serialization.
    pub fn file_output_bit_depth(&self) -> BitDepth {
        let b = unsafe {
            ocio_sys::ocio_lut3d_transform_get_file_output_bit_depth(self.handle.as_ptr())
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
            .expect("failed to set LUT3D file output bit depth");
    }

    /// Set the serialized output bit depth and surface any OCIO validation error.
    pub fn try_set_file_output_bit_depth(&self, bit_depth: BitDepth) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut3d_transform_set_file_output_bit_depth(
                self.handle.as_ptr(),
                bit_depth as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Return the transform direction.
    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_lut3d_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the transform direction.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set LUT3D transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut3d_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
        crate::ocio_call_status()
    }

    /// Create an independent copy of this transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the number of grid points along each axis.
    pub fn grid_size(&self) -> u64 {
        unsafe { ocio_sys::ocio_lut3d_transform_get_grid_size_u64(self.handle.as_ptr()) }
    }

    /// Return the number of grid points along each axis.
    pub fn grid_size_u64(&self) -> u64 {
        self.grid_size()
    }

    /// Resize the cubic LUT, returning an error when OCIO rejects the size.
    pub fn set_grid_size(&self, size: u64) -> Result<()> {
        validate_ocio_size(size, "Lut3DTransform::set_grid_size")?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_lut3d_transform_set_grid_size_u64(self.handle.as_ptr(), size) };
        crate::ocio_call_status()
    }

    /// Resize the cubic LUT, returning an error when OCIO rejects the size.
    pub fn set_grid_size_u64(&self, size: u64) -> Result<()> {
        self.set_grid_size(size)
    }

    /// Return all RGB LUT values as a flat `f64` vector.
    pub fn try_values(&self) -> Result<Vec<f64>> {
        let mut data = vec![0.0f64; required_value_count(self.grid_size())?];
        if data.is_empty() {
            return Ok(data);
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut3d_transform_get_values(self.handle.as_ptr(), data.as_mut_ptr())
        };
        crate::ocio_call_status()?;
        Ok(data)
    }

    /// Return all RGB LUT values as a flat `f64` vector.
    pub fn values(&self) -> Vec<f64> {
        self.try_values().unwrap_or_default()
    }

    /// Replace every RGB LUT entry in blue-major, then green, then red order.
    pub fn set_values(&self, data: &[f64]) -> Result<()> {
        let expected = required_value_count(self.grid_size())?;
        if data.len() != expected {
            return Err(OcioError::InvalidInput(format!(
                "Lut3DTransform::set_values: expected {expected} values, got {}",
                data.len()
            )));
        }
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_lut3d_transform_set_values(self.handle.as_ptr(), data.as_ptr()) };
        crate::ocio_call_status()
    }

    /// Return a single RGB LUT entry by grid coordinates, preserving OCIO failures.
    ///
    /// `Ok(None)` means at least one coordinate is outside the current grid.
    pub fn try_value(&self, index_r: u64, index_g: u64, index_b: u64) -> Result<Option<[f32; 3]>> {
        let edge = self.grid_size();
        if index_r >= edge || index_g >= edge || index_b >= edge {
            return Ok(None);
        }
        let mut value = [0.0f32; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut3d_transform_get_value(
                self.handle.as_ptr(),
                index_r as *mut c_void,
                index_g as *mut c_void,
                index_b as *mut c_void,
                (&mut value[0] as *mut f32).cast(),
                (&mut value[1] as *mut f32).cast(),
                (&mut value[2] as *mut f32).cast(),
            );
        }
        crate::ocio_call_status()?;
        Ok(Some(value))
    }

    /// Return a single RGB LUT entry by grid coordinates, or `None` if out of range.
    pub fn value(&self, index_r: u64, index_g: u64, index_b: u64) -> Option<[f32; 3]> {
        self.try_value(index_r, index_g, index_b).ok().flatten()
    }

    /// Set one RGB LUT entry.
    pub fn set_value(
        &self,
        index_r: u64,
        index_g: u64,
        index_b: u64,
        value: [f32; 3],
    ) -> Result<()> {
        let edge = self.grid_size();
        if index_r >= edge || index_g >= edge || index_b >= edge {
            return Err(OcioError::InvalidInput(format!(
                "Lut3DTransform::set_value: index ({index_r}, {index_g}, {index_b}) is outside {edge}x{edge}x{edge} LUT"
            )));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_lut3d_transform_set_value(
                self.handle.as_ptr(),
                index_r as *mut c_void,
                index_g as *mut c_void,
                index_b as *mut c_void,
                value[0],
                value[1],
                value[2],
            );
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
            ocio_sys::ocio_lut3d_transform_equals(self.handle.as_ptr(), other.handle.as_ptr())
        }
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
        lt.try_set_interpolation(Interpolation::Tetrahedral)
            .unwrap();
    }

    #[test]
    fn bit_depth_no_crash() {
        let lt = Lut3DTransform::create().unwrap();
        let _ = lt.file_output_bit_depth();
        lt.try_set_file_output_bit_depth(BitDepth::F32).unwrap();
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
        t.set_grid_size(10).unwrap();
        let v = t.values();
        t.set_values(&v).unwrap();
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = Lut3DTransform::create().unwrap();
        let _ = t.format_metadata();
    }
}
