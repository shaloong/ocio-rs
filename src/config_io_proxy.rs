use std::ffi::{c_void, CStr};
use std::ptr::NonNull;

use crate::{cstr_to_opt_string, cstring, OcioError, Result};
use ocio_sys;

/// On-demand provider for the OCIO config and its referenced LUTs.
///
/// Paths are fully resolved by OCIO. A missing LUT must have an empty fast
/// hash. Keep hashes stable for unchanged content and change them when content
/// changes so OCIO's caches remain correct. Methods may be called from native
/// threads; implementations must be thread-safe and must not reenter OCIO.
pub trait ConfigIOProvider: Send + Sync + 'static {
    /// Return the configuration YAML.
    fn config_data(&self) -> Result<String>;
    /// Return the bytes of a referenced LUT, or an empty vector if unavailable.
    fn lut_data(&self, path: &CStr) -> Result<Vec<u8>>;
    /// Return a fast cache identifier, or an empty string if unavailable.
    fn fast_lut_file_hash(&self, path: &CStr) -> Result<String>;
}

/// Provider for OCIO config text and LUT file payloads.
///
/// This is primarily useful when embedding configs or serving OCIO assets from
/// archives, databases, virtual file systems, or application-managed caches.
pub struct ConfigIOProxy {
    pub(crate) handle: NonNull<c_void>,
}

