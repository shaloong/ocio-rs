use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, OcioError, Result, EnvironmentMode};

pub struct Context {
    handle: NonNull<c_void>,
}

impl Context {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_context_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_context_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_context_get_cache_id(self.handle.as_ptr())) }
    }

    pub fn search_path(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_context_get_search_path(self.handle.as_ptr())) }
    }

    pub fn set_search_path(&self, path: impl AsRef<str>) -> Result<()> {
        let p = cstring(path)?;
        unsafe { ocio_sys::ocio_context_set_search_path(self.handle.as_ptr(), p.as_ptr().cast()) };
        Ok(())
    }

    pub fn num_search_paths(&self) -> i32 {
        unsafe { ocio_sys::ocio_context_get_num_search_paths(self.handle.as_ptr()) }
    }

    pub fn search_path_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_context_get_search_path_by_index(
                self.handle.as_ptr(), index,
            ))
        }
    }

    pub fn working_dir(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_context_get_working_dir(self.handle.as_ptr())) }
    }

    pub fn set_working_dir(&self, dirname: impl AsRef<str>) -> Result<()> {
        let d = cstring(dirname)?;
        unsafe { ocio_sys::ocio_context_set_working_dir(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn string_var(&self, name: impl AsRef<str>) -> Option<String> {
        let name = cstring(name).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_context_get_string_var(
                self.handle.as_ptr(), name.as_ptr().cast(),
            ))
        }
    }

    pub fn set_string_var(&self, name: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        let v = cstring(value)?;
        unsafe {
            ocio_sys::ocio_context_set_string_var(
                self.handle.as_ptr(), n.as_ptr().cast(), v.as_ptr().cast(),
            );
        }
        Ok(())
    }

    pub fn num_string_vars(&self) -> i32 {
        unsafe { ocio_sys::ocio_context_get_num_string_vars(self.handle.as_ptr()) }
    }

    pub fn string_var_name_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_context_get_string_var_name_by_index(
                self.handle.as_ptr(), index,
            ))
        }
    }

    pub fn string_var_by_index(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_context_get_string_var_by_index(
                self.handle.as_ptr(), index,
            ))
        }
    }

    pub fn resolve_string_var(&self, string: impl AsRef<str>) -> Option<String> {
        let s = cstring(string).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_context_resolve_string_var(
                self.handle.as_ptr(), s.as_ptr().cast(),
            ))
        }
    }

    pub fn resolve_file_location(&self, filename: impl AsRef<str>) -> Option<String> {
        let f = cstring(filename).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_context_resolve_file_location(
                self.handle.as_ptr(), f.as_ptr().cast(),
            ))
        }
    }

    pub fn clear_string_vars(&self) {
        unsafe { ocio_sys::ocio_context_clear_string_vars(self.handle.as_ptr()) };
    }

    pub fn set_environment_mode(&self, mode: EnvironmentMode) {
        unsafe {
            ocio_sys::ocio_context_set_environment_mode(self.handle.as_ptr(), mode as i32);
        }
    }

    pub fn environment_mode(&self) -> EnvironmentMode {
        let m = unsafe { ocio_sys::ocio_context_get_environment_mode(self.handle.as_ptr()) };
        match m { 1 => EnvironmentMode::LoadAll, _ => EnvironmentMode::LoadPredefined }
    }

    pub fn load_environment(&self) {
        unsafe { ocio_sys::ocio_context_load_environment(self.handle.as_ptr()) };
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
}
