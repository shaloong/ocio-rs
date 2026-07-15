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

    transform.try_make_dynamic().expect("make dynamic");
    assert!(transform.is_dynamic());
    transform.try_make_non_dynamic().expect("make non dynamic");
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
    transform
        .try_set_style(GradingStyle::Video)
        .expect("set style video");

    let reset = transform.value();
    let expected = GradingTone::new(GradingStyle::Video);
    assert_eq!(transform.style(), GradingStyle::Video);
    assert_eq!(reset, expected);
}

#[test]
fn grading_tone_legacy_value_handle_survives_parent_drop() {
    let _guard = grading_tone_transform_test_lock();
    if is_stub() {
        return;
    }

    unsafe {
        let source =
            ocio_sys::ocio_grading_tone_transform_create_with_style(GradingStyle::Lin as i32);
        assert!(!source.is_null(), "source grading tone transform");

        let mut seeded_flat = [0.0f64; 31];
        seeded_flat[0] = 1.0;
        seeded_flat[1] = 1.0;
        seeded_flat[2] = 1.0;
        seeded_flat[3] = 1.0;
        seeded_flat[5] = 4.0;
        seeded_flat[6] = 1.0;
        seeded_flat[7] = 1.0;
        seeded_flat[8] = 1.0;
        seeded_flat[9] = 1.0;
        seeded_flat[10] = 2.0;
        seeded_flat[11] = -7.0;
        seeded_flat[12] = 1.0;
        seeded_flat[13] = 1.0;
        seeded_flat[14] = 1.0;
        seeded_flat[15] = 1.0;
        seeded_flat[16] = 1.5;
        seeded_flat[17] = 8.0;
        seeded_flat[18] = 1.0;
        seeded_flat[19] = 1.0;
        seeded_flat[20] = 1.0;
        seeded_flat[21] = 1.0;
        seeded_flat[22] = -2.0;
        seeded_flat[23] = 9.0;
        seeded_flat[24] = 1.0;
        seeded_flat[25] = 1.0;
        seeded_flat[26] = 1.0;
        seeded_flat[27] = 1.0;
        seeded_flat[29] = 8.0;
        seeded_flat[30] = 1.0;
        assert!(ocio_sys::ocio_grading_tone_transform_set_value_from_f64(
            source,
            seeded_flat.as_ptr(),
            seeded_flat.len()
        ));

        let value_handle = ocio_sys::ocio_grading_tone_transform_get_value(source);
        assert!(!value_handle.is_null(), "grading tone value handle");
        ocio_sys::ocio_grading_tone_transform_destroy(source);

        let target =
            ocio_sys::ocio_grading_tone_transform_create_with_style(GradingStyle::Lin as i32);
        assert!(!target.is_null(), "target grading tone transform");
        ocio_sys::ocio_grading_tone_transform_set_value(target, value_handle);

        let mut flat = [0.0f64; 31];
        assert!(ocio_sys::ocio_grading_tone_transform_copy_value(
            target,
            flat.as_mut_ptr(),
            flat.len()
        ));
        assert_close(flat[16], 1.5, 1e-10);

        ocio_sys::ocio_grading_tone_value_destroy(value_handle);
        ocio_sys::ocio_grading_tone_transform_destroy(target);
    }
}

#[test]
fn grading_tone_try_setters_surface_errors() {
    let _guard = grading_tone_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = GradingToneTransform::create(GradingStyle::Log).expect("grading tone create");

    // try_set_style with valid values
    transform
        .try_set_style(GradingStyle::Lin)
        .expect("try_set_style Lin");
    assert_eq!(transform.style(), GradingStyle::Lin);

    transform
        .try_set_style(GradingStyle::Video)
        .expect("try_set_style Video");
    assert_eq!(transform.style(), GradingStyle::Video);

    transform
        .try_set_style(GradingStyle::Log)
        .expect("try_set_style Log");
    assert_eq!(transform.style(), GradingStyle::Log);

    // try_make_dynamic / try_make_non_dynamic with readback
    transform.try_make_dynamic().expect("try_make_dynamic");
    assert!(transform.is_dynamic());

    transform
        .try_make_non_dynamic()
        .expect("try_make_non_dynamic");
    assert!(!transform.is_dynamic());

    // try_set_value with readback
    let mut custom = GradingTone::new(GradingStyle::Log);
    custom.blacks.red = 1.5;
    custom.scontrast = 1.2;
    transform.try_set_value(&custom).expect("try_set_value");
    let read_back = transform.value();
    assert_close(read_back.blacks.red, 1.5, 1e-10);
    assert_close(read_back.scontrast, 1.2, 1e-10);
}
