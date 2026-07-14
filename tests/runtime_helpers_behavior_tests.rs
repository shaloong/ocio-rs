//! Top-level runtime-helper behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after unit-level smoke coverage in
//! `src/lib.rs` and `src/config.rs`. In bundled/real mode they validate global
//! version/logging helpers and config version-mutation behavior.

mod common;
use common::*;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::FileTransform;
use ocio_rs::{
    extract_ocioz_archive, get_env_variable, is_env_variable_present, logging_level,
    reset_compute_hash_callback, reset_logging_callback, resolve_config_path,
    set_compute_hash_callback, set_env_variable, set_logging_callback, try_clear_all_caches,
    try_log_message, try_set_logging_level, unset_env_variable, version, version_hex, Config,
    LoggingLevel,
};

fn runtime_helpers_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn global_version_and_logging_helper_behavior() {
    let _guard = runtime_helpers_test_lock();
    if is_stub() {
        return;
    }

    let runtime_version = version().expect("runtime version");
    assert!(!runtime_version.trim().is_empty());
    assert!(runtime_version.starts_with("2.5"));
    assert!(version_hex() > 0);

    let original_level = logging_level();
    try_set_logging_level(LoggingLevel::Warning).expect("set warning logging level");
    assert_eq!(logging_level(), LoggingLevel::Warning);

    try_set_logging_level(LoggingLevel::Debug).expect("set debug logging level");
    assert_eq!(logging_level(), LoggingLevel::Debug);

    try_set_logging_level(original_level).expect("restore logging level");
}

#[test]
fn global_config_path_and_archive_helpers_preserve_behavior() {
    let _guard = runtime_helpers_test_lock();

    assert_eq!(
        resolve_config_path("definitely-not-an-ocio-config-path").expect("resolve ordinary path"),
        "definitely-not-an-ocio-config-path"
    );

    if is_stub() {
        assert!(extract_ocioz_archive("missing.ocioz", "missing-output").is_err());
        return;
    }

    let resolved_builtin = resolve_config_path("ocio://default").expect("resolve builtin path");
    assert!(!resolved_builtin.trim().is_empty());

    let missing_archive = std::env::temp_dir().join("ocio-rs-missing-archive.ocioz");
    let destination = std::env::temp_dir().join("ocio-rs-missing-archive-output");
    assert!(extract_ocioz_archive(
        missing_archive.to_string_lossy(),
        destination.to_string_lossy(),
    )
    .is_err());
}

#[test]
fn global_logging_and_environment_helpers_preserve_behavior() {
    let _guard = runtime_helpers_test_lock();
    try_log_message(LoggingLevel::Debug, "ocio-rs runtime helper test").expect("log OCIO message");

    let name = format!("OCIO_RS_RUNTIME_TEST_{}", std::process::id());
    if is_stub() {
        assert_eq!(
            unsafe { get_env_variable(&name) }.expect("stub env lookup"),
            None
        );
        assert!(!unsafe { is_env_variable_present(&name) }.expect("stub env presence"));
        return;
    }

    let original = unsafe { get_env_variable(&name) }.expect("read original environment value");
    unsafe { set_env_variable(&name, "runtime-helper-value") }.expect("set OCIO environment");
    assert_eq!(
        unsafe { get_env_variable(&name) }.expect("read OCIO environment"),
        Some("runtime-helper-value".to_owned())
    );
    assert!(unsafe { is_env_variable_present(&name) }.expect("check OCIO environment"));

    unsafe { unset_env_variable(&name) }.expect("unset OCIO environment");
    assert!(!unsafe { is_env_variable_present(&name) }.expect("check removed OCIO environment"));
    if let Some(original) = original {
        unsafe { set_env_variable(&name, original) }.expect("restore OCIO environment");
    }
}

#[test]
fn global_logging_callback_preserves_lifetime_and_panic_boundaries() {
    let _guard = runtime_helpers_test_lock();
    if is_stub() {
        return;
    }

    let calls = Arc::new(AtomicUsize::new(0));
    let callback_calls = Arc::clone(&calls);
    set_logging_callback(move |_| {
        callback_calls.fetch_add(1, Ordering::SeqCst);
    })
    .expect("install logging callback");
    try_log_message(LoggingLevel::Warning, "ocio-rs logging callback test")
        .expect("emit logging callback message");
    reset_logging_callback().expect("reset logging callback");
    assert!(calls.load(Ordering::SeqCst) > 0);
}

