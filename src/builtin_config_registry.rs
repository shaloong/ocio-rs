use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, Config, OcioError, Result};

pub struct BuiltinConfigRegistry {
    handle: NonNull<c_void>,
}

impl BuiltinConfigRegistry {
    pub fn get() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_builtin_config_registry_get() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn num_builtin_configs(&self) -> i32 {
        unsafe { ocio_sys::ocio_builtin_config_registry_get_num_builtin_configs(self.handle.as_ptr()) }
    }

    pub fn config_name(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_builtin_config_registry_get_config_name(
                self.handle.as_ptr(), index,
            ))
        }
    }

    pub fn config_ui_name(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_builtin_config_registry_get_config_ui_name(
                self.handle.as_ptr(), index,
            ))
        }
    }

    pub fn is_config_recommended(&self, index: i32) -> bool {
        unsafe {
            ocio_sys::ocio_builtin_config_registry_is_config_recommended(self.handle.as_ptr(), index)
        }
    }

    pub fn config_by_index(&self, index: i32) -> Option<Config> {
        let handle = unsafe {
            ocio_sys::ocio_builtin_config_registry_get_config_by_index(self.handle.as_ptr(), index)
        };
        NonNull::new(handle).map(|h| Config { handle: h })
    }

    pub fn config_by_name(&self, name: impl AsRef<str>) -> Option<Config> {
        let n = cstring(name).ok()?;
        let handle = unsafe {
            ocio_sys::ocio_builtin_config_registry_get_config_by_name(
                self.handle.as_ptr(), n.as_ptr().cast(),
            )
        };
        NonNull::new(handle).map(|h| Config { handle: h })
    }
}

impl Drop for BuiltinConfigRegistry {
    fn drop(&mut self) {
        // BuiltinConfigRegistry::Get() returns a reference singleton, no destroy needed.
        // We just let the handle drop. In real mode the pointer is to a static reference.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_registry() {
        let reg = BuiltinConfigRegistry::get();
        if crate::is_stub_build() {
            // In stub mode, returns error
            assert!(reg.is_err());
        }
    }

    #[test]
    fn registry_methods_no_crash() {
        if let Ok(reg) = BuiltinConfigRegistry::get() {
            let _ = reg.num_builtin_configs();
            let _ = reg.config_name(0);
            let _ = reg.config_ui_name(0);
            let _ = reg.is_config_recommended(0);
            let _ = reg.config_by_index(0);
            let _ = reg.config_by_name("default");
        }
    }
}
