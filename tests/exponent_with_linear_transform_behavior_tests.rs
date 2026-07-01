//! ExponentWithLinearTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/exponent_with_linear.rs`. In bundled/real mode they validate
//! gamma/offset round trips, negative-style and direction state, editable-copy
//! independence, and real processor execution for positive-domain moncurve
//! inputs above the linear breakpoint.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::ExponentWithLinearTransform;
use ocio_rs::{NegativeStyle, TransformDirection};

fn exponent_with_linear_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configured_exponent_with_linear_transform() -> ExponentWithLinearTransform {
    let transform =
        ExponentWithLinearTransform::create().expect("exponent-with-linear transform create");
    transform.set_gamma(&[2.0, 2.0, 2.0, 1.0]);
    transform.set_offset(&[0.1, 0.1, 0.1, 0.0]);
    transform.set_negative_style(NegativeStyle::Linear);
    transform
}

#[test]
fn exponent_with_linear_transform_value_copy_and_direction_behavior() {
    let _guard = exponent_with_linear_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = configured_exponent_with_linear_transform();

    assert_vec_close(&transform.gamma(), &[2.0, 2.0, 2.0, 1.0], 1e-10);
    assert_vec_close(&transform.offset(), &[0.1, 0.1, 0.1, 0.0], 1e-10);
    assert_eq!(transform.negative_style(), NegativeStyle::Linear);
    assert_eq!(transform.direction(), TransformDirection::Forward);

    let copy = transform
        .create_editable_copy()
        .expect("exponent-with-linear transform editable copy");
    copy.set_gamma(&[2.2, 2.2, 2.2, 1.0]);
    copy.set_offset(&[0.055, 0.055, 0.055, 0.0]);
    copy.set_negative_style(NegativeStyle::Mirror);
    copy.set_direction(TransformDirection::Inverse);

    assert_vec_close(&copy.gamma(), &[2.2, 2.2, 2.2, 1.0], 1e-10);
    assert_vec_close(&copy.offset(), &[0.055, 0.055, 0.055, 0.0], 1e-10);
    assert_eq!(copy.negative_style(), NegativeStyle::Mirror);
    assert_eq!(copy.direction(), TransformDirection::Inverse);

    assert_vec_close(&transform.gamma(), &[2.0, 2.0, 2.0, 1.0], 1e-10);
    assert_vec_close(&transform.offset(), &[0.1, 0.1, 0.1, 0.0], 1e-10);
    assert_eq!(transform.negative_style(), NegativeStyle::Linear);
    assert_eq!(transform.direction(), TransformDirection::Forward);
}

#[test]
fn exponent_with_linear_transform_positive_domain_processor_behavior() {
    let _guard = exponent_with_linear_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = configured_exponent_with_linear_transform();

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

    // With gamma=2 and offset=0.1, the breakpoint is 0.1. These inputs all
    // sit above it, so the moncurve reduces to pow((x+o)/(1+o), gamma).
    let original = [0.2f32, 0.5, 0.8, 1.0];
    let mut processed = original;
    forward_cpu.apply_rgba(&mut processed);

    assert_close(processed[0] as f64, 0.07438017, 2e-5);
    assert_close(processed[1] as f64, 0.29752067, 2e-5);
    assert_close(processed[2] as f64, 0.66942149, 2e-5);
    assert_close(processed[3] as f64, 1.0, 2e-5);

    let mut restored = processed;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 5e-5);
    assert_close(restored[1] as f64, original[1] as f64, 5e-5);
    assert_close(restored[2] as f64, original[2] as f64, 5e-5);
    assert_close(restored[3] as f64, original[3] as f64, 5e-5);
}
