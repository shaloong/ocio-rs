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
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_named_transform_create() };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_named_transform_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn name(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_name(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe { ocio_sys::ocio_named_transform_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        Ok(())
    }

    pub fn family(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_family(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn set_family(&self, family: impl AsRef<str>) -> Result<()> {
        let f = cstring(family)?;
        unsafe {
            ocio_sys::ocio_named_transform_set_family(self.handle.as_ptr(), f.as_ptr().cast())
        };
        Ok(())
    }

    pub fn description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_description(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn set_description(&self, description: impl AsRef<str>) -> Result<()> {
        let d = cstring(description)?;
        unsafe {
            ocio_sys::ocio_named_transform_set_description(self.handle.as_ptr(), d.as_ptr().cast())
        };
        Ok(())
    }

    pub fn encoding(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_encoding(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn set_encoding(&self, encoding: impl AsRef<str>) -> Result<()> {
        let e = cstring(encoding)?;
        unsafe {
            ocio_sys::ocio_named_transform_set_encoding(self.handle.as_ptr(), e.as_ptr().cast())
        };
        Ok(())
    }

    pub fn num_aliases(&self) -> i32 {
        unsafe { ocio_sys::ocio_named_transform_get_num_aliases(self.handle.as_ptr()) as i32 }
    }

    pub fn alias(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_alias(
                self.handle.as_ptr(),
                index as usize,
            ))
        }
    }

    pub fn add_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        unsafe {
            ocio_sys::ocio_named_transform_add_alias(self.handle.as_ptr(), a.as_ptr().cast())
        };
        Ok(())
    }

    pub fn remove_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        unsafe {
            ocio_sys::ocio_named_transform_remove_alias(self.handle.as_ptr(), a.as_ptr().cast())
        };
        Ok(())
    }

    pub fn has_alias(&self, alias: impl AsRef<str>) -> bool {
        let a = match cstring(alias) {
            Ok(a) => a,
            Err(_) => return false,
        };
        unsafe { ocio_sys::ocio_named_transform_has_alias(self.handle.as_ptr(), a.as_ptr().cast()) }
    }

    pub fn clear_aliases(&self) {
        unsafe {
            ocio_sys::ocio_named_transform_clear_aliases(self.handle.as_ptr() as *mut c_void)
        };
    }

    pub fn num_categories(&self) -> i32 {
        unsafe { ocio_sys::ocio_named_transform_get_num_categories(self.handle.as_ptr()) }
    }

    pub fn category(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_named_transform_get_category(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    pub fn has_category(&self, category: impl AsRef<str>) -> bool {
        let c = match cstring(category) {
            Ok(c) => c,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_named_transform_has_category(self.handle.as_ptr(), c.as_ptr().cast())
        }
    }

    pub fn add_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        unsafe {
            ocio_sys::ocio_named_transform_add_category(self.handle.as_ptr(), c.as_ptr().cast())
        };
        Ok(())
    }

    pub fn remove_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        unsafe {
            ocio_sys::ocio_named_transform_remove_category(self.handle.as_ptr(), c.as_ptr().cast())
        };
        Ok(())
    }

    pub fn clear_categories(&self) {
        unsafe { ocio_sys::ocio_named_transform_clear_categories(self.handle.as_ptr()) };
    }

    pub fn transform(&self, direction: TransformDirection) -> Option<Transform> {
        let handle = unsafe {
            ocio_sys::ocio_named_transform_get_transform(self.handle.as_ptr(), direction as i32)
        };
        transform_from_raw_handle(handle)
    }

    pub fn set_transform(&self, transform: &impl TransformHandle, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_named_transform_set_transform(
                self.handle.as_ptr(),
                transform.as_ptr() as *mut c_void,
                direction as i32,
            );
        }
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
        nt.clear_aliases();
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
        nt.clear_categories();
    }
}
