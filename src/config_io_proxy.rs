use std::borrow::Cow;
use std::ffi::{c_void, CStr};
use std::ptr::NonNull;

use crate::{cstring, OcioError, Result};
use ocio_sys;

/// On-demand provider for the OCIO config and its referenced LUTs.
///
/// Paths are fully resolved by OCIO. A missing LUT must have an empty fast
/// hash. Keep hashes stable for unchanged content and change them when content
/// changes so OCIO's caches remain correct. Methods may be called from native
/// threads; implementations must be thread-safe and must not reenter OCIO.
/// Providers may borrow data from `self` to avoid a Rust-side allocation. OCIO
/// requires an owned native return value, so the bridge still copies the bytes
/// into native storage.
///
/// ```
/// use std::{borrow::Cow, ffi::CStr};
/// use ocio_rs::{ConfigIOProvider, Result};
/// struct Assets { config: String, lut: Vec<u8> }
/// impl ConfigIOProvider for Assets {
///     fn config_data(&self) -> Result<Cow<'_, str>> { Ok(Cow::Borrowed(&self.config)) }
///     fn lut_data<'a>(&'a self, _: &CStr) -> Result<Cow<'a, [u8]>> {
///         Ok(Cow::Borrowed(&self.lut))
///     }
///     fn fast_lut_file_hash<'a>(&'a self, _: &CStr) -> Result<Cow<'a, [u8]>> {
///         Ok(Cow::Borrowed(b"asset-v1"))
///     }
/// }
/// ```
pub trait ConfigIOProvider: Send + Sync + 'static {
    /// Return UTF-8 configuration YAML. Implement this method or override
    /// [`Self::config_data_bytes`] to provide the native byte representation.
    fn config_data(&self) -> Result<Cow<'_, str>> {
        Err(OcioError::InvalidInput(
            "config provider does not implement config_data or config_data_bytes".into(),
        ))
    }
    /// Return the complete native config payload. The default adapts the text
    /// method without copying borrowed data and moves owned strings into bytes.
    /// Override this method when the payload is not UTF-8. OCIO is responsible
    /// for deciding whether the returned bytes are valid configuration YAML.
    fn config_data_bytes(&self) -> Result<Cow<'_, [u8]>> {
        self.config_data().map(|data| match data {
            Cow::Borrowed(text) => Cow::Borrowed(text.as_bytes()),
            Cow::Owned(text) => Cow::Owned(text.into_bytes()),
        })
    }
    /// Return the bytes of a referenced LUT, or an empty slice if unavailable.
    fn lut_data<'a>(&'a self, path: &CStr) -> Result<Cow<'a, [u8]>>;
    /// Return a fast cache identifier as bytes, or an empty slice if unavailable.
    fn fast_lut_file_hash<'a>(&'a self, path: &CStr) -> Result<Cow<'a, [u8]>>;
}

unsafe extern "C" fn append_bytes(destination: *mut c_void, data: *const u8, len: usize) -> bool {
    if destination.is_null() || len > isize::MAX as usize || (len != 0 && data.is_null()) {
        return false;
    }
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let result = unsafe { &mut *destination.cast::<Vec<u8>>() };
        if len != 0 {
            if result.try_reserve(len).is_err() {
                return false;
            }
            let data = unsafe { std::slice::from_raw_parts(data, len) };
            result.extend_from_slice(data);
        }
        true
    }))
    .unwrap_or(false)
}

/// Provider for OCIO config text and LUT file payloads.
///
/// This is primarily useful when embedding configs or serving OCIO assets from
/// archives, databases, virtual file systems, or application-managed caches.
/// The in-memory implementation synchronizes access through shared native
/// aliases. An individual read observes a complete payload snapshot, but
/// separate hash and payload reads are not a transaction. Changing data does
/// not itself invalidate OCIO's caches; coordinate updates with processing
/// and cache management when consistent content across calls is required.
pub struct ConfigIOProxy {
    pub(crate) handle: NonNull<c_void>,
}

