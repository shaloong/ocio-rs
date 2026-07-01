//! GradingPrimaryTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/grading_primary.rs`. In bundled/real mode they validate
//! default/style state, dynamic toggles, editable-copy independence, and real
//! linear grading processor execution.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::grading::GradingPrimary;
use ocio_rs::transform::GradingPrimaryTransform;
use ocio_rs::{GradingStyle, TransformDirection};

fn grading_primary_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configured_linear_grading_primary_transform() -> GradingPrimaryTransform {
    let transform =
        GradingPrimaryTransform::create(GradingStyle::Lin).expect("grading primary create");

    let mut value = GradingPrimary::new(GradingStyle::Lin);
    value.offset.green = 0.1;
    value.exposure.red = 1.0;
    transform.set_value(&value);

    transform
}

#[test]
fn grading_primary_default_style_dynamic_and_copy_behavior() {
    let _guard = grading_primary_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform =
        GradingPrimaryTransform::create(GradingStyle::Lin).expect("grading primary create");
    let value = transform.value();

    assert_eq!(transform.style(), GradingStyle::Lin);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert!(!transform.is_dynamic());

    assert_close(value.contrast.red, 1.0, 1e-10);
    assert_close(value.contrast.green, 1.0, 1e-10);
    assert_close(value.contrast.blue, 1.0, 1e-10);
    assert_close(value.contrast.master, 1.0, 1e-10);
    assert_close(value.gamma.red, 1.0, 1e-10);
    assert_close(value.gamma.green, 1.0, 1e-10);
    assert_close(value.gamma.blue, 1.0, 1e-10);
    assert_close(value.gamma.master, 1.0, 1e-10);
    assert_close(value.gain.red, 1.0, 1e-10);
    assert_close(value.gain.green, 1.0, 1e-10);
    assert_close(value.gain.blue, 1.0, 1e-10);
    assert_close(value.gain.master, 1.0, 1e-10);
    assert_close(value.saturation, 1.0, 1e-10);
    assert_close(value.pivot, 0.18, 1e-10);
    assert_close(value.pivot_black, 0.0, 1e-10);
    assert_close(value.pivot_white, 1.0, 1e-10);
    assert_eq!(value.clamp_black, GradingPrimary::no_clamp_black());
    assert_eq!(value.clamp_white, GradingPrimary::no_clamp_white());

    transform.make_dynamic();
    assert!(transform.is_dynamic());
    transform.make_non_dynamic();
    assert!(!transform.is_dynamic());

    let copy = transform
        .create_editable_copy()
        .expect("grading primary editable copy");
    copy.set_direction(TransformDirection::Inverse);

    let mut copy_value = copy.value();
    copy_value.offset.blue = 0.25;
    copy.set_value(&copy_value);

    assert_eq!(copy.direction(), TransformDirection::Inverse);
    assert_close(copy.value().offset.blue, 0.25, 1e-10);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert_close(transform.value().offset.blue, 0.0, 1e-10);

    let mut changed = transform.value();
    changed.offset.red = 0.2;
    transform.set_value(&changed);
    transform.set_style(GradingStyle::Video);

    let reset = transform.value();
    let expected = GradingPrimary::new(GradingStyle::Video);
    assert_eq!(transform.style(), GradingStyle::Video);
    assert_eq!(reset, expected);
}

#[test]
fn grading_primary_linear_processor_forward_inverse_behavior() {
    let _guard = grading_primary_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = configured_linear_grading_primary_transform();

    let forward_cpu = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .expect("forward processor")
        .default_cpu_processor()
        .expect("forward cpu");
    let inverse_cpu = config
        .processor_from_transform(&transform, TransformDirection::Inverse)
        .expect("inverse processor")
        .default_cpu_processor()
        .expect("inverse cpu");

    let original = [0.25f32, 0.5, 0.125, 0.75];
    let mut graded = original;
    forward_cpu.apply_rgba(&mut graded);

    // In linear grading with identity contrast/saturation and no clamp, the
    // active controls here reduce to:
    //   red'   = red * 2^1
    //   green' = green + 0.1
    assert_close(graded[0] as f64, 0.5, 2e-5);
    assert_close(graded[1] as f64, 0.6, 2e-5);
    assert_close(graded[2] as f64, original[2] as f64, 2e-5);
    assert_close(graded[3] as f64, original[3] as f64, 2e-5);

    let mut restored = graded;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 5e-5);
    assert_close(restored[1] as f64, original[1] as f64, 5e-5);
    assert_close(restored[2] as f64, original[2] as f64, 5e-5);
    assert_close(restored[3] as f64, original[3] as f64, 5e-5);
}
