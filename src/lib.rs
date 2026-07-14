//! Safe Rust bindings for [OpenColorIO](https://opencolorio.org/) (OCIO v2.5).
//!
//! This crate wraps the OpenColorIO C++ API through a generated FFI layer
//! ([`ocio_sys`]) and exposes safe Rust types for configuration, color-space
//! queries, processor creation, CPU/GPU execution, GPU shader extraction, and
//! transform composition.
//!
//! # Getting started
//!
//! ```rust,no_run
//! use ocio_rs::Config;
//!
//! // Load an OCIO config from a file (real OCIO mode required).
//! # fn example() -> ocio_rs::Result<()> {
//! let config = Config::from_file("/path/to/config.ocio")?;
//!
//! // Create a processor between two color spaces.
//! let processor = config.processor("scene_linear", "srgb_texture")?;
//!
//! // Apply the default CPU path to a single RGBA pixel.
//! let mut pixel = [0.5f32, 0.5, 0.5, 1.0];
//! processor.default_cpu_processor()?.apply_rgba(&mut pixel);
//! # Ok(())
//! # }
//! ```
//!
//! # Project status
//!
//! This crate targets broad OpenColorIO 2.5 coverage. The low-level FFI and
//! safe-wrapper layers are available for most of the exposed OCIO 2.5 surface,
//! while release hardening and long-tail behavioral validation are still in
//! progress.
//!
//! # Build modes
//!
//! | Mode | Feature flag | Notes |
//! |------|-------------|-------|
//! | **Stub** (default) | `--no-default-features` | APIs return safe defaults; no real color management |
//! | **Real OCIO** | `OCIO_RS_ENABLE_REAL=1` | Links against an installed OpenColorIO |
//! | **Bundled** | `--features bundled` | Builds OCIO from source and links statically |
//!
//! ```bash
//! # Stub mode (CI, API-shape testing)
//! cargo build
//!
//! # Bundled real-OCIO build
//! cargo build --features bundled
//!
//! # Use an installed OCIO
//! OCIO_RS_ENABLE_REAL=1 OCIO_INSTALL_DIR=/path/to/ocio cargo build
//! ```
//!
//! # Crate-level helpers
//!
//! Free functions such as [`current_config`], [`set_current_config`],
//! [`version`], and [`clear_all_caches`] operate on the process-global OCIO
//! config. Use them when your application maintains a single shared config
//! rather than passing `Config` handles through every call site.
//!
//! [`ocio_sys`]: crate::ocio_sys

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    unused_imports,
    clippy::single_component_path_imports,
    clippy::unnecessary_cast
)]

mod baker;
mod builtin_config_registry;
mod builtin_transform_registry;
mod color_space_set;
mod colorspace;
mod config;
mod config_io_proxy;
mod context;
mod file_rules;
mod format_metadata;
pub mod grading;
mod look;
mod named_transform;
mod processor;
mod processor_metadata;
pub mod transform;
mod types;
mod view_transform;
mod viewing_rules;

pub use baker::Baker;
pub use builtin_config_registry::BuiltinConfigRegistry;
pub use builtin_transform_registry::BuiltinTransformRegistry;
pub use color_space_set::ColorSpaceSet;
pub use colorspace::ColorSpace;
pub use config::Config;
pub use config_io_proxy::ConfigIOProxy;
pub use context::Context;
pub use file_rules::FileRules;
pub use format_metadata::FormatMetadata;
pub use look::Look;
pub use named_transform::NamedTransform;
pub use processor::{
    CPUProcessor, DynamicProperty, GPUProcessor, GpuShaderDesc, GpuTexture2D, GpuTexture3D,
    GpuTextureChannel, GpuTextureDimensions, GpuUniform, GpuUniformType, GpuUniformValue,
    Processor, TextureInfo,
};
pub use processor_metadata::ProcessorMetadata;
pub use types::*;
pub use view_transform::ViewTransform;
pub use viewing_rules::ViewingRules;

use std::ffi::{c_void, CString};
use std::ptr::NonNull;
use thiserror::Error;

pub use ocio_sys;

#[derive(Debug, Error)]
/// Unified error type for safe `ocio-rs` operations.
///
/// Most bridge failures arrive here either as a propagated OpenColorIO runtime
/// error string or as an input/ownership validation error detected on the Rust
/// side before the C++ bridge is called.
pub enum OcioError {
    #[error("OpenColorIO error: {0}")]
    Ocio(String),
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("path contains interior NUL byte")]
    InteriorNul,
    #[error("OpenColorIO handle allocation failed")]
    AllocationFailed,
}

/// Crate-local result alias used by the safe wrapper layer.
pub type Result<T> = std::result::Result<T, OcioError>;

/// Returns `true` when the crate was built in stub mode instead of linking a
/// real OpenColorIO implementation.
///
/// Stub mode is useful for API-shape validation and lightweight CI, but it
/// does not perform real color processing.
pub fn is_stub_build() -> bool {
    unsafe { ocio_sys::ocio_runtime_is_stub() }
}

