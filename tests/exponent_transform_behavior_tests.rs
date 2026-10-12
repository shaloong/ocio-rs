//! ExponentTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/exponent.rs`. In bundled/real mode they validate exponent
//! value round trips, negative-style and direction state, editable-copy
//! independence, and real processor execution for positive-domain inputs.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::ExponentTransform;
use ocio_rs::{NegativeStyle, TransformDirection};

fn exponent_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configured_exponent_transform() -> ExponentTransform {
    let transform = ExponentTransform::create().expect("exponent transform create");
    transform
        .set_value(&[2.0, 2.0, 2.0, 1.0])
        .expect("set exponent transform value");
    transform
        .set_negative_style(NegativeStyle::PassThru)
        .expect("set exponent negative style");
    transform
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn exponent_transform_value_copy_and_direction_behavior() {
    let _guard = exponent_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = configured_exponent_transform();

    assert_vec_close(
        &transform.value().expect("exponent transform value"),
        &[2.0, 2.0, 2.0, 1.0],
        1e-10,
    );
    assert_eq!(transform.negative_style(), NegativeStyle::PassThru);
    assert_eq!(transform.direction(), TransformDirection::Forward);

    let copy = transform
        .create_editable_copy()
        .expect("exponent transform editable copy");
    copy.set_value(&[1.0, 1.0, 1.0, 1.0])
        .expect("set exponent copy value");
    copy.set_negative_style(NegativeStyle::Mirror)
        .expect("set exponent copy negative style");
    copy.set_direction(TransformDirection::Inverse);

    assert_vec_close(
        &copy.value().expect("exponent copy value"),
        &[1.0, 1.0, 1.0, 1.0],
        1e-10,
    );
    assert_eq!(copy.negative_style(), NegativeStyle::Mirror);
    assert_eq!(copy.direction(), TransformDirection::Inverse);

    assert_vec_close(
        &transform
            .value()
            .expect("exponent transform value after copy"),
        &[2.0, 2.0, 2.0, 1.0],
        1e-10,
    );
    assert_eq!(transform.negative_style(), NegativeStyle::PassThru);
    assert_eq!(transform.direction(), TransformDirection::Forward);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn exponent_transform_invalid_negative_style_surfaces_error() {
    let _guard = exponent_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = ExponentTransform::create().expect("exponent transform create");
    transform
        .set_value(&[2.0, 2.0, 2.0, 1.0])
        .expect("seed exponent value");

    let err = transform
        .set_negative_style(NegativeStyle::Linear)
        .expect_err("linear negative style should be rejected for basic exponent");
    assert!(
        matches!(err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn exponent_transform_positive_domain_processor_behavior() {
    let _guard = exponent_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = configured_exponent_transform();

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

    let original = [0.25f32, 0.5, 0.75, 1.0];
    let mut exponentiated = original;
    forward_cpu.apply_rgba(&mut exponentiated);

    assert_close(exponentiated[0] as f64, 0.0625, 1e-5);
    assert_close(exponentiated[1] as f64, 0.25, 1e-5);
    assert_close(exponentiated[2] as f64, 0.5625, 1e-5);
    assert_close(exponentiated[3] as f64, 1.0, 2e-5);

    let mut restored = exponentiated;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 5e-5);
    assert_close(restored[1] as f64, original[1] as f64, 5e-5);
    assert_close(restored[2] as f64, original[2] as f64, 5e-5);
    assert_close(restored[3] as f64, original[3] as f64, 5e-5);
}