#[test]
fn global_compute_hash_callback_is_invoked_for_file_backed_transforms() {
    let _guard = runtime_helpers_test_lock();
    if is_stub() {
        return;
    }

    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/clf/matrix_3x4_example.clf");

    let calls = Arc::new(AtomicUsize::new(0));
    let callback_calls = Arc::clone(&calls);
    set_compute_hash_callback(move |_| {
        callback_calls.fetch_add(1, Ordering::SeqCst);
        b"intentionally-invalid-hash".to_vec()
    })
    .expect("install compute hash callback");
    try_clear_all_caches().expect("clear OCIO caches");
    let transform = FileTransform::create().expect("create file transform");
    transform
        .set_src(path.to_string_lossy())
        .expect("set CLF path");
    let loaded = Config::raw()
        .expect("raw config")
        .processor_from_transform_default_direction(&transform);
    reset_compute_hash_callback().expect("reset compute hash callback");

    let _ = loaded;
    assert!(calls.load(Ordering::SeqCst) > 0);
}

#[test]
fn global_compute_hash_callback_panic_does_not_cross_ffi() {
    let _guard = runtime_helpers_test_lock();
    if is_stub() {
        return;
    }

    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/clf/matrix_3x4_example.clf");
    let calls = Arc::new(AtomicUsize::new(0));
    let callback_calls = Arc::clone(&calls);
    set_compute_hash_callback(move |_| {
        callback_calls.fetch_add(1, Ordering::SeqCst);
        panic!("intentional compute hash callback panic")
    })
    .expect("install compute hash callback");
    try_clear_all_caches().expect("clear OCIO caches");
    let transform = FileTransform::create().expect("create file transform");
    transform
        .set_src(path.to_string_lossy())
        .expect("set CLF path");
    let result = Config::raw()
        .expect("raw config")
        .processor_from_transform_default_direction(&transform);
    reset_compute_hash_callback().expect("reset compute hash callback");

    let _ = result;
    assert!(calls.load(Ordering::SeqCst) > 0);
}

#[test]
fn config_version_mutation_and_upgrade_behavior() {
    let _guard = runtime_helpers_test_lock();
    if is_stub() {
        return;
    }

    let config = Config::raw()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");

    assert_eq!(config.major_version(), 2);
    assert_eq!(config.minor_version(), 0);

    config.set_minor_version(1).expect("set minor version 1");
    assert_eq!(config.major_version(), 2);
    assert_eq!(config.minor_version(), 1);

    config.set_major_version(1).expect("set major version 1");
    config.set_minor_version(0).expect("set minor version 0");
    assert_eq!(config.major_version(), 1);
    assert_eq!(config.minor_version(), 0);

    config.set_version(2, 0).expect("set version 2.0");
    assert_eq!(config.major_version(), 2);
    assert_eq!(config.minor_version(), 0);

    config
        .upgrade_to_latest_version()
        .expect("upgrade config version");
    assert_eq!(config.major_version(), 2);
    assert_eq!(config.minor_version(), 0);
}

#[test]
fn config_version_mutation_surfaces_invalid_version_errors() {
    let _guard = runtime_helpers_test_lock();
    if is_stub() {
        return;
    }

    let config = Config::raw()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");

    let invalid_major = config
        .set_major_version(99)
        .expect_err("unsupported major version should fail");
    assert!(
        matches!(invalid_major, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {invalid_major:?}"
    );

    config.set_major_version(1).expect("set major version 1");
    let invalid_minor = config
        .set_minor_version(99)
        .expect_err("unsupported minor version should fail");
    assert!(
        matches!(invalid_minor, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {invalid_minor:?}"
    );

    let invalid_pair = config
        .set_version(99, 99)
        .expect_err("unsupported version pair should fail");
    assert!(
        matches!(invalid_pair, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {invalid_pair:?}"
    );
}
