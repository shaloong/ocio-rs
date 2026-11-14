//! Native OpenColorIO enum and string conversions.
use crate::{OcioError, Result};
use std::ffi::CStr;

fn native_to_string(kind: i32, value: i32) -> Result<String> {
    crate::clear_last_error();
    let ptr = unsafe { ocio_sys::ocio_enum_to_string(kind, value) };
    let result = unsafe { crate::cstr_to_opt_string(ptr) };
    crate::ocio_call_status()?;
    result.ok_or_else(|| OcioError::Ocio("null native enum string".into()))
}
fn native_write_string(kind: i32, value: i32, output: &mut [u8]) -> Result<usize> {
    crate::clear_last_error();
    let ptr = unsafe { ocio_sys::ocio_enum_to_string(kind, value) };
    crate::ocio_call_status()?;
    if ptr.is_null() {
        return Err(OcioError::Ocio("null native enum string".into()));
    }
    let bytes = unsafe { CStr::from_ptr(ptr) }.to_bytes();
    if output.len() < bytes.len() {
        return Err(OcioError::InvalidInput(format!(
            "enum string buffer requires {} bytes",
            bytes.len()
        )));
    }
    output[..bytes.len()].copy_from_slice(bytes);
    Ok(bytes.len())
}
fn native_from_string(kind: i32, value: &CStr) -> Result<i32> {
    crate::clear_last_error();
    let result = unsafe { ocio_sys::ocio_enum_from_string(kind, value.as_ptr()) };
    crate::ocio_call_status()?;
    Ok(result)
}

/// Native OCIO text conversion for an enum or boolean.
///
/// Parsing and formatting delegate to the linked OCIO version, including its
/// aliases, sentinels and exceptions. Names are returned as owned strings.
pub trait OcioEnumText: Sized + Copy {
    /// Format using the native OCIO conversion.
    fn to_ocio_string(self) -> Result<String>;
    /// Write the native spelling as UTF-8 bytes, without a terminator or allocation.
    fn write_ocio_string(self, output: &mut [u8]) -> Result<usize>;
    /// Parse a NUL-terminated string using the native OCIO conversion.
    fn from_ocio_string(value: &CStr) -> Result<Self>;
}

impl OcioEnumText for bool {
    fn to_ocio_string(self) -> Result<String> {
        native_to_string(0, i32::from(self))
    }
    fn from_ocio_string(value: &CStr) -> Result<Self> {
        Ok(native_from_string(0, value)? != 0)
    }
    fn write_ocio_string(self, output: &mut [u8]) -> Result<usize> {
        native_write_string(0, i32::from(self), output)
    }
}

macro_rules! native_enum {
    ($ty:ident, $kind:expr, [$($(#[$attr:meta])* $variant:ident),+ $(,)?]) => {
        impl OcioEnumText for crate::$ty {
            fn to_ocio_string(self) -> Result<String> {
                let raw = self as i32;
                let mut valid = false;
                $( $(#[$attr])* { valid |= raw == crate::$ty::$variant as i32; } )+
                if !valid { return Err(OcioError::InvalidInput("value is not supported by OpenColorIO".into())); }
                native_to_string($kind, raw)
            }
            fn write_ocio_string(self, output: &mut [u8]) -> Result<usize> {
                let raw = self as i32;
                let mut valid = false;
                $( $(#[$attr])* { valid |= raw == crate::$ty::$variant as i32; } )+
                if !valid { return Err(OcioError::InvalidInput("value is not supported by OpenColorIO".into())); }
                native_write_string($kind, raw, output)
            }
            fn from_ocio_string(value: &CStr) -> Result<Self> {
                let raw = native_from_string($kind, value)?;
                $( $(#[$attr])* if raw == crate::$ty::$variant as i32 { return Ok(crate::$ty::$variant); } )+
                Err(OcioError::Ocio(format!("native {} value {raw} is not exposed by the selected interface", stringify!($ty))))
            }
        }
    };
}

native_enum!(LoggingLevel, 1, [None, Warning, Info, Debug, Unknown]);
native_enum!(TransformDirection, 2, [Forward, Inverse]);
native_enum!(
    BitDepth,
    3,
    [Unknown, Uint8, Uint10, Uint12, Uint14, Uint16, Uint32, F16, F32]
);
native_enum!(Allocation, 4, [Unknown, Uniform, Lg2]);
native_enum!(
    Interpolation,
    5,
    [Unknown, Nearest, Linear, Tetrahedral, Cubic, Default, Best]
);
native_enum!(
    GpuLanguage,
    6,
    [
        Cg,
        Glsl1_2,
        Glsl1_3,
        Glsl4_0,
        #[cfg(feature = "v2_5")]
        GlslVk4_6,
        HlslSm5_0,
        Osl1,
        GlslEs1_0,
        GlslEs3_0,
        Msl2_0
    ]
);
native_enum!(EnvironmentMode, 7, [Unknown, LoadPredefined, LoadAll]);
native_enum!(CDLStyle, 8, [Asc, NoClamp]);
native_enum!(RangeStyle, 9, [NoClamp, Clamp]);
native_enum!(
    FixedFunctionStyle,
    10,
    [
        AcesRedMod03,
        AcesRedMod10,
        AcesGlow03,
        AcesGlow10,
        AcesDarkToDim10,
        Rec2100Surround,
        RgbToHsv,
        XyzToxyY,
        XyzTouvY,
        XyzToLuv,
        AcesGamutMap02,
        AcesGamutMap07,
        AcesGamutCompress13,
        LinToPq,
        LinToGammaLog,
        LinToDoubleLog,
        AcesOutputTransform20,
        AcesRgbToJmh20,
        AcesTonescaleCompress20,
        AcesGamutCompress20,
        #[cfg(feature = "v2_5")]
        RgbToHsyLin,
        #[cfg(feature = "v2_5")]
        RgbToHsyLog,
        #[cfg(feature = "v2_5")]
        RgbToHsyVid,
        #[cfg(feature = "v2_6")]
        AcesRgbToHmj20
    ]
);
native_enum!(GradingStyle, 11, [Log, Lin, Video]);
native_enum!(ExposureContrastStyle, 12, [Linear, Video, Logarithmic]);
native_enum!(NegativeStyle, 13, [Clamp, Mirror, PassThru, Linear]);

impl crate::BitDepth {
    /// Query whether OCIO considers the depth floating point.
    pub fn ocio_is_float(self) -> Result<bool> {
        Ok(self.ocio_utility(false)? != 0)
    }
    /// Query OCIO's integer bit count for this depth.
    pub fn ocio_bit_count(self) -> Result<i32> {
        self.ocio_utility(true)
    }
    fn ocio_utility(self, as_integer: bool) -> Result<i32> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_bit_depth_utility(self as i32, as_integer) };
        crate::ocio_call_status()?;
        Ok(value)
    }
}
impl crate::TransformDirection {
    /// Compute the inverse using native OCIO semantics.
    pub fn ocio_inverse(self) -> Result<Self> {
        self.ocio_direction(self, false)
    }
    /// Combine two directions using native OCIO semantics.
    pub fn ocio_combine(self, other: Self) -> Result<Self> {
        self.ocio_direction(other, true)
    }
    fn ocio_direction(self, other: Self, combine: bool) -> Result<Self> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_direction_utility(self as i32, other as i32, combine) };
        crate::ocio_call_status()?;
        match value {
            0 => Ok(Self::Forward),
            1 => Ok(Self::Inverse),
            _ => Err(OcioError::Ocio("unknown native transform direction".into())),
        }
    }
}
