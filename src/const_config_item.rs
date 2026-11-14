use crate::transform::ConstTransform;
use crate::{
    Config, Look, NamedTransform, Result, TransformDirection, ViewTransform, ViewTransformDirection,
};
use std::ffi::{c_void, CStr};
use std::ptr::NonNull;

struct Item {
    handle: NonNull<c_void>,
    kind: i32,
}
impl Item {
    fn text(&self, field: i32, index: usize, argument: Option<&CStr>) -> Result<Option<&CStr>> {
        crate::clear_last_error();
        let ptr = unsafe {
            ocio_sys::ocio_const_named_item_text(
                self.handle.as_ptr(),
                self.kind,
                field,
                index,
                argument.map_or(std::ptr::null(), CStr::as_ptr),
            )
        };
        crate::ocio_call_status()?;
        Ok(if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr) })
        })
    }
    fn number(&self, field: i32) -> Result<usize> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_const_named_item_number(self.handle.as_ptr(), self.kind, field)
        };
        crate::ocio_call_status()?;
        Ok(value)
    }
    fn contains(&self, field: i32, value: &CStr) -> Result<bool> {
        crate::clear_last_error();
        let found = unsafe {
            ocio_sys::ocio_const_named_item_contains(
                self.handle.as_ptr(),
                self.kind,
                field,
                value.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(found)
    }
    fn name(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let ptr = unsafe { ocio_sys::ocio_const_named_item_name(self.handle.as_ptr(), self.kind) };
        let name = unsafe { crate::cstr_to_opt_string(ptr) };
        crate::ocio_call_status()?;
        Ok(name)
    }
    fn copy(&self) -> Result<NonNull<c_void>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_const_named_item_editable_copy(self.handle.as_ptr(), self.kind)
        };
        crate::handle_result(handle)
    }
    fn transform(&self, kind: i32, direction: i32) -> Result<Option<ConstTransform>> {
        ConstTransform::from_parent(self.handle.as_ptr(), kind, direction)
    }
}
impl Drop for Item {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_const_named_item_destroy(self.handle.as_ptr(), self.kind) }
    }
}

/// Shared read-only native look. Borrowed text remains valid while this object
/// is alive, provided external native code does not mutate the same object.
pub struct ConstLook(Item);
/// Shared read-only native named transform. Borrowed text remains valid while
/// this object is alive, provided external native code does not mutate it.
pub struct ConstNamedTransform(Item);
/// Shared read-only native view transform. Borrowed text remains valid while
/// this object is alive, provided external native code does not mutate it.
pub struct ConstViewTransform(Item);

macro_rules! accessors {
    ($ty:ident, $editable:ident) => {
        impl $ty {
            /// Copy the native name as owned text.
            pub fn name(&self) -> Result<Option<String>> {
                self.0.name()
            }
            /// Borrow the native name without allocating.
            pub fn name_ref(&self) -> Result<Option<&CStr>> {
                self.0.text(0, 0, None)
            }
            /// Borrow the native description without allocating.
            pub fn description(&self) -> Result<Option<&CStr>> {
                self.0.text(1, 0, None)
            }
            /// Create an independently editable copy.
            pub fn create_editable_copy(&self) -> Result<$editable> {
                Ok($editable {
                    handle: self.0.copy()?,
                })
            }
        }
    };
}
accessors!(ConstLook, Look);
accessors!(ConstNamedTransform, NamedTransform);
accessors!(ConstViewTransform, ViewTransform);

