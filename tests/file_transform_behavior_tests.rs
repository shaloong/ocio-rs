//! FileTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/file.rs`. In bundled/real mode they validate property round
//! trips, editable-copy independence, CLF/CDL file loading, and processor
//! behavior through the file-backed transform path.

mod common;
use common::*;

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::{CDLTransform, FileTransform};
use ocio_rs::{CDLStyle, Interpolation, TransformDirection};

fn file_transform_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn test_data_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(rel)
}

#[test]
fn file_transform_round_trip_and_copy_behavior() {
    let _guard = file_transform_test_lock();
    if is_stub() {
        return;
    }

    let matrix_path = test_data_path("clf/matrix_3x4_example.clf");
    let transform = FileTransform::create().expect("file transform create");
    transform
        .set_src(matrix_path.to_string_lossy())
        .expect("set src");
    transform.set_ccc_id("cc0002").expect("set ccc id");
    transform.set_interpolation(Interpolation::Tetrahedral);
    transform.set_cdl_style(CDLStyle::NoClamp);
    transform.set_direction(TransformDirection::Inverse);

    assert_eq!(
        transform.src().as_deref(),
        Some(matrix_path.to_string_lossy().as_ref())
    );
    assert_eq!(transform.ccc_id().as_deref(), Some("cc0002"));
    assert_eq!(transform.interpolation(), Interpolation::Tetrahedral);
    assert_eq!(transform.cdl_style(), CDLStyle::NoClamp);
    assert_eq!(transform.direction(), TransformDirection::Inverse);

    let copy = transform
        .create_editable_copy()
        .expect("file transform editable copy");
    copy.set_ccc_id("cc0003").expect("set copy ccc id");
    copy.set_interpolation(Interpolation::Linear);
    copy.set_cdl_style(CDLStyle::Asc);
    copy.set_direction(TransformDirection::Forward);

    assert_eq!(copy.ccc_id().as_deref(), Some("cc0003"));
    assert_eq!(copy.interpolation(), Interpolation::Linear);
    assert_eq!(copy.cdl_style(), CDLStyle::Asc);
    assert_eq!(copy.direction(), TransformDirection::Forward);

    assert_eq!(transform.ccc_id().as_deref(), Some("cc0002"));
    assert_eq!(transform.interpolation(), Interpolation::Tetrahedral);
    assert_eq!(transform.cdl_style(), CDLStyle::NoClamp);
    assert_eq!(transform.direction(), TransformDirection::Inverse);
}

#[test]
fn file_transform_clf_processor_behavior() {
    let _guard = file_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let matrix_path = test_data_path("clf/matrix_3x4_example.clf");
    let transform = FileTransform::create().expect("file transform create");
    transform
        .set_src(matrix_path.to_string_lossy())
        .expect("set src");

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

    let original = [0.1f32, 0.2, 0.3, 0.4];
    let mut processed = original;
    forward_cpu.apply_rgba(&mut processed);

    // This CLF carries integer in/out bit-depth metadata, so real OCIO
    // normalizes code values around the matrix op instead of treating it like a
    // bare 32f matrix.
    assert_close(processed[0] as f64, 0.08001465, 1e-6);
    assert_close(processed[1] as f64, 0.18735044, 1e-6);
    assert_close(processed[2] as f64, 0.24222468, 1e-6);
    assert_close(processed[3] as f64, 0.4, 1e-6);

    let mut restored = processed;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 2e-5);
    assert_close(restored[1] as f64, original[1] as f64, 2e-5);
    assert_close(restored[2] as f64, original[2] as f64, 2e-5);
    assert_close(restored[3] as f64, original[3] as f64, 2e-5);
}

#[test]
fn file_transform_ccc_id_and_default_direction_behavior() {
    let _guard = file_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let ccc_path = test_data_path("cdl/cdl_test1.ccc");

    let file_transform = FileTransform::create().expect("file transform create");
    file_transform
        .set_src(ccc_path.to_string_lossy())
        .expect("set ccc src");
    file_transform.set_ccc_id("cc0002").expect("set ccc id");
    file_transform.set_cdl_style(CDLStyle::NoClamp);
    file_transform.set_direction(TransformDirection::Inverse);

    let cdl = CDLTransform::create_from_file(ccc_path.to_string_lossy(), "cc0002")
        .expect("load cdl from ccc");
    cdl.set_style(CDLStyle::NoClamp);

    let file_cpu = config
        .processor_from_transform_default_direction(&file_transform)
        .expect("file processor")
        .default_cpu_processor()
        .expect("file cpu");
    let cdl_cpu = config
        .processor_from_transform(&cdl, TransformDirection::Inverse)
        .expect("cdl processor")
        .default_cpu_processor()
        .expect("cdl cpu");

    let mut file_pixel = [0.28f32, 0.42, 0.56, 1.0];
    let mut cdl_pixel = file_pixel;
    file_cpu.apply_rgba(&mut file_pixel);
    cdl_cpu.apply_rgba(&mut cdl_pixel);

    assert_close(file_pixel[0] as f64, cdl_pixel[0] as f64, 1e-6);
    assert_close(file_pixel[1] as f64, cdl_pixel[1] as f64, 1e-6);
    assert_close(file_pixel[2] as f64, cdl_pixel[2] as f64, 1e-6);
    assert_close(file_pixel[3] as f64, cdl_pixel[3] as f64, 1e-6);
}
