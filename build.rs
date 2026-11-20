//! Re-exports `ocio-sys`'s backend metadata as cfgs for this crate and its tests.
//!
//! `cargo:rustc-cfg` only applies to the crate whose build script emitted it,
//! so `ocio-sys` publishes `cargo:stub=1` through its `links = "OpenColorIO"`
//! metadata and this script turns it back into `cfg(ocio_stub)`.

fn main() {
    println!("cargo:rustc-check-cfg=cfg(ocio_stub)");
    println!("cargo:rustc-check-cfg=cfg(ocio_vendored_lut_rows_fixed)");
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var_os("DEP_OPENCOLORIO_STUB").is_some() {
        println!("cargo:rustc-cfg=ocio_stub");
    }
    if std::env::var_os("DEP_OPENCOLORIO_VENDORED_LUT_ROWS_FIXED").is_some() {
        println!("cargo:rustc-cfg=ocio_vendored_lut_rows_fixed");
    }
}
