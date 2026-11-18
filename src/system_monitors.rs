use crate::{OcioError, Result};
use std::{
    ffi::{c_void, CStr},
    ptr::NonNull,
};

/// Native operating-system monitor profile snapshot.
pub struct SystemMonitors {
    handle: NonNull<c_void>,
}
impl SystemMonitors {
    /// Obtain the native singleton snapshot.
    pub fn get() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_system_monitors_get() };
        Ok(Self {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Whether the operating system supports monitor profile discovery.
    pub fn is_supported(&self) -> bool {
        unsafe { ocio_sys::ocio_system_monitors_is_supported(self.handle.as_ptr()) }
    }
    /// Number of monitors in the snapshot.
    pub fn num_monitors(&self) -> usize {
        unsafe { ocio_sys::ocio_system_monitors_get_num_monitors(self.handle.as_ptr()) }
    }
    /// Native monitor name, valid while this snapshot is alive.
    pub fn monitor_name(&self, index: usize) -> Result<&CStr> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_system_monitors_get_monitor_name(self.handle.as_ptr(), index) };
        self.text(value)
    }
    /// Native ICC profile path, valid while this snapshot is alive.
    pub fn profile_filepath(&self, index: usize) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_system_monitors_get_profile_filepath(self.handle.as_ptr(), index)
        };
        self.text(value)
    }

    fn text(&self, value: *const std::ffi::c_char) -> Result<&CStr> {
        crate::ocio_call_status()?;
        if value.is_null() {
            return Err(OcioError::Ocio("null system monitor text".into()));
        }
        // The immutable native snapshot owns the returned NUL-terminated text.
        Ok(unsafe { CStr::from_ptr(value) })
    }
}
impl Drop for SystemMonitors {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_system_monitors_destroy(self.handle.as_ptr()) }
    }
}
