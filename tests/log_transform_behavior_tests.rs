//! LogTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/log.rs`. In bundled/real mode they validate base/direction
//! state, editable-copy independence, and real processor execution for the
//! positive log-domain path.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::LogTransform;
use ocio_rs::TransformDirection;

fn log_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configured_log_transform() -> LogTransform {
    let transform = LogTransform::create().expect("log transform create");
    transform.try_set_base(2.0).expect("set logarithm base");
    transform
}

#[test]
fn log_transform_base_copy_and_direction_behavior() {
    let _guard = log_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = configured_log_transform();
    assert_close(transform.base(), 2.0, 1e-10);
    assert_eq!(transform.direction(), TransformDirection::Forward);

    let copy = transform
        .create_editable_copy()
        .expect("log transform editable copy");
    copy.try_set_base(10.0).expect("set copy logarithm base");
    copy.set_direction(TransformDirection::Inverse);

    assert_close(copy.base(), 10.0, 1e-10);
    assert_eq!(copy.direction(), TransformDirection::Inverse);

    assert_close(transform.base(), 2.0, 1e-10);
    assert_eq!(transform.direction(), TransformDirection::Forward);
}

#[test]
fn log_transform_positive_domain_processor_behavior() {
    let _guard = log_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = configured_log_transform();

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

    let original = [0.25f32, 0.5, 1.0, 0.75];
    let mut logged = original;
    forward_cpu.apply_rgba(&mut logged);

    assert_close(logged[0] as f64, -2.0, 2e-5);
    assert_close(logged[1] as f64, -1.0, 2e-5);
    assert_close(logged[2] as f64, 0.0, 2e-5);
    assert_close(logged[3] as f64, original[3] as f64, 2e-5);

    let mut restored = logged;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 5e-5);
    assert_close(restored[1] as f64, original[1] as f64, 5e-5);
    assert_close(restored[2] as f64, original[2] as f64, 5e-5);
    assert_close(restored[3] as f64, original[3] as f64, 5e-5);
}
