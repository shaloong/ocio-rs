use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, OcioError, Result};

pub struct Processor {
    pub(crate) handle: NonNull<c_void>,
}

impl Processor {
    pub fn apply_rgba(&self, rgba: &mut [f32; 4]) -> Result<()> {
        unsafe {
            ocio_sys::ocio_processor_apply_rgba(self.handle.as_ptr(), rgba.as_mut_ptr(), rgba.len());
        }
        Ok(())
    }

    pub fn is_no_op(&self) -> bool {
        unsafe { ocio_sys::ocio_processor_is_no_op(self.handle.as_ptr()) }
    }

    pub fn has_channel_crosstalk(&self) -> bool {
        unsafe { ocio_sys::ocio_processor_has_channel_crosstalk(self.handle.as_ptr()) }
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_processor_get_cache_id(self.handle.as_ptr())) }
    }

    pub fn default_cpu_processor(&self) -> Result<CPUProcessor> {
        let handle = unsafe { ocio_sys::ocio_processor_get_default_cpu_processor(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| CPUProcessor { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn optimized_cpu_processor(&self, flags: u64) -> Result<CPUProcessor> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_cpu_processor(self.handle.as_ptr(), flags)
        };
        NonNull::new(handle).map(|h| CPUProcessor { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn default_gpu_processor(&self) -> Result<GPUProcessor> {
        let handle = unsafe { ocio_sys::ocio_processor_get_default_gpu_processor(self.handle.as_ptr()) };
        NonNull::new(handle).map(|h| GPUProcessor { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn optimized_gpu_processor(&self, flags: u64) -> Result<GPUProcessor> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_gpu_processor(self.handle.as_ptr(), flags)
        };
        NonNull::new(handle).map(|h| GPUProcessor { handle: h }).ok_or(OcioError::AllocationFailed)
    }
}

impl Drop for Processor {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_processor_destroy(self.handle.as_ptr()) };
    }
}

// --- CPUProcessor ---

pub struct CPUProcessor {
    handle: NonNull<c_void>,
}

impl CPUProcessor {
    pub fn apply_rgba(&self, rgba: &mut [f32; 4]) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgba(self.handle.as_ptr(), rgba.as_mut_ptr());
        }
    }

    pub fn apply_rgb(&self, rgb: &mut [f32; 3]) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgb(self.handle.as_ptr(), rgb.as_mut_ptr());
        }
    }

    pub fn is_no_op(&self) -> bool {
        unsafe { ocio_sys::ocio_cpu_processor_is_no_op(self.handle.as_ptr()) }
    }

    pub fn has_channel_crosstalk(&self) -> bool {
        unsafe { ocio_sys::ocio_cpu_processor_has_channel_crosstalk(self.handle.as_ptr()) }
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_cpu_processor_get_cache_id(self.handle.as_ptr())) }
    }

    pub fn input_bit_depth(&self) -> i32 {
        unsafe { ocio_sys::ocio_cpu_processor_get_input_bit_depth(self.handle.as_ptr()) }
    }

    pub fn output_bit_depth(&self) -> i32 {
        unsafe { ocio_sys::ocio_cpu_processor_get_output_bit_depth(self.handle.as_ptr()) }
    }

    pub fn is_identity(&self) -> bool {
        unsafe { ocio_sys::ocio_cpu_processor_is_identity(self.handle.as_ptr()) }
    }
}

impl Drop for CPUProcessor {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_cpu_processor_destroy(self.handle.as_ptr()) };
    }
}

// --- GPUProcessor ---

pub struct GPUProcessor {
    handle: NonNull<c_void>,
}

impl GPUProcessor {
    pub fn is_no_op(&self) -> bool {
        unsafe { ocio_sys::ocio_gpu_processor_is_no_op(self.handle.as_ptr()) }
    }

    pub fn has_channel_crosstalk(&self) -> bool {
        unsafe { ocio_sys::ocio_gpu_processor_has_channel_crosstalk(self.handle.as_ptr()) }
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_gpu_processor_get_cache_id(self.handle.as_ptr())) }
    }

    pub fn extract_shader_info(&self, shader_desc: &mut GpuShaderDesc) {
        unsafe {
            ocio_sys::ocio_gpu_processor_extract_shader_info(
                self.handle.as_ptr(), shader_desc.handle.as_ptr(),
            );
        }
    }
}

impl Drop for GPUProcessor {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_gpu_processor_destroy(self.handle.as_ptr()) };
    }
}

// --- GpuShaderDesc ---

pub struct GpuShaderDesc {
    handle: NonNull<c_void>,
}

impl GpuShaderDesc {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_gpu_shader_desc_create() };
        NonNull::new(handle).map(|h| Self { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn shader_text(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_shader_text(self.handle.as_ptr()))
        }
    }

    pub fn num_textures(&self) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_num_textures(self.handle.as_ptr()) }
    }

    pub fn texture_info(&self, index: u32) -> Option<TextureInfo> {
        let mut texture_name: *const i8 = std::ptr::null();
        let mut sampler_name: *const i8 = std::ptr::null();
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut channel: i32 = 0;
        let mut dimensions: i32 = 0;
        let mut interpolation: i32 = 0;

        unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_texture(
                self.handle.as_ptr(), index,
                &mut texture_name, &mut sampler_name,
                &mut width, &mut height,
                &mut channel, &mut dimensions, &mut interpolation,
            );
        }

        let tex_name = unsafe { cstr_to_opt_string(texture_name) }?;
        let smp_name = unsafe { cstr_to_opt_string(sampler_name) }?;

        Some(TextureInfo {
            texture_name: tex_name,
            sampler_name: smp_name,
            width,
            height,
            channel,
            dimensions,
            interpolation,
        })
    }
}

impl Drop for GpuShaderDesc {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_gpu_shader_desc_destroy(self.handle.as_ptr()) };
    }
}

pub struct TextureInfo {
    pub texture_name: String,
    pub sampler_name: String,
    pub width: u32,
    pub height: u32,
    pub channel: i32,
    pub dimensions: i32,
    pub interpolation: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config;

    #[test]
    fn processor_apply_rgba() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        let mut pixel = [0.5, 0.25, 0.125, 1.0];
        proc.apply_rgba(&mut pixel).unwrap();
    }

    #[test]
    fn processor_metadata() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        let _ = proc.is_no_op();
        let _ = proc.has_channel_crosstalk();
        let _ = proc.cache_id();
    }

    #[test]
    fn cpu_processor() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        // In stub mode, creating a CPU processor may or may not succeed
        if let Ok(cpu) = proc.default_cpu_processor() {
            let mut pixel = [0.5, 0.25, 0.125, 1.0];
            cpu.apply_rgba(&mut pixel);
            let _ = cpu.is_no_op();
            let _ = cpu.is_identity();
            let _ = cpu.cache_id();
            let _ = cpu.input_bit_depth();
            let _ = cpu.output_bit_depth();
        }
    }

    #[test]
    fn gpu_processor() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(gpu) = proc.default_gpu_processor() {
            let _ = gpu.is_no_op();
            let _ = gpu.cache_id();
        }
    }

    #[test]
    fn gpu_shader_desc() {
        if let Ok(desc) = GpuShaderDesc::create() {
            // Stub mode returns empty shader text
            let _ = desc.shader_text();
            let _ = desc.num_textures();
        }
    }
}
