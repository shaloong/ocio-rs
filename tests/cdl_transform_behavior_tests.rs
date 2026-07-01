//! CDLTransform behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/transform/cdl.rs`. In bundled/real mode they validate property round
//! trips, editable-copy independence, file/group loading, and real CPU
//! processing semantics.

mod common;
use common::*;

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::{CDLTransform, Transform};
use ocio_rs::{CDLStyle, TransformDirection};

fn cdl_transform_test_lock() -> MutexGuard<'static, ()> {
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

fn configured_cdl_transform() -> CDLTransform {
    let transform = CDLTransform::create().expect("cdl transform create");
    transform.set_slope(&[1.1, 1.2, 1.3]);
    transform.set_offset(&[0.01, 0.02, 0.03]);
    transform.set_power(&[1.0, 1.0, 1.0]);
    transform.set_sat(1.0);
    transform.set_style(CDLStyle::NoClamp);
    transform.set_id("cdl-behavior").expect("set id");
    transform
        .set_first_sop_description("behavior test")
        .expect("set sop description");
    transform
}

#[test]
fn cdl_transform_round_trip_and_copy_behavior() {
    let _guard = cdl_transform_test_lock();
    if is_stub() {
        return;
    }

    let transform = configured_cdl_transform();

    assert_vec_close(&transform.slope(), &[1.1, 1.2, 1.3], 1e-10);
    assert_vec_close(&transform.offset(), &[0.01, 0.02, 0.03], 1e-10);
    assert_vec_close(&transform.power_(), &[1.0, 1.0, 1.0], 1e-10);
    assert_close(transform.sat(), 1.0, 1e-10);
    assert_eq!(transform.style(), CDLStyle::NoClamp);
    assert_eq!(transform.id().as_deref(), Some("cdl-behavior"));
    assert_eq!(
        transform.first_sop_description().as_deref(),
        Some("behavior test")
    );
    assert_eq!(transform.direction(), TransformDirection::Forward);

    let sop = transform.sop();
    assert_vec_close(&sop[0..3], &[1.1, 1.2, 1.3], 1e-10);
    assert_vec_close(&sop[3..6], &[0.01, 0.02, 0.03], 1e-10);
    assert_vec_close(&sop[6..9], &[1.0, 1.0, 1.0], 1e-10);

    let copy = transform
        .create_editable_copy()
        .expect("cdl transform editable copy");
    copy.set_slope(&[0.9, 0.8, 0.7]);
    copy.set_offset(&[0.0, 0.0, 0.0]);
    copy.set_power(&[0.9, 0.9, 0.9]);
    copy.set_sat(0.7);
    copy.set_style(CDLStyle::Asc);
    copy.set_direction(TransformDirection::Inverse);
    copy.set_id("cdl-copy").expect("set copy id");
    copy.set_first_sop_description("copy")
        .expect("set copy sop description");

    assert_vec_close(&copy.slope(), &[0.9, 0.8, 0.7], 1e-10);
    assert_close(copy.sat(), 0.7, 1e-10);
    assert_eq!(copy.style(), CDLStyle::Asc);
    assert_eq!(copy.direction(), TransformDirection::Inverse);
    assert_eq!(copy.id().as_deref(), Some("cdl-copy"));
    assert_eq!(copy.first_sop_description().as_deref(), Some("copy"));

    assert_vec_close(&transform.slope(), &[1.1, 1.2, 1.3], 1e-10);
    assert_close(transform.sat(), 1.0, 1e-10);
    assert_eq!(transform.style(), CDLStyle::NoClamp);
    assert_eq!(transform.direction(), TransformDirection::Forward);
    assert_eq!(transform.id().as_deref(), Some("cdl-behavior"));
    assert_eq!(
        transform.first_sop_description().as_deref(),
        Some("behavior test")
    );
}

#[test]
fn cdl_transform_processor_behavior() {
    let _guard = cdl_transform_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = configured_cdl_transform();

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

    let original = [0.2f32, 0.4, 0.6, 1.0];
    let mut processed = original;
    forward_cpu.apply_rgba(&mut processed);

    assert_close(processed[0] as f64, 0.23, 1e-6);
    assert_close(processed[1] as f64, 0.50, 1e-6);
    assert_close(processed[2] as f64, 0.81, 1e-6);
    assert_close(processed[3] as f64, 1.0, 1e-6);

    let mut restored = processed;
    inverse_cpu.apply_rgba(&mut restored);

    assert_close(restored[0] as f64, original[0] as f64, 2e-5);
    assert_close(restored[1] as f64, original[1] as f64, 2e-5);
    assert_close(restored[2] as f64, original[2] as f64, 2e-5);
    assert_close(restored[3] as f64, original[3] as f64, 2e-5);
}

#[test]
fn cdl_transform_file_loading_behavior() {
    let _guard = cdl_transform_test_lock();
    if is_stub() {
        return;
    }

    let ccc_path = test_data_path("cdl/cdl_test1.ccc");
    let cdl = CDLTransform::create_from_file(ccc_path.to_string_lossy(), "cc0002")
        .expect("load cdl from ccc");

    assert_vec_close(&cdl.slope(), &[0.9, 0.7, 0.6], 1e-10);
    assert_vec_close(&cdl.offset(), &[0.1, 0.1, 0.1], 1e-10);
    assert_vec_close(&cdl.power_(), &[0.9, 0.9, 0.9], 1e-10);
    assert_close(cdl.sat(), 0.7, 1e-10);
    assert_eq!(cdl.id().as_deref(), Some("cc0002"));
    assert_eq!(cdl.first_sop_description().as_deref(), Some("pastel"));

    let group = CDLTransform::create_group_from_file(ccc_path.to_string_lossy())
        .expect("load cdl group from ccc");
    assert_eq!(group.num_transforms(), 5);

    match group.transform(0) {
        Some(Transform::CDL(first)) => {
            assert_eq!(first.id().as_deref(), Some("cc0001"));
            assert_eq!(first.first_sop_description().as_deref(), Some("Example look"));
        }
        other => panic!("expected first transform to be CDL, got {:?}", other.is_some()),
    }
}
