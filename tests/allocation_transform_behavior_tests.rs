//! AllocationTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/allocation.rs`. In bundled/real mode they validate
//! allocation/vars state, editable-copy independence, and real processor
//! execution for both uniform and logarithmic allocation modes.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::AllocationTransform;
use ocio_rs::{Allocation, TransformDirection};

fn allocation_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn allocation_transform_value_copy_and_direction_behavior() {
    let _guard = allocation_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = AllocationTransform::create().expect("allocation transform create");
    transform
        .try_set_allocation(Allocation::Lg2)
        .expect("set allocation");
    transform
        .set_vars(&[-1.0, 1.0, 0.0])
        .expect("set allocation variables");
    transform
        .try_set_direction(TransformDirection::Forward)
        .expect("set direction");

    assert_eq!(transform.allocation(), Allocation::Lg2);
    assert_eq!(transform.num_vars(), 3);
    assert_f32_vec_close(&transform.vars(), &[-1.0, 1.0, 0.0], 1e-6);
    assert_eq!(transform.direction(), TransformDirection::Forward);

    let copy = transform
        .create_editable_copy()
        .expect("allocation transform editable copy");
    copy.try_set_allocation(Allocation::Uniform)
        .expect("set copy allocation");
    copy.set_vars(&[-2.0, 2.0])
        .expect("set allocation copy variables");
    copy.try_set_direction(TransformDirection::Inverse)
        .expect("set copy direction");

    assert_eq!(copy.allocation(), Allocation::Uniform);
    assert_eq!(copy.num_vars(), 2);
    assert_f32_vec_close(&copy.vars(), &[-2.0, 2.0], 1e-6);
    assert_eq!(copy.direction(), TransformDirection::Inverse);

    assert_eq!(transform.allocation(), Allocation::Lg2);
    assert_eq!(transform.num_vars(), 3);
    assert_f32_vec_close(&transform.vars(), &[-1.0, 1.0, 0.0], 1e-6);
    assert_eq!(transform.direction(), TransformDirection::Forward);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn allocation_transform_uniform_processor_behavior() {
    let _guard = allocation_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = AllocationTransform::create().expect("allocation transform create");
    transform
        .try_set_allocation(Allocation::Uniform)
        .expect("set allocation");
    transform
        .set_vars(&[-1.0, 1.0])
        .expect("set allocation variables");

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

    // Uniform allocation with vars [-1, 1] fits the RGB range [-1, 1] to [0, 1].
    let original = [-1.0f32, 0.0, 1.0, 0.25];
    let mut allocated = original;
    forward_cpu.apply_rgba(&mut allocated);

    assert_close(allocated[0] as f64, 0.0, 1e-6);
    assert_close(allocated[1] as f64, 0.5, 1e-6);
    assert_close(allocated[2] as f64, 1.0, 1e-6);
    assert_close(allocated[3] as f64, original[3] as f64, 1e-6);

    let mut restored = allocated;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 1e-5);
    assert_close(restored[1] as f64, original[1] as f64, 1e-5);
    assert_close(restored[2] as f64, original[2] as f64, 1e-5);
    assert_close(restored[3] as f64, original[3] as f64, 1e-5);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn allocation_transform_lg2_processor_behavior() {
    let _guard = allocation_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = AllocationTransform::create().expect("allocation transform create");
    transform
        .try_set_allocation(Allocation::Lg2)
        .expect("set allocation");
    transform
        .set_vars(&[-1.0, 1.0, 0.0])
        .expect("set allocation variables");

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

    // Lg2 allocation with vars [-1, 1, 0] applies log2 to RGB then fits
    // the range [-1, 1] into [0, 1].
    let original = [0.5f32, 1.0, 2.0, 0.75];
    let mut allocated = original;
    forward_cpu.apply_rgba(&mut allocated);

    assert_close(allocated[0] as f64, 0.0, 2e-5);
    assert_close(allocated[1] as f64, 0.5, 2e-5);
    assert_close(allocated[2] as f64, 1.0, 2e-5);
    assert_close(allocated[3] as f64, original[3] as f64, 2e-5);

    let mut restored = allocated;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 5e-5);
    assert_close(restored[1] as f64, original[1] as f64, 5e-5);
    assert_close(restored[2] as f64, original[2] as f64, 5e-5);
    assert_close(restored[3] as f64, original[3] as f64, 5e-5);
}
