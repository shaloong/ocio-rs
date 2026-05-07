use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstr_from_mut, cstring, OcioError, Result, TransformDirection};
use crate::transform::{TransformHandle, Transform, transform_from_raw_handle};

pub struct Look {
    pub(crate) handle: NonNull<c_void>,
}

impl Look {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_look_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_look_create_editable_copy(self.handle.as_ptr() as *mut c_void) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn name(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_look_get_name(self.handle.as_ptr() as *mut c_void)) }
    }

    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe { ocio_sys::ocio_look_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        Ok(())
    }

    pub fn process_space(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_look_get_process_space(self.handle.as_ptr() as *mut c_void)) }
    }

    pub fn set_process_space(&self, space: impl AsRef<str>) -> Result<()> {
        let s = cstring(space)?;
        unsafe { ocio_sys::ocio_look_set_process_space(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn description(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_look_get_description(self.handle.as_ptr() as *mut c_void)) }
    }

    pub fn set_description(&self, description: impl AsRef<str>) -> Result<()> {
        let d = cstring(description)?;
        unsafe { ocio_sys::ocio_look_set_description(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn transform(&self) -> Option<Transform> {
        let handle = unsafe { ocio_sys::ocio_look_get_transform(self.handle.as_ptr() as *mut c_void) };
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
        let handle = unsafe { ocio_sys::ocio_look_get_inverse_transform(self.handle.as_ptr() as *mut c_void) };
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

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_look_get_direction(self.handle.as_ptr() as *mut c_void) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_look_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn num_aliases(&self) -> i32 {
        unsafe { ocio_sys::ocio_look_get_num_aliases(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn alias(&self, index: i32) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_look_get_alias(self.handle.as_ptr(), index)) }
    }

    pub fn add_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        unsafe { ocio_sys::ocio_look_add_alias(self.handle.as_ptr(), a.as_ptr().cast()) };
        Ok(())
    }

    pub fn remove_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        unsafe { ocio_sys::ocio_look_remove_alias(self.handle.as_ptr(), a.as_ptr().cast()) };
        Ok(())
    }

    pub fn clear_aliases(&self) {
        unsafe { ocio_sys::ocio_look_clear_aliases(self.handle.as_ptr() as *mut c_void) };
    }

    pub fn is_inactive(&self) -> bool {
        unsafe { ocio_sys::ocio_look_is_inactive(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn set_inactive(&self, inactive: bool) {
        unsafe { ocio_sys::ocio_look_set_inactive(self.handle.as_ptr(), inactive) };
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
        let _ = look.direction();
    }

    #[test]
    fn set_name() {
        let look = Look::create().unwrap();
        assert!(look.set_name("MyLook").is_ok());
    }

    #[test]
    fn set_direction() {
        let look = Look::create().unwrap();
        look.set_direction(TransformDirection::Inverse);
        let _ = look.direction();
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
    fn aliases_no_crash() {
        let look = Look::create().unwrap();
        let _ = look.num_aliases();
        let _ = look.alias(0);
        assert!(look.add_alias("test_alias").is_ok());
        assert!(look.remove_alias("test_alias").is_ok());
        look.clear_aliases();
    }

    #[test]
    fn inactive_no_crash() {
        let look = Look::create().unwrap();
        let _ = look.is_inactive();
        look.set_inactive(true);
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
