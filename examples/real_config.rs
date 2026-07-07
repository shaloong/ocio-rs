//! Real-config demonstration for ocio-rs.
//!
//! This example loads the bundled `tests/data/configs/context_test1/config.ocio`
//! sample when a real OCIO implementation is available. In stub mode it exits
//! early after explaining that real file-based processing is unavailable.

use std::path::PathBuf;

use ocio_rs::{Config, Result};

fn sample_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("configs")
        .join("context_test1")
        .join("config.ocio")
}

fn main() -> Result<()> {
    if ocio_rs::is_stub_build() {
        println!("stub build detected");
        println!("this example needs a real OCIO link to load config.ocio");
        return Ok(());
    }

    let config_path = sample_config_path();
    let working_dir = config_path
        .parent()
        .expect("sample config should have a parent directory");

    println!("Loading config: {}", config_path.display());
    let config = Config::from_file(config_path.to_string_lossy())?;
    config.set_working_dir(working_dir.to_string_lossy())?;

    println!("Default display: {:?}", config.default_display());
    println!("Color spaces: {}", config.num_color_spaces());
    println!("Looks: {}", config.num_looks());

    let processor = config.processor("plain_lut1_cs", "reference")?;
    let cpu = processor.default_cpu_processor()?;

    let mut rgba = [1.0f32, 1.0, 1.0, 1.0];
    println!("Input pixel:  {rgba:?}");
    cpu.apply_rgba(&mut rgba);
    println!("Output pixel: {rgba:?}");

    let mut camera_rgba = [1.0f32, 1.0, 1.0, 1.0];
    let camera_cpu = config
        .processor("context_camera", "reference")?
        .default_cpu_processor()?;
    camera_cpu.apply_rgba(&mut camera_rgba);
    println!("Camera pixel: {camera_rgba:?}");

    Ok(())
}
