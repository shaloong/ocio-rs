use std::ffi::c_void;
use std::ptr::NonNull;

use crate::transform::{transform_from_raw_handle, GroupTransform, Transform};
use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, BitDepth, DynamicPropertyType, FormatMetadata,
    GpuLanguage, HueCurveType, Interpolation, OcioError, ProcessorMetadata, RGBCurveType, Result,
};
use ocio_sys;

fn required_scalar_len(api: &str, num_pixels: i64, stride: i64) -> Result<usize> {
    let num_pixels = usize::try_from(num_pixels)
        .map_err(|_| OcioError::InvalidInput(format!("{api}: num_pixels must be non-negative")))?;
    let stride = usize::try_from(stride)
        .map_err(|_| OcioError::InvalidInput(format!("{api}: stride must be non-negative")))?;
    num_pixels
        .checked_mul(stride)
        .ok_or_else(|| OcioError::InvalidInput(format!("{api}: num_pixels * stride overflowed")))
}

fn bit_depth_bytes_per_channel(api: &str, bit_depth: BitDepth) -> Result<usize> {
    match bit_depth {
        BitDepth::Uint8 => Ok(1),
        BitDepth::Uint10
        | BitDepth::Uint12
        | BitDepth::Uint14
        | BitDepth::Uint16
        | BitDepth::F16 => Ok(2),
        BitDepth::Uint32 | BitDepth::F32 => Ok(4),
        BitDepth::Unknown => Err(OcioError::InvalidInput(format!(
            "{api}: BitDepth::Unknown is not valid for packed pixel IO"
        ))),
    }
}

fn validate_scalar_buffer_len(
    api: &str,
    actual_len: usize,
    num_pixels: i64,
    stride: i64,
) -> Result<()> {
    let required_len = required_scalar_len(api, num_pixels, stride)?;
    if actual_len < required_len {
        return Err(OcioError::InvalidInput(format!(
            "{api}: buffer too small for num_pixels={num_pixels}, stride={stride}; required at least {required_len} scalars, got {actual_len}"
        )));
    }
    Ok(())
}

fn validate_packed_buffer_len(
    api: &str,
    actual_len: usize,
    bit_depth: BitDepth,
    num_pixels: i64,
    stride: i64,
) -> Result<()> {
    let required_scalars = required_scalar_len(api, num_pixels, stride)?;
    let bytes_per_channel = bit_depth_bytes_per_channel(api, bit_depth)?;
    let required_len = required_scalars
        .checked_mul(bytes_per_channel)
        .ok_or_else(|| {
            OcioError::InvalidInput(format!("{api}: required packed buffer size overflowed"))
        })?;
    if actual_len < required_len {
        return Err(OcioError::InvalidInput(format!(
            "{api}: buffer too small for num_pixels={num_pixels}, stride={stride}, bit_depth={bit_depth:?}; required at least {required_len} bytes, got {actual_len}"
        )));
    }
    Ok(())
}

/// An immutable color-processing pipeline produced from a `Config`.
///
/// Use `default_cpu_processor` or `default_gpu_processor` to execute or extract
/// the processing implementation.
pub struct Processor {
    pub(crate) handle: NonNull<c_void>,
}

impl Processor {
    /// Return whether the processor is an identity/no-op pipeline.
    pub fn is_no_op(&self) -> bool {
        unsafe { ocio_sys::ocio_processor_is_no_op(self.handle.as_ptr() as *mut c_void) }
    }

    /// Return whether the processor mixes channels rather than operating lane-wise.
    pub fn has_channel_crosstalk(&self) -> bool {
        unsafe {
            ocio_sys::ocio_processor_has_channel_crosstalk(self.handle.as_ptr() as *mut c_void)
        }
    }

