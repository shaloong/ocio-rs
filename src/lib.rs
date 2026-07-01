//! Rust bindings for OpenColorIO.
//!
//! # Project status
//!
//! This crate targets broad OpenColorIO 2.5 coverage. The low-level FFI and
//! safe-wrapper layers are available for most of the exposed OCIO 2.5 surface,
//! while release hardening and long-tail behavioral validation are still in
//! progress.
//!
//! # Stub mode
//!
//! By default, the crate builds in stub mode for CI and API-shape testing.
//! Stub mode does not perform real color management.
//!
//! # Real OCIO mode
//!
//! Use `OCIO_RS_ENABLE_REAL=1` with `OCIO_INSTALL_DIR`, or build with
//! `--features bundled`.
//!
//! The published `ocio-sys` crate vendors the upstream OpenColorIO source tree
//! used by bundled builds. However, the upstream bundled CMake flow still
//! downloads several transitive dependency sources during configure/build, so
//! packaged bundled builds are not yet fully offline/self-contained.

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
pub mod transform;
mod types;
mod view_transform;

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
pub use types::*;
pub use view_transform::ViewTransform;

use std::ffi::{c_void, CString};
use thiserror::Error;

pub use ocio_sys;

#[derive(Debug, Error)]
pub enum OcioError {
    #[error("OpenColorIO error: {0}")]
    Ocio(String),
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    #[error("path contains interior NUL byte")]
    InteriorNul,
    #[error("OpenColorIO handle allocation failed")]
    AllocationFailed,
}

pub type Result<T> = std::result::Result<T, OcioError>;

pub fn is_stub_build() -> bool {
    unsafe { ocio_sys::ocio_runtime_is_stub() }
}

pub fn current_config() -> Option<Config> {
    let handle = unsafe { ocio_sys::ocio_get_current_config() };
    if handle.is_null() {
        None
    } else {
        Some(Config {
            handle: std::ptr::NonNull::new(handle).unwrap(),
        })
    }
}

#[deprecated(since = "0.2.0", note = "compat alias; prefer current_config()")]
pub fn get_current_config() -> Option<Config> {
    current_config()
}

pub fn set_current_config(config: &Config) {
    unsafe { ocio_sys::ocio_set_current_config(config.handle.as_ptr()) };
}

pub fn clear_all_caches() {
    unsafe { ocio_sys::ocio_clear_all_caches() };
}

pub fn version() -> Option<String> {
    unsafe { cstr_to_opt_string(ocio_sys::ocio_get_version()) }
}

pub fn version_hex() -> i32 {
    unsafe { ocio_sys::ocio_get_version_hex() }
}

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

pub fn set_logging_level(level: crate::LoggingLevel) {
    unsafe { ocio_sys::ocio_set_logging_level(level as i32) };
}

pub fn set_logging_level_to_override(level: crate::LoggingLevel) {
    unsafe { ocio_sys::ocio_set_logging_level(level as i32) };
}

pub fn processor_cache_flags() -> crate::ProcessorCacheFlags {
    current_config()
        .map(|config| crate::ProcessorCacheFlags(config.processor_cache_flags() as u32))
        .unwrap_or(crate::ProcessorCacheFlags(0))
}

pub fn set_processor_cache_flags(flags: crate::ProcessorCacheFlags) {
    if let Some(config) = current_config() {
        config.set_processor_cache_flags(flags.0 as i32);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processor_cache_flags_no_crash() {
        let f = processor_cache_flags();
        set_processor_cache_flags(f);
    }

    #[test]
    fn current_config_no_crash() {
        let _ = current_config();
    }

    #[test]
    #[allow(deprecated)]
    fn current_config_compat_alias_no_crash() {
        let _ = get_current_config();
    }
}
