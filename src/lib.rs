mod baker;
mod colorspace;
mod config;
mod context;
pub mod grading;
mod look;
mod named_transform;
mod processor;
pub mod transform;
mod types;
mod view_transform;

pub use baker::Baker;
pub use colorspace::ColorSpace;
pub use config::Config;
pub use context::Context;
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
