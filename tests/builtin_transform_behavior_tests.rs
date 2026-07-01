//! BuiltinTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/builtin.rs`. In bundled/real mode they validate style and
//! description round trips, editable-copy independence, and real processor
//! execution through a known builtin transform style.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::BuiltinTransform;
use ocio_rs::{BuiltinTransformRegistry, TransformDirection};

fn builtin_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn known_builtin_style() -> &'static str {
    "ACEScct_to_ACES2065-1"
}

#[test]
fn builtin_transform_style_description_and_copy_behavior() {
    let _guard = builtin_transform_test_lock();
    if is_stub() {
        return;
    }

    let registry = BuiltinTransformRegistry::get().expect("builtin transform registry");
    let style = known_builtin_style();
    assert!(BuiltinTransform::is_valid_builtin_style(style));

    let transform = BuiltinTransform::create().expect("builtin transform create");
    transform.set_style(style).expect("set builtin style");
    transform.set_direction(TransformDirection::Inverse);

    assert_eq!(transform.style().as_deref(), Some(style));
    assert_eq!(transform.direction(), TransformDirection::Inverse);

    let mut matched_description = None;
    for index in 0..registry.num_builtins() {
        if registry.builtin_style(index).as_deref() == Some(style) {
            matched_description = registry.builtin_description(index);
            break;
        }
    }
    assert_eq!(
        transform.description().as_deref(),
        matched_description.as_deref()
    );

    let copy = transform
        .create_editable_copy()
        .expect("builtin transform editable copy");
    copy.set_direction(TransformDirection::Forward);
    assert_eq!(copy.style().as_deref(), Some(style));
    assert_eq!(copy.direction(), TransformDirection::Forward);
    assert_eq!(transform.direction(), TransformDirection::Inverse);
}

#[test]
fn builtin_transform_processor_round_trip_behavior() {
    let _guard = builtin_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let style = known_builtin_style();
    assert!(BuiltinTransform::is_valid_builtin_style(style));

    let transform = BuiltinTransform::create().expect("builtin transform create");
    transform.set_style(style).expect("set builtin style");

    let forward_processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .expect("forward processor");
    let inverse_processor = config
        .processor_from_transform(&transform, TransformDirection::Inverse)
        .expect("inverse processor");

    assert!(!forward_processor.is_no_op());
    assert!(!inverse_processor.is_no_op());

    let forward_cpu = forward_processor
        .default_cpu_processor()
        .expect("forward cpu processor");
    let inverse_cpu = inverse_processor
        .default_cpu_processor()
        .expect("inverse cpu processor");

    let original = [0.2f32, 0.35, 0.6, 1.0];
    let mut transformed = original;
    forward_cpu.apply_rgba(&mut transformed);

    let changed = (transformed[0] - original[0]).abs() > 1e-5
        || (transformed[1] - original[1]).abs() > 1e-5
        || (transformed[2] - original[2]).abs() > 1e-5;
    assert!(
        changed,
        "builtin transform should change at least one RGB channel"
    );
    assert_close(transformed[3] as f64, original[3] as f64, 1e-6);

    let mut round_tripped = transformed;
    inverse_cpu.apply_rgba(&mut round_tripped);

    assert_close(round_tripped[0] as f64, original[0] as f64, 1e-4);
    assert_close(round_tripped[1] as f64, original[1] as f64, 1e-4);
    assert_close(round_tripped[2] as f64, original[2] as f64, 1e-4);
    assert_close(round_tripped[3] as f64, original[3] as f64, 1e-6);
}
