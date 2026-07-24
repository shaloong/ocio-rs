use std::fmt;

/// Direction in which an OCIO transform or processor is evaluated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TransformDirection {
    Forward = 0,
    Inverse = 1,
}

impl fmt::Display for TransformDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransformDirection::Forward => write!(f, "forward"),
            TransformDirection::Inverse => write!(f, "inverse"),
        }
    }
}

/// Reference space domain used by a color space or view transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReferenceSpaceType {
    Scene = 0,
    Display = 1,
}

/// Domain filter used when searching for a color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SearchReferenceSpaceType {
    Scene = 0,
    Display = 1,
    All = 2,
}

/// Visibility filter for color-space enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ColorSpaceVisibility {
    Active = 0,
    Inactive = 1,
    All = 2,
}

/// Runtime OCIO transform subtype represented by a generic transform handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TransformType {
    Allocation = 0,
    Builtin = 1,
    Cdl = 2,
    ColorSpace = 3,
    DisplayView = 4,
    Exponent = 5,
    ExponentWithLinear = 6,
    ExposureContrast = 7,
    File = 8,
    FixedFunction = 9,
    #[cfg(feature = "v2_5")]
    GradingHueCurve = 10,
    GradingPrimary = 11,
    GradingRgbCurve = 12,
    GradingTone = 13,
    Group = 14,
    LogAffine = 15,
    LogCamera = 16,
    Log = 17,
    Look = 18,
    Lut1D = 19,
    Lut3D = 20,
    Matrix = 21,
    Range = 22,
}

impl fmt::Display for TransformType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Interpolation algorithm used by LUT and related transforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Interpolation {
    Unknown = 0,
    Nearest = 1,
    Linear = 2,
    Tetrahedral = 3,
    Cubic = 4,
    // OCIO deliberately numbers the "meta" modes apart from the real ones.
    Default = 254,
    Best = 255,
}

/// Pixel sample bit depth understood by OCIO processors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BitDepth {
    Unknown = 0,
    Uint8 = 1,
    Uint10 = 2,
    Uint12 = 3,
    Uint14 = 4,
    Uint16 = 5,
    Uint32 = 6,
    F16 = 7,
    F32 = 8,
}

/// Allocation domain used to map image values for processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Allocation {
    Unknown = 0,
    Uniform = 1,
    Lg2 = 2,
}

/// Target shading language for OCIO GPU shader extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GpuLanguage {
    Cg = 0,
    Glsl1_2 = 1,
    Glsl1_3 = 2,
    Glsl4_0 = 3,
    #[cfg(feature = "v2_5")]
    GlslVk4_6 = 4,
    HlslSm5_0 = 5,
    Osl1 = 6,
    GlslEs1_0 = 7,
    GlslEs3_0 = 8,
    Msl2_0 = 9,
}

/// Policy used to populate OCIO context variables from the environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EnvironmentMode {
    LoadPredefined = 0,
    LoadAll = 1,
}

/// Whether a range transform clamps values at its domain boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RangeStyle {
    NoClamp = 0,
    Clamp = 1,
}

/// Built-in OCIO fixed-function transform style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FixedFunctionStyle {
    AcesRedMod03 = 0,
    AcesRedMod10 = 1,
    AcesGlow03 = 2,
    AcesGlow10 = 3,
    AcesDarkToDim10 = 4,
    Rec2100Surround = 5,
    RgbToHsv = 6,
    XyzToxyY = 7,
    XyzTouvY = 8,
    XyzToLuv = 9,
    AcesGamutMap02 = 10,
    AcesGamutMap07 = 11,
    AcesGamutCompress13 = 12,
    LinToPq = 13,
    LinToGammaLog = 14,
    LinToDoubleLog = 15,
    AcesOutputTransform20 = 16,
    AcesRgbToJmh20 = 17,
    AcesTonescaleCompress20 = 18,
    AcesGamutCompress20 = 19,
    #[cfg(feature = "v2_5")]
    RgbToHsyLin = 20,
    #[cfg(feature = "v2_5")]
    RgbToHsyLog = 21,
    #[cfg(feature = "v2_5")]
    RgbToHsyVid = 22,
}

/// Domain in which exposure/contrast parameters are applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExposureContrastStyle {
    Linear = 0,
    Video = 1,
    Logarithmic = 2,
}

/// ASC CDL clamping behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CDLStyle {
    Asc = 0,
    NoClamp = 1,
}

