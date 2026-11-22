//! Safe Rust bindings for [OpenColorIO](https://opencolorio.org/) (OCIO 2.4.1 through 2.6.x).
//!
//! This crate wraps the OpenColorIO C++ API through a generated FFI layer
//! ([`ocio_sys`]) and exposes safe Rust types for configuration, color-space
//! queries, processor creation, CPU/GPU execution, GPU shader extraction, and
//! transform composition.
//!
//! # OCIO version support
//!
//! The baseline API works with OpenColorIO >= 2.4.1 and < 2.7. API added in newer
//! OCIO versions is opt-in through additive `vX_Y` cargo features (e.g.
//! `v2_5` for hue-curve grading, interchange attributes, and GPU uniform
//! buffers); enabling one raises the OpenColorIO version requirement checked
//! at build time.
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
//! | **Auto** (default) | `auto` | Probe system OCIO, otherwise build bundled source |
//! | **Stub** | `--no-default-features --features stub` | No real color management; testing only |
//! | **System** | `--features system` | Requires installed OpenColorIO and pkg-config |
//! | **Bundled** | `--features bundled` | Builds vendored OCIO; static linking by default |
//!
//! Set `OCIO_RS_LINK=dynamic` for shared linking. With default features
//! disabled and no backend feature, a system library is required.
//!
//! ```bash
//! # Stub mode (CI, API-shape testing)
//! cargo build --no-default-features --features stub
//!
//! # Bundled real-OCIO build
//! cargo build --features bundled
//!
//! # Use an installed OCIO
//! OCIO_INSTALL_DIR=/path/to/ocio cargo build --features system
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

mod app_helpers;
mod spline;
mod system_monitors;
#[cfg(feature = "v2_5")]
pub use spline::BSplineType;
pub use spline::{
    ConstGradingBSplineCurve, GradingBSplineCurve, GradingRGBCurve, SharedGradingBSplineCurve,
    SharedGradingRGBCurve,
};
#[cfg(feature = "v2_5")]
pub use spline::{GradingHueCurve, SharedGradingHueCurve};
pub use system_monitors::SystemMonitors;
mod baker;
pub use app_helpers::{
    ColorSpaceMenuHelper, ColorSpaceMenuParameters, DisplayViewFromFile, LegacyViewingPipeline,
    LegacyViewingStage, MixingColorSpaceManager, MixingSlider,
};
mod builtin_config_registry;
mod builtin_transform_registry;
mod color_space_set;
mod colorspace;
mod config;
#[cfg(feature = "v2_5")]
mod config_merging;
#[cfg(feature = "v2_5")]
pub use config_merging::{ConfigMerger, ConfigMergingParameters, ConstConfigMerger, MergeStrategy};
mod config_io_proxy;
mod const_config_item;
mod const_rules;
pub mod constants;
mod context;
mod enum_text;
mod file_rules;
mod format_metadata;
pub mod grading;
mod image_desc;
mod look;
mod named_transform;
mod output_writer;
mod processor;
mod processor_metadata;
pub mod transform;
mod types;
mod view_transform;
mod viewing_rules;

pub use baker::Baker;
pub use builtin_config_registry::BuiltinConfigRegistry;
pub use builtin_transform_registry::BuiltinTransformRegistry;
pub use color_space_set::{ColorSpaceSet, ColorSpaceSetRef, ConstColorSpaceSet};
pub use colorspace::{ColorSpace, ColorSpaceRef, ConstColorSpace};
pub use config::{Config, ConfigProcessorTarget, ConfigRef, ConstConfig};
pub use config_io_proxy::{ConfigIOProvider, ConfigIOProxy};
pub use const_config_item::{ConstLook, ConstNamedTransform, ConstViewTransform};
pub use const_rules::{ConstFileRules, ConstViewingRules};
pub use context::{ConstContext, Context};
pub use enum_text::OcioEnumText;
pub use file_rules::FileRules;
pub use format_metadata::{ConstFormatMetadata, FormatMetadata};
pub use image_desc::{
    CustomImageLayout, ImageDescChannel, ImageDescProvider, NativeImageDataPointers,
    NativeImageDesc, NativeImageDescInfo, NativeImageLayout,
};
pub use look::Look;
pub use named_transform::NamedTransform;
pub use processor::{
    CPUProcessor, DynamicProperty, GPUProcessor, GpuShaderCreator, GpuShaderDesc,
    GpuShaderDescView, GpuTexture2D, GpuTexture3D, GpuTextureChannel, GpuTextureDimensions,
    GpuUniform, GpuUniformCache, GpuUniformCopy, GpuUniformType, GpuUniformValue,
    PackedConversionLayout, PackedImageLayout, PlanarByteLayout, PlanarBytes, PlanarBytesMut,
    PlanarF32Layout, Processor, SignedPlanarByteLayout, SignedPlanarBytes, SignedPlanarBytesMut,
    StaticCPUProcessor, TextureInfo,
};
pub use processor::{
    ShaderCodeSection, ShaderCreatorOptions, ShaderExtractionHooks, ShaderFinalizeControl,
    ShaderLifecycle, ShaderSections, ShaderTexture, ShaderTextureAction, ShaderTexturePolicy,
    ShaderUniform, ShaderUniformAction, ShaderUniformControl, ShaderUniformGetter,
    ShaderUniformRegistration,
};
pub use processor_metadata::{ConstProcessorMetadata, ProcessorMetadata};
pub use types::*;
#[cfg(feature = "v2_6")]
mod v2_6;
#[cfg(feature = "v2_6")]
pub use v2_6::{metadata_id_element, ViewWithAliases};
pub use view_transform::ViewTransform;
pub use viewing_rules::ViewingRules;

