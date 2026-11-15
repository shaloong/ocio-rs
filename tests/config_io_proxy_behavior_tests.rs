//! ConfigIOProxy behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/config_io_proxy.rs`. In bundled/real mode they validate payload round
//! trips, config/context proxy attachment, and full config-driven processing
//! from in-memory config and LUT assets.

mod common;
use common::*;

use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::{Config, ConfigIOProvider, ConfigIOProxy, Context};

struct OnDemandProvider {
    config: String,
    lut: Vec<u8>,
    calls: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl ConfigIOProvider for OnDemandProvider {
    fn config_data(&self) -> ocio_rs::Result<Cow<'_, str>> {
        Ok(Cow::Borrowed(&self.config))
    }
    fn lut_data<'a>(&'a self, path: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
        self.calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if path.to_bytes().ends_with(b"lut1.clf") {
            Ok(Cow::Borrowed(&self.lut))
        } else {
            Ok(Cow::Borrowed(&[]))
        }
    }
    fn fast_lut_file_hash<'a>(&'a self, path: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
        Ok(Cow::Borrowed(if path.to_bytes().ends_with(b"lut1.clf") {
            &b"\0on-demand-lut-v1\xff"[..]
        } else {
            &b""[..]
        }))
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_and_context_can_detach_proxy_without_invalidating_retained_handle() {
    let _guard = config_io_proxy_test_lock();
    let proxy = ConfigIOProxy::create().unwrap();
    proxy.set_config_data("retained").unwrap();
    let config = Config::raw().unwrap();
    let context = Context::create().unwrap();
    config.set_config_io_proxy_object(&proxy).unwrap();
    context.set_config_io_proxy_object(&proxy).unwrap();
    let retained = context.try_config_io_proxy_object().unwrap().unwrap();
    let retained_from_config = config.try_config_io_proxy_object().unwrap().unwrap();
    drop(proxy);
    config.clear_config_io_proxy().unwrap();
    context.clear_config_io_proxy().unwrap();
    assert!(config.try_config_io_proxy_object().unwrap().is_none());
    assert!(context.try_config_io_proxy_object().unwrap().is_none());
    config.clear_config_io_proxy().unwrap();
    context.clear_config_io_proxy().unwrap();
    drop(context);
    drop(config);
    assert_eq!(
        retained.try_config_data().unwrap().as_deref(),
        Some("retained")
    );
    assert_eq!(
        retained_from_config.try_config_data().unwrap().as_deref(),
        Some("retained")
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_payload_preserves_native_length_and_provider_errors() {
    let _guard = config_io_proxy_test_lock();
    let proxy = ConfigIOProxy::create().unwrap();
    for data in [&b""[..], &b"before\0after\xff"[..]] {
        proxy.set_config_data_bytes(data).unwrap();
        assert_eq!(
            proxy.try_config_data_bytes().unwrap().as_deref(),
            Some(data)
        );
    }
    assert_eq!(
        proxy.try_config_data().unwrap().unwrap(),
        "before\0after\u{fffd}"
    );
    proxy.set_config_data("a\0b").unwrap();
    assert_eq!(proxy.try_config_data().unwrap().as_deref(), Some("a\0b"));

    struct Provider {
        fail: bool,
    }
    impl ConfigIOProvider for Provider {
        fn config_data(&self) -> ocio_rs::Result<Cow<'_, str>> {
            if self.fail {
                Err(ocio_rs::OcioError::InvalidInput("config failure".into()))
            } else {
                Ok(Cow::Borrowed("provider\0payload"))
            }
        }
        fn lut_data<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(&[]))
        }
        fn fast_lut_file_hash<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(&[]))
        }
    }
    let provider = ConfigIOProxy::from_provider(Provider { fail: false }).unwrap();
    assert_eq!(
        provider.try_config_data_bytes().unwrap().as_deref(),
        Some(&b"provider\0payload"[..])
    );
    assert!(provider.set_config_data_bytes(b"x").is_err());
    let failure = ConfigIOProxy::from_provider(Provider { fail: true }).unwrap();
    assert!(failure
        .try_config_data_bytes()
        .unwrap_err()
        .to_string()
        .contains("config failure"));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn custom_provider_returns_arbitrary_config_bytes() {
    struct BinaryProvider {
        owned: bool,
    }
    impl ConfigIOProvider for BinaryProvider {
        fn config_data_bytes(&self) -> ocio_rs::Result<Cow<'_, [u8]>> {
            let data = &b"\xff\0configuration"[..];
            Ok(if self.owned {
                Cow::Owned(data.to_vec())
            } else {
                Cow::Borrowed(data)
            })
        }
        fn lut_data<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(&[]))
        }
        fn fast_lut_file_hash<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(&[]))
        }
    }
    let _guard = config_io_proxy_test_lock();
    for owned in [false, true] {
        let proxy = ConfigIOProxy::from_provider(BinaryProvider { owned }).unwrap();
        assert_eq!(
            proxy.try_config_data_bytes().unwrap().as_deref(),
            Some(&b"\xff\0configuration"[..])
        );
        assert_eq!(
            proxy.try_config_data().unwrap().as_deref(),
            Some("\u{fffd}\0configuration")
        );
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn byte_provider_supplies_config_and_lut_to_native_processor() {
    struct Assets {
        config: Vec<u8>,
        lut: Vec<u8>,
    }
    impl ConfigIOProvider for Assets {
        fn config_data_bytes(&self) -> ocio_rs::Result<Cow<'_, [u8]>> {
            Ok(Cow::Borrowed(&self.config))
        }
        fn lut_data<'a>(&'a self, path: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(if path.to_bytes().ends_with(b"lut1.clf") {
                &self.lut
            } else {
                &[]
            }))
        }
        fn fast_lut_file_hash<'a>(
            &'a self,
            path: &std::ffi::CStr,
        ) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(if path.to_bytes().ends_with(b"lut1.clf") {
                &b"byte-provider-lut-v1"[..]
            } else {
                b""
            }))
        }
    }
    let _guard = config_io_proxy_test_lock();
    let proxy = ConfigIOProxy::from_provider(Assets {
        config: fs::read(test_data_path("configs/context_test1/config.ocio")).unwrap(),
        lut: fs::read(test_data_path("configs/context_test1/shot1/lut1.clf")).unwrap(),
    })
    .unwrap();
    let config = Config::from_config_io_proxy(&proxy).unwrap();
    config.set_working_dir("E:/virtual/context").unwrap();
    drop(proxy);
    let cpu = config
        .processor("shot1_lut1_cs", "reference")
        .unwrap()
        .optimized_cpu_processor(0)
        .unwrap();
    let mut pixel = [1.0f32; 4];
    cpu.try_apply_rgba(&mut pixel).unwrap();
    assert_close(pixel[0] as f64, 10.0, 1e-6);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn custom_provider_loads_lut_on_demand_and_outlives_proxy_handle() {
    let _guard = config_io_proxy_test_lock();
    let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let provider = OnDemandProvider {
        config: fs::read_to_string(test_data_path("configs/context_test1/config.ocio")).unwrap(),
        lut: fs::read(test_data_path("configs/context_test1/shot1/lut1.clf")).unwrap(),
        calls: calls.clone(),
    };
    let proxy = ConfigIOProxy::from_provider(provider).unwrap();
    assert!(proxy.try_config_data().unwrap().is_some());
    assert_eq!(
        proxy
            .try_fast_lut_file_hash_bytes("E:/virtual/context/shot1/lut1.clf")
            .unwrap()
            .as_deref(),
        Some(&b"\0on-demand-lut-v1\xff"[..])
    );
    assert_eq!(
        proxy
            .try_lut_data("E:/virtual/context/shot1/lut1.clf")
            .unwrap()
            .unwrap(),
        fs::read(test_data_path("configs/context_test1/shot1/lut1.clf")).unwrap()
    );
    assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(proxy.try_lut_data("missing.clf").unwrap(), None);
    assert!(!proxy.try_copy_lut_data("missing.clf", &mut []).unwrap());
    let mut small = [42; 1];
    assert!(!proxy
        .try_copy_lut_data("E:/virtual/context/shot1/lut1.clf", &mut small)
        .unwrap());
    assert_eq!(small, [42]);
    let mut copied = vec![
        0;
        fs::metadata(test_data_path("configs/context_test1/shot1/lut1.clf"))
            .unwrap()
            .len() as usize
    ];
    assert!(proxy
        .try_copy_lut_data("E:/virtual/context/shot1/lut1.clf", &mut copied)
        .unwrap());
    assert_eq!(
        copied,
        fs::read(test_data_path("configs/context_test1/shot1/lut1.clf")).unwrap()
    );
    assert!(proxy.set_config_data("not supported").is_err());
    assert!(proxy.set_lut_data("file", &[1], "hash").is_err());
    let config = Config::from_config_io_proxy(&proxy).unwrap();
    config.set_working_dir("E:/virtual/context").unwrap();
    drop(proxy);
    let cpu = config
        .processor("shot1_lut1_cs", "reference")
        .unwrap()
        .optimized_cpu_processor(0)
        .unwrap();
    let mut pixel = [1.0f32; 4];
    cpu.try_apply_rgba(&mut pixel).unwrap();
    assert_close(pixel[0] as f64, 10.0, 1e-6);
    assert!(calls.load(std::sync::atomic::Ordering::Relaxed) > 0);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn memory_proxy_normalizes_mixed_separators_and_replaces_payload_and_hash() {
    let _guard = config_io_proxy_test_lock();
    let proxy = ConfigIOProxy::create().unwrap();
    assert!(proxy
        .set_lut_data(r"dir\sub/file.clf", &[1, 2, 3], "first")
        .unwrap());
    assert!(proxy
        .set_lut_data(r"dir/sub\file.clf", &[4, 5], "second")
        .unwrap());
    for path in [
        r"dir\sub/file.clf",
        r"dir/sub\file.clf",
        "dir/sub/file.clf",
        r"dir\sub\file.clf",
    ] {
        assert_eq!(proxy.try_lut_data(path).unwrap(), Some(vec![4, 5]));
        assert_eq!(
            proxy.try_fast_lut_file_hash(path).unwrap().as_deref(),
            Some("second")
        );
        let mut data = [0; 2];
        assert!(proxy.try_copy_lut_data(path, &mut data).unwrap());
        assert_eq!(data, [4, 5]);
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn shared_memory_proxy_concurrent_read_and_replace_uses_stable_snapshots() {
    use std::sync::{Arc, Barrier};

    struct Restore(Config);
    impl Drop for Restore {
        fn drop(&mut self) {
            ocio_rs::try_set_current_config(&self.0).unwrap();
        }
    }
    let _guard = config_io_proxy_test_lock();
    let _restore = Restore(ocio_rs::try_current_config().unwrap().unwrap());
    let proxy = ConfigIOProxy::create().unwrap();
    proxy.set_config_data("first").unwrap();
    proxy.set_lut_data("shared.clf", &[1; 4096], "one").unwrap();
    let config = Config::raw().unwrap();
    config.set_config_io_proxy_object(&proxy).unwrap();
    ocio_rs::try_set_current_config(&config).unwrap();

    let start = Arc::new(Barrier::new(3));
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let start = Arc::clone(&start);
            std::thread::spawn(move || {
                let config = ocio_rs::try_current_config().unwrap().unwrap();
                let proxy = config.try_config_io_proxy_object().unwrap().unwrap();
                start.wait();
                start.wait();
                let initial = proxy.try_lut_data("shared.clf").unwrap();
                start.wait();
                assert_eq!(initial.unwrap(), vec![2; 4096]);
                for _ in 0..1000 {
                    let data = proxy.try_lut_data("shared.clf").unwrap().unwrap();
                    assert_eq!(data.len(), 4096);
                    assert!(data.iter().all(|&byte| byte == data[0]));
                    assert!([1, 2].contains(&data[0]));
                    let mut buffer = [0; 4096];
                    assert!(proxy.try_copy_lut_data("shared.clf", &mut buffer).unwrap());
                    assert!(buffer.iter().all(|&byte| byte == buffer[0]));
                    assert!([1, 2].contains(&buffer[0]));
                    let config_data = proxy.try_config_data().unwrap().unwrap();
                    assert!(config_data == "first" || config_data == "second");
                }
            })
        })
        .collect();
    start.wait();
    proxy.set_lut_data("shared.clf", &[2; 4096], "two").unwrap();
    start.wait();
    start.wait();
    for index in 0..1000 {
        let (value, hash, data) = if index % 2 == 0 {
            (1, "one", "first")
        } else {
            (2, "two", "second")
        };
        proxy
            .set_lut_data("shared.clf", &[value; 4096], hash)
            .unwrap();
        proxy.set_config_data(data).unwrap();
    }
    for thread in threads {
        thread.join().unwrap();
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn custom_provider_errors_and_panics_do_not_cross_ffi() {
    struct Failing(bool);
    impl ConfigIOProvider for Failing {
        fn config_data(&self) -> ocio_rs::Result<Cow<'_, str>> {
            if self.0 {
                panic!("provider panic");
            }
            Err(ocio_rs::OcioError::InvalidInput("provider error".into()))
        }
        fn lut_data<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(&[]))
        }
        fn fast_lut_file_hash<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(b""))
        }
    }
    let _guard = config_io_proxy_test_lock();
    for panic in [false, true] {
        let proxy = ConfigIOProxy::from_provider(Failing(panic)).unwrap();
        let expected = if panic {
            "provider panicked"
        } else {
            "provider error"
        };
        assert!(proxy
            .try_config_data()
            .unwrap_err()
            .to_string()
            .contains(expected));
        assert!(Config::from_config_io_proxy(&proxy)
            .err()
            .unwrap()
            .to_string()
            .contains(expected));
    }
    struct LutFailure;
    impl ConfigIOProvider for LutFailure {
        fn config_data(&self) -> ocio_rs::Result<Cow<'_, str>> {
            Ok(Cow::Borrowed(""))
        }
        fn lut_data<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Err(ocio_rs::OcioError::InvalidInput(
                "LUT backend unavailable".into(),
            ))
        }
        fn fast_lut_file_hash<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(b"known"))
        }
    }
    let proxy = ConfigIOProxy::from_provider(LutFailure).unwrap();
    assert!(proxy
        .try_lut_data("test.clf")
        .unwrap_err()
        .to_string()
        .contains("LUT backend unavailable"));
    assert!(proxy
        .try_copy_lut_data("test.clf", &mut [0; 8])
        .unwrap_err()
        .to_string()
        .contains("LUT backend unavailable"));
    struct HashFailure;
    impl ConfigIOProvider for HashFailure {
        fn config_data(&self) -> ocio_rs::Result<Cow<'_, str>> {
            Ok(Cow::Borrowed(""))
        }
        fn lut_data<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(b"data"))
        }
        fn fast_lut_file_hash<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Err(ocio_rs::OcioError::InvalidInput(
                "hash backend unavailable".into(),
            ))
        }
    }
    let proxy = ConfigIOProxy::from_provider(HashFailure).unwrap();
    assert!(proxy
        .try_fast_lut_file_hash_bytes("test.clf")
        .unwrap_err()
        .to_string()
        .contains("hash backend unavailable"));
    assert!(proxy
        .try_copy_lut_data("test.clf", &mut [0; 8])
        .unwrap_err()
        .to_string()
        .contains("hash backend unavailable"));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn custom_provider_accepts_owned_dynamic_results() {
    struct Owned;
    impl ConfigIOProvider for Owned {
        fn config_data(&self) -> ocio_rs::Result<Cow<'_, str>> {
            Ok(Cow::Owned(format!("config-{}", 42)))
        }
        fn lut_data<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Owned(vec![1, 0, 255]))
        }
        fn fast_lut_file_hash<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Owned(format!("hash-{}", 42).into_bytes()))
        }
    }
    let _guard = config_io_proxy_test_lock();
    let proxy = ConfigIOProxy::from_provider(Owned).unwrap();
    assert_eq!(
        proxy.try_config_data().unwrap().as_deref(),
        Some("config-42")
    );
    assert_eq!(
        proxy.try_fast_lut_file_hash("x").unwrap().as_deref(),
        Some("hash-42")
    );
    assert_eq!(proxy.try_lut_data("x").unwrap(), Some(vec![1, 0, 255]));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn lut_hash_preserves_binary_bytes_for_memory_and_custom_providers() {
    struct Binary;
    impl ConfigIOProvider for Binary {
        fn config_data(&self) -> ocio_rs::Result<Cow<'_, str>> {
            Ok(Cow::Borrowed(""))
        }
        fn lut_data<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(b"lut"))
        }
        fn fast_lut_file_hash<'a>(&'a self, _: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            Ok(Cow::Borrowed(b"\0\x80\xffx"))
        }
    }
    let _guard = config_io_proxy_test_lock();
    let memory = ConfigIOProxy::create().unwrap();
    memory
        .set_lut_data_with_hash_bytes("asset.clf", b"lut", b"\0\x80\xffx")
        .unwrap();
    let provider = ConfigIOProxy::from_provider(Binary).unwrap();
    for proxy in [&memory, &provider] {
        assert_eq!(
            proxy
                .try_fast_lut_file_hash_bytes("asset.clf")
                .unwrap()
                .as_deref(),
            Some(&b"\0\x80\xffx"[..])
        );
        assert_eq!(
            proxy.try_lut_data("asset.clf").unwrap().as_deref(),
            Some(&b"lut"[..])
        );
        assert_eq!(
            proxy.try_fast_lut_file_hash("asset.clf").unwrap().unwrap(),
            "\0\u{fffd}\u{fffd}x"
        );
    }
    assert_eq!(
        memory.try_fast_lut_file_hash_bytes("missing").unwrap(),
        None
    );
    struct RawPath;
    impl ConfigIOProvider for RawPath {
        fn config_data(&self) -> ocio_rs::Result<Cow<'_, str>> {
            Ok(Cow::Borrowed(""))
        }
        fn lut_data<'a>(&'a self, path: &std::ffi::CStr) -> ocio_rs::Result<Cow<'a, [u8]>> {
            assert_eq!(path.to_bytes(), b"lut-\xff");
            Ok(Cow::Borrowed(b"value"))
        }
        fn fast_lut_file_hash<'a>(
            &'a self,
            path: &std::ffi::CStr,
        ) -> ocio_rs::Result<Cow<'a, [u8]>> {
            assert_eq!(path.to_bytes(), b"lut-\xff");
            Ok(Cow::Borrowed(b"hash"))
        }
    }
    let raw = ConfigIOProxy::from_provider(RawPath).unwrap();
    let path = c"lut-\xff";
    assert_eq!(
        raw.try_lut_data_cstr(path).unwrap().as_deref(),
        Some(&b"value"[..])
    );
    assert_eq!(
        raw.try_fast_lut_file_hash_bytes_cstr(path)
            .unwrap()
            .as_deref(),
        Some(&b"hash"[..])
    );
}

