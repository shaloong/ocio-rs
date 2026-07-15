use std::ffi::c_void;
use std::fs;
use std::ptr::NonNull;

use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, Config, FormatMetadata, OcioError, Result,
};
use ocio_sys;

/// Builds LUT and shader outputs from a [`Config`] using OCIO's baker API.
pub struct Baker {
    handle: NonNull<c_void>,
}

impl Baker {
    /// Create a new, unconfigured `Baker` instance.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_baker_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an editable deep copy of this baker.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_baker_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Attach the OCIO config used to resolve Baker color spaces and views.
    pub fn set_config(&self, config: &Config) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_baker_set_config(self.handle.as_ptr(), config.handle.as_ptr());
        }
        crate::ocio_call_status()
    }

    /// Get the config currently attached to this baker.
    pub fn config(&self) -> Result<Config> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_baker_get_config(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Config { handle })
    }

    /// Get the output format name (e.g. `"resolve_cube"`).
    pub fn format(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_baker_get_format(self.handle.as_ptr())) }
    }

    /// Set the output format by name.
    pub fn set_format(&self, format_name: impl AsRef<str>) -> Result<()> {
        let name = cstring(format_name)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_baker_set_format(self.handle.as_ptr(), name.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the input color space name.
    pub fn input_space(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_baker_get_input_space(self.handle.as_ptr())) }
    }

    /// Set the input color space by name.
    pub fn set_input_space(&self, space: impl AsRef<str>) -> Result<()> {
        let s = cstring(space)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_baker_set_input_space(self.handle.as_ptr(), s.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the shaper color space name used for the optional shaper LUT.
    pub fn shaper_space(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_baker_get_shaper_space(self.handle.as_ptr())) }
    }

    /// Set the shaper color space by name.
    pub fn set_shaper_space(&self, space: impl AsRef<str>) -> Result<()> {
        let s = cstring(space)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_baker_set_shaper_space(self.handle.as_ptr(), s.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the looks string applied during baking.
    pub fn looks(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_baker_get_looks(self.handle.as_ptr())) }
    }

    /// Set the looks string to apply during baking.
    pub fn set_looks(&self, looks: impl AsRef<str>) -> Result<()> {
        let s = cstring(looks)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_baker_set_looks(self.handle.as_ptr(), s.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the target color space name.
    pub fn target_space(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_baker_get_target_space(self.handle.as_ptr())) }
    }

    /// Set the target color space by name.
    pub fn set_target_space(&self, space: impl AsRef<str>) -> Result<()> {
        let s = cstring(space)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_baker_set_target_space(self.handle.as_ptr(), s.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the display name used for display/view baking.
    pub fn display(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_baker_get_display(self.handle.as_ptr())) }
    }

    /// Get the view name used for display/view baking.
    pub fn view(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_baker_get_view(self.handle.as_ptr())) }
    }

    /// Set the display and view names used for baking.
    pub fn set_display_view(&self, display: impl AsRef<str>, view: impl AsRef<str>) -> Result<()> {
        let d = cstring(display)?;
        let v = cstring(view)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_baker_set_display_view(
                self.handle.as_ptr(),
                d.as_ptr().cast(),
                v.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    /// Get the number of samples in the optional shaper LUT.
    pub fn shaper_size(&self) -> i32 {
        unsafe { ocio_sys::ocio_baker_get_shaper_size(self.handle.as_ptr()) }
    }

    /// Set the number of samples used for the optional shaper LUT.
    pub fn set_shaper_size(&self, size: i32) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_baker_set_shaper_size(self.handle.as_ptr(), size) };
        crate::ocio_call_status()
    }

    /// Get the edge length of the generated cube LUT.
    pub fn cube_size(&self) -> i32 {
        unsafe { ocio_sys::ocio_baker_get_cube_size(self.handle.as_ptr()) }
    }

    /// Set the edge length used for the generated cube LUT.
    pub fn set_cube_size(&self, size: i32) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_baker_set_cube_size(self.handle.as_ptr(), size) };
        crate::ocio_call_status()
    }

    /// Bake the configured output to an in-memory string.
    ///
    /// Returns `Ok(None)` in stub builds where no real OCIO baker is linked.
    ///
    /// Returns an error when OCIO cannot bake the configured output.
    pub fn bake_to_string(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let text =
            unsafe { cstr_from_mut(ocio_sys::ocio_baker_bake_to_string(self.handle.as_ptr())) };
        crate::ocio_call_status()?;
        Ok(text)
    }

    /// Bake the configured output and write it to `output_path`.
    ///
    /// In stub builds, no output text is generated and this method returns `Ok(())`
    /// without writing a file.
    pub fn bake(&self, output_path: impl AsRef<str>) -> Result<()> {
        let Some(contents) = self.bake_to_string()? else {
            return Ok(());
        };
        fs::write(output_path.as_ref(), contents).map_err(|e| OcioError::Ocio(e.to_string()))?;
        Ok(())
    }

    /// Get the format metadata attached to this baker.
    pub fn format_metadata(&self) -> Option<FormatMetadata> {
        let handle = unsafe { ocio_sys::ocio_baker_get_format_metadata(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| FormatMetadata { handle: h })
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer format_metadata()")]
    pub fn format_metadata_v1(&self) -> Option<FormatMetadata> {
        self.format_metadata()
    }

    // --- Static format metadata ---

    /// Get the number of available output formats.
    pub fn num_formats() -> i32 {
        unsafe { ocio_sys::ocio_baker_get_num_formats() }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_formats()")]
    pub fn get_num_formats() -> i32 {
        Self::num_formats()
    }

    /// Get the format name at the given index.
    pub fn format_name_by_index(index: i32) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_baker_get_format_name_by_index(index)) }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer format_name_by_index()")]
    pub fn get_format_name_by_index(index: i32) -> Option<String> {
        Self::format_name_by_index(index)
    }

    /// Get the file extension for the format at the given index.
    pub fn format_extension_by_index(index: i32) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_baker_get_format_extension_by_index(index)) }
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer format_extension_by_index()"
    )]
    pub fn get_format_extension_by_index(index: i32) -> Option<String> {
        Self::format_extension_by_index(index)
    }
}

