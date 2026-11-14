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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn processor_metadata_copies_do_not_mutate_shared_processor_state() {
    use ocio_rs::transform::FileTransform;
    let config = ocio_rs::Config::raw().unwrap();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/clf/matrix_3x4_example.clf");
    let transform = FileTransform::create().unwrap();
    transform.set_src(path.to_string_lossy()).unwrap();
    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap();
    let shared = processor.try_const_processor_metadata().unwrap().unwrap();
    let original_count = shared.num_files().unwrap();
    assert!(original_count > 0);
    let first = shared.file(0).unwrap().unwrap().to_owned();
    let editable = processor.try_processor_metadata().unwrap();
    let other_copy = shared.create_editable_copy().unwrap();
    assert_eq!(editable.num_files(), original_count);
    assert_eq!(editable.file(0).as_deref(), first.to_str().ok());
    editable.add_file("synthetic-edit.clf").unwrap();
    other_copy.add_look("synthetic-look").unwrap();
    assert_eq!(shared.num_files().unwrap(), original_count);
    assert_eq!(shared.num_looks().unwrap(), 0);
    drop(processor);
    assert_eq!(shared.file(0).unwrap().unwrap(), first.as_c_str());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn processor_metadata_copy_preserves_look_entries() {
    use ocio_rs::transform::LookTransform;
    let config = ocio_rs::Config::raw().unwrap();
    let scene = ocio_rs::ColorSpace::create().unwrap();
    scene.set_name("scene").unwrap();
    config.try_add_color_space(&scene).unwrap();
    let look = ocio_rs::Look::create().unwrap();
    look.set_name("film").unwrap();
    look.set_process_space("scene").unwrap();
    look.try_set_transform(&MatrixTransform::scale(&[2.0, 1.0, 1.0, 1.0]).unwrap())
        .unwrap();
    config.try_add_look(&look).unwrap();
    let transform = LookTransform::create().unwrap();
    transform.set_src("scene").unwrap();
    transform.set_dst("scene").unwrap();
    transform.set_looks("film").unwrap();
    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap();
    let shared = processor.try_const_processor_metadata().unwrap().unwrap();
    assert_eq!(shared.num_looks().unwrap(), 1);
    assert_eq!(shared.look(0).unwrap().unwrap(), c"film");
    let copy = processor.try_processor_metadata().unwrap();
    assert_eq!(copy.num_looks(), 1);
    assert_eq!(copy.look(0).as_deref(), Some("film"));
    copy.add_look("other").unwrap();
    assert_eq!(shared.num_looks().unwrap(), 1);
}
