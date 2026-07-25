use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

static NEXT_TEMP_ID: AtomicUsize = AtomicUsize::new(0);
static CARGO_LOCK: Mutex<()> = Mutex::new(());

const FAKE_PKG_CONFIG_SOURCE: &str = r#"
use std::env;
use std::path::{Path, PathBuf};
use std::process;

fn normalized(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn main() {
    if env::var_os("FAKE_PKG_CONFIG_FAIL").is_some() {
        eprintln!("fake pkg-config: requested probe failure");
        process::exit(1);
    }

    if let Some(expected) = env::var_os("FAKE_PKG_CONFIG_EXPECT_PATH") {
        let expected = PathBuf::from(expected);
        let actual = env::var_os("PKG_CONFIG_PATH").unwrap_or_default();
        if !env::split_paths(&actual).any(|path| path == expected) {
            eprintln!(
                "fake pkg-config: PKG_CONFIG_PATH does not contain {} (actual: {:?})",
                expected.display(),
                actual
            );
            process::exit(2);
        }
    }

    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.iter().any(|arg| arg == "--libs") {
        if let Some(expected) = env::var_os("FAKE_PKG_CONFIG_EXPECT_ARG") {
            let expected = expected.to_string_lossy();
            if !args.iter().any(|arg| arg == expected.as_ref()) {
                eprintln!(
                    "fake pkg-config: expected argument {expected:?} (actual: {args:?})"
                );
                process::exit(3);
            }
        }
    }
    if args.iter().any(|arg| arg == "--modversion") {
        println!("2.5.2");
        return;
    }

    let include = PathBuf::from(env::var_os("FAKE_PKG_CONFIG_INCLUDE").unwrap());
    let lib = PathBuf::from(env::var_os("FAKE_PKG_CONFIG_LIB").unwrap());
    println!(
        "-I{} -L{} -lOpenColorIO",
        normalized(&include),
        normalized(&lib)
    );
}
"#;

struct ProbeFixture {
    root: PathBuf,
    manifest: PathBuf,
    fake_pkg_config: PathBuf,
    install_dir: PathBuf,
    pkg_config_dir: PathBuf,
    include_dir: PathBuf,
    lib_dir: PathBuf,
}

impl ProbeFixture {
    fn new(enable_bundled: bool) -> Self {
        let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ocio-sys-build-configuration-{}-{id}",
            std::process::id()
        ));
        fs::create_dir_all(root.join("src")).unwrap();

        let ocio_sys_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dependency = if enable_bundled {
            format!(
                "ocio-sys = {{ path = '{}', features = ['bundled'] }}",
                toml_path(&ocio_sys_dir)
            )
        } else {
            format!("ocio-sys = {{ path = '{}' }}", toml_path(&ocio_sys_dir))
        };
        fs::write(
            root.join("Cargo.toml"),
            format!(
                "[package]\nname = 'ocio-sys-build-probe'\nversion = '0.0.0'\nedition = '2021'\n\n[dependencies]\n{dependency}\n"
            ),
        )
        .unwrap();
        fs::write(root.join("src/lib.rs"), "pub fn probe() {}\n").unwrap();

        let install_dir = root.join("installed-ocio");
        let pkg_config_dir = install_dir.join("lib").join("pkgconfig");
        let include_dir = install_dir.join("include");
        let lib_dir = install_dir.join("lib");
        fs::create_dir_all(&pkg_config_dir).unwrap();
        fs::create_dir_all(&lib_dir).unwrap();
        prepare_headers(&ocio_sys_dir, &include_dir);
        fs::write(
            lib_dir.join(if cfg!(target_os = "windows") {
                "OpenColorIO.lib"
            } else {
                "libOpenColorIO.a"
            }),
            [],
        )
        .unwrap();

        let fake_source = root.join("fake-pkg-config.rs");
        fs::write(&fake_source, FAKE_PKG_CONFIG_SOURCE).unwrap();
        let fake_pkg_config = root.join(format!("fake-pkg-config{}", std::env::consts::EXE_SUFFIX));
        let output = Command::new("rustc")
            .arg(&fake_source)
            .arg("-o")
            .arg(&fake_pkg_config)
            .output()
            .expect("rustc should compile the fake pkg-config boundary");
        assert!(
            output.status.success(),
            "failed to compile fake pkg-config:\n{}",
            output_text(&output)
        );

        Self {
            manifest: root.join("Cargo.toml"),
            root,
            fake_pkg_config,
            install_dir,
            pkg_config_dir,
            include_dir,
            lib_dir,
        }
    }

    fn cargo_check(&self, configure: impl FnOnce(&mut Command)) -> Output {
        let _guard = CARGO_LOCK.lock().unwrap();
        let ocio_sys_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let target_dir = ocio_sys_dir
            .parent()
            .unwrap()
            .join("target")
            .join("build-configuration-tests");
        let mut command = Command::new(env!("CARGO"));
        command
            .arg("check")
            .arg("--offline")
            .arg("--color=never")
            .arg("-vv")
            .arg("--manifest-path")
            .arg(&self.manifest)
            .env("CARGO_TARGET_DIR", target_dir)
            .env("PKG_CONFIG", &self.fake_pkg_config)
            .env("FAKE_PKG_CONFIG_INCLUDE", &self.include_dir)
            .env("FAKE_PKG_CONFIG_LIB", &self.lib_dir)
            .env_remove("PKG_CONFIG_PATH")
            .env_remove("FAKE_PKG_CONFIG_EXPECT_ARG")
            .env_remove("SYSTEM_DEPS_BUILD_INTERNAL")
            .env_remove("SYSTEM_DEPS_OPENCOLORIO_BUILD_INTERNAL")
            .env_remove("SYSTEM_DEPS_OPENCOLORIO_INCLUDE")
            .env_remove("SYSTEM_DEPS_OPENCOLORIO_LIB")
            .env_remove("SYSTEM_DEPS_OPENCOLORIO_NO_PKG_CONFIG")
            .env_remove("SYSTEM_DEPS_OPENCOLORIO_SEARCH_NATIVE")
            .env_remove("OCIO_INSTALL_DIR")
            .env_remove("OCIO_SOURCE_DIR")
            .env_remove("OCIO_RS_ENABLE_REAL")
            .env_remove("OCIO_RS_LINK");
        configure(&mut command);
        command.output().expect("nested cargo check should run")
    }
}