impl Drop for Baker {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_baker_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_baker() {
        let baker = Baker::create();
        assert!(baker.is_ok());
    }

    #[test]
    fn baker_methods_no_crash() {
        let baker = Baker::create().unwrap();
        let _ = baker.format();
        let _ = baker.input_space();
        let _ = baker.target_space();
        let _ = baker.shaper_size();
        let _ = baker.cube_size();
    }

    #[test]
    fn format_metadata_no_crash() {
        let baker = Baker::create().unwrap();
        let md = baker.format_metadata();
        assert!(md.is_some());
    }

    #[test]
    fn create_editable_copy_round_trip() {
        let baker = Baker::create().unwrap();
        let _ = baker.set_format("resolve_cube");
        let copy = baker.create_editable_copy().unwrap();
        if !crate::is_stub_build() {
            assert_eq!(copy.format().as_deref(), Some("resolve_cube"));
        }
    }

    #[test]
    fn bake_to_string_no_crash() {
        let baker = Baker::create().unwrap();
        if !crate::is_stub_build() {
            let config = Config::raw().unwrap();
            baker.set_config(&config).unwrap();
            if Baker::num_formats() > 0 {
                if let Some(format) = Baker::format_name_by_index(0) {
                    let _ = baker.set_format(format);
                }
            }
            let _ = baker.set_input_space("raw");
            let _ = baker.set_target_space("raw");
        }
        let baked = baker.bake_to_string().unwrap();
        if crate::is_stub_build() {
            assert!(baked.is_none());
        }
    }

    #[test]
    fn static_format_metadata() {
        let num = Baker::num_formats();
        assert!(num >= 0);
        if num > 0 {
            let name = Baker::format_name_by_index(0);
            let ext = Baker::format_extension_by_index(0);
            assert!(name.is_some());
            assert!(ext.is_some());
        }
    }

    #[test]
    #[allow(deprecated)]
    fn static_format_metadata_compat_aliases_no_crash() {
        let num = Baker::get_num_formats();
        assert!(num >= 0);
        if num > 0 {
            let _ = Baker::get_format_name_by_index(0);
            let _ = Baker::get_format_extension_by_index(0);
        }
    }
}
