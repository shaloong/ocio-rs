use std::cell::{Cell, RefCell};
use std::env;
use std::path::PathBuf;
use std::rc::Rc;

#[cfg(all(feature = "bundled", target_os = "windows"))]
use std::collections::hash_map::DefaultHasher;
#[cfg(all(feature = "bundled", target_os = "windows"))]
use std::hash::{Hash, Hasher};
#[cfg(all(feature = "bundled", target_os = "windows"))]
use std::path::Path;

#[cfg(feature = "bundled")]
const BUNDLED_OCIO_VERSION: &str = "2.5.2";
const OPENCOLORIO_DEPENDENCY_NAME: &str = "opencolorio";
const OPENCOLORIO_LIB_NAME: &str = "OpenColorIO";
const OPENCOLORIO_PKG_CONFIG_FILES: [&str; 2] = ["OpenColorIO.pc", "opencolorio.pc"];

fn main() {
    let link_mode = LinkMode::from_env();

    // Real OCIO is enabled when:
    // 1. OCIO_RS_ENABLE_REAL=1 is explicitly set (manual override), OR
    // 2. The "bundled" feature is active (enables the from-source build fallback)
    let is_bundled = env::var_os("CARGO_FEATURE_BUNDLED").is_some();
    let enable_real_ocio = env_flag("OCIO_RS_ENABLE_REAL") || is_bundled;

    let mut include_paths = Vec::<PathBuf>::new();

    // Runtime DLL directories collected by the bundled build closure, if it ran.
    // Windows only: `rustc-link-search` only affects linking, not the loader path
    // test/example binaries use at runtime, so these need to be copied alongside them.
    let runtime_dll_dirs = Rc::new(RefCell::new(Vec::<PathBuf>::new()));
    let built_internally = Rc::new(Cell::new(false));

    let has_real_ocio = if !enable_real_ocio {
        println!("cargo:warning=OCIO_RS_ENABLE_REAL is not set; building ocio-sys in stub mode.");
        println!(
            "cargo:warning=Enable the 'bundled' feature, or set OCIO_RS_ENABLE_REAL=1, for real OCIO."
        );
        false
    } else {
        let install = configure_install_dir(link_mode, &runtime_dll_dirs);
        include_paths.extend(install.include_paths.iter().cloned());

        // system-deps resolves OpenColorIO via, in order: a system pkg-config
        // install, then the registered internal fallback — see
        // SYSTEM_DEPS_OPENCOLORIO_BUILD_INTERNAL to control this at build time.
        // The "bundled" feature supplies the real from-source implementation;
        // without it, selecting the fallback reports how to enable it.
        //
        // Forward OCIO_RS_LINK, this crate's public static/dynamic linking
        // knob, to system-deps' equivalent env var.
        // SAFETY: build scripts are single-threaded at this point.
        unsafe {
            env::set_var(
                "SYSTEM_DEPS_OPENCOLORIO_LINK",
                if link_mode.is_static() {
                    "static"
                } else {
                    "dynamic"
                },
            );
        }

        // `add_build_internal`'s status always defaults to `never` regardless of
        // whether a closure is registered. The "bundled" feature previously
        // built from source by default, but an explicit OCIO_INSTALL_DIR took
        // precedence. Preserve both behaviors while still allowing
        // SYSTEM_DEPS_OPENCOLORIO_BUILD_INTERNAL to override the default.
        #[cfg(feature = "bundled")]
        if env::var_os("SYSTEM_DEPS_OPENCOLORIO_BUILD_INTERNAL").is_none()
            && env::var_os("OCIO_INSTALL_DIR").is_none()
        {
            // SAFETY: build scripts are single-threaded at this point.
            unsafe {
                env::set_var("SYSTEM_DEPS_OPENCOLORIO_BUILD_INTERNAL", "always");
            }
        }

        let config = {
            let runtime_dll_dirs = runtime_dll_dirs.clone();
            let built_internally = built_internally.clone();
            system_deps::Config::new().add_build_internal(
                OPENCOLORIO_DEPENDENCY_NAME,
                move |_lib_name, version| {
                    built_internally.set(true);
                    build_ocio_from_source(version, link_mode, runtime_dll_dirs)
                },
            )
        };

        let deps = config
            .probe()
            .expect("system-deps failed to resolve the opencolorio dependency");
        let lib = deps
            .get_by_name(OPENCOLORIO_DEPENDENCY_NAME)
            .expect("system-deps should always resolve opencolorio when real OCIO is enabled");
        include_paths.extend(lib.include_paths.iter().cloned());
        if link_mode.is_static() && !built_internally.get() {
            let mut transitive_link_paths = lib.link_paths.clone();
            for path in install.link_paths {
                push_existing_path(&mut transitive_link_paths, path);
            }
            if resolved_static_opencolorio(lib, &transitive_link_paths) {
                emit_transitive_static_deps(&transitive_link_paths);
            }
        }

        true
    };

    if cfg!(target_os = "windows") && link_mode.is_dynamic() {
        copy_runtime_dlls_to_cargo_target_dirs(&runtime_dll_dirs.borrow());
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

fn configure_install_dir(
    link_mode: LinkMode,
    runtime_dll_dirs: &Rc<RefCell<Vec<PathBuf>>>,
) -> InstallDirConfiguration {
    let Some(install_dir) = env::var_os("OCIO_INSTALL_DIR").map(PathBuf::from) else {
        return InstallDirConfiguration::default();
    };

    let mut pkg_config_paths = [
        install_dir.join("lib").join("pkgconfig"),
        install_dir.join("lib64").join("pkgconfig"),
        install_dir.join("share").join("pkgconfig"),
    ]
    .into_iter()
    // Other dependencies may create the pkgconfig directory without
    // installing OpenColorIO metadata. Only skip the legacy prefix layout
    // when this prefix can actually satisfy the OpenColorIO probe.
    .filter(|path| {
        OPENCOLORIO_PKG_CONFIG_FILES
            .iter()
            .any(|file_name| path.join(file_name).is_file())
    })
    .collect::<Vec<_>>();

    let extension_roots = [
        install_dir.join("ext").join("dist"),
        install_dir.join("share").join("ocio").join("ext"),
    ];
    let include_paths = std::iter::once(install_dir.join("include"))
        .chain(extension_roots.iter().map(|root| root.join("include")))
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    let link_paths = [install_dir.clone()]
        .into_iter()
        .chain(extension_roots.iter().cloned())
        .flat_map(|root| [root.join("lib"), root.join("lib64")])
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();

    if link_mode.is_dynamic() {
        let mut runtime_paths = runtime_dll_dirs.borrow_mut();
        for root in [install_dir.clone()]
            .into_iter()
            .chain(extension_roots.iter().cloned())
        {
            for directory in [root.join("bin"), root.join("lib"), root.join("lib64")] {
                push_existing_path(&mut runtime_paths, directory);
            }
        }
    }

    if pkg_config_paths.is_empty() {
        set_env_if_missing("SYSTEM_DEPS_OPENCOLORIO_NO_PKG_CONFIG", "1");
        set_env_if_missing("SYSTEM_DEPS_OPENCOLORIO_LIB", "OpenColorIO");
        if !link_paths.is_empty() {
            let joined = env::join_paths(&link_paths)
                .expect("OCIO_INSTALL_DIR produced invalid library search paths");
            set_env_if_missing("SYSTEM_DEPS_OPENCOLORIO_SEARCH_NATIVE", joined);
        }
        if !include_paths.is_empty() {
            let joined = env::join_paths(&include_paths)
                .expect("OCIO_INSTALL_DIR produced invalid include paths");
            set_env_if_missing("SYSTEM_DEPS_OPENCOLORIO_INCLUDE", joined);
        }
    } else {
        if let Some(existing) = env::var_os("PKG_CONFIG_PATH") {
            pkg_config_paths.extend(env::split_paths(&existing));
        }
        let joined = env::join_paths(pkg_config_paths)
            .expect("OCIO_INSTALL_DIR produced an invalid pkg-config path");
        // SAFETY: build scripts are single-threaded at this point.
        unsafe {
            env::set_var("PKG_CONFIG_PATH", joined);
        }

        // OpenColorIO's pkg-config file omits its private static dependencies.
        // Keep the legacy extension directories visible to the linker so the
        // explicit transitive libraries below can be resolved.
        for path in &link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
    }

    InstallDirConfiguration {
        include_paths,
        link_paths,
    }
}

#[derive(Default)]
struct InstallDirConfiguration {
    include_paths: Vec<PathBuf>,
    link_paths: Vec<PathBuf>,
}

fn set_env_if_missing(name: &str, value: impl AsRef<std::ffi::OsStr>) {
    if env::var_os(name).is_none() {
        // SAFETY: build scripts are single-threaded at this point.
        unsafe {
            env::set_var(name, value);
        }
    }
}

#[cfg(not(feature = "bundled"))]
fn build_ocio_from_source(
    _version_requirement: &str,
    _link_mode: LinkMode,
    _runtime_dll_dirs: Rc<RefCell<Vec<PathBuf>>>,
) -> Result<system_deps::Library, system_deps::BuildInternalClosureError> {
    Err(system_deps::BuildInternalClosureError::failed(
        "OpenColorIO internal fallback requires enabling ocio-sys' `bundled` feature",
    ))
}

/// Builds OpenColorIO from source via CMake, to be used as a `system-deps`
/// `add_build_internal` fallback. Returns a [`system_deps::Library`] built from the
/// real `.pc` file OpenColorIO's own CMake build generates and installs (see
/// `src/OpenColorIO/CMakeLists.txt`'s `configure_file(res/OpenColorIO.pc.in ...)`),
/// plus the transitive static dependencies it doesn't declare there itself.
#[cfg(feature = "bundled")]
fn build_ocio_from_source(
    _version_requirement: &str,
    link_mode: LinkMode,
    runtime_dll_dirs: Rc<RefCell<Vec<PathBuf>>>,
) -> Result<system_deps::Library, system_deps::BuildInternalClosureError> {
    use system_deps::BuildInternalClosureError;

    let ocio_source = resolve_ocio_source_dir().ok_or_else(|| {
        BuildInternalClosureError::failed(
            "OpenColorIO source not found. Use a recursive checkout or set OCIO_SOURCE_DIR.",
        )
    })?;

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
            bundled_out_dir = bundled_cmake_out_dir(&bundled_out_dir, &ocio_source, &generator_tag);

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
    })
    .map_err(|e| {
        let msg = e
            .downcast_ref::<String>()
            .map(|s| s.as_str())
            .or_else(|| e.downcast_ref::<&str>().copied())
            .unwrap_or("unknown error");
        BuildInternalClosureError::failed(&format!("OpenColorIO bundled build failed: {msg}"))
    })?;

    // CMAKE_INSTALL_LIBDIR (and so where OpenColorIO.pc ends up) defaults to "lib" on
    // some platforms and "lib64" on others (e.g. Fedora); check both. This version of
    // system-deps only accepts a single directory here, unlike its still-unmerged
    // `binary` branch, so pick whichever one actually exists.
    let pkgconfig_dir = [dst.join("lib"), dst.join("lib64")]
        .into_iter()
        .map(|dir| dir.join("pkgconfig"))
        .find(|dir| dir.exists())
        .unwrap_or_else(|| dst.join("lib").join("pkgconfig"));
    // `from_internal_pkg_config` accepts a minimum version rather than the
    // full range syntax used by package metadata. The vendored source is
    // pinned to this exact release, so probe it with that concrete version.
    let mut lib = system_deps::Library::from_internal_pkg_config(
        &pkgconfig_dir,
        "OpenColorIO",
        BUNDLED_OCIO_VERSION,
    )?;

    let mut runtime_paths = Vec::new();
    if link_mode.is_dynamic() {
        push_existing_path(&mut runtime_paths, dst.join("bin"));
    }

    // OCIO_INSTALL_EXT_PACKAGES=ALL builds transitive deps under <build_dir>/ext/dist.
    // The cmake crate uses dst/build/ as the build directory. Individual ext packages
    // are inconsistent about "lib" vs "lib64" (even with each other: on Fedora, e.g.,
    // expat/Imath/yaml-cpp/minizip-ng install to lib64 but zlib installs to lib), so
    // check both everywhere.
    for ext_candidate in &[
        dst.join("build").join("ext").join("dist"),
        dst.join("ext").join("dist"),
    ] {
        for lib_dir in [ext_candidate.join("lib"), ext_candidate.join("lib64")] {
            if lib_dir.exists() {
                lib.link_paths.push(lib_dir);
            }
        }
        let inc_dir = ext_candidate.join("include");
        if inc_dir.exists() {
            lib.include_paths.push(inc_dir);
        }
        if link_mode.is_dynamic() {
            push_existing_path(&mut runtime_paths, ext_candidate.join("bin"));
            push_existing_path(&mut runtime_paths, ext_candidate.join("lib"));
            push_existing_path(&mut runtime_paths, ext_candidate.join("lib64"));
        }
    }

    if link_mode.is_dynamic() {
        collect_runtime_dll_dirs(
            &mut runtime_paths,
            &dst.join("build").join("ext").join("build"),
        );
        // Some generators place import .lib files alongside the DLLs.
        lib.link_paths.extend(runtime_paths.iter().cloned());
        runtime_dll_dirs.borrow_mut().extend(runtime_paths);
    }

    if link_mode.is_static() {
        // Static OpenColorIO doesn't embed its mandatory dependencies, and its
        // .pc file doesn't declare them either — link them explicitly until
        // https://github.com/AcademySoftwareFoundation/OpenColorIO/pull/2328
        // ships in a vendored release.
        //
        // Library file names vary by platform and build type; we emit the most
        // common name for each. If a particular name is wrong, the linker will
        // report which library is missing.
        add_transitive_static_libs(&mut lib);
    }

    Ok(lib)
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

    #[cfg(feature = "bundled")]
    fn cmake_build_shared_libs(self) -> &'static str {
        match self {
            Self::Static => "OFF",
            Self::Dynamic => "ON",
        }
    }

    #[cfg(feature = "bundled")]
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

