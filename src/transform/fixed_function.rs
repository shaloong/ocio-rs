use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{FixedFunctionStyle, OcioError, Result, TransformDirection};
use ocio_sys;

/// Wraps one of OCIO's fixed-function transform styles.
pub struct FixedFunctionTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl FixedFunctionTransform {
    /// Create a fixed-function transform with the given style using default parameters.
    pub fn create(style: FixedFunctionStyle) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_fixed_function_transform_create_with_params(
                style as i32,
                std::ptr::null(),
                0,
            )
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create a fixed-function transform with the given style and custom parameters.
    pub fn create_with_params(style: FixedFunctionStyle, params: &[f64]) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_fixed_function_transform_create_with_params(
                style as i32,
                params.as_ptr(),
                params.len(),
            )
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the current fixed-function style.
    ///
    /// For compatibility, unknown styles fall back to `AcesRedMod03`.
    /// Use [`Self::try_style`] to detect a style unavailable at this feature level.
    pub fn style(&self) -> FixedFunctionStyle {
        self.try_style().unwrap_or(FixedFunctionStyle::AcesRedMod03)
    }

    /// Return the current style, reporting unknown styles and native errors.
    ///
    /// A config may contain a style supported by the native library but not by
    /// the enabled Rust interface feature. Enable the corresponding feature
    /// to represent that style as a [`FixedFunctionStyle`].
    pub fn try_style(&self) -> Result<FixedFunctionStyle> {
        crate::clear_last_error();
        let s = unsafe {
            ocio_sys::ocio_fixed_function_transform_get_style(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        let style = match s {
            0 => FixedFunctionStyle::AcesRedMod03,
            1 => FixedFunctionStyle::AcesRedMod10,
            2 => FixedFunctionStyle::AcesGlow03,
            3 => FixedFunctionStyle::AcesGlow10,
            4 => FixedFunctionStyle::AcesDarkToDim10,
            5 => FixedFunctionStyle::Rec2100Surround,
            6 => FixedFunctionStyle::RgbToHsv,
            7 => FixedFunctionStyle::XyzToxyY,
            8 => FixedFunctionStyle::XyzTouvY,
            9 => FixedFunctionStyle::XyzToLuv,
            10 => FixedFunctionStyle::AcesGamutMap02,
            11 => FixedFunctionStyle::AcesGamutMap07,
            12 => FixedFunctionStyle::AcesGamutCompress13,
            13 => FixedFunctionStyle::LinToPq,
            14 => FixedFunctionStyle::LinToGammaLog,
            15 => FixedFunctionStyle::LinToDoubleLog,
            16 => FixedFunctionStyle::AcesOutputTransform20,
            17 => FixedFunctionStyle::AcesRgbToJmh20,
            18 => FixedFunctionStyle::AcesTonescaleCompress20,
            19 => FixedFunctionStyle::AcesGamutCompress20,
            #[cfg(feature = "v2_5")]
            20 => FixedFunctionStyle::RgbToHsyLin,
            #[cfg(feature = "v2_5")]
            21 => FixedFunctionStyle::RgbToHsyLog,
            #[cfg(feature = "v2_5")]
            22 => FixedFunctionStyle::RgbToHsyVid,
            #[cfg(feature = "v2_6")]
            23 => FixedFunctionStyle::AcesRgbToHmj20,
            _ => {
                return Err(OcioError::ValidationFailed(format!(
                    "fixed-function style {s} is unavailable at the enabled interface level"
                )))
            }
        };
        Ok(style)
    }

    /// Set the fixed-function style, panicking on validation error.
    pub fn set_style(&self, style: FixedFunctionStyle) {
        self.try_set_style(style)
            .expect("failed to set fixed-function style");
    }

    /// Set the fixed-function style and surface any OCIO validation error.
    pub fn try_set_style(&self, style: FixedFunctionStyle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_fixed_function_transform_set_style(self.handle.as_ptr(), style as i32);
        }
        crate::ocio_call_status()
    }

    /// Return the number of parameters required by the current style.
    pub fn try_num_params(&self) -> Result<i32> {
        i32::try_from(self.try_num_params_usize()?).map_err(|_| {
            OcioError::ValidationFailed("fixed-function parameter count exceeds i32::MAX".into())
        })
    }

    /// Return the full native parameter count without narrowing it to `i32`.
    pub fn try_num_params_usize(&self) -> Result<usize> {
        crate::clear_last_error();
        let v =
            unsafe { ocio_sys::ocio_fixed_function_transform_get_num_params(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(v)
    }

    /// Return the number of parameters required by the current style.
    pub fn num_params(&self) -> i32 {
        self.try_num_params().unwrap_or(0)
    }

    /// Return the style-specific parameter values.
    pub fn try_params(&self) -> Result<Vec<f64>> {
        let n = self.try_num_params_usize()?;
        if n == 0 {
            return Ok(Vec::new());
        }
        let mut params = super::reserve_native_items(n, "FixedFunctionTransform::try_params")?;
        params.resize(n, 0.0f64);
        self.read_params(&mut params)?;
        Ok(params)
    }

    /// Copy parameters into an exactly sized buffer without allocating.
    /// A length mismatch leaves the buffer unchanged.
    pub fn try_params_into(&self, output: &mut [f64]) -> Result<()> {
        let count = self.try_num_params_usize()?;
        if output.len() != count {
            return Err(OcioError::InvalidInput(
                "fixed-function parameter buffer length mismatch".into(),
            ));
        }
        if count == 0 {
            return Ok(());
        }
        self.read_params(output)
    }

    fn read_params(&self, output: &mut [f64]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_fixed_function_transform_get_params(
                self.handle.as_ptr(),
                output.as_mut_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    /// Return the style-specific parameter values.
    pub fn params(&self) -> Vec<f64> {
        self.try_params().unwrap_or_default()
    }

    /// Set the style-specific parameters for this transform.
    ///
    /// OCIO may defer style-specific validation until the transform is
    /// validated or used to construct a processor.
    pub fn set_params(&self, params: &[f64]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_fixed_function_transform_set_params(
                self.handle.as_ptr(),
                params.as_ptr(),
                params.len() as usize,
            );
        }
        crate::ocio_call_status()
    }

    /// Return the evaluation direction.
    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe {
            ocio_sys::ocio_fixed_function_transform_get_direction(
                self.handle.as_ptr() as *mut c_void
            )
        };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the evaluation direction, panicking on validation error.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set fixed function transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_fixed_function_transform_set_direction(
                self.handle.as_ptr(),
                direction as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Create an editable copy that is independent from the original transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return format metadata attached to the transform, when available.
    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer format_metadata()")]
    pub fn format_metadata_v1(&self) -> Option<crate::FormatMetadata> {
        self.format_metadata()
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer format_metadata()")]
    pub fn format_metadata_v2(&self) -> Option<crate::FormatMetadata> {
        self.format_metadata()
    }

    /// Return whether `other` is equivalent to this transform.
    pub fn equals(&self, other: &Self) -> bool {
        unsafe {
            ocio_sys::ocio_fixed_function_transform_equals(
                self.handle.as_ptr(),
                other.handle.as_ptr(),
            )
        }
    }
}

impl Drop for FixedFunctionTransform {
    fn drop(&mut self) {
        unsafe {
            ocio_sys::ocio_fixed_function_transform_destroy(self.handle.as_ptr() as *mut c_void)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_fixed_function() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03);
        assert!(ft.is_ok());
    }

    #[test]
    fn style_no_crash() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        assert_eq!(ft.try_style().unwrap(), FixedFunctionStyle::AcesRedMod03);
        ft.try_set_style(FixedFunctionStyle::RgbToHsv).unwrap();
    }

    #[test]
    #[cfg(not(feature = "v2_5"))]
    #[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.5 or later")]
    fn newer_native_style_is_not_misreported_by_fallible_query() {
        if crate::version_hex() < 0x0205_0000 {
            return;
        }
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_fixed_function_transform_create_with_params(20, std::ptr::null(), 0)
        };
        let ft = FixedFunctionTransform {
            handle: crate::handle_result(handle).unwrap(),
        };
        assert!(matches!(
            ft.try_style(),
            Err(OcioError::ValidationFailed(_))
        ));
        assert_eq!(ft.style(), FixedFunctionStyle::AcesRedMod03);
    }

    #[test]
    #[cfg(not(feature = "v2_6"))]
    #[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO 2.6 or later")]
    fn native_26_hmj_is_not_misreported_at_older_interface_levels() {
        if crate::version_hex() < 0x0206_0000 {
            return;
        }
        let params = [0.7347, 0.2653, 0.0, 1.0, 0.0001, -0.0770, 0.32168, 0.33767];
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_fixed_function_transform_create_with_params(
                23,
                params.as_ptr(),
                params.len(),
            )
        };
        let ft = FixedFunctionTransform {
            handle: crate::handle_result(handle).unwrap(),
        };
        assert!(matches!(
            ft.try_style(),
            Err(OcioError::ValidationFailed(_))
        ));
        assert_eq!(ft.style(), FixedFunctionStyle::AcesRedMod03);
    }

    #[test]
    fn params_no_crash() {
        let ft =
            FixedFunctionTransform::create_with_params(FixedFunctionStyle::Rec2100Surround, &[1.0])
                .unwrap();
        let _ = ft.num_params();
        let _ = ft.params();
        ft.set_params(&[1.0]).unwrap();
    }

    #[test]
    fn direction_no_crash() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let _ = ft.direction();
        ft.set_direction(TransformDirection::Inverse);
    }

    #[test]
    fn create_with_params() {
        let ft =
            FixedFunctionTransform::create_with_params(FixedFunctionStyle::Rec2100Surround, &[1.0]);
        assert!(ft.is_ok());
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let _ = ft.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let ft = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let _ = ft.format_metadata();
    }

    #[test]
    fn equals_no_crash() {
        let a = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let b = FixedFunctionTransform::create(FixedFunctionStyle::AcesRedMod03).unwrap();
        let _ = a.equals(&b);
    }
}
