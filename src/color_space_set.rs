use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstr_to_opt_string, cstring, ColorSpace, OcioError, Result};
use ocio_sys;

/// Mutable collection of [`ColorSpace`] handles used by OCIO set-style APIs.
pub struct ColorSpaceSet {
    pub(crate) handle: NonNull<c_void>,
}

impl ColorSpaceSet {
    /// Create an empty color-space set.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_color_space_set_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an editable copy that is independent from the original set.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_color_space_set_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the number of color spaces currently in the set.
    pub fn num_color_spaces(&self) -> i32 {
        unsafe { ocio_sys::ocio_color_space_set_get_num_color_spaces(self.handle.as_ptr()) }
    }

    /// Return one color-space name by index.
    pub fn color_space_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(
                ocio_sys::ocio_color_space_set_get_color_space_name_by_index(
                    self.handle.as_ptr(),
                    index,
                ),
            )
        }
    }

    /// Return one color-space handle by index.
    pub fn color_space_by_index(&self, index: i32) -> Option<ColorSpace> {
        let handle = unsafe {
            ocio_sys::ocio_color_space_set_get_color_space_by_index(self.handle.as_ptr(), index)
        };
        NonNull::new(handle).map(|h| ColorSpace { handle: h })
    }

    /// Look up a color-space handle by name.
    pub fn color_space(&self, name: impl AsRef<str>) -> Option<ColorSpace> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_color_space_set_get_color_space(self.handle.as_ptr(), n.as_ptr().cast())
        };
        NonNull::new(handle).map(|h| ColorSpace { handle: h })
    }

    /// Return the index of a color space by name, or `-1` when missing.
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

    /// Return whether a color space named `name` is present in the set.
    pub fn has_color_space(&self, name: impl AsRef<str>) -> bool {
        let n = match cstring(name) {
            Ok(n) => n,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_color_space_set_has_color_space(self.handle.as_ptr(), n.as_ptr().cast())
        }
    }

    /// Insert one color space into the set.
    pub fn add_color_space(&self, color_space: &ColorSpace) {
        self.try_add_color_space(color_space)
            .expect("failed to add color space to set");
    }

    /// Insert one color space into the set and surface any OCIO validation error.
    pub fn try_add_color_space(&self, color_space: &ColorSpace) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_add_color_space(
                self.handle.as_ptr(),
                color_space.handle.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Insert every color space from `other` into the set.
    pub fn add_color_spaces(&self, other: &ColorSpaceSet) {
        self.try_add_color_spaces(other)
            .expect("failed to add color spaces to set");
    }

    /// Insert every color space from `other` and surface any OCIO validation error.
    pub fn try_add_color_spaces(&self, other: &ColorSpaceSet) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_add_color_spaces(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Remove one color space by name.
    pub fn remove_color_space(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_remove_color_space(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Remove every color space found in `other` from this set.
    pub fn remove_color_spaces(&self, other: &ColorSpaceSet) {
        unsafe {
            ocio_sys::ocio_color_space_set_remove_color_spaces(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            );
        }
    }

    /// Remove every color space from the set.
    pub fn clear_color_spaces(&self) {
        unsafe { ocio_sys::ocio_color_space_set_clear_color_spaces(self.handle.as_ptr()) };
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

    #[test]
    fn mutate_color_space_set_no_crash() {
        let set = ColorSpaceSet::create().unwrap();
        let other = ColorSpaceSet::create().unwrap();
        let cs = ColorSpace::create().unwrap();
        cs.set_name("UnitColorSpace").unwrap();
        set.add_color_space(&cs);
        set.add_color_spaces(&other);
        let _ = set.remove_color_space("raw");
        set.remove_color_spaces(&other);
        set.clear_color_spaces();
    }
}