#[cfg(all(feature = "bundled", target_os = "windows"))]
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

#[cfg(all(feature = "bundled", not(target_os = "windows")))]
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

// OpenColorIO's `.pc` file omits its private dependencies, so a static OCIO has
// to spell them out. Only do that when OCIO itself actually resolved to a static
// archive: a prefix shipping only a shared library (Homebrew, most distros)
// satisfies `OCIO_RS_LINK=static` dynamically, and emitting `static=expat`
// against it fails the link with "could not find native static library".
fn resolved_static_opencolorio(lib: &system_deps::Library, link_paths: &[PathBuf]) -> bool {
    if !lib.libs.is_empty() {
        return lib.libs.iter().any(|entry| entry.is_static_available);
    }

    // The legacy OCIO_INSTALL_DIR layout bypasses pkg-config, so system-deps
    // reports no per-library detail and the archive has to be found on disk.
    let file_name = static_library_file_name(OPENCOLORIO_LIB_NAME);
    link_paths.iter().any(|dir| dir.join(&file_name).exists())
}

fn emit_transitive_static_deps(link_paths: &[PathBuf]) {
    for candidates in SYSTEM_TRANSITIVE_STATIC_LIBRARIES {
        let selected = select_static_lib(link_paths, candidates);
        println!("cargo:rustc-link-lib=static={selected}");
    }

    #[cfg(target_os = "macos")]
    for framework in MACOS_STATIC_FRAMEWORKS {
        println!("cargo:rustc-link-lib=framework={framework}");
    }
}