impl Drop for ProbeFixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

#[test]
fn installed_prefix_is_added_to_pkg_config_search_path() {
    let fixture = ProbeFixture::new(false);
    let output = fixture.cargo_check(|command| {
        command
            .env("OCIO_RS_ENABLE_REAL", "1")
            .env("OCIO_INSTALL_DIR", &fixture.install_dir)
            .env("FAKE_PKG_CONFIG_EXPECT_PATH", &fixture.pkg_config_dir)
            .env("FAKE_PKG_CONFIG_EXPECT_ARG", "opencolorio < 2.6");
    });

    assert!(
        output.status.success(),
        "OCIO_INSTALL_DIR should make its pkg-config metadata discoverable:\n{}",
        output_text(&output)
    );
}

#[test]
fn installed_prefix_without_pkg_config_uses_the_legacy_layout() {
    let fixture = ProbeFixture::new(false);
    fs::remove_dir_all(&fixture.pkg_config_dir).unwrap();
    let output = fixture.cargo_check(|command| {
        command
            .env("OCIO_RS_ENABLE_REAL", "1")
            .env("OCIO_INSTALL_DIR", &fixture.install_dir)
            .env("FAKE_PKG_CONFIG_FAIL", "1");
    });
    let text = output_text(&output);

    assert!(
        output.status.success(),
        "OCIO_INSTALL_DIR should keep supporting include/ and lib/ without pkg-config:\n{text}"
    );
    assert!(
        text.contains(&format!(
            "rustc-link-search=native={}",
            fixture.lib_dir.display()
        )),
        "the legacy installation's library directory should be linked:\n{text}"
    );
}

#[test]
fn auto_mode_reaches_the_registered_fallback_when_pkg_config_misses() {
    let fixture = ProbeFixture::new(false);
    let output = fixture.cargo_check(|command| {
        command
            .env("OCIO_RS_ENABLE_REAL", "1")
            .env("SYSTEM_DEPS_OPENCOLORIO_BUILD_INTERNAL", "auto")
            .env("FAKE_PKG_CONFIG_FAIL", "1");
    });
    let text = output_text(&output);

    assert!(
        !output.status.success(),
        "the unavailable internal build should stop the fallback"
    );
    assert!(
        text.contains("requires enabling ocio-sys' `bundled` feature"),
        "auto mode should reach the registered fallback without requiring bundled dependencies:\n{text}"
    );
    assert!(
        !text.contains("BuildInternalNoClosure"),
        "auto mode must not lose the fallback closure through a name mismatch:\n{text}"
    );
}

