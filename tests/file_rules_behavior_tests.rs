//! FileRules behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after verifying the entry points can
//! be called from other smoke tests. In bundled/real mode they validate actual
//! round-trip behavior for rule metadata and config attachment.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::FileRules;

#[test]
#[cfg(ocio_stub)]
fn stub_rule_lookup_reports_unavailable() {
    let rules = FileRules::create().expect("file_rules create");
    assert!(rules.try_index_for_rule("anything").is_err());
    assert_eq!(rules.rule_index("anything"), None);
}

fn file_rules_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn custom_key_count_preserves_invalid_rule_errors() {
    let rules = FileRules::create().unwrap();
    assert_eq!(rules.try_num_custom_keys(0).unwrap(), 0);
    assert!(matches!(
        rules.try_num_custom_keys(rules.num_entries()),
        Err(ocio_rs::OcioError::Ocio(_))
    ));
    assert_eq!(rules.num_custom_keys(rules.num_entries()), 0);
    assert_eq!(rules.try_num_custom_keys(0).unwrap(), 0);
}

#[test]
#[cfg(target_pointer_width = "32")]
fn oversized_indices_do_not_alias_existing_rules() {
    let rules = FileRules::create().unwrap();
    let oversized = u64::from(u32::MAX) + 1;
    let entries = rules.num_entries();
    assert!(matches!(
        rules.try_remove_rule(oversized),
        Err(ocio_rs::OcioError::InvalidInput(_))
    ));
    assert!(matches!(
        rules.try_num_custom_keys(oversized),
        Err(ocio_rs::OcioError::InvalidInput(_))
    ));
    assert!(matches!(
        rules.try_custom_key_name(0, oversized),
        Err(ocio_rs::OcioError::InvalidInput(_))
    ));
    assert_eq!(rules.num_entries(), entries);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
    assert_eq!(
        rules.try_name(0).expect("rule name query").as_deref(),
        Some("UnitRule")
    );
    assert_eq!(
        rules
            .try_color_space(0)
            .expect("rule color-space query")
            .as_deref(),
        Some("raw")
    );
    assert_eq!(
        rules.try_pattern(0).expect("rule pattern query").as_deref(),
        Some("*.exr")
    );
    assert_eq!(
        rules
            .try_extension(0)
            .expect("rule extension query")
            .as_deref(),
        Some("exr")
    );
    assert_eq!(rules.index_for_rule("UnitRule"), 0);
    assert_eq!(
        rules
            .try_index_for_rule("UnitRule")
            .expect("existing rule index"),
        0
    );
    assert_eq!(rules.rule_index("UnitRule"), Some(0));
    assert_eq!(rules.rule_index("unitrule"), Some(0));
    assert!(matches!(
        rules.try_index_for_rule("DefinitelyMissingRule"),
        Err(ocio_rs::OcioError::Ocio(_))
    ));
    assert_eq!(rules.index_for_rule("DefinitelyMissingRule"), u64::MAX);
    assert_eq!(rules.rule_index("DefinitelyMissingRule"), None);
    assert_eq!(rules.rule_index("bad\0rule"), None);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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

    assert_eq!(
        rules.try_name(0).expect("rule name query").as_deref(),
        Some("RegexRule")
    );
    assert_eq!(
        rules
            .try_color_space(0)
            .expect("rule color-space query")
            .as_deref(),
        Some("raw")
    );
    assert_eq!(
        rules.try_regex(0).expect("rule regex query").as_deref(),
        Some(".*\\.(exr|dpx)")
    );
    assert_eq!(rules.num_custom_keys(0), 1);
    assert_eq!(
        rules
            .try_custom_key_name(0, 0)
            .expect("custom-key name query")
            .as_deref(),
        Some("camera")
    );
    assert_eq!(
        rules
            .try_custom_key_value(0, 0)
            .expect("custom-key value query")
            .as_deref(),
        Some("A001")
    );
    rules.set_custom_key(0, "camera", "").unwrap();
    assert_eq!(rules.try_num_custom_keys(0).unwrap(), 0);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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

    config.set_file_rules(&rules).expect("attach file rules");

    let attached = config.file_rules().expect("config file_rules");
    assert!(attached.num_entries() >= 1);
    assert_eq!(attached.name(0).as_deref(), Some("ConfigRule"));
    assert_eq!(attached.color_space(0).as_deref(), Some("raw"));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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

    config.set_file_rules(&rules).expect("attach file rules");

    // File rules still select the default when strict parsing is enabled;
    // callers can distinguish that case by the matched rule index.
    config.try_set_strict_parsing_enabled(true).unwrap();

    assert_eq!(
        config
            .try_color_space_from_filepath_with_rule_index("plate_main.exr")
            .expect("plate filepath query")
            .as_ref()
            .map(|(_, rule_index)| *rule_index),
        Some(0)
    );
    assert_eq!(
        config
            .try_color_space_from_filepath_with_rule_index("clip_proxy.mov")
            .expect("clip filepath query")
            .as_ref()
            .map(|(_, rule_index)| *rule_index),
        Some(1)
    );
    assert!(config
        .try_color_space_from_filepath_with_rule_index("plate_main\0.exr")
        .is_err());
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
    assert_eq!(
        config
            .try_color_space_from_filepath_with_rule_index("fallback.unknown")
            .unwrap(),
        Some(("raw".to_owned(), 2))
    );
    assert!(!config
        .try_filepath_only_matches_default_rule("plate_main.exr")
        .unwrap());
    assert!(config
        .try_filepath_only_matches_default_rule("fallback.unknown")
        .unwrap());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn file_rule_order_and_native_reserved_names() {
    let config = create_test_config().unwrap();
    let rules = FileRules::create().unwrap();
    assert_eq!(rules.rule_index(FileRules::DEFAULT_RULE_NAME), Some(0));
    rules
        .insert_rule(0, "Pattern", "raw", "plate_*", "exr")
        .unwrap();
    rules
        .insert_rule_regex(1, "Regex", "raw", ".*\\.exr")
        .unwrap();
    rules.try_insert_path_search_rule(2).unwrap();
    assert_eq!(
        rules.rule_index(FileRules::FILE_PATH_SEARCH_RULE_NAME),
        Some(2)
    );
    assert_eq!(rules.rule_index(FileRules::DEFAULT_RULE_NAME), Some(3));

    let matching_name = || {
        let (_, index) = config
            .try_color_space_from_filepath_with_rule_index("plate_a.exr")
            .unwrap()
            .unwrap();
        config
            .try_const_file_rules()
            .unwrap()
            .unwrap()
            .name(index)
            .unwrap()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    };
    config.set_file_rules(&rules).unwrap();
    assert_eq!(matching_name(), "Pattern");
    rules.try_increase_rule_priority(1).unwrap();
    // Config::setFileRules installs an independent copy.
    assert_eq!(matching_name(), "Pattern");
    config.set_file_rules(&rules).unwrap();
    assert_eq!(matching_name(), "Regex");
    rules.try_decrease_rule_priority(0).unwrap();
    config.set_file_rules(&rules).unwrap();
    assert_eq!(matching_name(), "Pattern");
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn file_rules_invalid_index_reports_error_behavior() {
    let _guard = file_rules_test_lock();
    if is_stub() {
        return;
    }

    let rules = FileRules::create().expect("file_rules create");
    let invalid_index = rules.num_entries() + 100;

    let name_error = rules
        .try_name(invalid_index)
        .expect_err("invalid index query should fail");
    assert!(
        matches!(name_error, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {name_error:?}"
    );

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
