use std::collections::BTreeMap;
use std::ffi::{c_void, CStr};
use std::ptr::NonNull;

use crate::transform::{transform_from_raw_handle, Transform, TransformHandle};
use crate::{cstr_from_mut, cstr_to_opt_string, cstring, OcioError, Result};
use ocio_sys;

/// Wraps an OCIO look definition with forward and inverse transform slots.
pub struct Look {
    pub(crate) handle: NonNull<c_void>,
}

impl Look {
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

    /// Borrow the native name without copying. The exclusive borrow prevents
    /// mutation through this handle while the returned string is in use.
    /// External native code must not mutate the same object during the borrow.
    ///
    /// ```compile_fail
    /// # use ocio_rs::Look;
    /// # fn example(mut look: Look) -> ocio_rs::Result<()> {
    /// let name = look.name_ref()?;
    /// look.set_name("changed")?;
    /// println!("{:?}", name);
    /// # Ok(()) }
    /// ```
    pub fn name_ref(&mut self) -> Result<Option<&CStr>> {
        self.borrow_text(ocio_sys::ocio_look_get_name)
    }

    /// Borrow the native process-space name without copying.
    pub fn process_space_ref(&mut self) -> Result<Option<&CStr>> {
        self.borrow_text(ocio_sys::ocio_look_get_process_space)
    }

    /// Borrow the native description without copying.
    pub fn description_ref(&mut self) -> Result<Option<&CStr>> {
        self.borrow_text(ocio_sys::ocio_look_get_description)
    }

    /// Return OCIO's diagnostic stream output, not a stable interchange format.
    /// It may contain sensitive paths from transforms.
    pub fn serialize(&self) -> Result<String> {
        crate::const_config_item::serialize_named_item(self.handle, 0, false)
    }
    /// Share the native forward transform without an editable copy.
    pub fn try_const_transform(&self) -> Result<Option<crate::transform::ConstTransform>> {
        crate::transform::ConstTransform::from_parent(self.handle.as_ptr(), 2, 0)
    }
    /// Share the native inverse transform without an editable copy.
    pub fn try_const_inverse_transform(&self) -> Result<Option<crate::transform::ConstTransform>> {
        crate::transform::ConstTransform::from_parent(self.handle.as_ptr(), 3, 0)
    }
    /// Create a new default look.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_look_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an editable copy of this look.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_look_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Get the name of this look.
    pub fn name(&self) -> Option<String> {
        self.try_name().ok().flatten()
    }

