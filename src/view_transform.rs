use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstr_from_mut, cstring, OcioError, Result, ReferenceSpaceType, TransformDirection};
use crate::transform::{TransformHandle, Transform, transform_from_raw_handle};

pub struct ViewTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl ViewTransform {
    pub fn create(reference_space: ReferenceSpaceType) -> Result<Self> {
        let handle = unsafe {
            std::ptr::null_mut()
        };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn name(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_view_transform_get_name(self.handle.as_ptr() as *mut c_void)) }
    }

    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe { ocio_sys::ocio_view_transform_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        Ok(())
    }

    pub fn src(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_view_transform_get_src(self.handle.as_ptr() as *mut c_void)) }
    }

    pub fn set_src(&self, src: impl AsRef<str>) -> Result<()> {
        let s = cstring(src)?;
        unsafe { ocio_sys::ocio_view_transform_set_src(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn family(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_view_transform_get_family(self.handle.as_ptr() as *mut c_void)) }
    }

    pub fn set_family(&self, family: impl AsRef<str>) -> Result<()> {
        let f = cstring(family)?;
        unsafe { ocio_sys::ocio_view_transform_set_family(self.handle.as_ptr(), f.as_ptr().cast()) };
        Ok(())
    }

    pub fn encoding(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_view_transform_get_encoding(self.handle.as_ptr() as *mut c_void)) }
    }

    pub fn set_encoding(&self, encoding: impl AsRef<str>) -> Result<()> {
        let e = cstring(encoding)?;
        unsafe { ocio_sys::ocio_view_transform_set_encoding(self.handle.as_ptr(), e.as_ptr().cast()) };
        Ok(())
    }

    pub fn display(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_view_transform_get_display(self.handle.as_ptr() as *mut c_void)) }
    }

    pub fn set_display(&self, display: impl AsRef<str>) -> Result<()> {
        let d = cstring(display)?;
        unsafe { ocio_sys::ocio_view_transform_set_display(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn view(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_view_transform_get_view(self.handle.as_ptr() as *mut c_void)) }
    }

    pub fn set_view(&self, view: impl AsRef<str>) -> Result<()> {
        let v = cstring(view)?;
        unsafe { ocio_sys::ocio_view_transform_set_view(self.handle.as_ptr(), v.as_ptr().cast()) };
        Ok(())
    }

    pub fn looks_bypass(&self) -> bool {
        unsafe { ocio_sys::ocio_view_transform_get_looks_bypass(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn set_looks_bypass(&self, bypass: bool) {
        unsafe { ocio_sys::ocio_view_transform_set_looks_bypass(self.handle.as_ptr(), bypass) };
    }

    pub fn rule(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_view_transform_get_rule(self.handle.as_ptr() as *mut c_void)) }
    }

    pub fn set_rule(&self, rule: impl AsRef<str>) -> Result<()> {
        let r = cstring(rule)?;
        unsafe { ocio_sys::ocio_view_transform_set_rule(self.handle.as_ptr(), r.as_ptr().cast()) };
        Ok(())
    }

    pub fn transform(&self) -> Option<Transform> {
        let handle = unsafe {
            std::ptr::null_mut()
        };
        transform_from_raw_handle(handle)
    }

    pub fn set_transform(&self, _transform: &impl TransformHandle) {
        // v2.5.1: API changed — takes 1 param now
    }

    pub fn inverse_transform(&self) -> Option<Transform> {
        let handle = unsafe {
            ocio_sys::ocio_view_transform_get_inverse_transform(self.handle.as_ptr() as *mut c_void)
        };
        transform_from_raw_handle(handle)
    }

    pub fn set_inverse_transform(&self, transform: &impl TransformHandle) {
        unsafe {
            ocio_sys::ocio_view_transform_set_inverse_transform(
                self.handle.as_ptr(), transform.as_ptr() as *mut c_void,
            );
        }
    }

    pub fn reference_space_type(&self) -> ReferenceSpaceType {
        let r = unsafe {
            ocio_sys::ocio_view_transform_get_reference_space_type(self.handle.as_ptr() as *mut c_void)
        };
        match r { 1 => ReferenceSpaceType::Display, _ => ReferenceSpaceType::Scene }
    }

    pub fn set_reference_space_type(&self, ref_type: ReferenceSpaceType) {
        unsafe {
            ocio_sys::ocio_view_transform_set_reference_space_type(
                self.handle.as_ptr(), ref_type as i32,
            );
        }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_view_transform_get_direction(self.handle.as_ptr() as *mut c_void) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_view_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn num_aliases(&self) -> i32 {
        unsafe { ocio_sys::ocio_view_transform_get_num_aliases(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn alias(&self, index: i32) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_view_transform_get_alias(self.handle.as_ptr(), index)) }
    }

    pub fn add_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        unsafe { ocio_sys::ocio_view_transform_add_alias(self.handle.as_ptr(), a.as_ptr().cast()) };
        Ok(())
    }

    pub fn remove_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        unsafe { ocio_sys::ocio_view_transform_remove_alias(self.handle.as_ptr(), a.as_ptr().cast()) };
        Ok(())
    }

    pub fn clear_aliases(&self) {
        unsafe { ocio_sys::ocio_view_transform_clear_aliases(self.handle.as_ptr() as *mut c_void) };
    }

    pub fn is_inactive(&self) -> bool {
        unsafe { ocio_sys::ocio_view_transform_is_inactive(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn set_inactive(&self, inactive: bool) {
        unsafe { ocio_sys::ocio_view_transform_set_inactive(self.handle.as_ptr(), inactive) };
    }

    pub fn category(&self) -> Option<String> {
        unsafe { cstr_from_mut(std::ptr::null_mut()) }
    }

    pub fn set_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        unsafe { ocio_sys::ocio_view_transform_set_category(self.handle.as_ptr(), c.as_ptr().cast()) };
        Ok(())
    }

    pub fn description(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_view_transform_get_description(self.handle.as_ptr() as *mut c_void)) }
    }

    pub fn set_description(&self, desc: impl AsRef<str>) -> Result<()> {
        let d = cstring(desc)?;
        unsafe { ocio_sys::ocio_view_transform_set_description(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_view_transform_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }
}

impl Drop for ViewTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_view_transform_destroy(self.handle.as_ptr() as *mut c_void) };
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

    #[test]
    fn aliases_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let _ = vt.num_aliases();
        let _ = vt.alias(0);
        assert!(vt.add_alias("test_alias").is_ok());
        assert!(vt.remove_alias("test_alias").is_ok());
        vt.clear_aliases();
    }

    #[test]
    fn inactive_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let _ = vt.is_inactive();
        vt.set_inactive(true);
        vt.set_inactive(false);
    }

    #[test]
    fn category_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let _ = vt.category();
        assert!(vt.set_category("test_category").is_ok());
    }

    #[test]
    fn description_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let _ = vt.description();
        assert!(vt.set_description("test description").is_ok());
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let _ = vt.create_editable_copy();
    }

    #[test]
    fn name_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let _ = vt.name();
        assert!(vt.set_name("MyViewTransform").is_ok());
    }

    #[test]
    fn family_encoding_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let _ = vt.family();
        assert!(vt.set_family("TestFamily").is_ok());
        let _ = vt.encoding();
        assert!(vt.set_encoding("scene-linear").is_ok());
    }

    #[test]
    fn inverse_transform_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let _ = vt.inverse_transform();
        let ft = crate::transform::FileTransform::create().unwrap();
        vt.set_inverse_transform(&ft);
    }

    #[test]
    fn reference_space_type_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let _ = vt.reference_space_type();
        vt.set_reference_space_type(ReferenceSpaceType::Display);
    }
}
