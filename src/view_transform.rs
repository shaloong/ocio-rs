use std::collections::BTreeMap;
use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;

use crate::transform::{transform_from_raw_handle, Transform, TransformHandle};
use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, ReferenceSpaceType, Result, ViewTransformDirection,
};

/// A scene-to-display or display-to-scene view transform entry.
///
/// View transforms define the color-space mapping between a scene-referred
/// reference space and a display-referred output. Each view in a display
/// definition typically references a view transform that provides the
/// scene-to-display mapping (e.g., an ACES output transform or a
/// film-emulation LUT).
///
/// A `ViewTransform` holds:
///
/// - **Metadata**: name, family, description, categories, and interchange
///   attributes (e.g., ACES transform IDs).
/// - **Transforms**: a scene-to-display (`FromReference`) and optionally a
///   display-to-scene (`ToReference`) transform, each of which may be any
///   supported OCIO transform type.
///
/// View transforms are typically obtained from a [`Config`](crate::Config)
/// via display/view queries, but can also be constructed independently and
/// attached to a config.
///
/// [`Config`]: crate::Config
pub struct ViewTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl ViewTransform {
    /// Create a new view transform with the given reference space (alias for [`Self::create`]).
    pub fn create_with_reference_space(reference_space: ReferenceSpaceType) -> Result<Self> {
        Self::create(reference_space)
    }

    /// Create a new view transform with the given reference space type.
    pub fn create(reference_space: ReferenceSpaceType) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_view_transform_create_with_reference_space(reference_space as i32)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Get the name of this view transform.
    pub fn name(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_view_transform_get_name(self.handle.as_ptr())) }
    }

    /// Set the name of this view transform.
    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_view_transform_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the family of this view transform.
    pub fn family(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_view_transform_get_family(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the family of this view transform.
    pub fn set_family(&self, family: impl AsRef<str>) -> Result<()> {
        let f = cstring(family)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_view_transform_set_family(self.handle.as_ptr(), f.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Get the description of this view transform.
    pub fn description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_view_transform_get_description(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the description of this view transform.
    pub fn set_description(&self, desc: impl AsRef<str>) -> Result<()> {
        let d = cstring(desc)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_view_transform_set_description(self.handle.as_ptr(), d.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Set an interchange attribute (e.g. ACES transform ID) by name and value.
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

    /// Get an interchange attribute value by name.
    pub fn interchange_attribute(&self, name: impl AsRef<str>) -> Option<String> {
        self.try_interchange_attribute(name).ok().flatten()
    }

    /// Get an interchange attribute value while preserving invalid-name errors.
    pub fn try_interchange_attribute(&self, name: impl AsRef<str>) -> Result<Option<String>> {
        let name = cstring(name)?;
        crate::clear_last_error();
        let value = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_view_transform_get_interchange_attribute(
                self.handle.as_ptr(),
                name.as_ptr(),
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Get all interchange attributes as a map.
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

    /// Check whether this view transform has the given category.
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

    /// Add a category to this view transform.
    pub fn add_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_view_transform_add_category(self.handle.as_ptr(), c.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Remove a category from this view transform.
    pub fn remove_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_view_transform_remove_category(self.handle.as_ptr(), c.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Get the number of categories on this view transform.
    pub fn num_categories(&self) -> i32 {
        unsafe { ocio_sys::ocio_view_transform_get_num_categories(self.handle.as_ptr()) }
    }

    /// Get the category name at the given index.
    pub fn category(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_view_transform_get_category(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Clear all categories (panics on error; use [`Self::try_clear_categories`] for fallible version).
    pub fn clear_categories(&self) {
        self.try_clear_categories()
            .expect("failed to clear view transform categories");
    }

    /// Clear all categories and surface any OCIO validation error.
    pub fn try_clear_categories(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_view_transform_clear_categories(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    /// Get the reference space type for this view transform.
    pub fn reference_space_type(&self) -> ReferenceSpaceType {
        let r =
            unsafe { ocio_sys::ocio_view_transform_get_reference_space_type(self.handle.as_ptr()) };
        match r {
            1 => ReferenceSpaceType::Display,
            _ => ReferenceSpaceType::Scene,
        }
    }

    /// Get the transform for the given direction (scene-to-display or display-to-scene).
    pub fn transform(&self, direction: ViewTransformDirection) -> Option<Transform> {
        self.try_transform(direction).ok().flatten()
    }

    /// Get the transform for the given direction, preserving OCIO query failures.
    pub fn try_transform(&self, direction: ViewTransformDirection) -> Result<Option<Transform>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_view_transform_get_transform(self.handle.as_ptr(), direction as i32)
        };
        crate::ocio_call_status()?;
        Ok(transform_from_raw_handle(handle))
    }

    /// Set or clear the transform for the given direction (panics on error).
    pub fn set_transform(
        &self,
        transform: Option<&impl TransformHandle>,
        direction: ViewTransformDirection,
    ) {
        self.try_set_transform(transform, direction)
            .expect("failed to set view transform");
    }

    /// Try to attach or clear a transform for the given direction.
    pub fn try_set_transform(
        &self,
        transform: Option<&impl TransformHandle>,
        direction: ViewTransformDirection,
    ) -> Result<()> {
        let transform = transform.map_or(std::ptr::null_mut(), TransformHandle::as_ptr);
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_view_transform_set_transform(
                self.handle.as_ptr(),
                transform,
                direction as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Create an editable deep copy of this view transform.
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
        vt.try_clear_categories().unwrap();
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