    /// Get the name while preserving native bridge errors.
    pub fn try_name(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_look_get_name(
                self.handle.as_ptr() as *mut c_void
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set the name of this look.
    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        self.set_name_cstr(&cstring(name)?)
    }

    /// Set the name from a C string without allocating an intermediate string.
    pub fn set_name_cstr(&self, name: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_look_set_name(self.handle.as_ptr(), name.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the process space of this look.
    pub fn process_space(&self) -> Option<String> {
        self.try_process_space().ok().flatten()
    }

    /// Get the process space while preserving native bridge errors.
    pub fn try_process_space(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_look_get_process_space(
                self.handle.as_ptr() as *mut c_void
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set the process space of this look.
    pub fn set_process_space(&self, space: impl AsRef<str>) -> Result<()> {
        self.set_process_space_cstr(&cstring(space)?)
    }

    /// Set the process space without allocating an intermediate C string.
    pub fn set_process_space_cstr(&self, space: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_look_set_process_space(self.handle.as_ptr(), space.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Get the description of this look.
    pub fn description(&self) -> Option<String> {
        self.try_description().ok().flatten()
    }

    /// Get the description while preserving native bridge errors.
    pub fn try_description(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_look_get_description(
                self.handle.as_ptr() as *mut c_void
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set the description of this look.
    pub fn set_description(&self, description: impl AsRef<str>) -> Result<()> {
        self.set_description_cstr(&cstring(description)?)
    }

    /// Set the description without allocating an intermediate C string.
    pub fn set_description_cstr(&self, description: &CStr) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_look_set_description(self.handle.as_ptr(), description.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Set an interchange attribute by name and value.
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
            ocio_sys::ocio_look_set_interchange_attribute(
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
            cstr_to_opt_string(ocio_sys::ocio_look_get_interchange_attribute(
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
        crate::colorspace::interchange_attributes_snapshot(self.handle.as_ptr(), false, Some(0))
    }

    /// Get an editable copy of the forward transform. A copied group still
    /// shares its children; see [`crate::transform::GroupTransform::create_deep_copy`].
    pub fn transform(&self) -> Option<Transform> {
        self.try_transform().ok().flatten()
    }

    /// Copy the forward transform, preserving OCIO query failures.
    pub fn try_transform(&self) -> Result<Option<Transform>> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_look_get_transform(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()?;
        Ok(transform_from_raw_handle(handle))
    }

    /// Set the forward transform for this look (panics on error).
    pub fn set_transform(&self, transform: &impl TransformHandle) {
        self.try_set_transform(transform)
            .expect("failed to set look transform");
    }

    /// Try to set the forward transform used by this look.
    pub fn try_set_transform(&self, transform: &impl TransformHandle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_look_set_transform(
                self.handle.as_ptr(),
                transform.as_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    /// Set the forward transform from a shared read-only handle. OCIO copies it.
    pub fn try_set_const_transform(
        &self,
        transform: &crate::transform::ConstTransform,
    ) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_item_set_const_transform(
                self.handle.as_ptr(),
                0,
                transform.handle.as_ptr(),
                0,
            )
        };
        crate::ocio_call_status()
    }

    /// Get an editable copy of the inverse transform. A copied group still
    /// shares its children; see [`crate::transform::GroupTransform::create_deep_copy`].
    pub fn inverse_transform(&self) -> Option<Transform> {
        self.try_inverse_transform().ok().flatten()
    }

    /// Copy the inverse transform, preserving OCIO query failures.
    pub fn try_inverse_transform(&self) -> Result<Option<Transform>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_look_get_inverse_transform(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        Ok(transform_from_raw_handle(handle))
    }

    /// Set the inverse transform for this look (panics on error).
    pub fn set_inverse_transform(&self, transform: &impl TransformHandle) {
        self.try_set_inverse_transform(transform)
            .expect("failed to set look inverse transform");
    }

    /// Try to set the inverse transform used by this look.
    pub fn try_set_inverse_transform(&self, transform: &impl TransformHandle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_look_set_inverse_transform(
                self.handle.as_ptr(),
                transform.as_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    /// Set the inverse transform from a shared read-only handle. OCIO copies it.
    pub fn try_set_const_inverse_transform(
        &self,
        transform: &crate::transform::ConstTransform,
    ) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_item_set_const_transform(
                self.handle.as_ptr(),
                1,
                transform.handle.as_ptr(),
                0,
            )
        };
        crate::ocio_call_status()
    }
}

impl Drop for Look {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_look_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_look() {
        let look = Look::create();
        assert!(look.is_ok());
    }

    #[test]
    fn look_methods_no_crash() {
        let look = Look::create().unwrap();
        let _ = look.name();
        let _ = look.process_space();
    }

    #[test]
    fn set_name() {
        let look = Look::create().unwrap();
        assert!(look.set_name("MyLook").is_ok());
    }

    #[test]
    fn create_editable_copy_round_trip() {
        let look = Look::create().unwrap();
        assert!(look.set_name("EditableLook").is_ok());
        let copy = look.create_editable_copy().unwrap();
        if !crate::is_stub_build() {
            assert_eq!(copy.name().as_deref(), Some("EditableLook"));
        }
    }

    #[cfg(feature = "v2_5")]
    #[test]
    fn interchange_attribute_no_crash() {
        let look = Look::create().unwrap();
        assert!(look
            .set_interchange_attribute("amf_transform_ids", "urn:ampas:aces:transformId:v1.5:Look")
            .is_ok());
        let _ = look.interchange_attribute("amf_transform_ids");
        let _ = look.interchange_attributes();
    }

    #[cfg(feature = "v2_5")]
    #[test]
    #[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
    fn interchange_attribute_real_round_trip() {
        if crate::is_stub_build() {
            return;
        }

        let look = Look::create().unwrap();
        look.set_interchange_attribute("amf_transform_ids", "urn:test:look")
            .unwrap();
        assert_eq!(
            look.interchange_attribute("amf_transform_ids").as_deref(),
            Some("urn:test:look")
        );
        assert_eq!(
            look.interchange_attributes()
                .get("amf_transform_ids")
                .map(String::as_str),
            Some("urn:test:look")
        );
    }

    #[test]
    fn transform_no_crash() {
        let look = Look::create().unwrap();
        let _ = look.transform();
    }

    #[test]
    fn set_transform_no_crash() {
        let look = Look::create().unwrap();
        let ft = crate::transform::FileTransform::create().unwrap();
        look.set_transform(&ft);
    }

    #[test]
    fn inverse_transform_no_crash() {
        let look = Look::create().unwrap();
        let _ = look.inverse_transform();
    }

    #[test]
    fn set_inverse_transform_no_crash() {
        let look = Look::create().unwrap();
        let ft = crate::transform::FileTransform::create().unwrap();
        look.set_inverse_transform(&ft);
    }
}
