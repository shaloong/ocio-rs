//! FixedFunctionTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/fixed_function.rs`. In bundled/real mode they validate style
//! and parameter round trips, editable-copy independence, and real processor
//! execution for a stable fixed-function style.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::FixedFunctionTransform;
use ocio_rs::{FixedFunctionStyle, TransformDirection};

fn fixed_function_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn fixed_function_style_params_and_copy_behavior() {
    let _guard = fixed_function_test_lock();
    if is_stub() {
        return;
    }

    let transform =
        FixedFunctionTransform::create_with_params(FixedFunctionStyle::Rec2100Surround, &[1.25])
            .expect("fixed function create with params");

    assert_eq!(transform.style(), FixedFunctionStyle::Rec2100Surround);
    assert_eq!(transform.num_params(), 1);
    assert_vec_close(&transform.params(), &[1.25], 1e-10);

    let copy = transform
        .create_editable_copy()
        .expect("fixed function editable copy");
    copy.set_params(&[0.75]).expect("fixed function parameters");
    copy.set_direction(TransformDirection::Inverse);

    assert_eq!(copy.direction(), TransformDirection::Inverse);

    copy.set_style(FixedFunctionStyle::RgbToHsv);

    assert_eq!(copy.style(), FixedFunctionStyle::RgbToHsv);
    assert_eq!(copy.num_params(), copy.params().len() as i32);

    assert_eq!(transform.style(), FixedFunctionStyle::Rec2100Surround);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert_vec_close(&transform.params(), &[1.25], 1e-10);
}

#[test]
fn fixed_function_rgb_to_hsv_processor_behavior() {
    let _guard = fixed_function_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = FixedFunctionTransform::create(FixedFunctionStyle::RgbToHsv)
        .expect("fixed function create");

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

    let original = [0.2f32, 0.5, 0.3, 1.0];
    let mut hsv = original;
    forward_cpu.apply_rgba(&mut hsv);

    // RGB(0.2, 0.5, 0.3) -> HSV(h=7/18, s=0.6, v=0.5)
    assert_close(hsv[0] as f64, 7.0 / 18.0, 1e-6);
    assert_close(hsv[1] as f64, 0.6, 1e-6);
    assert_close(hsv[2] as f64, 0.5, 1e-6);
    assert_close(hsv[3] as f64, 1.0, 1e-6);

    let mut restored = hsv;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 1e-6);
    assert_close(restored[1] as f64, original[1] as f64, 1e-6);
    assert_close(restored[2] as f64, original[2] as f64, 1e-6);
    assert_close(restored[3] as f64, original[3] as f64, 1e-6);
}

#[test]
fn fixed_function_invalid_params_surface_real_error_behavior() {
    let _guard = fixed_function_test_lock();
    if is_stub() {
        return;
    }

    let err = match FixedFunctionTransform::create_with_params(FixedFunctionStyle::RgbToHsv, &[1.0])
    {
        Ok(_) => panic!("RgbToHsv should reject unexpected parameters"),
        Err(err) => err,
    };
    assert!(
        matches!(err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );
}

#[test]
fn fixed_function_set_params_uses_ocio_delayed_validation() {
    let _guard = fixed_function_test_lock();
    if is_stub() {
        return;
    }

    let transform = FixedFunctionTransform::create(FixedFunctionStyle::RgbToHsv)
        .expect("fixed function create");
    transform
        .set_params(&[1.0])
        .expect("OCIO accepts setter parameters before transform validation");

    assert_eq!(transform.style(), FixedFunctionStyle::RgbToHsv);
    assert_vec_close(&transform.params(), &[1.0], 1e-10);

    let config = create_test_config().expect("raw config");
    let err = match config.processor_from_transform(&transform, TransformDirection::Forward) {
        Ok(_) => panic!("an invalid fixed-function parameter list must fail validation"),
        Err(err) => err,
    };
    assert!(matches!(err, ocio_rs::OcioError::Ocio(_)));
}
