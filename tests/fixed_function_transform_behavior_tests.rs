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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn fixed_function_style_params_and_copy_behavior() {
    let _guard = fixed_function_test_lock();
    if is_stub() {
        return;
    }

    let transform =
        FixedFunctionTransform::create_with_params(FixedFunctionStyle::Rec2100Surround, &[1.25])
            .expect("fixed function create with params");

    assert_eq!(transform.style(), FixedFunctionStyle::Rec2100Surround);
    assert_eq!(
        transform.try_style().unwrap(),
        FixedFunctionStyle::Rec2100Surround
    );
    assert_eq!(transform.num_params(), 1);
    assert_eq!(transform.try_num_params_usize().unwrap(), 1);
    let mut params = [0.0];
    transform.try_params_into(&mut params).unwrap();
    assert_vec_close(&params, &[1.25], 1e-10);
    let mut wrong = [42.0; 2];
    assert!(transform.try_params_into(&mut wrong).is_err());
    assert_eq!(wrong, [42.0; 2]);
    assert_vec_close(&transform.try_params().unwrap(), &[1.25], 1e-10);
    assert_vec_close(&transform.params(), &[1.25], 1e-10);

    let copy = transform
        .create_editable_copy()
        .expect("fixed function editable copy");
    copy.set_params(&[0.75]).expect("fixed function parameters");
    copy.set_direction(TransformDirection::Inverse);

    assert_eq!(copy.direction(), TransformDirection::Inverse);

    copy.try_set_style(FixedFunctionStyle::RgbToHsv)
        .expect("set copy style");

    assert_eq!(copy.style(), FixedFunctionStyle::RgbToHsv);
    let no_params = FixedFunctionTransform::create(FixedFunctionStyle::RgbToHsv).unwrap();
    no_params.try_params_into(&mut []).unwrap();
    assert_eq!(copy.num_params(), copy.params().len() as i32);

    assert_eq!(transform.style(), FixedFunctionStyle::Rec2100Surround);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert_vec_close(&transform.params(), &[1.25], 1e-10);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn fixed_function_empty_parameter_buffers_use_native_factory_safely() {
    let _guard = fixed_function_test_lock();
    let style = FixedFunctionStyle::RgbToHsv;
    let via_empty_slice = FixedFunctionTransform::create_with_params(style, &[]).unwrap();
    let via_default = FixedFunctionTransform::create(style).unwrap();
    assert!(via_empty_slice.equals(&via_default));
    assert_eq!(via_default.try_num_params_usize().unwrap(), 0);
    via_default.set_params(&[1.0]).unwrap();
    via_default.set_params(&[]).unwrap();
    assert_eq!(via_default.try_num_params_usize().unwrap(), 0);

    // The low-level API also accepts a null pointer for an empty range.
    unsafe {
        let handle = ocio_sys::ocio_fixed_function_transform_create_with_params(
            style as i32,
            std::ptr::null(),
            0,
        );
        assert!(!handle.is_null());
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_fixed_function_transform_set_params(handle, std::ptr::null(), 0);
        assert!(ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_fixed_function_transform_set_params(handle, std::ptr::null(), 1);
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
        assert_eq!(
            ocio_sys::ocio_fixed_function_transform_get_num_params(handle),
            0
        );
        ocio_sys::ocio_fixed_function_transform_destroy(handle);

        // A non-empty range must never be forwarded with a null pointer.
        let invalid = ocio_sys::ocio_fixed_function_transform_create_with_params(
            style as i32,
            std::ptr::null(),
            1,
        );
        assert!(invalid.is_null());
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn hsy_bridge_discriminants_select_the_native_styles() {
    use ocio_rs::{transform::GroupTransform, Config};
    let config = Config::raw().unwrap();
    for (style, native_name) in [
        (FixedFunctionStyle::RgbToHsyLin, "RGB_TO_HSY_LIN"),
        (FixedFunctionStyle::RgbToHsyLog, "RGB_TO_HSY_LOG"),
        (FixedFunctionStyle::RgbToHsyVid, "RGB_TO_HSY_VID"),
    ] {
        let transform = FixedFunctionTransform::create_with_params(style, &[]).unwrap();
        assert_eq!(transform.style(), style);
        assert_eq!(transform.try_style().unwrap(), style);
        let group = GroupTransform::create().unwrap();
        group.append_transform(&transform).unwrap();
        let ctf = group
            .write_to_string(&config, "Color Transform Format")
            .unwrap()
            .unwrap();
        assert!(ctf.contains(native_name), "{ctf}");
    }
}