/// Returns the process-global current OCIO config, if one is installed.
///
/// The returned handle follows OCIO's shared ownership semantics and can be
/// queried or copied like any other `Config`. This compatibility helper
/// returns `None` both when no config is installed and when OCIO reports an
/// error. Use [`try_current_config`] when those cases must be distinguished.
///
/// # Threading
/// These process-global helpers do not synchronize OCIO global state. Serialize
/// calls that mutate the current config, global caches, or logging level with
/// other application threads that use the same OCIO process.
pub fn current_config() -> Option<Config> {
    try_current_config().ok().flatten()
}

/// Try to get the process-global current OCIO config.
///
/// Returns `Ok(None)` when no config is installed, and surfaces OCIO bridge
/// errors separately.
pub fn try_current_config() -> Result<Option<Config>> {
    clear_last_error();
    let handle = unsafe { ocio_sys::ocio_get_current_config() };
    ocio_call_status()?;
    Ok(std::ptr::NonNull::new(handle).map(|handle| Config { handle }))
}

#[deprecated(since = "0.2.0", note = "compat alias; prefer current_config()")]
pub fn get_current_config() -> Option<Config> {
    current_config()
}

/// Installs `config` as the process-global current OCIO config.
///
/// This mutates process-global OCIO state; see [`current_config`] for threading
/// requirements.
#[deprecated(
    since = "0.2.0",
    note = "discarded OCIO errors; prefer try_set_current_config()"
)]
pub fn set_current_config(config: &Config) {
    let _ = try_set_current_config(config);
}

/// Try to install `config` as the process-global current OCIO config.
pub fn try_set_current_config(config: &Config) -> Result<()> {
    clear_last_error();
    unsafe { ocio_sys::ocio_set_current_config(config.handle.as_ptr()) };
    ocio_call_status()
}

/// Clears OCIO's process-global caches.
///
/// This is mainly useful in tests and tooling that intentionally mutate config
/// state and want subsequent processor creation to observe the new values.
/// Serialize this operation with other application threads using OCIO globals.
#[deprecated(
    since = "0.2.0",
    note = "discarded OCIO errors; prefer try_clear_all_caches()"
)]
pub fn clear_all_caches() {
    let _ = try_clear_all_caches();
}

/// Try to clear OCIO's process-global caches.
pub fn try_clear_all_caches() -> Result<()> {
    clear_last_error();
    unsafe { ocio_sys::ocio_clear_all_caches() };
    ocio_call_status()
}

/// Returns the linked OpenColorIO version string, if available.
pub fn version() -> Option<String> {
    unsafe { cstr_to_opt_string(ocio_sys::ocio_get_version()) }
}

/// Returns the linked OpenColorIO version as an integer hex constant.
pub fn version_hex() -> i32 {
    unsafe { ocio_sys::ocio_get_version_hex() }
}

/// Returns the current OCIO global logging level.
pub fn logging_level() -> crate::LoggingLevel {
    let l = unsafe { ocio_sys::ocio_get_logging_level() };
    match l {
        0 => crate::LoggingLevel::None,
        1 => crate::LoggingLevel::Warning,
        2 => crate::LoggingLevel::Info,
        3 => crate::LoggingLevel::Debug,
        4 => crate::LoggingLevel::Trace,
        _ => crate::LoggingLevel::None,
    }
}

/// Sets the OCIO global logging level.
///
/// Serialize this process-global mutation with other application threads using
/// OCIO globals.
#[deprecated(
    since = "0.2.0",
    note = "discarded OCIO errors; prefer try_set_logging_level()"
)]
pub fn set_logging_level(level: crate::LoggingLevel) {
    let _ = try_set_logging_level(level);
}

/// Try to set the OCIO global logging level.
pub fn try_set_logging_level(level: crate::LoggingLevel) -> Result<()> {
    clear_last_error();
    unsafe { ocio_sys::ocio_set_logging_level(level as i32) };
    ocio_call_status()
}

/// Resolve an OCIO configuration path to its current persistent form.
///
/// In particular, this replaces special built-in config aliases such as
/// `ocio://default` with the versioned name selected by the linked OCIO build.
/// The returned string is owned by Rust and may be stored independently of OCIO.
pub fn resolve_config_path(original_path: impl AsRef<str>) -> Result<String> {
    let original_path = cstring(original_path)?;
    clear_last_error();
    let resolved =
        unsafe { cstr_to_opt_string(ocio_sys::ocio_resolve_config_path(original_path.as_ptr())) };
    ocio_call_status()?;
    resolved
        .ok_or_else(|| OcioError::Ocio("OpenColorIO returned a null resolved config path".into()))
}

