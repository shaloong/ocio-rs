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

/// Shared read-only native look.
pub struct ConstLook(Item);
/// Shared read-only native named transform.
pub struct ConstNamedTransform(Item);
/// Shared read-only native view transform.
pub struct ConstViewTransform(Item);

macro_rules! accessors {
    ($ty:ident, $editable:ident) => {
        impl $ty {
            /// Copy the native name as owned text.
            pub fn name(&self) -> Result<Option<String>> {
                self.0.name()
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
