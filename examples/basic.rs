use ocio_rs::{Config, Result};

fn main() -> Result<()> {
    println!("OCIO stub build: {}", ocio_rs::is_stub_build());

    let config = Config::raw()?;

    // Config metadata
    println!("Config name: {:?}", config.name());
    println!("Config major version: {}", config.major_version());
    println!("Config minor version: {}", config.minor_version());
    println!("Config family separator: {:?}", config.family_separator());

    // Color spaces
    let n = config.num_color_spaces();
    println!("Number of color spaces: {n}");
    for i in 0..n.min(5) {
        println!("  ColorSpace[{}]: {:?}", i, config.color_space_name_by_index(i));
    }

    // Displays and views
    println!("Default display: {:?}", config.default_display());
    let nd = config.num_displays();
    println!("Number of displays: {nd}");
    for i in 0..nd {
        let display = config.display(i);
        println!("  Display[{}]: {:?}", i, display);
        if let Some(ref d) = display {
            println!("    Default view: {:?}", config.default_view(d));
            let nv = config.num_views(d);
            for j in 0..nv.min(3) {
                println!("    View[{}]: {:?}", j, config.view(d, j));
            }
        }
    }

    // Looks
    println!("Number of looks: {}", config.num_looks());
    for i in 0..config.num_looks().min(3) {
        println!("  Look[{}]: {:?}", i, config.look_name_by_index(i));
    }

    // Luma coefficients
    println!("Default luma coefs: {:?}", config.default_luma_coefs());

    // Roles
    let nr = config.num_roles();
    println!("Number of roles: {nr}");
    for i in 0..nr {
        let name = config.role_name(i);
        let cs = config.role_color_space_by_index(i);
        println!("  Role[{}]: {:?} = {:?}", i, name, cs);
    }

    // Active displays/views
    println!("Active displays: {:?}", config.active_displays());
    println!("Active views: {:?}", config.active_views());

    // Process a pixel
    let processor = config.processor("raw", "raw")?;
    println!("Processor is no-op: {}", processor.is_no_op());
    println!("Processor cache ID: {:?}", processor.cache_id());

    let mut pixel = [0.5, 0.25, 0.125, 1.0];
    processor.apply_rgba(&mut pixel)?;
    println!("Converted pixel: {pixel:?}");

    // CPU processor
    if let Ok(cpu) = processor.default_cpu_processor() {
        let mut rgb = [0.5, 0.25, 0.125];
        cpu.apply_rgb(&mut rgb);
        println!("CPU applied RGB: {rgb:?}");
        println!("CPU is identity: {}", cpu.is_identity());
    }

    Ok(())
}
