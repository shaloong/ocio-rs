use std::env;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Real(RealReason),
    Stub,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RealReason {
    BundledFeature,
    SystemFeature,
    EnableRealFlag,
    InstallDir,
}

impl Backend {
    pub fn select() -> Self {
        let bundled = cfg!(feature = "bundled");
        let system = cfg!(feature = "system");

        let legacy_real = env::var("OCIO_RS_ENABLE_REAL")
            .ok()
            .map(|value| parse_bool("OCIO_RS_ENABLE_REAL", &value));
        let install_dir = env::var_os("OCIO_INSTALL_DIR").is_some();

        if bundled {
            if legacy_real == Some(false) {
                panic!("the 'bundled' feature conflicts with OCIO_RS_ENABLE_REAL=0");
            }
            return Self::Real(RealReason::BundledFeature);
        }
        if system {
            if legacy_real == Some(false) {
                panic!("the 'system' feature conflicts with OCIO_RS_ENABLE_REAL=0");
            }
            return Self::Real(RealReason::SystemFeature);
        }

        match legacy_real {
            Some(true) => Self::Real(RealReason::EnableRealFlag),
            Some(false) => Self::Stub,
            None if install_dir => Self::Real(RealReason::InstallDir),
            None => Self::Stub,
        }
    }

    pub fn is_stub(self) -> bool {
        matches!(self, Self::Stub)
    }

    pub fn resolution_failure(self, error: &system_deps::Error) -> String {
        let requested_by = match self {
            Self::Real(RealReason::BundledFeature) => "the 'bundled' feature is enabled",
            Self::Real(RealReason::SystemFeature) => "the 'system' feature is enabled",
            Self::Real(RealReason::EnableRealFlag) => "OCIO_RS_ENABLE_REAL is set",
            Self::Real(RealReason::InstallDir) => "OCIO_INSTALL_DIR is set",
            Self::Stub => "a real OpenColorIO was requested",
        };
        format!(
            "{requested_by}, so ocio-sys requires a usable OpenColorIO, but resolving one failed.\n\
             Remove the real-backend request to use the default stub mode.\n\n{error}"
        )
    }
}

fn parse_bool(name: &str, value: &str) -> bool {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" => true,
        "0" | "false" | "no" => false,
        _ => panic!("{name} must be one of 1/true/yes or 0/false/no; got {value:?}"),
    }
}
