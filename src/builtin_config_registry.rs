use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstring, Config, OcioError, Result};
use ocio_sys;

/// Enumerates built-in OCIO configurations bundled with the upstream library.
pub struct BuiltinConfigRegistry {
    handle: NonNull<c_void>,
}

impl BuiltinConfigRegistry {
    /// Get the process-wide built-in config registry singleton.
    pub fn get() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_builtin_config_registry_get() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the number of built-in configs exposed by the linked OCIO build.
    pub fn num_builtin_configs(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_builtin_config_registry_get_num_builtin_configs(self.handle.as_ptr())
                as i32
        }
    }

    /// Return the stable OCIO name for the built-in config at `index`.
    pub fn config_name(&self, index: i32) -> Option<String> {
        self.try_config_name(index).ok().flatten()
    }

    /// Return a built-in config name while preserving bridge failures.
    pub fn try_config_name(&self, index: i32) -> Result<Option<String>> {
        if index < 0 {
            return Ok(None);
        }
        crate::clear_last_error();
        let name = unsafe {
            cstr_from_mut(
                ocio_sys::ocio_builtin_config_registry_get_builtin_config_name(
                    self.handle.as_ptr(),
                    index as usize,
                ),
            )
        };
        crate::ocio_call_status()?;
        Ok(name)
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer config_name()")]
    pub fn get_builtin_config_name(&self, index: i32) -> Option<String> {
        self.config_name(index)
    }

    /// Return the user-facing UI name for the built-in config at `index`.
    pub fn config_ui_name(&self, index: i32) -> Option<String> {
        self.try_config_ui_name(index).ok().flatten()
    }

    /// Return a built-in config UI name while preserving bridge failures.
    pub fn try_config_ui_name(&self, index: i32) -> Result<Option<String>> {
        if index < 0 {
            return Ok(None);
        }
        crate::clear_last_error();
        let name = unsafe {
            cstr_from_mut(
                ocio_sys::ocio_builtin_config_registry_get_builtin_config_ui_name(
                    self.handle.as_ptr(),
                    index as usize,
                ),
            )
        };
        crate::ocio_call_status()?;
        Ok(name)
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer config_ui_name()")]
    pub fn get_builtin_config_ui_name(&self, index: i32) -> Option<String> {
        self.config_ui_name(index)
    }

    /// Return whether the built-in config at `index` is marked recommended upstream.
    pub fn is_config_recommended(&self, index: i32) -> bool {
        self.try_is_config_recommended(index).unwrap_or(false)
    }

    /// Return whether the built-in config at `index` is marked recommended upstream.
    ///
    /// Invalid indices are reported as OCIO errors. Use [`Self::is_config_recommended`]
    /// when the legacy `false` fallback is preferred.
    pub fn try_is_config_recommended(&self, index: i32) -> Result<bool> {
        if index < 0 {
            return Err(OcioError::InvalidInput(
                "BuiltinConfigRegistry index must be non-negative".to_owned(),
            ));
        }
        crate::clear_last_error();
        let recommended = unsafe {
            ocio_sys::ocio_builtin_config_registry_is_builtin_config_recommended(
                self.handle.as_ptr(),
                index as usize,
            )
        };
        crate::ocio_call_status()?;
        Ok(recommended)
    }

    /// Compatibility alias for [`Self::is_config_recommended`].
    pub fn is_builtin_config_recommended(&self, index: i32) -> bool {
        self.is_config_recommended(index)
    }

    /// Create a live [`Config`] from the built-in config at `index`.
    pub fn config_by_index(&self, index: i32) -> Option<Config> {
        let name = self.config_name(index)?;
        self.config_by_name(name)
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer config_yaml_by_index()")]
    pub fn get_builtin_config(&self, index: i32) -> Option<String> {
        self.config_yaml_by_index(index)
    }

    /// Create a live [`Config`] from the built-in config named `name`.
    pub fn config_by_name(&self, name: impl AsRef<str>) -> Option<Config> {
        self.try_config_by_name(name).ok().flatten()
    }

    /// Create a built-in config by name while preserving bridge failures.
    pub fn try_config_by_name(&self, name: impl AsRef<str>) -> Result<Option<Config>> {
        let n = cstring(name)?;
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_config_create_from_builtin_config(n.as_ptr().cast()) };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| Config { handle }))
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer config_yaml_by_name()")]
    pub fn get_builtin_config_by_name(&self, name: impl AsRef<str>) -> Option<String> {
        self.config_yaml_by_name(name)
    }

    /// Return the serialized OCIO YAML/text for the built-in config at `index`.
    pub fn config_yaml_by_index(&self, index: i32) -> Option<String> {
        self.try_config_yaml_by_index(index).ok().flatten()
    }

    /// Return built-in config YAML by index while preserving bridge failures.
    pub fn try_config_yaml_by_index(&self, index: i32) -> Result<Option<String>> {
        if index < 0 {
            return Ok(None);
        }
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_builtin_config_registry_get_builtin_config(
                self.handle.as_ptr(),
                index as usize,
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { cstr_from_mut(handle) })
    }

    /// Return the serialized OCIO YAML/text for the built-in config named `name`.
    pub fn config_yaml_by_name(&self, name: impl AsRef<str>) -> Option<String> {
        self.try_config_yaml_by_name(name).ok().flatten()
    }

    /// Return built-in config YAML by name while preserving bridge failures.
    pub fn try_config_yaml_by_name(&self, name: impl AsRef<str>) -> Result<Option<String>> {
        let n = cstring(name)?;
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_builtin_config_registry_get_builtin_config_by_name(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()?;
        Ok(unsafe { cstr_from_mut(handle) })
    }
}

impl Drop for BuiltinConfigRegistry {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_builtin_config_registry_destroy(self.handle.as_ptr()) }
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
            let _ = reg.is_builtin_config_recommended(0);
            let _ = reg.config_yaml_by_index(0);
            let _ = reg.config_yaml_by_name("default");
            let _ = reg.config_by_index(0);
            let _ = reg.config_by_name("default");
        }
    }

    #[test]
    #[allow(deprecated)]
    fn registry_compat_aliases_no_crash() {
        if let Ok(reg) = BuiltinConfigRegistry::get() {
            let _ = reg.get_builtin_config_name(0);
            let _ = reg.get_builtin_config_ui_name(0);
            let _ = reg.get_builtin_config(0);
            let _ = reg.get_builtin_config_by_name("default");
        }
    }
}
