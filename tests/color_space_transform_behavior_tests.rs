//! ColorSpaceTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/color_space.rs`. In bundled/real mode they validate field
//! round trips, editable-copy independence, processor execution equivalence
//! with named color-space processors, and real `data_bypass` behavior.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::{ColorSpaceTransform, MatrixTransform};
use ocio_rs::{Allocation, ColorSpace, ColorSpaceDirection, Config, TransformDirection};

fn color_space_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn scaled_color_space(
    name: &str,
    equality_group: &str,
    is_data: bool,
    scale: [f64; 4],
    inverse_scale: [f64; 4],
) -> ColorSpace {
    let cs = ColorSpace::create().expect("color space create");
    cs.set_name(name).expect("set color space name");
    cs.set_family("Unit/ColorSpaceTransforms")
        .expect("set color space family");
    cs.set_description("color space transform behavior test")
        .expect("set color space description");
    cs.set_equality_group(equality_group)
        .expect("set equality group");
    cs.set_encoding("scene-linear").expect("set encoding");
    cs.set_is_data(is_data);
    cs.set_allocation(Allocation::Lg2);
    cs.set_allocation_vars(&[-8.0, 8.0]);

    let to_reference = MatrixTransform::scale(&scale).expect("to-reference matrix");
    let from_reference = MatrixTransform::scale(&inverse_scale).expect("from-reference matrix");
    cs.set_transform(&to_reference, ColorSpaceDirection::ToReference);
    cs.set_transform(&from_reference, ColorSpaceDirection::FromReference);
    cs
}

fn configured_color_space_transform_config(source_is_data: bool) -> Config {
    let config = Config::raw().expect("raw config");
    let src = scaled_color_space(
        "UnitColorSpaceTransformSrc",
        "unit-cst-src",
        source_is_data,
        [2.0, 1.0, 0.5, 1.0],
        [0.5, 1.0, 2.0, 1.0],
    );
    let dst = scaled_color_space(
        "UnitColorSpaceTransformDst",
        "unit-cst-dst",
        false,
        [1.0, 1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    );
    config.add_color_space(&src);
    config.add_color_space(&dst);
    config
}

#[test]
fn color_space_transform_field_copy_and_validate_behavior() {
    let _guard = color_space_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = ColorSpaceTransform::create().expect("color space transform create");
    transform
        .set_src("UnitColorSpaceTransformSrc")
        .expect("set src");
    transform
        .set_dst("UnitColorSpaceTransformDst")
        .expect("set dst");
    transform.set_data_bypass(true);
    transform.set_direction(TransformDirection::Inverse);
    transform.validate();

    assert_eq!(
        transform.src().as_deref(),
        Some("UnitColorSpaceTransformSrc")
    );
    assert_eq!(
        transform.dst().as_deref(),
        Some("UnitColorSpaceTransformDst")
    );
    assert!(transform.data_bypass());
    assert_eq!(transform.direction(), TransformDirection::Inverse);

    let copy = transform
        .create_editable_copy()
        .expect("color space transform editable copy");
    copy.set_dst("UnitColorSpaceTransformSrc")
        .expect("set copy dst");
    copy.set_data_bypass(false);
    copy.set_direction(TransformDirection::Forward);

    assert_eq!(copy.dst().as_deref(), Some("UnitColorSpaceTransformSrc"));
    assert!(!copy.data_bypass());
    assert_eq!(copy.direction(), TransformDirection::Forward);

    assert_eq!(
        transform.dst().as_deref(),
        Some("UnitColorSpaceTransformDst")
    );
    assert!(transform.data_bypass());
    assert_eq!(transform.direction(), TransformDirection::Inverse);
}

#[test]
fn color_space_transform_matches_named_processor_behavior() {
    let _guard = color_space_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = configured_color_space_transform_config(false);
    let transform = ColorSpaceTransform::create().expect("color space transform create");
    transform
        .set_src("UnitColorSpaceTransformSrc")
        .expect("set src");
    transform
        .set_dst("UnitColorSpaceTransformDst")
        .expect("set dst");

    let transform_cpu = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .expect("processor from transform")
        .default_cpu_processor()
        .expect("transform cpu");
    let named_cpu = config
        .processor("UnitColorSpaceTransformSrc", "UnitColorSpaceTransformDst")
        .expect("named processor")
        .default_cpu_processor()
        .expect("named cpu");

    let mut transform_pixel = [0.25f32, 0.5, 0.5, 1.0];
    let mut named_pixel = transform_pixel;
    transform_cpu.apply_rgba(&mut transform_pixel);
    named_cpu.apply_rgba(&mut named_pixel);

    assert_close(transform_pixel[0] as f64, 0.5, 1e-6);
    assert_close(transform_pixel[1] as f64, 0.5, 1e-6);
    assert_close(transform_pixel[2] as f64, 0.25, 1e-6);
    assert_close(transform_pixel[3] as f64, 1.0, 1e-6);

    assert_close(named_pixel[0] as f64, transform_pixel[0] as f64, 1e-6);
    assert_close(named_pixel[1] as f64, transform_pixel[1] as f64, 1e-6);
    assert_close(named_pixel[2] as f64, transform_pixel[2] as f64, 1e-6);
    assert_close(named_pixel[3] as f64, transform_pixel[3] as f64, 1e-6);
}

#[test]
fn color_space_transform_data_bypass_behavior() {
    let _guard = color_space_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = configured_color_space_transform_config(true);

    let bypassed = ColorSpaceTransform::create().expect("bypassed color space transform");
    bypassed
        .set_src("UnitColorSpaceTransformSrc")
        .expect("set bypassed src");
    bypassed
        .set_dst("UnitColorSpaceTransformDst")
        .expect("set bypassed dst");
    bypassed.set_data_bypass(true);

    let forced = ColorSpaceTransform::create().expect("forced color space transform");
    forced
        .set_src("UnitColorSpaceTransformSrc")
        .expect("set forced src");
    forced
        .set_dst("UnitColorSpaceTransformDst")
        .expect("set forced dst");
    forced.set_data_bypass(false);

    let bypassed_cpu = config
        .processor_from_transform(&bypassed, TransformDirection::Forward)
        .expect("bypassed processor")
        .default_cpu_processor()
        .expect("bypassed cpu");
    let forced_cpu = config
        .processor_from_transform(&forced, TransformDirection::Forward)
        .expect("forced processor")
        .default_cpu_processor()
        .expect("forced cpu");

    let original = [0.25f32, 0.5, 0.5, 1.0];
    let mut bypassed_pixel = original;
    let mut forced_pixel = original;
    bypassed_cpu.apply_rgba(&mut bypassed_pixel);
    forced_cpu.apply_rgba(&mut forced_pixel);

    assert_close(bypassed_pixel[0] as f64, original[0] as f64, 1e-6);
    assert_close(bypassed_pixel[1] as f64, original[1] as f64, 1e-6);
    assert_close(bypassed_pixel[2] as f64, original[2] as f64, 1e-6);
    assert_close(bypassed_pixel[3] as f64, original[3] as f64, 1e-6);

    assert_close(forced_pixel[0] as f64, 0.5, 1e-6);
    assert_close(forced_pixel[1] as f64, 0.5, 1e-6);
    assert_close(forced_pixel[2] as f64, 0.25, 1e-6);
    assert_close(forced_pixel[3] as f64, 1.0, 1e-6);
}
