use std::ffi::c_void;
use std::ptr::NonNull;

use crate::transform::{transform_from_raw_handle, Transform, TransformHandle};
use crate::{cstr_from_mut, cstring, OcioError, Result, TransformDirection};
use ocio_sys;

/// Names a reusable transform pair inside a config for interchange and lookup.
pub struct NamedTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl NamedTransform {
    /// Create a new default named transform.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_named_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an editable copy of this named transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_named_transform_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Get the name of this named transform.
    pub fn name(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_name(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Set the name of this named transform.
    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_named_transform_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the family of this named transform.
    pub fn family(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_family(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Set the family of this named transform.
    pub fn set_family(&self, family: impl AsRef<str>) -> Result<()> {
        let f = cstring(family)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_transform_set_family(self.handle.as_ptr(), f.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Get the description of this named transform.
    pub fn description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_description(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Set the description of this named transform.
    pub fn set_description(&self, description: impl AsRef<str>) -> Result<()> {
        let d = cstring(description)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_transform_set_description(self.handle.as_ptr(), d.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Get the encoding of this named transform.
    pub fn encoding(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_encoding(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Set the encoding of this named transform.
    pub fn set_encoding(&self, encoding: impl AsRef<str>) -> Result<()> {
        let e = cstring(encoding)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_transform_set_encoding(self.handle.as_ptr(), e.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Get the number of aliases for this named transform.
    pub fn num_aliases(&self) -> i32 {
        unsafe { ocio_sys::ocio_named_transform_get_num_aliases(self.handle.as_ptr()) as i32 }
    }

    /// Get the alias at the given index.
    pub fn alias(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_alias(
                self.handle.as_ptr(),
                index as usize,
            ))
        }
    }

    /// Add an alias for this named transform.
    pub fn add_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_transform_add_alias(self.handle.as_ptr(), a.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Remove an alias from this named transform.
    pub fn remove_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_transform_remove_alias(self.handle.as_ptr(), a.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Returns whether this named transform has the given alias.
    pub fn has_alias(&self, alias: impl AsRef<str>) -> bool {
        let a = match cstring(alias) {
            Ok(a) => a,
            Err(_) => return false,
        };
        unsafe { ocio_sys::ocio_named_transform_has_alias(self.handle.as_ptr(), a.as_ptr().cast()) }
    }

    /// Clear all aliases, panicking on error.
    pub fn clear_aliases(&self) {
        self.try_clear_aliases()
            .expect("failed to clear named transform aliases");
    }

    /// Clear all aliases and surface any OCIO validation error.
    pub fn try_clear_aliases(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_transform_clear_aliases(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()
    }

    /// Get the number of categories for this named transform.
    pub fn num_categories(&self) -> i32 {
        unsafe { ocio_sys::ocio_named_transform_get_num_categories(self.handle.as_ptr()) }
    }

    /// Get the category at the given index.
    pub fn category(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_category(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Returns whether this named transform has the given category.
    pub fn has_category(&self, category: impl AsRef<str>) -> bool {
        let c = match cstring(category) {
            Ok(c) => c,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_named_transform_has_category(self.handle.as_ptr(), c.as_ptr().cast())
        }
    }

    /// Add a category to this named transform.
    pub fn add_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_transform_add_category(self.handle.as_ptr(), c.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Remove a category from this named transform.
    pub fn remove_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_transform_remove_category(self.handle.as_ptr(), c.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Clear all categories, panicking on error.
    pub fn clear_categories(&self) {
        self.try_clear_categories()
            .expect("failed to clear named transform categories");
    }

    /// Clear all categories and surface any OCIO validation error.
    pub fn try_clear_categories(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_named_transform_clear_categories(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    /// Get the transform for the given direction.
    pub fn transform(&self, direction: TransformDirection) -> Option<Transform> {
        let handle = unsafe {
            ocio_sys::ocio_named_transform_get_transform(self.handle.as_ptr(), direction as i32)
        };
        transform_from_raw_handle(handle)
    }

    /// Set the transform for the given direction.
    pub fn set_transform(&self, transform: &impl TransformHandle, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_named_transform_set_transform(
                self.handle.as_ptr(),
                transform.as_ptr() as *mut c_void,
                direction as i32,
            );
        }
    }

    /// Try to attach a transform for the given direction.
    pub fn try_set_transform(
        &self,
        transform: &impl TransformHandle,
        direction: TransformDirection,
    ) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_named_transform_set_transform(
                self.handle.as_ptr(),
                transform.as_ptr() as *mut c_void,
                direction as i32,
            );
        }
        crate::ocio_call_status()
    }
}

impl Drop for NamedTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_named_transform_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::FileTransform;

    #[test]
    fn create_named_transform() {
        let nt = NamedTransform::create();
        assert!(nt.is_ok());
    }

    #[test]
    fn named_transform_methods_no_crash() {
        let nt = NamedTransform::create().unwrap();
        let _ = nt.name();
        let _ = nt.family();
        let _ = nt.description();
        let _ = nt.encoding();
    }

    #[test]
    fn set_name_no_crash() {
        let nt = NamedTransform::create().unwrap();
        assert!(nt.set_name("MyNamedTransform").is_ok());
    }

    #[test]
    fn set_family_no_crash() {
        let nt = NamedTransform::create().unwrap();
        assert!(nt.set_family("TestFamily").is_ok());
    }

    #[test]
    fn set_description_no_crash() {
        let nt = NamedTransform::create().unwrap();
        assert!(nt.set_description("Test description").is_ok());
    }

    #[test]
    fn set_encoding_no_crash() {
        let nt = NamedTransform::create().unwrap();
        assert!(nt.set_encoding("scene-linear").is_ok());
    }

    #[test]
    fn transform_no_crash() {
        let nt = NamedTransform::create().unwrap();
        let _ = nt.transform(TransformDirection::Forward);
        let _ = nt.transform(TransformDirection::Inverse);
    }

    #[test]
    fn set_transform_no_crash() {
        let nt = NamedTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        nt.set_transform(&ft, TransformDirection::Forward);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let nt = NamedTransform::create().unwrap();
        let _ = nt.create_editable_copy();
    }

    #[test]
    fn aliases_no_crash() {
        let nt = NamedTransform::create().unwrap();
        let _ = nt.num_aliases();
        let _ = nt.alias(0);
        assert!(!nt.has_alias("test_alias"));
        assert!(nt.add_alias("test_alias").is_ok());
        let _ = nt.has_alias("test_alias");
        assert!(nt.remove_alias("test_alias").is_ok());
        nt.try_clear_aliases().unwrap();
    }

    #[test]
    fn category_no_crash() {
        let nt = NamedTransform::create().unwrap();
        let _ = nt.num_categories();
        let _ = nt.category(0);
        let _ = nt.has_category("test_category");
        assert!(nt.add_category("test_category").is_ok());
        let _ = nt.has_category("test_category");
        assert!(nt.remove_category("test_category").is_ok());
        nt.try_clear_categories().unwrap();
    }
}