use std::cell::RefCell;
use std::ffi::{c_void, CStr, CString};
use std::ptr::NonNull;
use std::sync::{Arc, Mutex, OnceLock};
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
    /// Native `ExceptionMissingFile`: an unavailable file or lookup result,
    /// such as a missing correction in an existing CDL collection.
    #[error("OpenColorIO file not found: {0}")]
    MissingFile(String),
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

type LoggingCallback = dyn Fn(&str) + Send + Sync + 'static;
type ComputeHashCallback = dyn Fn(&[u8]) -> Vec<u8> + Send + Sync + 'static;

thread_local! {
    static COMPUTE_HASH_OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static IN_LOGGING_CALLBACK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn check_logging_reentrancy() -> Result<()> {
    if IN_LOGGING_CALLBACK.with(std::cell::Cell::get) {
        return Err(OcioError::InvalidInput(
            "OCIO logging APIs cannot be called from an OCIO logging callback".into(),
        ));
    }
    Ok(())
}

fn logging_callback_slot() -> &'static Mutex<Option<Arc<LoggingCallback>>> {
    static SLOT: OnceLock<Mutex<Option<Arc<LoggingCallback>>>> = OnceLock::new();
    SLOT.get_or_init(|| Mutex::new(None))
}

fn logging_callback_update_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn compute_hash_callback_slot() -> &'static Mutex<Option<Arc<ComputeHashCallback>>> {
    static SLOT: OnceLock<Mutex<Option<Arc<ComputeHashCallback>>>> = OnceLock::new();
    SLOT.get_or_init(|| Mutex::new(None))
}

fn compute_hash_callback_update_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

unsafe extern "C" fn rust_logging_callback(message: *const std::ffi::c_char) {
    if message.is_null() {
        return;
    }

    let callback = logging_callback_slot()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone();
    if let Some(callback) = callback {
        let message = unsafe { CStr::from_ptr(message).to_string_lossy() };
        IN_LOGGING_CALLBACK.with(|active| {
            struct Reset<'a>(&'a std::cell::Cell<bool>);
            impl Drop for Reset<'_> {
                fn drop(&mut self) {
                    self.0.set(false);
                }
            }
            active.set(true);
            let _reset = Reset(active);
            let _ = catch_callback_unwind(std::panic::AssertUnwindSafe(|| callback(&message)));
        });
    }
}

unsafe extern "C" fn rust_compute_hash_callback(
    input: *const u8,
    input_len: usize,
    output: *mut *const u8,
    output_len: *mut usize,
) -> bool {
    if (input.is_null() && input_len != 0) || output.is_null() || output_len.is_null() {
        return false;
    }
    let callback = compute_hash_callback_slot()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone();
    let Some(callback) = callback else {
        return false;
    };
    let input = if input_len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(input, input_len) }
    };
    let Ok(value) = catch_callback_unwind(std::panic::AssertUnwindSafe(|| callback(input))) else {
        return false;
    };
    COMPUTE_HASH_OUTPUT.with(|slot| {
        let mut slot = slot.borrow_mut();
        *slot = value;
        unsafe {
            *output = slot.as_ptr();
            *output_len = slot.len();
        }
    });
    true
}

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
/// The returned config is an editable snapshot with group-transform children
/// detached from the installed config. This helper
/// returns `None` both when no config is installed and when OCIO reports an
/// error. Use [`try_current_config`] when those cases must be distinguished.
///
/// # Safety
/// The caller must meet the synchronization requirements of [`try_current_config`].
///
/// ```compile_fail
/// let _ = ocio_rs::current_config();
/// ```
pub unsafe fn current_config() -> Option<Config> {
    unsafe { try_current_config() }.ok().flatten()
}

