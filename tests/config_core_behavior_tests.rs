//! Core Config behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/config.rs`. In bundled/real mode they validate file/env/stream loading,
//! config metadata and role access, search-path mutation, strict parsing and
//! luma round trips, cache-id mutation, and serialization.

mod common;
use common::*;

use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::{Config, OcioError};

fn config_core_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn test_data_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(rel)
}

fn packaged_context_test1_path() -> PathBuf {
    if cfg!(windows) {
        test_data_path("configs/context_test1/context_test1_windows.ocioz")
    } else {
        test_data_path("configs/context_test1/context_test1_linux.ocioz")
    }
}

fn assert_context_test1_metadata(config: &Config) {
    assert_eq!(config.major_version(), 2);
    assert_eq!(config.minor_version(), 0);
    assert_eq!(config.default_display().as_deref(), Some("sRGB"));
    assert_eq!(config.default_view("sRGB").as_deref(), Some("Raw"));
    assert_eq!(config.num_looks(), 1);
    assert_eq!(config.look_name_by_index(0).as_deref(), Some("shot_look"));
    assert!(config.num_color_spaces() >= 11);

    assert!(config.has_role("default"));
    assert!(config.has_role("scene_linear"));
    assert_eq!(config.role_color_space("default").as_deref(), Some("raw"));
    assert_eq!(
        config.role_color_space("scene_linear").as_deref(),
        Some("reference")
    );

    assert_eq!(config.num_search_paths(), 6);
    assert_eq!(config.search_path_by_index(0).as_deref(), Some("./$SHOT"));
    assert_eq!(config.search_path_by_index(1).as_deref(), Some("shot1"));
    assert_eq!(config.search_path_by_index(2).as_deref(), Some("shot2"));
    assert_eq!(config.search_path_by_index(3).as_deref(), Some("shot3"));
    assert_eq!(
        config.search_path_by_index(4).as_deref(),
        Some("shot3/subdir")
    );
    assert_eq!(config.search_path_by_index(5).as_deref(), Some("."));

    config.validate().expect("validate config");
}

#[test]
fn config_from_file_env_and_stream_load_context_test1_behavior() {
    let _guard = config_core_test_lock();
    if is_stub() {
        return;
    }

    let config_path = test_data_path("configs/context_test1/config.ocio");
    let config_text = fs::read_to_string(&config_path).expect("read config text");

    let from_file =
        Config::from_file(config_path.to_string_lossy()).expect("load config from file");
    assert_context_test1_metadata(&from_file);

    let prev = std::env::var_os("OCIO");
    unsafe {
        std::env::set_var("OCIO", &config_path);
    }
    let from_env = Config::from_env().expect("load config from env");
    match prev {
        Some(value) => unsafe { std::env::set_var("OCIO", value) },
        None => unsafe { std::env::remove_var("OCIO") },
    }
    assert_context_test1_metadata(&from_env);

    let from_stream = Config::from_stream(config_text).expect("load config from stream");
    assert_context_test1_metadata(&from_stream);
}

#[test]
fn config_current_context_exposes_environment_defaults_behavior() {
    let _guard = config_core_test_lock();
    if is_stub() {
        return;
    }

    let config_path = test_data_path("configs/context_test1/config.ocio");
    let working_dir = config_path.parent().expect("config parent");
    let config = Config::from_file(config_path.to_string_lossy()).expect("load config from file");
    config
        .set_working_dir(working_dir.to_string_lossy())
        .expect("set working dir");

    let context = config.current_context().expect("current context");
    assert_eq!(context.string_var("SHOT").as_deref(), Some("shot4"));
    assert_eq!(
        context.string_var("LUT_PATH").as_deref(),
        Some("shot3/lut1.clf")
    );
    assert_eq!(context.string_var("CCCID").as_deref(), Some("look-02"));
    assert_eq!(context.string_var("CAMERA").as_deref(), Some("arri"));
    assert_eq!(
        context.resolve_string_var("${SHOT}/lut1.clf").as_deref(),
        Some("shot4/lut1.clf")
    );
}

#[test]
fn config_environment_declarations_and_loading_behavior() {
    let _guard = config_core_test_lock();
    if is_stub() {
        return;
    }

    const SHOT: &str = "SHOT";
    let config_path = test_data_path("configs/context_test1/config.ocio");
    let config = Config::from_file(config_path.to_string_lossy()).expect("load config from file");

    assert_eq!(config.num_environment_vars(), 4);
    assert_eq!(
        config.environment_var_default(SHOT).as_deref(),
        Some("shot4")
    );
    assert_eq!(
        config.environment_var_default("LUT_PATH").as_deref(),
        Some("shot3/lut1.clf")
    );

    config
        .set_environment_mode(ocio_rs::EnvironmentMode::LoadPredefined)
        .expect("select predefined mode");
    config
        .load_environment()
        .expect("load configured environment");
    assert_eq!(
        config
            .current_context()
            .expect("current context")
            .string_var(SHOT)
            .as_deref(),
        Some("shot4")
    );

    config
        .clear_environment_vars()
        .expect("clear environment declarations");
    assert_eq!(config.num_environment_vars(), 0);
    assert_eq!(
        config
            .current_context()
            .expect("current context")
            .string_var(SHOT),
        Some(String::new())
    );
}

