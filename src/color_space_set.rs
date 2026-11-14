use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstr_to_opt_string, cstring, ColorSpace, OcioError, Result};
use ocio_sys;

/// Mutable collection of [`ColorSpace`] handles used by OCIO set-style APIs.
pub struct ColorSpaceSet {
    pub(crate) handle: NonNull<c_void>,
}

/// Read-only native result of a color-space set operation.
///
/// The result owns OCIO's original const set, with no additional editable copy.
pub struct ConstColorSpaceSet {
    handle: NonNull<c_void>,
}

/// A borrowed editable or read-only color-space set.
#[derive(Clone, Copy)]
pub enum ColorSpaceSetRef<'a> {
    Editable(&'a ColorSpaceSet),
    ReadOnly(&'a ConstColorSpaceSet),
}

impl<'a> From<&'a ColorSpaceSet> for ColorSpaceSetRef<'a> {
    fn from(value: &'a ColorSpaceSet) -> Self {
        Self::Editable(value)
    }
}
impl<'a> From<&'a ConstColorSpaceSet> for ColorSpaceSetRef<'a> {
    fn from(value: &'a ConstColorSpaceSet) -> Self {
        Self::ReadOnly(value)
    }
}
impl ColorSpaceSetRef<'_> {
    fn raw(self) -> (*mut c_void, bool) {
        match self {
            Self::Editable(value) => (value.handle.as_ptr(), false),
            Self::ReadOnly(value) => (value.handle.as_ptr(), true),
        }
    }

    fn operation(self, other: Self, operation: i32) -> Result<ConstColorSpaceSet> {
        let (left, left_const) = self.raw();
        let (right, right_const) = other.raw();
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_color_space_set_operation(
                left,
                left_const,
                right,
                right_const,
                operation,
            )
        };
        crate::handle_result(handle).map(|handle| ConstColorSpaceSet { handle })
    }
    fn equals(self, other: Self) -> Result<bool> {
        let (left, left_const) = self.raw();
        let (right, right_const) = other.raw();
        crate::clear_last_error();
        let equal =
            unsafe { ocio_sys::ocio_color_space_set_equal(left, left_const, right, right_const) };
        crate::ocio_call_status()?;
        Ok(equal)
    }
}

macro_rules! set_operations {
    () => {
        /// Compare names using OCIO's set equality semantics, not deep color-space equality.
        pub fn try_equals<'a>(&self, other: impl Into<ColorSpaceSetRef<'a>>) -> Result<bool> {
            ColorSpaceSetRef::from(self).equals(other.into())
        }
        /// Return OCIO's union as an independently owned read-only set.
        pub fn union<'a>(
            &self,
            other: impl Into<ColorSpaceSetRef<'a>>,
        ) -> Result<ConstColorSpaceSet> {
            ColorSpaceSetRef::from(self).operation(other.into(), 0)
        }
        /// Return OCIO's intersection as an independently owned read-only set.
        pub fn intersection<'a>(
            &self,
            other: impl Into<ColorSpaceSetRef<'a>>,
        ) -> Result<ConstColorSpaceSet> {
            ColorSpaceSetRef::from(self).operation(other.into(), 1)
        }
        /// Return OCIO's set difference as an independently owned read-only set.
        pub fn difference<'a>(
            &self,
            other: impl Into<ColorSpaceSetRef<'a>>,
        ) -> Result<ConstColorSpaceSet> {
            ColorSpaceSetRef::from(self).operation(other.into(), 2)
        }
    };
}

impl ConstColorSpaceSet {
    set_operations!();

    /// Return the number of color spaces, preserving native errors.
    pub fn try_num_color_spaces(&self) -> Result<i32> {
        crate::clear_last_error();
        let count = unsafe {
            ocio_sys::ocio_const_color_space_set_get_num_color_spaces(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(count)
    }
    /// Copy the name at `index`, or return `None` for an invalid index.
    pub fn try_color_space_name_by_index(&self, index: i32) -> Result<Option<String>> {
        crate::clear_last_error();
        let name = unsafe {
            cstr_to_opt_string(
                ocio_sys::ocio_const_color_space_set_get_color_space_name_by_index(
                    self.handle.as_ptr(),
                    index,
                ),
            )
        };
        crate::ocio_call_status()?;
        Ok(name)
    }
    /// Copy a named color space into an independent editable object.
    pub fn copy_color_space(&self, name: impl AsRef<str>) -> Result<Option<ColorSpace>> {
        let name = cstring(name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_const_color_space_set_copy_color_space(
                self.handle.as_ptr(),
                name.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| ColorSpace { handle }))
    }
    /// Copy an indexed color space into an independent editable object.
    pub fn copy_color_space_by_index(&self, index: i32) -> Result<Option<ColorSpace>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_const_color_space_set_copy_color_space_by_index(
                self.handle.as_ptr(),
                index,
            )
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| ColorSpace { handle }))
    }
    /// Create an editable copy on explicit request.
    pub fn create_editable_copy(&self) -> Result<ColorSpaceSet> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_const_color_space_set_create_editable_copy(self.handle.as_ptr())
        };
        crate::handle_result(handle).map(|handle| ColorSpaceSet { handle })
    }
}
impl Drop for ConstColorSpaceSet {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_const_color_space_set_destroy(self.handle.as_ptr()) }
    }
}

