//! LookTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/look_transform.rs`. In bundled/real mode they validate
//! field round trips, editable-copy independence, processor execution through a
//! configured look pipeline, and the real effect of
//! `skip_color_space_conversion`.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::{LookTransform, MatrixTransform};
use ocio_rs::{Allocation, ColorSpace, Config, Look, TransformDirection};

fn look_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn swapped_color_space(name: &str) -> ColorSpace {
    let cs = ColorSpace::create().expect("color space create");
    cs.set_name(name).expect("set color space name");
    cs.set_family("Unit/LookTransforms")
        .expect("set color space family");
    cs.set_description("look transform test source color space")
        .expect("set color space description");
    cs.set_is_data(false);
    cs.set_allocation(Allocation::Lg2);
    cs.set_allocation_vars(&[-8.0, 8.0])
        .expect("set allocation variables");

    let swap = MatrixTransform::create().expect("swap matrix create");
    swap.set_matrix(&[
        0.0, 0.0, 1.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        1.0, 0.0, 0.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    ])
    .expect("set channel-swap matrix");
    swap.set_offset(&[0.0, 0.0, 0.0, 0.0])
        .expect("set channel-swap offset");
    cs.try_set_transform(&swap, ocio_rs::ColorSpaceDirection::ToReference)
        .expect("attach to-reference transform");
    cs.try_set_transform(&swap, ocio_rs::ColorSpaceDirection::FromReference)
        .expect("attach from-reference transform");
    cs
}

fn identity_color_space(name: &str) -> ColorSpace {
    let cs = ColorSpace::create().expect("color space create");
    cs.set_name(name).expect("set color space name");
    cs.set_family("Unit/LookTransforms")
        .expect("set color space family");
    cs.set_description("look transform test process color space")
        .expect("set color space description");
    cs.set_is_data(false);
    cs.set_allocation(Allocation::Lg2);
    cs.set_allocation_vars(&[-8.0, 8.0])
        .expect("set allocation variables");
    cs
}

fn red_scale_look(name: &str, process_space: &str) -> Look {
    let look = Look::create().expect("look create");
    look.set_name(name).expect("set look name");
    look.set_process_space(process_space)
        .expect("set process space");
    look.set_description("look transform behavior test")
        .expect("set description");

    let forward = MatrixTransform::scale(&[1.5, 1.0, 1.0, 1.0]).expect("forward look matrix");
    let inverse = MatrixTransform::scale(&[2.0 / 3.0, 1.0, 1.0, 1.0]).expect("inverse look matrix");
    look.try_set_transform(&forward)
        .expect("attach forward look transform");
    look.try_set_inverse_transform(&inverse)
        .expect("attach inverse look transform");
    look
}

fn configured_look_transform_config() -> Config {
    let config = create_test_config().expect("raw config");
    let source = swapped_color_space("UnitLookSource");
    let process = identity_color_space("UnitLookProcess");
    let look = red_scale_look("UnitLook", "UnitLookProcess");

    config.add_color_space(&source);
    config.add_color_space(&process);
    config.add_look(&look);
    config
}

#[test]
fn look_transform_field_copy_and_validate_behavior() {
    let _guard = look_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = LookTransform::create().expect("look transform create");
    transform.set_src("UnitLookSource").expect("set src");
    transform.set_dst("UnitLookSource").expect("set dst");
    transform.set_looks("UnitLook").expect("set looks");
    transform.set_direction(TransformDirection::Inverse);
    transform.set_skip_color_space_conversion(true);
    transform.validate().expect("validate look transform");

    assert_eq!(transform.src().as_deref(), Some("UnitLookSource"));
    assert_eq!(transform.dst().as_deref(), Some("UnitLookSource"));
    assert_eq!(transform.looks().as_deref(), Some("UnitLook"));
    assert_eq!(transform.direction(), TransformDirection::Inverse);
    assert!(transform.skip_color_space_conversion());

    let copy = transform
        .create_editable_copy()
        .expect("look transform editable copy");
    copy.set_dst("UnitLookProcess").expect("set copy dst");
    copy.set_direction(TransformDirection::Forward);
    copy.set_skip_color_space_conversion(false);

    assert_eq!(copy.dst().as_deref(), Some("UnitLookProcess"));
    assert_eq!(copy.direction(), TransformDirection::Forward);
    assert!(!copy.skip_color_space_conversion());

    assert_eq!(transform.dst().as_deref(), Some("UnitLookSource"));
    assert_eq!(transform.direction(), TransformDirection::Inverse);
    assert!(transform.skip_color_space_conversion());
}

#[test]
fn look_transform_skip_color_space_conversion_behavior() {
    let _guard = look_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = configured_look_transform_config();

    let with_conversion = LookTransform::create().expect("look transform with conversion");
    with_conversion
        .set_src("UnitLookSource")
        .expect("set src with conversion");
    with_conversion
        .set_dst("UnitLookSource")
        .expect("set dst with conversion");
    with_conversion
        .set_looks("UnitLook")
        .expect("set looks with conversion");
    with_conversion.set_skip_color_space_conversion(false);

    let without_conversion = LookTransform::create().expect("look transform without conversion");
    without_conversion
        .set_src("UnitLookSource")
        .expect("set src without conversion");
    without_conversion
        .set_dst("UnitLookSource")
        .expect("set dst without conversion");
    without_conversion
        .set_looks("UnitLook")
        .expect("set looks without conversion");
    without_conversion.set_skip_color_space_conversion(true);

    let with_cpu = config
        .processor_from_transform(&with_conversion, TransformDirection::Forward)
        .expect("processor with conversion")
        .default_cpu_processor()
        .expect("cpu with conversion");
    let without_cpu = config
        .processor_from_transform(&without_conversion, TransformDirection::Forward)
        .expect("processor without conversion")
        .default_cpu_processor()
        .expect("cpu without conversion");

    let original = [0.4f32, 0.5, 0.6, 1.0];
    let mut with_pixel = original;
    let mut without_pixel = original;
    with_cpu.apply_rgba(&mut with_pixel);
    without_cpu.apply_rgba(&mut without_pixel);

    // With color-space conversion, the source color space swaps R/B before the
    // look scales the process-space red channel, so the original blue channel
    // is the one that gets boosted once we convert back.
    assert_close(with_pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(with_pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(with_pixel[2] as f64, 0.9, 1e-6);
    assert_close(with_pixel[3] as f64, 1.0, 1e-6);

    // Skipping color-space conversion applies the look directly in source
    // space, so the original red channel is the one that changes.
    assert_close(without_pixel[0] as f64, 0.6, 1e-6);
    assert_close(without_pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(without_pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(without_pixel[3] as f64, 1.0, 1e-6);
}
