use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, CDLStyle, Interpolation, OcioError, Result,
    TransformDirection,
};
use ocio_sys;

/// A transform backed by an external LUT or color-correction file.
///
/// The source path is resolved by OCIO using the active config/context.
pub struct FileTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl FileTransform {
    /// Create an empty file transform.
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_file_transform_create() };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    /// Return the source path or URI attached to the transform.
    pub fn src(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_file_transform_get_src(self.handle.as_ptr())) }
    }

    /// Set the source path or URI used to load the external transform data.
    pub fn set_src(&self, src: impl AsRef<str>) -> Result<()> {
        let src = cstring(src)?;
        unsafe { ocio_sys::ocio_file_transform_set_src(self.handle.as_ptr(), src.as_ptr().cast()) };
        Ok(())
    }

    /// Return the optional CCC identifier used with multi-grade CDL files.
    pub fn ccc_id(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_file_transform_get_ccc_id(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the optional CCC identifier used with multi-grade CDL files.
    pub fn set_ccc_id(&self, id: impl AsRef<str>) -> Result<()> {
        let id = cstring(id)?;
        unsafe {
            ocio_sys::ocio_file_transform_set_ccc_id(self.handle.as_ptr(), id.as_ptr().cast())
        };
        Ok(())
    }

    /// Return the interpolation mode requested for LUT sampling.
    pub fn interpolation(&self) -> Interpolation {
        let interp =
            unsafe { ocio_sys::ocio_file_transform_get_interpolation(self.handle.as_ptr()) };
        match interp {
            1 => Interpolation::Nearest,
            2 => Interpolation::Linear,
            3 => Interpolation::Tetrahedral,
            4 => Interpolation::Cubic,
            5 => Interpolation::Default,
            6 => Interpolation::Best,
            _ => Interpolation::Unknown,
        }
    }

    /// Set the interpolation mode requested for LUT sampling.
    pub fn set_interpolation(&self, interp: Interpolation) {
        unsafe {
            ocio_sys::ocio_file_transform_set_interpolation(self.handle.as_ptr(), interp as i32);
        }
    }

    /// Return the CDL style used when the file source is CDL-based.
    pub fn cdl_style(&self) -> CDLStyle {
        let s = unsafe { ocio_sys::ocio_file_transform_get_cdl_style(self.handle.as_ptr()) };
        match s {
            1 => CDLStyle::NoClamp,
            _ => CDLStyle::Asc,
        }
    }

    /// Set the CDL style used when the file source is CDL-based.
    pub fn set_cdl_style(&self, style: CDLStyle) {
        unsafe {
            ocio_sys::ocio_file_transform_set_cdl_style(self.handle.as_ptr(), style as i32);
        }
    }

    /// Return the transform direction used when this op is evaluated.
    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_file_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the transform direction used when this op is evaluated.
    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_file_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    /// Create an editable copy that is independent from the original transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle =
            unsafe { ocio_sys::ocio_file_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    /// Return format metadata attached to the transform, when available.
    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }

    /// Ask OCIO to validate the transform in place.
    pub fn validate(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_file_transform_validate(self.handle.as_ptr()) };
        crate::validation_status()
    }

    /// Return the number of reader formats supported by `FileTransform`.
    pub fn num_formats() -> i32 {
        unsafe { ocio_sys::ocio_file_transform_get_num_formats() }
    }

    /// Return the reader format name at `index`.
    pub fn format_name_by_index(index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_file_transform_get_format_name_by_index(
                index,
            ))
        }
    }

    /// Return the reader format extension at `index`, without a leading dot.
    pub fn format_extension_by_index(index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_file_transform_get_format_extension_by_index(
                index,
            ))
        }
    }

    /// Return whether `extension` is recognized by the upstream file readers.
    pub fn is_format_extension_supported(extension: impl AsRef<str>) -> bool {
        let extension = match cstring(extension) {
            Ok(value) => value,
            Err(_) => return false,
        };
        unsafe {
            ocio_sys::ocio_file_transform_is_format_extension_supported(extension.as_ptr().cast())
        }
    }
}

impl Drop for FileTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_file_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_file_transform() {
        let ft = FileTransform::create();
        assert!(ft.is_ok());
    }

    #[test]
    fn set_src_no_crash() {
        let ft = FileTransform::create().unwrap();
        assert!(ft.set_src("test.lut").is_ok());
    }

    #[test]
    fn interpolation_no_crash() {
        let ft = FileTransform::create().unwrap();
        ft.set_interpolation(Interpolation::Linear);
        let _ = ft.interpolation();
    }

    #[test]
    fn direction_no_crash() {
        let ft = FileTransform::create().unwrap();
        ft.set_direction(TransformDirection::Inverse);
        let _ = ft.direction();
    }

    #[test]
    fn cdl_style_no_crash() {
        let ft = FileTransform::create().unwrap();
        let _ = ft.cdl_style();
        ft.set_cdl_style(CDLStyle::NoClamp);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let ft = FileTransform::create().unwrap();
        let _ = ft.create_editable_copy();
    }

    #[test]
    fn format_metadata_no_crash() {
        let ft = FileTransform::create().unwrap();
        let _ = ft.format_metadata();
    }

    #[test]
    fn validate_no_crash() {
        let ft = FileTransform::create().unwrap();
        let _ = ft.validate();
    }

    #[test]
    fn static_format_queries_no_crash() {
        let count = FileTransform::num_formats();
        assert!(count >= 0);
        let _ = FileTransform::format_name_by_index(0);
        let _ = FileTransform::format_extension_by_index(0);
        let _ = FileTransform::is_format_extension_supported("clf");
    }

    #[test]
    fn static_format_queries_real_behavior() {
        if crate::is_stub_build() {
            return;
        }

        let count = FileTransform::num_formats();
        assert!(count > 0);
        assert!(FileTransform::format_name_by_index(0).is_some());
        assert!(FileTransform::format_extension_by_index(0).is_some());
        assert!(FileTransform::is_format_extension_supported("clf"));
        assert!(FileTransform::is_format_extension_supported(".clf"));
        assert!(!FileTransform::is_format_extension_supported(
            "definitely_not_a_lut_ext"
        ));
    }
}
