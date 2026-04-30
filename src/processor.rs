use std::ffi::c_void;
use std::ptr::NonNull;

use ocio_sys;
use crate::{cstr_to_opt_string, cstring, OcioError, Result, GpuLanguage, DynamicPropertyType};
use crate::transform::{Transform, GroupTransform, transform_from_raw_handle};

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

    pub fn apply_rgba_pixels(&self, rgba: &mut [f32], num_pixels: i64, stride: i64) {
        unsafe {
            ocio_sys::ocio_processor_apply_rgba_pixels(self.handle.as_ptr(), rgba.as_mut_ptr(), num_pixels, stride);
        }
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

    pub fn default_cpu_processor_bitdepth(&self, in_bit_depth: i32, out_bit_depth: i32) -> Result<CPUProcessor> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_default_cpu_processor_bitdepth(
                self.handle.as_ptr(), in_bit_depth, out_bit_depth,
            )
        };
        NonNull::new(handle).map(|h| CPUProcessor { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn optimized_cpu_processor_bitdepth(&self, in_bit_depth: i32, out_bit_depth: i32, flags: u64) -> Result<CPUProcessor> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_cpu_processor_bitdepth(
                self.handle.as_ptr(), in_bit_depth, out_bit_depth, flags,
            )
        };
        NonNull::new(handle).map(|h| CPUProcessor { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn default_gpu_processor_bitdepth(&self, in_bit_depth: i32, out_bit_depth: i32) -> Result<GPUProcessor> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_default_gpu_processor_bitdepth(
                self.handle.as_ptr(), in_bit_depth, out_bit_depth,
            )
        };
        NonNull::new(handle).map(|h| GPUProcessor { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn optimized_gpu_processor_bitdepth(&self, in_bit_depth: i32, out_bit_depth: i32, flags: u64) -> Result<GPUProcessor> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_gpu_processor_bitdepth(
                self.handle.as_ptr(), in_bit_depth, out_bit_depth, flags,
            )
        };
        NonNull::new(handle).map(|h| GPUProcessor { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn dynamic_property(&self, property_type: DynamicPropertyType) -> Result<DynamicProperty> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_dynamic_property(self.handle.as_ptr(), property_type as i32)
        };
        NonNull::new(handle).map(|h| DynamicProperty { handle: h }).ok_or(OcioError::AllocationFailed)
    }

    pub fn num_transforms(&self) -> i32 {
        unsafe { ocio_sys::ocio_processor_get_num_transforms(self.handle.as_ptr()) }
    }

    pub fn create_group_transform(&self) -> Option<GroupTransform> {
        let handle = unsafe { ocio_sys::ocio_processor_create_group_transform(self.handle.as_ptr()) };
        match transform_from_raw_handle(handle) {
            Some(Transform::Group(gt)) => Some(gt),
            _ => None,
        }
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

    pub fn apply_rgba_pixels(&self, rgba: &mut [f32], num_pixels: i64, stride: i64) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgba_pixels(self.handle.as_ptr(), rgba.as_mut_ptr(), num_pixels, stride);
        }
    }

    pub fn apply_rgb_pixels(&self, rgb: &mut [f32], num_pixels: i64, stride: i64) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgb_pixels(self.handle.as_ptr(), rgb.as_mut_ptr(), num_pixels, stride);
        }
    }

    pub fn apply_rgba_packed(&self, rgba: &mut [u8], bit_depth: i32, num_pixels: i64, stride: i64) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgba_packed(
                self.handle.as_ptr(), rgba.as_mut_ptr() as *mut std::ffi::c_void, bit_depth, num_pixels, stride,
            );
        }
    }

    pub fn apply_rgb_packed(&self, rgb: &mut [u8], bit_depth: i32, num_pixels: i64, stride: i64) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgb_packed(
                self.handle.as_ptr(), rgb.as_mut_ptr() as *mut std::ffi::c_void, bit_depth, num_pixels, stride,
            );
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

    pub fn extract_gpu_shader_info_cache_id(&self, shader_desc: &mut GpuShaderDesc) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_processor_extract_gpu_shader_info_cache_id(
                self.handle.as_ptr(), shader_desc.handle.as_ptr(),
            ))
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

    pub fn language(&self) -> GpuLanguage {
        let l = unsafe { ocio_sys::ocio_gpu_shader_desc_get_language(self.handle.as_ptr()) };
        match l {
            0 => GpuLanguage::Cg,
            1 => GpuLanguage::Glsl1_2,
            2 => GpuLanguage::Glsl1_3,
            3 => GpuLanguage::Glsl4_0,
            4 => GpuLanguage::GlslVk4_6,
            5 => GpuLanguage::HlslSm5_0,
            6 => GpuLanguage::Osl1,
            7 => GpuLanguage::GlslEs1_0,
            8 => GpuLanguage::GlslEs3_0,
            9 => GpuLanguage::Msl2_0,
            _ => GpuLanguage::Glsl1_2,
        }
    }

    pub fn set_language(&self, language: GpuLanguage) {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_language(self.handle.as_ptr(), language as i32);
        }
    }

    pub fn function_name(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_function_name(self.handle.as_ptr()))
        }
    }

    pub fn set_function_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_function_name(self.handle.as_ptr(), n.as_ptr().cast());
        }
        Ok(())
    }

    pub fn pixel_name(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_pixel_name(self.handle.as_ptr()))
        }
    }

    pub fn set_pixel_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_pixel_name(self.handle.as_ptr(), n.as_ptr().cast());
        }
        Ok(())
    }

    pub fn resource_prefix(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_resource_prefix(self.handle.as_ptr()))
        }
    }

    pub fn set_resource_prefix(&self, prefix: impl AsRef<str>) -> Result<()> {
        let p = cstring(prefix)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_resource_prefix(self.handle.as_ptr(), p.as_ptr().cast());
        }
        Ok(())
    }

    pub fn texture_values(&self, index: u32) -> &[f32] {
        let info = self.texture_info(index);
        let len = info.map_or(0, |ti| {
            let dim = ti.width.max(ti.height);
            let pixel_count = match ti.dimensions {
                1 => dim as usize,
                2 => (dim as usize) * (dim as usize),
                3 | _ => (dim as usize) * (dim as usize) * (dim as usize),
            };
            pixel_count * (ti.channel as usize).max(1)
        });
        unsafe {
            let ptr = ocio_sys::ocio_gpu_shader_desc_get_texture_values(self.handle.as_ptr(), index);
            if ptr.is_null() || len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(ptr, len)
            }
        }
    }

    pub fn finalize(&self) {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_finalize(self.handle.as_ptr());
        }
    }

    pub fn texture_max_width(&self, index: i32) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_texture_max_width(self.handle.as_ptr(), index) }
    }

    pub fn texture_max_height(&self, index: i32) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_texture_max_height(self.handle.as_ptr(), index) }
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe { cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_cache_id(self.handle.as_ptr())) }
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

