use std::env;
use std::fs;
use std::path::PathBuf;

pub const BASELINE_REQUIREMENT: &str = ">= 2.4.1, < 2.6";
pub const V2_5_REQUIREMENT: &str = ">= 2.5.1, < 2.6";
const BASELINE_API_HEX: u32 = 0x02040100;
const V2_5_API_HEX: u32 = 0x02050100;

pub fn prepare() -> PathBuf {
    assert_cargo_metadata_matches();

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo must provide OUT_DIR"));
    let header = out_dir.join("ocio_rs_compatibility.h");
    let contents = format!(
        "#pragma once\n\
         \n\
         #ifdef OCIO_RS_FEATURE_V2_5\n\
         #define OCIO_RS_INTERFACE_API_HEX 0x{V2_5_API_HEX:08X}\n\
         #else\n\
         #define OCIO_RS_INTERFACE_API_HEX 0x{BASELINE_API_HEX:08X}\n\
         #endif\n\
         \n\
         #ifdef OCIO_RS_STUB\n\
         #define OCIO_RS_NATIVE_API_HEX OCIO_RS_INTERFACE_API_HEX\n\
         #else\n\
         #define OCIO_RS_NATIVE_API_HEX OCIO_VERSION_HEX\n\
         #if OCIO_VERSION_HEX < 0x{BASELINE_API_HEX:08X}\n\
         #error \"ocio-rs requires OpenColorIO 2.4.1 or newer\"\n\
         #endif\n\
         #if defined(OCIO_RS_FEATURE_V2_5) && OCIO_VERSION_HEX < 0x{V2_5_API_HEX:08X}\n\
         #error \"ocio-rs feature v2_5 requires OpenColorIO 2.5.1 or newer\"\n\
         #endif\n\
         #endif\n"
    );
    fs::write(&header, contents).expect("failed to write the OCIO compatibility header");
    out_dir
}

fn assert_cargo_metadata_matches() {
    let manifest = include_str!("../Cargo.toml");
    for requirement in [BASELINE_REQUIREMENT, V2_5_REQUIREMENT] {
        let declaration = format!("version = \"{requirement}\"");
        assert!(
            manifest.contains(&declaration),
            "ocio-sys/Cargo.toml must contain `{declaration}`; update build_support/compatibility.rs and Cargo metadata together"
        );
    }
}
