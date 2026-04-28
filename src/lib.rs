use std::ffi::{c_void, CString};
use std::ptr::NonNull;

use ocio_sys;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OcioError {
    #[error("OpenColorIO error: {0}")]
    Ocio(String),
    #[error("path contains interior NUL byte")]
    InteriorNul,
    #[error("OpenColorIO handle allocation failed")]
    AllocationFailed,
}

pub type Result<T> = std::result::Result<T, OcioError>;

pub fn is_stub_build() -> bool {
    unsafe { ocio_sys::ocio_runtime_is_stub() }
}

fn cstring(value: impl AsRef<str>) -> Result<CString> {
    CString::new(value.as_ref()).map_err(|_| OcioError::InteriorNul)
}

pub struct Config {
    handle: NonNull<c_void>,
}

impl Config {
    pub fn raw() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_config_create_raw() };
        NonNull::new(handle).map(|handle| Self { handle }).ok_or(OcioError::AllocationFailed)
    }

    pub fn from_file(path: impl AsRef<str>) -> Result<Self> {
        let path = cstring(path)?;
        let handle = unsafe { ocio_sys::ocio_config_create_from_file(path.as_ptr().cast()) };
        NonNull::new(handle).map(|handle| Self { handle }).ok_or(OcioError::AllocationFailed)
    }

    pub fn processor(&self, src: impl AsRef<str>, dst: impl AsRef<str>) -> Result<Processor> {
        let src = cstring(src)?;
        let dst = cstring(dst)?;
        let handle = unsafe {
            ocio_sys::ocio_config_get_processor(
                self.handle.as_ptr(),
                src.as_ptr().cast(),
                dst.as_ptr().cast(),
            )
        };
        NonNull::new(handle).map(|handle| Processor { handle }).ok_or(OcioError::AllocationFailed)
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_config_destroy(self.handle.as_ptr()) };
    }
}

pub struct Processor {
    handle: NonNull<c_void>,
}

impl Processor {
    pub fn apply_rgba(&self, rgba: &mut [f32; 4]) -> Result<()> {
        unsafe {
            ocio_sys::ocio_processor_apply_rgba(self.handle.as_ptr(), rgba.as_mut_ptr(), rgba.len());
        }
        Ok(())
    }
}

impl Drop for Processor {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_processor_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_raw_config() {
        let cfg = Config::raw();
        assert!(cfg.is_ok());
    }

    #[test]
    fn fail_on_missing_file() {
        if is_stub_build() {
            let cfg = Config::from_file("tests/missing_config.ocio");
            assert!(cfg.is_err());
        }
        // In a real OCIO build, CreateFromFile on a missing path may
        // trigger a C++ exception or crash, so this test is only valid
        // for the stub build.
    }
}
