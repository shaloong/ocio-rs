use std::env;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use std::collections::hash_map::DefaultHasher;
#[cfg(target_os = "windows")]
use std::hash::{Hash, Hasher};
#[cfg(target_os = "windows")]
use std::path::Path;

fn main() {
    let mut include_paths = Vec::<PathBuf>::new();
    let mut link_paths = Vec::<PathBuf>::new();
    let mut runtime_paths = Vec::<PathBuf>::new();
    let mut has_real_ocio = false;
    let link_mode = LinkMode::from_env();

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
        if link_mode.is_dynamic() {
            push_existing_path(&mut runtime_paths, dir.join("bin"));
            push_existing_path(&mut runtime_paths, dir.join("lib"));
            push_existing_path(&mut runtime_paths, dir.join("lib64"));
        }
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
            if link_mode.is_dynamic() {
                push_existing_path(&mut runtime_paths, ext_candidate.join("bin"));
                push_existing_path(&mut runtime_paths, ext_candidate.join("lib"));
            }
        }
        has_real_ocio = true;
    } else if is_bundled {
        if let Some(ocio_source) = resolve_ocio_source_dir() {
            let dst = std::panic::catch_unwind(|| {
                let mut config = cmake::Config::new(&ocio_source);
                config
                    .profile("Release")
                    .define("BUILD_SHARED_LIBS", link_mode.cmake_build_shared_libs())
                    .define("OCIO_BUILD_APPS", "OFF")
                    .define("OCIO_BUILD_OPENFX", "OFF")
                    .define("OCIO_BUILD_NUKE", "OFF")
                    .define("OCIO_BUILD_PYTHON", "OFF")
                    .define("OCIO_BUILD_JAVA", "OFF")
                    .define("OCIO_BUILD_DOCS", "OFF")
                    .define("OCIO_BUILD_TESTS", "OFF")
                    .define("OCIO_BUILD_GPU_TESTS", "OFF")
                    .define("OCIO_INSTALL_EXT_PACKAGES", "ALL")
                    .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON");

                let mut bundled_out_dir = env::var_os("OUT_DIR")
                    .map(PathBuf::from)
                    .expect("OUT_DIR is always set by Cargo");

                #[cfg(target_os = "windows")]
                {
                    if env::var_os("CMAKE_GENERATOR").is_none() {
                        if let Some(ninja_path) = find_windows_ninja() {
                            let mut extra_path_entries = Vec::new();
                            if let Some(ninja_dir) = ninja_path.parent() {
                                extra_path_entries.push(ninja_dir.as_os_str().to_owned());
                            }
                            if let Some(msvc_cl) = find_msvc_compiler() {
                                config
                                    .env("CC", msvc_cl.as_os_str())
                                    .env("CXX", msvc_cl.as_os_str())
                                    .env("ASM", msvc_cl.as_os_str());
                                if let Some(msvc_dir) = msvc_cl.parent() {
                                    extra_path_entries.push(msvc_dir.as_os_str().to_owned());
                                }
                            }
                            if let Some(rc_path) = find_windows_sdk_tool("rc.exe") {
                                config.define("CMAKE_RC_COMPILER", cmake_path(&rc_path));
                                if let Some(rc_dir) = rc_path.parent() {
                                    extra_path_entries.push(rc_dir.as_os_str().to_owned());
                                }
                            }
                            if let Some(mt_path) = find_windows_sdk_tool("mt.exe") {
                                config.define("CMAKE_MT", cmake_path(&mt_path));
                                if let Some(mt_dir) = mt_path.parent() {
                                    extra_path_entries.push(mt_dir.as_os_str().to_owned());
                                }
                            }
                            if let Some(path_env) = env::var_os("PATH") {
                                let mut path = std::ffi::OsString::new();
                                for entry in extra_path_entries {
                                    if !path.is_empty() {
                                        path.push(";");
                                    }
                                    path.push(entry);
                                }
                                if !path.is_empty() {
                                    path.push(";");
                                }
                                path.push(path_env);
                                config.env("PATH", path);
                            }

                            config
                                .generator("Ninja")
                                .define("CMAKE_MAKE_PROGRAM", cmake_path(&ninja_path));
                        }
                    }

                    let configured_generator = env::var("CMAKE_GENERATOR").ok();
                    let using_ninja = configured_generator
                        .as_deref()
                        .map(|value| value.eq_ignore_ascii_case("ninja"))
                        .unwrap_or(false)
                        || (configured_generator.is_none() && find_windows_ninja().is_some());

                    let generator_tag = if using_ninja {
                        link_mode.cmake_out_dir_tag("cmake-ninja")
                    } else {
                        link_mode.cmake_out_dir_tag("cmake-vs")
                    };
                    bundled_out_dir =
                        bundled_cmake_out_dir(&bundled_out_dir, &ocio_source, &generator_tag);

                    if !using_ninja {
                        // Visual Studio builds of bundled OCIO have shown
                        // intermittent tracked-file log failures when the
                        // generated solution is built with Cargo's default
                        // parallel job count. Force the CMake/MSBuild layer to
                        // serialize project execution for reliability.
                        unsafe {
                            env::set_var("NUM_JOBS", "1");
                            env::set_var("CMAKE_BUILD_PARALLEL_LEVEL", "1");
                        }
                        config.build_arg("/m:1");
                    }
                }

                #[cfg(not(target_os = "windows"))]
                bundled_out_dir.push(link_mode.cmake_out_dir_tag("cmake"));

                config.out_dir(&bundled_out_dir);
                config.build()
            });

            match dst {
                Ok(dst) => {
                    include_paths.push(dst.join("include"));
                    link_paths.push(dst.join("lib"));
                    link_paths.push(dst.join("lib64"));
                    if link_mode.is_dynamic() {
                        push_existing_path(&mut runtime_paths, dst.join("bin"));
                    }

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
                        if link_mode.is_dynamic() {
                            push_existing_path(&mut runtime_paths, ext_candidate.join("bin"));
                            push_existing_path(&mut runtime_paths, ext_candidate.join("lib"));
                        }
                    }

                    if link_mode.is_dynamic() {
                        collect_runtime_dll_dirs(
                            &mut runtime_paths,
                            &dst.join("build").join("ext").join("build"),
                        );
                    }

                    has_real_ocio = true;
                }
                Err(e) => {
                    let msg = e
                        .downcast_ref::<String>()
                        .map(|s| s.as_str())
                        .or_else(|| e.downcast_ref::<&str>().copied())
                        .unwrap_or("unknown error");
                    panic!("OpenColorIO bundled build failed: {msg}");
                }
            }
        } else {
            panic!(
                "OpenColorIO source not found. Use a recursive checkout or set OCIO_SOURCE_DIR."
            );
        }
    } else {
        panic!("Real OCIO requested but no OCIO_INSTALL_DIR was provided and bundled source is disabled.");
    }

    if cfg!(target_os = "windows") && is_bundled && link_mode.is_dynamic() {
        copy_runtime_dlls_to_cargo_target_dirs(&runtime_paths);
    }

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("src/bridge.cpp")
        .flag_if_supported("/std:c++17")
        .flag_if_supported("-std=c++17");

    if cfg!(target_os = "windows") {
        if link_mode.is_static() {
            build.define("OpenColorIO_SKIP_IMPORTS", None);
        }
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
        if link_mode.is_dynamic() {
            for runtime_dir in &runtime_paths {
                println!("cargo:rustc-link-search=native={}", runtime_dir.display());
            }
        }

        // Primary OCIO library
        println!(
            "cargo:rustc-link-lib={}={}",
            link_mode.rustc_link_kind(),
            "OpenColorIO"
        );

        if link_mode.is_static() {
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
            link_transitive_deps(&link_paths);
        }

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
    println!("cargo:rerun-if-env-changed=OCIO_RS_LINK");
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LinkMode {
    Static,
    Dynamic,
}

impl LinkMode {
    fn from_env() -> Self {
        match env::var("OCIO_RS_LINK") {
            Ok(value) => {
                let normalized = value.trim().to_ascii_lowercase();
                match normalized.as_str() {
                    "" | "static" => Self::Static,
                    "dynamic" | "dylib" | "shared" => Self::Dynamic,
                    other => panic!(
                        "unsupported OCIO_RS_LINK value '{other}'; expected 'static' or 'dynamic'"
                    ),
                }
            }
            Err(_) => Self::Static,
        }
    }

    fn is_static(self) -> bool {
        matches!(self, Self::Static)
    }

    fn is_dynamic(self) -> bool {
        matches!(self, Self::Dynamic)
    }

    fn cmake_build_shared_libs(self) -> &'static str {
        match self {
            Self::Static => "OFF",
            Self::Dynamic => "ON",
        }
    }

    fn rustc_link_kind(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Dynamic => "dylib",
        }
    }

    fn cmake_out_dir_tag(self, base: &str) -> String {
        let suffix = match self {
            Self::Static => "static",
            Self::Dynamic => "dynamic",
        };
        format!("{base}-{suffix}")
    }
}

