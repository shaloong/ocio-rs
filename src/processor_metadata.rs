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

impl ProcessorMetadata {
    /// Create an empty metadata container.
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_processor_metadata_create() };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    /// Return the number of source files referenced by the processor metadata.
    pub fn num_files(&self) -> i32 {
        unsafe { ocio_sys::ocio_processor_metadata_get_num_files(self.handle.as_ptr()) }
    }

    /// Return one referenced source file by index.
    pub fn file(&self, index: i32) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_processor_metadata_get_file(self.handle.as_ptr(), index)) }
    }

    /// Return the number of look names referenced by the processor metadata.
    pub fn num_looks(&self) -> i32 {
        unsafe { ocio_sys::ocio_processor_metadata_get_num_looks(self.handle.as_ptr()) }
    }

    /// Return one referenced look name by index.
    pub fn look(&self, index: i32) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_processor_metadata_get_look(self.handle.as_ptr(), index)) }
    }

    /// Append a file reference to the metadata.
    pub fn add_file(&self, file_name: impl AsRef<str>) -> Result<()> {
        let file_name = cstring(file_name)?;
        unsafe {
            ocio_sys::ocio_processor_metadata_add_file(
                self.handle.as_ptr(),
                file_name.as_ptr().cast(),
            )
        };
        Ok(())
    }

    /// Append a look reference to the metadata.
    pub fn add_look(&self, look: impl AsRef<str>) -> Result<()> {
        let look = cstring(look)?;
        unsafe { ocio_sys::ocio_processor_metadata_add_look(self.handle.as_ptr(), look.as_ptr().cast()) };
        Ok(())
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