/// Treatment of negative values for transforms that require a policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NegativeStyle {
    Clamp = 0,
    Mirror = 1,
    PassThru = 2,
    Linear = 3,
}

/// One of the RGB or master curves in a grading RGB curve transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RGBCurveType {
    Red = 0,
    Green = 1,
    Blue = 2,
    Master = 3,
}

/// One of OCIO's hue-dependent grading curve families.
#[cfg(feature = "v2_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum HueCurveType {
    HueHue = 0,
    HueSat = 1,
    HueLum = 2,
    LumSat = 3,
    SatSat = 4,
    LumLum = 5,
    SatLum = 6,
    HueFx = 7,
}

/// HSY conversion style used by hue-curve operations.
#[cfg(feature = "v2_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum HSYTransformStyle {
    None = 0,
    Default = 1,
}

/// Grading domain used by OCIO grading transforms and values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GradingStyle {
    Log = 0,
    Lin = 1,
    Video = 2,
}

/// Bit flags selecting processor optimization passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptimizationFlags(pub u32);

impl OptimizationFlags {
    pub const NONE: Self = Self(0x00000000);
    pub const IDENTITY: Self = Self(0x00000001);
    pub const PAIR_IDENTITY_CDL: Self = Self(0x00000002);
    pub const PAIR_IDENTITY_LUT1D: Self = Self(0x00000004);
    pub const PAIR_IDENTITY_LUT3D: Self = Self(0x00000008);
    pub const PAIR_IDENTITY_LOG: Self = Self(0x00000010);
    pub const PAIR_IDENTITY_EXPONENT: Self = Self(0x00000020);
    pub const COMP_EXPONENT: Self = Self(0x00000040);
    pub const COMP_MATRIX: Self = Self(0x00000080);
    pub const COMP_RANGE: Self = Self(0x00000100);
    pub const LUT_INV_FAST: Self = Self(0x00000200);
    pub const FAST_LOG_EXP_POW: Self = Self(0x00000400);
    pub const SIMPLIFY_OPS: Self = Self(0x00000800);
    pub const NO_DYNAMIC_PROPERTIES: Self = Self(0x00001000);
    pub const LOSSLESS: Self = Self(Self::IDENTITY.0);
    pub const VERY_GOOD: Self = Self(0x000001ff);
    pub const GOOD: Self = Self(0x000003ff);
    pub const DRAFT: Self = Self(0x00000fff);
    pub const ALL: Self = Self(0xFFFFFFFF);
    pub const DEFAULT: Self = Self::VERY_GOOD;
}

impl std::ops::BitOr for OptimizationFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// Verbosity level for OCIO's process-global diagnostic logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LoggingLevel {
    None = 0,
    Warning = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

/// Bit flags controlling config-level processor-cache behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessorCacheFlags(pub u32);

impl ProcessorCacheFlags {
    pub const OFF: Self = Self(0x00000000);
    pub const ENABLED: Self = Self(0x00000001);
    pub const SHARE_DYN_PROPERTIES: Self = Self(0x00000002);
    pub const DEFAULT: Self = Self::ENABLED;
}

impl std::ops::BitOr for ProcessorCacheFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// Origin of a display/view entry in a config.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ViewType {
    Shared = 0,
    DisplayDefined = 1,
}

/// Direction between a color space and its reference space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ColorSpaceDirection {
    ToReference = 0,
    FromReference = 1,
}

/// Direction between a view transform and its reference space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ViewTransformDirection {
    ToReference = 0,
    FromReference = 1,
}

/// Visibility filter for named-transform enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NamedTransformVisibility {
    Active = 0,
    Inactive = 1,
    All = 2,
}

/// Runtime-adjustable OCIO processor property kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DynamicPropertyType {
    Exposure = 0,
    Contrast = 1,
    Gamma = 2,
    GradingPrimary = 3,
    GradingRgbCurve = 4,
    GradingTone = 5,
    #[cfg(feature = "v2_5")]
    GradingHueCurve = 6,
}

/// Hue adjustment mode for one-dimensional LUT transforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Lut1DHueAdjust {
    None_ = 0,
    Dw3 = 1,
    Wypn = 2,
}

/// Channel memory ordering for OCIO packed image descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChannelOrdering {
    Rgba = 0,
    Bgra = 1,
    Abgr = 2,
    Rgb = 3,
    Bgr = 4,
}
