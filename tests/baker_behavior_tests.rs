//! Baker behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/baker.rs`. In bundled/real mode they validate setter round trips,
//! format enumeration, and actual LUT text generation.

mod common;
use common::*;

use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use ocio_rs::{Baker, Config};

fn baker_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn unique_temp_path(label: &str, extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    std::env::temp_dir().join(format!("ocio-rs-{label}-{nanos}.{extension}"))
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn baker_property_round_trip_behavior() {
    let _guard = baker_test_lock();
    if is_stub() {
        return;
    }

    let baker = Baker::create().expect("baker create");
    let config = Config::raw().expect("raw config");
    baker.set_config(&config).expect("attach config");

    baker.set_format("resolve_cube").expect("set format");
    baker.set_input_space("raw").expect("set input");
    baker.set_target_space("raw").expect("set target");
    baker.set_shaper_space("raw").expect("set shaper");
    baker.set_looks("").expect("set looks");
    baker.set_shaper_size(8).expect("set shaper size");
    baker.set_cube_size(2).expect("set cube size");

    assert_eq!(baker.format().as_deref(), Some("resolve_cube"));
    assert_eq!(baker.input_space().as_deref(), Some("raw"));
    assert_eq!(baker.target_space().as_deref(), Some("raw"));
    assert_eq!(baker.shaper_space().as_deref(), Some("raw"));
    assert_eq!(baker.looks().as_deref(), Some(""));
    assert_eq!(baker.shaper_size(), 8);
    assert_eq!(baker.cube_size(), 2);

    let attached_config = baker.config().expect("baker config");
    attached_config
        .validate()
        .expect("attached config validate");
    assert!(attached_config.num_color_spaces() > 0);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn baker_configuration_isolation_and_shared_const_lifetime() {
    let _guard = baker_test_lock();
    let source = Config::raw().unwrap().create_editable_copy().unwrap();
    source.set_name("original").unwrap();
    let baker = Baker::create().unwrap();
    assert!(baker.const_config().unwrap().is_none());
    baker.set_config(&source).unwrap();
    source.set_name("source changed").unwrap();

    let shared = baker.const_config().unwrap().unwrap();
    assert_eq!(shared.name().unwrap().as_deref(), Some("original"));
    let editable = baker.config().unwrap();
    editable.set_name("editable changed").unwrap();
    assert_eq!(shared.name().unwrap().as_deref(), Some("original"));
    assert_eq!(baker.config().unwrap().name().as_deref(), Some("original"));

    let other = Baker::create().unwrap();
    other.set_const_config(&shared).unwrap();
    assert_eq!(other.config().unwrap().name().as_deref(), Some("original"));
    baker.set_config(&source).unwrap();
    drop(baker);
    assert_eq!(shared.name().unwrap().as_deref(), Some("original"));
    assert_eq!(other.config().unwrap().name().as_deref(), Some("original"));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn baker_static_format_registry_contains_resolve_cube_behavior() {
    let _guard = baker_test_lock();
    if is_stub() {
        return;
    }

    let num_formats = Baker::num_formats();
    assert!(num_formats > 0);

    let mut found_resolve_cube = false;
    for index in 0..num_formats {
        let name = Baker::format_name_by_index(index).expect("format name");
        let ext = Baker::format_extension_by_index(index).expect("format extension");
        assert!(!name.is_empty());
        assert!(!ext.is_empty());

        if name == "resolve_cube" {
            found_resolve_cube = true;
            assert_eq!(ext, "cube");
        }
    }

    assert!(found_resolve_cube, "expected resolve_cube baker format");
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn baker_bake_to_string_and_file_behavior() {
    let _guard = baker_test_lock();
    if is_stub() {
        return;
    }

    let baker = Baker::create().expect("baker create");
    let config = Config::raw().expect("raw config");
    baker.set_config(&config).expect("attach config");
    baker.set_format("resolve_cube").expect("set format");
    baker.set_input_space("raw").expect("set input");
    baker.set_target_space("raw").expect("set target");
    baker.set_cube_size(2).expect("set cube size");

    let baked = baker
        .bake_to_string()
        .expect("bake_to_string")
        .expect("real baker output");
    assert!(!baked.trim().is_empty());
    assert!(baked.contains("LUT_1D_SIZE 2"));
    assert!(baked.lines().any(|line| line.contains("0.000000")));
    assert_eq!(baker.bake_to_bytes().unwrap().unwrap(), baked.as_bytes());
    struct ShortWriter(Vec<u8>);
    impl std::io::Write for ShortWriter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            let n = bytes.len().min(7);
            self.0.extend_from_slice(&bytes[..n]);
            Ok(n)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut writer = ShortWriter(Vec::new());
    baker.bake_to_writer(&mut writer).unwrap();
    assert_eq!(writer.0, baked.as_bytes());
    let mut dynamic = Vec::new();
    baker
        .bake_to_writer(&mut dynamic as &mut dyn std::io::Write)
        .unwrap();
    assert_eq!(dynamic, baked.as_bytes());

    let output_path = unique_temp_path("baker", "cube");
    baker
        .bake(output_path.to_string_lossy())
        .expect("bake file");
    let written = fs::read_to_string(&output_path).expect("read baked file");
    assert_eq!(written, baked);

    let _ = fs::remove_file(output_path);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn baker_writer_errors_and_panics_do_not_cross_ffi() {
    struct Failed(bool);
    impl std::io::Write for Failed {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            assert!(!self.0, "intentional writer panic");
            Err(std::io::Error::other("writer rejected output"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let _guard = baker_test_lock();
    let baker = Baker::create().unwrap();
    baker.set_config(&Config::raw().unwrap()).unwrap();
    baker.set_format("resolve_cube").unwrap();
    baker.set_input_space("raw").unwrap();
    baker.set_target_space("raw").unwrap();
    baker.set_cube_size(2).unwrap();
    for panic in [false, true] {
        let error = baker
            .bake_to_writer(&mut Failed(panic))
            .unwrap_err()
            .to_string();
        assert!(error.contains(if panic {
            "writer panicked"
        } else {
            "writer rejected output"
        }));
    }
    #[derive(Debug)]
    struct PanicDisplay;
    impl std::fmt::Display for PanicDisplay {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            panic!("formatting writer error");
        }
    }
    impl std::error::Error for PanicDisplay {}
    struct PanicDrop;
    impl Drop for PanicDrop {
        fn drop(&mut self) {
            panic!("dropping writer panic payload");
        }
    }
    struct AdversarialWriter(bool);
    impl std::io::Write for AdversarialWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            if self.0 {
                std::panic::panic_any(PanicDrop);
            }
            Err(std::io::Error::other(PanicDisplay))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    for panic_payload in [false, true] {
        let error = baker
            .bake_to_writer(&mut AdversarialWriter(panic_payload))
            .unwrap_err();
        assert!(error.to_string().contains("writer panicked"));
    }
    struct Zero;
    impl std::io::Write for Zero {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    assert!(baker.bake_to_writer(&mut Zero).is_err());
    struct Partial(usize);
    impl std::io::Write for Partial {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if self.0 != 0 {
                return Err(std::io::Error::other("partial write failure"));
            }
            let n = bytes.len().min(5);
            self.0 += n;
            Ok(n)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut partial = Partial(0);
    assert!(baker
        .bake_to_writer(&mut partial)
        .unwrap_err()
        .to_string()
        .contains("partial write failure"));
    assert_eq!(partial.0, 5);
    assert!(baker.bake_to_bytes().unwrap().is_some());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn baker_streams_large_output_in_bounded_chunks() {
    struct Sink {
        bytes: Vec<u8>,
        max_chunk: usize,
        calls: usize,
    }
    impl std::io::Write for Sink {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.max_chunk = self.max_chunk.max(bytes.len());
            self.calls += 1;
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let _guard = baker_test_lock();
    let baker = Baker::create().unwrap();
    baker.set_config(&Config::raw().unwrap()).unwrap();
    baker.set_format("resolve_cube").unwrap();
    baker.set_input_space("raw").unwrap();
    baker.set_target_space("raw").unwrap();
    baker.set_cube_size(2048).unwrap();
    let mut sink = Sink {
        bytes: Vec::new(),
        max_chunk: 0,
        calls: 0,
    };
    baker.bake_to_writer(&mut sink).unwrap();
    assert!(sink.calls > 1);
    assert!(sink.max_chunk <= 16384);
    assert_eq!(sink.bytes, baker.bake_to_bytes().unwrap().unwrap());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn baker_writer_mutation_cannot_invalidate_native_metadata_in_flight() {
    struct MutatingWriter<'a> {
        metadata: &'a ocio_rs::FormatMetadata,
        bytes: Vec<u8>,
        mutated: bool,
    }
    impl std::io::Write for MutatingWriter<'_> {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if !self.mutated {
                self.metadata.clear();
                self.mutated = true;
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let _guard = baker_test_lock();
    let baker = Baker::create().unwrap();
    baker.set_config(&Config::raw().unwrap()).unwrap();
    baker.set_format("resolve_cube").unwrap();
    baker.set_input_space("raw").unwrap();
    baker.set_target_space("raw").unwrap();
    baker.set_cube_size(2).unwrap();
    let metadata = baker.format_metadata().unwrap();
    metadata
        .add_child_element("Comment", "x".repeat(40_000))
        .unwrap();
    let expected = baker.bake_to_bytes().unwrap().unwrap();
    assert!(expected.len() > 40_000);
    let mut writer = MutatingWriter {
        metadata: &metadata,
        bytes: Vec::new(),
        mutated: false,
    };
    baker.bake_to_writer(&mut writer).unwrap();
    assert!(writer.mutated);
    assert_eq!(writer.bytes, expected);
    assert!(baker.bake_to_bytes().unwrap().unwrap().len() < expected.len());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn baker_invalid_format_reports_error_behavior() {
    let _guard = baker_test_lock();
    if is_stub() {
        return;
    }

    let baker = Baker::create().expect("baker create");
    let err = baker
        .set_format("definitely_not_a_real_bake_format")
        .expect_err("invalid baker format should fail");
    assert!(
        matches!(err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn baker_unconfigured_bake_surfaces_ocio_error_behavior() {
    let _guard = baker_test_lock();
    if is_stub() {
        return;
    }

    let baker = Baker::create().expect("baker create");
    let err = baker
        .bake_to_string()
        .expect_err("unconfigured Baker must not silently produce no output");
    assert!(matches!(err, ocio_rs::OcioError::Ocio(_)));

    use ocio_rs::ocio_sys;
    unsafe {
        let handle = ocio_sys::ocio_baker_create();
        assert!(!handle.is_null());
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_baker_bake(handle, std::ptr::null_mut());
        let reports_error = !ocio_sys::ocio_error_get_last().is_null();
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_baker_destroy(handle);
        assert!(reports_error);
    }
}
