//! RangeTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/range.rs`. In bundled/real mode they validate parameter
//! round trips, unset/has behavior, editable-copy independence, and real
//! processor execution for both clamp and inverse cases.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::RangeTransform;
use ocio_rs::{BitDepth, RangeStyle, TransformDirection};

fn range_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn configured_range_transform(style: RangeStyle) -> RangeTransform {
    let transform = RangeTransform::create().expect("range transform create");
    transform.try_set_style(style).expect("set style");
    transform.try_set_min_in_value(0.0).expect("set min in");
    transform.try_set_max_in_value(1.0).expect("set max in");
    transform.try_set_min_out_value(0.25).expect("set min out");
    transform.try_set_max_out_value(0.75).expect("set max out");
    transform
        .try_set_file_input_bit_depth(BitDepth::F32)
        .expect("set file input bit depth");
    transform
        .try_set_file_output_bit_depth(BitDepth::F32)
        .expect("set file output bit depth");
    transform
}

#[test]
fn range_transform_parameter_unset_and_copy_behavior() {
    let _guard = range_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = configured_range_transform(RangeStyle::Clamp);

    assert_eq!(transform.style(), RangeStyle::Clamp);
    assert_close(transform.min_in_value(), 0.0, 1e-10);
    assert_close(transform.max_in_value(), 1.0, 1e-10);
    assert_close(transform.min_out_value(), 0.25, 1e-10);
    assert_close(transform.max_out_value(), 0.75, 1e-10);
    assert_eq!(transform.file_input_bit_depth(), BitDepth::F32);
    assert_eq!(transform.file_output_bit_depth(), BitDepth::F32);
    assert!(transform.has_min_in_value());
    assert!(transform.has_max_in_value());
    assert!(transform.has_min_out_value());
    assert!(transform.has_max_out_value());

    let copy = transform
        .create_editable_copy()
        .expect("range transform editable copy");
    copy.try_unset_min_in_value().expect("unset copy min in");
    copy.try_unset_max_out_value().expect("unset copy max out");
    copy.set_direction(TransformDirection::Inverse);

    assert!(!copy.has_min_in_value());
    assert!(!copy.has_max_out_value());
    assert_eq!(copy.direction(), TransformDirection::Inverse);

    assert!(transform.has_min_in_value());
    assert!(transform.has_max_out_value());
    assert_eq!(transform.direction(), TransformDirection::Forward);
}

#[test]
fn range_transform_clamp_processor_behavior() {
    let _guard = range_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = configured_range_transform(RangeStyle::Clamp);

    let cpu = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .expect("range processor")
        .default_cpu_processor()
        .expect("range cpu");

    let mut pixel = [-0.5f32, 0.5, 1.5, 1.0];
    cpu.apply_rgba(&mut pixel);

    assert_close(pixel[0] as f64, 0.25, 1e-6);
    assert_close(pixel[1] as f64, 0.5, 1e-6);
    assert_close(pixel[2] as f64, 0.75, 1e-6);
    assert_close(pixel[3] as f64, 1.0, 1e-6);
}

#[test]
fn range_transform_inverse_processor_behavior() {
    let _guard = range_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = configured_range_transform(RangeStyle::NoClamp);

    let forward_cpu = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .expect("forward range processor")
        .default_cpu_processor()
        .expect("forward range cpu");
    let inverse_cpu = config
        .processor_from_transform(&transform, TransformDirection::Inverse)
        .expect("inverse range processor")
        .default_cpu_processor()
        .expect("inverse range cpu");

    let original = [0.25f32, 0.5, 0.75, 1.0];
    let mut mapped = original;
    forward_cpu.apply_rgba(&mut mapped);

    assert_close(mapped[0] as f64, 0.375, 1e-6);
    assert_close(mapped[1] as f64, 0.5, 1e-6);
    assert_close(mapped[2] as f64, 0.625, 1e-6);
    assert_close(mapped[3] as f64, 1.0, 1e-6);

    let mut restored = mapped;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 1e-6);
    assert_close(restored[1] as f64, original[1] as f64, 1e-6);
    assert_close(restored[2] as f64, original[2] as f64, 1e-6);
    assert_close(restored[3] as f64, original[3] as f64, 1e-6);
}