impl ColorSpaceSet {
    /// Compare the sets by color-space names, as OCIO does.
    pub fn try_equals<'a>(&self, other: impl Into<ColorSpaceSetRef<'a>>) -> Result<bool> {
        ColorSpaceSetRef::from(self).equals(other.into())
    }
    /// Return OCIO's union as an independently owned read-only set.
    pub fn union<'a>(&self, other: impl Into<ColorSpaceSetRef<'a>>) -> Result<ConstColorSpaceSet> {
        ColorSpaceSetRef::from(self).operation(other.into(), 0)
    }
    /// Return OCIO's intersection as an independently owned read-only set.
    pub fn intersection<'a>(
        &self,
        other: impl Into<ColorSpaceSetRef<'a>>,
    ) -> Result<ConstColorSpaceSet> {
        ColorSpaceSetRef::from(self).operation(other.into(), 1)
    }
    /// Return OCIO's difference as an independently owned read-only set.
    pub fn difference<'a>(
        &self,
        other: impl Into<ColorSpaceSetRef<'a>>,
    ) -> Result<ConstColorSpaceSet> {
        ColorSpaceSetRef::from(self).operation(other.into(), 2)
    }
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
        self.try_color_space_name_by_index(index).ok().flatten()
    }

    /// Return a color-space name by index while preserving bridge failures.
    pub fn try_color_space_name_by_index(&self, index: i32) -> Result<Option<String>> {
        crate::clear_last_error();
        let name = unsafe {
            cstr_from_mut(
                ocio_sys::ocio_color_space_set_get_color_space_name_by_index(
                    self.handle.as_ptr(),
                    index,
                ),
            )
        };
        crate::ocio_call_status()?;
        Ok(name)
    }

    /// Return an independent editable copy of a color space by index.
    pub fn color_space_by_index(&self, index: i32) -> Option<ColorSpace> {
        self.try_color_space_by_index(index).ok().flatten()
    }

    /// Copy a color space by index while preserving bridge failures.
    pub fn try_color_space_by_index(&self, index: i32) -> Result<Option<ColorSpace>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_color_space_set_get_color_space_by_index(self.handle.as_ptr(), index)
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|h| ColorSpace { handle: h }))
    }

    /// Look up an independent editable color-space copy by name.
    pub fn color_space(&self, name: impl AsRef<str>) -> Option<ColorSpace> {
        self.try_color_space(name).ok().flatten()
    }

    /// Look up a shared read-only color space without copying the native object.
    pub fn try_const_color_space(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Option<crate::ConstColorSpace>> {
        let name = cstring(name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_color_space_set_get_const_color_space(
                self.handle.as_ptr(),
                name.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| crate::ConstColorSpace { handle }))
    }

    /// Copy a color space by name while preserving bridge failures.
    pub fn try_color_space(&self, name: impl AsRef<str>) -> Result<Option<ColorSpace>> {
        let n = cstring(name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_color_space_set_get_color_space(self.handle.as_ptr(), n.as_ptr().cast())
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|h| ColorSpace { handle: h }))
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
        self.try_remove_color_spaces(other)
            .expect("failed to remove color spaces from set");
    }

    /// Remove every color space found in `other` and surface any OCIO validation error.
    pub fn try_remove_color_spaces(&self, other: &ColorSpaceSet) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_remove_color_spaces(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Remove every color space from the set.
    pub fn clear_color_spaces(&self) {
        self.try_clear_color_spaces()
            .expect("failed to clear color spaces from set");
    }

    /// Remove every color space and surface any OCIO validation error.
    pub fn try_clear_color_spaces(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_set_clear_color_spaces(self.handle.as_ptr()) };
        crate::ocio_call_status()
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
        set.try_remove_color_spaces(&other).unwrap();
        set.try_clear_color_spaces().unwrap();
    }
}
