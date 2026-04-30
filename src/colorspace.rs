use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, OcioError, Result, ReferenceSpaceType, BitDepth, Allocation, ColorSpaceDirection, ColorSpaceVisibility};
use crate::transform::{TransformHandle, Transform, transform_from_raw_handle};

pub struct ColorSpace {
    pub(crate) handle: NonNull<c_void>,
}

impl ColorSpace {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_color_space_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_color_space_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn name(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_name(self.handle.as_ptr())) }
    }

    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe { ocio_sys::ocio_color_space_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        Ok(())
    }

    pub fn family(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_family(self.handle.as_ptr())) }
    }

    pub fn set_family(&self, family: impl AsRef<str>) -> Result<()> {
        let f = cstring(family)?;
        unsafe { ocio_sys::ocio_color_space_set_family(self.handle.as_ptr(), f.as_ptr().cast()) };
        Ok(())
    }

    pub fn equality_group(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_equality_group(self.handle.as_ptr())) }
    }

    pub fn set_equality_group(&self, group: impl AsRef<str>) -> Result<()> {
        let g = cstring(group)?;
        unsafe { ocio_sys::ocio_color_space_set_equality_group(self.handle.as_ptr(), g.as_ptr().cast()) };
        Ok(())
    }

    pub fn description(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_description(self.handle.as_ptr())) }
    }

    pub fn set_description(&self, description: impl AsRef<str>) -> Result<()> {
        let d = cstring(description)?;
        unsafe { ocio_sys::ocio_color_space_set_description(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn bit_depth(&self) -> BitDepth {
        let b = unsafe { ocio_sys::ocio_color_space_get_bit_depth(self.handle.as_ptr()) };
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

    pub fn set_bit_depth(&self, bit_depth: BitDepth) {
        unsafe { ocio_sys::ocio_color_space_set_bit_depth(self.handle.as_ptr(), bit_depth as i32) };
    }

    pub fn reference_space_type(&self) -> ReferenceSpaceType {
        let r = unsafe { ocio_sys::ocio_color_space_get_reference_space_type(self.handle.as_ptr()) };
        match r { 1 => ReferenceSpaceType::Display, _ => ReferenceSpaceType::Scene }
    }

    pub fn is_data(&self) -> bool {
        unsafe { ocio_sys::ocio_color_space_is_data(self.handle.as_ptr()) }
    }

    pub fn set_is_data(&self, is_data: bool) {
        unsafe { ocio_sys::ocio_color_space_set_is_data(self.handle.as_ptr(), is_data) };
    }

    pub fn category(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_category(self.handle.as_ptr())) }
    }

    pub fn set_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        unsafe { ocio_sys::ocio_color_space_set_category(self.handle.as_ptr(), c.as_ptr().cast()) };
        Ok(())
    }

    pub fn allocation(&self) -> Allocation {
        let a = unsafe { ocio_sys::ocio_color_space_get_allocation(self.handle.as_ptr()) };
        match a {
            1 => Allocation::Uniform,
            2 => Allocation::Lg2,
            _ => Allocation::Unknown,
        }
    }

    pub fn set_allocation(&self, allocation: Allocation) {
        unsafe { ocio_sys::ocio_color_space_set_allocation(self.handle.as_ptr(), allocation as i32) };
    }

    pub fn allocation_num_vars(&self) -> i32 {
        unsafe { ocio_sys::ocio_color_space_get_allocation_num_vars(self.handle.as_ptr()) }
    }

    pub fn allocation_vars(&self) -> Vec<f32> {
        let n = self.allocation_num_vars();
        if n <= 0 {
            return Vec::new();
        }
        let mut vars = vec![0.0f32; n as usize];
        unsafe {
            ocio_sys::ocio_color_space_get_allocation_vars(
                self.handle.as_ptr(), vars.as_mut_ptr(),
            );
        }
        vars
    }

    pub fn set_allocation_vars(&self, vars: &[f32]) {
        unsafe {
            ocio_sys::ocio_color_space_set_allocation_vars(
                self.handle.as_ptr(), vars.len() as i32, vars.as_ptr(),
            );
        }
    }

    pub fn encoding(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_encoding(self.handle.as_ptr())) }
    }

    pub fn set_encoding(&self, encoding: impl AsRef<str>) -> Result<()> {
        let e = cstring(encoding)?;
        unsafe { ocio_sys::ocio_color_space_set_encoding(self.handle.as_ptr(), e.as_ptr().cast()) };
        Ok(())
    }

    pub fn transform(&self, direction: ColorSpaceDirection) -> Option<Transform> {
        let handle = unsafe {
            ocio_sys::ocio_color_space_get_transform(self.handle.as_ptr(), direction as i32)
        };
        transform_from_raw_handle(handle)
    }

    pub fn set_transform(&self, transform: &impl TransformHandle, direction: ColorSpaceDirection) {
        unsafe {
            ocio_sys::ocio_color_space_set_transform(
                self.handle.as_ptr(),
                transform.as_ptr() as *const c_void,
                direction as i32,
            );
        }
    }

    pub fn num_aliases(&self) -> i32 {
        unsafe { ocio_sys::ocio_color_space_get_num_aliases(self.handle.as_ptr()) }
    }

    pub fn alias(&self, index: i32) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_alias(self.handle.as_ptr(), index)) }
    }

    pub fn add_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        unsafe { ocio_sys::ocio_color_space_add_alias(self.handle.as_ptr(), a.as_ptr().cast()) };
        Ok(())
    }

    pub fn remove_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        unsafe { ocio_sys::ocio_color_space_remove_alias(self.handle.as_ptr(), a.as_ptr().cast()) };
        Ok(())
    }

    pub fn clear_aliases(&self) {
        unsafe { ocio_sys::ocio_color_space_clear_aliases(self.handle.as_ptr()) };
    }

    pub fn is_inactive(&self) -> bool {
        unsafe { ocio_sys::ocio_color_space_is_inactive(self.handle.as_ptr()) }
    }

    pub fn set_inactive(&self, inactive: bool) {
        unsafe { ocio_sys::ocio_color_space_set_inactive(self.handle.as_ptr(), inactive) };
    }

    pub fn visibility(&self) -> ColorSpaceVisibility {
        let v = unsafe { ocio_sys::ocio_color_space_get_visibility(self.handle.as_ptr()) };
        match v {
            1 => ColorSpaceVisibility::Inactive,
            2 => ColorSpaceVisibility::All,
            _ => ColorSpaceVisibility::Active,
        }
    }

    pub fn set_visibility(&self, visibility: ColorSpaceVisibility) {
        unsafe { ocio_sys::ocio_color_space_set_visibility(self.handle.as_ptr(), visibility as i32) };
    }

    pub fn set_reference_space_type(&self, reference_space: ReferenceSpaceType) {
        unsafe {
            ocio_sys::ocio_color_space_set_reference_space_type(self.handle.as_ptr(), reference_space as i32);
        }
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_cache_id(self.handle.as_ptr())) }
    }

    pub fn is_transform_defined(&self, direction: ColorSpaceDirection) -> bool {
        unsafe { ocio_sys::ocio_color_space_is_transform_defined(self.handle.as_ptr(), direction as i32) }
    }
}

