use ocio_rs::{Config, Result};

fn main() -> Result<()> {
    let config = Config::raw()?;
    let processor = config.processor("raw", "raw")?;

    let mut pixel = [0.5, 0.25, 0.125, 1.0];
    processor.apply_rgba(&mut pixel)?;

    println!("Converted pixel: {pixel:?}");
    Ok(())
}