#[test]
#[cfg(ocio_stub)]
fn custom_provider_stub_releases_ownership() {
    let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let provider = OnDemandProvider {
        config: String::new(),
        lut: Vec::new(),
        calls: calls.clone(),
    };
    assert!(ConfigIOProxy::from_provider(provider).is_err());
    assert_eq!(std::sync::Arc::strong_count(&calls), 1);
}

fn config_io_proxy_test_lock() -> MutexGuard<'static, ()> {
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

fn populate_context_test_proxy(proxy: &ConfigIOProxy) {
    let config_text =
        fs::read_to_string(test_data_path("configs/context_test1/config.ocio")).expect("config");
    proxy
        .set_config_data(&config_text)
        .expect("set proxy config data");

    let virtual_root = "E:/virtual/context";
    let asset_mappings = [
        ("configs/context_test1/lut1.clf", "lut1.clf", "lut1-hash"),
        ("configs/context_test1/looks.cdl", "looks.cdl", "looks-hash"),
        (
            "configs/context_test1/shot1/lut1.clf",
            "shot1/lut1.clf",
            "shot1-lut1-hash",
        ),
        (
            "configs/context_test1/shot2/lut1.clf",
            "shot2/lut1.clf",
            "shot2-lut1-hash",
        ),
        (
            "configs/context_test1/shot2/lut2.clf",
            "shot2/lut2.clf",
            "shot2-lut2-hash",
        ),
        (
            "configs/context_test1/shot3/lut1.clf",
            "shot3/lut1.clf",
            "shot3-lut1-hash",
        ),
        (
            "configs/context_test1/shot3/subdir/lut3.clf",
            "shot3/subdir/lut3.clf",
            "shot3-lut3-hash",
        ),
        (
            "configs/context_test1/shot4/lut1.clf",
            "shot4/lut1.clf",
            "shot4-lut1-hash",
        ),
        (
            "configs/context_test1/shot4/lut4.clf",
            "shot4/lut4.clf",
            "shot4-lut4-hash",
        ),
    ];

    for (source_rel, virtual_rel, hash) in asset_mappings {
        let payload = fs::read(test_data_path(source_rel)).expect("read proxy asset");
        let virtual_path = format!("{virtual_root}/{virtual_rel}");
        assert!(
            proxy
                .set_lut_data(&virtual_path, &payload, hash)
                .expect("set proxy lut data"),
            "expected proxy to accept {virtual_path}"
        );

        let dotted_virtual_path = format!("{virtual_root}/./{virtual_rel}");
        assert!(
            proxy
                .set_lut_data(&dotted_virtual_path, &payload, hash)
                .expect("set dotted proxy lut data"),
            "expected proxy to accept {dotted_virtual_path}"
        );
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_io_proxy_payload_and_attachment_round_trip_behavior() {
    let _guard = config_io_proxy_test_lock();
    if is_stub() {
        return;
    }

    let proxy = ConfigIOProxy::create().expect("config io proxy");
    proxy
        .set_config_data(
            "ocio_profile_version: 2\nroles:\n  default: raw\ncolorspaces:\n  - !<ColorSpace> {name: raw, isdata: true}\n",
        )
        .expect("set config data");
    assert_eq!(
        proxy
            .try_config_data()
            .expect("proxy config-data query")
            .as_deref(),
        Some(
            "ocio_profile_version: 2\nroles:\n  default: raw\ncolorspaces:\n  - !<ColorSpace> {name: raw, isdata: true}\n",
        )
    );

    assert!(proxy
        .set_lut_data("E:/virtual/context/empty.spi1d", &[], "empty-hash")
        .expect("set empty lut data"));
    assert_eq!(
        proxy
            .try_fast_lut_file_hash("E:/virtual/context/empty.spi1d")
            .expect("proxy LUT hash query")
            .as_deref(),
        Some("empty-hash")
    );
    assert_eq!(
        proxy.lut_data("E:/virtual/context/empty.spi1d").as_deref(),
        Some([].as_slice())
    );
    assert_eq!(proxy.lut_data("E:/virtual/context/missing.spi1d"), None);
    assert!(proxy.try_fast_lut_file_hash("bad\0path").is_err());
    let path = c"E:/virtual/context/empty.spi1d";
    assert_eq!(proxy.try_lut_data_cstr(path).unwrap(), Some(Vec::new()));
    assert!(proxy.try_copy_lut_data_cstr(path, &mut []).unwrap());
    assert_eq!(proxy.try_get_lut_data_size_cstr(path).unwrap(), 0);
    assert!(proxy.try_has_lut_data_cstr(path).unwrap());
    assert_eq!(
        proxy
            .try_fast_lut_file_hash_bytes_cstr(path)
            .unwrap()
            .as_deref(),
        Some(&b"empty-hash"[..])
    );
    let missing = c"E:/virtual/context/missing.spi1d";
    assert!(proxy.try_lut_data_cstr(missing).unwrap().is_none());
    assert!(!proxy.try_copy_lut_data_cstr(missing, &mut []).unwrap());
    assert!(!proxy.try_has_lut_data_cstr(missing).unwrap());
    assert!(proxy
        .try_fast_lut_file_hash_bytes_cstr(missing)
        .unwrap()
        .is_none());
    assert!(proxy
        .set_lut_data_with_hash_bytes("E:/virtual/context/binary.spi1d", b"abc", b"a\0b")
        .unwrap());
    let mut bytes = [0; 3];
    assert!(proxy
        .try_copy_lut_data_cstr(c"E:/virtual/context/binary.spi1d", &mut bytes)
        .unwrap());
    assert_eq!(&bytes, b"abc");
    assert_eq!(
        proxy
            .try_fast_lut_file_hash_bytes_cstr(c"E:/virtual/context/binary.spi1d")
            .unwrap()
            .as_deref(),
        Some(&b"a\0b"[..])
    );

    let config = Config::raw().expect("raw config");
    config
        .set_config_io_proxy_object(&proxy)
        .expect("attach config proxy");
    let config_proxy = config
        .config_io_proxy_object()
        .expect("config proxy object");
    assert!(config
        .try_config_io_proxy_object()
        .expect("config proxy query")
        .is_some());
    assert_eq!(config_proxy.config_data(), proxy.config_data());
    assert_eq!(
        config_proxy.fast_lut_file_hash("E:/virtual/context/empty.spi1d"),
        proxy.fast_lut_file_hash("E:/virtual/context/empty.spi1d")
    );

    let context = Context::create().expect("context create");
    context
        .set_config_io_proxy_object(&proxy)
        .expect("attach context proxy");
    let context_proxy = context
        .config_io_proxy_object()
        .expect("context proxy object");
    assert!(context
        .try_config_io_proxy_object()
        .expect("context proxy query")
        .is_some());
    assert_eq!(context_proxy.config_data(), proxy.config_data());
    assert_eq!(
        context_proxy.lut_data("E:/virtual/context/empty.spi1d"),
        proxy.lut_data("E:/virtual/context/empty.spi1d")
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_io_proxy_embedded_context_config_processing_behavior() {
    let _guard = config_io_proxy_test_lock();
    if is_stub() {
        return;
    }

    let proxy = ConfigIOProxy::create().expect("config io proxy");
    populate_context_test_proxy(&proxy);

    let config = Config::from_config_io_proxy(&proxy).expect("config from proxy");
    config
        .set_working_dir("E:/virtual/context")
        .expect("set working dir");

    let plain_cpu = config
        .processor("plain_lut1_cs", "reference")
        .expect("plain processor")
        .optimized_cpu_processor(0)
        .expect("plain cpu");
    let shot1_cpu = config
        .processor("shot1_lut1_cs", "reference")
        .expect("shot1 processor")
        .optimized_cpu_processor(0)
        .expect("shot1 cpu");
    let shot_cpu = config
        .processor("SHOT_lut1_cs", "reference")
        .expect("shot processor")
        .optimized_cpu_processor(0)
        .expect("shot cpu");
    let lut_path_cpu = config
        .processor("lut_path_cs", "reference")
        .expect("lut_path processor")
        .optimized_cpu_processor(0)
        .expect("lut_path cpu");
    let camera_cpu = config
        .processor("context_camera", "reference")
        .expect("camera processor")
        .optimized_cpu_processor(0)
        .expect("camera cpu");

    let original = [1.0f32, 1.0, 1.0, 1.0];
    let mut plain_pixel = original;
    let mut shot1_pixel = original;
    let mut shot_pixel = original;
    let mut lut_path_pixel = original;
    let mut camera_pixel = original;

    plain_cpu.apply_rgba(&mut plain_pixel);
    shot1_cpu.apply_rgba(&mut shot1_pixel);
    shot_cpu.apply_rgba(&mut shot_pixel);
    lut_path_cpu.apply_rgba(&mut lut_path_pixel);
    camera_cpu.apply_rgba(&mut camera_pixel);

    // For a bare "lut1.clf" source, OCIO searches configured search paths
    // before falling back to the working directory. In this config the first
    // hit comes from ./$SHOT -> shot4/lut1.clf.
    assert_close(plain_pixel[0] as f64, 40.0, 1e-6);
    assert_close(plain_pixel[1] as f64, 40.0, 1e-6);
    assert_close(plain_pixel[2] as f64, 40.0, 1e-6);
    assert_close(shot1_pixel[0] as f64, 10.0, 1e-6);
    assert_close(shot1_pixel[1] as f64, 10.0, 1e-6);
    assert_close(shot1_pixel[2] as f64, 10.0, 1e-6);
    assert_close(shot_pixel[0] as f64, 40.0, 1e-6);
    assert_close(shot_pixel[1] as f64, 40.0, 1e-6);
    assert_close(shot_pixel[2] as f64, 40.0, 1e-6);
    assert_close(lut_path_pixel[0] as f64, 30.0, 1e-6);
    assert_close(lut_path_pixel[1] as f64, 30.0, 1e-6);
    assert_close(lut_path_pixel[2] as f64, 30.0, 1e-6);
    assert_close(camera_pixel[0] as f64, 3.0, 1e-6);
    assert_close(camera_pixel[1] as f64, 3.0, 1e-6);
    assert_close(camera_pixel[2] as f64, 3.0, 1e-6);
    assert_close(plain_pixel[3] as f64, 1.0, 1e-6);
    assert_close(shot1_pixel[3] as f64, 1.0, 1e-6);
    assert_close(shot_pixel[3] as f64, 1.0, 1e-6);
    assert_close(lut_path_pixel[3] as f64, 1.0, 1e-6);
    assert_close(camera_pixel[3] as f64, 1.0, 1e-6);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_io_proxy_handle_survives_parent_drop_behavior() {
    let _guard = config_io_proxy_test_lock();
    if is_stub() {
        return;
    }

    let proxy = ConfigIOProxy::create().expect("config io proxy");
    proxy
        .set_config_data(
            "ocio_profile_version: 2\nroles:\n  default: raw\ncolorspaces:\n  - !<ColorSpace> {name: raw, isdata: true}\n",
        )
        .expect("set config data");
    proxy
        .set_lut_data("E:/virtual/context/empty.spi1d", &[], "empty-hash")
        .expect("set lut data");

    let config_proxy = {
        let config = Config::raw().expect("raw config");
        config
            .set_config_io_proxy_object(&proxy)
            .expect("attach config proxy");
        config
            .config_io_proxy_object()
            .expect("config proxy object")
    };
    assert_eq!(config_proxy.config_data(), proxy.config_data());
    assert_eq!(
        config_proxy.fast_lut_file_hash("E:/virtual/context/empty.spi1d"),
        Some(String::from("empty-hash"))
    );

    let context_proxy = {
        let context = Context::create().expect("context create");
        context
            .set_config_io_proxy_object(&proxy)
            .expect("attach context proxy");
        context
            .config_io_proxy_object()
            .expect("context proxy object")
    };
    assert_eq!(context_proxy.config_data(), proxy.config_data());
    assert_eq!(
        context_proxy.lut_data("E:/virtual/context/empty.spi1d"),
        Some(Vec::new())
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
#[allow(deprecated)]
fn raw_config_io_proxy_handles_are_owned_and_destroyable_behavior() {
    let _guard = config_io_proxy_test_lock();
    if is_stub() {
        return;
    }

    let proxy = ConfigIOProxy::create().expect("config io proxy");
    proxy
        .set_config_data(
            "ocio_profile_version: 2\nroles:\n  default: raw\ncolorspaces:\n  - !<ColorSpace> {name: raw, isdata: true}\n",
        )
        .expect("set config data");

    let config = Config::raw().expect("raw config");
    config
        .set_config_io_proxy_object(&proxy)
        .expect("attach config proxy");
    let config_raw = config.config_io_proxy();
    assert!(!config_raw.is_null(), "config raw proxy handle");
    unsafe { ocio_sys::ocio_config_io_proxy_destroy(config_raw) };
    assert!(config
        .try_config_io_proxy_object()
        .expect("config proxy remains attached")
        .is_some());

    let context = Context::create().expect("context create");
    context
        .set_config_io_proxy_object(&proxy)
        .expect("attach context proxy");
    let context_raw = context.config_io_proxy();
    assert!(!context_raw.is_null(), "context raw proxy handle");
    unsafe { ocio_sys::ocio_config_io_proxy_destroy(context_raw) };
    assert!(context
        .try_config_io_proxy_object()
        .expect("context proxy remains attached")
        .is_some());
}
