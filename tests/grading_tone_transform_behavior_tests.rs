//! GradingToneTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/grading_tone.rs`. In bundled/real mode they validate
//! default/style state, dynamic toggles, editable-copy independence, and
//! upstream style-reset semantics.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::grading::GradingTone;
use ocio_rs::transform::GradingToneTransform;
use ocio_rs::{GradingStyle, TransformDirection};

fn grading_tone_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn grading_tone_default_style_dynamic_and_copy_behavior() {
    let _guard = grading_tone_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = GradingToneTransform::create(GradingStyle::Lin).expect("grading tone create");
    let value = transform.value();

    assert_eq!(transform.style(), GradingStyle::Lin);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert!(!transform.is_dynamic());

    assert_eq!(value, GradingTone::new(GradingStyle::Lin));
    assert_close(value.blacks.width, 4.0, 1e-10);
    assert_close(value.shadows.start, 2.0, 1e-10);
    assert_close(value.shadows.width, -7.0, 1e-10);
    assert_close(value.midtones.width, 8.0, 1e-10);
    assert_close(value.highlights.start, -2.0, 1e-10);
    assert_close(value.highlights.width, 9.0, 1e-10);
    assert_close(value.whites.width, 8.0, 1e-10);
    assert_close(value.scontrast, 1.0, 1e-10);

    transform.make_dynamic();
    assert!(transform.is_dynamic());
    transform.make_non_dynamic();
    assert!(!transform.is_dynamic());

    let copy = transform
        .create_editable_copy()
        .expect("grading tone editable copy");
    copy.set_direction(TransformDirection::Inverse);

    let mut copy_value = copy.value();
    copy_value.midtones.start = 1.25;
    copy_value.whites.master = 1.2;
    copy.set_value(&copy_value);

    assert_eq!(copy.direction(), TransformDirection::Inverse);
    assert_close(copy.value().midtones.start, 1.25, 1e-10);
    assert_close(copy.value().whites.master, 1.2, 1e-10);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert_close(transform.value().midtones.start, 0.0, 1e-10);
    assert_close(transform.value().whites.master, 1.0, 1e-10);

    let mut changed = transform.value();
    changed.blacks.red = 1.3;
    changed.scontrast = 1.4;
    transform.set_value(&changed);
    transform.set_style(GradingStyle::Video);

    let reset = transform.value();
    let expected = GradingTone::new(GradingStyle::Video);
    assert_eq!(transform.style(), GradingStyle::Video);
    assert_eq!(reset, expected);
}
