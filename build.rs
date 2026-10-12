//! Re-exports `ocio-sys`'s stub state as a cfg for this crate and its tests.
//!
//! `cargo:rustc-cfg` only applies to the crate whose build script emitted it,
//! so `ocio-sys` publishes `cargo:stub=1` through its `links = "OpenColorIO"`
//! metadata and this script turns it back into `cfg(ocio_stub)`.

fn main() {
    println!("cargo:rustc-check-cfg=cfg(ocio_stub)");
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var_os("DEP_OPENCOLORIO_STUB").is_some() {
        println!("cargo:rustc-cfg=ocio_stub");
    }
}
