use std::ffi::c_void;
use std::ptr::NonNull;

use crate::transform::{transform_from_raw_handle, GroupTransform, Transform};
use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, BitDepth, DynamicPropertyType, FormatMetadata,
    GpuLanguage, HueCurveType, Interpolation, OcioError, RGBCurveType, Result,
};
use ocio_sys;

/// An immutable color-processing pipeline produced from a `Config`.
///
/// Use `default_cpu_processor` or `default_gpu_processor` to execute or extract
/// the processing implementation.
pub struct Processor {
    pub(crate) handle: NonNull<c_void>,
}

impl Processor {
    /// Apply the processor to a single RGBA pixel in place.
    pub fn apply_rgba(&self, rgba: &mut [f32; 4]) -> Result<()> {
        unsafe {
            ocio_sys::ocio_processor_apply_rgba(
                self.handle.as_ptr(),
                rgba.as_mut_ptr(),
                rgba.len(),
            );
        }
        Ok(())
    }

    /// Apply the processor to packed RGBA pixel data in place.
    pub fn apply_rgba_pixels(&self, rgba: &mut [f32], num_pixels: i64, stride: i64) {
        unsafe {
            ocio_sys::ocio_processor_apply_rgba_pixels(
                self.handle.as_ptr(),
                rgba.as_mut_ptr(),
                num_pixels,
                stride,
            );
        }
    }

