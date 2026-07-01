//! Config runtime-settings behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/config.rs` and `src/lib.rs`. In bundled/real mode they validate active
//! display/view state, environment-variable metadata, per-config processor
//! cache flags, and crate-level current-config helpers.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::{
    current_config, processor_cache_flags, set_current_config, set_processor_cache_flags,
    ProcessorCacheFlags,
};

fn config_runtime_settings_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn config_active_display_view_environment_and_cache_flag_behavior() {
    let _guard = config_runtime_settings_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");

    let first_display = config.display(0).expect("first display");
    let first_view = config.view(&first_display, 0).expect("first view");

    assert!(config.num_views(&first_display) > 0);

    config
        .set_active_displays(&first_display)
        .expect("set active displays");
    config.set_active_views(&first_view).expect("set active views");

    assert_eq!(config.active_displays().as_deref(), Some(first_display.as_str()));
    assert_eq!(config.active_views().as_deref(), Some(first_view.as_str()));
    assert_eq!(config.num_active_displays(), 1);
    assert_eq!(config.num_active_views(), 1);
    assert_eq!(config.active_display(0).as_deref(), Some(first_display.as_str()));
    assert_eq!(config.active_view(0).as_deref(), Some(first_view.as_str()));

    config
        .add_environment_var("UNIT_OCIO_SHOW", "demo_show")
        .expect("add environment var");
    config
        .add_environment_var("UNIT_OCIO_SHOT", "shot010")
        .expect("add environment var");
    assert_eq!(config.num_environment_vars(), 2);
    let env_names = [
        config
            .environment_var_name_by_index(0)
            .expect("environment var name 0"),
        config
            .environment_var_name_by_index(1)
            .expect("environment var name 1"),
    ];
    assert!(env_names.iter().any(|name| name == "UNIT_OCIO_SHOW"));
    assert!(env_names.iter().any(|name| name == "UNIT_OCIO_SHOT"));
    assert_eq!(
        config.environment_var_default("UNIT_OCIO_SHOW").as_deref(),
        Some("demo_show")
    );
    assert_eq!(
        config.environment_var_default("UNIT_OCIO_SHOT").as_deref(),
        Some("shot010")
    );

    let initial_cache_flags = config.processor_cache_flags();
    assert_eq!(
        initial_cache_flags,
        (ProcessorCacheFlags::ENABLED | ProcessorCacheFlags::SHARE_DYN_PROPERTIES).0 as i32
    );
    let custom_flags =
        ProcessorCacheFlags::ENABLED | ProcessorCacheFlags::SHARE_DYN_PROPERTIES;
    config.set_processor_cache_flags(custom_flags.0 as i32);
    assert_eq!(config.processor_cache_flags(), custom_flags.0 as i32);

    config.clear_active_displays();
    config.clear_active_views();
    config.clear_environment_vars();
    assert_eq!(config.num_active_displays(), 0);
    assert_eq!(config.num_active_views(), 0);
    assert_eq!(config.num_environment_vars(), 0);
}

#[test]
fn global_current_config_and_processor_cache_flag_behavior() {
    let _guard = config_runtime_settings_test_lock();
    if is_stub() {
        return;
    }

    let original = current_config();

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");
    let custom_flags =
        ProcessorCacheFlags::ENABLED | ProcessorCacheFlags::SHARE_DYN_PROPERTIES;

    set_current_config(&config);
    let installed = current_config().expect("current config after install");
    assert_eq!(installed.processor_cache_flags(), config.processor_cache_flags());

    set_processor_cache_flags(custom_flags);
    assert_eq!(processor_cache_flags(), custom_flags);
    assert_eq!(config.processor_cache_flags(), custom_flags.0 as i32);

    if let Some(ref original_config) = original {
        set_current_config(original_config);
    }
}
