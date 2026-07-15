use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{Result, TransformDirection};
use ocio_sys;

/// Camera log OCIO transform with per-channel log-domain parameters.
pub struct LogCameraTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl LogCameraTransform {
    /// Create a log camera transform with the given linear-side break values.
    pub fn create_with_lin_side_break(lin_side_break_values: &[f64; 3]) -> Result<Self> {
        Self::create(lin_side_break_values)
    }

    /// Create a log camera transform with the given linear-side break values.
    pub fn create(lin_side_break_values: &[f64; 3]) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_log_camera_transform_create_with_lin_side_break(
                lin_side_break_values.as_ptr(),
            )
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an editable copy that is independent from the original transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the log base value.
    pub fn base(&self) -> Result<f64> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_log_camera_transform_get_base(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set the log base value.
    pub fn set_base(&self, base: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_set_base(self.handle.as_ptr(), base);
        }
        crate::ocio_call_status()
    }

    /// Return the per-channel log-side slope values.
    pub fn log_side_slope_value(&self) -> Result<[f64; 3]> {
        let mut v = [0.0f64; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_get_log_side_slope_value(
                self.handle.as_ptr(),
                v.as_mut_ptr(),
            );
        }
        crate::ocio_call_status()?;
        Ok(v)
    }

    /// Set the per-channel log-side slope values.
    pub fn set_log_side_slope_value(&self, values: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_set_log_side_slope_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Return the per-channel log-side offset values.
    pub fn log_side_offset_value(&self) -> Result<[f64; 3]> {
        let mut v = [0.0f64; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_get_log_side_offset_value(
                self.handle.as_ptr(),
                v.as_mut_ptr(),
            );
        }
        crate::ocio_call_status()?;
        Ok(v)
    }

    /// Set the per-channel log-side offset values.
    pub fn set_log_side_offset_value(&self, values: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_set_log_side_offset_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Return the per-channel linear-side slope values.
    pub fn lin_side_slope_value(&self) -> Result<[f64; 3]> {
        let mut v = [0.0f64; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_get_lin_side_slope_value(
                self.handle.as_ptr(),
                v.as_mut_ptr(),
            );
        }
        crate::ocio_call_status()?;
        Ok(v)
    }

    /// Set the per-channel linear-side slope values.
    pub fn set_lin_side_slope_value(&self, values: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_set_lin_side_slope_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Return the per-channel linear-side offset values.
    pub fn lin_side_offset_value(&self) -> Result<[f64; 3]> {
        let mut v = [0.0f64; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_get_lin_side_offset_value(
                self.handle.as_ptr(),
                v.as_mut_ptr(),
            );
        }
        crate::ocio_call_status()?;
        Ok(v)
    }

    /// Set the per-channel linear-side offset values.
    pub fn set_lin_side_offset_value(&self, values: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_set_lin_side_offset_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Return the per-channel linear-side break values.
    pub fn lin_side_break_value(&self) -> Result<[f64; 3]> {
        let mut v = [0.0f64; 3];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_get_lin_side_break_value(
                self.handle.as_ptr(),
                v.as_mut_ptr(),
            );
        }
        crate::ocio_call_status()?;
        Ok(v)
    }

    /// Set the per-channel linear-side break values.
    pub fn set_lin_side_break_value(&self, values: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_set_lin_side_break_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Return the per-channel linear slope values, if set.
    pub fn linear_slope_value(&self) -> Result<Option<[f64; 3]>> {
        let mut v = [0.0f64; 3];
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_log_camera_transform_get_linear_slope_value(
                self.handle.as_ptr(),
                v.as_mut_ptr(),
            )
        };
        crate::ocio_call_status()?;
        if ok {
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    /// Set the per-channel linear slope values.
    pub fn set_linear_slope_value(&self, values: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_set_linear_slope_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Clear the linear slope values, reverting to the default.
    pub fn unset_linear_slope_value(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_unset_linear_slope_value(self.handle.as_ptr());
        }
        crate::ocio_call_status()
    }

    /// Return the evaluation direction.
    pub fn direction(&self) -> TransformDirection {
        let dir =
            unsafe { ocio_sys::ocio_log_camera_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the evaluation direction, panicking on validation error.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set log camera transform direction");
    }

    /// Set evaluation direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_log_camera_transform_set_direction(
                self.handle.as_ptr(),
                direction as i32,
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

    /// Return whether `other` is equivalent to this transform.
    pub fn equals(&self, other: &Self) -> bool {
        unsafe {
            ocio_sys::ocio_log_camera_transform_equals(self.handle.as_ptr(), other.handle.as_ptr())
        }
    }
}

impl Drop for LogCameraTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_log_camera_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_log_camera_transform() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]);
        assert!(t.is_ok());
    }

    #[test]
    fn log_camera_transform_methods_no_crash() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        let _ = t.base();
        let _ = t.log_side_slope_value();
        let _ = t.log_side_offset_value();
        let _ = t.lin_side_slope_value();
        let _ = t.lin_side_offset_value();
        let _ = t.lin_side_break_value();
        let _ = t.linear_slope_value();
    }

    #[test]
    fn set_values_no_crash() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        let _ = t.set_base(10.0);
        let _ = t.set_log_side_slope_value(&[1.0, 1.0, 1.0]);
        let _ = t.set_log_side_offset_value(&[0.0, 0.0, 0.0]);
        let _ = t.set_lin_side_slope_value(&[1.0, 1.0, 1.0]);
        let _ = t.set_lin_side_offset_value(&[0.0, 0.0, 0.0]);
        let _ = t.set_lin_side_break_value(&[0.01, 0.01, 0.01]);
        let _ = t.set_linear_slope_value(&[1.0, 1.0, 1.0]);
        let _ = t.unset_linear_slope_value();
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        let _ = t.create_editable_copy();
    }

    #[test]
    fn equals_no_crash() {
        let a = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        let b = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        let _ = a.equals(&b);
    }

    #[test]
    fn direction_no_crash() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        let _ = t.direction();
        t.try_set_direction(TransformDirection::Inverse).unwrap();
    }

    #[test]
    fn format_metadata_no_crash() {
        let t = LogCameraTransform::create(&[0.01, 0.01, 0.01]).unwrap();
        let _ = t.format_metadata();
    }
}
