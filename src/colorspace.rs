use std::collections::BTreeMap;
use std::ffi::c_void;
use std::ptr::NonNull;

use crate::transform::{transform_from_raw_handle, Transform, TransformHandle};
use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, Allocation, BitDepth, ColorSpaceDirection,
    OcioError, ReferenceSpaceType, Result,
};
use ocio_sys;

/// Describes a color space definition stored in a [`Config`](crate::Config).
pub struct ColorSpace {
    pub(crate) handle: NonNull<c_void>,
}

impl ColorSpace {
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_color_space_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_color_space_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn name(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_color_space_get_name(self.handle.as_ptr())) }
    }

    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe { ocio_sys::ocio_color_space_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        Ok(())
    }

    pub fn family(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_color_space_get_family(self.handle.as_ptr())) }
    }

    pub fn set_family(&self, family: impl AsRef<str>) -> Result<()> {
        let f = cstring(family)?;
        unsafe { ocio_sys::ocio_color_space_set_family(self.handle.as_ptr(), f.as_ptr().cast()) };
        Ok(())
    }

    pub fn equality_group(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_equality_group(
                self.handle.as_ptr(),
            ))
        }
    }

    pub fn set_equality_group(&self, group: impl AsRef<str>) -> Result<()> {
        let g = cstring(group)?;
        unsafe {
            ocio_sys::ocio_color_space_set_equality_group(self.handle.as_ptr(), g.as_ptr().cast())
        };
        Ok(())
    }

    pub fn description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_description(
                self.handle.as_ptr(),
            ))
        }
    }

    pub fn set_description(&self, description: impl AsRef<str>) -> Result<()> {
        let d = cstring(description)?;
        unsafe {
            ocio_sys::ocio_color_space_set_description(self.handle.as_ptr(), d.as_ptr().cast())
        };
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
        let r =
            unsafe { ocio_sys::ocio_color_space_get_reference_space_type(self.handle.as_ptr()) };
        match r {
            1 => ReferenceSpaceType::Display,
            _ => ReferenceSpaceType::Scene,
        }
    }

    pub fn is_data(&self) -> bool {
        unsafe { ocio_sys::ocio_color_space_is_data(self.handle.as_ptr()) }
    }

    pub fn set_is_data(&self, is_data: bool) {
        unsafe { ocio_sys::ocio_color_space_set_is_data(self.handle.as_ptr(), is_data) };
    }

    pub fn category(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_category(
                self.handle.as_ptr(),
                0,
            ))
        }
    }

    pub fn set_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        unsafe { ocio_sys::ocio_color_space_add_category(self.handle.as_ptr(), c.as_ptr().cast()) };
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
        unsafe {
            ocio_sys::ocio_color_space_set_allocation(self.handle.as_ptr(), allocation as i32)
        };
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
                self.handle.as_ptr(),
                vars.as_mut_ptr() as *mut c_void,
            );
        }
        vars
    }

    pub fn set_allocation_vars(&self, vars: &[f32]) {
        unsafe {
            ocio_sys::ocio_color_space_set_allocation_vars(
                self.handle.as_ptr(),
                vars.len() as i32,
                vars.as_ptr() as *mut c_void,
            );
        }
    }

    pub fn encoding(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_encoding(
                self.handle.as_ptr(),
            ))
        }
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
                transform.as_ptr() as *mut c_void,
                direction as i32,
            );
        }
    }

    pub fn num_aliases(&self) -> i32 {
        unsafe { ocio_sys::ocio_color_space_get_num_aliases(self.handle.as_ptr()) as i32 }
    }

    pub fn alias(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_alias(
                self.handle.as_ptr(),
                index as usize,
            ))
        }
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

    pub fn is_transform_defined(&self, direction: ColorSpaceDirection) -> bool {
        unsafe {
            ocio_sys::ocio_color_space_is_transform_defined(self.handle.as_ptr(), direction as i32)
        }
    }

    // ── v2.5.1 new methods ──

    pub fn add_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        unsafe { ocio_sys::ocio_color_space_add_category(self.handle.as_ptr(), c.as_ptr().cast()) };
        Ok(())
    }

    pub fn remove_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        unsafe {
            ocio_sys::ocio_color_space_remove_category(self.handle.as_ptr(), c.as_ptr().cast())
        };
        Ok(())
    }

    pub fn clear_categories(&self) {
        unsafe { ocio_sys::ocio_color_space_clear_categories(self.handle.as_ptr() as *mut c_void) };
    }

    pub fn has_category(&self, category: impl AsRef<str>) -> bool {
        let c = match cstring(category) {
            Ok(c) => c,
            Err(_) => return false,
        };
        unsafe { ocio_sys::ocio_color_space_has_category(self.handle.as_ptr(), c.as_ptr().cast()) }
    }

    pub fn num_categories(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_color_space_get_num_categories(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn has_alias(&self, alias: impl AsRef<str>) -> bool {
        let a = match cstring(alias) {
            Ok(a) => a,
            Err(_) => return false,
        };
        unsafe { ocio_sys::ocio_color_space_has_alias(self.handle.as_ptr(), a.as_ptr().cast()) }
    }

    pub fn interop_id(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_interop_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn set_interchange_attribute(
        &self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<()> {
        let name = cstring(name)?;
        let value = cstring(value)?;
        unsafe {
            ocio_sys::ocio_color_space_set_interchange_attribute(
                self.handle.as_ptr(),
                name.as_ptr().cast(),
                value.as_ptr().cast(),
            )
        };
        Ok(())
    }

    pub fn interchange_attribute(&self, name: impl AsRef<str>) -> Option<String> {
        let name = cstring(name).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_color_space_get_interchange_attribute(
                self.handle.as_ptr(),
                name.as_ptr(),
            ))
        }
    }

    pub fn interchange_attributes(&self) -> BTreeMap<String, String> {
        let mut attrs = BTreeMap::new();
        let count = unsafe {
            ocio_sys::ocio_color_space_get_num_interchange_attributes(self.handle.as_ptr())
        };
        for index in 0..count {
            let name = unsafe {
                cstr_to_opt_string(
                    ocio_sys::ocio_color_space_get_interchange_attribute_name_by_index(
                        self.handle.as_ptr(),
                        index,
                    ),
                )
            };
            let value = unsafe {
                cstr_to_opt_string(
                    ocio_sys::ocio_color_space_get_interchange_attribute_value_by_index(
                        self.handle.as_ptr(),
                        index,
                    ),
                )
            };
            if let (Some(name), Some(value)) = (name, value) {
                attrs.insert(name, value);
            }
        }
        attrs
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
    fn create_editable_copy_round_trip() {
        let cs = ColorSpace::create().unwrap();
        assert!(cs.set_name("EditableColorSpace").is_ok());
        let copy = cs.create_editable_copy().unwrap();
        if !crate::is_stub_build() {
            assert_eq!(copy.name().as_deref(), Some("EditableColorSpace"));
        }
    }

    #[test]
    fn interchange_attribute_no_crash() {
        let cs = ColorSpace::create().unwrap();
        assert!(cs
            .set_interchange_attribute("amf_transform_ids", "urn:ampas:aces:transformId:v1.5:CSC")
            .is_ok());
        let _ = cs.interchange_attribute("amf_transform_ids");
        let _ = cs.interchange_attributes();
    }

    #[test]
    fn interchange_attribute_real_round_trip() {
        if crate::is_stub_build() {
            return;
        }

        let cs = ColorSpace::create().unwrap();
        cs.set_interchange_attribute("amf_transform_ids", "urn:test:colorspace")
            .unwrap();
        assert_eq!(
            cs.interchange_attribute("amf_transform_ids").as_deref(),
            Some("urn:test:colorspace")
        );
        assert_eq!(
            cs.interchange_attributes()
                .get("amf_transform_ids")
                .map(String::as_str),
            Some("urn:test:colorspace")
        );
    }

    #[test]
    fn is_transform_defined_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let _ = cs.is_transform_defined(ColorSpaceDirection::ToReference);
        let _ = cs.is_transform_defined(ColorSpaceDirection::FromReference);
    }
}
