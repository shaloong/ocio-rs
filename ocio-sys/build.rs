use std::env;
use std::path::PathBuf;

fn main() {
    let mut include_paths = Vec::<PathBuf>::new();
    let mut link_paths = Vec::<PathBuf>::new();
    let mut has_real_ocio = false;

    // Real OCIO is enabled when:
    // 1. OCIO_RS_ENABLE_REAL=1 is explicitly set (manual override), OR
    // 2. The "bundled" feature is active and OCIO source is available
    let is_bundled = env::var_os("CARGO_FEATURE_BUNDLED").is_some();
    let enable_real_ocio = env_flag("OCIO_RS_ENABLE_REAL") || is_bundled;

    if !enable_real_ocio {
        println!("cargo:warning=OCIO_RS_ENABLE_REAL is not set; building ocio-sys in stub mode.");
        println!("cargo:warning=Enable the 'bundled' feature or set OCIO_RS_ENABLE_REAL=1 for real OCIO.");
    } else if let Some(dir) = env::var_os("OCIO_INSTALL_DIR") {
        let dir = PathBuf::from(dir);
        include_paths.push(dir.join("include"));
        link_paths.push(dir.join("lib"));
        link_paths.push(dir.join("lib64"));
        // Transitive deps may also be installed alongside OCIO
        for ext_candidate in &[
            dir.join("ext").join("dist"),
            dir.join("share").join("ocio").join("ext"),
        ] {
            let lib_dir = ext_candidate.join("lib");
            if lib_dir.exists() {
                link_paths.push(lib_dir);
            }
            let inc_dir = ext_candidate.join("include");
            if inc_dir.exists() {
                include_paths.push(inc_dir);
            }
        }
        has_real_ocio = true;
    } else if is_bundled {
        if let Some(ocio_source) = resolve_ocio_source_dir() {
            let dst = std::panic::catch_unwind(|| {
                cmake::Config::new(&ocio_source)
                    .define("BUILD_SHARED_LIBS", "OFF")
                    .define("OCIO_BUILD_APPS", "OFF")
                    .define("OCIO_BUILD_OPENFX", "OFF")
                    .define("OCIO_BUILD_NUKE", "OFF")
                    .define("OCIO_BUILD_PYTHON", "OFF")
                    .define("OCIO_BUILD_JAVA", "OFF")
                    .define("OCIO_BUILD_DOCS", "OFF")
                    .define("OCIO_BUILD_TESTS", "OFF")
                    .define("OCIO_BUILD_GPU_TESTS", "OFF")
                    .define("OCIO_INSTALL_EXT_PACKAGES", "ALL")
                    .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
                    .build()
            });

            match dst {
                Ok(dst) => {
                    include_paths.push(dst.join("include"));
                    link_paths.push(dst.join("lib"));
                    link_paths.push(dst.join("lib64"));

                    // OCIO_INSTALL_EXT_PACKAGES=ALL builds transitive deps
                    // under <build_dir>/ext/dist. The cmake crate uses
                    // dst/build/ as the build directory.
                    for ext_candidate in &[
                        dst.join("build").join("ext").join("dist"),
                        dst.join("ext").join("dist"),
                    ] {
                        let lib_dir = ext_candidate.join("lib");
                        if lib_dir.exists() {
                            link_paths.push(lib_dir);
                        }
                        let inc_dir = ext_candidate.join("include");
                        if inc_dir.exists() {
                            include_paths.push(inc_dir);
                        }
                    }

                    has_real_ocio = true;
                }
                Err(e) => {
                    let msg = e.downcast_ref::<String>()
                        .map(|s| s.as_str())
                        .or_else(|| e.downcast_ref::<&str>().copied())
                        .unwrap_or("unknown error");
                    println!("cargo:warning=OpenColorIO bundled build failed: {msg}");
                    println!("cargo:warning=Falling back to stub mode.");
                }
            }
        } else {
            println!("cargo:warning=OpenColorIO source not found. Building in stub mode.");
        }
    } else {
        println!(
            "cargo:warning=Real OCIO requested but bundled source is disabled. Building stub mode."
        );
    }

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("src/bridge.cpp")
        .flag_if_supported("/std:c++17")
        .flag_if_supported("-std=c++17");

    if cfg!(target_os = "windows") {
        build.define("OpenColorIO_SKIP_IMPORTS", None);
        // Enable asynchronous exception handling so the bridge can catch
        // structured exceptions (access violations, etc.) that OCIO's
        // C++ code might trigger on invalid input paths.
        build.flag_if_supported("/EHa");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=oleaut32");
        println!("cargo:rustc-link-lib=uuid");
        println!("cargo:rustc-link-lib=comdlg32");
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=winspool");
    }

    if !has_real_ocio {
        build.define("OCIO_RS_STUB", None);
    }

    // MSVC standard headers (e.g. <stddef.h>) may not be found when cc-rs
    // runs outside a Visual Studio Developer Command Prompt (common with
    // Git Bash / MSYS2).  Add the toolchain include dirs explicitly so the
    // bridge can compile regardless of the shell that launched Cargo.
    if cfg!(target_os = "windows") && has_real_ocio {
        if let Some(msvc_include) = find_msvc_include() {
            build.include(msvc_include);
        }
        for sdk_dir in find_windows_sdk_includes() {
            build.include(sdk_dir);
        }
    }

    build.include("src");
    for include in &include_paths {
        if include.exists() {
            build.include(include);
        }
    }

    build.compile("ocio_sys_bridge");

    if has_real_ocio {
        for lib_dir in &link_paths {
            if lib_dir.exists() {
                println!("cargo:rustc-link-search=native={}", lib_dir.display());
            }
        }

        // Primary OCIO library
        println!("cargo:rustc-link-lib=static=OpenColorIO");

        // Static OpenColorIO does not embed its mandatory dependencies:
        //   expat, yaml-cpp, Imath, pystring, minizip-ng, ZLIB
        // They must be linked into the final binary explicitly.
        //
        // When OCIO_INSTALL_EXT_PACKAGES=ALL was used these libraries
        // reside alongside OpenColorIO in the ext/dist/lib directory.
        //
        // Library file names vary by platform and build type; we emit the
        // most common name for each. If a particular name is not found,
        // the linker will report which library is missing.
        link_transitive_deps();

        if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-lib=stdc++");
            println!("cargo:rustc-link-lib=dl");
            println!("cargo:rustc-link-lib=pthread");
        }

        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-lib=c++");
        }
    }

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/bridge.hpp");
    println!("cargo:rerun-if-changed=src/bridge.cpp");
    println!("cargo:rerun-if-env-changed=OCIO_SOURCE_DIR");
    println!("cargo:rerun-if-env-changed=OCIO_INSTALL_DIR");
    println!("cargo:rerun-if-env-changed=OCIO_RS_ENABLE_REAL");
}

