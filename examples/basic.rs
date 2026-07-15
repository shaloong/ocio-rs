//! Basic API demonstration for ocio-rs.
//!
//! This example shows the core config-query and processor workflow using
//! the default stub build. Stub mode returns safe placeholder values and
//! does not perform real color management - it is useful for verifying
//! that the API surface compiles and runs without an OCIO installation.

use ocio_rs::{Config, Result};

fn main() -> Result<()> {
    // Build mode
    // In stub mode this prints `true`; with a real OCIO link it prints
    // `false`. The rest of the example works identically in both modes.
    println!("Build mode - stub: {}", ocio_rs::is_stub_build());
    println!();

    // Config loading
    // `Config::raw()` creates an empty, editable config using OCIO
    // defaults. For a real config you would use `Config::from_file()`
    // or `Config::from_env()`, but those require a working OCIO
    // installation.
    let config = Config::raw()?;

    // Config metadata
    println!("=== Config metadata ===");
    println!("  name:             {:?}", config.name());
    println!("  major version:    {}", config.major_version());
    println!("  minor version:    {}", config.minor_version());
    println!("  family separator: {:?}", config.family_separator());
    println!();

    // Color spaces
    let n = config.num_color_spaces();
    println!("=== Color spaces ({n} total) ===");
    for i in 0..n.min(5) {
        println!("  [{i}] {:?}", config.color_space_name_by_index(i));
    }
    if n > 5 {
        println!("  ... and {} more", n - 5);
    }
    println!();

    // Displays and views
    println!("=== Displays and views ===");
    println!("  default display: {:?}", config.default_display());
    let nd = config.num_displays();
    println!("  display count:   {nd}");
    for i in 0..nd {
        let display = config.display(i);
        println!("  display[{i}]:    {:?}", display);
        if let Some(ref d) = display {
            println!("    default view: {:?}", config.default_view(d));
            let nv = config.num_views(d);
            for j in 0..nv.min(3) {
                println!("    view[{j}]:   {:?}", config.view(d, j));
            }
            if nv > 3 {
                println!("    ... and {} more views", nv - 3);
            }
        }
    }
    println!();

    // Looks
    let nl = config.num_looks();
    println!("=== Looks ({nl} total) ===");
    for i in 0..nl.min(3) {
        println!("  [{i}] {:?}", config.look_name_by_index(i));
    }
    println!();

    // Luma coefficients
    println!("=== Luma coefficients ===");
    println!("  {:?}", config.default_luma_coefs());
    println!();

    // Roles
    let nr = config.num_roles();
    println!("=== Roles ({nr} total) ===");
    for i in 0..nr {
        let name = config.role_name(i);
        let cs = config.role_color_space_by_index(i);
        println!("  [{i}] {:?} = {:?}", name, cs);
    }
    println!();

    // Active displays / views
    println!("=== Active displays / views ===");
    println!("  active displays: {:?}", config.active_displays());
    println!("  active views:    {:?}", config.active_views());
    println!();

    // Processor creation
    // Create a processor for the identity "raw -> raw" path. In stub mode
    // this exercises the processor API without requiring real color-space
    // definitions.
    println!("=== Processor (raw -> raw) ===");
    let processor = config.processor("raw", "raw")?;
    println!("  is no-op:  {}", processor.is_no_op());
    println!("  cache ID:  {:?}", processor.cache_id());

    // Pixel processing
    // Apply the processor to a single RGBA pixel in-place. Because the
    // pipeline is identity in stub mode the values are unchanged.
    let mut rgba = [0.5, 0.25, 0.125, 1.0];
    println!("  input  RGBA: {rgba:?}");
    processor.default_cpu_processor()?.apply_rgba(&mut rgba);
    println!("  output RGBA: {rgba:?}");
    println!();

    // CPUProcessor details
    // `default_cpu_processor()` can also be stored and reused for
    // multiple pixels.
    println!("=== CPUProcessor details ===");
    if let Ok(cpu) = processor.default_cpu_processor() {
        let mut rgb = [0.5, 0.25, 0.125];
        println!("  input  RGB:  {rgb:?}");
        cpu.apply_rgb(&mut rgb);
        println!("  output RGB:  {rgb:?}");
        println!("  is identity: {}", cpu.is_identity());
    }

    Ok(())
}