/// Try to get the process-global current OCIO config.
///
/// Returns `Ok(None)` when no config is installed, and surfaces OCIO bridge
/// errors separately.
///
/// # Safety
/// During this call, the caller must prevent independent native code, including
/// other libraries, from accessing any config that may be current or mutating
/// objects retained by it. OCIO const operations such as cache-ID queries can
/// mutate lazy caches. Concurrent snapshot calls from this crate are permitted
/// when this condition holds.
///
/// OCIO copies can retain external objects, such as a native config IO proxy or
/// state owned by a custom transform. For as long as the snapshot, any object
/// derived from it, or a config subsequently installed from it may use such
/// objects, the caller must ensure those objects support concurrent use or
/// coordinate all other native access. Rust-managed [`ConfigIOProxy`] providers
/// are required to be thread-safe.
///
/// ```compile_fail
/// let _ = ocio_rs::try_current_config(); // Requires explicit synchronization.
/// ```
pub unsafe fn try_current_config() -> Result<Option<Config>> {
    clear_last_error();
    let handle = unsafe { ocio_sys::ocio_get_isolated_current_config() };
    ocio_call_status()?;
    Ok(std::ptr::NonNull::new(handle).map(|handle| Config { handle }))
}

/// Return OCIO's native editable copy of the process-global config.
///
/// Unlike [`try_current_config`], this does not detach group-transform
/// children; OCIO's native copy retains them. It avoids that extra graph copy
/// when callers can coordinate the resulting shared objects.
///
/// # Safety
/// The caller must meet the synchronization requirements of [`try_current_config`]
/// while this function copies the global config. For the lifetime of the
/// returned config and every derived handle, callers must also prevent
/// conflicting access through all aliases to shared group children. This
/// includes native aliases and other snapshots derived from the global config.
/// OCIO may additionally retain external IO proxies and custom-transform state;
/// the lifetime-wide requirements in [`try_current_config`] apply to them.
/// Native processor caches are not automatically invalidated by edits through
/// a shared transform alias.
///
/// ```compile_fail
/// let _ = ocio_rs::try_current_config_native_copy();
/// ```
pub unsafe fn try_current_config_native_copy() -> Result<Option<Config>> {
    clear_last_error();
    let handle = unsafe { ocio_sys::ocio_get_current_config() };
    ocio_call_status()?;
    Ok(std::ptr::NonNull::new(handle).map(|handle| Config { handle }))
}

/// Return a separate read-only snapshot of the process-global config.
///
/// This makes a native config copy: OCIO's copy operation reads mutable lazy
/// caches, so exposing the global object itself to safe callers on multiple
/// threads would make concurrent reads and copies unsound. Reuse the snapshot
/// for repeated operations. Group children are detached from the global config;
/// further ordinary OCIO copies may share their own group children.
/// The snapshot remains alive when another config is installed.
///
/// # Safety
/// The caller must meet the synchronization requirements of [`try_current_config`]
/// for the duration of this call.
///
/// ```compile_fail
/// let _ = ocio_rs::try_const_current_config();
/// ```
pub unsafe fn try_const_current_config() -> Result<Option<ConstConfig>> {
    clear_last_error();
    let handle = unsafe { ocio_sys::ocio_get_isolated_const_current_config() };
    ocio_call_status()?;
    Ok(std::ptr::NonNull::new(handle).map(|handle| ConstConfig { handle }))
}

/// Retain the native global config without copying it.
///
/// # Safety
/// The caller must serialize all access to the retained native config and its
/// aliases and to any shared native objects with all other OCIO users, including
/// global snapshot getters, for the lifetime of this handle and derived handles.
/// OCIO may mutate lazy caches even for const operations; copying it while a
/// different thread uses those caches can also race.
pub unsafe fn try_const_current_config_shared() -> Result<Option<ConstConfig>> {
    clear_last_error();
    let handle = unsafe { ocio_sys::ocio_get_const_current_config() };
    ocio_call_status()?;
    Ok(std::ptr::NonNull::new(handle).map(|handle| ConstConfig { handle }))
}

