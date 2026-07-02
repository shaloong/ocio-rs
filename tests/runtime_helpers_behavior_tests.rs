//! Top-level runtime-helper behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after unit-level smoke coverage in
//! `src/lib.rs` and `src/config.rs`. In bundled/real mode they validate global
//! version/logging helpers and config version-mutation behavior.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::{logging_level, set_logging_level, version, version_hex, Config, LoggingLevel};

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
    set_logging_level(LoggingLevel::Warning);
    assert_eq!(logging_level(), LoggingLevel::Warning);

    set_logging_level(LoggingLevel::Debug);
    assert_eq!(logging_level(), LoggingLevel::Debug);

    set_logging_level(original_level);
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

    config.upgrade_to_latest_version();
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