fn push_existing_path(paths: &mut Vec<PathBuf>, path: PathBuf) {
    if path.exists() && !paths.iter().any(|existing| existing == &path) {
        paths.push(path);
    }
}

#[cfg(target_os = "windows")]
fn collect_runtime_dll_dirs(paths: &mut Vec<PathBuf>, root: &std::path::Path) {
    if !root.exists() {
        return;
    }

    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };

        let mut has_dll = false;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("dll"))
            {
                has_dll = true;
            }
        }

        if has_dll {
            push_existing_path(paths, dir);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn collect_runtime_dll_dirs(_paths: &mut Vec<PathBuf>, _root: &std::path::Path) {}

#[cfg(target_os = "windows")]
fn copy_runtime_dlls_to_cargo_target_dirs(runtime_paths: &[PathBuf]) {
    let Some(out_dir) = env::var_os("OUT_DIR").map(PathBuf::from) else {
        return;
    };
    let Some(profile_dir) = out_dir.ancestors().nth(3).map(PathBuf::from) else {
        return;
    };

    let mut destination_dirs = vec![profile_dir.clone(), profile_dir.join("deps")];
    destination_dirs.retain(|dir| dir.exists());

    for runtime_dir in runtime_paths {
        let Ok(entries) = std::fs::read_dir(runtime_dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let source = entry.path();
            if !source
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("dll"))
            {
                continue;
            }

            for destination_dir in &destination_dirs {
                let destination = destination_dir.join(entry.file_name());
                if source == destination {
                    continue;
                }
                if let Err(err) = std::fs::copy(&source, &destination) {
                    panic!(
                        "failed to copy runtime DLL '{}' to '{}': {err}",
                        source.display(),
                        destination.display()
                    );
                }
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn copy_runtime_dlls_to_cargo_target_dirs(_runtime_paths: &[PathBuf]) {}

#[cfg(target_os = "windows")]
fn link_transitive_deps(link_paths: &[PathBuf]) {
    link_static_library(link_paths, &["libexpatMD", "expat", "libexpatdMD"]);
    link_static_library(link_paths, &["yaml-cpp", "yaml-cppd"]);
    link_static_library(link_paths, &["Imath-3_2", "Imath-3_2_d"]);
    link_static_library(link_paths, &["pystring"]);
    link_static_library(link_paths, &["minizip-ng"]);
    link_static_library(link_paths, &["zlibstatic", "zlib", "zlibstaticd", "zlibd"]);
}

#[cfg(not(target_os = "windows"))]
fn link_transitive_deps(_link_paths: &[PathBuf]) {
    println!("cargo:rustc-link-lib=static=expat");
    println!("cargo:rustc-link-lib=static=yaml-cpp");
    println!("cargo:rustc-link-lib=static=Imath-3_2");
    println!("cargo:rustc-link-lib=static=pystring");
    println!("cargo:rustc-link-lib=static=minizip-ng");
    println!("cargo:rustc-link-lib=static=z");
}

#[cfg(target_os = "windows")]
fn link_static_library(link_paths: &[PathBuf], candidates: &[&str]) {
    for candidate in candidates {
        let file_name = format!("{candidate}.lib");
        if link_paths.iter().any(|dir| dir.join(&file_name).exists()) {
            println!("cargo:rustc-link-lib=static={candidate}");
            return;
        }
    }

    if let Some(candidate) = candidates.first() {
        println!("cargo:rustc-link-lib=static={candidate}");
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
    let packaged_vendor = manifest_dir.join("vendor").join("OpenColorIO");
    if packaged_vendor.join("CMakeLists.txt").exists() {
        return Some(packaged_vendor);
    }

    let candidate = manifest_dir
        .parent()?
        .join("third_party")
        .join("OpenColorIO");
    if candidate.join("CMakeLists.txt").exists() {
        return Some(candidate);
    }

    None
}

#[cfg(target_os = "windows")]
fn find_windows_ninja() -> Option<PathBuf> {
    let candidates = [
        r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe",
        r"C:\Program Files\Microsoft Visual Studio\2022\BuildTools\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe",
        r"C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe",
        r"C:\Program Files\Microsoft Visual Studio\2022\Professional\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe",
        r"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe",
    ];

    candidates
        .into_iter()
        .map(PathBuf::from)
        .find(|path| path.exists())
}

fn find_msvc_include() -> Option<PathBuf> {
    // Probe well-known VS 2022 / 2019 MSVC include directories.
    let base = std::path::Path::new("C:/Program Files (x86)/Microsoft Visual Studio");
    for year in &["2022", "2019"] {
        let vc_dir = base
            .join(year)
            .join("BuildTools")
            .join("VC")
            .join("Tools")
            .join("MSVC");
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

#[cfg(target_os = "windows")]
fn find_msvc_compiler() -> Option<PathBuf> {
    let base = std::path::Path::new("C:/Program Files (x86)/Microsoft Visual Studio");
    for year in &["2022", "2019"] {
        let vc_dir = base
            .join(year)
            .join("BuildTools")
            .join("VC")
            .join("Tools")
            .join("MSVC");
        if let Ok(entries) = std::fs::read_dir(&vc_dir) {
            let mut entries: Vec<_> = entries.flatten().collect();
            entries.sort_by_key(|entry| std::cmp::Reverse(entry.file_name()));
            for entry in entries {
                let candidate = entry
                    .path()
                    .join("bin")
                    .join("Hostx64")
                    .join("x64")
                    .join("cl.exe");
                if candidate.exists() {
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
        versions.sort_by_key(|entry| std::cmp::Reverse(entry.file_name()));
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

#[cfg(target_os = "windows")]
fn find_windows_sdk_tool(tool_name: &str) -> Option<PathBuf> {
    let kits = std::path::Path::new("C:/Program Files (x86)/Windows Kits/10/bin");
    let mut versions = std::fs::read_dir(kits).ok()?.flatten().collect::<Vec<_>>();
    versions.sort_by_key(|entry| std::cmp::Reverse(entry.file_name()));

    for entry in versions {
        let candidate = entry.path().join("x64").join(tool_name);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn cmake_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

#[cfg(target_os = "windows")]
fn bundled_cmake_out_dir(out_dir: &Path, ocio_source: &Path, generator_tag: &str) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    out_dir.hash(&mut hasher);
    ocio_source.hash(&mut hasher);
    env::var_os("TARGET").hash(&mut hasher);
    generator_tag.hash(&mut hasher);

    let mut short_dir = env::temp_dir();
    short_dir.push("ocrs");
    short_dir.push(format!("{:016x}", hasher.finish()));
    short_dir.push(generator_tag);
    short_dir
}