#[deprecated(since = "0.2.0", note = "compat alias; prefer current_config()")]
/// # Safety
/// The caller must meet the synchronization requirements of [`try_current_config`].
///
/// ```compile_fail
/// # #[allow(deprecated)]
/// let _ = ocio_rs::get_current_config();
/// ```
pub unsafe fn get_current_config() -> Option<Config> {
    unsafe { current_config() }
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
///
/// Group-transform graphs are detached from the input before installation so
/// mutable input children cannot be shared with other threads via the global.
pub fn try_set_current_config(config: &Config) -> Result<()> {
    clear_last_error();
    unsafe { ocio_sys::ocio_set_isolated_current_config(config.handle.as_ptr(), false) };
    ocio_call_status()
}

/// Install a shared read-only config as the process-global current config.
///
/// Group-transform graphs are detached from the input before installation;
/// OCIO then makes its own config copy.
/// See [`current_config`] for the process-global threading considerations.
pub fn try_set_const_current_config(config: &ConstConfig) -> Result<()> {
    clear_last_error();
    unsafe { ocio_sys::ocio_set_isolated_current_config(config.handle.as_ptr(), true) };
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

/// Returns the current OCIO global logging level, or `Unknown` when called
/// reentrantly from a logging callback. Use [`try_logging_level`] to distinguish
/// that case.
pub fn logging_level() -> crate::LoggingLevel {
    try_logging_level().unwrap_or(crate::LoggingLevel::Unknown)
}

/// Get the global logging level, reporting attempts to reenter from a callback.
pub fn try_logging_level() -> Result<crate::LoggingLevel> {
    check_logging_reentrancy()?;
    let l = unsafe { ocio_sys::ocio_get_logging_level() };
    Ok(match l {
        0 => crate::LoggingLevel::None,
        1 => crate::LoggingLevel::Warning,
        2 => crate::LoggingLevel::Info,
        3 => crate::LoggingLevel::Debug,
        _ => crate::LoggingLevel::Unknown,
    })
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
    check_logging_reentrancy()?;
    if level as i32 == 4 {
        return Err(OcioError::InvalidInput(
            "OpenColorIO does not support trace logging".into(),
        ));
    }
    clear_last_error();
    unsafe { ocio_sys::ocio_set_logging_level(level as i32) };
    ocio_call_status()
}

/// Send a message through OCIO's configured logging function.
///
/// OCIO's logging mechanism is thread-safe. This only invokes the current
/// OCIO logger; use `try_set_logging_level` to control filtering.
pub fn try_log_message(level: crate::LoggingLevel, message: impl AsRef<str>) -> Result<()> {
    check_logging_reentrancy()?;
    if level as i32 == 4 {
        return Err(OcioError::InvalidInput(
            "OpenColorIO does not support trace logging".into(),
        ));
    }
    let message = cstring(message)?;
    clear_last_error();
    unsafe { ocio_sys::ocio_log_message(level as i32, message.as_ptr()) };
    ocio_call_status()
}

/// Install a process-global Rust callback for OCIO log messages.
///
/// OCIO invokes the callback from any thread that emits a log message. The
/// callback is retained by the crate until [`reset_logging_callback`] is
/// called. Panics are caught at the FFI boundary and do not unwind through
/// OCIO.
///
/// Installing or resetting the callback serializes updates made through this
/// crate. Coordinate with other native code that changes OCIO's global logger.
///
/// OCIO invokes the callback while holding a non-recursive global logging lock.
/// Do not call OCIO from the callback (or wait for another thread that may do
/// so): even operations unrelated to logging can emit log messages and deadlock.
/// Direct calls to this crate's logging helpers on the callback thread return an
/// error; [`logging_level`] returns `Unknown` there for compatibility.
///
/// OCIO versions before 2.5.2 do not synchronize logger replacement against
/// logging. On those versions this safe function returns an error; applications
/// that can exclude all concurrent OCIO logging may use
/// [`set_logging_callback_unchecked`] instead.
pub fn set_logging_callback(callback: impl Fn(&str) + Send + Sync + 'static) -> Result<()> {
    check_logging_reentrancy()?;
    if !is_stub_build() && version_hex() < 0x0205_0200 {
        return Err(OcioError::InvalidInput(
            "OCIO before 2.5.2 requires externally synchronized logging callback replacement"
                .into(),
        ));
    }
    // SAFETY: Supported native versions serialize replacement with logging;
    // stub mode has no native logger.
    unsafe { set_logging_callback_unchecked(callback) }
}

/// Install a logger even on versions whose logger replacement is not synchronized.
///
/// # Safety
/// For OCIO before 2.5.2, no other thread or native component may log or replace
/// the process-global OCIO logger while this function executes. OCIO operations
/// that appear unrelated may log. This obligation includes native users outside
/// this crate. The normal callback reentrancy restrictions still apply.
pub unsafe fn set_logging_callback_unchecked(
    callback: impl Fn(&str) + Send + Sync + 'static,
) -> Result<()> {
    check_logging_reentrancy()?;
    let update = logging_callback_update_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let callback = Arc::new(callback) as Arc<LoggingCallback>;
    let previous = {
        let mut slot = logging_callback_slot()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        (*slot).replace(callback)
    };

    clear_last_error();
    unsafe { ocio_sys::ocio_set_logging_callback(Some(rust_logging_callback)) };
    let result = ocio_call_status();
    let discarded = if result.is_err() {
        let mut slot = logging_callback_slot()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        std::mem::replace(&mut *slot, previous)
    } else {
        previous
    };
    drop(update);
    drop(discarded);
    result
}

/// Restore OCIO's default process-global logging function.
///
/// Any callback previously installed through [`set_logging_callback`] is
/// dropped after OCIO accepts the reset.
/// Returns an error before changing state on native versions before 2.5.2;
/// see [`reset_logging_callback_unchecked`] for externally synchronized use.
pub fn reset_logging_callback() -> Result<()> {
    check_logging_reentrancy()?;
    if !is_stub_build() && version_hex() < 0x0205_0200 {
        return Err(OcioError::InvalidInput(
            "OCIO before 2.5.2 requires externally synchronized logging callback replacement"
                .into(),
        ));
    }
    // SAFETY: Supported native versions serialize replacement with logging;
    // stub mode has no native logger.
    unsafe { reset_logging_callback_unchecked() }
}

/// Reset a logger on versions without synchronized replacement.
///
/// # Safety
/// The same exclusion of concurrent OCIO logging and logger changes required by
/// [`set_logging_callback_unchecked`] applies for OCIO before 2.5.2.
pub unsafe fn reset_logging_callback_unchecked() -> Result<()> {
    check_logging_reentrancy()?;
    let update = logging_callback_update_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    clear_last_error();
    unsafe { ocio_sys::ocio_reset_logging_callback() };
    ocio_call_status()?;
    let previous = logging_callback_slot()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .take();
    drop(update);
    drop(previous);
    Ok(())
}

/// Install a process-global OCIO compute-hash callback.
///
/// The callback receives OCIO's resolved file path as bytes and returns the
/// file hash as bytes. OCIO `std::string` values, including embedded NUL bytes,
/// round-trip without loss. Its output is copied
/// by the bridge immediately after the callback returns. Panics are caught and never unwind
/// across the FFI boundary.
/// OCIO caches file hashes, and replacing this callback does not invalidate
/// previously cached entries. In particular, a failing callback may leave an
/// empty cached result. After correcting a failure or changing hash semantics,
/// coordinate with other OCIO users and call [`try_clear_all_caches`] before
/// retrying affected files.
///
/// # Safety
///
/// OCIO does not synchronize replacement of its process-global hash function
/// with calls to it. The caller must ensure that no thread or reentrant
/// callback is executing or can start an OCIO operation that may hash a file
/// until this function returns. Coordinate with other native users of OCIO too;
/// the crate's internal update lock only serializes updates through this crate.
///
/// ```compile_fail
/// ocio_rs::set_compute_hash_callback(|bytes| bytes.to_vec()).unwrap();
/// ```
pub unsafe fn set_compute_hash_callback(
    callback: impl Fn(&[u8]) -> Vec<u8> + Send + Sync + 'static,
) -> Result<()> {
    let update = compute_hash_callback_update_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let callback = Arc::new(callback) as Arc<ComputeHashCallback>;
    let previous = {
        let mut slot = compute_hash_callback_slot()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        (*slot).replace(callback)
    };
    clear_last_error();
    unsafe { ocio_sys::ocio_set_compute_hash_callback(Some(rust_compute_hash_callback)) };
    let result = ocio_call_status();
    let discarded = if result.is_err() {
        let mut slot = compute_hash_callback_slot()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        std::mem::replace(&mut *slot, previous)
    } else {
        previous
    };
    drop(update);
    drop(discarded);
    result
}

/// Restore OCIO's default process-global compute-hash function.
///
/// # Safety
///
/// The same process-wide exclusion requirements as
/// [`set_compute_hash_callback`] apply, including exclusion of a callback
/// currently executing on the calling thread.
///
/// ```compile_fail
/// ocio_rs::reset_compute_hash_callback().unwrap();
/// ```
pub unsafe fn reset_compute_hash_callback() -> Result<()> {
    let update = compute_hash_callback_update_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    clear_last_error();
    unsafe { ocio_sys::ocio_reset_compute_hash_callback() };
    ocio_call_status()?;
    let previous = compute_hash_callback_slot()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .take();
    drop(update);
    drop(previous);
    Ok(())
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
/// OCIO 2.4–2.6 archives created on Unix may contain a `config.ocio` with zero
/// permission bits, which native extraction preserves. The caller may need to
/// restore appropriate file permissions before reading it.
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

/// Read an OCIO process environment variable.
///
/// # Safety
/// OCIO documents its environment-variable helpers as not thread-safe. The
/// caller must serialize this call with all OCIO environment-variable calls
/// and other native code that accesses the same process environment.
pub unsafe fn get_env_variable(name: impl AsRef<str>) -> Result<Option<String>> {
    let name = cstring(name)?;
    clear_last_error();
    let value = unsafe { cstr_to_opt_string(ocio_sys::ocio_get_env_variable(name.as_ptr())) };
    ocio_call_status()?;
    Ok(value)
}

/// Set an OCIO process environment variable.
///
/// # Safety
/// OCIO documents its environment-variable helpers as not thread-safe. The
/// caller must serialize this call with all OCIO environment-variable calls
/// and other native code that accesses the same process environment.
pub unsafe fn set_env_variable(name: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
    let name = cstring(name)?;
    let value = cstring(value)?;
    clear_last_error();
    unsafe { ocio_sys::ocio_set_env_variable(name.as_ptr(), value.as_ptr()) };
    ocio_call_status()
}

/// Remove an OCIO process environment variable.
///
/// # Safety
/// OCIO documents its environment-variable helpers as not thread-safe. The
/// caller must serialize this call with all OCIO environment-variable calls
/// and other native code that accesses the same process environment.
pub unsafe fn unset_env_variable(name: impl AsRef<str>) -> Result<()> {
    let name = cstring(name)?;
    clear_last_error();
    unsafe { ocio_sys::ocio_unset_env_variable(name.as_ptr()) };
    ocio_call_status()
}

/// Return whether an OCIO process environment variable is present.
///
/// # Safety
/// OCIO documents its environment-variable helpers as not thread-safe. The
/// caller must serialize this call with all OCIO environment-variable calls
/// and other native code that accesses the same process environment.
pub unsafe fn is_env_variable_present(name: impl AsRef<str>) -> Result<bool> {
    let name = cstring(name)?;
    clear_last_error();
    let present = unsafe { ocio_sys::ocio_is_env_variable_present(name.as_ptr()) };
    ocio_call_status()?;
    Ok(present)
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
///
/// # Safety
/// The caller must meet the synchronization requirements of [`try_current_config`].
///
/// ```compile_fail
/// let _ = ocio_rs::processor_cache_flags(); // Reads a native global snapshot.
/// ```
pub unsafe fn processor_cache_flags() -> crate::ProcessorCacheFlags {
    unsafe { try_const_current_config() }
        .ok()
        .flatten()
        .and_then(|config| config.processor_cache_flags().ok())
        .unwrap_or(crate::ProcessorCacheFlags(0))
}

/// Updates the processor cache flags on the current global config, if any.
#[deprecated(
    since = "0.2.0",
    note = "discarded OCIO errors; prefer try_set_processor_cache_flags()"
)]
/// # Safety
/// The caller must meet the requirements of [`try_set_processor_cache_flags`].
///
/// ```compile_fail
/// # #[allow(deprecated)]
/// ocio_rs::set_processor_cache_flags(ocio_rs::ProcessorCacheFlags::DEFAULT);
/// ```
pub unsafe fn set_processor_cache_flags(flags: crate::ProcessorCacheFlags) {
    let _ = unsafe { try_set_processor_cache_flags(flags) };
}

/// Try to update processor-cache flags on the current global config.
///
/// When the policy changes, this installs a new config with fresh caches.
/// Earlier read-only snapshots and existing processors are unaffected.
/// Serialize this read/modify/install operation with other global config writes.
/// This returns `Ok(())` when no global config is installed.
///
/// # Safety
/// The caller must meet the synchronization requirements of [`try_current_config`]
/// throughout this read/modify/install operation and coordinate global writes.
///
/// ```compile_fail
/// let _ = ocio_rs::try_set_processor_cache_flags(ocio_rs::ProcessorCacheFlags::DEFAULT);
/// ```
pub unsafe fn try_set_processor_cache_flags(flags: crate::ProcessorCacheFlags) -> Result<()> {
    if let Some(current) = unsafe { try_const_current_config()? } {
        if current.processor_cache_flags()? != flags {
            let config = current.create_editable_copy()?;
            config.try_set_processor_cache_flags(flags.0 as i32)?;
            try_set_current_config(&config)?;
        }
    }
    Ok(())
}

pub(crate) fn cstring(value: impl AsRef<str>) -> Result<CString> {
    CString::new(value.as_ref()).map_err(|_| OcioError::InteriorNul)
}

pub(crate) unsafe fn cstr_to_opt_string(ptr: *const std::ffi::c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned())
    }
}

