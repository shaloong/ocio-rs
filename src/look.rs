use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, OcioError, Result, TransformDirection};

pub struct Look {
    pub(crate) handle: NonNull<c_void>,
}

impl Look {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_look_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_look_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn name(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_look_get_name(self.handle.as_ptr())) }
    }

    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe { ocio_sys::ocio_look_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        Ok(())
    }

    pub fn process_space(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_look_get_process_space(self.handle.as_ptr())) }
    }

    pub fn set_process_space(&self, space: impl AsRef<str>) -> Result<()> {
        let s = cstring(space)?;
        unsafe { ocio_sys::ocio_look_set_process_space(self.handle.as_ptr(), s.as_ptr().cast()) };
        Ok(())
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_look_get_direction(self.handle.as_ptr()) };
        match dir { 1 => TransformDirection::Inverse, _ => TransformDirection::Forward }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_look_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }
}

impl Drop for Look {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_look_destroy(self.handle.as_ptr()) };
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
}
