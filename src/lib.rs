mod baker;
mod builtin_config_registry;
mod color_space_set;
mod colorspace;
mod config;
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
pub use color_space_set::ColorSpaceSet;
pub use colorspace::ColorSpace;
pub use config::Config;
pub use context::Context;
pub use file_rules::FileRules;
pub use format_metadata::FormatMetadata;
pub use look::Look;
pub use named_transform::NamedTransform;
pub use processor::{CPUProcessor, GPUProcessor, GpuShaderDesc, DynamicProperty, Processor, TextureInfo};
pub use types::*;
pub use view_transform::ViewTransform;

use std::ffi::CString;
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

pub fn get_current_config() -> Option<Config> {
    let handle = unsafe { ocio_sys::ocio_get_current_config() };
    if handle.is_null() {
        None
    } else {
        Some(Config { handle: std::ptr::NonNull::new(handle).unwrap() })
    }
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
    unsafe { ocio_sys::ocio_set_logging_level_to_override(level as i32) };
}

pub fn processor_cache_flags() -> crate::ProcessorCacheFlags {
    let flags = unsafe { ocio_sys::ocio_get_processor_cache_flags() };
    crate::ProcessorCacheFlags(flags as u32)
}

pub fn set_processor_cache_flags(flags: crate::ProcessorCacheFlags) {
    unsafe { ocio_sys::ocio_set_processor_cache_flags(flags.0 as i32) };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processor_cache_flags_no_crash() {
        let f = processor_cache_flags();
        set_processor_cache_flags(f);
    }
}
