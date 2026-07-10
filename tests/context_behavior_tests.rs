//! Context behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/context.rs`. In bundled/real mode they validate that search paths,
//! working directories, string variables, editable copies, and file resolution
//! behave like real OCIO contexts.

mod common;
use common::*;

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use ocio_rs::{Context, EnvironmentMode};

fn context_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn unique_test_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    std::env::temp_dir().join(format!("ocio-rs-{label}-{nanos}"))
}

fn path_str(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[test]
fn context_string_vars_round_trip_and_copy_behavior() {
    let _guard = context_test_lock();
    if is_stub() {
        return;
    }

    let ctx = Context::create().expect("context create");
    ctx.set_string_var("SHOT", "abc123").expect("set SHOT");
    ctx.set_string_var("SEQ", "sq01").expect("set SEQ");

    assert_eq!(ctx.string_var("SHOT").as_deref(), Some("abc123"));
    assert_eq!(ctx.string_var("SEQ").as_deref(), Some("sq01"));
    assert_eq!(ctx.num_string_vars(), 2);

    let resolved = ctx
        .resolve_string_var("${SEQ}/${SHOT}/plate.exr")
        .expect("resolve_string_var");
    assert_eq!(resolved, "sq01/abc123/plate.exr");

    let copy = ctx.create_editable_copy().expect("editable copy");
    assert_eq!(copy.string_var("SHOT").as_deref(), Some("abc123"));
    assert_eq!(copy.string_var("SEQ").as_deref(), Some("sq01"));

    copy.set_string_var("SHOT", "xyz999")
        .expect("override SHOT");
    assert_eq!(copy.string_var("SHOT").as_deref(), Some("xyz999"));
    assert_eq!(ctx.string_var("SHOT").as_deref(), Some("abc123"));
}

#[test]
fn context_add_string_vars_merges_other_context_behavior() {
    let _guard = context_test_lock();
    if is_stub() {
        return;
    }

    let base = Context::create().expect("base context");
    base.set_string_var("SHOW", "demo").expect("set SHOW");

    let overlay = Context::create().expect("overlay context");
    overlay.set_string_var("SHOT", "010").expect("set SHOT");
    overlay.set_string_var("TASK", "comp").expect("set TASK");

    base.add_string_vars(&overlay);

    assert_eq!(base.string_var("SHOW").as_deref(), Some("demo"));
    assert_eq!(base.string_var("SHOT").as_deref(), Some("010"));
    assert_eq!(base.string_var("TASK").as_deref(), Some("comp"));
}

#[test]
fn context_search_paths_and_working_dir_round_trip_behavior() {
    let _guard = context_test_lock();
    if is_stub() {
        return;
    }

    let ctx = Context::create().expect("context create");
    ctx.clear_search_paths();

    let dir_a = unique_test_dir("search-a");
    let dir_b = unique_test_dir("search-b");
    fs::create_dir_all(&dir_a).expect("create dir_a");
    fs::create_dir_all(&dir_b).expect("create dir_b");

    ctx.add_search_path(path_str(&dir_a)).expect("add dir_a");
    ctx.add_search_path(path_str(&dir_b)).expect("add dir_b");
    ctx.set_working_dir(path_str(&dir_a))
        .expect("set working dir");

    assert_eq!(ctx.num_search_paths(), 2);
    assert_eq!(
        ctx.search_path_by_index(0).as_deref(),
        Some(dir_a.to_string_lossy().as_ref())
    );
    assert_eq!(
        ctx.search_path_by_index(1).as_deref(),
        Some(dir_b.to_string_lossy().as_ref())
    );
    assert_eq!(
        ctx.working_dir().as_deref(),
        Some(dir_a.to_string_lossy().as_ref())
    );

    let copy = ctx.create_editable_copy().expect("editable copy");
    assert_eq!(copy.num_search_paths(), 2);
    assert_eq!(
        copy.search_path_by_index(0).as_deref(),
        Some(dir_a.to_string_lossy().as_ref())
    );
    assert_eq!(
        copy.search_path_by_index(1).as_deref(),
        Some(dir_b.to_string_lossy().as_ref())
    );
    assert_eq!(
        copy.working_dir().as_deref(),
        Some(dir_a.to_string_lossy().as_ref())
    );

    let _ = fs::remove_dir_all(&dir_a);
    let _ = fs::remove_dir_all(&dir_b);
}

#[test]
fn context_resolve_file_location_uses_working_dir_as_fallback_behavior() {
    let _guard = context_test_lock();
    if is_stub() {
        return;
    }

    let root = unique_test_dir("resolve-working-file");
    let working_dir = root.join("working");
    fs::create_dir_all(&working_dir).expect("create working dir");

    let working_file = working_dir.join("working_only.spi1d");
    fs::write(&working_file, "# working file\n").expect("write working file");

    let ctx = Context::create().expect("context create");
    ctx.clear_search_paths();
    ctx.set_working_dir(path_str(&working_dir))
        .expect("set working dir");

    let resolved_working = ctx
        .resolve_file_location("working_only.spi1d")
        .expect("resolve working file");

    assert_eq!(PathBuf::from(resolved_working), working_file);

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn context_resolve_file_location_uses_explicit_search_paths_behavior() {
    let _guard = context_test_lock();
    if is_stub() {
        return;
    }

    let root = unique_test_dir("resolve-search-file");
    let working_dir = root.join("working");
    let search_dir = root.join("search");
    fs::create_dir_all(&working_dir).expect("create working dir");
    fs::create_dir_all(&search_dir).expect("create search dir");

    let search_file = search_dir.join("search_only.spi3d");
    fs::write(&search_file, "# search file\n").expect("write search file");

    let ctx = Context::create().expect("context create");
    ctx.clear_search_paths();
    ctx.set_working_dir(path_str(&working_dir))
        .expect("set working dir");
    ctx.add_search_path(path_str(&search_dir))
        .expect("add search dir");

    let resolved_search = ctx
        .resolve_file_location("search_only.spi3d")
        .expect("resolve search file");

    assert_eq!(PathBuf::from(resolved_search), search_file);

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn context_cache_id_changes_with_mutation_behavior() {
    let _guard = context_test_lock();
    if is_stub() {
        return;
    }

    let ctx = Context::create().expect("context create");
    let initial_cache_id = ctx.cache_id().expect("initial cache id");

    ctx.set_string_var("SHOT", "abc123").expect("set SHOT");
    let after_string_var = ctx.cache_id().expect("cache id after string var");
    assert_ne!(after_string_var, initial_cache_id);

    ctx.set_environment_mode(EnvironmentMode::LoadAll)
        .expect("set environment mode");
    assert_eq!(ctx.environment_mode(), EnvironmentMode::LoadAll);
}

#[test]
fn context_load_environment_honors_selected_mode_behavior() {
    let _guard = context_test_lock();
    if is_stub() {
        return;
    }

    const VAR: &str = "OCIO_RS_CONTEXT_AUTHORED_TEST";

    let ctx = Context::create().expect("context create");
    ctx.clear_string_vars();
    ctx.set_string_var(VAR, "authored-default")
        .expect("set authored default");
    ctx.set_environment_mode(EnvironmentMode::LoadPredefined)
        .expect("select predefined mode");
    ctx.load_environment().expect("load predefined environment");
    assert_eq!(ctx.string_var(VAR).as_deref(), Some("authored-default"));

    ctx.clear_string_vars();
    ctx.load_environment()
        .expect("reload predefined environment");
    assert_eq!(ctx.num_string_vars(), 0);
    let predefined_cache_id = ctx.cache_id().expect("predefined cache id");

    ctx.set_environment_mode(EnvironmentMode::LoadAll)
        .expect("select all mode");
    ctx.load_environment().expect("load complete environment");
    assert_eq!(ctx.environment_mode(), EnvironmentMode::LoadAll);
    assert_ne!(
        ctx.cache_id().as_deref(),
        Some(predefined_cache_id.as_str())
    );
}
