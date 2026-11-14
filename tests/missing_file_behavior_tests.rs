use ocio_rs::transform::FileTransform;
use ocio_rs::{Config, OcioError, TransformDirection};

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn missing_config_and_lut_keep_native_error_kind() {
    let error = match Config::from_file("ocio-rs-nonexistent-config.ocio") {
        Ok(_) => panic!("missing config must fail"),
        Err(error) => error,
    };
    if ocio_rs::version_hex() >= 0x02060000 {
        assert!(matches!(error, OcioError::MissingFile(_)), "{error:?}");
    } else {
        assert!(
            matches!(error, OcioError::Ocio(_) | OcioError::MissingFile(_)),
            "{error:?}"
        );
    }

    let config = Config::raw().expect("raw config");
    let transform = FileTransform::create().expect("file transform");
    transform
        .set_src("ocio-rs-nonexistent-lut.cube")
        .expect("source");
    let error = match config.processor_from_transform(&transform, TransformDirection::Forward) {
        Ok(_) => panic!("missing LUT must fail"),
        Err(error) => error,
    };
    assert!(matches!(error, OcioError::MissingFile(_)), "{error:?}");

    let error = match config.processor("ocio-rs-nonexistent-space", "raw") {
        Ok(_) => panic!("missing color space must fail"),
        Err(error) => error,
    };
    assert!(matches!(error, OcioError::Ocio(_)), "{error:?}");
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn missing_file_errors_are_thread_local_and_consumed() {
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                let config = Config::raw().expect("raw config");
                let transform = FileTransform::create().expect("file transform");
                transform
                    .set_src("ocio-rs-nonexistent-lut.cube")
                    .expect("source");
                let error = match config
                    .processor_from_transform(&transform, TransformDirection::Forward)
                {
                    Ok(_) => panic!("missing LUT must fail"),
                    Err(error) => error,
                };
                assert!(matches!(error, OcioError::MissingFile(_)));
                assert_eq!(unsafe { ocio_rs::ocio_sys::ocio_error_get_last_kind() }, 0);
                Config::raw().expect("successful call after error");
            })
        })
        .collect();
    for thread in threads {
        thread.join().expect("worker");
    }
}
