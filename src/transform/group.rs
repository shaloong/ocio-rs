use std::ffi::c_void;
use std::ptr::NonNull;

use super::{transform_from_raw_handle, Transform, TransformHandle};
use crate::{cstr_from_mut, cstring, Config, OcioError, Result, TransformDirection};
use ocio_sys;

/// An ordered list of transforms evaluated as one transform.
///
/// Group transforms are useful for composing generated or user-authored
/// transform chains before asking a `Config` for a processor.
pub struct GroupTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl GroupTransform {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_group_transform_create() };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn num_transforms(&self) -> i32 {
        unsafe { ocio_sys::ocio_group_transform_get_num_transforms(self.handle.as_ptr()) }
    }

    pub fn append_transform(&self, child: &impl TransformHandle) {
        unsafe {
            ocio_sys::ocio_group_transform_append_transform(self.handle.as_ptr(), child.as_ptr());
        }
    }

    pub fn prepend_transform(&self, child: &impl TransformHandle) {
        unsafe {
            ocio_sys::ocio_group_transform_prepend_transform(self.handle.as_ptr(), child.as_ptr());
        }
    }

    pub fn get_transform(&self, index: i32) -> Option<Transform> {
        let handle =
            unsafe { ocio_sys::ocio_group_transform_get_transform(self.handle.as_ptr(), index) };
        transform_from_raw_handle(handle)
    }

    pub fn get_transform_v1(&self, index: i32) -> Option<Transform> {
        self.get_transform(index)
    }

    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_group_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    pub fn set_direction(&self, direction: TransformDirection) {
        unsafe {
            ocio_sys::ocio_group_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn remove_transform(&self, index: usize) {
        unsafe {
            ocio_sys::ocio_group_transform_remove_transform(self.handle.as_ptr(), index as u64)
        };
    }

    pub fn clear_transforms(&self) {
        unsafe { ocio_sys::ocio_group_transform_clear_transforms(self.handle.as_ptr()) };
    }

    /// # Safety
    /// `config`, `format_name`, and `os` must be valid pointers accepted by the OCIO ABI.
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO ostream entry point; prefer write_to_string(&Config, format_name) for Rust callers"
    )]
    pub unsafe fn write(&self, config: *mut c_void, format_name: *const i8, os: *mut c_void) {
        unsafe {
            ocio_sys::ocio_group_transform_write(self.handle.as_ptr(), config, format_name, os);
        }
    }

    /// Serialize this group transform using OCIO's writer for `format_name`.
    ///
    /// Returns `None` in stub builds where no real OCIO serializer is linked.
    pub fn write_to_string(
        &self,
        config: &Config,
        format_name: impl AsRef<str>,
    ) -> Result<Option<String>> {
        let format_name = cstring(format_name)?;
        Ok(unsafe {
            cstr_from_mut(ocio_sys::ocio_group_transform_write_to_string(
                self.handle.as_ptr(),
                config.handle.as_ptr(),
                format_name.as_ptr(),
            ))
        })
    }

    pub fn format_metadata(&self) -> Option<crate::FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_transform_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| crate::FormatMetadata { handle: h })
    }

    pub fn format_metadata_v1(&self) -> Option<crate::FormatMetadata> {
        self.format_metadata()
    }

    pub fn format_metadata_v2(&self) -> Option<crate::FormatMetadata> {
        self.format_metadata()
    }
}

impl Drop for GroupTransform {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_group_transform_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::{CDLTransform, FileTransform};

    #[test]
    fn create_group() {
        let gt = GroupTransform::create();
        assert!(gt.is_ok());
    }

    #[test]
    fn num_transforms() {
        let gt = GroupTransform::create().unwrap();
        let _ = gt.num_transforms();
    }

    #[test]
    fn append_transform_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        gt.append_transform(&ft);
    }

    #[test]
    fn prepend_transform_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        gt.prepend_transform(&ft);
    }

    #[test]
    fn get_transform_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        gt.append_transform(&ft);
        let _ = gt.get_transform(0);
    }

    #[test]
    fn get_transform_out_of_range() {
        let gt = GroupTransform::create().unwrap();
        assert!(gt.get_transform(0).is_none());
    }

    #[test]
    fn append_multiple_transforms_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        let ct = CDLTransform::create().unwrap();
        gt.append_transform(&ft);
        gt.append_transform(&ct);
    }

    #[test]
    fn append_via_enum_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let t = Transform::File(FileTransform::create().unwrap());
        gt.append_transform(&t);
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let _ = gt.create_editable_copy();
    }

    #[test]
    fn remove_clear_no_crash() {
        let g = GroupTransform::create().unwrap();
        let cdl = CDLTransform::create().unwrap();
        g.append_transform(&cdl);
        g.remove_transform(0);
        g.clear_transforms();
    }

    #[test]
    fn format_metadata_no_crash() {
        let g = GroupTransform::create().unwrap();
        let _ = g.format_metadata();
    }

    #[test]
    fn write_to_string_no_crash() {
        let g = GroupTransform::create().unwrap();
        let cdl = CDLTransform::create().unwrap();
        g.append_transform(&cdl);

        let config = Config::raw().unwrap();
        let written = g.write_to_string(&config, "Academy/ASC Common LUT Format");
        assert!(written.is_ok());

        if crate::is_stub_build() {
            assert!(written.unwrap().is_none());
        }
    }
}
