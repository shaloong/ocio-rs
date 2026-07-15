//! BuiltinTransformRegistry behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/builtin_transform_registry.rs` and `src/transform/builtin.rs`.
//! In bundled/real mode they validate that builtin-style enumeration remains
//! coherent across the registry singleton and the BuiltinTransform helpers.

mod common;
use common::*;

use std::cmp;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::BuiltinTransform;
use ocio_rs::{BuiltinTransformRegistry, TransformDirection};

fn builtin_transform_registry_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn builtin_transform_registry_matches_builtin_transform_helpers() {
    let _guard = builtin_transform_registry_test_lock();
    if is_stub() {
        return;
    }

    let registry = BuiltinTransformRegistry::get().expect("builtin transform registry");
    let registry_count = registry.num_builtins();
    let helper_count = BuiltinTransform::num_builtin_styles();

    assert!(registry_count > 0);
    assert_eq!(registry_count, helper_count);

    let sample_count = cmp::min(registry_count, 5);
    for index in 0..sample_count {
        let registry_style = registry
            .try_builtin_style(index)
            .expect("registry builtin style query")
            .expect("registry builtin style");
        let helper_style = BuiltinTransform::builtin_style(index).expect("helper builtin style");
        let _description = registry
            .try_builtin_description(index)
            .expect("registry builtin description query")
            .expect("registry builtin description");

        assert_eq!(registry_style, helper_style);
        assert!(!registry_style.is_empty());
        assert!(BuiltinTransform::is_valid_builtin_style(&registry_style));
    }
}

#[test]
fn builtin_transform_instance_round_trip_behavior() {
    let _guard = builtin_transform_registry_test_lock();
    if is_stub() {
        return;
    }

    let registry = BuiltinTransformRegistry::get().expect("builtin transform registry");
    let style = registry
        .try_builtin_style(0)
        .expect("first builtin style query")
        .expect("first builtin style");
    let description = registry
        .try_builtin_description(0)
        .expect("first builtin description query")
        .expect("first builtin description");

    assert_eq!(
        registry.try_builtin_style(-1).expect("negative index"),
        None
    );
    assert_eq!(
        registry
            .try_builtin_description(-1)
            .expect("negative index"),
        None
    );

    let transform = BuiltinTransform::create().expect("builtin transform create");
    transform.set_style(&style).expect("set builtin style");
    transform.set_direction(TransformDirection::Inverse);

    assert_eq!(transform.style().as_deref(), Some(style.as_str()));
    assert_eq!(transform.direction(), TransformDirection::Inverse);
    assert_eq!(
        transform.description().as_deref(),
        Some(description.as_str())
    );

    let copy = transform
        .create_editable_copy()
        .expect("builtin transform editable copy");
    assert_eq!(copy.style().as_deref(), Some(style.as_str()));
    assert_eq!(copy.direction(), TransformDirection::Inverse);
}
