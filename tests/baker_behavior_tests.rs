//! Baker behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/baker.rs`. In bundled/real mode they validate setter round trips,
//! format enumeration, and actual LUT text generation.

mod common;
use common::*;

use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use ocio_rs::{Baker, Config};

fn baker_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn unique_temp_path(label: &str, extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    std::env::temp_dir().join(format!("ocio-rs-{label}-{nanos}.{extension}"))
}

#[test]
fn baker_property_round_trip_behavior() {
    let _guard = baker_test_lock();
    if is_stub() {
        return;
    }

    let baker = Baker::create().expect("baker create");
    let config = Config::raw().expect("raw config");
    baker.set_config(&config);

    baker.set_format("resolve_cube").expect("set format");
    baker.set_input_space("raw").expect("set input");
    baker.set_target_space("raw").expect("set target");
    baker.set_shaper_space("raw").expect("set shaper");
    baker.set_looks("").expect("set looks");
    baker.set_shaper_size(8);
    baker.set_cube_size(2);

    assert_eq!(baker.format().as_deref(), Some("resolve_cube"));
    assert_eq!(baker.input_space().as_deref(), Some("raw"));
    assert_eq!(baker.target_space().as_deref(), Some("raw"));
    assert_eq!(baker.shaper_space().as_deref(), Some("raw"));
    assert_eq!(baker.looks().as_deref(), Some(""));
    assert_eq!(baker.shaper_size(), 8);
    assert_eq!(baker.cube_size(), 2);

    let attached_config = baker.config().expect("baker config");
    attached_config
        .validate()
        .expect("attached config validate");
    assert!(attached_config.num_color_spaces() > 0);
}

#[test]
fn baker_static_format_registry_contains_resolve_cube_behavior() {
    let _guard = baker_test_lock();
    if is_stub() {
        return;
    }

    let num_formats = Baker::num_formats();
    assert!(num_formats > 0);

    let mut found_resolve_cube = false;
    for index in 0..num_formats {
        let name = Baker::format_name_by_index(index).expect("format name");
        let ext = Baker::format_extension_by_index(index).expect("format extension");
        assert!(!name.is_empty());
        assert!(!ext.is_empty());

        if name == "resolve_cube" {
            found_resolve_cube = true;
            assert_eq!(ext, "cube");
        }
    }

    assert!(found_resolve_cube, "expected resolve_cube baker format");
}

#[test]
fn baker_bake_to_string_and_file_behavior() {
    let _guard = baker_test_lock();
    if is_stub() {
        return;
    }

    let baker = Baker::create().expect("baker create");
    let config = Config::raw().expect("raw config");
    baker.set_config(&config);
    baker.set_format("resolve_cube").expect("set format");
    baker.set_input_space("raw").expect("set input");
    baker.set_target_space("raw").expect("set target");
    baker.set_cube_size(2);

    let baked = baker.bake_to_string().expect("bake_to_string");
    assert!(!baked.trim().is_empty());
    assert!(baked.contains("LUT_1D_SIZE 2"));
    assert!(baked.lines().any(|line| line.contains("0.000000")));

    let output_path = unique_temp_path("baker", "cube");
    baker
        .bake(output_path.to_string_lossy())
        .expect("bake file");
    let written = fs::read_to_string(&output_path).expect("read baked file");
    assert_eq!(written, baked);

    let _ = fs::remove_file(output_path);
}

#[test]
fn baker_invalid_format_reports_error_behavior() {
    let _guard = baker_test_lock();
    if is_stub() {
        return;
    }

    let baker = Baker::create().expect("baker create");
    let err = baker
        .set_format("definitely_not_a_real_bake_format")
        .expect_err("invalid baker format should fail");
    assert!(
        matches!(err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );
}
