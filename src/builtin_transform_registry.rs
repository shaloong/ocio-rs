use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_to_opt_string, OcioError, Result};
use ocio_sys;

/// Enumerates the built-in transform styles provided by the linked OCIO build.
pub struct BuiltinTransformRegistry {
    handle: NonNull<c_void>,
}

impl BuiltinTransformRegistry {
    /// Get the process-wide built-in transform registry singleton.
    pub fn get() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_builtin_transform_registry_get() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the number of registered built-in styles.
    pub fn num_builtins(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_builtin_transform_registry_get_num_builtins(self.handle.as_ptr()) as i32
        }
    }

    /// Return the stable OCIO style identifier at `index`.
    pub fn builtin_style(&self, index: i32) -> Option<String> {
        self.try_builtin_style(index).ok().flatten()
    }

    /// Return a built-in style identifier while preserving bridge failures.
    pub fn try_builtin_style(&self, index: i32) -> Result<Option<String>> {
        if index < 0 {
            return Ok(None);
        }
        crate::clear_last_error();
        let style = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_builtin_transform_registry_get_builtin_style(
                self.handle.as_ptr(),
                index as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(style)
    }

    /// Return the human-readable description for the built-in style at `index`.
    pub fn builtin_description(&self, index: i32) -> Option<String> {
        self.try_builtin_description(index).ok().flatten()
    }

    /// Return a built-in style description while preserving bridge failures.
    pub fn try_builtin_description(&self, index: i32) -> Result<Option<String>> {
        if index < 0 {
            return Ok(None);
        }
        crate::clear_last_error();
        let description = unsafe {
            cstr_to_opt_string(
                ocio_sys::ocio_builtin_transform_registry_get_builtin_description(
                    self.handle.as_ptr(),
                    index as usize,
                ),
            )
        };
        crate::ocio_call_status()?;
        Ok(description)
    }
}

impl Drop for BuiltinTransformRegistry {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_builtin_transform_registry_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_registry() {
        let registry = BuiltinTransformRegistry::get();
        if crate::is_stub_build() {
            assert!(registry.is_err());
        } else {
            assert!(registry.is_ok());
        }
    }

    #[test]
    fn registry_methods_no_crash() {
        if let Ok(registry) = BuiltinTransformRegistry::get() {
            let count = registry.num_builtins();
            assert!(count >= 0);
            let _ = registry.builtin_style(0);
            let _ = registry.builtin_description(0);
        }
    }

    #[test]
    #[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
    fn registry_methods_real_behavior() {
        if crate::is_stub_build() {
            return;
        }

        let registry = BuiltinTransformRegistry::get().expect("builtin transform registry");
        let count = registry.num_builtins();
        assert!(count > 0);
        assert!(registry.builtin_style(0).is_some());
        assert!(registry.builtin_description(0).is_some());
    }
}
