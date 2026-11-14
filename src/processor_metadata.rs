use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstring, OcioError, Result};
use ocio_sys;

/// Technical metadata collected while OCIO builds a processor.
///
/// This tracks source file references and look names that contributed to a
/// processor. It is distinct from [`crate::FormatMetadata`], which models CLF /
/// CTF-style metadata trees.
pub struct ProcessorMetadata {
    pub(crate) handle: NonNull<c_void>,
}

/// Shared read-only native processor metadata.
pub struct ConstProcessorMetadata {
    pub(crate) handle: NonNull<c_void>,
}
impl ConstProcessorMetadata {
    fn count(&self, looks: bool) -> Result<i32> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_const_processor_metadata_count(self.handle.as_ptr(), looks) };
        crate::ocio_call_status()?;
        Ok(value)
    }
    fn entry(&self, looks: bool, index: i32) -> Result<Option<&std::ffi::CStr>> {
        crate::clear_last_error();
        let ptr = unsafe {
            ocio_sys::ocio_const_processor_metadata_entry(self.handle.as_ptr(), looks, index)
        };
        crate::ocio_call_status()?;
        Ok(if ptr.is_null() {
            None
        } else {
            Some(unsafe { std::ffi::CStr::from_ptr(ptr) })
        })
    }
    /// Number of source files.
    pub fn num_files(&self) -> Result<i32> {
        self.count(false)
    }
    /// Borrow a source file name.
    pub fn file(&self, index: i32) -> Result<Option<&std::ffi::CStr>> {
        self.entry(false, index)
    }
    /// Number of looks.
    pub fn num_looks(&self) -> Result<i32> {
        self.count(true)
    }
    /// Borrow a look name.
    pub fn look(&self, index: i32) -> Result<Option<&std::ffi::CStr>> {
        self.entry(true, index)
    }
    /// Copy entries into an independent editable metadata object.
    pub fn create_editable_copy(&self) -> Result<ProcessorMetadata> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_const_processor_metadata_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| ProcessorMetadata { handle })
    }
}
impl Drop for ConstProcessorMetadata {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_const_processor_metadata_destroy(self.handle.as_ptr()) }
    }
}

impl ProcessorMetadata {
    /// Create an empty metadata container.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_processor_metadata_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the number of source files referenced by the processor metadata.
    pub fn num_files(&self) -> i32 {
        unsafe { ocio_sys::ocio_processor_metadata_get_num_files(self.handle.as_ptr()) }
    }

    /// Return one referenced source file by index.
    pub fn file(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_processor_metadata_get_file(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Return the number of look names referenced by the processor metadata.
    pub fn num_looks(&self) -> i32 {
        unsafe { ocio_sys::ocio_processor_metadata_get_num_looks(self.handle.as_ptr()) }
    }

    /// Return one referenced look name by index.
    pub fn look(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_processor_metadata_get_look(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    /// Append a file reference to the metadata.
    pub fn add_file(&self, file_name: impl AsRef<str>) -> Result<()> {
        let file_name = cstring(file_name)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_processor_metadata_add_file(
                self.handle.as_ptr(),
                file_name.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Append a look reference to the metadata.
    pub fn add_look(&self, look: impl AsRef<str>) -> Result<()> {
        let look = cstring(look)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_processor_metadata_add_look(self.handle.as_ptr(), look.as_ptr().cast())
        };
        crate::ocio_call_status()
    }
}

impl Drop for ProcessorMetadata {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_processor_metadata_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_processor_metadata() {
        assert!(ProcessorMetadata::create().is_ok());
    }

    #[test]
    fn processor_metadata_mutation_no_crash() {
        let metadata = ProcessorMetadata::create().unwrap();
        let _ = metadata.num_files();
        let _ = metadata.file(0);
        let _ = metadata.num_looks();
        let _ = metadata.look(0);
        assert!(metadata.add_file("lut.clf").is_ok());
        assert!(metadata.add_look("film").is_ok());
    }
}