// v2.5.1 compat: accept *mut c_void (new return type for many getter functions)
pub(crate) unsafe fn cstr_from_mut(ptr: *mut c_void) -> Option<String> {
    cstr_to_opt_string(ptr as *const std::ffi::c_char)
}

pub(crate) fn clear_last_error() {
    unsafe { ocio_sys::ocio_error_clear_last() };
}

// A user-supplied panic payload can itself panic when dropped. Never let a
// second panic unwind across a C callback boundary.
pub(crate) fn discard_callback_panic(payload: Box<dyn std::any::Any + Send>) {
    if let Err(second) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(payload))) {
        std::mem::forget(second);
    }
}

pub(crate) fn catch_callback_unwind<F, T>(callback: F) -> std::result::Result<T, ()>
where
    F: FnOnce() -> T + std::panic::UnwindSafe,
{
    match std::panic::catch_unwind(callback) {
        Ok(value) => Ok(value),
        Err(payload) => {
            discard_callback_panic(payload);
            Err(())
        }
    }
}

pub(crate) fn last_error_message() -> Option<String> {
    unsafe { cstr_to_opt_string(ocio_sys::ocio_error_get_last()) }
}

/// Read and clear the bridge's thread-local error slot.
///
/// Each safe wrapper clears the slot before calling into the bridge. Consuming
/// an error here prevents a handled failure from being observed by a later FFI
/// call on the same thread.
fn take_last_error() -> Option<(String, bool)> {
    let message = last_error_message();
    message.map(|message| {
        let missing_file = unsafe { ocio_sys::ocio_error_get_last_kind() == 1 };
        clear_last_error();
        (message, missing_file)
    })
}

