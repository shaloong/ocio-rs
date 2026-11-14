use ocio_rs::{
    Allocation, BitDepth, CDLStyle, EnvironmentMode, ExposureContrastStyle, FixedFunctionStyle,
    GpuLanguage, GradingStyle, Interpolation, LoggingLevel, NegativeStyle, OcioEnumText,
    RangeStyle, TransformDirection,
};
use std::ffi::CString;

fn round_trip<T: OcioEnumText + std::fmt::Debug + PartialEq>(value: T) {
    let name = CString::new(value.to_ocio_string().unwrap()).unwrap();
    assert_eq!(T::from_ocio_string(&name).unwrap(), value);
}

#[test]
#[allow(deprecated)]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn native_enum_conversions_preserve_ocio_semantics() {
    assert!(bool::from_ocio_string(c"YES").unwrap());
    assert!(!bool::from_ocio_string(c"not-a-boolean").unwrap());
    assert_eq!(true.to_ocio_string().unwrap(), "true");
    let mut buffer = [0u8; 32];
    let n = GpuLanguage::HlslSm5_0
        .write_ocio_string(&mut buffer)
        .unwrap();
    assert_eq!(&buffer[..n], b"hlsl_sm_5.0");
    assert!(GpuLanguage::HlslSm5_0
        .write_ocio_string(&mut [0; 2])
        .is_err());
    assert_eq!(
        LoggingLevel::from_ocio_string(c"2").unwrap(),
        LoggingLevel::Info
    );
    assert_eq!(
        LoggingLevel::from_ocio_string(c"invalid").unwrap(),
        LoggingLevel::Unknown
    );
    assert!(TransformDirection::from_ocio_string(c"invalid").is_err());
    assert_eq!(
        BitDepth::from_ocio_string(c"invalid").unwrap(),
        BitDepth::Unknown
    );
    assert_eq!(
        Interpolation::from_ocio_string(c"invalid").unwrap(),
        Interpolation::Unknown
    );
    assert_eq!(
        TransformDirection::Forward.ocio_inverse().unwrap(),
        TransformDirection::Inverse
    );
    assert_eq!(
        TransformDirection::Inverse
            .ocio_combine(TransformDirection::Inverse)
            .unwrap(),
        TransformDirection::Forward
    );
    assert!(BitDepth::F32.ocio_is_float().unwrap());
    assert!(!BitDepth::Uint10.ocio_is_float().unwrap());
    assert_eq!(BitDepth::Uint10.ocio_bit_count().unwrap(), 10);
    assert_eq!(BitDepth::F32.ocio_bit_count().unwrap(), 0);
    assert_eq!(Interpolation::Default.to_ocio_string().unwrap(), "default");
    assert_eq!(
        Interpolation::from_ocio_string(c"default").unwrap(),
        Interpolation::Unknown
    );
    assert!(FixedFunctionStyle::AcesGamutMap02.to_ocio_string().is_err());
    for value in [
        BitDepth::Unknown,
        BitDepth::Uint8,
        BitDepth::Uint10,
        BitDepth::Uint12,
        BitDepth::Uint14,
        BitDepth::Uint16,
        BitDepth::Uint32,
        BitDepth::F16,
        BitDepth::F32,
    ] {
        round_trip(value);
    }
    for value in [
        Interpolation::Unknown,
        Interpolation::Nearest,
        Interpolation::Linear,
        Interpolation::Tetrahedral,
        Interpolation::Cubic,
        Interpolation::Best,
    ] {
        round_trip(value);
    }
    for value in [
        GpuLanguage::Cg,
        GpuLanguage::Glsl1_2,
        GpuLanguage::Glsl1_3,
        GpuLanguage::Glsl4_0,
        GpuLanguage::HlslSm5_0,
        GpuLanguage::Osl1,
        GpuLanguage::GlslEs1_0,
        GpuLanguage::GlslEs3_0,
        GpuLanguage::Msl2_0,
    ] {
        round_trip(value);
    }
    for value in [
        FixedFunctionStyle::AcesRedMod03,
        FixedFunctionStyle::AcesRedMod10,
        FixedFunctionStyle::AcesGlow03,
        FixedFunctionStyle::AcesGlow10,
        FixedFunctionStyle::AcesDarkToDim10,
        FixedFunctionStyle::Rec2100Surround,
        FixedFunctionStyle::RgbToHsv,
        FixedFunctionStyle::XyzToxyY,
        FixedFunctionStyle::XyzTouvY,
        FixedFunctionStyle::XyzToLuv,
        FixedFunctionStyle::AcesGamutCompress13,
        FixedFunctionStyle::LinToPq,
        FixedFunctionStyle::LinToGammaLog,
        FixedFunctionStyle::LinToDoubleLog,
        FixedFunctionStyle::AcesOutputTransform20,
        FixedFunctionStyle::AcesRgbToJmh20,
        FixedFunctionStyle::AcesTonescaleCompress20,
        FixedFunctionStyle::AcesGamutCompress20,
    ] {
        round_trip(value);
    }
    round_trip(LoggingLevel::Warning);
    round_trip(TransformDirection::Inverse);
    round_trip(BitDepth::F16);
    round_trip(Allocation::Lg2);
    round_trip(Interpolation::Tetrahedral);
    round_trip(GpuLanguage::HlslSm5_0);
    round_trip(GpuLanguage::Msl2_0);
    round_trip(EnvironmentMode::LoadAll);
    round_trip(CDLStyle::NoClamp);
    round_trip(RangeStyle::NoClamp);
    round_trip(FixedFunctionStyle::AcesRedMod03);
    round_trip(GradingStyle::Log);
    round_trip(ExposureContrastStyle::Video);
    round_trip(NegativeStyle::Mirror);
    #[cfg(feature = "v2_5")]
    for value in [
        FixedFunctionStyle::RgbToHsyLin,
        FixedFunctionStyle::RgbToHsyLog,
        FixedFunctionStyle::RgbToHsyVid,
    ] {
        round_trip(value);
    }
    #[cfg(feature = "v2_5")]
    round_trip(GpuLanguage::GlslVk4_6);
    #[cfg(feature = "v2_6")]
    round_trip(FixedFunctionStyle::AcesRgbToHmj20);
    #[cfg(not(feature = "v2_6"))]
    if ocio_rs::version_hex() >= 0x02060000 {
        assert!(FixedFunctionStyle::from_ocio_string(c"ACES2_RGB_TO_HMJ").is_err());
    }
    assert!(LoggingLevel::Trace.to_ocio_string().is_err());
}