impl ConstLook {
    /// Borrow the process-space name.
    pub fn process_space(&self) -> Result<Option<&CStr>> {
        self.0.text(2, 0, None)
    }
    /// Borrow an interchange attribute; invalid attribute names return an error.
    #[cfg(feature = "v2_5")]
    pub fn interchange_attribute(&self, name: &CStr) -> Result<Option<&CStr>> {
        self.0.text(7, 0, Some(name))
    }
    /// Share the forward transform without copying it.
    pub fn try_const_transform(&self) -> Result<Option<ConstTransform>> {
        self.0.transform(6, 0)
    }
    /// Share the inverse transform without copying it.
    pub fn try_const_inverse_transform(&self) -> Result<Option<ConstTransform>> {
        self.0.transform(7, 0)
    }
}
impl ConstNamedTransform {
    /// Borrow the family string.
    pub fn family(&self) -> Result<Option<&CStr>> {
        self.0.text(3, 0, None)
    }
    /// Borrow the encoding string.
    pub fn encoding(&self) -> Result<Option<&CStr>> {
        self.0.text(4, 0, None)
    }
    /// Number of aliases.
    pub fn num_aliases(&self) -> Result<usize> {
        self.0.number(0)
    }
    /// Borrow an alias; an out-of-range index returns `None`.
    pub fn alias(&self, index: usize) -> Result<Option<&CStr>> {
        self.0.text(5, index, None)
    }
    /// Check an alias without allocating a C string.
    pub fn has_alias(&self, value: &CStr) -> Result<bool> {
        self.0.contains(0, value)
    }
    /// Number of categories.
    pub fn num_categories(&self) -> Result<usize> {
        self.0.number(1)
    }
    /// Borrow a category; an out-of-range index returns `None`.
    pub fn category(&self, index: usize) -> Result<Option<&CStr>> {
        self.0.text(6, index, None)
    }
    /// Check a category without allocating a C string.
    pub fn has_category(&self, value: &CStr) -> Result<bool> {
        self.0.contains(1, value)
    }
    /// Resolve a transform, synthesizing it from the opposite direction if needed.
    /// Returns an error if neither direction is defined.
    pub fn resolve_const_transform(
        &self,
        direction: TransformDirection,
    ) -> Result<Option<ConstTransform>> {
        self.0.transform(10, direction as i32)
    }
    pub(crate) fn raw(&self) -> *mut c_void {
        self.0.handle.as_ptr()
    }
    /// Share the transform in the requested direction.
    pub fn try_const_transform(
        &self,
        direction: TransformDirection,
    ) -> Result<Option<ConstTransform>> {
        self.0.transform(8, direction as i32)
    }
}
impl ConstViewTransform {
    /// Borrow the family string.
    pub fn family(&self) -> Result<Option<&CStr>> {
        self.0.text(3, 0, None)
    }
    /// Number of categories.
    pub fn num_categories(&self) -> Result<usize> {
        self.0.number(1)
    }
    /// Borrow a category; an out-of-range index returns `None`.
    pub fn category(&self, index: usize) -> Result<Option<&CStr>> {
        self.0.text(6, index, None)
    }
    /// Check a category without allocating a C string.
    pub fn has_category(&self, value: &CStr) -> Result<bool> {
        self.0.contains(1, value)
    }
    /// Return the native reference-space type.
    pub fn reference_space_type(&self) -> Result<crate::ReferenceSpaceType> {
        match self.0.number(2)? {
            0 => Ok(crate::ReferenceSpaceType::Scene),
            1 => Ok(crate::ReferenceSpaceType::Display),
            _ => Err(crate::OcioError::Ocio(
                "unknown reference space type".into(),
            )),
        }
    }
    /// Borrow an interchange attribute; invalid attribute names return an error.
    #[cfg(feature = "v2_5")]
    pub fn interchange_attribute(&self, name: &CStr) -> Result<Option<&CStr>> {
        self.0.text(7, 0, Some(name))
    }
    /// Share the transform in the requested direction.
    pub fn try_const_transform(
        &self,
        direction: ViewTransformDirection,
    ) -> Result<Option<ConstTransform>> {
        self.0.transform(9, direction as i32)
    }
}

fn lookup(config: *mut c_void, name: Option<&CStr>, kind: i32) -> Result<Option<Item>> {
    crate::clear_last_error();
    let handle = unsafe {
        ocio_sys::ocio_config_get_const_named_item(
            config,
            name.map_or(std::ptr::null(), CStr::as_ptr),
            kind,
        )
    };
    crate::ocio_call_status()?;
    Ok(NonNull::new(handle).map(|handle| Item {
        handle,
        kind: if kind == 3 { 2 } else { kind },
    }))
}

fn lookup_const(config: *mut c_void, name: Option<&CStr>, kind: i32) -> Result<Option<Item>> {
    crate::clear_last_error();
    let handle = unsafe {
        ocio_sys::ocio_const_config_get_named_item(
            config,
            name.map_or(std::ptr::null(), CStr::as_ptr),
            kind,
        )
    };
    crate::ocio_call_status()?;
    Ok(NonNull::new(handle).map(|handle| Item {
        handle,
        kind: if kind == 3 { 2 } else { kind },
    }))
}

impl crate::ConstConfig {
    /// Share a native look without making an editable copy.
    pub fn try_const_look(&self, name: &CStr) -> Result<Option<ConstLook>> {
        Ok(lookup_const(self.handle.as_ptr(), Some(name), 0)?.map(ConstLook))
    }
    /// Share a native named transform without making an editable copy.
    pub fn try_const_named_transform(&self, name: &CStr) -> Result<Option<ConstNamedTransform>> {
        Ok(lookup_const(self.handle.as_ptr(), Some(name), 1)?.map(ConstNamedTransform))
    }
    /// Share a native view transform without making an editable copy.
    pub fn try_const_view_transform(&self, name: &CStr) -> Result<Option<ConstViewTransform>> {
        Ok(lookup_const(self.handle.as_ptr(), Some(name), 2)?.map(ConstViewTransform))
    }
    /// Share the default scene-to-display view transform.
    pub fn try_const_default_scene_to_display_view_transform(
        &self,
    ) -> Result<Option<ConstViewTransform>> {
        Ok(lookup_const(self.handle.as_ptr(), None, 3)?.map(ConstViewTransform))
    }
}

impl Config {
    /// Share a named native look without making an editable copy.
    pub fn try_const_look(&self, name: &CStr) -> Result<Option<ConstLook>> {
        Ok(lookup(self.handle.as_ptr(), Some(name), 0)?.map(ConstLook))
    }
    /// Share a native named transform without making an editable copy.
    pub fn try_const_named_transform(&self, name: &CStr) -> Result<Option<ConstNamedTransform>> {
        Ok(lookup(self.handle.as_ptr(), Some(name), 1)?.map(ConstNamedTransform))
    }
    /// Share a native view transform without making an editable copy.
    pub fn try_const_view_transform(&self, name: &CStr) -> Result<Option<ConstViewTransform>> {
        Ok(lookup(self.handle.as_ptr(), Some(name), 2)?.map(ConstViewTransform))
    }
    /// Share OCIO's default scene-to-display view transform, if one exists.
    pub fn try_const_default_scene_to_display_view_transform(
        &self,
    ) -> Result<Option<ConstViewTransform>> {
        Ok(lookup(self.handle.as_ptr(), None, 3)?.map(ConstViewTransform))
    }
}
