use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, OcioError, Result, ReferenceSpaceType, BitDepth, Allocation, ColorSpaceDirection};
use crate::transform::{TransformHandle, Transform, transform_from_raw_handle};

pub struct ColorSpace {
    handle: NonNull<c_void>,
}

impl ColorSpace {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_color_space_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_color_space_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn name(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_name(self.handle.as_ptr())) }
    }

    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe { ocio_sys::ocio_color_space_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        Ok(())
    }

    pub fn family(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_family(self.handle.as_ptr())) }
    }

    pub fn set_family(&self, family: impl AsRef<str>) -> Result<()> {
        let f = cstring(family)?;
        unsafe { ocio_sys::ocio_color_space_set_family(self.handle.as_ptr(), f.as_ptr().cast()) };
        Ok(())
    }

    pub fn equality_group(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_equality_group(self.handle.as_ptr())) }
    }

    pub fn set_equality_group(&self, group: impl AsRef<str>) -> Result<()> {
        let g = cstring(group)?;
        unsafe { ocio_sys::ocio_color_space_set_equality_group(self.handle.as_ptr(), g.as_ptr().cast()) };
        Ok(())
    }

    pub fn description(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_description(self.handle.as_ptr())) }
    }

    pub fn set_description(&self, description: impl AsRef<str>) -> Result<()> {
        let d = cstring(description)?;
        unsafe { ocio_sys::ocio_color_space_set_description(self.handle.as_ptr(), d.as_ptr().cast()) };
        Ok(())
    }

    pub fn bit_depth(&self) -> BitDepth {
        let b = unsafe { ocio_sys::ocio_color_space_get_bit_depth(self.handle.as_ptr()) };
        match b {
            1 => BitDepth::Uint8,
            2 => BitDepth::Uint10,
            3 => BitDepth::Uint12,
            4 => BitDepth::Uint14,
            5 => BitDepth::Uint16,
            6 => BitDepth::Uint32,
            7 => BitDepth::F16,
            8 => BitDepth::F32,
            _ => BitDepth::Unknown,
        }
    }

    pub fn set_bit_depth(&self, bit_depth: BitDepth) {
        unsafe { ocio_sys::ocio_color_space_set_bit_depth(self.handle.as_ptr(), bit_depth as i32) };
    }

    pub fn reference_space_type(&self) -> ReferenceSpaceType {
        let r = unsafe { ocio_sys::ocio_color_space_get_reference_space_type(self.handle.as_ptr()) };
        match r { 1 => ReferenceSpaceType::Display, _ => ReferenceSpaceType::Scene }
    }

    pub fn is_data(&self) -> bool {
        unsafe { ocio_sys::ocio_color_space_is_data(self.handle.as_ptr()) }
    }

    pub fn set_is_data(&self, is_data: bool) {
        unsafe { ocio_sys::ocio_color_space_set_is_data(self.handle.as_ptr(), is_data) };
    }

    pub fn allocation(&self) -> Allocation {
        let a = unsafe { ocio_sys::ocio_color_space_get_allocation(self.handle.as_ptr()) };
        match a {
            1 => Allocation::Uniform,
            2 => Allocation::Lg2,
            _ => Allocation::Unknown,
        }
    }

    pub fn set_allocation(&self, allocation: Allocation) {
        unsafe { ocio_sys::ocio_color_space_set_allocation(self.handle.as_ptr(), allocation as i32) };
    }

    pub fn allocation_num_vars(&self) -> i32 {
        unsafe { ocio_sys::ocio_color_space_get_allocation_num_vars(self.handle.as_ptr()) }
    }

    pub fn allocation_vars(&self) -> Vec<f32> {
        let n = self.allocation_num_vars();
        if n <= 0 {
            return Vec::new();
        }
        let mut vars = vec![0.0f32; n as usize];
        unsafe {
            ocio_sys::ocio_color_space_get_allocation_vars(
                self.handle.as_ptr(), vars.as_mut_ptr(),
            );
        }
        vars
    }

    pub fn set_allocation_vars(&self, vars: &[f32]) {
        unsafe {
            ocio_sys::ocio_color_space_set_allocation_vars(
                self.handle.as_ptr(), vars.len() as i32, vars.as_ptr(),
            );
        }
    }

    pub fn encoding(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_color_space_get_encoding(self.handle.as_ptr())) }
    }

    pub fn set_encoding(&self, encoding: impl AsRef<str>) -> Result<()> {
        let e = cstring(encoding)?;
        unsafe { ocio_sys::ocio_color_space_set_encoding(self.handle.as_ptr(), e.as_ptr().cast()) };
        Ok(())
    }

    pub fn transform(&self, direction: ColorSpaceDirection) -> Option<Transform> {
        let handle = unsafe {
            ocio_sys::ocio_color_space_get_transform(self.handle.as_ptr(), direction as i32)
        };
        transform_from_raw_handle(handle)
    }

    pub fn set_transform(&self, transform: &impl TransformHandle, direction: ColorSpaceDirection) {
        unsafe {
            ocio_sys::ocio_color_space_set_transform(
                self.handle.as_ptr(),
                transform.as_ptr() as *const c_void,
                direction as i32,
            );
        }
    }
}

impl Drop for ColorSpace {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_color_space_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_color_space() {
        let cs = ColorSpace::create();
        assert!(cs.is_ok());
    }

    #[test]
    fn color_space_methods_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let _ = cs.name();
        let _ = cs.family();
        let _ = cs.description();
        let _ = cs.bit_depth();
        let _ = cs.reference_space_type();
        let _ = cs.is_data();
        let _ = cs.allocation();
        let _ = cs.encoding();
    }

    #[test]
    fn set_name() {
        let cs = ColorSpace::create().unwrap();
        assert!(cs.set_name("MyColorSpace").is_ok());
    }

    #[test]
    fn set_is_data() {
        let cs = ColorSpace::create().unwrap();
        cs.set_is_data(true);
        let _ = cs.is_data();
    }

    #[test]
    fn transform_no_crash() {
        let cs = ColorSpace::create().unwrap();
        // In stub mode, returns None since bridge has no transform
        let _ = cs.transform(ColorSpaceDirection::ToReference);
    }

    #[test]
    fn set_transform_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let ft = crate::transform::FileTransform::create().unwrap();
        cs.set_transform(&ft, ColorSpaceDirection::ToReference);
    }
}
