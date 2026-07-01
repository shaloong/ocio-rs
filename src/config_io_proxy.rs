use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_to_opt_string, cstring, OcioError, Result};
use ocio_sys;

/// In-memory provider for OCIO config text and LUT file payloads.
///
/// This is primarily useful when embedding configs or serving OCIO assets from
/// archives, databases, virtual file systems, or application-managed caches.
pub struct ConfigIOProxy {
    pub(crate) handle: NonNull<c_void>,
}

impl ConfigIOProxy {
    /// Create an empty in-memory config/LUT provider.
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_config_io_proxy_create() };
        NonNull::new(handle)
            .map(|handle| Self { handle })
            .ok_or(OcioError::AllocationFailed)
    }

    /// Replace the primary OCIO config text payload.
    pub fn set_config_data(&self, data: impl AsRef<str>) -> Result<()> {
        let data = cstring(data)?;
        unsafe {
            ocio_sys::ocio_config_io_proxy_set_config_data(self.handle.as_ptr(), data.as_ptr())
        };
        Ok(())
    }

    /// Return the primary OCIO config text payload, if set.
    pub fn config_data(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_io_proxy_get_config_data(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Register the byte payload for a LUT file path and its fast hash.
    ///
    /// Returns whether the payload was accepted by the upstream OCIO proxy.
    pub fn set_lut_data(
        &self,
        filepath: impl AsRef<str>,
        data: &[u8],
        fast_hash: impl AsRef<str>,
    ) -> Result<bool> {
        let filepath = cstring(filepath)?;
        let fast_hash = cstring(fast_hash)?;
        Ok(unsafe {
            ocio_sys::ocio_config_io_proxy_set_lut_data(
                self.handle.as_ptr(),
                filepath.as_ptr(),
                data.as_ptr(),
                data.len(),
                fast_hash.as_ptr(),
            )
        })
    }

    /// Return the registered LUT payload for `filepath`, if present.
    pub fn lut_data(&self, filepath: impl AsRef<str>) -> Option<Vec<u8>> {
        let filepath = filepath.as_ref();
        let len = self.get_lut_data_size(filepath) as usize;
        let mut bytes = vec![0u8; len];
        let copied = self.copy_lut_data(filepath, &mut bytes);
        copied.then_some(bytes)
    }

    /// Return the upstream fast hash associated with `filepath`, if present.
    pub fn fast_lut_file_hash(&self, filepath: impl AsRef<str>) -> Option<String> {
        let filepath = cstring(filepath).ok()?;
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_io_proxy_get_fast_lut_file_hash(
                self.handle.as_ptr(),
                filepath.as_ptr(),
            ))
        }
    }

    #[doc(hidden)]
    pub fn get_lut_data_size(&self, filepath: impl AsRef<str>) -> u64 {
        let filepath = match cstring(filepath) {
            Ok(value) => value,
            Err(_) => return 0,
        };
        unsafe {
            ocio_sys::ocio_config_io_proxy_get_lut_data_size(
                self.handle.as_ptr(),
                filepath.as_ptr(),
            ) as u64
        }
    }

    #[doc(hidden)]
    pub fn copy_lut_data(&self, filepath: impl AsRef<str>, data: &mut [u8]) -> bool {
        let filepath = match cstring(filepath) {
            Ok(value) => value,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_config_io_proxy_copy_lut_data(
                self.handle.as_ptr(),
                filepath.as_ptr(),
                data.as_mut_ptr(),
                data.len(),
            )
        }
    }
}

impl Drop for ConfigIOProxy {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_config_io_proxy_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn create_proxy() {
        let proxy = ConfigIOProxy::create();
        if crate::is_stub_build() {
            assert!(proxy.is_err());
        } else {
            assert!(proxy.is_ok());
        }
    }

    #[test]
    fn proxy_payload_round_trip() {
        if crate::is_stub_build() {
            return;
        }

        let proxy = ConfigIOProxy::create().expect("config io proxy");
        let config_text =
            fs::read_to_string("tests/data/configs/context_test1/config.ocio").expect("config");
        let lut = fs::read("tests/data/configs/context_test1/lut1.clf").expect("lut");

        proxy
            .set_config_data(&config_text)
            .expect("set config data");
        assert_eq!(proxy.config_data().as_deref(), Some(config_text.as_str()));

        assert!(proxy
            .set_lut_data("E:/virtual/context/lut1.clf", &lut, "lut1-hash")
            .expect("set lut data"));
        assert_eq!(
            proxy
                .fast_lut_file_hash("E:/virtual/context/lut1.clf")
                .as_deref(),
            Some("lut1-hash")
        );
        assert_eq!(
            proxy.lut_data("E:/virtual/context/lut1.clf").as_deref(),
            Some(lut.as_slice())
        );
    }
}