#[cfg(target_os = "windows")]
const SYSTEM_TRANSITIVE_STATIC_LIBRARIES: &[&[&str]] = &[
    &["libexpatMD", "expat", "libexpatdMD"],
    &["yaml-cpp", "yaml-cppd"],
    &["Imath-3_2", "Imath-3_2_d", "Imath-3_1", "Imath-3_1_d"],
    &["pystring"],
    &["libminizip-ng", "minizip-ng", "libminizip", "minizip"],
    &["zlibstatic", "zlib", "zlibstaticd", "zlibd"],
];

#[cfg(not(target_os = "windows"))]
const SYSTEM_TRANSITIVE_STATIC_LIBRARIES: &[&[&str]] = &[
    &["expat"],
    &["yaml-cpp"],
    &["Imath-3_2", "Imath-3_1"],
    &["pystring"],
    &["minizip-ng", "minizip"],
    &["z"],
];

#[cfg(all(feature = "bundled", not(target_os = "windows")))]
const BUNDLED_TRANSITIVE_STATIC_LIBRARIES: [&str; 6] = [
    "expat",
    "yaml-cpp",
    "Imath-3_2",
    "pystring",
    "minizip-ng",
    "z",
];

#[cfg(target_os = "macos")]
const MACOS_STATIC_FRAMEWORKS: [&str; 4] = ["ColorSync", "CoreFoundation", "CoreGraphics", "IOKit"];

