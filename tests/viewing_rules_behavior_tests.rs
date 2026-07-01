//! ViewingRules behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/viewing_rules.rs` and `src/config.rs`. In bundled/real mode they
//! validate rule mutation, editable-copy independence, and config attachment.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::ViewingRules;

fn viewing_rules_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn viewing_rules_round_trip_and_copy_behavior() {
    let _guard = viewing_rules_test_lock();
    if is_stub() {
        return;
    }

    let rules = ViewingRules::create().expect("viewing rules create");
    assert_eq!(rules.num_entries(), 0);

    rules.insert_rule(0, "SceneRule").expect("insert rule");
    assert_eq!(rules.num_entries(), 1);
    assert_eq!(rules.index_for_rule("SceneRule"), 0);
    assert_eq!(rules.rule_index("SceneRule"), Some(0));
    assert_eq!(rules.rule_index("DefinitelyMissingRule"), None);
    assert_eq!(rules.rule_index("bad\0rule"), None);
    assert_eq!(rules.name(0).as_deref(), Some("SceneRule"));

    rules.add_color_space(0, "raw").expect("add color space");
    rules
        .set_custom_key(0, "camera", "A001")
        .expect("set custom key");

    assert_eq!(rules.num_color_spaces(0), 1);
    assert_eq!(rules.color_space(0, 0).as_deref(), Some("raw"));
    assert_eq!(rules.num_custom_keys(0), 1);
    assert_eq!(rules.custom_key_name(0, 0).as_deref(), Some("camera"));
    assert_eq!(rules.custom_key_value(0, 0).as_deref(), Some("A001"));

    let copy = rules.create_editable_copy().expect("editable copy");
    copy.insert_rule(1, "EncodingRule")
        .expect("insert copy rule");
    copy.add_encoding(1, "scene-linear").expect("add encoding");

    assert_eq!(copy.num_entries(), 2);
    assert_eq!(copy.name(1).as_deref(), Some("EncodingRule"));
    assert_eq!(copy.num_encodings(1), 1);
    assert_eq!(copy.encoding(1, 0).as_deref(), Some("scene-linear"));

    assert_eq!(rules.num_entries(), 1);
    assert_eq!(rules.num_encodings(0), 0);
}

#[test]
fn config_viewing_rules_attachment_behavior() {
    let _guard = viewing_rules_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config()
        .expect("raw config")
        .create_editable_copy()
        .expect("editable config copy");
    let rules = ViewingRules::create().expect("viewing rules create");
    rules.insert_rule(0, "SceneRule").expect("insert rule");
    rules.add_color_space(0, "raw").expect("add color space");
    rules
        .set_custom_key(0, "camera", "A001")
        .expect("set custom key");

    config.set_viewing_rules_object(&rules);

    let attached = config.viewing_rules().expect("attached viewing rules");
    assert_eq!(attached.num_entries(), 1);
    assert_eq!(attached.name(0).as_deref(), Some("SceneRule"));
    assert_eq!(attached.num_color_spaces(0), 1);
    assert_eq!(attached.color_space(0, 0).as_deref(), Some("raw"));
    assert_eq!(attached.num_custom_keys(0), 1);
    assert_eq!(attached.custom_key_name(0, 0).as_deref(), Some("camera"));
    assert_eq!(attached.custom_key_value(0, 0).as_deref(), Some("A001"));
}
