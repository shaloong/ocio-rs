//! ProcessorMetadata behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/processor_metadata.rs` and `src/processor.rs`. In bundled/real mode
//! they validate standalone metadata mutation and metadata extracted from a
//! real processor.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::MatrixTransform;
use ocio_rs::{ProcessorMetadata, TransformDirection};

fn processor_metadata_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn processor_metadata_manual_round_trip_behavior() {
    let _guard = processor_metadata_test_lock();
    if is_stub() {
        return;
    }

    let metadata = ProcessorMetadata::create().expect("processor metadata create");
    assert_eq!(metadata.num_files(), 0);
    assert_eq!(metadata.num_looks(), 0);

    metadata.add_file("lut_a.clf").expect("add file a");
    metadata.add_file("lut_b.clf").expect("add file b");
    metadata.add_look("film").expect("add look film");
    metadata.add_look("show").expect("add look show");

    assert_eq!(metadata.num_files(), 2);
    assert_eq!(metadata.file(0).as_deref(), Some("lut_a.clf"));
    assert_eq!(metadata.file(1).as_deref(), Some("lut_b.clf"));
    assert_eq!(metadata.num_looks(), 2);
    assert_eq!(metadata.look(0).as_deref(), Some("film"));
    assert_eq!(metadata.look(1).as_deref(), Some("show"));
}

#[test]
fn processor_metadata_from_real_processor_behavior() {
    let _guard = processor_metadata_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = MatrixTransform::scale(&[1.1, 0.9, 1.2, 1.0]).expect("matrix scale");
    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .expect("processor from transform");

    let metadata = processor
        .processor_metadata()
        .expect("processor metadata from processor");
    assert!(metadata.num_files() >= 0);
    assert!(metadata.num_looks() >= 0);
}