impl ConfigIOProxy {
    /// Create an on-demand provider whose lifetime is retained by OCIO.
    /// Provider errors and panics become OCIO errors without unwinding across
    /// the native boundary. The in-memory `set_*` methods do not apply to it.
    pub fn from_provider<P: ConfigIOProvider>(provider: P) -> Result<Self> {
        type Write = unsafe extern "C" fn(*mut c_void, *const u8, usize) -> bool;
        unsafe extern "C" fn read<P: ConfigIOProvider>(
            context: *mut c_void,
            kind: i32,
            path: *const i8,
            destination: *mut c_void,
            write: Write,
        ) -> bool {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<bool> {
                let provider = unsafe { &*context.cast::<P>() };
                let bytes = match kind {
                    0 => provider.config_data()?.into_bytes(),
                    1 | 2 => {
                        if path.is_null() {
                            return Ok(false);
                        }
                        let path = unsafe { CStr::from_ptr(path) };
                        if kind == 1 {
                            provider.lut_data(path)?
                        } else {
                            provider.fast_lut_file_hash(path)?.into_bytes()
                        }
                    }
                    _ => return Ok(false),
                };
                Ok(unsafe { write(destination, bytes.as_ptr(), bytes.len()) })
            }))
            .unwrap_or(Ok(false))
            .unwrap_or(false)
        }
        unsafe extern "C" fn release<P>(context: *mut c_void) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                drop(Box::from_raw(context.cast::<P>()));
            }));
        }
        let context = Box::into_raw(Box::new(provider)).cast();
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_config_io_proxy_create_with_callback(context, read::<P>, release::<P>)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an empty in-memory config/LUT provider.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_config_io_proxy_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Replace the primary OCIO config text payload.
    pub fn set_config_data(&self, data: impl AsRef<str>) -> Result<()> {
        let data = cstring(data)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_config_io_proxy_set_config_data(self.handle.as_ptr(), data.as_ptr())
        };
        crate::ocio_call_status()
    }

    /// Return the primary OCIO config text payload, if set.
    pub fn config_data(&self) -> Option<String> {
        self.try_config_data().ok().flatten()
    }

    /// Return the primary config text payload while preserving bridge failures.
    pub fn try_config_data(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let data = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_io_proxy_get_config_data(
                self.handle.as_ptr(),
            ))
        };
        crate::ocio_call_status()?;
        Ok(data)
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
        crate::clear_last_error();
        let accepted = unsafe {
            ocio_sys::ocio_config_io_proxy_set_lut_data(
                self.handle.as_ptr(),
                filepath.as_ptr(),
                data.as_ptr(),
                data.len(),
                fast_hash.as_ptr(),
            )
        };
        if accepted {
            Ok(true)
        } else {
            match crate::ocio_call_status() {
                Ok(()) => Ok(false),
                Err(err) => Err(err),
            }
        }
    }

    /// Return the registered LUT payload for `filepath`, if present.
    pub fn lut_data(&self, filepath: impl AsRef<str>) -> Option<Vec<u8>> {
        self.try_lut_data(filepath).ok().flatten()
    }

    /// Return the registered LUT payload for `filepath`, distinguishing
    /// "not found" (`Ok(None)`) from "internal error" (`Err`).
    pub fn try_lut_data(&self, filepath: impl AsRef<str>) -> Result<Option<Vec<u8>>> {
        unsafe extern "C" fn write(destination: *mut c_void, data: *const u8, len: usize) -> bool {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let result = unsafe { &mut *destination.cast::<Vec<u8>>() };
                if len != 0 {
                    let data = unsafe { std::slice::from_raw_parts(data, len) };
                    result.extend_from_slice(data);
                }
            }))
            .is_ok()
        }
        let filepath = cstring(filepath)?;
        let mut bytes = Vec::new();
        crate::clear_last_error();
        let found = unsafe {
            ocio_sys::ocio_config_io_proxy_visit_lut_data(
                self.handle.as_ptr(),
                filepath.as_ptr(),
                (&mut bytes as *mut Vec<u8>).cast(),
                write,
            )
        };
        crate::ocio_call_status()?;
        Ok(found.then_some(bytes))
    }

    /// Copy the LUT payload into `data`, returning whether the copy succeeded.
    ///
    /// Returns `Ok(true)` if data was copied, `Ok(false)` if the LUT was not
    /// found or the buffer is too small, and `Err` on internal errors.
    pub fn try_copy_lut_data(&self, filepath: impl AsRef<str>, data: &mut [u8]) -> Result<bool> {
        let filepath = cstring(filepath)?;
        crate::clear_last_error();
        let copied = unsafe {
            ocio_sys::ocio_config_io_proxy_copy_lut_data(
                self.handle.as_ptr(),
                filepath.as_ptr(),
                data.as_mut_ptr(),
                data.len(),
            )
        };
        if copied {
            Ok(true)
        } else {
            match crate::ocio_call_status() {
                Ok(()) => Ok(false),
                Err(err) => Err(err),
            }
        }
    }

    /// Return the upstream fast hash associated with `filepath`, if present.
    pub fn fast_lut_file_hash(&self, filepath: impl AsRef<str>) -> Option<String> {
        self.try_fast_lut_file_hash(filepath).ok().flatten()
    }

    /// Return an upstream LUT fast hash while preserving bridge failures.
    pub fn try_fast_lut_file_hash(&self, filepath: impl AsRef<str>) -> Result<Option<String>> {
        let filepath = cstring(filepath)?;
        crate::clear_last_error();
        let hash = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_config_io_proxy_get_fast_lut_file_hash(
                self.handle.as_ptr(),
                filepath.as_ptr(),
            ))
        };
        crate::ocio_call_status()?;
        Ok(hash)
    }

    #[doc(hidden)]
    pub fn get_lut_data_size(&self, filepath: impl AsRef<str>) -> u64 {
        self.try_get_lut_data_size(filepath).unwrap_or(0)
    }

    /// Return the size of the registered LUT payload, or 0 if not found.
    pub fn try_get_lut_data_size(&self, filepath: impl AsRef<str>) -> Result<u64> {
        let filepath = cstring(filepath)?;
        crate::clear_last_error();
        let size = unsafe {
            ocio_sys::ocio_config_io_proxy_get_lut_data_size(
                self.handle.as_ptr(),
                filepath.as_ptr(),
            )
        } as u64;
        crate::ocio_call_status()?;
        Ok(size)
    }

    /// Return whether a LUT payload is registered for `filepath`.
    pub fn has_lut_data(&self, filepath: impl AsRef<str>) -> bool {
        self.try_has_lut_data(filepath).unwrap_or(false)
    }

    /// Return whether a LUT payload is registered for `filepath`, surfacing
    /// internal OCIO bridge failures.
    pub fn try_has_lut_data(&self, filepath: impl AsRef<str>) -> Result<bool> {
        let filepath = cstring(filepath)?;
        crate::clear_last_error();
        let exists = unsafe {
            ocio_sys::ocio_config_io_proxy_has_lut_data(self.handle.as_ptr(), filepath.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(exists)
    }

    #[doc(hidden)]
    pub fn copy_lut_data(&self, filepath: impl AsRef<str>, data: &mut [u8]) -> bool {
        self.try_copy_lut_data(filepath, data).unwrap_or(false)
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
    #[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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

        // try_lut_data returns Ok(None) for missing LUTs, Ok(Some(..)) for found ones
        assert!(
            proxy
                .try_lut_data("E:/virtual/context/missing.clf")
                .unwrap()
                .is_none(),
            "missing LUT should return None"
        );
        assert!(
            proxy
                .try_lut_data("E:/virtual/context/lut1.clf")
                .unwrap()
                .is_some(),
            "found LUT should return Some"
        );

        proxy
            .set_lut_data("E:/virtual/context/empty.clf", &[], "empty-hash")
            .expect("set empty LUT data");
        assert!(proxy
            .try_has_lut_data("E:/virtual/context/empty.clf")
            .expect("query empty LUT"));
        assert_eq!(
            proxy
                .try_lut_data("E:/virtual/context/empty.clf")
                .expect("read empty LUT"),
            Some(Vec::new())
        );

        // try_copy_lut_data returns Ok(true) for found, Ok(false) for missing
        let mut buf = vec![0u8; lut.len()];
        assert!(proxy
            .try_copy_lut_data("E:/virtual/context/lut1.clf", &mut buf)
            .unwrap());
        assert_eq!(buf, lut);

        let mut empty_buf = vec![0u8; 1];
        assert!(!proxy
            .try_copy_lut_data("E:/virtual/context/missing.clf", &mut empty_buf)
            .unwrap());
    }
}