fn bridge_error(message: String, missing_file: bool) -> OcioError {
    if missing_file {
        OcioError::MissingFile(message)
    } else {
        OcioError::Ocio(message)
    }
}

pub(crate) fn validation_status() -> Result<()> {
    match take_last_error() {
        Some((message, _)) => Err(OcioError::ValidationFailed(message)),
        None => Ok(()),
    }
}

pub(crate) fn ocio_call_status() -> Result<()> {
    match take_last_error() {
        Some((message, missing_file)) => Err(bridge_error(message, missing_file)),
        None => Ok(()),
    }
}

pub(crate) fn handle_result(handle: *mut c_void) -> Result<NonNull<c_void>> {
    match NonNull::new(handle) {
        Some(handle) => Ok(handle),
        None => match take_last_error() {
            Some((message, missing_file)) => Err(bridge_error(message, missing_file)),
            None => Err(OcioError::AllocationFailed),
        },
    }
}

#[cfg(test)]
mod tests {
    #[cfg(ocio_stub)]
    static CALLBACK_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn hash_callback_accepts_null_empty_input() {
        #[cfg(ocio_stub)]
        let _test_guard = CALLBACK_TEST_LOCK.lock().unwrap();
        let _guard = super::compute_hash_callback_update_lock().lock().unwrap();
        let previous =
            super::compute_hash_callback_slot()
                .lock()
                .unwrap()
                .replace(std::sync::Arc::new(|input| {
                    assert!(input.is_empty());
                    b"hash".to_vec()
                }));
        let mut output = std::ptr::null();
        let mut output_len = 0;
        assert!(!unsafe {
            super::rust_compute_hash_callback(std::ptr::null(), 1, &mut output, &mut output_len)
        });
        let succeeded = unsafe {
            super::rust_compute_hash_callback(std::ptr::null(), 0, &mut output, &mut output_len)
        };
        assert!(succeeded);
        assert_eq!(
            unsafe { std::slice::from_raw_parts(output, output_len) },
            b"hash"
        );
        *super::compute_hash_callback_slot().lock().unwrap() = previous;
    }

