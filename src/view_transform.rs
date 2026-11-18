use std::collections::BTreeMap;
use std::ffi::{c_void, CStr};
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
    fn borrow_text(
        &mut self,
        getter: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    ) -> Result<Option<&CStr>> {
        crate::clear_last_error();
        let ptr = unsafe { getter(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr.cast()) })
        })
    }

    /// Borrow the native name without copying. External native aliases must not
    /// mutate this object during the borrow.
    ///
    /// ```compile_fail
    /// # use ocio_rs::ViewTransform;
    /// # fn example(mut view: ViewTransform) -> ocio_rs::Result<()> {
    /// let name = view.name_ref()?;
    /// view.set_name("changed")?;
    /// println!("{:?}", name);
    /// # Ok(()) }
    /// ```
    pub fn name_ref(&mut self) -> Result<Option<&CStr>> {
        self.borrow_text(ocio_sys::ocio_view_transform_get_name)
    }

    /// Borrow the native family without copying; the aliasing rule of
    /// [`Self::name_ref`] also applies.
    pub fn family_ref(&mut self) -> Result<Option<&CStr>> {
        self.borrow_text(ocio_sys::ocio_view_transform_get_family)
    }

    /// Borrow the native description without copying; the aliasing rule of
    /// [`Self::name_ref`] also applies.
    pub fn description_ref(&mut self) -> Result<Option<&CStr>> {
        self.borrow_text(ocio_sys::ocio_view_transform_get_description)
    }

    /// Return OCIO's diagnostic stream output, not a stable interchange format.
    /// It may contain sensitive paths from transforms.
    pub fn serialize(&self) -> Result<String> {
        crate::const_config_item::serialize_named_item(self.handle, 2, false)
    }
    /// Share the native transform without an editable copy.
    pub fn try_const_transform(
        &self,
        direction: ViewTransformDirection,
    ) -> Result<Option<crate::transform::ConstTransform>> {
        crate::transform::ConstTransform::from_parent(self.handle.as_ptr(), 5, direction as i32)
    }
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
        self.try_name().ok().flatten()
    }

    /// Get the name while preserving bridge errors.
    pub fn try_name(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value =
            unsafe { cstr_from_mut(ocio_sys::ocio_view_transform_get_name(self.handle.as_ptr())) };
        crate::ocio_call_status()?;
        Ok(value)
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
        self.try_family().ok().flatten()
    }

    /// Get the family while preserving bridge errors.
    pub fn try_family(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_view_transform_get_family(
                self.handle.as_ptr(),
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
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
        self.try_description().ok().flatten()
    }

    /// Get the description while preserving bridge errors.
    pub fn try_description(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_view_transform_get_description(
                self.handle.as_ptr(),
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
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
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
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
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn interchange_attribute(&self, name: impl AsRef<str>) -> Option<String> {
        self.try_interchange_attribute(name).ok().flatten()
    }

    /// Get an interchange attribute value while preserving invalid-name errors.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
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
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn interchange_attributes(&self) -> BTreeMap<String, String> {
        self.try_interchange_attributes().unwrap_or_default()
    }

    /// Return a snapshot of all interchange attributes, preserving errors.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn try_interchange_attributes(&self) -> Result<BTreeMap<String, String>> {
        crate::colorspace::interchange_attributes_snapshot(self.handle.as_ptr(), false, Some(2))
    }

    /// Check whether this view transform has the given category.
    pub fn has_category(&self, category: impl AsRef<str>) -> bool {
        self.try_has_category(category).unwrap_or(false)
    }

    /// Check membership while preserving invalid-input and native errors.
    pub fn try_has_category(&self, category: impl AsRef<str>) -> Result<bool> {
        self.try_has_category_cstr(&cstring(category)?)
    }

    /// Check membership without allocating an intermediate C string.
    pub fn try_has_category_cstr(&self, category: &std::ffi::CStr) -> Result<bool> {
        crate::clear_last_error();
        let present = unsafe {
            ocio_sys::ocio_view_transform_has_category(
                self.handle.as_ptr(),
                category.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()?;
        Ok(present)
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
        self.try_num_categories().unwrap_or(0)
    }

    /// Get the category count while preserving native errors.
    pub fn try_num_categories(&self) -> Result<i32> {
        crate::clear_last_error();
        let count =
            unsafe { ocio_sys::ocio_view_transform_get_num_categories(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(count)
    }

    /// Get the category name at the given index.
    pub fn category(&self, index: i32) -> Option<String> {
        self.try_category(index).ok().flatten()
    }

    /// Get a category by index while preserving native errors.
    pub fn try_category(&self, index: i32) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_view_transform_get_category(
                self.handle.as_ptr(),
                index,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
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
        self.try_reference_space_type()
            .unwrap_or(ReferenceSpaceType::Scene)
    }

    /// Query the reference space without silently accepting unknown native values.
    pub fn try_reference_space_type(&self) -> Result<ReferenceSpaceType> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_view_transform_get_reference_space_type(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        match value {
            0 => Ok(ReferenceSpaceType::Scene),
            1 => Ok(ReferenceSpaceType::Display),
            _ => Err(crate::OcioError::Ocio(format!(
                "unknown reference space type {value}"
            ))),
        }
    }

    /// Get an editable copy for the given direction. A copied group still
    /// shares its children; see [`crate::transform::GroupTransform::create_deep_copy`].
    pub fn transform(&self, direction: ViewTransformDirection) -> Option<Transform> {
        self.try_transform(direction).ok().flatten()
    }

    /// Copy the transform for the given direction, preserving OCIO query failures.
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

    /// Set a shared read-only transform or clear the selected direction.
    /// OCIO copies a supplied transform into this view transform.
    pub fn try_set_const_transform(
        &self,
        transform: Option<&crate::transform::ConstTransform>,
        direction: ViewTransformDirection,
    ) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_item_set_const_transform(
                self.handle.as_ptr(),
                2,
                transform.map_or(std::ptr::null_mut(), |value| value.handle.as_ptr()),
                direction as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// Create an editable copy using OCIO's native copy semantics. A group
    /// transform in the copy may still share its children with the original.
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

    #[cfg(feature = "v2_5")]
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

    #[cfg(feature = "v2_5")]
    #[test]
    #[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
