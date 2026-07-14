//! BuiltinConfigRegistry behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/builtin_config_registry.rs`. In bundled/real mode they validate that
//! builtin config enumeration, YAML access, and config creation stay coherent.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::{BuiltinConfigRegistry, Config, OcioError};

fn builtin_config_registry_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn builtin_config_registry_round_trip_behavior() {
    let _guard = builtin_config_registry_test_lock();
    if is_stub() {
        return;
    }

    let registry = BuiltinConfigRegistry::get().expect("builtin config registry");
    let count = registry.num_builtin_configs();
    assert!(count > 0);

    let name = registry
        .try_config_name(0)
        .expect("builtin config name query")
        .expect("builtin config name");
    let ui_name = registry
        .try_config_ui_name(0)
        .expect("builtin config UI name query")
        .expect("builtin config UI name");
    let yaml_by_index = registry
        .try_config_yaml_by_index(0)
        .expect("builtin config YAML index query")
        .expect("builtin config yaml by index");
    let yaml_by_name = registry
        .try_config_yaml_by_name(&name)
        .expect("builtin config YAML name query")
        .expect("builtin config yaml by name");

    assert!(!name.is_empty());
    assert!(!ui_name.is_empty());
    assert_eq!(yaml_by_index, yaml_by_name);
    assert!(yaml_by_index.contains("ocio_profile_version"));
    assert!(yaml_by_index.contains("colorspaces:"));

    let _recommended = registry
        .try_is_config_recommended(0)
        .expect("recommended flag query");
    let out_of_range = registry
        .try_is_config_recommended(count)
        .expect_err("out-of-range recommendation query should fail");
    assert!(matches!(out_of_range, OcioError::Ocio(_)));
    assert!(!registry.is_config_recommended(count));
    assert!(matches!(
        registry.try_is_config_recommended(-1),
        Err(OcioError::InvalidInput(_))
    ));

    let config_by_index = registry.config_by_index(0).expect("config by index");
    let config_by_name = registry
        .try_config_by_name(&name)
        .expect("config by name query")
        .expect("config by name");
    config_by_index
        .validate()
        .expect("config by index validate");
    config_by_name.validate().expect("config by name validate");

    assert!(config_by_index.num_color_spaces() > 0);
    assert_eq!(config_by_index.cache_id(), config_by_name.cache_id());

    let serialized = config_by_index
        .serialize()
        .expect("serialize builtin config")
        .expect("real serialized builtin config");
    assert!(serialized.contains("ocio_profile_version"));
    assert!(serialized.contains("colorspaces:"));
    assert!(registry.try_config_yaml_by_name("bad\0config").is_err());
    assert!(registry.try_config_by_name("bad\0config").is_err());
}

#[test]
fn config_create_from_builtin_config_matches_registry_behavior() {
    let _guard = builtin_config_registry_test_lock();
    if is_stub() {
        return;
    }

    let registry = BuiltinConfigRegistry::get().expect("builtin config registry");
    let name = registry.config_name(0).expect("builtin config name");

    let via_registry = registry.config_by_name(&name).expect("config by registry");
    let via_config = Config::create_from_builtin_config(&name).expect("config by Config");

    via_registry.validate().expect("registry config validate");
    via_config.validate().expect("config validate");

    assert_eq!(via_registry.cache_id(), via_config.cache_id());
    assert_eq!(
        via_registry.num_color_spaces(),
        via_config.num_color_spaces()
    );
    assert_eq!(
        via_registry.num_displays_all(),
        via_config.num_displays_all()
    );
}

#[test]
fn config_create_from_builtin_config_invalid_name_reports_error_behavior() {
    let _guard = builtin_config_registry_test_lock();
    if is_stub() {
        return;
    }

    let err = match Config::create_from_builtin_config("definitely_missing_builtin_config") {
        Ok(_) => panic!("missing builtin config should fail"),
        Err(err) => err,
    };
    assert!(
        matches!(err, OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );
}
