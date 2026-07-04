use std::collections::BTreeMap;
use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;

use crate::transform::{transform_from_raw_handle, Transform, TransformHandle};
use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, ReferenceSpaceType, Result, ViewTransformDirection,
};

/// Describes a scene/display view transform entry stored in a [`Config`](crate::Config).
pub struct ViewTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl ViewTransform {
    pub fn create_with_reference_space(reference_space: ReferenceSpaceType) -> Result<Self> {
        Self::create(reference_space)
    }

    pub fn create(reference_space: ReferenceSpaceType) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_view_transform_create_with_reference_space(reference_space as i32)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn name(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_view_transform_get_name(self.handle.as_ptr())) }
    }

    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe { ocio_sys::ocio_view_transform_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        Ok(())
    }

    pub fn family(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_view_transform_get_family(
                self.handle.as_ptr(),
            ))
        }
    }

    pub fn set_family(&self, family: impl AsRef<str>) -> Result<()> {
        let f = cstring(family)?;
        unsafe {
            ocio_sys::ocio_view_transform_set_family(self.handle.as_ptr(), f.as_ptr().cast())
        };
        Ok(())
    }

    pub fn description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_view_transform_get_description(
                self.handle.as_ptr(),
            ))
        }
    }

    pub fn set_description(&self, desc: impl AsRef<str>) -> Result<()> {
        let d = cstring(desc)?;
        unsafe {
            ocio_sys::ocio_view_transform_set_description(self.handle.as_ptr(), d.as_ptr().cast())
        };
        Ok(())
    }

    pub fn set_interchange_attribute(
        &self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<()> {
        let name = cstring(name)?;
        let value = cstring(value)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_view_transform_set_interchange_attribute(
                self.handle.as_ptr(),
                name.as_ptr().cast(),
                value.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    pub fn interchange_attribute(&self, name: impl AsRef<str>) -> Option<String> {
        let name = cstring(name).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_view_transform_get_interchange_attribute(
                self.handle.as_ptr(),
                name.as_ptr(),
            ))
        }
    }

    pub fn interchange_attributes(&self) -> BTreeMap<String, String> {
        let mut attrs = BTreeMap::new();
        let count = unsafe {
            ocio_sys::ocio_view_transform_get_num_interchange_attributes(self.handle.as_ptr())
        };
        for index in 0..count {
            let name = unsafe {
                cstr_to_opt_string(
                    ocio_sys::ocio_view_transform_get_interchange_attribute_name_by_index(
                        self.handle.as_ptr(),
                        index,
                    ),
                )
            };
            let value = unsafe {
                cstr_to_opt_string(
                    ocio_sys::ocio_view_transform_get_interchange_attribute_value_by_index(
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

    pub fn has_category(&self, category: impl AsRef<str>) -> bool {
        let category = match cstring(category) {
            Ok(category) => category,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_view_transform_has_category(
                self.handle.as_ptr(),
                category.as_ptr().cast(),
            )
        }
    }

    pub fn add_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        unsafe {
            ocio_sys::ocio_view_transform_add_category(self.handle.as_ptr(), c.as_ptr().cast())
        };
        Ok(())
    }

    pub fn remove_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        unsafe {
            ocio_sys::ocio_view_transform_remove_category(self.handle.as_ptr(), c.as_ptr().cast())
        };
        Ok(())
    }

    pub fn num_categories(&self) -> i32 {
        unsafe { ocio_sys::ocio_view_transform_get_num_categories(self.handle.as_ptr()) }
    }

    pub fn category(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_view_transform_get_category(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    pub fn clear_categories(&self) {
        unsafe { ocio_sys::ocio_view_transform_clear_categories(self.handle.as_ptr()) };
    }

    pub fn reference_space_type(&self) -> ReferenceSpaceType {
        let r =
            unsafe { ocio_sys::ocio_view_transform_get_reference_space_type(self.handle.as_ptr()) };
        match r {
            1 => ReferenceSpaceType::Display,
            _ => ReferenceSpaceType::Scene,
        }
    }

    pub fn transform(&self, direction: ViewTransformDirection) -> Option<Transform> {
        let handle = unsafe {
            ocio_sys::ocio_view_transform_get_transform(self.handle.as_ptr(), direction as i32)
        };
        transform_from_raw_handle(handle)
    }

    pub fn set_transform(
        &self,
        transform: Option<&impl TransformHandle>,
        direction: ViewTransformDirection,
    ) {
        let transform = transform.map_or(std::ptr::null_mut(), TransformHandle::as_ptr);
        unsafe {
            ocio_sys::ocio_view_transform_set_transform(
                self.handle.as_ptr(),
                transform,
                direction as i32,
            );
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_view_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
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
    use crate::transform::FileTransform;

    #[test]
    fn create_view_transform() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene);
        assert!(vt.is_ok());
    }

    #[test]
    fn create_view_transform_with_reference_space() {
        let vt = ViewTransform::create_with_reference_space(ReferenceSpaceType::Display);
        assert!(vt.is_ok());
    }

    #[test]
    fn metadata_round_trip_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        assert!(vt.set_name("MyViewTransform").is_ok());
        assert!(vt.set_family("TestFamily").is_ok());
        assert!(vt.set_description("test description").is_ok());
        assert!(vt
            .set_interchange_attribute("amf_transform_ids", "urn:ampas:aces:transformId:v1.5:ODT")
            .is_ok());
        let _ = vt.name();
        let _ = vt.family();
        let _ = vt.description();
        let _ = vt.interchange_attribute("amf_transform_ids");
        let _ = vt.interchange_attributes();
    }

    #[test]
    fn interchange_attribute_real_round_trip() {
        if crate::is_stub_build() {
            return;
        }

        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        vt.set_interchange_attribute("amf_transform_ids", "urn:test:view")
            .unwrap();
        assert_eq!(
            vt.interchange_attribute("amf_transform_ids").as_deref(),
            Some("urn:test:view")
        );
        assert_eq!(
            vt.interchange_attributes()
                .get("amf_transform_ids")
                .map(String::as_str),
            Some("urn:test:view")
        );
    }

    #[test]
    fn categories_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        assert!(vt.add_category("viewing").is_ok());
        let _ = vt.has_category("viewing");
        let _ = vt.num_categories();
        let _ = vt.category(0);
        assert!(vt.remove_category("viewing").is_ok());
        vt.clear_categories();
    }

    #[test]
    fn transform_directions_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let ft = FileTransform::create().unwrap();
        vt.set_transform(Some(&ft), ViewTransformDirection::ToReference);
        let _ = vt.transform(ViewTransformDirection::ToReference);
        vt.set_transform(None::<&FileTransform>, ViewTransformDirection::ToReference);
    }

    #[test]
    fn reference_space_type_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Display).unwrap();
        let _ = vt.reference_space_type();
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let vt = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
        let _ = vt.create_editable_copy();
    }
}
