//! BuiltinConfigRegistry behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/builtin_config_registry.rs`. In bundled/real mode they validate that
//! builtin config enumeration, YAML access, and config creation stay coherent.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::BuiltinConfigRegistry;

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

    let name = registry.config_name(0).expect("builtin config name");
    let ui_name = registry.config_ui_name(0).expect("builtin config ui name");
    let yaml_by_index = registry
        .config_yaml_by_index(0)
        .expect("builtin config yaml by index");
    let yaml_by_name = registry
        .config_yaml_by_name(&name)
        .expect("builtin config yaml by name");

    assert!(!name.is_empty());
    assert!(!ui_name.is_empty());
    assert_eq!(yaml_by_index, yaml_by_name);
    assert!(yaml_by_index.contains("ocio_profile_version"));
    assert!(yaml_by_index.contains("colorspaces:"));

    let config_by_index = registry.config_by_index(0).expect("config by index");
    let config_by_name = registry.config_by_name(&name).expect("config by name");
    config_by_index
        .validate()
        .expect("config by index validate");
    config_by_name.validate().expect("config by name validate");

    assert!(config_by_index.num_color_spaces() > 0);
    assert_eq!(config_by_index.cache_id(), config_by_name.cache_id());

    let serialized = config_by_index
        .serialize()
        .expect("serialize builtin config");
    assert!(serialized.contains("ocio_profile_version"));
    assert!(serialized.contains("colorspaces:"));
}
