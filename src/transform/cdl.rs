use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, transform::GroupTransform, CDLStyle, OcioError,
    Result, TransformDirection,
};
use ocio_sys;

/// ASC CDL slope/offset/power and saturation transform.
pub struct CDLTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl CDLTransform {
    /// Create a new empty CDL transform.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_cdl_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create a CDL transform from a CCC file and optional id.
    pub fn create_from_file(src: impl AsRef<str>, ccc_id: impl AsRef<str>) -> Result<Self> {
        let src = cstring(src)?;
        let ccc_id = cstring(ccc_id)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_cdl_transform_create_from_file(
                src.as_ptr().cast(),
                ccc_id.as_ptr().cast(),
            )
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer create_from_file()")]
    pub fn from_file(src: impl AsRef<str>, ccc_id: impl AsRef<str>) -> Result<Self> {
        let src = cstring(src)?;
        let ccc_id = cstring(ccc_id)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_cdl_transform_from_file(src.as_ptr().cast(), ccc_id.as_ptr().cast())
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create a group transform from a CCC file.
    pub fn create_group_from_file(src: impl AsRef<str>) -> Result<GroupTransform> {
        let src = cstring(src)?;
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_cdl_transform_create_group_from_file(src.as_ptr().cast()) };
        crate::handle_result(handle).map(|handle| GroupTransform { handle })
    }

    /// Return the per-channel slope values.
    pub fn slope(&self) -> [f64; 3] {
        let mut rgb = [1.0f64; 3];
        unsafe {
            ocio_sys::ocio_cdl_transform_get_slope(
                self.handle.as_ptr(),
                rgb.as_mut_ptr() as *mut c_void,
            )
        };
        rgb
    }

    /// Set the per-channel slope values.
    pub fn set_slope(&self, rgb: &[f64; 3]) {
        self.try_set_slope(rgb).expect("failed to set CDL slope");
    }

    /// Set slope values and surface any OCIO validation error.
    pub fn try_set_slope(&self, rgb: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cdl_transform_set_slope(
                self.handle.as_ptr(),
                rgb.as_ptr() as *mut c_void,
            )
        };
        crate::ocio_call_status()
    }

    /// Return the per-channel offset values.
    pub fn offset(&self) -> [f64; 3] {
        let mut rgb = [0.0f64; 3];
        unsafe {
            ocio_sys::ocio_cdl_transform_get_offset(
                self.handle.as_ptr(),
                rgb.as_mut_ptr() as *mut c_void,
            )
        };
        rgb
    }

    /// Set the per-channel offset values.
    pub fn set_offset(&self, rgb: &[f64; 3]) {
        self.try_set_offset(rgb).expect("failed to set CDL offset");
    }

