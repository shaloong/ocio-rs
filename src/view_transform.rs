use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, OcioError, Result, ReferenceSpaceType, TransformDirection};
use crate::transform::{TransformHandle, Transform, transform_from_raw_handle};

pub struct ViewTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl ViewTransform {
    pub fn create(reference_space: ReferenceSpaceType) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_view_transform_create(reference_space as i32)
        };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn src(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_view_transform_get_src(self.handle.as_ptr())) }
    }

    pub fn set_src(&self, src: impl AsRef<str>) -> Result<()> {
        let s = cstring(src)?;
        unsafe { ocio_sys::ocio_view_transform_set_src(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn display(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_view_transform_get_display(self.handle.as_ptr())) }
    }

    pub fn set_display(&self, display: impl AsRef<str>) -> Result<()> {
        let d = cstring(display)?;
        unsafe { ocio_sys::ocio_view_transform_set_display(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn view(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_view_transform_get_view(self.handle.as_ptr())) }
    }

    pub fn set_view(&self, view: impl AsRef<str>) -> Result<()> {
        let v = cstring(view)?;
        unsafe { ocio_sys::ocio_view_transform_set_view(self.handle.as_ptr(), v.as_ptr().cast()) };
        Ok(())
    }

    pub fn looks_bypass(&self) -> bool {
        unsafe { ocio_sys::ocio_view_transform_get_looks_bypass(self.handle.as_ptr()) }
    }

    pub fn set_looks_bypass(&self, bypass: bool) {
        unsafe { ocio_sys::ocio_view_transform_set_looks_bypass(self.handle.as_ptr(), bypass) };
    }

    pub fn rule(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_view_transform_get_rule(self.handle.as_ptr())) }
    }

    pub fn set_rule(&self, rule: impl AsRef<str>) -> Result<()> {
        let r = cstring(rule)?;
        unsafe { ocio_sys::ocio_view_transform_set_rule(self.handle.as_ptr(), r.as_ptr().cast()) };
        Ok(())
    }

    pub fn transform(&self) -> Option<Transform> {
        let handle = unsafe {
            ocio_sys::ocio_view_transform_get_transform(self.handle.as_ptr())
        };
        transform_from_raw_handle(handle)
    }

    pub fn set_transform(&self, transform: &impl TransformHandle) {
        unsafe {
            ocio_sys::ocio_view_transform_set_transform(
                self.handle.as_ptr(), transform.as_ptr() as *const c_void,
            );
        }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_view_transform_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_view_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }
}

impl Drop for ViewTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_view_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ReferenceSpaceType, TransformDirection};
    use crate::transform::FileTransform;

    #[test]
    fn create_view_transform() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene);
        assert!(vt.is_ok());
    }

    #[test]
    fn view_transform_methods_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let _ = vt.src();
        let _ = vt.display();
        let _ = vt.view();
        let _ = vt.looks_bypass();
        let _ = vt.rule();
        let _ = vt.direction();
        let _ = vt.transform();
    }

    #[test]
    fn set_src_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        assert!(vt.set_src("ACEScg").is_ok());
    }

    #[test]
    fn set_display_view_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        assert!(vt.set_display("sRGB").is_ok());
        assert!(vt.set_view("ACES 1.0").is_ok());
    }

    #[test]
    fn set_looks_bypass_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        vt.set_looks_bypass(true);
    }

    #[test]
    fn set_rule_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        assert!(vt.set_rule("srgb").is_ok());
    }

    #[test]
    fn set_direction_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        vt.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn set_transform_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let ft = FileTransform::create().unwrap();
        vt.set_transform(&ft);
    }
}
