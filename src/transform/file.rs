use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, OcioError, Result, TransformDirection, Interpolation, CDLStyle};

pub struct FileTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl FileTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_file_transform_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn src(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_file_transform_get_src(self.handle.as_ptr())) }
    }

    pub fn set_src(&self, src: impl AsRef<str>) -> Result<()> {
        let src = cstring(src)?;
        unsafe { ocio_sys::ocio_file_transform_set_src(self.handle.as_ptr(), src.as_ptr().cast()) };
        Ok(())
    }

    pub fn ccc_id(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_file_transform_get_ccc_id(self.handle.as_ptr())) }
    }

    pub fn set_ccc_id(&self, id: impl AsRef<str>) -> Result<()> {
        let id = cstring(id)?;
        unsafe { ocio_sys::ocio_file_transform_set_ccc_id(self.handle.as_ptr(), id.as_ptr().cast()) };
        Ok(())
    }

    pub fn interpolation(&self) -> Interpolation {
        let interp = unsafe { ocio_sys::ocio_file_transform_get_interpolation(self.handle.as_ptr()) };
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

    pub fn set_interpolation(&self, interp: Interpolation) {
        unsafe {
            ocio_sys::ocio_file_transform_set_interpolation(self.handle.as_ptr(), interp as i32);
        }
    }

    pub fn cdl_style(&self) -> CDLStyle {
        let s = unsafe { ocio_sys::ocio_file_transform_get_cdl_style(self.handle.as_ptr()) };
        match s { 1 => CDLStyle::NoClamp, _ => CDLStyle::Asc }
    }

    pub fn set_cdl_style(&self, style: CDLStyle) {
        unsafe { ocio_sys::ocio_file_transform_set_cdl_style(self.handle.as_ptr(), style as i32); }
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_file_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_file_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
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
}