    #[test]
    #[cfg(ocio_stub)]
    fn callback_captures_are_dropped_without_internal_locks() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        struct Probe {
            update: &'static std::sync::Mutex<()>,
            slot_unlocked: fn() -> bool,
            ok: Arc<AtomicBool>,
        }
        impl Drop for Probe {
            fn drop(&mut self) {
                self.ok.store(
                    self.update.try_lock().is_ok() && (self.slot_unlocked)(),
                    Ordering::SeqCst,
                );
            }
        }
        let _test_guard = CALLBACK_TEST_LOCK.lock().unwrap();
        for reset in [false, true] {
            let ok = Arc::new(AtomicBool::new(false));
            let probe = Probe {
                update: super::logging_callback_update_lock(),
                slot_unlocked: || super::logging_callback_slot().try_lock().is_ok(),
                ok: Arc::clone(&ok),
            };
            super::set_logging_callback(move |_| {
                let _ = &probe;
            })
            .unwrap();
            if reset {
                super::reset_logging_callback().unwrap();
            } else {
                super::set_logging_callback(|_| {}).unwrap();
                super::reset_logging_callback().unwrap();
            }
            assert!(ok.load(Ordering::SeqCst));

            let ok = Arc::new(AtomicBool::new(false));
            let probe = Probe {
                update: super::compute_hash_callback_update_lock(),
                slot_unlocked: || super::compute_hash_callback_slot().try_lock().is_ok(),
                ok: Arc::clone(&ok),
            };
            // Stub mode performs no native hashing; the test lock excludes the
            // other test that accesses the callback slot in this process.
            unsafe {
                super::set_compute_hash_callback(move |_| {
                    let _ = &probe;
                    Vec::new()
                })
            }
            .unwrap();
            if reset {
                unsafe { super::reset_compute_hash_callback() }.unwrap();
            } else {
                unsafe { super::set_compute_hash_callback(|_| Vec::new()) }.unwrap();
                unsafe { super::reset_compute_hash_callback() }.unwrap();
            }
            assert!(ok.load(Ordering::SeqCst));
        }
    }

    #[test]
    #[allow(deprecated)]
    fn unsupported_trace_logging_is_rejected_before_native_call() {
        assert!(matches!(
            super::try_set_logging_level(super::LoggingLevel::Trace),
            Err(super::OcioError::InvalidInput(_))
        ));
        assert!(matches!(
            super::try_log_message(super::LoggingLevel::Trace, "message"),
            Err(super::OcioError::InvalidInput(_))
        ));
    }

    use super::*;

    #[test]
    fn processor_cache_flags_no_crash() {
        // This test process has no foreign native global-config users.
        let f = unsafe { processor_cache_flags() };
        assert!(unsafe { try_set_processor_cache_flags(f) }.is_ok());
    }

    #[test]
    fn current_config_no_crash() {
        let _ = unsafe { current_config() };
    }

    #[test]
    fn try_current_config_no_crash() {
        let _ = unsafe { try_current_config() };
    }

    #[cfg(feature = "bundled")]
    #[test]
    fn bridge_errors_are_consumed_after_handle_conversion() {
        let path = cstring("this-file-does-not-exist.ocio").expect("valid test path");
        clear_last_error();
        let handle = unsafe { ocio_sys::ocio_config_create_from_file(path.as_ptr().cast()) };

        let error = handle_result(handle).expect_err("missing config");
        if version_hex() >= 0x02060000 {
            assert!(matches!(error, OcioError::MissingFile(_)));
        } else {
            assert!(matches!(
                error,
                OcioError::Ocio(_) | OcioError::MissingFile(_)
            ));
        }
        assert!(last_error_message().is_none());
        assert_eq!(unsafe { ocio_sys::ocio_error_get_last_kind() }, 0);
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
        let _ = unsafe { get_current_config() };
    }
}
