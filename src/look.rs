use std::collections::BTreeMap;
use std::ffi::c_void;
use std::ptr::NonNull;

use crate::transform::{transform_from_raw_handle, Transform, TransformHandle};
use crate::{cstr_from_mut, cstr_to_opt_string, cstring, OcioError, Result};
use ocio_sys;

/// Wraps an OCIO look definition with forward and inverse transform slots.
pub struct Look {
    pub(crate) handle: NonNull<c_void>,
}

impl Look {
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
        unsafe {
            cstr_from_mut(ocio_sys::ocio_look_get_name(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    /// Set the name of this look.
    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_look_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the process space of this look.
    pub fn process_space(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_look_get_process_space(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    /// Set the process space of this look.
    pub fn set_process_space(&self, space: impl AsRef<str>) -> Result<()> {
        let s = cstring(space)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_look_set_process_space(self.handle.as_ptr(), s.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the description of this look.
    pub fn description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_look_get_description(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    /// Set the description of this look.
    pub fn set_description(&self, description: impl AsRef<str>) -> Result<()> {
        let d = cstring(description)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_look_set_description(self.handle.as_ptr(), d.as_ptr().cast()) };
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
        let mut attrs = BTreeMap::new();
        let count =
            unsafe { ocio_sys::ocio_look_get_num_interchange_attributes(self.handle.as_ptr()) };
        for index in 0..count {
            let name = unsafe {
                cstr_to_opt_string(ocio_sys::ocio_look_get_interchange_attribute_name_by_index(
                    self.handle.as_ptr(),
                    index,
                ))
            };
            let value = unsafe {
                cstr_to_opt_string(
                    ocio_sys::ocio_look_get_interchange_attribute_value_by_index(
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

    /// Get the forward transform for this look.
    pub fn transform(&self) -> Option<Transform> {
        self.try_transform().ok().flatten()
    }

    /// Get the forward transform for this look, preserving OCIO query failures.
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

    /// Get the inverse transform for this look.
    pub fn inverse_transform(&self) -> Option<Transform> {
        self.try_inverse_transform().ok().flatten()
    }

    /// Get the inverse transform for this look, preserving OCIO query failures.
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
