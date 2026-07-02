//! FileRules behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after verifying the entry points can
//! be called from other smoke tests. In bundled/real mode they validate actual
//! round-trip behavior for rule metadata and config attachment.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::FileRules;

fn file_rules_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[test]
fn file_rules_insert_rule_round_trip_behavior() {
    let _guard = file_rules_test_lock();
    if is_stub() {
        return;
    }

    let rules = FileRules::create().expect("file_rules create");
    let initial_entries = rules.num_entries();

    rules
        .insert_rule(0, "UnitRule", "raw", "*.exr", "exr")
        .expect("insert_rule");

    assert_eq!(rules.num_entries(), initial_entries + 1);
    assert_eq!(rules.name(0).as_deref(), Some("UnitRule"));
    assert_eq!(rules.color_space(0).as_deref(), Some("raw"));
    assert_eq!(rules.pattern(0).as_deref(), Some("*.exr"));
    assert_eq!(rules.extension(0).as_deref(), Some("exr"));
    assert_eq!(rules.index_for_rule("UnitRule"), 0);
    assert_eq!(rules.rule_index("UnitRule"), Some(0));
    assert_eq!(rules.rule_index("DefinitelyMissingRule"), None);
    assert_eq!(rules.rule_index("bad\0rule"), None);
}

#[test]
fn file_rules_regex_and_custom_keys_round_trip_behavior() {
    let _guard = file_rules_test_lock();
    if is_stub() {
        return;
    }

    let rules = FileRules::create().expect("file_rules create");
    rules
        .insert_rule_regex(0, "RegexRule", "raw", ".*\\.(exr|dpx)")
        .expect("insert_rule_regex");
    rules
        .set_custom_key(0, "camera", "A001")
        .expect("set_custom_key");

    assert_eq!(rules.name(0).as_deref(), Some("RegexRule"));
    assert_eq!(rules.color_space(0).as_deref(), Some("raw"));
    assert_eq!(rules.regex(0).as_deref(), Some(".*\\.(exr|dpx)"));
    assert_eq!(rules.num_custom_keys(0), 1);
    assert_eq!(rules.custom_key_name(0, 0).as_deref(), Some("camera"));
    assert_eq!(rules.custom_key_value(0, 0).as_deref(), Some("A001"));
}

#[test]
fn config_file_rules_attachment_round_trip_behavior() {
    let _guard = file_rules_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let rules = FileRules::create().expect("file_rules create");
    rules
        .insert_rule(0, "ConfigRule", "raw", "*.ocio", "ocio")
        .expect("insert_rule");
    rules
        .set_default_rule_color_space("raw")
        .expect("set_default_rule_color_space");

    config.set_file_rules(&rules);

    let attached = config.file_rules().expect("config file_rules");
    assert!(attached.num_entries() >= 1);
    assert_eq!(attached.name(0).as_deref(), Some("ConfigRule"));
    assert_eq!(attached.color_space(0).as_deref(), Some("raw"));
}

#[test]
fn config_file_rules_drive_filepath_resolution_behavior() {
    let _guard = file_rules_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");
    let rules = FileRules::create().expect("file_rules create");
    rules
        .insert_rule(0, "ExrRule", "raw", "plate_*", "exr")
        .expect("insert exr rule");
    rules
        .insert_rule(1, "MovRule", "raw", "clip_*", "mov")
        .expect("insert mov rule");
    rules
        .set_default_rule_color_space("raw")
        .expect("set default rule color space");

    config.set_file_rules(&rules);

    assert_eq!(
        config
            .color_space_from_filepath_with_rule_index("plate_main.exr")
            .as_ref()
            .map(|(_, rule_index)| *rule_index),
        Some(0)
    );
    assert_eq!(
        config
            .color_space_from_filepath_with_rule_index("clip_proxy.mov")
            .as_ref()
            .map(|(_, rule_index)| *rule_index),
        Some(1)
    );
    assert_eq!(
        config
            .color_space_from_filepath("plate_main.exr")
            .as_deref(),
        Some("raw")
    );
    assert_eq!(
        config
            .color_space_from_filepath("clip_proxy.mov")
            .as_deref(),
        Some("raw")
    );
    assert_eq!(
        config
            .color_space_from_filepath("fallback.unknown")
            .as_deref(),
        Some("raw")
    );
}

#[test]
fn file_rules_invalid_index_reports_error_behavior() {
    let _guard = file_rules_test_lock();
    if is_stub() {
        return;
    }

    let rules = FileRules::create().expect("file_rules create");
    let invalid_index = rules.num_entries() + 100;

    let set_pattern_error = rules
        .set_pattern(invalid_index, "*.exr")
        .expect_err("invalid index should fail");
    assert!(
        matches!(set_pattern_error, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {set_pattern_error:?}"
    );

    let insert_path_search_error = rules
        .try_insert_path_search_rule(invalid_index)
        .expect_err("invalid path-search insertion index should fail");
    assert!(
        matches!(insert_path_search_error, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {insert_path_search_error:?}"
    );

    let remove_default_error = rules
        .try_remove_rule(0)
        .expect_err("default rule removal should fail");
    assert!(
        matches!(remove_default_error, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {remove_default_error:?}"
    );

    let increase_default_error = rules
        .try_increase_rule_priority(0)
        .expect_err("default rule priority increase should fail");
    assert!(
        matches!(increase_default_error, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {increase_default_error:?}"
    );

    rules
        .insert_rule(0, "PriorityRule", "raw", "*.exr", "exr")
        .expect("insert priority test rule");
    let decrease_front_rule_error = rules
        .try_decrease_rule_priority(0)
        .expect_err("front rule should not move onto default rule");
    assert!(
        matches!(decrease_front_rule_error, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {decrease_front_rule_error:?}"
    );
}