    /// Return OCIO's cache identifier for this processor instance.
    pub fn cache_id(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_processor_get_cache_id(
                self.handle.as_ptr() as *mut c_void
            ))
        }
    }

    /// Create the default CPU execution path for this processor.
    pub fn default_cpu_processor(&self) -> Result<CPUProcessor> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_get_default_cpu_processor(self.handle.as_ptr() as *mut c_void)
        };
        crate::handle_result(handle).map(|handle| CPUProcessor { handle })
    }

    /// Create an optimized CPU execution path for this processor.
    pub fn optimized_cpu_processor(&self, flags: u64) -> Result<CPUProcessor> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_cpu_processor(self.handle.as_ptr(), flags as i32)
        };
        crate::handle_result(handle).map(|handle| CPUProcessor { handle })
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer optimized_processor_bitdepth or optimized_cpu/gpu_processor helpers"
    )]
    pub fn optimized_processor_v1(&self, flags: u64) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_processor_v1(self.handle.as_ptr(), flags as i32)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an optimized processor variant for explicit input/output bit depths.
    ///
    /// This is mainly useful when matching a specific host application's pixel
    /// format contract before extracting CPU or GPU execution helpers.
    pub fn optimized_processor_bitdepth(
        &self,
        in_bit_depth: i32,
        out_bit_depth: i32,
        flags: u64,
    ) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_processor_v2(
                self.handle.as_ptr(),
                in_bit_depth,
                out_bit_depth,
                flags as i32,
            )
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    #[doc(hidden)]
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
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_get_default_gpu_processor(self.handle.as_ptr() as *mut c_void)
        };
        crate::handle_result(handle).map(|handle| GPUProcessor { handle })
    }

    /// Create an optimized GPU execution path for this processor.
    pub fn optimized_gpu_processor(&self, flags: u64) -> Result<GPUProcessor> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_gpu_processor(self.handle.as_ptr(), flags as i32)
        };
        crate::handle_result(handle).map(|handle| GPUProcessor { handle })
    }

    #[doc(hidden)]
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
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_legacy_gpu_processor(
                self.handle.as_ptr(),
                flags as i32,
                edge_len,
            )
        };
        crate::handle_result(handle).map(|handle| GPUProcessor { handle })
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer optimized_processor_bitdepth() or optimized_cpu/gpu_processor helpers"
    )]
    pub fn optimized_processor(&self, flags: u64) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_optimized_processor(self.handle.as_ptr(), flags as i32)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Borrow a runtime-adjustable dynamic property from the processor.
    pub fn dynamic_property(&self, property_type: DynamicPropertyType) -> Result<DynamicProperty> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_get_dynamic_property(
                self.handle.as_ptr(),
                property_type as i32,
            )
        };
        crate::handle_result(handle).map(|handle| DynamicProperty { handle })
    }

    /// Return the number of transforms represented by this processor.
    pub fn num_transforms(&self) -> i32 {
        unsafe { ocio_sys::ocio_processor_get_num_transforms(self.handle.as_ptr() as *mut c_void) }
    }

    /// Materialize the processor back into an equivalent group transform, when available.
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
    /// Return the top-level format metadata attached to the processor.
    pub fn format_metadata(&self) -> Option<FormatMetadata> {
        let h = unsafe {
            ocio_sys::ocio_processor_get_format_metadata(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(h).map(|h| FormatMetadata { handle: h })
    }

    /// Return format metadata for the transform at `index`, when exposed by OCIO.
    pub fn transform_format_metadata(&self, index: i32) -> Option<FormatMetadata> {
        let h = unsafe {
            ocio_sys::ocio_processor_get_transform_format_metadata(self.handle.as_ptr(), index)
        };
        NonNull::new(h).map(|h| FormatMetadata { handle: h })
    }

    /// Return technical processor metadata such as contributing files and looks.
    pub fn processor_metadata(&self) -> Option<ProcessorMetadata> {
        let h = unsafe {
            ocio_sys::ocio_processor_get_processor_metadata(self.handle.as_ptr() as *mut c_void)
        };
        NonNull::new(h).map(|h| ProcessorMetadata { handle: h })
    }

    /// Return whether the processor exposes a dynamic property of `prop_type`.
    pub fn has_dynamic_property_kind(&self, prop_type: DynamicPropertyType) -> bool {
        unsafe {
            ocio_sys::ocio_processor_has_dynamic_property(self.handle.as_ptr(), prop_type as i32)
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer has_dynamic_property_kind with DynamicPropertyType"
    )]
    pub fn has_dynamic_property(&self, prop_type: i32) -> bool {
        unsafe { ocio_sys::ocio_processor_has_dynamic_property(self.handle.as_ptr(), prop_type) }
    }

    /// Return whether this processor contains any runtime-adjustable properties.
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
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO image-descriptor entry point; prefer apply_rgb/apply_rgba/apply_*_pixels for Rust callers"
    )]
    pub unsafe fn apply(&self, img_desc: *mut c_void) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply(self.handle.as_ptr(), img_desc);
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer apply() or the typed pixel helpers"
    )]
    pub unsafe fn apply_v1(&self, img_desc: *mut c_void) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_v1(self.handle.as_ptr(), img_desc);
        }
    }

    /// # Safety
    /// `src_img_desc` and `dst_img_desc` must point to valid OCIO image descriptors.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO image-descriptor entry point; prefer apply_rgb/apply_rgba/apply_*_pixels for Rust callers"
    )]
    pub unsafe fn apply_v2(&self, src_img_desc: *mut c_void, dst_img_desc: *mut c_void) {
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_v2(self.handle.as_ptr(), src_img_desc, dst_img_desc);
        }
    }

    /// Apply the processor in place to one RGBA pixel.
    pub fn apply_rgba(&self, rgba: &mut [f32; 4]) {
        self.try_apply_rgba(rgba)
            .expect("CPUProcessor::apply_rgba failed");
    }

    /// Apply the processor in place to one RGBA pixel.
    pub fn try_apply_rgba(&self, rgba: &mut [f32; 4]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgba(
                self.handle.as_ptr(),
                rgba.as_mut_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    /// Apply the processor in place to one RGB pixel.
    pub fn apply_rgb(&self, rgb: &mut [f32; 3]) {
        self.try_apply_rgb(rgb)
            .expect("CPUProcessor::apply_rgb failed");
    }

    /// Apply the processor in place to one RGB pixel.
    pub fn try_apply_rgb(&self, rgb: &mut [f32; 3]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgb(
                self.handle.as_ptr(),
                rgb.as_mut_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    /// Apply the processor to an RGBA float buffer with an explicit scalar stride.
    ///
    /// `stride` is measured in `f32` elements, not bytes.
    pub fn apply_rgba_pixels(&self, rgba: &mut [f32], num_pixels: i64, stride: i64) {
        self.try_apply_rgba_pixels(rgba, num_pixels, stride)
            .expect("CPUProcessor::apply_rgba_pixels failed");
    }

    /// Apply the processor to an RGBA float buffer with an explicit scalar stride.
    ///
    /// `stride` is measured in `f32` elements, not bytes.
    pub fn try_apply_rgba_pixels(
        &self,
        rgba: &mut [f32],
        num_pixels: i64,
        stride: i64,
    ) -> Result<()> {
        validate_scalar_buffer_len(
            "CPUProcessor::apply_rgba_pixels",
            rgba.len(),
            num_pixels,
            stride,
        )?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgba_pixels(
                self.handle.as_ptr(),
                rgba.as_mut_ptr(),
                num_pixels,
                stride,
            );
        }
        crate::ocio_call_status()
    }

    /// Apply the processor to an RGB float buffer with an explicit scalar stride.
    ///
    /// `stride` is measured in `f32` elements, not bytes.
    pub fn apply_rgb_pixels(&self, rgb: &mut [f32], num_pixels: i64, stride: i64) {
        self.try_apply_rgb_pixels(rgb, num_pixels, stride)
            .expect("CPUProcessor::apply_rgb_pixels failed");
    }

    /// Apply the processor to an RGB float buffer with an explicit scalar stride.
    ///
    /// `stride` is measured in `f32` elements, not bytes.
    pub fn try_apply_rgb_pixels(
        &self,
        rgb: &mut [f32],
        num_pixels: i64,
        stride: i64,
    ) -> Result<()> {
        validate_scalar_buffer_len(
            "CPUProcessor::apply_rgb_pixels",
            rgb.len(),
            num_pixels,
            stride,
        )?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgb_pixels(
                self.handle.as_ptr(),
                rgb.as_mut_ptr(),
                num_pixels,
                stride,
            );
        }
        crate::ocio_call_status()
    }

    /// Apply the processor to packed RGBA bytes using an explicit OCIO bit depth.
    pub fn apply_rgba_packed_bit_depth(
        &self,
        rgba: &mut [u8],
        bit_depth: BitDepth,
        num_pixels: i64,
        stride: i64,
    ) {
        self.try_apply_rgba_packed_bit_depth(rgba, bit_depth, num_pixels, stride)
            .expect("CPUProcessor::apply_rgba_packed_bit_depth failed");
    }

    /// Apply the processor to packed RGBA bytes using an explicit OCIO bit depth.
    pub fn try_apply_rgba_packed_bit_depth(
        &self,
        rgba: &mut [u8],
        bit_depth: BitDepth,
        num_pixels: i64,
        stride: i64,
    ) -> Result<()> {
        validate_packed_buffer_len(
            "CPUProcessor::apply_rgba_packed_bit_depth",
            rgba.len(),
            bit_depth,
            num_pixels,
            stride,
        )?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgba_packed(
                self.handle.as_ptr(),
                rgba.as_mut_ptr() as *mut std::ffi::c_void,
                bit_depth as i32,
                num_pixels,
                stride,
            );
        }
        crate::ocio_call_status()
    }

    #[doc(hidden)]
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

    /// Apply the processor to packed RGB bytes using an explicit OCIO bit depth.
    pub fn apply_rgb_packed_bit_depth(
        &self,
        rgb: &mut [u8],
        bit_depth: BitDepth,
        num_pixels: i64,
        stride: i64,
    ) {
        self.try_apply_rgb_packed_bit_depth(rgb, bit_depth, num_pixels, stride)
            .expect("CPUProcessor::apply_rgb_packed_bit_depth failed");
    }

    /// Apply the processor to packed RGB bytes using an explicit OCIO bit depth.
    pub fn try_apply_rgb_packed_bit_depth(
        &self,
        rgb: &mut [u8],
        bit_depth: BitDepth,
        num_pixels: i64,
        stride: i64,
    ) -> Result<()> {
        validate_packed_buffer_len(
            "CPUProcessor::apply_rgb_packed_bit_depth",
            rgb.len(),
            bit_depth,
            num_pixels,
            stride,
        )?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_rgb_packed(
                self.handle.as_ptr(),
                rgb.as_mut_ptr() as *mut std::ffi::c_void,
                bit_depth as i32,
                num_pixels,
                stride,
            );
        }
        crate::ocio_call_status()
    }

    #[doc(hidden)]
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

    /// Return whether the CPU path is an identity/no-op transform.
    pub fn is_no_op(&self) -> bool {
        unsafe { ocio_sys::ocio_cpu_processor_is_no_op(self.handle.as_ptr() as *mut c_void) }
    }

    /// Return whether the CPU path mixes color channels.
    pub fn has_channel_crosstalk(&self) -> bool {
        unsafe {
            ocio_sys::ocio_cpu_processor_has_channel_crosstalk(self.handle.as_ptr() as *mut c_void)
        }
    }

    /// Return OCIO's cache identifier for this CPU processor instance.
    pub fn cache_id(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_cpu_processor_get_cache_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Return the declared input bit depth for this CPU path.
    pub fn input_bit_depth(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_cpu_processor_get_input_bit_depth(self.handle.as_ptr() as *mut c_void)
        }
    }

    /// Return the declared output bit depth for this CPU path.
    pub fn output_bit_depth(&self) -> i32 {
        unsafe {
            ocio_sys::ocio_cpu_processor_get_output_bit_depth(self.handle.as_ptr() as *mut c_void)
        }
    }

    /// Return whether this CPU path is functionally identity.
    pub fn is_identity(&self) -> bool {
        unsafe { ocio_sys::ocio_cpu_processor_is_identity(self.handle.as_ptr() as *mut c_void) }
    }

    // ── v2.5.1 ──
    /// Borrow a runtime-adjustable dynamic property from the CPU processor.
    pub fn dynamic_property(&self, prop_type: DynamicPropertyType) -> Option<DynamicProperty> {
        let h = unsafe {
            ocio_sys::ocio_cpu_processor_get_dynamic_property(
                self.handle.as_ptr(),
                prop_type as i32,
            )
        };
        NonNull::new(h).map(|h| DynamicProperty { handle: h })
    }

    #[doc(hidden)]
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

    /// Return whether the CPU processor exposes a dynamic property of `prop_type`.
    pub fn has_dynamic_property_kind(&self, prop_type: DynamicPropertyType) -> bool {
        unsafe {
            ocio_sys::ocio_cpu_processor_has_dynamic_property(
                self.handle.as_ptr(),
                prop_type as i32,
            )
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer has_dynamic_property_kind with DynamicPropertyType"
    )]
    pub fn has_dynamic_property(&self, prop_type: i32) -> bool {
        unsafe {
            ocio_sys::ocio_cpu_processor_has_dynamic_property(self.handle.as_ptr(), prop_type)
        }
    }

    /// Return whether this CPU processor contains any runtime-adjustable properties.
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
    /// Return whether the GPU path is an identity/no-op transform.
    pub fn is_no_op(&self) -> bool {
        unsafe { ocio_sys::ocio_gpu_processor_is_no_op(self.handle.as_ptr() as *mut c_void) }
    }

    /// Return whether the GPU path mixes color channels.
    pub fn has_channel_crosstalk(&self) -> bool {
        unsafe {
            ocio_sys::ocio_gpu_processor_has_channel_crosstalk(self.handle.as_ptr() as *mut c_void)
        }
    }

    /// Return OCIO's cache identifier for this GPU processor instance.
    pub fn cache_id(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_gpu_processor_get_cache_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Fill `shader_desc` with OCIO-generated shader text, uniforms, and textures.
    pub fn extract_shader_info(&self, shader_desc: &mut GpuShaderDesc) {
        unsafe {
            ocio_sys::ocio_gpu_processor_extract_gpu_shader_info_v1(
                self.handle.as_ptr(),
                shader_desc.handle.as_ptr(),
            );
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer extract_shader_info()")]
    pub fn extract_gpu_shader_info(&self, shader_desc: &mut GpuShaderDesc) {
        unsafe {
            ocio_sys::ocio_gpu_processor_extract_gpu_shader_info(
                self.handle.as_ptr(),
                shader_desc.handle.as_ptr(),
            );
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer extract_shader_info()")]
    pub fn extract_gpu_shader_info_v1(&self, shader_desc: &mut GpuShaderDesc) {
        self.extract_shader_info(shader_desc);
    }

    /// # Safety
    /// `shader_creator` must point to a valid OCIO shader creator object for the active ABI.
    #[doc(hidden)]
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
/// Channel layout used by extracted 1D/2D GPU texture payloads.
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
/// Dimensionality metadata for extracted 1D/2D GPU texture payloads.
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

fn required_ocio_string(ptr: *const i8) -> Option<String> {
    let value = unsafe { cstr_to_opt_string(ptr) }?;
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Uniform value encoding reported by OCIO GPU shader extraction.
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
    /// OCIO-generated texture symbol name used in emitted shader code.
    pub texture_name: String,
    /// Sampler symbol name paired with `texture_name` in the emitted shader.
    pub sampler_name: String,
    /// Logical texture width in texels.
    pub width: u32,
    /// Logical texture height in texels.
    pub height: u32,
    /// Channel packing used by the texture values.
    pub channel: GpuTextureChannel,
    /// Whether the resource is logically treated as 1D or 2D.
    pub dimensions: GpuTextureDimensions,
    /// Interpolation mode OCIO expects the texture to use.
    pub interpolation: Interpolation,
    /// API-facing binding slot reported by OCIO for this texture.
    pub binding_index: u32,
    /// Flattened texel payload in row-major order.
    pub values: Vec<f32>,
}

impl GpuTexture2D {
    /// Returns the number of `f32` values implied by the current metadata.
    pub fn expected_value_count(&self) -> usize {
        self.width as usize * self.height as usize * self.channel.channel_count()
    }
}

#[derive(Debug, Clone)]
/// Texture payload and metadata for a 3D GPU LUT resource.
pub struct GpuTexture3D {
    /// OCIO-generated texture symbol name used in emitted shader code.
    pub texture_name: String,
    /// Sampler symbol name paired with `texture_name` in the emitted shader.
    pub sampler_name: String,
    /// Cube edge length in texels.
    pub edge_len: u32,
    /// Interpolation mode OCIO expects the 3D LUT to use.
    pub interpolation: Interpolation,
    /// API-facing binding slot reported by OCIO for this texture.
    pub binding_index: u32,
    /// Flattened texel payload in OCIO's native 3D LUT ordering.
    pub values: Vec<f32>,
}

impl GpuTexture3D {
    /// Returns the number of `f32` values implied by the current metadata.
    pub fn expected_value_count(&self) -> usize {
        let edge = self.edge_len as usize;
        edge * edge * edge * 3
    }
}

#[derive(Debug, Clone)]
/// Typed value payload for a GPU uniform extracted from OCIO.
pub enum GpuUniformValue {
    /// Floating-point uniform payload.
    F32(Vec<f32>),
    /// Integer uniform payload.
    I32(Vec<i32>),
    /// Uniform payload could not be represented through the current Rust helper.
    Unsupported,
}

#[derive(Debug, Clone)]
/// GPU uniform metadata and current value payload extracted from OCIO.
pub struct GpuUniform {
    /// Uniform symbol name used in the emitted shader.
    pub name: String,
    /// OCIO-reported uniform value encoding.
    pub uniform_type: GpuUniformType,
    /// Byte offset into the packed uniform buffer layout, when applicable.
    pub buffer_offset: usize,
    /// Logical scalar count for the current uniform payload.
    pub value_count: usize,
    /// Typed uniform payload copied into Rust-owned memory.
    pub value: GpuUniformValue,
}

impl GpuShaderDesc {
    /// Creates an empty OCIO GPU shader descriptor that can be configured and
    /// passed to `GPUProcessor::extract_shader_info`.
    pub fn create() -> Result<Self> {
        let handle = unsafe { ocio_sys::ocio_gpu_shader_desc_create() };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    /// Returns the extracted shader source text, if any.
    pub fn shader_text(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_gpu_shader_desc_get_shader_text(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Returns the number of reported 1D/2D texture resources.
    pub fn num_textures(&self) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_num_textures_u32(self.handle.as_ptr()) }
    }

    /// Returns lightweight legacy metadata for a 1D/2D texture resource.
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

    /// Returns a structured 1D/2D texture resource and copied texel payload.
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
        let texture_name = required_ocio_string(info.texture_name)?;
        let sampler_name = required_ocio_string(info.sampler_name)?;
        Some(GpuTexture2D {
            texture_name,
            sampler_name,
            width: info.width,
            height: info.height,
            channel: GpuTextureChannel::from_raw(info.channel),
            dimensions: GpuTextureDimensions::from_raw(info.dimensions),
            interpolation: interpolation_from_raw(info.interpolation),
            binding_index: info.binding_index,
            values,
        })
    }

    /// Returns all structured 1D/2D texture resources currently reported by OCIO.
    pub fn textures_2d(&self) -> Vec<GpuTexture2D> {
        (0..self.num_textures())
            .filter_map(|index| self.texture_2d(index))
            .collect()
    }

    /// Returns the shader language currently configured on the descriptor.
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

    /// Sets the shader language OCIO should target during extraction.
    pub fn set_language(&self, language: GpuLanguage) {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_language(self.handle.as_ptr(), language as i32);
        }
    }

    /// Returns the configured shader entry-point name, if any.
    pub fn function_name(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_function_name(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Sets the shader entry-point name used during extraction.
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

    /// Returns the configured pixel variable name, if any.
    pub fn pixel_name(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_pixel_name(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Sets the pixel variable name used in emitted shader code.
    pub fn set_pixel_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_pixel_name(self.handle.as_ptr(), n.as_ptr().cast());
        }
        Ok(())
    }

    /// Returns the explicit unique identifier configured for shader extraction, if any.
    pub fn unique_id(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_unique_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Sets the unique identifier OCIO should use for generated shader resources.
    pub fn set_unique_id(&self, uid: impl AsRef<str>) -> Result<()> {
        let uid = cstring(uid)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_unique_id(self.handle.as_ptr(), uid.as_ptr().cast());
        }
        Ok(())
    }

    /// Returns the configured resource-name prefix, if any.
    pub fn resource_prefix(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_resource_prefix(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Sets the prefix OCIO uses for generated resource names.
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

    /// Configures the descriptor-set index and starting texture-binding slot used by OCIO.
    pub fn set_descriptor_set_index(&self, index: u32, texture_binding_start: u32) {
        self.try_set_descriptor_set_index(index, texture_binding_start)
            .expect("GpuShaderDesc::set_descriptor_set_index failed");
    }

    /// Configures the descriptor-set index and starting texture-binding slot used by OCIO.
    pub fn try_set_descriptor_set_index(
        &self,
        index: u32,
        texture_binding_start: u32,
    ) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_descriptor_set_index(
                self.handle.as_ptr(),
                index,
                texture_binding_start,
            );
        }
        crate::ocio_call_status()
    }

    /// Returns the configured descriptor-set index.
    pub fn descriptor_set_index(&self) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_descriptor_set_index(self.handle.as_ptr()) }
    }

    /// Returns the configured starting binding slot for extracted textures.
    pub fn texture_binding_start(&self) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_texture_binding_start(self.handle.as_ptr()) }
    }

    /// Sets the maximum width OCIO may use when laying out extracted 1D textures.
    pub fn set_texture_max_width(&self, max_width: u32) {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_texture_max_width_u32(
                self.handle.as_ptr(),
                max_width,
            );
        }
    }

    /// Controls whether OCIO may use native 1D textures instead of always promoting to 2D.
    pub fn set_allow_texture_1d(&self, allowed: bool) {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_allow_texture_1d(self.handle.as_ptr(), allowed);
        }
    }

    /// Returns whether OCIO may use native 1D textures during extraction.
    pub fn allow_texture_1d(&self) -> bool {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_allow_texture_1d(self.handle.as_ptr()) }
    }

    /// Marks the beginning of shader-data collection with the provided OCIO resource UID.
    pub fn begin(&self, uid: impl AsRef<str>) -> Result<()> {
        let uid = cstring(uid)?;
        unsafe { ocio_sys::ocio_gpu_shader_desc_begin(self.handle.as_ptr(), uid.as_ptr()) };
        Ok(())
    }

    /// Marks the end of shader-data collection.
    pub fn end(&self) {
        unsafe { ocio_sys::ocio_gpu_shader_desc_end(self.handle.as_ptr()) };
    }

    /// Returns the next OCIO-managed resource index and advances the internal counter.
    pub fn next_resource_index(&self) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_next_resource_index(self.handle.as_ptr()) }
    }

    /// Appends text to the shader's parameter-declaration section.
    pub fn add_to_parameter_declare_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_parameter_declare_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        Ok(())
    }

    /// Appends text to the shader's texture-declaration section.
    pub fn add_to_texture_declare_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_texture_declare_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        Ok(())
    }

    /// Appends text to the shader's helper-method section.
    pub fn add_to_helper_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_helper_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        Ok(())
    }

    /// Appends text to the shader function's header section.
    pub fn add_to_function_header_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_function_header_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        Ok(())
    }

    /// Appends text to the shader function's body section.
    pub fn add_to_function_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_function_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        Ok(())
    }

    /// Appends text to the shader function's footer section.
    pub fn add_to_function_footer_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_function_footer_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        Ok(())
    }

    /// Returns the copied texel payload for a 1D/2D texture resource.
    pub fn texture_values(&self, index: u32) -> Vec<f32> {
        self.texture_2d(index)
            .map(|texture| texture.values)
            .unwrap_or_default()
    }

    /// Adds a manual 1D/2D texture resource to the descriptor and returns its OCIO binding index.
    #[allow(clippy::too_many_arguments)]
    pub fn add_texture_2d(
        &self,
        texture_name: impl AsRef<str>,
        sampler_name: impl AsRef<str>,
        width: u32,
        height: u32,
        channel: GpuTextureChannel,
        dimensions: GpuTextureDimensions,
        interpolation: Interpolation,
        values: &[f32],
    ) -> Result<u32> {
        let expected = width as usize * height as usize * channel.channel_count();
        if values.len() != expected {
            return Err(OcioError::ValidationFailed(format!(
                "gpu texture value count mismatch: expected {expected}, got {}",
                values.len()
            )));
        }
        let texture_name = cstring(texture_name)?;
        let sampler_name = cstring(sampler_name)?;
        let binding_index = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_texture(
                self.handle.as_ptr(),
                texture_name.as_ptr(),
                sampler_name.as_ptr(),
                width,
                height,
                channel as i32,
                dimensions as i32,
                interpolation as i32,
                values.as_ptr(),
                values.len(),
            )
        };
        if binding_index == 0 {
            Err(OcioError::AllocationFailed)
        } else {
            Ok(binding_index)
        }
    }

    /// Rebuilds the full shader text from the provided OCIO shader sections.
    pub fn create_shader_text(
        &self,
        shader_parameter_declarations: impl AsRef<str>,
        shader_texture_declarations: impl AsRef<str>,
        shader_helper_methods: impl AsRef<str>,
        shader_function_header: impl AsRef<str>,
        shader_function_body: impl AsRef<str>,
        shader_function_footer: impl AsRef<str>,
    ) -> Result<()> {
        let shader_parameter_declarations = cstring(shader_parameter_declarations)?;
        let shader_texture_declarations = cstring(shader_texture_declarations)?;
        let shader_helper_methods = cstring(shader_helper_methods)?;
        let shader_function_header = cstring(shader_function_header)?;
        let shader_function_body = cstring(shader_function_body)?;
        let shader_function_footer = cstring(shader_function_footer)?;
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_create_shader_text(
                self.handle.as_ptr(),
                shader_parameter_declarations.as_ptr(),
                shader_texture_declarations.as_ptr(),
                shader_helper_methods.as_ptr(),
                shader_function_header.as_ptr(),
                shader_function_body.as_ptr(),
                shader_function_footer.as_ptr(),
            );
        }
        Ok(())
    }

    /// Finalizes descriptor configuration before extraction when OCIO requires it.
    pub fn finalize(&self) {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_finalize(self.handle.as_ptr() as *mut c_void);
        }
    }

    /// Returns the maximum width OCIO would like to use for the given texture.
    pub fn texture_max_width(&self, index: i32) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_texture_max_width(self.handle.as_ptr(), index) }
    }

    /// Returns the maximum height OCIO would like to use for the given texture.
    pub fn texture_max_height(&self, index: i32) -> u32 {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_texture_max_height(self.handle.as_ptr(), index)
        }
    }

    /// Returns OCIO's cache identifier for the current descriptor configuration.
    pub fn cache_id(&self) -> Option<String> {
        unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_cache_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Returns the OCIO texture UID associated with a 1D/2D resource, if any.
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
    /// function name, pixel name, resource prefix, and descriptor-set settings.
    /// Lower-level creator implementation knobs such as 1D texture width and
    /// `allow_texture_1d` may fall back to OCIO defaults on clone, and
    /// extracted shader payloads are not guaranteed to be copied into the clone.
    pub fn clone_desc(&self) -> Option<GpuShaderDesc> {
        let h =
            unsafe { ocio_sys::ocio_gpu_shader_desc_clone(self.handle.as_ptr() as *mut c_void) };
        NonNull::new(h).map(|h| GpuShaderDesc { handle: h })
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer clone_desc()")]
    #[allow(clippy::should_implement_trait)]
    pub fn clone(&self) -> Option<GpuShaderDesc> {
        self.clone_desc()
    }

    /// Returns the number of reported uniforms.
    pub fn num_uniforms(&self) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_num_uniforms_u32(self.handle.as_ptr()) }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_uniforms()")]
    pub fn get_num_uniforms_u32(&self) -> u32 {
        self.num_uniforms()
    }

    /// Returns the size in bytes of OCIO's packed uniform buffer layout.
    pub fn uniform_buffer_size(&self) -> usize {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_uniform_buffer_size_bytes(self.handle.as_ptr())
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer uniform_buffer_size()")]
    pub fn get_uniform_buffer_size_bytes(&self) -> usize {
        self.uniform_buffer_size()
    }

    /// Adds a scalar floating-point uniform and returns `false` when the name already exists.
    pub fn add_uniform_f64(&self, name: impl AsRef<str>, value: f64) -> Result<bool> {
        let name = cstring(name)?;
        Ok(unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_double(
                self.handle.as_ptr(),
                name.as_ptr(),
                value,
            )
        })
    }

    /// Adds a boolean uniform and returns `false` when the name already exists.
    pub fn add_uniform_bool(&self, name: impl AsRef<str>, value: bool) -> Result<bool> {
        let name = cstring(name)?;
        Ok(unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_bool(
                self.handle.as_ptr(),
                name.as_ptr(),
                value,
            )
        })
    }

    /// Adds a three-component floating-point uniform and returns `false` when the name already exists.
    pub fn add_uniform_float3(&self, name: impl AsRef<str>, value: [f32; 3]) -> Result<bool> {
        let name = cstring(name)?;
        Ok(unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_float3(
                self.handle.as_ptr(),
                name.as_ptr(),
                value[0],
                value[1],
                value[2],
            )
        })
    }

    /// Adds a floating-point array uniform and returns `false` when the name already exists.
    pub fn add_uniform_f32_array(
        &self,
        name: impl AsRef<str>,
        values: &[f32],
        max_size: u32,
    ) -> Result<bool> {
        if max_size < values.len() as u32 {
            return Err(OcioError::ValidationFailed(format!(
                "gpu uniform max_size {} is smaller than value count {}",
                max_size,
                values.len()
            )));
        }
        let name = cstring(name)?;
        Ok(unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_vector_float(
                self.handle.as_ptr(),
                name.as_ptr(),
                values.as_ptr(),
                values.len(),
                max_size,
            )
        })
    }

    /// Adds an integer array uniform and returns `false` when the name already exists.
    pub fn add_uniform_i32_array(
        &self,
        name: impl AsRef<str>,
        values: &[i32],
        max_size: u32,
    ) -> Result<bool> {
        if max_size < values.len() as u32 {
            return Err(OcioError::ValidationFailed(format!(
                "gpu uniform max_size {} is smaller than value count {}",
                max_size,
                values.len()
            )));
        }
        let name = cstring(name)?;
        Ok(unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_vector_int(
                self.handle.as_ptr(),
                name.as_ptr(),
                values.as_ptr(),
                values.len(),
                max_size,
            )
        })
    }

    /// Returns the number of dynamic properties attached to the descriptor.
    pub fn num_dynamic_properties(&self) -> u32 {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_num_dynamic_properties_u32(self.handle.as_ptr())
        }
    }

    /// Returns a dynamic property by ordinal index, if present.
    pub fn dynamic_property_by_index(&self, index: u32) -> Option<DynamicProperty> {
        let handle = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_dynamic_property_by_index(
                self.handle.as_ptr(),
                index,
            )
        };
        NonNull::new(handle).map(|handle| DynamicProperty { handle })
    }

    /// Returns the dynamic property associated with the given OCIO property kind, if present.
    pub fn dynamic_property(&self, property_type: DynamicPropertyType) -> Option<DynamicProperty> {
        let handle = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_dynamic_property(
                self.handle.as_ptr(),
                property_type as i32,
            )
        };
        NonNull::new(handle).map(|handle| DynamicProperty { handle })
    }

    /// Returns whether the descriptor exposes the given OCIO dynamic property kind.
    pub fn has_dynamic_property_kind(&self, prop_type: DynamicPropertyType) -> bool {
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_has_dynamic_property(
                self.handle.as_ptr(),
                prop_type as i32,
            )
        }
    }

    /// Returns a structured uniform record with Rust-owned payload values.
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
        let name = required_ocio_string(info.name)?;
        Some(GpuUniform {
            name,
            uniform_type,
            buffer_offset: info.buffer_offset,
            value_count: info.value_count,
            value,
        })
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer uniform()")]
    pub fn get_uniform_info(&self, index: u32) -> Option<GpuUniform> {
        self.uniform(index)
    }

    /// Returns the scalar value count for a uniform, or `0` when absent.
    pub fn uniform_value_count(&self, index: u32) -> usize {
        self.uniform(index)
            .map(|uniform| uniform.value_count)
            .unwrap_or(0)
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer uniform_value_count()")]
    pub fn get_uniform_value_count(&self, index: u32) -> usize {
        self.uniform_value_count(index)
    }

    /// Returns copied floating-point uniform values when the uniform uses an `f32` payload.
    pub fn uniform_values_f32(&self, index: u32) -> Vec<f32> {
        match self.uniform(index).map(|uniform| uniform.value) {
            Some(GpuUniformValue::F32(values)) => values,
            _ => Vec::new(),
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer uniform_values_f32() or uniform()"
    )]
    pub fn copy_uniform_f32_values(&self, index: u32) -> Vec<f32> {
        self.uniform_values_f32(index)
    }

    /// Returns copied integer uniform values when the uniform uses an `i32` payload.
    pub fn uniform_values_i32(&self, index: u32) -> Vec<i32> {
        match self.uniform(index).map(|uniform| uniform.value) {
            Some(GpuUniformValue::I32(values)) => values,
            _ => Vec::new(),
        }
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer uniform_values_i32() or uniform()"
    )]
    pub fn copy_uniform_i32_values(&self, index: u32) -> Vec<i32> {
        self.uniform_values_i32(index)
    }

    /// Returns all structured uniform records currently reported by OCIO.
    pub fn uniforms(&self) -> Vec<GpuUniform> {
        (0..self.num_uniforms())
            .filter_map(|index| self.uniform(index))
            .collect()
    }

    /// Returns the number of reported 3D texture resources.
    pub fn num_3d_textures(&self) -> u32 {
        unsafe { ocio_sys::ocio_gpu_shader_desc_get_num3d_textures_u32(self.handle.as_ptr()) }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_textures()")]
    pub fn get_num_textures_u32(&self) -> u32 {
        self.num_textures()
    }

    /// Returns the copied scalar count for a 1D/2D texture resource.
    pub fn texture_value_count(&self, index: u32) -> usize {
        self.texture_values(index).len()
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer texture_value_count()")]
    pub fn get_texture_value_count(&self, index: u32) -> usize {
        self.texture_value_count(index)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer texture_2d() or textures_2d()"
    )]
    pub fn copy_texture_values(&self, index: u32) -> Vec<f32> {
        self.texture_values(index)
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_3d_textures()")]
    pub fn get_num3d_textures_u32(&self) -> u32 {
        self.num_3d_textures()
    }

    /// Returns a structured 3D texture resource and copied texel payload.
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
        let texture_name = required_ocio_string(info.texture_name)?;
        let sampler_name = required_ocio_string(info.sampler_name)?;
        Some(GpuTexture3D {
            texture_name,
            sampler_name,
            edge_len: info.edge_len,
            interpolation: interpolation_from_raw(info.interpolation),
            binding_index: info.binding_index,
            values,
        })
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer texture_3d()")]
    pub fn get3d_texture_info(&self, index: u32) -> Option<GpuTexture3D> {
        self.texture_3d(index)
    }

    /// Returns the copied scalar count for a 3D texture resource.
    pub fn texture_3d_value_count(&self, index: u32) -> usize {
        self.texture_3d(index)
            .map(|texture| texture.values.len())
            .unwrap_or(0)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer texture_3d_value_count()"
    )]
    pub fn get3d_texture_value_count(&self, index: u32) -> usize {
        self.texture_3d_value_count(index)
    }

    /// Returns the copied texel payload for a 3D texture resource.
    pub fn texture_3d_values(&self, index: u32) -> Vec<f32> {
        self.texture_3d(index)
            .map(|texture| texture.values)
            .unwrap_or_default()
    }

    /// Adds a manual 3D texture resource to the descriptor and returns its OCIO binding index.
    pub fn add_texture_3d(
        &self,
        texture_name: impl AsRef<str>,
        sampler_name: impl AsRef<str>,
        edge_len: u32,
        interpolation: Interpolation,
        values: &[f32],
    ) -> Result<u32> {
        let edge = edge_len as usize;
        let expected = edge * edge * edge * 3;
        if values.len() != expected {
            return Err(OcioError::ValidationFailed(format!(
                "gpu 3d texture value count mismatch: expected {expected}, got {}",
                values.len()
            )));
        }
        let texture_name = cstring(texture_name)?;
        let sampler_name = cstring(sampler_name)?;
        let binding_index = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add3d_texture(
                self.handle.as_ptr(),
                texture_name.as_ptr(),
                sampler_name.as_ptr(),
                edge_len,
                interpolation as i32,
                values.as_ptr(),
                values.len(),
            )
        };
        if binding_index == 0 {
            Err(OcioError::AllocationFailed)
        } else {
            Ok(binding_index)
        }
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer texture_3d_values()")]
    pub fn copy3d_texture_values(&self, index: u32) -> Vec<f32> {
        self.texture_3d_values(index)
    }

    /// Returns all structured 3D texture resources currently reported by OCIO.
    pub fn textures_3d(&self) -> Vec<GpuTexture3D> {
        (0..self.num_3d_textures())
            .filter_map(|index| self.texture_3d(index))
            .collect()
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_3d_textures()")]
    pub fn get_num3d_textures(&self) -> u32 {
        self.num_3d_textures()
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer texture_3d()")]
    pub fn get3d_texture(&self, index: u32) -> Option<GpuTexture3D> {
        self.texture_3d(index)
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer texture_3d_values()")]
    pub fn get3d_texture_values(&self, index: u32) -> Vec<f32> {
        self.texture_3d_values(index)
    }

    /// Returns the binding index for a 3D texture resource, if present.
    pub fn texture_3d_shader_binding_index(&self, index: u32) -> Option<u32> {
        self.texture_3d(index).map(|texture| texture.binding_index)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer texture_3d_shader_binding_index()"
    )]
    pub fn get3d_texture_shader_binding_index(&self, index: u32) -> Option<u32> {
        self.texture_3d_shader_binding_index(index)
    }

    /// Returns the binding index for a 1D/2D texture resource, if present.
    pub fn texture_shader_binding_index(&self, index: u32) -> Option<u32> {
        self.texture_2d(index).map(|texture| texture.binding_index)
    }

    /// Returns the uniform symbol name for the given index, if present.
    pub fn uniform_name(&self, index: u32) -> Option<String> {
        self.uniform(index).map(|uniform| uniform.name)
    }
}

impl Drop for GpuShaderDesc {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_gpu_shader_desc_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

#[doc(hidden)]
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
    fn require_property_type(
        &self,
        expected: DynamicPropertyType,
        operation: &'static str,
    ) -> Result<()> {
        let actual = self.property_type();
        if actual == expected {
            Ok(())
        } else {
            Err(OcioError::InvalidInput(format!(
                "{operation} requires {:?}, got {:?}",
                expected, actual
            )))
        }
    }

    fn require_double_property(&self, operation: &'static str) -> Result<()> {
        let actual = self.property_type();
        match actual {
            DynamicPropertyType::Exposure
            | DynamicPropertyType::Contrast
            | DynamicPropertyType::Gamma => Ok(()),
            _ => Err(OcioError::InvalidInput(format!(
                "{operation} requires Exposure, Contrast, or Gamma, got {:?}",
                actual
            ))),
        }
    }

    fn require_grading_rgb_curve_property(&self, operation: &'static str) -> Result<()> {
        self.require_property_type(DynamicPropertyType::GradingRgbCurve, operation)
    }

    fn require_grading_hue_curve_property(&self, operation: &'static str) -> Result<()> {
        self.require_property_type(DynamicPropertyType::GradingHueCurve, operation)
    }

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

    pub fn double_value(&self) -> Result<f64> {
        self.require_double_property("DynamicProperty::double_value")?;
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_dynamic_property_double_get_value(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    pub fn set_double_value(&self, value: f64) -> Result<()> {
        self.require_double_property("DynamicProperty::set_double_value")?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_dynamic_property_double_set_value(self.handle.as_ptr(), value) };
        crate::ocio_call_status()
    }

    pub fn grading_primary_value(&self) -> Result<crate::grading::GradingPrimary> {
        self.require_property_type(
            DynamicPropertyType::GradingPrimary,
            "DynamicProperty::grading_primary_value",
        )?;
        let mut values = [0.0f64; 34];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_primary_get_value(
                self.handle.as_ptr(),
                values.as_mut_ptr(),
            );
        }
        crate::ocio_call_status()?;
        Ok(crate::grading::GradingPrimary::from_flat_array(&values))
    }

    pub fn set_grading_primary_value(&self, value: &crate::grading::GradingPrimary) -> Result<()> {
        self.require_property_type(
            DynamicPropertyType::GradingPrimary,
            "DynamicProperty::set_grading_primary_value",
        )?;
        let values = value.to_flat_array();
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_primary_set_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn grading_tone_value(&self) -> Result<crate::grading::GradingTone> {
        self.require_property_type(
            DynamicPropertyType::GradingTone,
            "DynamicProperty::grading_tone_value",
        )?;
        let mut values = [0.0f64; 31];
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_tone_get_value(
                self.handle.as_ptr(),
                values.as_mut_ptr(),
            );
        }
        crate::ocio_call_status()?;
        Ok(crate::grading::GradingTone::from_flat_array(&values))
    }

    pub fn set_grading_tone_value(&self, value: &crate::grading::GradingTone) -> Result<()> {
        self.require_property_type(
            DynamicPropertyType::GradingTone,
            "DynamicProperty::set_grading_tone_value",
        )?;
        let values = value.to_flat_array();
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_tone_set_value(
                self.handle.as_ptr(),
                values.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    pub fn grading_rgb_curve_num_control_points(&self, curve_type: RGBCurveType) -> Result<i32> {
        self.require_grading_rgb_curve_property(
            "DynamicProperty::grading_rgb_curve_num_control_points",
        )?;
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    pub fn grading_rgb_curve_set_num_control_points(
        &self,
        curve_type: RGBCurveType,
        num: i32,
    ) -> Result<()> {
        self.require_grading_rgb_curve_property(
            "DynamicProperty::grading_rgb_curve_set_num_control_points",
        )?;
        if num < 0 {
            return Err(OcioError::InvalidInput(
                "DynamicProperty::grading_rgb_curve_set_num_control_points: num must be non-negative"
                    .to_string(),
            ));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
                num,
            );
        }
        crate::ocio_call_status()
    }

    pub fn grading_rgb_curve_control_point(
        &self,
        curve_type: RGBCurveType,
        index: i32,
    ) -> Result<(f32, f32)> {
        self.require_grading_rgb_curve_property(
            "DynamicProperty::grading_rgb_curve_control_point",
        )?;
        if index < 0 {
            return Err(OcioError::InvalidInput(
                "DynamicProperty::grading_rgb_curve_control_point: index must be non-negative"
                    .to_string(),
            ));
        }
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                &mut x,
                &mut y,
            );
        }
        crate::ocio_call_status()?;
        Ok((x, y))
    }

    pub fn grading_rgb_curve_set_control_point(
        &self,
        curve_type: RGBCurveType,
        index: i32,
        x: f32,
        y: f32,
    ) -> Result<()> {
        self.require_grading_rgb_curve_property(
            "DynamicProperty::grading_rgb_curve_set_control_point",
        )?;
        if index < 0 {
            return Err(OcioError::InvalidInput(
                "DynamicProperty::grading_rgb_curve_set_control_point: index must be non-negative"
                    .to_string(),
            ));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                x,
                y,
            );
        }
        crate::ocio_call_status()
    }

    pub fn grading_rgb_curve_slope(&self, curve_type: RGBCurveType, index: i32) -> Result<f32> {
        self.require_grading_rgb_curve_property("DynamicProperty::grading_rgb_curve_slope")?;
        if index < 0 {
            return Err(OcioError::InvalidInput(
                "DynamicProperty::grading_rgb_curve_slope: index must be non-negative".to_string(),
            ));
        }
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    pub fn grading_rgb_curve_set_slope(
        &self,
        curve_type: RGBCurveType,
        index: i32,
        slope: f32,
    ) -> Result<()> {
        self.require_grading_rgb_curve_property("DynamicProperty::grading_rgb_curve_set_slope")?;
        if index < 0 {
            return Err(OcioError::InvalidInput(
                "DynamicProperty::grading_rgb_curve_set_slope: index must be non-negative"
                    .to_string(),
            ));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                slope,
            );
        }
        crate::ocio_call_status()
    }

    pub fn grading_rgb_curve_slopes_are_default(&self, curve_type: RGBCurveType) -> Result<bool> {
        self.require_grading_rgb_curve_property(
            "DynamicProperty::grading_rgb_curve_slopes_are_default",
        )?;
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_slopes_are_default(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    pub fn grading_hue_curve_num_control_points(&self, curve_type: HueCurveType) -> Result<i32> {
        self.require_grading_hue_curve_property(
            "DynamicProperty::grading_hue_curve_num_control_points",
        )?;
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_get_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    pub fn grading_hue_curve_set_num_control_points(
        &self,
        curve_type: HueCurveType,
        num: i32,
    ) -> Result<()> {
        self.require_grading_hue_curve_property(
            "DynamicProperty::grading_hue_curve_set_num_control_points",
        )?;
        if num < 0 {
            return Err(OcioError::InvalidInput(
                "DynamicProperty::grading_hue_curve_set_num_control_points: num must be non-negative"
                    .to_string(),
            ));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_set_num_control_points(
                self.handle.as_ptr(),
                curve_type as i32,
                num,
            );
        }
        crate::ocio_call_status()
    }

    pub fn grading_hue_curve_control_point(
        &self,
        curve_type: HueCurveType,
        index: i32,
    ) -> Result<(f32, f32)> {
        self.require_grading_hue_curve_property(
            "DynamicProperty::grading_hue_curve_control_point",
        )?;
        if index < 0 {
            return Err(OcioError::InvalidInput(
                "DynamicProperty::grading_hue_curve_control_point: index must be non-negative"
                    .to_string(),
            ));
        }
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_get_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                &mut x,
                &mut y,
            );
        }
        crate::ocio_call_status()?;
        Ok((x, y))
    }

    pub fn grading_hue_curve_set_control_point(
        &self,
        curve_type: HueCurveType,
        index: i32,
        x: f32,
        y: f32,
    ) -> Result<()> {
        self.require_grading_hue_curve_property(
            "DynamicProperty::grading_hue_curve_set_control_point",
        )?;
        if index < 0 {
            return Err(OcioError::InvalidInput(
                "DynamicProperty::grading_hue_curve_set_control_point: index must be non-negative"
                    .to_string(),
            ));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_set_control_point(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                x,
                y,
            );
        }
        crate::ocio_call_status()
    }

    pub fn grading_hue_curve_slope(&self, curve_type: HueCurveType, index: i32) -> Result<f32> {
        self.require_grading_hue_curve_property("DynamicProperty::grading_hue_curve_slope")?;
        if index < 0 {
            return Err(OcioError::InvalidInput(
                "DynamicProperty::grading_hue_curve_slope: index must be non-negative".to_string(),
            ));
        }
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_get_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    pub fn grading_hue_curve_set_slope(
        &self,
        curve_type: HueCurveType,
        index: i32,
        slope: f32,
    ) -> Result<()> {
        self.require_grading_hue_curve_property("DynamicProperty::grading_hue_curve_set_slope")?;
        if index < 0 {
            return Err(OcioError::InvalidInput(
                "DynamicProperty::grading_hue_curve_set_slope: index must be non-negative"
                    .to_string(),
            ));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_set_slope(
                self.handle.as_ptr(),
                curve_type as i32,
                index,
                slope,
            );
        }
        crate::ocio_call_status()
    }

    pub fn grading_hue_curve_slopes_are_default(&self, curve_type: HueCurveType) -> Result<bool> {
        self.require_grading_hue_curve_property(
            "DynamicProperty::grading_hue_curve_slopes_are_default",
        )?;
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_slopes_are_default(
                self.handle.as_ptr(),
                curve_type as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
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
    fn processor_metadata() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        let _ = proc.is_no_op();
        let _ = proc.has_channel_crosstalk();
        let _ = proc.cache_id();
        if let Some(metadata) = proc.processor_metadata() {
            let _ = metadata.num_files();
            let _ = metadata.file(0);
            let _ = metadata.num_looks();
            let _ = metadata.look(0);
        }
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
            let _ = dp.set_double_value(1.5);
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
            let _ = dp.set_grading_primary_value(&v);
        }
    }

    #[test]
    fn dynamic_property_grading_tone_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(dp) = proc.dynamic_property(DynamicPropertyType::GradingTone) {
            let _ = dp.grading_tone_value();
            let v = crate::grading::GradingTone::new(crate::GradingStyle::Log);
            let _ = dp.set_grading_tone_value(&v);
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
            let _ = dp.grading_rgb_curve_set_num_control_points(RGBCurveType::Red, 2);
            let _ = dp.grading_rgb_curve_set_control_point(RGBCurveType::Red, 0, 0.0, 0.0);
            let _ = dp.grading_rgb_curve_set_slope(RGBCurveType::Red, 0, 1.0);
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
            let _ = dp.grading_hue_curve_set_num_control_points(HueCurveType::HueHue, 2);
            let _ = dp.grading_hue_curve_set_control_point(HueCurveType::HueHue, 0, 0.0, 0.0);
            let _ = dp.grading_hue_curve_set_slope(HueCurveType::HueHue, 0, 1.0);
        }
    }

    #[test]
    fn cpu_processor_apply_pixels_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(cpu) = proc.default_cpu_processor() {
            let mut rgba = vec![0.0f32; 16]; // 4 pixels RGBA
            cpu.try_apply_rgba_pixels(&mut rgba, 4, 4).unwrap();
            let mut rgb = vec![0.0f32; 12]; // 4 pixels RGB
            cpu.try_apply_rgb_pixels(&mut rgb, 4, 3).unwrap();
        }
    }

    #[test]
    fn cpu_processor_apply_packed_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(cpu) = proc.default_cpu_processor() {
            let mut rgba = vec![0u8; 32]; // 2 RGBA pixels as packed f32 bytes
            cpu.try_apply_rgba_packed_bit_depth(&mut rgba, BitDepth::F32, 2, 4)
                .unwrap();
            let mut rgb = vec![0u8; 24]; // 2 RGB pixels as packed f32 bytes
            cpu.try_apply_rgb_packed_bit_depth(&mut rgb, BitDepth::F32, 2, 3)
                .unwrap();
        }
    }

    #[test]
    fn cpu_processor_try_apply_reports_invalid_input() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(cpu) = proc.default_cpu_processor() {
            let mut rgba = vec![0.0f32; 3];
            let err = cpu.try_apply_rgba_pixels(&mut rgba, 1, 4).unwrap_err();
            assert!(matches!(err, OcioError::InvalidInput(_)));

            let mut packed = vec![0u8; 4];
            let err = cpu
                .try_apply_rgba_packed_bit_depth(&mut packed, BitDepth::Unknown, 1, 4)
                .unwrap_err();
            assert!(matches!(err, OcioError::InvalidInput(_)));
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
}
