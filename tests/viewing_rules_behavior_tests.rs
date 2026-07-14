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
    assert_eq!(
        rules.try_name(0).expect("rule name").as_deref(),
        Some("SceneRule")
    );

    rules.add_color_space(0, "raw").expect("add color space");
    rules
        .set_custom_key(0, "camera", "A001")
        .expect("set custom key");

    assert_eq!(rules.try_num_color_spaces(0).expect("color-space count"), 1);
    assert_eq!(
        rules
            .try_color_space(0, 0)
            .expect("rule color space")
            .as_deref(),
        Some("raw")
    );
    assert_eq!(rules.try_num_custom_keys(0).expect("custom-key count"), 1);
    assert_eq!(
        rules
            .try_custom_key_name(0, 0)
            .expect("custom key name")
            .as_deref(),
        Some("camera")
    );
    assert_eq!(
        rules
            .try_custom_key_value(0, 0)
            .expect("custom key value")
            .as_deref(),
        Some("A001")
    );

    let copy = rules.create_editable_copy().expect("editable copy");
    copy.insert_rule(1, "EncodingRule")
        .expect("insert copy rule");
    copy.add_encoding(1, "scene-linear").expect("add encoding");

    assert_eq!(copy.num_entries(), 2);
    assert_eq!(
        copy.try_name(1).expect("copy rule name").as_deref(),
        Some("EncodingRule")
    );
    assert_eq!(copy.try_num_encodings(1).expect("encoding count"), 1);
    assert_eq!(
        copy.try_encoding(1, 0)
            .expect("copy rule encoding")
            .as_deref(),
        Some("scene-linear")
    );

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

    config
        .set_viewing_rules_object(&rules)
        .expect("attach viewing rules");

    let attached = config.viewing_rules().expect("attached viewing rules");
    assert!(config
        .try_viewing_rules()
        .expect("attached viewing rules query")
        .is_some());
    assert_eq!(attached.num_entries(), 1);
    assert_eq!(attached.name(0).as_deref(), Some("SceneRule"));
    assert_eq!(attached.num_color_spaces(0), 1);
    assert_eq!(attached.color_space(0, 0).as_deref(), Some("raw"));
    assert_eq!(attached.num_custom_keys(0), 1);
    assert_eq!(attached.custom_key_name(0, 0).as_deref(), Some("camera"));
    assert_eq!(attached.custom_key_value(0, 0).as_deref(), Some("A001"));
}

#[test]
fn config_viewing_rules_handle_survives_parent_drop_behavior() {
    let _guard = viewing_rules_test_lock();
    if is_stub() {
        return;
    }

    let attached = {
        let config = create_test_config().expect("raw config");
        let rules = ViewingRules::create().expect("viewing rules create");
        rules
            .insert_rule(0, "SurvivesParentDrop")
            .expect("insert rule");
        config
            .set_viewing_rules_object(&rules)
            .expect("attach viewing rules");
        config.viewing_rules().expect("attached viewing rules")
    };

    assert_eq!(attached.num_entries(), 1);
    assert_eq!(attached.name(0).as_deref(), Some("SurvivesParentDrop"));
}

#[test]
#[allow(deprecated)]
fn raw_viewing_rules_handle_is_owned_and_destroyable_behavior() {
    let _guard = viewing_rules_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let rules = ViewingRules::create().expect("viewing rules create");
    rules.insert_rule(0, "RawHandleRule").expect("insert rule");
    config
        .set_viewing_rules_object(&rules)
        .expect("attach viewing rules");

    let raw = unsafe { config.get_viewing_rules() };
    assert!(!raw.is_null(), "raw viewing-rules handle");
    unsafe { ocio_sys::ocio_viewing_rules_destroy(raw) };
    assert!(config
        .try_viewing_rules()
        .expect("viewing rules remain attached")
        .is_some());
}

#[test]
fn viewing_rules_mutation_errors_surface_behavior() {
    let _guard = viewing_rules_test_lock();
    if is_stub() {
        return;
    }

    let rules = ViewingRules::create().expect("viewing rules create");
    rules
        .insert_rule(0, "ColorSpaceRule")
        .expect("insert color-space rule");
    rules
        .add_color_space(0, "raw")
        .expect("add initial color space");

    let add_encoding_err = rules
        .add_encoding(0, "scene-linear")
        .expect_err("encoding should fail when colorspaces exist");
    assert!(
        matches!(add_encoding_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {add_encoding_err:?}"
    );

    let remove_missing_color_space_err = rules
        .try_remove_color_space(0, 1)
        .expect_err("missing color-space index should fail");
    assert!(
        matches!(remove_missing_color_space_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {remove_missing_color_space_err:?}"
    );

    rules
        .insert_rule(1, "EncodingRule")
        .expect("insert encoding rule");
    rules
        .add_encoding(1, "scene-linear")
        .expect("add initial encoding");

    let add_color_space_err = rules
        .add_color_space(1, "raw")
        .expect_err("colorspace should fail when encodings exist");
    assert!(
        matches!(add_color_space_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {add_color_space_err:?}"
    );

    let duplicate_rule_err = rules
        .insert_rule(2, "EncodingRule")
        .expect_err("duplicate rule name should fail");
    assert!(
        matches!(duplicate_rule_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {duplicate_rule_err:?}"
    );

    let remove_missing_rule_err = rules
        .try_remove_rule(5)
        .expect_err("missing rule index should fail");
    assert!(
        matches!(remove_missing_rule_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {remove_missing_rule_err:?}"
    );

    let missing_name_err = rules
        .try_name(5)
        .expect_err("missing rule name should fail");
    assert!(
        matches!(missing_name_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {missing_name_err:?}"
    );

    let missing_count_err = rules
        .try_num_color_spaces(5)
        .expect_err("missing rule color-space count should fail");
    assert!(
        matches!(missing_count_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {missing_count_err:?}"
    );
}