/// Extract an `.ocioz` archive into a destination directory.
///
/// Both paths must be absolute according to OCIO's contract. The destination directory is
/// created by OCIO along with `config.ocio` and the archive's referenced LUT assets.
///
/// This operation requires a real OCIO build. Stub mode returns an [`OcioError`] rather than
/// pretending that archive extraction succeeded.
pub fn extract_ocioz_archive(
    archive_path: impl AsRef<str>,
    destination_dir: impl AsRef<str>,
) -> Result<()> {
    let archive_path = cstring(archive_path)?;
    let destination_dir = cstring(destination_dir)?;
    clear_last_error();
    unsafe {
        ocio_sys::ocio_extract_ocioz_archive(archive_path.as_ptr(), destination_dir.as_ptr());
    }
    ocio_call_status()
}

/// Compatibility alias for overriding the OCIO global logging level.
#[deprecated(
    since = "0.2.0",
    note = "discarded OCIO errors; prefer try_set_logging_level()"
)]
pub fn set_logging_level_to_override(level: crate::LoggingLevel) {
    let _ = try_set_logging_level(level);
}

/// Returns the processor cache flags from the current global config.
///
/// If no current config is installed, this returns an empty flag set.
pub fn processor_cache_flags() -> crate::ProcessorCacheFlags {
    current_config()
        .map(|config| crate::ProcessorCacheFlags(config.processor_cache_flags() as u32))
        .unwrap_or(crate::ProcessorCacheFlags(0))
}

/// Updates the processor cache flags on the current global config, if any.
#[deprecated(
    since = "0.2.0",
    note = "discarded OCIO errors; prefer try_set_processor_cache_flags()"
)]
pub fn set_processor_cache_flags(flags: crate::ProcessorCacheFlags) {
    let _ = try_set_processor_cache_flags(flags);
}

/// Try to update processor-cache flags on the current global config.
///
/// This returns `Ok(())` when no global config is installed.
pub fn try_set_processor_cache_flags(flags: crate::ProcessorCacheFlags) -> Result<()> {
    if let Some(config) = try_current_config()? {
        config.try_set_processor_cache_flags(flags.0 as i32)?;
    }
    Ok(())
}

pub(crate) fn cstring(value: impl AsRef<str>) -> Result<CString> {
    CString::new(value.as_ref()).map_err(|_| OcioError::InteriorNul)
}

pub(crate) unsafe fn cstr_to_opt_string(ptr: *const i8) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned())
    }
}

// v2.5.1 compat: accept *mut c_void (new return type for many getter functions)
pub(crate) unsafe fn cstr_from_mut(ptr: *mut c_void) -> Option<String> {
    cstr_to_opt_string(ptr as *const i8)
}

pub(crate) fn clear_last_error() {
    unsafe { ocio_sys::ocio_error_clear_last() };
}

pub(crate) fn last_error_message() -> Option<String> {
    unsafe { cstr_to_opt_string(ocio_sys::ocio_error_get_last()) }
}

/// Read and clear the bridge's thread-local error slot.
///
/// Each safe wrapper clears the slot before calling into the bridge. Consuming
/// an error here prevents a handled failure from being observed by a later FFI
/// call on the same thread.
pub(crate) fn take_last_error_message() -> Option<String> {
    let message = last_error_message();
    if message.is_some() {
        clear_last_error();
    }
    message
}

pub(crate) fn validation_status() -> Result<()> {
    match take_last_error_message() {
        Some(message) => Err(OcioError::ValidationFailed(message)),
        None => Ok(()),
    }
}

pub(crate) fn ocio_call_status() -> Result<()> {
    match take_last_error_message() {
        Some(message) => Err(OcioError::Ocio(message)),
        None => Ok(()),
    }
}

pub(crate) fn handle_result(handle: *mut c_void) -> Result<NonNull<c_void>> {
    match NonNull::new(handle) {
        Some(handle) => Ok(handle),
        None => match take_last_error_message() {
            Some(message) => Err(OcioError::Ocio(message)),
            None => Err(OcioError::AllocationFailed),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processor_cache_flags_no_crash() {
        let f = processor_cache_flags();
        assert!(try_set_processor_cache_flags(f).is_ok());
    }

    #[test]
    fn current_config_no_crash() {
        let _ = current_config();
    }

    #[test]
    fn try_current_config_no_crash() {
        let _ = try_current_config();
    }

    #[cfg(feature = "bundled")]
    #[test]
    fn bridge_errors_are_consumed_after_handle_conversion() {
        let path = cstring("this-file-does-not-exist.ocio").expect("valid test path");
        clear_last_error();
        let handle = unsafe { ocio_sys::ocio_config_create_from_file(path.as_ptr().cast()) };

        assert!(matches!(handle_result(handle), Err(OcioError::Ocio(_))));
        assert!(last_error_message().is_none());
    }

    #[cfg(feature = "bundled")]
    #[test]
    fn bridge_errors_are_consumed_after_validation() {
        let transform = crate::transform::FileTransform::create().expect("file transform create");

        assert!(matches!(
            transform.validate(),
            Err(OcioError::ValidationFailed(_))
        ));
        assert!(last_error_message().is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn current_config_compat_alias_no_crash() {
        let _ = get_current_config();
    }
}