fn select_static_lib<'a>(link_paths: &[PathBuf], candidates: &'a [&str]) -> &'a str {
    candidates
        .iter()
        .copied()
        .find(|candidate| {
            let file_name = static_library_file_name(candidate);
            link_paths.iter().any(|dir| dir.join(&file_name).exists())
        })
        .or_else(|| candidates.first().copied())
        .expect("every static dependency has at least one candidate")
}

#[cfg(target_os = "windows")]
fn static_library_file_name(name: &str) -> String {
    format!("{name}.lib")
}

#[cfg(not(target_os = "windows"))]
fn static_library_file_name(name: &str) -> String {
    format!("lib{name}.a")
}

// See the call site's comment: OpenColorIO's own .pc file doesn't declare these,
// so pkg-config can't find them and they're linked explicitly instead.
#[cfg(all(feature = "bundled", target_os = "windows"))]
fn add_transitive_static_libs(lib: &mut system_deps::Library) {
    let link_paths = lib.link_paths.clone();
    add_static_lib(lib, &link_paths, &["libexpatMD", "expat", "libexpatdMD"]);
    add_static_lib(lib, &link_paths, &["yaml-cpp", "yaml-cppd"]);
    add_static_lib(lib, &link_paths, &["Imath-3_2", "Imath-3_2_d"]);
    add_static_lib(lib, &link_paths, &["pystring"]);
    add_static_lib(lib, &link_paths, &["minizip-ng"]);
    add_static_lib(
        lib,
        &link_paths,
        &["zlibstatic", "zlib", "zlibstaticd", "zlibd"],
    );
}

