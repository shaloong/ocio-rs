use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=fixture.cpp");
    println!("cargo:rerun-if-env-changed=OCIO_INSTALL_DIR");
    let include = PathBuf::from(env::var_os("OCIO_INSTALL_DIR").expect("set OCIO_INSTALL_DIR"))
        .join("include");
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("fixture.cpp")
        .include(include)
        .flag_if_supported("/std:c++17")
        .flag_if_supported("-std=c++17");
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        build.define("OpenColorIO_SKIP_IMPORTS", None);
    }
    build.compile("ocio_creator_fixture");
}