impl Drop for ColorSpace {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_color_space_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_color_space() {
        let cs = ColorSpace::create();
        assert!(cs.is_ok());
    }

    #[test]
    fn color_space_methods_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let _ = cs.name();
        let _ = cs.family();
        let _ = cs.description();
        let _ = cs.bit_depth();
        let _ = cs.reference_space_type();
        let _ = cs.is_data();
        let _ = cs.allocation();
        let _ = cs.encoding();
    }

    #[test]
    fn set_name() {
        let cs = ColorSpace::create().unwrap();
        assert!(cs.set_name("MyColorSpace").is_ok());
    }

    #[test]
    fn set_is_data() {
        let cs = ColorSpace::create().unwrap();
        cs.set_is_data(true);
        let _ = cs.is_data();
    }

    #[test]
    fn transform_no_crash() {
        let cs = ColorSpace::create().unwrap();
        // In stub mode, returns None since bridge has no transform
        let _ = cs.transform(ColorSpaceDirection::ToReference);
    }

    #[test]
    fn set_transform_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let ft = crate::transform::FileTransform::create().unwrap();
        cs.set_transform(&ft, ColorSpaceDirection::ToReference);
    }

    #[test]
    fn aliases_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let _ = cs.num_aliases();
        let _ = cs.alias(0);
        assert!(cs.add_alias("test_alias").is_ok());
        assert!(cs.remove_alias("test_alias").is_ok());
        cs.clear_aliases();
    }

    #[test]
    fn inactive_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let _ = cs.is_inactive();
        cs.set_inactive(true);
    }

    #[test]
    fn visibility_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let _ = cs.visibility();
        cs.set_visibility(ColorSpaceVisibility::Inactive);
    }

    #[test]
    fn set_reference_space_type_no_crash() {
        let cs = ColorSpace::create().unwrap();
        cs.set_reference_space_type(ReferenceSpaceType::Display);
    }

    #[test]
    fn cache_id_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let _ = cs.cache_id();
    }

    #[test]
    fn is_transform_defined_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let _ = cs.is_transform_defined(ColorSpaceDirection::ToReference);
        let _ = cs.is_transform_defined(ColorSpaceDirection::FromReference);
    }
}