fn link_transitive_deps() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=static=libexpatMD");
        println!("cargo:rustc-link-lib=static=yaml-cpp");
        println!("cargo:rustc-link-lib=static=Imath-3_2");
        println!("cargo:rustc-link-lib=static=pystring");
        println!("cargo:rustc-link-lib=static=minizip-ng");
        println!("cargo:rustc-link-lib=static=zlibstatic");
    }
    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-lib=static=expat");
        println!("cargo:rustc-link-lib=static=yaml-cpp");
        println!("cargo:rustc-link-lib=static=Imath-3_2");
        println!("cargo:rustc-link-lib=static=pystring");
        println!("cargo:rustc-link-lib=static=minizip-ng");
        println!("cargo:rustc-link-lib=static=z");
    }
}

fn env_flag(name: &str) -> bool {
    match env::var(name) {
        Ok(value) => {
            let value = value.trim();
            value == "1" || value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("yes")
        }
        Err(_) => false,
    }
}

fn resolve_ocio_source_dir() -> Option<PathBuf> {
    if let Some(dir) = env::var_os("OCIO_SOURCE_DIR") {
        let path = PathBuf::from(dir);
        if path.join("CMakeLists.txt").exists() {
            return Some(path);
        }
    }

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from)?;
    let candidate = manifest_dir
        .parent()?
        .join("third_party")
        .join("OpenColorIO");
    if candidate.join("CMakeLists.txt").exists() {
        return Some(candidate);
    }

    None
}

fn find_msvc_include() -> Option<PathBuf> {
    // Probe well-known VS 2022 / 2019 MSVC include directories.
    let base = std::path::Path::new("C:/Program Files (x86)/Microsoft Visual Studio");
    for year in &["2022", "2019"] {
        let vc_dir = base.join(year).join("BuildTools").join("VC").join("Tools").join("MSVC");
        if let Ok(entries) = std::fs::read_dir(&vc_dir) {
            for entry in entries.flatten() {
                let candidate = entry.path().join("include");
                if candidate.join("vcruntime.h").exists() {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

fn find_windows_sdk_includes() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let kits = std::path::Path::new("C:/Program Files (x86)/Windows Kits/10/Include");
    if let Ok(versions) = std::fs::read_dir(kits) {
        // Pick the latest SDK version.
        let mut versions: Vec<_> = versions.flatten().collect();
        versions.sort_by(|a, b| b.file_name().cmp(&a.file_name()));
        for entry in versions {
            let base = entry.path();
            for sub in &["ucrt", "shared", "um", "winrt"] {
                let p = base.join(sub);
                if p.exists() {
                    dirs.push(p);
                }
            }
            if !dirs.is_empty() {
                break;
            }
        }
    }
    dirs
}
