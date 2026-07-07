use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{NegativeStyle, OcioError, Result, TransformDirection};
use ocio_sys;

/// Raises RGBA channels to per-channel exponent values.
pub struct ExponentTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl ExponentTransform {
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_exponent_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn value(&self) -> Result<[f64; 4]> {
        let mut v = [1.0f64; 4];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exponent_transform_get_value(self.handle.as_ptr(), v.as_mut_ptr())
        };
        crate::ocio_call_status()?;
        Ok(v)
    }

    pub fn set_value(&self, vec4: &[f64; 4]) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_exponent_transform_set_value(self.handle.as_ptr(), vec4.as_ptr()) };
        crate::ocio_call_status()
    }

    pub fn negative_style(&self) -> NegativeStyle {
        let s =
            unsafe { ocio_sys::ocio_exponent_transform_get_negative_style(self.handle.as_ptr()) };
        match s {
            1 => NegativeStyle::Mirror,
            2 => NegativeStyle::PassThru,
            3 => NegativeStyle::Linear,
            _ => NegativeStyle::Clamp,
        }
    }

    pub fn set_negative_style(&self, style: NegativeStyle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_exponent_transform_set_negative_style(
                self.handle.as_ptr(),
                style as i32,
            );
        }
        crate::ocio_call_status()
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_exponent_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_exponent_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
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
            ocio_sys::ocio_exponent_transform_equals(self.handle.as_ptr(), other.handle.as_ptr())
        }
    }
}

impl Drop for ExponentTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_exponent_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_exponent() {
        let et = ExponentTransform::create();
        assert!(et.is_ok());
    }

    #[test]
    fn value_no_crash() {
        let et = ExponentTransform::create().unwrap();
        let _ = et.value();
        let _ = et.set_value(&[2.2, 2.2, 2.2, 1.0]);
    }

    #[test]
    fn negative_style_no_crash() {
        let et = ExponentTransform::create().unwrap();
        let _ = et.negative_style();
        let _ = et.set_negative_style(NegativeStyle::Mirror);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let et = ExponentTransform::create().unwrap();
        let _ = et.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let et = ExponentTransform::create().unwrap();
        let _ = et.format_metadata();
    }

    #[test]
    fn equals_no_crash() {
        let a = ExponentTransform::create().unwrap();
        let b = ExponentTransform::create().unwrap();
        let _ = a.equals(&b);
    }
}