    pub fn is_no_op(&self) -> bool {
        unsafe { ocio_sys::ocio_processor_is_no_op(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn has_channel_crosstalk(&self) -> bool {
        unsafe {
            ocio_sys::ocio_processor_has_channel_crosstalk(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_processor_get_cache_id(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    /// Create the default CPU execution path for this processor.
    pub fn default_cpu_processor(&self) -> Result<CPUProcessor> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_default_cpu_processor(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(handle)
            .map(|h| CPUProcessor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn optimized_cpu_processor(&self, flags: u64) -> Result<CPUProcessor> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_cpu_processor(self.handle.as_ptr(), flags as i32)
        };
        NonNull::new(handle)
            .map(|h| CPUProcessor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer optimized_processor_bitdepth or optimized_cpu/gpu_processor helpers"
    )]
    pub fn optimized_processor_v1(&self, flags: u64) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_processor_v1(self.handle.as_ptr(), flags as i32)
        };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn optimized_processor_bitdepth(
        &self,
        in_bit_depth: i32,
        out_bit_depth: i32,
        flags: u64,
    ) -> Result<Self> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_processor_v2(
                self.handle.as_ptr(),
                in_bit_depth,
                out_bit_depth,
                flags as i32,
            )
        };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer optimized_processor_bitdepth()"
    )]
    pub fn optimized_processor_v2(
        &self,
        in_bit_depth: i32,
        out_bit_depth: i32,
        flags: u64,
    ) -> Result<Self> {
        self.optimized_processor_bitdepth(in_bit_depth, out_bit_depth, flags)
    }

    /// Create the default GPU execution path for this processor.
    pub fn default_gpu_processor(&self) -> Result<GPUProcessor> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_default_gpu_processor(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(handle)
            .map(|h| GPUProcessor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn optimized_gpu_processor(&self, flags: u64) -> Result<GPUProcessor> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_gpu_processor(self.handle.as_ptr(), flags as i32)
        };
        NonNull::new(handle)
            .map(|h| GPUProcessor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    #[deprecated(
        since = "0.2.0",
        note = "legacy OCIO GPU optimization path; prefer optimized_gpu_processor or default_gpu_processor"
    )]
    pub fn optimized_legacy_gpu_processor(
        &self,
        flags: u64,
        edge_len: u32,
    ) -> Result<GPUProcessor> {
        // OCIO v1-style GPU path that bakes some ops into a 3D LUT.
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_legacy_gpu_processor(
                self.handle.as_ptr(),
                flags as i32,
                edge_len,
            )
        };
        NonNull::new(handle)
            .map(|h| GPUProcessor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn default_cpu_processor_bitdepth(
        &self,
        in_bit_depth: i32,
        out_bit_depth: i32,
    ) -> Result<CPUProcessor> {
        self.optimized_cpu_processor_bitdepth(in_bit_depth, out_bit_depth, 0)
    }

    pub fn optimized_cpu_processor_bitdepth(
        &self,
        in_bit_depth: i32,
        out_bit_depth: i32,
        flags: u64,
    ) -> Result<CPUProcessor> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_cpu_processor_v1(
                self.handle.as_ptr(),
                in_bit_depth,
                out_bit_depth,
                flags as i32,
            )
        };
        NonNull::new(handle)
            .map(|h| CPUProcessor { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer optimized_cpu_processor_bitdepth()"
    )]
    pub fn optimized_cpu_processor_v1(
        &self,
        in_bit_depth: i32,
        out_bit_depth: i32,
        flags: u64,
    ) -> Result<CPUProcessor> {
        self.optimized_cpu_processor_bitdepth(in_bit_depth, out_bit_depth, flags)
    }

    pub fn default_gpu_processor_bitdepth(
        &self,
        in_bit_depth: i32,
        out_bit_depth: i32,
    ) -> Result<GPUProcessor> {
        let _ = (in_bit_depth, out_bit_depth);
        self.default_gpu_processor()
    }

    pub fn optimized_gpu_processor_bitdepth(
        &self,
        in_bit_depth: i32,
        out_bit_depth: i32,
        flags: u64,
    ) -> Result<GPUProcessor> {
        let _ = (in_bit_depth, out_bit_depth);
        self.optimized_gpu_processor(flags)
    }

    pub fn dynamic_property(&self, property_type: DynamicPropertyType) -> Result<DynamicProperty> {
        let handle = unsafe {
            ocio_sys::ocio_processor_get_dynamic_property(
                self.handle.as_ptr(),
                property_type as i32,
            )
        };
        NonNull::new(handle)
            .map(|h| DynamicProperty { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn num_transforms(&self) -> i32 {
        unsafe { ocio_sys::ocio_processor_get_num_transforms(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn create_group_transform(&self) -> Option<GroupTransform> {
        let handle = unsafe {
            ocio_sys::ocio_processor_create_group_transform(self.handle.as_ptr() as *mut c_void)
        };
        match transform_from_raw_handle(handle) {
            Some(Transform::Group(gt)) => Some(gt),
            _ => None,
        }
    }

    // ── v2.5.1 ──
    pub fn format_metadata(&self) -> Option<FormatMetadata> {
        let h = unsafe {
            ocio_sys::ocio_processor_get_format_metadata(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(h).map(|h| FormatMetadata { handle: h })
    }

    pub fn transform_format_metadata(&self, index: i32) -> Option<FormatMetadata> {
        let h = unsafe {
            ocio_sys::ocio_processor_get_transform_format_metadata(self.handle.as_ptr(), index)
        };
        NonNull::new(h).map(|h| FormatMetadata { handle: h })
    }

    pub fn processor_metadata(&self) -> Option<FormatMetadata> {
        let h = unsafe {
            ocio_sys::ocio_processor_get_processor_metadata(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(h).map(|h| FormatMetadata { handle: h })
    }

    pub fn has_dynamic_property_kind(&self, prop_type: DynamicPropertyType) -> bool {
        unsafe {
            ocio_sys::ocio_processor_has_dynamic_property(self.handle.as_ptr(), prop_type as i32)
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer has_dynamic_property_kind with DynamicPropertyType"
    )]
    pub fn has_dynamic_property(&self, prop_type: i32) -> bool {
        unsafe { ocio_sys::ocio_processor_has_dynamic_property(self.handle.as_ptr(), prop_type) }
    }

    pub fn is_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_processor_is_dynamic(self.handle.as_ptr() as *mut c_void) }
    }
}

impl Drop for Processor {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_processor_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

// --- CPUProcessor ---

/// CPU implementation of a `Processor`.
///
/// Methods on this type apply color transforms to packed RGB/RGBA pixel data.
pub struct CPUProcessor {
    handle: NonNull<c_void>,
}

impl CPUProcessor {
    /// # Safety
    /// `img_desc` must point to a valid OCIO image descriptor compatible with the active ABI.
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO image-descriptor entry point; prefer apply_rgb/apply_rgba/apply_*_pixels for Rust callers"
    )]
    pub unsafe fn apply_v1(&self, img_desc: *mut c_void) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_v1(self.handle.as_ptr(), img_desc);
        }
    }

    /// # Safety
    /// `src_img_desc` and `dst_img_desc` must point to valid OCIO image descriptors.
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO image-descriptor entry point; prefer apply_rgb/apply_rgba/apply_*_pixels for Rust callers"
    )]
    pub unsafe fn apply_v2(&self, src_img_desc: *mut c_void, dst_img_desc: *mut c_void) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_v2(self.handle.as_ptr(), src_img_desc, dst_img_desc);
        }
    }

    pub fn apply_rgba(&self, rgba: &mut [f32; 4]) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgba(
                self.handle.as_ptr(),
                rgba.as_mut_ptr() as *mut c_void,
            );
        }
    }

    pub fn apply_rgb(&self, rgb: &mut [f32; 3]) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgb(
                self.handle.as_ptr(),
                rgb.as_mut_ptr() as *mut c_void,
            );
        }
    }

