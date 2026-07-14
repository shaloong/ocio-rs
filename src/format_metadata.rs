use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstr_to_opt_string, cstring, Result};
use ocio_sys;

/// Structured metadata attached to transforms and baked outputs.
pub struct FormatMetadata {
    pub(crate) handle: NonNull<c_void>,
}

impl FormatMetadata {
    /// Get the element name.
    pub fn element_name(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_format_metadata_get_element_name(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the element name.
    pub fn set_element_name(&self, name: impl AsRef<str>) -> Result<()> {
        let name = cstring(name)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_format_metadata_set_element_name(
                self.handle.as_ptr(),
                name.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Get the element value.
    pub fn element_value(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_format_metadata_get_element_value(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the element value.
    pub fn set_element_value(&self, value: impl AsRef<str>) -> Result<()> {
        let v = cstring(value)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_format_metadata_set_element_value(
                self.handle.as_ptr(),
                v.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Get the number of attributes on this metadata element.
    pub fn num_attributes(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_format_metadata_get_num_attributes(self.handle.as_ptr() as *mut c_void)
        }
    }

    /// Get the attribute name at the given index.
    pub fn attribute_name(&self, i: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_format_metadata_get_attribute_name(
                self.handle.as_ptr(),
                i,
            ))
        }
    }

    /// Get the attribute value at the given index.
    pub fn attribute_value_by_index(&self, i: i32) -> Option<String> {
        self.try_attribute_value_by_index(i).ok().flatten()
    }

    /// Get an attribute value by index, preserving OCIO query failures.
    pub fn try_attribute_value_by_index(&self, i: i32) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_format_metadata_get_attribute_value_by_index(
                self.handle.as_ptr(),
                i,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Get the attribute value by name.
    pub fn attribute_value(&self, name: impl AsRef<str>) -> Option<String> {
        self.try_attribute_value(name).ok().flatten()
    }

    /// Get an attribute value by name, preserving invalid input and OCIO failures.
    pub fn try_attribute_value(&self, name: impl AsRef<str>) -> Result<Option<String>> {
        let n = cstring(name)?;
        crate::clear_last_error();
        let value = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_format_metadata_get_attribute_value(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Add a name-value attribute to this metadata element.
    pub fn add_attribute(&self, name: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        let v = cstring(value)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_format_metadata_add_attribute(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
                v.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Get the number of child elements.
    pub fn num_children(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_format_metadata_get_num_children_elements(
                self.handle.as_ptr() as *mut c_void
            )
        }
    }

    /// Get a live child element at the given index.
    ///
    /// The returned wrapper remains usable when siblings are appended to this
    /// metadata object. OCIO invalidates child references when the parent is
    /// cleared, so discard child wrappers before calling [`Self::clear`] on
    /// their parent.
    pub fn child_element(&self, i: i32) -> Option<FormatMetadata> {
        self.try_child_element(i).ok().flatten()
    }

    /// Get a live child element by index, preserving OCIO query failures.
    ///
    /// See [`Self::child_element`] for the lifetime rule around
    /// [`Self::clear`].
    pub fn try_child_element(&self, i: i32) -> Result<Option<FormatMetadata>> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_format_metadata_get_child_element(self.handle.as_ptr(), i) };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| FormatMetadata { handle }))
    }

    /// Add a child element with the given name and value.
    pub fn add_child_element(&self, name: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        let v = cstring(value)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_format_metadata_add_child_element(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
                v.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Clear all data on this metadata element.
    pub fn clear(&self) {
        unsafe { ocio_sys::ocio_format_metadata_clear(self.handle.as_ptr() as *mut c_void) };
    }

    /// Get the name of this metadata element.
    pub fn name(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_format_metadata_get_name(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the name of this metadata element.
    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_format_metadata_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the ID of this metadata element.
    pub fn id(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_format_metadata_get_id(self.handle.as_ptr())) }
    }

    /// Set the ID of this metadata element.
    pub fn set_id(&self, id: impl AsRef<str>) -> Result<()> {
        let i = cstring(id)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_format_metadata_set_id(self.handle.as_ptr(), i.as_ptr().cast()) };
        crate::ocio_call_status()
    }
}

impl Drop for FormatMetadata {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_format_metadata_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

#[cfg(test)]
mod tests {
    use crate::Baker;

    #[test]
    fn format_metadata_from_baker_no_crash() {
        let baker = Baker::create().unwrap();
        let md = baker.format_metadata();
        if let Some(md) = md {
            let _ = md.element_name();
            let _ = md.element_value();
            let _ = md.num_attributes();
            let _ = md.attribute_name(0);
            let _ = md.attribute_value_by_index(0);
            let _ = md.attribute_value("key");
            let _ = md.num_children();
            let _ = md.child_element(0);
            let _ = md.name();
            let _ = md.id();
        }
    }

    #[test]
    fn format_metadata_setters_no_crash() {
        let baker = Baker::create().unwrap();
        let md = baker.format_metadata();
        if let Some(md) = md {
            let _ = md.set_element_name("TestElement");
            let _ = md.set_element_value("TestValue");
            let _ = md.add_attribute("attr1", "val1");
            let _ = md.add_child_element("child", "value");
            md.clear();
            let _ = md.set_name("TestName");
            let _ = md.set_id("TestID");
        }
    }

    #[test]
    fn format_metadata_attribute_by_name_no_crash() {
        let baker = Baker::create().unwrap();
        let md = baker.format_metadata();
        if let Some(md) = md {
            let _ = md.set_element_name("elem");
            let _ = md.attribute_value("nonexistent_key");
        }
    }

    #[test]
    fn format_metadata_child_element_no_crash() {
        let baker = Baker::create().unwrap();
        let md = baker.format_metadata();
        if let Some(md) = md {
            let child = md.child_element(0);
            if let Some(child) = child {
                let _ = child.element_name();
                let _ = child.element_value();
            }
        }
    }

    #[test]
    fn format_metadata_name_id_no_crash() {
        let baker = Baker::create().unwrap();
        let md = baker.format_metadata();
        if let Some(md) = md {
            let _ = md.name();
            let _ = md.id();
            let _ = md.set_name("MyName");
            let _ = md.set_id("MyID");
        }
    }
}
