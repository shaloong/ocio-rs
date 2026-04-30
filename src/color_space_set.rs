use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, ColorSpace, OcioError, Result};

pub struct ColorSpaceSet {
    pub(crate) handle: NonNull<c_void>,
}

impl ColorSpaceSet {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_color_space_set_create() };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle =
            unsafe { ocio_sys::ocio_color_space_set_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn num_color_spaces(&self) -> i32 {
        unsafe { ocio_sys::ocio_color_space_set_get_num_color_spaces(self.handle.as_ptr()) }
    }

    pub fn color_space_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(
                ocio_sys::ocio_color_space_set_get_color_space_name_by_index(
                    self.handle.as_ptr(),
                    index,
                ),
            )
        }
    }

    pub fn color_space_by_index(&self, index: i32) -> Option<ColorSpace> {
        let handle = unsafe {
            ocio_sys::ocio_color_space_set_get_color_space_by_index(self.handle.as_ptr(), index)
        };
        NonNull::new(handle).map(|h| ColorSpace { handle: h })
    }

    pub fn color_space(&self, name: impl AsRef<str>) -> Option<ColorSpace> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_color_space_set_get_color_space(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| ColorSpace { handle: h })
    }

    pub fn color_space_index(&self, name: impl AsRef<str>) -> i32 {
        let n = match cstring(name) {
            Ok(n) => n,
            Err(_) => return -1,
        };
        unsafe {
            ocio_sys::ocio_color_space_set_get_color_space_index(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
            )
        }
    }

    pub fn has_color_space(&self, name: impl AsRef<str>) -> bool {
        let n = match cstring(name) {
            Ok(n) => n,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_color_space_set_has_color_space(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
            )
        }
    }
}

impl Drop for ColorSpaceSet {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_color_space_set_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_color_space_set() {
        let set = ColorSpaceSet::create();
        assert!(set.is_ok());
    }

    #[test]
    fn color_space_set_methods_no_crash() {
        let set = ColorSpaceSet::create().unwrap();
        let _ = set.num_color_spaces();
        let _ = set.color_space_name_by_index(0);
        let _ = set.color_space_by_index(0);
        let _ = set.color_space("raw");
        let _ = set.color_space_index("raw");
        let _ = set.has_color_space("raw");
    }

    #[test]
    fn get_color_space_by_name_no_crash() {
        let set = ColorSpaceSet::create().unwrap();
        let cs = set.color_space("raw");
        if let Some(cs) = cs {
            let _ = cs.name();
        }
    }

    #[test]
    fn get_color_space_by_index_no_crash() {
        let set = ColorSpaceSet::create().unwrap();
        let cs = set.color_space_by_index(0);
        if let Some(cs) = cs {
            let _ = cs.name();
        }
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let set = ColorSpaceSet::create().unwrap();
        let _ = set.create_editable_copy();
    }

    #[test]
    fn color_space_index_negative_on_missing() {
        let set = ColorSpaceSet::create().unwrap();
        let idx = set.color_space_index("nonexistent_color_space_xyz");
        // In non-stub mode this returns -1 for missing
        // In stub mode it also returns -1
        assert_eq!(idx, -1);
    }
}
