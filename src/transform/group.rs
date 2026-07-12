use std::ffi::c_void;
use std::ptr::NonNull;

use super::{transform_from_raw_handle, Transform, TransformHandle};
use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, Config, OcioError, Result, TransformDirection,
};
use ocio_sys;

/// An ordered list of transforms evaluated as one transform.
///
/// Group transforms are useful for composing generated or user-authored
/// transform chains before asking a `Config` for a processor.
pub struct GroupTransform {
    pub(crate) handle: NonNull<c_void>,
}

impl GroupTransform {
    /// Create an empty group transform.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_group_transform_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the number of child transforms in the group.
    pub fn num_transforms(&self) -> i32 {
        unsafe { ocio_sys::ocio_group_transform_get_num_transforms(self.handle.as_ptr()) }
    }

    /// Append `child` to the end of the group.
    pub fn append_transform(&self, child: &impl TransformHandle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_group_transform_append_transform(self.handle.as_ptr(), child.as_ptr());
        }
        crate::ocio_call_status()
    }

    /// Insert `child` at the beginning of the group.
    pub fn prepend_transform(&self, child: &impl TransformHandle) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_group_transform_prepend_transform(self.handle.as_ptr(), child.as_ptr());
        }
        crate::ocio_call_status()
    }

    /// Return the child transform at `index`, if present.
    pub fn transform(&self, index: i32) -> Option<Transform> {
        let handle =
            unsafe { ocio_sys::ocio_group_transform_get_transform(self.handle.as_ptr(), index) };
        transform_from_raw_handle(handle)
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer transform()")]
    pub fn get_transform(&self, index: i32) -> Option<Transform> {
        self.transform(index)
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer transform()")]
    pub fn get_transform_v1(&self, index: i32) -> Option<Transform> {
        self.transform(index)
    }

    /// Return the transform direction used when this group is evaluated.
    pub fn direction(&self) -> TransformDirection {
        let dir = unsafe { ocio_sys::ocio_group_transform_get_direction(self.handle.as_ptr()) };
        match dir {
            1 => TransformDirection::Inverse,
            _ => TransformDirection::Forward,
        }
    }

    /// Set the transform direction used when this group is evaluated.
    pub fn set_direction(&self, direction: TransformDirection) {
        self.try_set_direction(direction)
            .expect("failed to set group transform direction");
    }

    /// Set the transform direction and surface any OCIO validation error.
    pub fn try_set_direction(&self, direction: TransformDirection) -> crate::Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_group_transform_set_direction(self.handle.as_ptr(), direction as i32);
        }
        crate::ocio_call_status()
    }

    /// Create an editable copy that is independent from the original transform.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_transform_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Remove the child transform at `index`.
    pub fn remove_transform(&self, index: usize) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_group_transform_remove_transform(self.handle.as_ptr(), index as u64)
        };
        crate::ocio_call_status()
    }

    /// Remove every child transform from the group.
    pub fn clear_transforms(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_group_transform_clear_transforms(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    /// # Safety
    /// `config`, `format_name`, and `os` must be valid pointers accepted by the OCIO ABI.
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO ostream entry point; prefer write_to_string(&Config, format_name) for Rust callers"
    )]
    pub unsafe fn write(&self, config: *mut c_void, format_name: *const i8, os: *mut c_void) {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_group_transform_write(self.handle.as_ptr(), config, format_name, os);
        }
        let _ = crate::ocio_call_status();
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
        crate::clear_last_error();
        let result = unsafe {
            cstr_from_mut(ocio_sys::ocio_group_transform_write_to_string(
                self.handle.as_ptr(),
                config.handle.as_ptr(),
                format_name.as_ptr(),
            ))
        };
        crate::ocio_call_status()?;
        Ok(result)
    }

    /// Return format metadata attached to the group, when available.
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

    /// Return the number of serialization formats supported by `write_to_string`.
    pub fn num_write_formats() -> i32 {
        unsafe { ocio_sys::ocio_group_transform_get_num_write_formats() }
    }

    /// Return the writer format name at `index`.
    pub fn format_name_by_index(index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_group_transform_get_format_name_by_index(
                index,
            ))
        }
    }

    /// Return the writer format extension at `index`, without a leading dot.
    pub fn format_extension_by_index(index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_group_transform_get_format_extension_by_index(index))
        }
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
        gt.append_transform(&ft).unwrap();
    }

    #[test]
    fn prepend_transform_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        gt.prepend_transform(&ft).unwrap();
    }

    #[test]
    fn get_transform_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        gt.append_transform(&ft).unwrap();
        let _ = gt.transform(0);
    }

    #[test]
    fn get_transform_out_of_range() {
        let gt = GroupTransform::create().unwrap();
        assert!(gt.transform(0).is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn get_transform_compat_aliases_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let _ = gt.get_transform(0);
        let _ = gt.get_transform_v1(0);
    }

    #[test]
    fn append_multiple_transforms_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let ft = FileTransform::create().unwrap();
        let ct = CDLTransform::create().unwrap();
        gt.append_transform(&ft).unwrap();
        gt.append_transform(&ct).unwrap();
    }

    #[test]
    fn append_via_enum_no_crash() {
        let gt = GroupTransform::create().unwrap();
        let t = Transform::File(FileTransform::create().unwrap());
        gt.append_transform(&t).unwrap();
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
        g.append_transform(&cdl).unwrap();
        g.remove_transform(0).unwrap();
        g.clear_transforms().unwrap();
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
        g.append_transform(&cdl).unwrap();

        let config = Config::raw().unwrap();
        let written = g.write_to_string(&config, "Academy/ASC Common LUT Format");
        assert!(written.is_ok());

        if crate::is_stub_build() {
            assert!(written.unwrap().is_none());
        }
    }

    #[test]
    fn static_write_format_queries_no_crash() {
        let count = GroupTransform::num_write_formats();
        assert!(count >= 0);
        let _ = GroupTransform::format_name_by_index(0);
        let _ = GroupTransform::format_extension_by_index(0);
    }

    #[test]
    fn static_write_format_queries_real_behavior() {
        if crate::is_stub_build() {
            return;
        }

        let count = GroupTransform::num_write_formats();
        assert!(count > 0);
        assert!(GroupTransform::format_name_by_index(0).is_some());
        assert!(GroupTransform::format_extension_by_index(0).is_some());
    }
}
