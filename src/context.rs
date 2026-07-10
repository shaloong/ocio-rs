use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstring, ConfigIOProxy, EnvironmentMode, OcioError, Result};
use ocio_sys;

/// Resolves search paths, environment variables, and working context for file-based operations.
pub struct Context {
    pub(crate) handle: NonNull<c_void>,
}

impl Context {
    /// Create a new OCIO context with default search-path and environment behavior.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_context_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an editable copy that is independent from the original context.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_context_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return OCIO's cache identifier for the current context state.
    pub fn cache_id(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_context_get_cache_id(self.handle.as_ptr())) }
    }

    /// Return the concatenated search-path string used by OCIO.
    pub fn search_path(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_context_get_search_path(self.handle.as_ptr())) }
    }

    /// Replace the concatenated search-path string used by OCIO.
    pub fn set_search_path(&self, path: impl AsRef<str>) -> Result<()> {
        let p = cstring(path)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_context_set_search_path(self.handle.as_ptr(), p.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Return the number of individual search-path entries.
    pub fn num_search_paths(&self) -> i32 {
        unsafe { ocio_sys::ocio_context_get_num_search_paths(self.handle.as_ptr()) }
    }

    /// Return one search-path entry by index.
    pub fn search_path_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_context_get_search_path_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Remove every explicit search-path entry.
    pub fn clear_search_paths(&self) {
        unsafe { ocio_sys::ocio_context_clear_search_paths(self.handle.as_ptr()) };
    }

    /// Append one search-path entry.
    pub fn add_search_path(&self, path: impl AsRef<str>) -> Result<()> {
        let p = cstring(path)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_context_add_search_path(self.handle.as_ptr(), p.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Return the working directory used for relative file resolution.
    pub fn working_dir(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_context_get_working_dir(self.handle.as_ptr())) }
    }

    /// Set the working directory used for relative file resolution.
    pub fn set_working_dir(&self, dirname: impl AsRef<str>) -> Result<()> {
        let d = cstring(dirname)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_context_set_working_dir(self.handle.as_ptr(), d.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Return one named string variable from the context.
    ///
    /// In real OCIO builds, an unknown name is represented as `Some("")`,
    /// matching OCIO's non-null empty-string ABI return. `None` indicates that
    /// this binding could not obtain a C string (for example, in stub mode).
    pub fn string_var(&self, name: impl AsRef<str>) -> Option<String> {
        let name = cstring(name).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_context_get_string_var(
                self.handle.as_ptr(),
                name.as_ptr().cast(),
            ))
        }
    }

    /// Set or replace one named string variable on the context.
    pub fn set_string_var(&self, name: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        let v = cstring(value)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_context_set_string_var(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
                v.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    /// Return the number of named string variables on the context.
    pub fn num_string_vars(&self) -> i32 {
        unsafe { ocio_sys::ocio_context_get_num_string_vars(self.handle.as_ptr()) }
    }

    /// Return one string-variable name by index.
    ///
    /// An out-of-range index is represented as `Some("")` in real OCIO builds.
    pub fn string_var_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_context_get_string_var_name_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Return one string-variable value by index.
    ///
    /// An out-of-range index is represented as `Some("")` in real OCIO builds.
    pub fn string_var_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_context_get_string_var_by_index(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Resolve `${VAR}`-style substitutions in `string` using this context.
    pub fn resolve_string_var(&self, string: impl AsRef<str>) -> Option<String> {
        let s = cstring(string).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_context_resolve_string_var(
                self.handle.as_ptr(),
                s.as_ptr().cast(),
            ))
        }
    }

    /// # Safety
    /// `used_context_vars` must be a valid pointer accepted by the OCIO ABI, or null.
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO context-vars pointer; prefer resolve_string_var unless you must interoperate with external OCIO ABI objects"
    )]
    pub unsafe fn resolve_string_var_v1(
        &self,
        string: impl AsRef<str>,
        used_context_vars: *mut c_void,
    ) -> Option<String> {
        let s = cstring(string).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_context_resolve_string_var_v1(
                self.handle.as_ptr(),
                s.as_ptr().cast(),
                used_context_vars,
            ))
        }
    }

    pub fn resolve_file_location(&self, filename: impl AsRef<str>) -> Option<String> {
        let f = cstring(filename).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_context_resolve_file_location(
                self.handle.as_ptr(),
                f.as_ptr().cast(),
            ))
        }
    }

    /// # Safety
    /// `used_context_vars` must be a valid pointer accepted by the OCIO ABI, or null.
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO context-vars pointer; prefer resolve_file_location unless you must interoperate with external OCIO ABI objects"
    )]
    pub unsafe fn resolve_file_location_v1(
        &self,
        filename: impl AsRef<str>,
        used_context_vars: *mut c_void,
    ) -> Option<String> {
        let f = cstring(filename).ok()?;
        unsafe {
            cstr_from_mut(ocio_sys::ocio_context_resolve_file_location_v1(
                self.handle.as_ptr(),
                f.as_ptr().cast(),
                used_context_vars,
            ))
        }
    }

    pub fn clear_string_vars(&self) {
        unsafe { ocio_sys::ocio_context_clear_string_vars(self.handle.as_ptr()) };
    }

    pub fn add_string_vars(&self, other: &Context) {
        unsafe {
            ocio_sys::ocio_context_add_string_vars(self.handle.as_ptr(), other.handle.as_ptr())
        };
    }

    /// Select whether OCIO imports only declared variables or the full process environment.
    pub fn set_environment_mode(&self, mode: EnvironmentMode) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_context_set_environment_mode(self.handle.as_ptr(), mode as i32);
        }
        crate::ocio_call_status()
    }

    pub fn environment_mode(&self) -> EnvironmentMode {
        let m = unsafe { ocio_sys::ocio_context_get_environment_mode(self.handle.as_ptr()) };
        match m {
            1 => EnvironmentMode::LoadAll,
            _ => EnvironmentMode::LoadPredefined,
        }
    }

    /// Refresh this context's string variables from the process environment.
    pub fn load_environment(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_context_load_environment(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    pub fn set_config_io_proxy_object(&self, proxy: &ConfigIOProxy) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_context_set_config_io_proxy(self.handle.as_ptr(), proxy.handle.as_ptr())
        };
        crate::ocio_call_status()
    }

    pub fn config_io_proxy_object(&self) -> Option<ConfigIOProxy> {
        let handle = unsafe { ocio_sys::ocio_context_get_config_io_proxy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|handle| ConfigIOProxy { handle })
    }

    /// # Safety
    /// The caller must pass a valid OCIO config-IO proxy pointer for the
    /// active ABI and keep it alive for as long as OCIO may use it. Prefer
    /// [`Self::set_config_io_proxy_object`] for a typed wrapper.
    #[doc(hidden)]
    /// Return a borrowed raw OCIO config-IO proxy handle.
    ///
    /// The pointer is owned by OCIO and must not be freed. It is only valid
    /// while this context remains alive and continues to reference the proxy.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO config-IO proxy handle; prefer standard Context path/string APIs where possible"
    )]
    pub unsafe fn set_config_io_proxy(&self, proxy: *mut c_void) {
        unsafe { ocio_sys::ocio_context_set_config_io_proxy(self.handle.as_ptr(), proxy) };
    }

    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO config-IO proxy handle; prefer standard Context path/string APIs where possible"
    )]
    pub fn config_io_proxy(&self) -> *mut c_void {
        unsafe { ocio_sys::ocio_context_get_config_io_proxy(self.handle.as_ptr()) }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer config_io_proxy()")]
    pub fn get_config_io_proxy(&self) -> *mut c_void {
        unsafe { ocio_sys::ocio_context_get_config_io_proxy(self.handle.as_ptr()) }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_context_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_context() {
        let ctx = Context::create();
        assert!(ctx.is_ok());
    }

    #[test]
    fn context_methods_no_crash() {
        let ctx = Context::create().unwrap();
        let _ = ctx.cache_id();
        let _ = ctx.search_path();
        let _ = ctx.num_search_paths();
        let _ = ctx.working_dir();
        let _ = ctx.num_string_vars();
        let _ = ctx.environment_mode();
    }

    #[test]
    fn string_var_no_crash() {
        let ctx = Context::create().unwrap();
        assert!(ctx.set_string_var("SHOT", "abc123").is_ok());
        let _ = ctx.string_var("SHOT");
        let _ = ctx.resolve_string_var("${SHOT}/file.exr");
    }

    #[test]
    fn search_paths_no_crash() {
        let ctx = Context::create().unwrap();
        ctx.clear_search_paths();
        assert!(ctx.add_search_path("/some/path").is_ok());
    }

    #[test]
    #[allow(deprecated)]
    fn context_v1_methods_no_crash() {
        let ctx = Context::create().unwrap();
        let other = Context::create().unwrap();
        ctx.add_string_vars(&other);
        let _ = unsafe { ctx.resolve_string_var_v1("${SHOT}/file.exr", std::ptr::null_mut()) };
        let _ = unsafe { ctx.resolve_file_location_v1("file.exr", std::ptr::null_mut()) };
        unsafe { ctx.set_config_io_proxy(std::ptr::null_mut()) };
        let _ = ctx.config_io_proxy();
    }

    #[test]
    #[allow(deprecated)]
    fn context_config_io_proxy_compat_alias_no_crash() {
        let ctx = Context::create().unwrap();
        let _ = ctx.get_config_io_proxy();
    }

    #[test]
    fn context_config_io_proxy_object_no_crash() {
        if crate::is_stub_build() {
            return;
        }

        let ctx = Context::create().unwrap();
        let proxy = ConfigIOProxy::create().unwrap();
        proxy
            .set_config_data(
                "ocio_profile_version: 2\nroles:\n  default: raw\ncolorspaces:\n  - !<ColorSpace> {name: raw, isdata: true}\n",
            )
            .unwrap();
        ctx.set_config_io_proxy_object(&proxy).unwrap();
        let _ = ctx.config_io_proxy_object();
    }
}
