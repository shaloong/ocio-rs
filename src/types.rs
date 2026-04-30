use std::fmt;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReferenceSpaceType {
    Scene = 0,
    Display = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SearchReferenceSpaceType {
    Scene = 0,
    Display = 1,
    All = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ColorSpaceVisibility {
    Active = 0,
    Inactive = 1,
    All = 2,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Interpolation {
    Unknown = 0,
    Nearest = 1,
    Linear = 2,
    Tetrahedral = 3,
    Cubic = 4,
    Default = 5,
    Best = 6,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Allocation {
    Unknown = 0,
    Uniform = 1,
    Lg2 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GpuLanguage {
    Cg = 0,
    Glsl1_2 = 1,
    Glsl1_3 = 2,
    Glsl4_0 = 3,
    GlslVk4_6 = 4,
    HlslSm5_0 = 5,
    Osl1 = 6,
    GlslEs1_0 = 7,
    GlslEs3_0 = 8,
    Msl2_0 = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EnvironmentMode {
    LoadPredefined = 0,
    LoadAll = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RangeStyle {
    NoClamp = 0,
    Clamp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FixedFunctionStyle {
    AcesRedMod03 = 0,
    AcesRedMod10 = 1,
    AcesGlow03 = 2,
    AcesGlow10 = 3,
    AcesGamutCompress13 = 4,
    AcesGamutCompress20 = 5,
    Rec2100Surround = 6,
    RgbToHsv = 7,
    XyzToxyY = 8,
    XyzTouvY = 9,
    XyzToLuv = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExposureContrastStyle {
    Linear = 0,
    Video = 1,
    Logarithmic = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CDLStyle {
    Asc = 0,
    NoClamp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NegativeStyle {
    Clamp = 0,
    Mirror = 1,
    PassThru = 2,
    Linear = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RGBCurveType {
    Red = 0,
    Green = 1,
    Blue = 2,
    Master = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GradingStyle {
    Log = 0,
    Lin = 1,
    Video = 2,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LoggingLevel {
    None = 0,
    Warning = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ProcessorCacheFlags {
    Off = 0x00000000,
    Enabled = 0x00000001,
    ShareDynProperties = 0x00000002,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ViewType {
    Shared = 0,
    DisplayDefined = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ColorSpaceDirection {
    ToReference = 0,
    FromReference = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ViewTransformDirection {
    ToReference = 0,
    FromReference = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NamedTransformVisibility {
    Active = 0,
    Inactive = 1,
    All = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DynamicPropertyType {
    Exposure = 0,
    Contrast = 1,
    Gamma = 2,
    GradingPrimary = 3,
    GradingRgbCurve = 4,
    GradingTone = 5,
    GradingHueCurve = 6,
}