    pub fn apply_rgba_pixels(&self, rgba: &mut [f32], num_pixels: i64, stride: i64) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgba_pixels(
                self.handle.as_ptr(),
                rgba.as_mut_ptr(),
                num_pixels,
                stride,
            );
        }
    }

    pub fn apply_rgb_pixels(&self, rgb: &mut [f32], num_pixels: i64, stride: i64) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgb_pixels(
                self.handle.as_ptr(),
                rgb.as_mut_ptr(),
                num_pixels,
                stride,
            );
        }
    }

    pub fn apply_rgba_packed_bit_depth(
        &self,
        rgba: &mut [u8],
        bit_depth: BitDepth,
        num_pixels: i64,
        stride: i64,
    ) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgba_packed(
                self.handle.as_ptr(),
                rgba.as_mut_ptr() as *mut std::ffi::c_void,
                bit_depth as i32,
                num_pixels,
                stride,
            );
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer apply_rgba_packed_bit_depth with the BitDepth enum"
    )]
    pub fn apply_rgba_packed(&self, rgba: &mut [u8], bit_depth: i32, num_pixels: i64, stride: i64) {
        let bit_depth = match bit_depth {
            1 => BitDepth::Uint8,
            2 => BitDepth::Uint10,
            3 => BitDepth::Uint12,
            4 => BitDepth::Uint14,
            5 => BitDepth::Uint16,
            6 => BitDepth::Uint32,
            7 => BitDepth::F16,
            8 => BitDepth::F32,
            _ => BitDepth::Unknown,
        };
        self.apply_rgba_packed_bit_depth(rgba, bit_depth, num_pixels, stride);
    }

    pub fn apply_rgb_packed_bit_depth(
        &self,
        rgb: &mut [u8],
        bit_depth: BitDepth,
        num_pixels: i64,
        stride: i64,
    ) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgb_packed(
                self.handle.as_ptr(),
                rgb.as_mut_ptr() as *mut std::ffi::c_void,
                bit_depth as i32,
                num_pixels,
                stride,
            );
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer apply_rgb_packed_bit_depth with the BitDepth enum"
    )]
    pub fn apply_rgb_packed(&self, rgb: &mut [u8], bit_depth: i32, num_pixels: i64, stride: i64) {
        let bit_depth = match bit_depth {
            1 => BitDepth::Uint8,
            2 => BitDepth::Uint10,
            3 => BitDepth::Uint12,
            4 => BitDepth::Uint14,
            5 => BitDepth::Uint16,
            6 => BitDepth::Uint32,
            7 => BitDepth::F16,
            8 => BitDepth::F32,
            _ => BitDepth::Unknown,
        };
        self.apply_rgb_packed_bit_depth(rgb, bit_depth, num_pixels, stride);
    }

    pub fn is_no_op(&self) -> bool {
        unsafe { ocio_sys::ocio_cpu_processor_is_no_op(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn has_channel_crosstalk(&self) -> bool {
        unsafe {
            ocio_sys::ocio_cpu_processor_has_channel_crosstalk(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_cpu_processor_get_cache_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn input_bit_depth(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_cpu_processor_get_input_bit_depth(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn output_bit_depth(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_cpu_processor_get_output_bit_depth(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn is_identity(&self) -> bool {
        unsafe { ocio_sys::ocio_cpu_processor_is_identity(self.handle.as_ptr() as *mut c_void) }
    }

    // ── v2.5.1 ──
    pub fn dynamic_property(&self, prop_type: DynamicPropertyType) -> Option<DynamicProperty> {
        let h = unsafe {
            ocio_sys::ocio_cpu_processor_get_dynamic_property(
                self.handle.as_ptr(),
                prop_type as i32,
            )
        };
        NonNull::new(h).map(|h| DynamicProperty { handle: h })
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer dynamic_property with DynamicPropertyType"
    )]
    pub fn get_dynamic_property(&self, prop_type: i32) -> Option<DynamicProperty> {
        let h = unsafe {
            ocio_sys::ocio_cpu_processor_get_dynamic_property(self.handle.as_ptr(), prop_type)
        };
        NonNull::new(h).map(|h| DynamicProperty { handle: h })
    }

    pub fn has_dynamic_property_kind(&self, prop_type: DynamicPropertyType) -> bool {
        unsafe {
            ocio_sys::ocio_cpu_processor_has_dynamic_property(
                self.handle.as_ptr(),
                prop_type as i32,
            )
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer has_dynamic_property_kind with DynamicPropertyType"
    )]
    pub fn has_dynamic_property(&self, prop_type: i32) -> bool {
        unsafe {
            ocio_sys::ocio_cpu_processor_has_dynamic_property(self.handle.as_ptr(), prop_type)
        }
    }

    pub fn is_dynamic(&self) -> bool {
        unsafe { ocio_sys::ocio_cpu_processor_is_dynamic(self.handle.as_ptr() as *mut c_void) }
    }
}

impl Drop for CPUProcessor {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_cpu_processor_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

// --- GPUProcessor ---

/// GPU implementation of a `Processor`.
///
/// Use this with `GpuShaderDesc` to extract shader text, textures, and uniforms.
pub struct GPUProcessor {
    handle: NonNull<c_void>,
}

impl GPUProcessor {
    pub fn is_no_op(&self) -> bool {
        unsafe { ocio_sys::ocio_gpu_processor_is_no_op(self.handle.as_ptr() as *mut c_void) }
    }

    pub fn has_channel_crosstalk(&self) -> bool {
        unsafe {
            ocio_sys::ocio_gpu_processor_has_channel_crosstalk(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_gpu_processor_get_cache_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn extract_shader_info(&self, shader_desc: &mut GpuShaderDesc) {
        unsafe {
            ocio_sys::ocio_gpu_processor_extract_gpu_shader_info_v1(
                self.handle.as_ptr(),
                shader_desc.handle.as_ptr(),
            );
        }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer extract_shader_info()")]
    pub fn extract_gpu_shader_info_v1(&self, shader_desc: &mut GpuShaderDesc) {
        self.extract_shader_info(shader_desc);
    }

    /// # Safety
    /// `shader_creator` must point to a valid OCIO shader creator object for the active ABI.
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO shader-creator entry point; prefer extract_shader_info with GpuShaderDesc for Rust callers"
    )]
    pub unsafe fn extract_gpu_shader_info_v2(&self, shader_creator: *mut c_void) {
        unsafe {
            ocio_sys::ocio_gpu_processor_extract_gpu_shader_info_v2(
                self.handle.as_ptr(),
                shader_creator,
            );
        }
    }
}

impl Drop for GPUProcessor {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_gpu_processor_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

// --- GpuShaderDesc ---

/// Collects parameters and emitted source for GPU shader extraction.
pub struct GpuShaderDesc {
    handle: NonNull<c_void>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GpuTextureChannel {
    Red = 0,
    Rgb = 1,
}

impl GpuTextureChannel {
    fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Red,
            _ => Self::Rgb,
        }
    }

    fn channel_count(self) -> usize {
        match self {
            Self::Red => 1,
            Self::Rgb => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GpuTextureDimensions {
    Texture1D = 0,
    Texture2D = 1,
}

impl GpuTextureDimensions {
    fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Texture2D,
            _ => Self::Texture1D,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GpuUniformType {
    Double = 0,
    Bool = 1,
    Float3 = 2,
    VectorFloat = 3,
    VectorInt = 4,
    Unknown = 5,
}

impl GpuUniformType {
    fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Double,
            1 => Self::Bool,
            2 => Self::Float3,
            3 => Self::VectorFloat,
            4 => Self::VectorInt,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
/// Texture payload and metadata for a 1D/2D GPU LUT resource.
pub struct GpuTexture2D {
    pub texture_name: String,
    pub sampler_name: String,
    pub width: u32,
    pub height: u32,
    pub channel: GpuTextureChannel,
    pub dimensions: GpuTextureDimensions,
    pub interpolation: Interpolation,
    pub binding_index: u32,
    pub values: Vec<f32>,
}

impl GpuTexture2D {
    pub fn expected_value_count(&self) -> usize {
        self.width as usize * self.height as usize * self.channel.channel_count()
    }
}

#[derive(Debug, Clone)]
/// Texture payload and metadata for a 3D GPU LUT resource.
pub struct GpuTexture3D {
    pub texture_name: String,
    pub sampler_name: String,
    pub edge_len: u32,
    pub interpolation: Interpolation,
    pub binding_index: u32,
    pub values: Vec<f32>,
}

impl GpuTexture3D {
    pub fn expected_value_count(&self) -> usize {
        let edge = self.edge_len as usize;
        edge * edge * edge * 3
    }
}

#[derive(Debug, Clone)]
/// Typed value payload for a GPU uniform extracted from OCIO.
pub enum GpuUniformValue {
    F32(Vec<f32>),
    I32(Vec<i32>),
    Unsupported,
}

#[derive(Debug, Clone)]
/// GPU uniform metadata and current value payload extracted from OCIO.
pub struct GpuUniform {
    pub name: String,
    pub uniform_type: GpuUniformType,
    pub buffer_offset: usize,
    pub value_count: usize,
    pub value: GpuUniformValue,
}

impl GpuShaderDesc {
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_gpu_shader_desc_create() };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    pub fn shader_text(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_gpu_shader_desc_get_shader_text(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn num_textures(&self) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_num_textures_u32(self.handle.as_ptr()) }
    }

    pub fn texture_info(&self, index: u32) -> Option<TextureInfo> {
        self.texture_2d(index).map(|texture| TextureInfo {
            texture_name: texture.texture_name,
            sampler_name: texture.sampler_name,
            width: texture.width,
            height: texture.height,
            channel: texture.channel as i32,
            dimensions: texture.dimensions as i32,
            interpolation: texture.interpolation as i32,
        })
    }

    pub fn texture_2d(&self, index: u32) -> Option<GpuTexture2D> {
        let mut info = ocio_sys::OcioGpuTexture2DInfo {
            texture_name: std::ptr::null(),
            sampler_name: std::ptr::null(),
            width: 0,
            height: 0,
            channel: 0,
            dimensions: 0,
            interpolation: 0,
            binding_index: 0,
        };
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_texture_info(self.handle.as_ptr(), index, &mut info)
        };
        if !ok {
            return None;
        }
        let value_count = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_texture_value_count(self.handle.as_ptr(), index)
        };
        let mut values = vec![0.0f32; value_count];
        let values_ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_copy_texture_values(
                self.handle.as_ptr(),
                index,
                values.as_mut_ptr(),
                values.len(),
            )
        };
        if !values_ok && value_count > 0 {
            return None;
        }
        Some(GpuTexture2D {
            texture_name: unsafe { cstr_to_opt_string(info.texture_name) }.unwrap_or_default(),
            sampler_name: unsafe { cstr_to_opt_string(info.sampler_name) }.unwrap_or_default(),
            width: info.width,
            height: info.height,
            channel: GpuTextureChannel::from_raw(info.channel),
            dimensions: GpuTextureDimensions::from_raw(info.dimensions),
            interpolation: interpolation_from_raw(info.interpolation),
            binding_index: info.binding_index,
            values,
        })
    }

    pub fn textures_2d(&self) -> Vec<GpuTexture2D> {
        (0..self.num_textures())
            .filter_map(|index| self.texture_2d(index))
            .collect()
    }

    pub fn language(&self) -> GpuLanguage {
        let l = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_language(self.handle.as_ptr() as *mut c_void)
        };
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
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_function_name(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn set_function_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_function_name(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
            );
        }
        Ok(())
    }

    pub fn pixel_name(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_pixel_name(
                self.handle.as_ptr() as *mut c_void,
            ))
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
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_resource_prefix(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn set_resource_prefix(&self, prefix: impl AsRef<str>) -> Result<()> {
        let p = cstring(prefix)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_resource_prefix(
                self.handle.as_ptr(),
                p.as_ptr().cast(),
            );
        }
        Ok(())
    }

    pub fn texture_values(&self, index: u32) -> Vec<f32> {
        self.texture_2d(index)
            .map(|texture| texture.values)
            .unwrap_or_default()
    }

    pub fn finalize(&self) {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_finalize(self.handle.as_ptr() as *mut c_void);
        }
    }

    pub fn texture_max_width(&self, index: i32) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_texture_max_width(self.handle.as_ptr(), index) }
    }

    pub fn texture_max_height(&self, index: i32) -> u32 {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_texture_max_height(self.handle.as_ptr(), index)
        }
    }

    pub fn cache_id(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_cache_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    pub fn texture_uid(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_texture_uid(
                self.handle.as_ptr(),
                index,
            ))
        }
    }

    // ── v2.5.1 ──
    /// Clone the descriptor configuration.
    ///
    /// In real OCIO builds this preserves descriptor settings such as language,
    /// function name, pixel name, and resource prefix. Extracted shader payloads
    /// are not guaranteed to be copied into the clone.
    pub fn clone_desc(&self) -> Option<GpuShaderDesc> {
        let h =
            unsafe { ocio_sys::ocio_gpu_shader_desc_clone(self.handle.as_ptr() as *mut c_void) };
        NonNull::new(h).map(|h| GpuShaderDesc { handle: h })
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer clone_desc()")]
    #[allow(clippy::should_implement_trait)]
    pub fn clone(&self) -> Option<GpuShaderDesc> {
        self.clone_desc()
    }

    pub fn num_uniforms(&self) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_num_uniforms_u32(self.handle.as_ptr()) }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_uniforms()")]
    pub fn get_num_uniforms_u32(&self) -> u32 {
        self.num_uniforms()
    }

    pub fn uniform_buffer_size(&self) -> usize {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_uniform_buffer_size_bytes(self.handle.as_ptr())
        }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer uniform_buffer_size()")]
    pub fn get_uniform_buffer_size_bytes(&self) -> usize {
        self.uniform_buffer_size()
    }

    pub fn uniform(&self, index: u32) -> Option<GpuUniform> {
        let mut info = ocio_sys::OcioGpuUniformInfo {
            name: std::ptr::null(),
            type_: 5,
            buffer_offset: 0,
            value_count: 0,
        };
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_uniform_info(self.handle.as_ptr(), index, &mut info)
        };
        if !ok {
            return None;
        }
        let uniform_type = GpuUniformType::from_raw(info.type_);
        let value = match uniform_type {
            GpuUniformType::VectorInt => {
                let mut values = vec![0i32; info.value_count];
                let ok = unsafe {
                    ocio_sys::ocio_gpu_shader_desc_copy_uniform_i32_values(
                        self.handle.as_ptr(),
                        index,
                        values.as_mut_ptr(),
                        values.len(),
                    )
                };
                if ok {
                    GpuUniformValue::I32(values)
                } else {
                    GpuUniformValue::Unsupported
                }
            }
            GpuUniformType::Unknown => GpuUniformValue::Unsupported,
            _ => {
                let mut values = vec![0.0f32; info.value_count];
                let ok = unsafe {
                    ocio_sys::ocio_gpu_shader_desc_copy_uniform_f32_values(
                        self.handle.as_ptr(),
                        index,
                        values.as_mut_ptr(),
                        values.len(),
                    )
                };
                if ok {
                    GpuUniformValue::F32(values)
                } else {
                    GpuUniformValue::Unsupported
                }
            }
        };
        Some(GpuUniform {
            name: unsafe { cstr_to_opt_string(info.name) }.unwrap_or_default(),
            uniform_type,
            buffer_offset: info.buffer_offset,
            value_count: info.value_count,
            value,
        })
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer uniform()")]
    pub fn get_uniform_info(&self, index: u32) -> Option<GpuUniform> {
        self.uniform(index)
    }

    pub fn uniform_value_count(&self, index: u32) -> usize {
        self.uniform(index)
            .map(|uniform| uniform.value_count)
            .unwrap_or(0)
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer uniform_value_count()")]
    pub fn get_uniform_value_count(&self, index: u32) -> usize {
        self.uniform_value_count(index)
    }

    pub fn uniform_values_f32(&self, index: u32) -> Vec<f32> {
        match self.uniform(index).map(|uniform| uniform.value) {
            Some(GpuUniformValue::F32(values)) => values,
            _ => Vec::new(),
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer uniform_values_f32() or uniform()"
    )]
    pub fn copy_uniform_f32_values(&self, index: u32) -> Vec<f32> {
        self.uniform_values_f32(index)
    }

    pub fn uniform_values_i32(&self, index: u32) -> Vec<i32> {
        match self.uniform(index).map(|uniform| uniform.value) {
            Some(GpuUniformValue::I32(values)) => values,
            _ => Vec::new(),
        }
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer uniform_values_i32() or uniform()"
    )]
    pub fn copy_uniform_i32_values(&self, index: u32) -> Vec<i32> {
        self.uniform_values_i32(index)
    }

    pub fn uniforms(&self) -> Vec<GpuUniform> {
        (0..self.num_uniforms())
            .filter_map(|index| self.uniform(index))
            .collect()
    }

    pub fn num_3d_textures(&self) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_num3d_textures_u32(self.handle.as_ptr()) }
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_textures()")]
    pub fn get_num_textures_u32(&self) -> u32 {
        self.num_textures()
    }

    pub fn texture_value_count(&self, index: u32) -> usize {
        self.texture_values(index).len()
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer texture_value_count()")]
    pub fn get_texture_value_count(&self, index: u32) -> usize {
        self.texture_value_count(index)
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer texture_2d() or textures_2d()"
    )]
    pub fn copy_texture_values(&self, index: u32) -> Vec<f32> {
        self.texture_values(index)
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_3d_textures()")]
    pub fn get_num3d_textures_u32(&self) -> u32 {
        self.num_3d_textures()
    }

    pub fn texture_3d(&self, index: u32) -> Option<GpuTexture3D> {
        let mut info = ocio_sys::OcioGpuTexture3DInfo {
            texture_name: std::ptr::null(),
            sampler_name: std::ptr::null(),
            edge_len: 0,
            interpolation: 0,
            binding_index: 0,
        };
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get3d_texture_info(
                self.handle.as_ptr(),
                index,
                &mut info,
            )
        };
        if !ok {
            return None;
        }
        let value_count = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get3d_texture_value_count(self.handle.as_ptr(), index)
        };
        let mut values = vec![0.0f32; value_count];
        let values_ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_copy3d_texture_values(
                self.handle.as_ptr(),
                index,
                values.as_mut_ptr(),
                values.len(),
            )
        };
        if !values_ok && value_count > 0 {
            return None;
        }
        Some(GpuTexture3D {
            texture_name: unsafe { cstr_to_opt_string(info.texture_name) }.unwrap_or_default(),
            sampler_name: unsafe { cstr_to_opt_string(info.sampler_name) }.unwrap_or_default(),
            edge_len: info.edge_len,
            interpolation: interpolation_from_raw(info.interpolation),
            binding_index: info.binding_index,
            values,
        })
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer texture_3d()")]
    pub fn get3d_texture_info(&self, index: u32) -> Option<GpuTexture3D> {
        self.texture_3d(index)
    }

    pub fn texture_3d_value_count(&self, index: u32) -> usize {
        self.texture_3d(index)
            .map(|texture| texture.values.len())
            .unwrap_or(0)
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer texture_3d_value_count()"
    )]
    pub fn get3d_texture_value_count(&self, index: u32) -> usize {
        self.texture_3d_value_count(index)
    }

    pub fn texture_3d_values(&self, index: u32) -> Vec<f32> {
        self.texture_3d(index)
            .map(|texture| texture.values)
            .unwrap_or_default()
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer texture_3d_values()")]
    pub fn copy3d_texture_values(&self, index: u32) -> Vec<f32> {
        self.texture_3d_values(index)
    }

    pub fn textures_3d(&self) -> Vec<GpuTexture3D> {
        (0..self.num_3d_textures())
            .filter_map(|index| self.texture_3d(index))
            .collect()
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_3d_textures()")]
    pub fn get_num3d_textures(&self) -> u32 {
        self.num_3d_textures()
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer texture_3d()")]
    pub fn get3d_texture(&self, index: u32) -> Option<GpuTexture3D> {
        self.texture_3d(index)
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer texture_3d_values()")]
    pub fn get3d_texture_values(&self, index: u32) -> Vec<f32> {
        self.texture_3d_values(index)
    }

    pub fn texture_3d_shader_binding_index(&self, index: u32) -> Option<u32> {
        self.texture_3d(index).map(|texture| texture.binding_index)
    }

    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer texture_3d_shader_binding_index()"
    )]
    pub fn get3d_texture_shader_binding_index(&self, index: u32) -> Option<u32> {
        self.texture_3d_shader_binding_index(index)
    }

    pub fn texture_shader_binding_index(&self, index: u32) -> Option<u32> {
        self.texture_2d(index).map(|texture| texture.binding_index)
    }

    pub fn uniform_name(&self, index: u32) -> Option<String> {
        self.uniform(index).map(|uniform| uniform.name)
    }
}

impl Drop for GpuShaderDesc {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_gpu_shader_desc_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

/// Lightweight texture metadata used by legacy GPU descriptor accessors.
pub struct TextureInfo {
    pub texture_name: String,
    pub sampler_name: String,
    pub width: u32,
    pub height: u32,
    pub channel: i32,
    pub dimensions: i32,
    pub interpolation: i32,
}

fn interpolation_from_raw(value: i32) -> Interpolation {
    match value {
        1 => Interpolation::Nearest,
        2 => Interpolation::Linear,
        3 => Interpolation::Tetrahedral,
        4 => Interpolation::Cubic,
        5 => Interpolation::Default,
        6 => Interpolation::Best,
        _ => Interpolation::Unknown,
    }
}

// --- DynamicProperty ---

/// References a processor property that may be adjusted dynamically at runtime.
pub struct DynamicProperty {
    handle: NonNull<c_void>,
}

impl DynamicProperty {
    pub fn property_type(&self) -> DynamicPropertyType {
        let t = unsafe {
            ocio_sys::ocio_dynamic_property_get_type(self.handle.as_ptr() as *mut c_void)
        };
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
        unsafe {
            ocio_sys::ocio_dynamic_property_double_get_value(self.handle.as_ptr() as *mut c_void)
        }
    }

    pub fn set_double_value(&self, value: f64) {
        unsafe { ocio_sys::ocio_dynamic_property_double_set_value(self.handle.as_ptr(), value) };
    }

    pub fn grading_primary_value(&self) -> Option<crate::grading::GradingPrimary> {
        if self.property_type() != DynamicPropertyType::GradingPrimary {
            return None;
        }
        let mut values = [0.0f64; 34];
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_primary_get_value(
                self.handle.as_ptr(),
                values.as_mut_ptr(),
            );
        }
        Some(crate::grading::GradingPrimary::from_flat_array(&values))
    }

    pub fn set_grading_primary_value(&self, value: &crate::grading::GradingPrimary) {
        let values = value.to_flat_array();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_primary_set_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
    }

    pub fn grading_tone_value(&self) -> Option<crate::grading::GradingTone> {
        if self.property_type() != DynamicPropertyType::GradingTone {
            return None;
        }
        let mut values = [0.0f64; 31];
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_tone_get_value(
                self.handle.as_ptr(),
                values.as_mut_ptr(),
            );
        }
        Some(crate::grading::GradingTone::from_flat_array(&values))
    }

    pub fn set_grading_tone_value(&self, value: &crate::grading::GradingTone) {
        let values = value.to_flat_array();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_tone_set_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
    }

    pub fn grading_rgb_curve_num_control_points(&self, curve_type: RGBCurveType) -> i32 {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        }
    }

    pub fn grading_rgb_curve_set_num_control_points(&self, curve_type: RGBCurveType, num: i32) {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
                num,
            );
        }
    }

    pub fn grading_rgb_curve_control_point(
        &self,
        curve_type: RGBCurveType,
        index: i32,
    ) -> (f32, f32) {
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                &mut x,
                &mut y,
            );
        }
        (x, y)
    }

    pub fn grading_rgb_curve_set_control_point(
        &self,
        curve_type: RGBCurveType,
        index: i32,
        x: f32,
        y: f32,
    ) {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                x,
                y,
            );
        }
    }

    pub fn grading_rgb_curve_slope(&self, curve_type: RGBCurveType, index: i32) -> f32 {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
            )
        }
    }

    pub fn grading_rgb_curve_set_slope(&self, curve_type: RGBCurveType, index: i32, slope: f32) {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                slope,
            );
        }
    }

    pub fn grading_rgb_curve_slopes_are_default(&self, curve_type: RGBCurveType) -> bool {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_slopes_are_default(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        }
    }

    pub fn grading_hue_curve_num_control_points(&self, curve_type: HueCurveType) -> i32 {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_get_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        }
    }

    pub fn grading_hue_curve_set_num_control_points(&self, curve_type: HueCurveType, num: i32) {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_set_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
                num,
            );
        }
    }

    pub fn grading_hue_curve_control_point(
        &self,
        curve_type: HueCurveType,
        index: i32,
    ) -> (f32, f32) {
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_get_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                &mut x,
                &mut y,
            );
        }
        (x, y)
    }

    pub fn grading_hue_curve_set_control_point(
        &self,
        curve_type: HueCurveType,
        index: i32,
        x: f32,
        y: f32,
    ) {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_set_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                x,
                y,
            );
        }
    }

    pub fn grading_hue_curve_slope(&self, curve_type: HueCurveType, index: i32) -> f32 {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_get_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
            )
        }
    }

    pub fn grading_hue_curve_set_slope(&self, curve_type: HueCurveType, index: i32, slope: f32) {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_set_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                slope,
            );
        }
    }

    pub fn grading_hue_curve_slopes_are_default(&self, curve_type: HueCurveType) -> bool {
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_slopes_are_default(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        }
    }
}