    /// Set offset values and surface any OCIO validation error.
    pub fn try_set_offset(&self, rgb: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cdl_transform_set_offset(
                self.handle.as_ptr(),
                rgb.as_ptr() as *mut c_void,
            )
        };
        crate::ocio_call_status()
    }

    /// Return the per-channel power values.
    pub fn power_(&self) -> [f64; 3] {
        let mut rgb = [1.0f64; 3];
        unsafe {
            ocio_sys::ocio_cdl_transform_get_power(
                self.handle.as_ptr(),
                rgb.as_mut_ptr() as *mut c_void,
            )
        };
        rgb
    }

    /// Set the per-channel power values.
    pub fn set_power(&self, rgb: &[f64; 3]) {
        self.try_set_power(rgb).expect("failed to set CDL power");
    }

    /// Set power values and surface any OCIO validation error.
    pub fn try_set_power(&self, rgb: &[f64; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cdl_transform_set_power(
                self.handle.as_ptr(),
                rgb.as_ptr() as *mut c_void,
            )
        };
        crate::ocio_call_status()
    }

    /// Return the saturation value.
    pub fn sat(&self) -> f64 {
        unsafe { ocio_sys::ocio_cdl_transform_get_sat(self.handle.as_ptr()) }
    }

    /// Set the saturation value.
    pub fn set_sat(&self, sat: f64) {
        self.try_set_sat(sat).expect("failed to set CDL saturation");
    }

    /// Set saturation and surface any OCIO validation error.
    pub fn try_set_sat(&self, sat: f64) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_cdl_transform_set_sat(self.handle.as_ptr(), sat) };
        crate::ocio_call_status()
    }

    /// Return the luma coefficients used for saturation.
    pub fn sat_luma_coefs(&self) -> [f64; 3] {
        let mut rgb = [0.0f64; 3];
        unsafe {
            ocio_sys::ocio_cdl_transform_get_sat_luma_coefs(
                self.handle.as_ptr(),
                rgb.as_mut_ptr() as *mut c_void,
            );
        }
        rgb
    }

    /// Return the current CDL style.
    pub fn style(&self) -> CDLStyle {
        let s = unsafe { ocio_sys::ocio_cdl_transform_get_style(self.handle.as_ptr()) };
        match s {
            1 => CDLStyle::NoClamp,
            _ => CDLStyle::Asc,
        }
    }

    /// Set the CDL style.
    pub fn set_style(&self, style: CDLStyle) {
        self.try_set_style(style).expect("failed to set CDL style");
    }

    /// Set the CDL style and surface any OCIO validation error.
    pub fn try_set_style(&self, style: CDLStyle) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_cdl_transform_set_style(self.handle.as_ptr(), style as i32) };
        crate::ocio_call_status()
    }

    /// Return the CDL transform id.
    pub fn id(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_cdl_transform_get_id(self.handle.as_ptr())) }
    }

    /// Set the CDL transform id.
    pub fn set_id(&self, id: impl AsRef<str>) -> Result<()> {
        let id = cstring(id)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_cdl_transform_set_id(self.handle.as_ptr(), id.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Return the transform direction.
    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_cdl_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the transform direction.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set CDL transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cdl_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
        crate::ocio_call_status()
    }

    /// Return the slope, offset, and power values as a 9-element array.
    pub fn sop(&self) -> [f64; 9] {
        let mut vec9 = [0.0f64; 9];
        unsafe {
            ocio_sys::ocio_cdl_transform_get_sop(
                self.handle.as_ptr(),
                vec9.as_mut_ptr() as *mut c_void,
            )
        };
        vec9
    }

    /// Set the slope, offset, and power values from a 9-element array.
    pub fn set_sop(&self, vec9: &[f64; 9]) {
        self.try_set_sop(vec9).expect("failed to set CDL SOP");
    }

    /// Set slope, offset, and power in one call and surface any OCIO validation error.
    pub fn try_set_sop(&self, vec9: &[f64; 9]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cdl_transform_set_sop(self.handle.as_ptr(), vec9.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()
    }

    /// Return the first SOP description.
    pub fn first_sop_description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_cdl_transform_get_first_sop_description(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the first SOP description.
    pub fn set_first_sop_description(&self, desc: impl AsRef<str>) -> Result<()> {
        let d = cstring(desc)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cdl_transform_set_first_sop_description(
                self.handle.as_ptr(),
                d.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Create an independent copy of this transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
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
        unsafe { ocio_sys::ocio_cdl_transform_equals(self.handle.as_ptr(), other.handle.as_ptr()) }
    }
}

impl Drop for CDLTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_cdl_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_cdl() {
        let cdl = CDLTransform::create();
        assert!(cdl.is_ok());
    }

    #[test]
    fn slope_offset_power_no_crash() {
        let cdl = CDLTransform::create().unwrap();
        let _ = cdl.slope();
        let _ = cdl.offset();
        let _ = cdl.power_();
        cdl.set_slope(&[1.2, 1.0, 0.9]);
        cdl.set_offset(&[0.1, 0.0, -0.1]);
        cdl.set_power(&[1.1, 1.0, 0.95]);
    }

    #[test]
    fn saturation_no_crash() {
        let cdl = CDLTransform::create().unwrap();
        let _ = cdl.sat();
        cdl.set_sat(1.5);
    }

    #[test]
    fn style_no_crash() {
        let cdl = CDLTransform::create().unwrap();
        let _ = cdl.style();
        cdl.set_style(CDLStyle::NoClamp);
    }

    #[test]
    fn id_no_crash() {
        let cdl = CDLTransform::create().unwrap();
        let _ = cdl.id();
        assert!(cdl.set_id("MyID").is_ok());
    }

    #[test]
    fn sop_no_crash() {
        let cdl = CDLTransform::create().unwrap();
        let _ = cdl.sop();
        cdl.set_sop(&[1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        let _ = cdl.first_sop_description();
        assert!(cdl.set_first_sop_description("desc").is_ok());
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let cdl = CDLTransform::create().unwrap();
        let _ = cdl.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let cdl = CDLTransform::create().unwrap();
        let _ = cdl.format_metadata();
    }

    #[test]
    fn equals_no_crash() {
        let a = CDLTransform::create().unwrap();
        let b = CDLTransform::create().unwrap();
        let _ = a.equals(&b);
    }
}
