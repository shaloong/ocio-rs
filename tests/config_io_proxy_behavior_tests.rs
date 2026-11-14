//! ConfigIOProxy behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/config_io_proxy.rs`. In bundled/real mode they validate payload round
//! trips, config/context proxy attachment, and full config-driven processing
//! from in-memory config and LUT assets.

mod common;
use common::*;

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
    fn config_data(&self) -> ocio_rs::Result<String> {
        Ok(self.config.clone())
    }
    fn lut_data(&self, path: &std::ffi::CStr) -> ocio_rs::Result<Vec<u8>> {
        self.calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if path.to_bytes().ends_with(b"lut1.clf") {
            Ok(self.lut.clone())
        } else {
            Ok(Vec::new())
        }
    }
    fn fast_lut_file_hash(&self, path: &std::ffi::CStr) -> ocio_rs::Result<String> {
        Ok(if path.to_bytes().ends_with(b"lut1.clf") {
            "on-demand-lut-v1"
        } else {
            ""
        }
        .into())
    }
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
            .try_lut_data("E:/virtual/context/shot1/lut1.clf")
            .unwrap()
            .unwrap(),
        fs::read(test_data_path("configs/context_test1/shot1/lut1.clf")).unwrap()
    );
    assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(proxy.try_lut_data("missing.clf").unwrap(), None);
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
fn custom_provider_errors_and_panics_do_not_cross_ffi() {
    struct Failing(bool);
    impl ConfigIOProvider for Failing {
        fn config_data(&self) -> ocio_rs::Result<String> {
            if self.0 {
                panic!("provider panic");
            }
            Err(ocio_rs::OcioError::InvalidInput("provider error".into()))
        }
        fn lut_data(&self, _: &std::ffi::CStr) -> ocio_rs::Result<Vec<u8>> {
            Ok(Vec::new())
        }
        fn fast_lut_file_hash(&self, _: &std::ffi::CStr) -> ocio_rs::Result<String> {
            Ok(String::new())
        }
    }
    let _guard = config_io_proxy_test_lock();
    for panic in [false, true] {
        let proxy = ConfigIOProxy::from_provider(Failing(panic)).unwrap();
        assert!(proxy.try_config_data().is_err());
        assert!(Config::from_config_io_proxy(&proxy).is_err());
    }
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
