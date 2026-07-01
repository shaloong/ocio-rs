//! Dynamic property behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the API-shape smoke tests in the
//! unit suite. In bundled/real mode they validate that dynamic properties are
//! exposed through both `Processor` and `CPUProcessor`, and that mutating them
//! affects CPU processing results.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::ExposureContrastTransform;
use ocio_rs::{DynamicPropertyType, ExposureContrastStyle, TransformDirection};

fn dynamic_property_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn dynamic_exposure_processor() -> Option<ocio_rs::Processor> {
    let config = create_test_config()?;
    let transform = ExposureContrastTransform::create().ok()?;
    transform.set_style(ExposureContrastStyle::Linear);
    transform.set_exposure(0.0);
    transform.set_contrast(1.0);
    transform.set_gamma(1.0);
    transform.make_exposure_dynamic();

    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()?;
    Some(processor)
}

#[test]
fn dynamic_exposure_processor_property_seeds_cpu_behavior() {
    let _guard = dynamic_property_test_lock();
    if is_stub() {
        return;
    }

    let processor = dynamic_exposure_processor().expect("dynamic exposure processor");

    assert!(processor.is_dynamic());
    assert!(processor.has_dynamic_property_kind(DynamicPropertyType::Exposure));

    let processor_prop = processor
        .dynamic_property(DynamicPropertyType::Exposure)
        .expect("processor dynamic property");

    assert_eq!(
        processor_prop.property_type(),
        DynamicPropertyType::Exposure
    );
    assert_close(processor_prop.double_value(), 0.0, 1e-8);
    processor_prop.set_double_value(1.0);
    assert_close(processor_prop.double_value(), 1.0, 1e-8);

    let cpu = processor
        .default_cpu_processor()
        .expect("default cpu processor");
    assert!(cpu.is_dynamic());
    assert!(cpu.has_dynamic_property_kind(DynamicPropertyType::Exposure));

    let cpu_prop = cpu
        .dynamic_property(DynamicPropertyType::Exposure)
        .expect("cpu dynamic property");
    assert_eq!(cpu_prop.property_type(), DynamicPropertyType::Exposure);
    assert_close(cpu_prop.double_value(), 1.0, 1e-8);

    let mut pixel = [0.25f32, 0.5, 0.125, 1.0];
    cpu.apply_rgba(&mut pixel);
    assert_close(pixel[0] as f64, 0.5, 1e-6);
    assert_close(pixel[1] as f64, 1.0, 1e-6);
    assert_close(pixel[2] as f64, 0.25, 1e-6);
    assert_close(pixel[3] as f64, 1.0, 1e-6);
}

#[test]
fn dynamic_exposure_cpu_property_round_trip_and_output_behavior() {
    let _guard = dynamic_property_test_lock();
    if is_stub() {
        return;
    }

    let processor = dynamic_exposure_processor().expect("dynamic exposure processor");
    let cpu = processor
        .default_cpu_processor()
        .expect("default cpu processor");
    let cpu_prop = cpu
        .dynamic_property(DynamicPropertyType::Exposure)
        .expect("cpu dynamic property");

    assert!(cpu.is_dynamic());
    assert!(cpu.has_dynamic_property_kind(DynamicPropertyType::Exposure));
    assert_eq!(cpu_prop.property_type(), DynamicPropertyType::Exposure);
    assert_close(cpu_prop.double_value(), 0.0, 1e-8);

    cpu_prop.set_double_value(-1.0);
    assert_close(cpu_prop.double_value(), -1.0, 1e-8);

    let input = [0.25f32, 0.5, 0.125, 1.0];

    let mut darkened = input;
    cpu.apply_rgba(&mut darkened);
    assert_close(darkened[0] as f64, 0.125, 1e-6);
    assert_close(darkened[1] as f64, 0.25, 1e-6);
    assert_close(darkened[2] as f64, 0.0625, 1e-6);
    assert_close(darkened[3] as f64, 1.0, 1e-6);
}
