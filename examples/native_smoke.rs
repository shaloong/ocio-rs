//! Verify an installed or bundled backend without external config/LUT files.
//! Run with `cargo run --example native_smoke --features system`.

use ocio_rs::{transform::MatrixTransform, Config, OcioError, Result, TransformDirection};

fn main() -> Result<()> {
    if ocio_rs::is_stub_build() {
        return Err(OcioError::ValidationFailed(
            "native_smoke requires the system or bundled backend".into(),
        ));
    }
    let config = Config::raw()?;
    let transform = MatrixTransform::scale(&[2.0, 1.0, 0.5, 1.0])?;
    let processor = config.processor_from_transform(&transform, TransformDirection::Forward)?;
    let cpu = processor.default_cpu_processor()?;
    let mut pixel = [0.25_f32, 0.5, 0.75, 1.0];
    cpu.try_apply_rgba(&mut pixel)?;
    let expected = [0.5_f32, 0.5, 0.375, 1.0];
    for (actual, expected) in pixel.into_iter().zip(expected) {
        if !actual.is_finite() || (actual - expected).abs() > 1.0e-6 {
            return Err(OcioError::ValidationFailed(format!(
                "native matrix processing returned {pixel:?}"
            )));
        }
    }
    println!(
        "OpenColorIO {:?}: native processing verified",
        ocio_rs::version()
    );
    Ok(())
}