// --- DynamicProperty ---

pub struct DynamicProperty {
    handle: NonNull<c_void>,
}

impl DynamicProperty {
    pub fn property_type(&self) -> DynamicPropertyType {
        let t = unsafe { ocio_sys::ocio_dynamic_property_get_type(self.handle.as_ptr()) };
        match t {
            0 => DynamicPropertyType::Exposure,
            1 => DynamicPropertyType::Contrast,
            2 => DynamicPropertyType::Gamma,
            3 => DynamicPropertyType::GradingPrimary,
            4 => DynamicPropertyType::GradingRgbCurve,
            5 => DynamicPropertyType::GradingTone,
            6 => DynamicPropertyType::GradingHueCurve,
            _ => DynamicPropertyType::Exposure,
        }
    }

    pub fn double_value(&self) -> f64 {
        unsafe { ocio_sys::ocio_dynamic_property_double_get_value(self.handle.as_ptr()) }
    }

    pub fn set_double_value(&self, value: f64) {
        unsafe { ocio_sys::ocio_dynamic_property_double_set_value(self.handle.as_ptr(), value) };
    }
}

impl Drop for DynamicProperty {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_dynamic_property_destroy(self.handle.as_ptr()) };
    }
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
    fn gpu_processor_extract_cache_id() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(gpu) = proc.default_gpu_processor() {
            if let Ok(mut desc) = GpuShaderDesc::create() {
                let _ = gpu.extract_gpu_shader_info_cache_id(&mut desc);
            }
        }
    }

    #[test]
    fn gpu_shader_desc() {
        if let Ok(desc) = GpuShaderDesc::create() {
            // Stub mode returns empty shader text
            let _ = desc.shader_text();
            let _ = desc.num_textures();
            let _ = desc.language();
            let _ = desc.function_name();
            let _ = desc.pixel_name();
            let _ = desc.resource_prefix();
            desc.set_language(GpuLanguage::Glsl1_2);
            let _ = desc.set_function_name("main");
            let _ = desc.set_pixel_name("outColor");
            let _ = desc.set_resource_prefix("ocio_");
            desc.finalize();
        }
    }

    #[test]
    fn gpu_shader_desc_texture_max_no_crash() {
        if let Ok(desc) = GpuShaderDesc::create() {
            let _ = desc.texture_max_width(0);
            let _ = desc.texture_max_height(0);
        }
    }

    #[test]
    fn gpu_shader_desc_cache_id_no_crash() {
        if let Ok(desc) = GpuShaderDesc::create() {
            let _ = desc.cache_id();
        }
    }

    #[test]
    fn processor_num_transforms() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        // Stub mode returns 0
        assert!(proc.num_transforms() >= 0);
    }

    #[test]
    fn processor_create_group_transform() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        // Stub mode returns None
        let _ = proc.create_group_transform();
    }

    #[test]
    fn dynamic_property() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        // In stub mode, creating a dynamic property may or may not succeed
        if let Ok(dp) = proc.dynamic_property(DynamicPropertyType::Exposure) {
            let _ = dp.property_type();
            let _ = dp.double_value();
            dp.set_double_value(1.5);
        }
    }

    #[test]
    fn processor_apply_rgba_pixels_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        let mut pixels = vec![0.0f32; 16]; // 4 pixels RGBA
        proc.apply_rgba_pixels(&mut pixels, 4, 4);
    }

    #[test]
    fn cpu_processor_apply_pixels_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(cpu) = proc.default_cpu_processor() {
            let mut rgba = vec![0.0f32; 16]; // 4 pixels RGBA
            cpu.apply_rgba_pixels(&mut rgba, 4, 4);
            let mut rgb = vec![0.0f32; 12]; // 4 pixels RGB
            cpu.apply_rgb_pixels(&mut rgb, 4, 3);
        }
    }

    #[test]
    fn cpu_processor_apply_packed_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(cpu) = proc.default_cpu_processor() {
            let mut rgba = vec![0u8; 32]; // packed rgba bytes
            cpu.apply_rgba_packed(&mut rgba, 8, 8, 4); // BIT_DEPTH_UINT8 = 0
            let mut rgb = vec![0u8; 24]; // packed rgb bytes
            cpu.apply_rgb_packed(&mut rgb, 8, 8, 3); // BIT_DEPTH_UINT8 = 0
        }
    }

    #[test]
    fn processor_bitdepth_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        // BIT_DEPTH_F32 = 8
        if let Ok(cpu) = proc.default_cpu_processor_bitdepth(8, 8) {
            let _ = cpu.is_no_op();
            let _ = cpu.is_identity();
        }
        if let Ok(cpu) = proc.optimized_cpu_processor_bitdepth(8, 8, 0) {
            let _ = cpu.is_no_op();
        }
        if let Ok(gpu) = proc.default_gpu_processor_bitdepth(8, 8) {
            let _ = gpu.is_no_op();
        }
        if let Ok(gpu) = proc.optimized_gpu_processor_bitdepth(8, 8, 0) {
            let _ = gpu.is_no_op();
        }
    }
}
