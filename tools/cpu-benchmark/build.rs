use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=baseline.cpp");
    println!("cargo:rerun-if-env-changed=OCIO_BENCH_INCLUDE_DIR");
    let include = PathBuf::from(
        env::var_os("OCIO_BENCH_INCLUDE_DIR")
            .expect("set OCIO_BENCH_INCLUDE_DIR to an installed OCIO 2.6 include directory"),
    );
    assert!(
        include.join("OpenColorIO/OpenColorABI.h").is_file(),
        "OCIO_BENCH_INCLUDE_DIR must contain OpenColorIO/OpenColorABI.h"
    );
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("baseline.cpp")
        .include(include)
        .flag_if_supported("/std:c++17")
        .flag_if_supported("-std=c++17");
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        build.define("OpenColorIO_SKIP_IMPORTS", None);
    }
    build.compile("ocio_direct_baseline");
}