impl ConfigIOProxy {
    /// Create an on-demand provider whose lifetime is retained by OCIO.
    /// Provider errors and panics become OCIO errors without unwinding across
    /// the native boundary. Error messages include the provider's error text;
    /// do not include secrets in it. The in-memory `set_*` methods do not apply.
    pub fn from_provider<P: ConfigIOProvider>(provider: P) -> Result<Self> {
        type Write = unsafe extern "C" fn(*mut c_void, *const u8, usize) -> bool;
        unsafe extern "C" fn read<P: ConfigIOProvider>(
            context: *mut c_void,
            kind: i32,
            path: *const i8,
            destination: *mut c_void,
            write: Write,
        ) -> bool {
            let result =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<bool> {
                    let provider = unsafe { &*context.cast::<P>() };
                    let emit =
                        |bytes: &[u8]| unsafe { write(destination, bytes.as_ptr(), bytes.len()) };
                    match kind {
                        0 => Ok(emit(&provider.config_data_bytes()?)),
                        1 | 2 => {
                            if path.is_null() {
                                return Ok(false);
                            }
                            let path = unsafe { CStr::from_ptr(path) };
                            if kind == 1 {
                                Ok(emit(&provider.lut_data(path)?))
                            } else {
                                Ok(emit(&provider.fast_lut_file_hash(path)?))
                            }
                        }
                        _ => Ok(false),
                    }
                }));
            match result {
                Ok(Ok(ok)) => ok,
                Ok(Err(error)) => {
                    let message = error.to_string();
                    unsafe { write(destination, message.as_ptr(), message.len()) };
                    false
                }
                Err(_) => {
                    let message = b"Rust config IO provider panicked";
                    unsafe { write(destination, message.as_ptr(), message.len()) };
                    false
                }
            }
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
        self.set_config_data_bytes(data.as_ref().as_bytes())
    }

    /// Replace the primary configuration payload with arbitrary native bytes.
    /// OCIO may reject the payload later when parsing it as YAML.
    pub fn set_config_data_bytes(&self, data: &[u8]) -> Result<()> {
        crate::clear_last_error();
        let success = unsafe {
            ocio_sys::ocio_config_io_proxy_set_config_data_bytes(
                self.handle.as_ptr(),
                data.as_ptr(),
                data.len(),
            )
        };
        crate::ocio_call_status()?;
        if success {
            Ok(())
        } else {
            Err(OcioError::InvalidInput(
                "config data was not accepted".into(),
            ))
        }
    }

    /// Return the primary OCIO config text payload, if set.
    pub fn config_data(&self) -> Option<String> {
        self.try_config_data().ok().flatten()
    }

    /// Return the primary config text payload, replacing invalid UTF-8.
    /// Use [`Self::try_config_data_bytes`] for a lossless result.
    pub fn try_config_data(&self) -> Result<Option<String>> {
        Ok(self.try_config_data_bytes()?.map(|data| {
            String::from_utf8(data)
                .unwrap_or_else(|error| String::from_utf8_lossy(error.as_bytes()).into_owned())
        }))
    }

    /// Return the complete native config payload, including embedded NULs.
    pub fn try_config_data_bytes(&self) -> Result<Option<Vec<u8>>> {
        let mut bytes = Vec::new();
        crate::clear_last_error();
        let success = unsafe {
            ocio_sys::ocio_config_io_proxy_visit_config_data(
                self.handle.as_ptr(),
                (&mut bytes as *mut Vec<u8>).cast(),
                append_bytes,
            )
        };
        crate::ocio_call_status()?;
        Ok(success.then_some(bytes))
    }

    /// Inspect OCIO's native config bytes without an additional Rust copy.
    ///
    /// The slice is valid only during `visitor`; the visitor must not reenter
    /// this proxy. OCIO still constructs its own returned string. Empty and
    /// non-UTF-8 payloads are passed through unchanged, and visitor errors or
    /// panics are returned as errors.
    ///
    /// ```compile_fail
    /// # use ocio_rs::{ConfigIOProxy, Result};
    /// # fn example(proxy: &ConfigIOProxy) -> Result<()> {
    /// let borrowed = proxy.with_config_data(|data| Ok(data))?;
    /// println!("{:?}", borrowed);
    /// # Ok(()) }
    /// ```
    pub fn with_config_data<T>(&self, visitor: impl FnOnce(&[u8]) -> Result<T>) -> Result<T> {
        self.with_native_cstr(None, false, visitor)
    }

    /// Register the byte payload for a LUT file path and its fast hash.
    ///
    /// Returns whether the in-memory provider accepted the payload. Custom
    /// providers implement their own storage and reject this operation.
    /// Slash and backslash separators are treated as equivalent by this
    /// in-memory provider; registering an equivalent path replaces its payload
    /// and hash together. Custom providers receive OCIO's resolved path as-is.
    pub fn set_lut_data(
        &self,
        filepath: impl AsRef<str>,
        data: &[u8],
        fast_hash: impl AsRef<str>,
    ) -> Result<bool> {
        self.set_lut_data_with_hash_bytes(filepath, data, fast_hash.as_ref().as_bytes())
    }

    /// Register a LUT using a byte hash, including embedded NULs.
    /// An empty hash signals an unavailable file to OCIO, even if data is stored.
    pub fn set_lut_data_with_hash_bytes(
        &self,
        filepath: impl AsRef<str>,
        data: &[u8],
        fast_hash: &[u8],
    ) -> Result<bool> {
        let filepath = cstring(filepath)?;
        crate::clear_last_error();
        let accepted = unsafe {
            ocio_sys::ocio_config_io_proxy_set_lut_data_bytes(
                self.handle.as_ptr(),
                filepath.as_ptr(),
                data.as_ptr(),
                data.len(),
                fast_hash.as_ptr(),
                fast_hash.len(),
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
        let filepath = cstring(filepath)?;
        self.try_lut_data_cstr(&filepath)
    }

    /// Query a LUT using an existing native path without allocating a path string.
    pub fn try_lut_data_cstr(&self, filepath: &CStr) -> Result<Option<Vec<u8>>> {
        let mut bytes = Vec::new();
        crate::clear_last_error();
        let found = unsafe {
            ocio_sys::ocio_config_io_proxy_visit_lut_data(
                self.handle.as_ptr(),
                filepath.as_ptr(),
                (&mut bytes as *mut Vec<u8>).cast(),
                append_bytes,
            )
        };
        crate::ocio_call_status()?;
        Ok(found.then_some(bytes))
    }

    /// Call OCIO's native `getLutData` directly, without first checking the hash.
    /// An empty result may mean an empty LUT or an unavailable file; this
    /// method does not infer availability. Native OCIO returns an owned vector,
    /// whose bytes are copied into Rust-owned storage across the bridge.
    pub fn read_lut_data_cstr(&self, filepath: &CStr) -> Result<Vec<u8>> {
        self.with_lut_data_cstr(filepath, |bytes| Ok(bytes.to_vec()))
    }

    /// Call OCIO's native `getFastLutFileHash` directly, preserving empty and
    /// non-UTF-8 hashes without interpreting them as an `Option`.
    pub fn read_fast_lut_file_hash_cstr(&self, filepath: &CStr) -> Result<Vec<u8>> {
        self.with_fast_lut_file_hash_cstr(filepath, |bytes| Ok(bytes.to_vec()))
    }

    /// Inspect the native `getLutData` result without copying it into Rust.
    /// The slice is valid only during `visitor`, which must not reenter this
    /// proxy. OCIO still constructs the native owned result. Errors and panics
    /// are contained at the FFI boundary.
    ///
    /// ```compile_fail
    /// # use ocio_rs::{ConfigIOProxy, Result};
    /// # fn example(proxy: &ConfigIOProxy) -> Result<()> {
    /// let leaked = proxy.with_lut_data_cstr(c"lut.clf", |data| Ok(data))?;
    /// println!("{:?}", leaked);
    /// # Ok(()) }
    /// ```
    pub fn with_lut_data_cstr<T>(
        &self,
        filepath: &CStr,
        visitor: impl FnOnce(&[u8]) -> Result<T>,
    ) -> Result<T> {
        self.with_native_cstr(Some(filepath), false, visitor)
    }

    /// Inspect native hash bytes without copying them into Rust-owned storage.
    /// The same callback, lifetime, and reentrancy rules as [`Self::with_lut_data_cstr`] apply.
    pub fn with_fast_lut_file_hash_cstr<T>(
        &self,
        filepath: &CStr,
        visitor: impl FnOnce(&[u8]) -> Result<T>,
    ) -> Result<T> {
        self.with_native_cstr(Some(filepath), true, visitor)
    }

    fn with_native_cstr<T, F: FnOnce(&[u8]) -> Result<T>>(
        &self,
        filepath: Option<&CStr>,
        hash: bool,
        visitor: F,
    ) -> Result<T> {
        struct State<F, T> {
            visitor: Option<F>,
            result: Option<Result<T>>,
        }
        unsafe extern "C" fn visit<T, F: FnOnce(&[u8]) -> Result<T>>(
            context: *mut c_void,
            data: *const u8,
            len: usize,
        ) -> bool {
            let state = unsafe { &mut *context.cast::<State<F, T>>() };
            let Some(visitor) = state.visitor.take() else {
                return false;
            };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if len > isize::MAX as usize || (len != 0 && data.is_null()) {
                    return Err(OcioError::InvalidInput(
                        "invalid native proxy buffer".into(),
                    ));
                }
                let bytes = if len == 0 {
                    &[]
                } else {
                    unsafe { std::slice::from_raw_parts(data, len) }
                };
                visitor(bytes)
            }))
            .unwrap_or_else(|_| Err(OcioError::Ocio("native proxy visitor panicked".into())));
            let ok = result.is_ok();
            state.result = Some(result);
            ok
        }
        let mut state = State {
            visitor: Some(visitor),
            result: None,
        };
        crate::clear_last_error();
        let ok = unsafe {
            match filepath {
                Some(filepath) => ocio_sys::ocio_config_io_proxy_read_native(
                    self.handle.as_ptr(),
                    filepath.as_ptr(),
                    hash,
                    (&mut state as *mut State<F, T>).cast(),
                    visit::<T, F>,
                ),
                None => ocio_sys::ocio_config_io_proxy_visit_config_data(
                    self.handle.as_ptr(),
                    (&mut state as *mut State<F, T>).cast(),
                    visit::<T, F>,
                ),
            }
        };
        if state.result.as_ref().is_some_and(Result::is_err) {
            return state.result.take().expect("checked visitor result");
        }
        crate::ocio_call_status()?;
        if !ok {
            return Err(OcioError::Ocio("native proxy read failed".into()));
        }
        state
            .result
            .ok_or_else(|| OcioError::Ocio("native proxy visitor was not called".into()))?
    }

    /// Copy the LUT payload into `data`, returning whether the copy succeeded.
    ///
    /// Returns `Ok(true)` if data was copied, `Ok(false)` if the LUT was not
    /// found or the buffer is too small, and `Err` on internal errors.
    pub fn try_copy_lut_data(&self, filepath: impl AsRef<str>, data: &mut [u8]) -> Result<bool> {
        let filepath = cstring(filepath)?;
        self.try_copy_lut_data_cstr(&filepath, data)
    }

    /// Copy a LUT to caller-owned storage using an existing native path.
    pub fn try_copy_lut_data_cstr(&self, filepath: &CStr, data: &mut [u8]) -> Result<bool> {
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

    /// Return a text hash, replacing invalid UTF-8. For lossless bytes use
    /// [`Self::try_fast_lut_file_hash_bytes`].
    pub fn try_fast_lut_file_hash(&self, filepath: impl AsRef<str>) -> Result<Option<String>> {
        Ok(self
            .try_fast_lut_file_hash_bytes(filepath)?
            .map(|value| String::from_utf8_lossy(&value).into_owned()))
    }

    /// Return the complete native hash bytes, including any embedded NULs.
    /// An empty native hash indicates that the LUT is unavailable.
    pub fn try_fast_lut_file_hash_bytes(
        &self,
        filepath: impl AsRef<str>,
    ) -> Result<Option<Vec<u8>>> {
        let filepath = cstring(filepath)?;
        self.try_fast_lut_file_hash_bytes_cstr(&filepath)
    }

    /// Query hash bytes using an existing native path without allocating it.
    pub fn try_fast_lut_file_hash_bytes_cstr(&self, filepath: &CStr) -> Result<Option<Vec<u8>>> {
        let mut bytes = Vec::new();
        crate::clear_last_error();
        let found = unsafe {
            ocio_sys::ocio_config_io_proxy_visit_fast_lut_file_hash(
                self.handle.as_ptr(),
                filepath.as_ptr(),
                (&mut bytes as *mut Vec<u8>).cast(),
                append_bytes,
            )
        };
        crate::ocio_call_status()?;
        Ok(found.then_some(bytes))
    }

    #[doc(hidden)]
    pub fn get_lut_data_size(&self, filepath: impl AsRef<str>) -> u64 {
        self.try_get_lut_data_size(filepath).unwrap_or(0)
    }

    /// Return the size of the registered LUT payload, or 0 if not found.
    pub fn try_get_lut_data_size(&self, filepath: impl AsRef<str>) -> Result<u64> {
        let filepath = cstring(filepath)?;
        self.try_get_lut_data_size_cstr(&filepath)
    }

    /// Query LUT size using an existing native path without allocating it.
    pub fn try_get_lut_data_size_cstr(&self, filepath: &CStr) -> Result<u64> {
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
        self.try_has_lut_data_cstr(&filepath)
    }

    /// Query LUT presence using an existing native path without allocating it.
    pub fn try_has_lut_data_cstr(&self, filepath: &CStr) -> Result<bool> {
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
    fn owned_payload_callback_rejects_invalid_buffer_without_mutation() {
        let mut values = vec![7_u8];
        let destination = (&mut values as *mut Vec<u8>).cast();
        unsafe {
            assert!(!append_bytes(std::ptr::null_mut(), std::ptr::null(), 0));
            assert!(!append_bytes(destination, std::ptr::null(), 1));
            assert!(!append_bytes(
                destination,
                std::ptr::NonNull::<u8>::dangling().as_ptr(),
                isize::MAX as usize + 1
            ));
            assert!(append_bytes(destination, std::ptr::null(), 0));
            assert_eq!(values, [7]);
            assert!(append_bytes(destination, [1_u8, 2].as_ptr(), 2));
        }
        assert_eq!(values, [7, 1, 2]);
    }

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