impl Drop for DynamicProperty {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_dynamic_property_destroy(self.handle.as_ptr() as *mut c_void) };
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
    fn processor_named_optimization_wrappers_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        let _ = proc.optimized_processor_bitdepth(8, 8, 0);
        let _ = proc.optimized_cpu_processor_bitdepth(8, 8, 0);
    }

    #[test]
    #[allow(deprecated)]
    fn legacy_gpu_processor() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(gpu) = proc.optimized_legacy_gpu_processor(0, 32) {
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
    fn gpu_extract_shader_info_named_wrapper_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let (Ok(gpu), Ok(mut desc)) = (proc.default_gpu_processor(), GpuShaderDesc::create()) {
            gpu.extract_shader_info(&mut desc);
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
    fn gpu_shader_desc_texture_uid_no_crash() {
        if let Ok(desc) = GpuShaderDesc::create() {
            let _ = desc.texture_uid(0);
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
        let _ = proc.has_dynamic_property_kind(DynamicPropertyType::Exposure);
        // In stub mode, creating a dynamic property may or may not succeed
        if let Ok(dp) = proc.dynamic_property(DynamicPropertyType::Exposure) {
            let _ = dp.property_type();
            let _ = dp.double_value();
            dp.set_double_value(1.5);
        }
    }

    #[test]
    #[allow(deprecated)]
    fn processor_dynamic_property_compat_alias_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        let _ = proc.has_dynamic_property(DynamicPropertyType::Exposure as i32);
    }

    #[test]
    fn dynamic_property_grading_primary_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(dp) = proc.dynamic_property(DynamicPropertyType::GradingPrimary) {
            let _ = dp.grading_primary_value();
            let v = crate::grading::GradingPrimary::new(crate::GradingStyle::Log);
            dp.set_grading_primary_value(&v);
        }
    }

    #[test]
    fn dynamic_property_grading_tone_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(dp) = proc.dynamic_property(DynamicPropertyType::GradingTone) {
            let _ = dp.grading_tone_value();
            let v = crate::grading::GradingTone::new(crate::GradingStyle::Log);
            dp.set_grading_tone_value(&v);
        }
    }

    #[test]
    fn dynamic_property_grading_rgb_curve_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(dp) = proc.dynamic_property(DynamicPropertyType::GradingRgbCurve) {
            for ct in [
                RGBCurveType::Red,
                RGBCurveType::Green,
                RGBCurveType::Blue,
                RGBCurveType::Master,
            ] {
                let _ = dp.grading_rgb_curve_num_control_points(ct);
                let _ = dp.grading_rgb_curve_control_point(ct, 0);
                let _ = dp.grading_rgb_curve_slope(ct, 0);
                let _ = dp.grading_rgb_curve_slopes_are_default(ct);
            }
            dp.grading_rgb_curve_set_num_control_points(RGBCurveType::Red, 2);
            dp.grading_rgb_curve_set_control_point(RGBCurveType::Red, 0, 0.0, 0.0);
            dp.grading_rgb_curve_set_slope(RGBCurveType::Red, 0, 1.0);
        }
    }

    #[test]
    fn dynamic_property_grading_hue_curve_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(dp) = proc.dynamic_property(DynamicPropertyType::GradingHueCurve) {
            for ct in [
                HueCurveType::HueHue,
                HueCurveType::HueSat,
                HueCurveType::HueLum,
                HueCurveType::LumSat,
            ] {
                let _ = dp.grading_hue_curve_num_control_points(ct);
                let _ = dp.grading_hue_curve_control_point(ct, 0);
                let _ = dp.grading_hue_curve_slope(ct, 0);
                let _ = dp.grading_hue_curve_slopes_are_default(ct);
            }
            dp.grading_hue_curve_set_num_control_points(HueCurveType::HueHue, 2);
            dp.grading_hue_curve_set_control_point(HueCurveType::HueHue, 0, 0.0, 0.0);
            dp.grading_hue_curve_set_slope(HueCurveType::HueHue, 0, 1.0);
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
            cpu.apply_rgba_packed_bit_depth(&mut rgba, BitDepth::F32, 8, 4);
            let mut rgb = vec![0u8; 24]; // packed rgb bytes
            cpu.apply_rgb_packed_bit_depth(&mut rgb, BitDepth::F32, 8, 3);
        }
    }

    #[test]
    fn cpu_processor_dynamic_property_typed_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(cpu) = proc.default_cpu_processor() {
            let _ = cpu.has_dynamic_property_kind(DynamicPropertyType::Exposure);
            let _ = cpu.dynamic_property(DynamicPropertyType::Exposure);
        }
    }

    #[test]
    fn gpu_shader_desc_structured_accessors_no_crash() {
        if let Ok(desc) = GpuShaderDesc::create() {
            let _ = desc.clone_desc();
            let _ = desc.num_uniforms();
            let _ = desc.uniform_buffer_size();
            let _ = desc.uniform(0);
            let _ = desc.uniform_name(0);
            let _ = desc.uniform_value_count(0);
            let _ = desc.uniform_values_f32(0);
            let _ = desc.uniform_values_i32(0);
            let _ = desc.uniforms();
            let _ = desc.texture_2d(0);
            let _ = desc.texture_shader_binding_index(0);
            let _ = desc.texture_values(0);
            let _ = desc.texture_value_count(0);
            let _ = desc.textures_2d();
            let _ = desc.texture_3d(0);
            let _ = desc.texture_3d_value_count(0);
            let _ = desc.texture_3d_values(0);
            let _ = desc.texture_3d_shader_binding_index(0);
            let _ = desc.textures_3d();
        }
    }

    #[test]
    #[allow(deprecated)]
    fn gpu_shader_desc_compat_value_accessors_no_crash() {
        if let Ok(desc) = GpuShaderDesc::create() {
            let _ = desc.clone();
            let _ = desc.get_num_uniforms_u32();
            let _ = desc.get_uniform_buffer_size_bytes();
            let _ = desc.get_uniform_info(0);
            let _ = desc.get_uniform_value_count(0);
            let _ = desc.copy_uniform_f32_values(0);
            let _ = desc.copy_uniform_i32_values(0);
            let _ = desc.get_num_textures_u32();
            let _ = desc.get_texture_value_count(0);
            let _ = desc.copy_texture_values(0);
            let _ = desc.get_num3d_textures_u32();
            let _ = desc.get3d_texture_info(0);
            let _ = desc.get3d_texture_value_count(0);
            let _ = desc.copy3d_texture_values(0);
            let _ = desc.get_num3d_textures();
            let _ = desc.get3d_texture(0);
            let _ = desc.get3d_texture_values(0);
            let _ = desc.get3d_texture_shader_binding_index(0);
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
