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
    processor_cache_flags, set_processor_cache_flags, try_clear_all_caches, try_current_config,
    try_set_current_config, ProcessorCacheFlags,
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
    config
        .set_active_views(&first_view)
        .expect("set active views");

    assert_eq!(
        config.active_displays().as_deref(),
        Some(first_display.as_str())
    );
    assert_eq!(config.active_views().as_deref(), Some(first_view.as_str()));
    assert_eq!(config.num_active_displays(), 1);
    assert_eq!(config.num_active_views(), 1);
    assert_eq!(
        config.active_display(0).as_deref(),
        Some(first_display.as_str())
    );
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
    let custom_flags = ProcessorCacheFlags::ENABLED | ProcessorCacheFlags::SHARE_DYN_PROPERTIES;
    config.set_processor_cache_flags(custom_flags.0 as i32);
    assert_eq!(config.processor_cache_flags(), custom_flags.0 as i32);

    config
        .try_clear_active_displays()
        .expect("clear active displays");
    config.try_clear_active_views().expect("clear active views");
    config
        .clear_environment_vars()
        .expect("clear environment variables");
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

    let original = try_current_config().expect("read current config");

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");
    let custom_flags = ProcessorCacheFlags::ENABLED | ProcessorCacheFlags::SHARE_DYN_PROPERTIES;

    try_set_current_config(&config).expect("install current config");
    let installed = try_current_config()
        .expect("read current config after install")
        .expect("current config after install");
    assert_eq!(
        installed.processor_cache_flags(),
        config.processor_cache_flags()
    );

    set_processor_cache_flags(custom_flags);
    assert_eq!(processor_cache_flags(), custom_flags);
    assert_eq!(config.processor_cache_flags(), custom_flags.0 as i32);
    try_clear_all_caches().expect("clear global caches");

    if let Some(ref original_config) = original {
        try_set_current_config(original_config).expect("restore current config");
    }
}

#[test]
fn config_active_display_view_mutation_errors_surface_behavior() {
    let _guard = config_runtime_settings_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");

    let empty_display_err = config
        .add_active_display("")
        .expect_err("empty active display name should fail");
    assert!(
        matches!(empty_display_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {empty_display_err:?}"
    );

    let missing_display_err = config
        .remove_active_display("MissingActiveDisplay")
        .expect_err("removing missing active display should fail");
    assert!(
        matches!(missing_display_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {missing_display_err:?}"
    );

    let empty_view_err = config
        .add_active_view("")
        .expect_err("empty active view name should fail");
    assert!(
        matches!(empty_view_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {empty_view_err:?}"
    );

    let missing_view_err = config
        .remove_active_view("MissingActiveView")
        .expect_err("removing missing active view should fail");
    assert!(
        matches!(missing_view_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {missing_view_err:?}"
    );
}

#[test]
fn config_default_display_view_compat_aliases_follow_active_lists_behavior() {
    let _guard = config_runtime_settings_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");

    let display = config.display(0).expect("first display");
    let view = config.view(&display, 0).expect("first view");

    #[allow(deprecated)]
    {
        config
            .set_default_display(&display)
            .expect("compat set_default_display");
        config
            .set_default_view(&view)
            .expect("compat set_default_view");
    }

    assert_eq!(config.active_displays().as_deref(), Some(display.as_str()));
    assert_eq!(config.active_views().as_deref(), Some(view.as_str()));
}
