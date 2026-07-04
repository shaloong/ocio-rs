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
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_look_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_look_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    pub fn name(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_look_get_name(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe { ocio_sys::ocio_look_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        Ok(())
    }

    pub fn process_space(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_look_get_process_space(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    pub fn set_process_space(&self, space: impl AsRef<str>) -> Result<()> {
        let s = cstring(space)?;
        unsafe { ocio_sys::ocio_look_set_process_space(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_look_get_description(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    pub fn set_description(&self, description: impl AsRef<str>) -> Result<()> {
        let d = cstring(description)?;
        unsafe { ocio_sys::ocio_look_set_description(self.handle.as_ptr(), d.as_ptr().cast()) };
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
            ocio_sys::ocio_look_set_interchange_attribute(
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
            cstr_to_opt_string(ocio_sys::ocio_look_get_interchange_attribute(
                self.handle.as_ptr(),
                name.as_ptr(),
            ))
        }
    }

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

    pub fn transform(&self) -> Option<Transform> {
        let handle =
            unsafe { ocio_sys::ocio_look_get_transform(self.handle.as_ptr() as *mut c_void) };
        transform_from_raw_handle(handle)
    }

    pub fn set_transform(&self, transform: &impl TransformHandle) {
        unsafe {
            ocio_sys::ocio_look_set_transform(
                self.handle.as_ptr(),
                transform.as_ptr() as *mut c_void,
            );
        }
    }

    pub fn inverse_transform(&self) -> Option<Transform> {
        let handle = unsafe {
            ocio_sys::ocio_look_get_inverse_transform(self.handle.as_ptr() as *mut c_void)
        };
        transform_from_raw_handle(handle)
    }

    pub fn set_inverse_transform(&self, transform: &impl TransformHandle) {
        unsafe {
            ocio_sys::ocio_look_set_inverse_transform(
                self.handle.as_ptr(),
                transform.as_ptr() as *mut c_void,
            );
        }
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

    #[test]
    fn interchange_attribute_no_crash() {
        let look = Look::create().unwrap();
        assert!(look
            .set_interchange_attribute("amf_transform_ids", "urn:ampas:aces:transformId:v1.5:Look")
            .is_ok());
        let _ = look.interchange_attribute("amf_transform_ids");
        let _ = look.interchange_attributes();
    }

    #[test]
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