#[test]
fn config_from_packaged_ocioz_loads_context_test1_behavior() {
    let _guard = config_core_test_lock();
    if is_stub() {
        return;
    }

    let packaged_path = packaged_context_test1_path();
    let packaged =
        Config::from_file(packaged_path.to_string_lossy()).expect("load packaged ocioz config");
    assert_context_test1_metadata(&packaged);
}

#[test]
fn config_search_paths_roles_and_serialization_behavior() {
    let _guard = config_core_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");

    config.set_name("UnitConfig").expect("set name");
    config
        .set_description("Unit config description")
        .expect("set description");

    config
        .try_clear_search_paths()
        .expect("clear config search paths");
    config.set_search_path("alpha").expect("set search path");
    config.add_search_path("beta").expect("add search path");
    config.add_search_path("gamma").expect("add search path");

    assert_eq!(config.search_path().as_deref(), Some("alpha:beta:gamma"));
    assert_eq!(config.num_search_paths(), 3);
    assert_eq!(config.search_path_by_index(0).as_deref(), Some("alpha"));
    assert_eq!(config.search_path_by_index(1).as_deref(), Some("beta"));
    assert_eq!(config.search_path_by_index(2).as_deref(), Some("gamma"));

    config.set_role("compositing_log", "raw").expect("set role");
    assert!(config.has_role("compositing_log"));
    assert_eq!(
        config.role_color_space("compositing_log").as_deref(),
        Some("raw")
    );

    let serialized = config
        .serialize()
        .expect("serialize config")
        .expect("real serialized config");
    assert!(serialized.contains("ocio_profile_version"));
    assert!(serialized.contains("name: UnitConfig"));
    assert!(serialized.contains("description: Unit config description"));
    assert!(serialized.contains("compositing_log: raw"));
    assert!(serialized.contains("alpha"));
    assert!(serialized.contains("beta"));
    assert!(serialized.contains("gamma"));
}

#[test]
fn config_archive_returns_payload_for_archivable_file_behavior() {
    let _guard = config_core_test_lock();
    if is_stub() {
        return;
    }

    let config_path = test_data_path("configs/context_test1/config.ocio");
    let config = Config::from_file(config_path.to_string_lossy()).expect("load config from file");

    assert!(config.is_archivable(), "context_test1 should be archivable");

    let archived = config
        .archive()
        .expect("archive config")
        .expect("real archived config");
    assert!(
        !archived.trim().is_empty(),
        "archive payload should not be empty"
    );
}

#[test]
fn config_unarchivable_archive_surfaces_ocio_error_behavior() {
    let _guard = config_core_test_lock();
    if is_stub() {
        return;
    }

    let config = Config::raw().expect("raw config");
    assert!(!config.is_archivable());
    let err = config
        .archive()
        .expect_err("unarchivable config must report an OCIO error");
    assert!(matches!(err, ocio_rs::OcioError::Ocio(_)));
}

#[test]
fn config_cache_id_strict_parsing_and_luma_behavior() {
    let _guard = config_core_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");

    let initial_cache_id = config.cache_id().expect("initial cache id");
    let initial_strict = config.is_strict_parsing_enabled();
    config.set_strict_parsing_enabled(!initial_strict);
    assert_eq!(config.is_strict_parsing_enabled(), !initial_strict);
    config.set_strict_parsing_enabled(initial_strict);
    assert_eq!(config.is_strict_parsing_enabled(), initial_strict);

    if config.num_displays_all() > 0 {
        config
            .set_display_temporary(0, true)
            .expect("mark display temporary");
        assert!(config.is_display_temporary(0));
        config
            .set_display_temporary(0, false)
            .expect("clear temporary display marker");
    }

    let custom_luma = [0.3, 0.59, 0.11];
    config
        .set_default_luma_coefs(&custom_luma)
        .expect("set default luma coefficients");
    let round_trip = config
        .default_luma_coefs()
        .expect("get default luma coefficients");
    assert_close(round_trip[0], custom_luma[0], 1e-12);
    assert_close(round_trip[1], custom_luma[1], 1e-12);
    assert_close(round_trip[2], custom_luma[2], 1e-12);

    config.set_name("CacheMutation").expect("set name");
    config
        .add_search_path("cache/path")
        .expect("add search path");

    let mutated_cache_id = config.cache_id().expect("mutated cache id");

    assert_ne!(mutated_cache_id, initial_cache_id);
}

#[test]
fn config_from_file_reports_real_ocio_errors() {
    let _guard = config_core_test_lock();
    if is_stub() {
        return;
    }

    let missing = test_data_path("configs/does-not-exist.ocio");
    match Config::from_file(missing.to_string_lossy()) {
        Ok(_) => panic!("missing config should fail"),
        Err(err) => assert!(matches!(err, OcioError::Ocio(_))),
    }
}