// See the call site's comment: OpenColorIO's own .pc file doesn't declare these,
// so pkg-config can't find them and they're linked explicitly instead.
#[cfg(all(feature = "bundled", not(target_os = "windows")))]
fn add_transitive_static_libs(lib: &mut system_deps::Library) {
    for name in BUNDLED_TRANSITIVE_STATIC_LIBRARIES {
        lib.libs.push(system_deps::InternalLib {
            name: name.to_string(),
            is_static_available: true,
        });
    }

    // Static libOpenColorIO.a doesn't carry these either: OCIO's SystemMonitor
    // support links them privately on Apple platforms (src/OpenColorIO/
    // CMakeLists.txt's `if(APPLE)` block), so the final binary must link them
    // when the archive is consumed directly.
    #[cfg(target_os = "macos")]
    lib.frameworks
        .extend(MACOS_STATIC_FRAMEWORKS.map(String::from));
}

// Called by add_transitive_static_libs above: picks whichever candidate .lib
// file name actually exists (debug/release and MD/MT builds name them
// differently), since pkg-config has no record of these libs to consult.
#[cfg(all(feature = "bundled", target_os = "windows"))]
fn add_static_lib(lib: &mut system_deps::Library, link_paths: &[PathBuf], candidates: &[&str]) {
    let candidate = select_static_lib(link_paths, candidates);
    lib.libs.push(system_deps::InternalLib {
        name: candidate.to_string(),
        is_static_available: true,
    });
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

#[cfg(feature = "bundled")]
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

#[cfg(all(feature = "bundled", target_os = "windows"))]
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

#[cfg(all(feature = "bundled", target_os = "windows"))]
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

#[cfg(all(feature = "bundled", target_os = "windows"))]
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

#[cfg(all(feature = "bundled", target_os = "windows"))]
fn cmake_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

#[cfg(all(feature = "bundled", target_os = "windows"))]
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