#[cfg(feature = "bundled")]
#[test]
fn auto_mode_reaches_the_bundled_build_when_pkg_config_misses() {
    let fixture = ProbeFixture::new(true);
    let invalid_source = fixture.root.join("invalid-ocio-source");
    fs::create_dir_all(&invalid_source).unwrap();
    fs::write(
        invalid_source.join("CMakeLists.txt"),
        "this is deliberately not valid CMake\n",
    )
    .unwrap();
    let output = fixture.cargo_check(|command| {
        command
            .env("SYSTEM_DEPS_OPENCOLORIO_BUILD_INTERNAL", "auto")
            .env("FAKE_PKG_CONFIG_FAIL", "1")
            .env("OCIO_SOURCE_DIR", &invalid_source);
    });
    let text = output_text(&output);

    assert!(
        !output.status.success(),
        "the deliberately missing source tree should stop the fallback"
    );
    assert!(
        text.contains("OpenColorIO bundled build failed"),
        "auto mode should reach the registered bundled fallback:\n{text}"
    );
    assert!(
        !text.contains("BuildInternalNoClosure"),
        "auto mode must not lose the fallback closure through a name mismatch:\n{text}"
    );
}

#[test]
fn static_system_install_links_open_color_io_transitive_dependencies() {
    let fixture = ProbeFixture::new(false);
    let output = fixture.cargo_check(|command| {
        command
            .env("OCIO_RS_ENABLE_REAL", "1")
            .env("OCIO_RS_LINK", "static");
    });
    let text = output_text(&output);

    assert!(
        output.status.success(),
        "the fake system OpenColorIO install should be probeable:\n{text}"
    );

    let expected = if cfg!(target_os = "windows") {
        [
            "libexpatMD",
            "yaml-cpp",
            "Imath-3_2",
            "pystring",
            "minizip-ng",
            "zlibstatic",
        ]
    } else {
        [
            "expat",
            "yaml-cpp",
            "Imath-3_2",
            "pystring",
            "minizip-ng",
            "z",
        ]
    };
    for library in expected {
        assert!(
            text.contains(&format!("rustc-link-lib=static={library}")),
            "static system OpenColorIO should link {library} explicitly:\n{text}"
        );
    }

    if cfg!(target_os = "macos") {
        for framework in ["ColorSync", "CoreFoundation", "CoreGraphics", "IOKit"] {
            assert!(
                text.contains(&format!("rustc-link-lib=framework={framework}")),
                "static system OpenColorIO should link {framework} explicitly:\n{text}"
            );
        }
    }
}

fn prepare_headers(ocio_sys_dir: &Path, include_dir: &Path) {
    let source = ocio_sys_dir
        .join("vendor")
        .join("OpenColorIO")
        .join("include")
        .join("OpenColorIO");
    let destination = include_dir.join("OpenColorIO");
    fs::create_dir_all(&destination).unwrap();
    for file in [
        "OpenColorAppHelpers.h",
        "OpenColorIO.h",
        "OpenColorTransforms.h",
        "OpenColorTypes.h",
    ] {
        fs::copy(source.join(file), destination.join(file)).unwrap();
    }

    let abi = fs::read_to_string(source.join("OpenColorABI.h.in"))
        .unwrap()
        .replace("@OCIO_NAMESPACE@", "OpenColorIO_v2_5")
        .replace("@OpenColorIO_VERSION@", "2.5.2")
        .replace("@OpenColorIO_VERSION_RELEASE_TYPE@", "")
        .replace("@OpenColorIO_VERSION_MAJOR@", "2")
        .replace("@OpenColorIO_VERSION_MINOR@", "5")
        .replace("@OpenColorIO_VERSION_PATCH@", "2");
    fs::write(destination.join("OpenColorABI.h"), abi).unwrap();
}

fn toml_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .replace('\'', "''")
}

fn output_text(output: &Output) -> String {
    format!(
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}
