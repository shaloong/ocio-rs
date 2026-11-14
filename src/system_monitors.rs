use crate::Result;
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
        crate::ocio_call_status()?;
        Ok(unsafe { CStr::from_ptr(value) })
    }
    /// Native ICC profile path, valid while this snapshot is alive.
    pub fn profile_filepath(&self, index: usize) -> Result<&CStr> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_system_monitors_get_profile_filepath(self.handle.as_ptr(), index)
        };
        crate::ocio_call_status()?;
        Ok(unsafe { CStr::from_ptr(value) })
    }
}
impl Drop for SystemMonitors {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_system_monitors_destroy(self.handle.as_ptr()) }
    }
}
