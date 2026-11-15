use std::ffi::c_void;
use std::ptr::NonNull;

use crate::transform::{transform_from_raw_handle, GroupTransform, Transform};
#[cfg(feature = "v2_5")]
use crate::HueCurveType;
use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, BitDepth, ChannelOrdering, DynamicPropertyType,
    FormatMetadata, GpuLanguage, Interpolation, OcioError, ProcessorMetadata, RGBCurveType, Result,
};
use ocio_sys;

fn normalized_stride(api: &str, stride: i64, channels_per_pixel: usize) -> Result<usize> {
    let stride = if stride == 0 {
        channels_per_pixel
    } else {
        usize::try_from(stride)
            .map_err(|_| OcioError::InvalidInput(format!("{api}: stride must be non-negative")))?
    };
    if stride < channels_per_pixel {
        return Err(OcioError::InvalidInput(format!(
            "{api}: stride={stride} is smaller than the {channels_per_pixel} channels per pixel"
        )));
    }
    Ok(stride)
}

fn required_scalar_len(
    api: &str,
    num_pixels: i64,
    stride: i64,
    channels_per_pixel: usize,
) -> Result<usize> {
    let num_pixels = usize::try_from(num_pixels)
        .map_err(|_| OcioError::InvalidInput(format!("{api}: num_pixels must be non-negative")))?;
    let stride = normalized_stride(api, stride, channels_per_pixel)?;
    if num_pixels == 0 {
        return Ok(0);
    }
    num_pixels
        .checked_sub(1)
        .and_then(|pixels_before_last| pixels_before_last.checked_mul(stride))
        .and_then(|last_pixel_offset| last_pixel_offset.checked_add(channels_per_pixel))
        .ok_or_else(|| OcioError::InvalidInput(format!("{api}: required buffer size overflowed")))
}

fn validate_stride_byte_width(
    api: &str,
    stride: i64,
    channels_per_pixel: usize,
    bytes_per_channel: usize,
) -> Result<()> {
    let stride = normalized_stride(api, stride, channels_per_pixel)?;
    let byte_stride = stride
        .checked_mul(bytes_per_channel)
        .ok_or_else(|| OcioError::InvalidInput(format!("{api}: byte stride overflowed")))?;
    if byte_stride > isize::MAX as usize {
        return Err(OcioError::InvalidInput(format!(
            "{api}: byte stride exceeds the C++ ptrdiff_t range"
        )));
    }
    Ok(())
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
    channels_per_pixel: usize,
) -> Result<()> {
    let required_len = required_scalar_len(api, num_pixels, stride, channels_per_pixel)?;
    if num_pixels > 0 {
        validate_stride_byte_width(api, stride, channels_per_pixel, std::mem::size_of::<f32>())?;
    }
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
    channels_per_pixel: usize,
) -> Result<()> {
    let required_scalars = required_scalar_len(api, num_pixels, stride, channels_per_pixel)?;
    let bytes_per_channel = bit_depth_bytes_per_channel(api, bit_depth)?;
    if num_pixels > 0 {
        validate_stride_byte_width(api, stride, channels_per_pixel, bytes_per_channel)?;
    }
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

fn validate_packed_buffer_alignment(
    api: &str,
    buffer: *const u8,
    bit_depth: BitDepth,
    num_pixels: i64,
) -> Result<()> {
    if num_pixels == 0 {
        return Ok(());
    }
    let alignment = bit_depth_bytes_per_channel(api, bit_depth)?;
    if !(buffer as usize).is_multiple_of(alignment) {
        return Err(OcioError::InvalidInput(format!(
            "{api}: buffer must be aligned to {alignment} bytes for {bit_depth:?} data"
        )));
    }
    Ok(())
}

/// An immutable color-processing pipeline produced from a [`Config`](crate::Config).
///
/// A `Processor` wraps the result of asking a config for a conversion between
/// two color spaces, a display/view pipeline, a transform, or a named
/// transform. It is the bridge between configuration and execution.
///
/// Use [`default_cpu_processor`] or [`default_gpu_processor`] to obtain an
/// execution-ready handle. For more control over optimization or bit depth,
/// see [`optimized_cpu_processor`], [`optimized_gpu_processor`], and
/// [`optimized_processor_bitdepth`].
///
/// The processor is immutable: once created, its pipeline does not change.
/// Dynamic properties (if any) are accessed through
/// [`dynamic_property`](Self::dynamic_property) and are scoped to the
/// processor or its derived CPU/GPU execution handles.
///
/// Processor and its execution handles are not `Send` or `Sync`: OCIO dynamic
/// property handles can outlive their parent and are not internally synchronized.
/// Create separate processing pipelines on the threads that use them.
///
/// ```compile_fail
/// # use ocio_rs::Processor;
/// fn require_send<T: Send>() {}
/// require_send::<Processor>();
/// ```
///
/// [`default_cpu_processor`]: Self::default_cpu_processor
/// [`default_gpu_processor`]: Self::default_gpu_processor
/// [`optimized_cpu_processor`]: Self::optimized_cpu_processor
/// [`optimized_gpu_processor`]: Self::optimized_gpu_processor
/// [`optimized_processor_bitdepth`]: Self::optimized_processor_bitdepth
pub struct Processor {
    pub(crate) handle: NonNull<c_void>,
}

impl Processor {
    /// Share the immutable top-level native format metadata without copying.
    pub fn try_const_format_metadata(&self) -> Result<Option<crate::ConstFormatMetadata>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_get_const_format_metadata(self.handle.as_ptr(), false, 0)
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| crate::ConstFormatMetadata { handle }))
    }
    /// Share immutable format metadata for an individual processor transform.
    pub fn try_const_transform_format_metadata(
        &self,
        index: i32,
    ) -> Result<Option<crate::ConstFormatMetadata>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_get_const_format_metadata(self.handle.as_ptr(), true, index)
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| crate::ConstFormatMetadata { handle }))
    }
    /// Build a native CPU processor for explicit input and output bit depths.
    pub fn optimized_cpu_processor_bit_depth(
        &self,
        input: BitDepth,
        output: BitDepth,
        flags: u32,
    ) -> Result<CPUProcessor> {
        if input == BitDepth::Unknown || output == BitDepth::Unknown {
            return Err(OcioError::InvalidInput(
                "CPU processor bit depth cannot be unknown".into(),
            ));
        }
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_get_optimized_cpu_processor_bit_depth(
                self.handle.as_ptr(),
                input as i32,
                output as i32,
                flags,
            )
        };
        Ok(CPUProcessor {
            handle: crate::handle_result(handle)?,
        })
    }
    /// Return whether the processor is an identity/no-op pipeline.
    pub fn is_no_op(&self) -> bool {
        self.try_is_no_op().unwrap_or(false)
    }

    /// Fallible variant of [`Self::is_no_op`].
    pub fn try_is_no_op(&self) -> Result<bool> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_processor_is_no_op(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return whether the processor mixes channels rather than operating lane-wise.
    pub fn has_channel_crosstalk(&self) -> bool {
        self.try_has_channel_crosstalk().unwrap_or(false)
    }

    /// Fallible variant of [`Self::has_channel_crosstalk`].
    pub fn try_has_channel_crosstalk(&self) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_processor_has_channel_crosstalk(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return OCIO's cache identifier for this processor instance.
    pub fn cache_id(&self) -> Option<String> {
        self.try_cache_id().ok().flatten()
    }

    /// Return OCIO's cache identifier, preserving bridge errors.
    pub fn try_cache_id(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_processor_get_cache_id(
                self.handle.as_ptr() as *mut c_void
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
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
    ///
    /// This compatibility helper discards OCIO error details. Prefer
    /// [`Self::try_create_group_transform`] in new code.
    pub fn create_group_transform(&self) -> Option<GroupTransform> {
        self.try_create_group_transform().ok()
    }

    /// Materialize the processor back into an equivalent group transform.
    pub fn try_create_group_transform(&self) -> Result<GroupTransform> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_processor_create_group_transform(self.handle.as_ptr() as *mut c_void)
        };
        let handle = crate::handle_result(handle)?;
        match transform_from_raw_handle(handle.as_ptr()) {
            Some(Transform::Group(transform)) => Ok(transform),
            Some(_) => Err(crate::OcioError::ValidationFailed(
                "OCIO returned a non-group transform for Processor::createGroupTransform".into(),
            )),
            None => Err(crate::OcioError::AllocationFailed),
        }
    }

    // ── v2.5.1 ──
    /// Return an independent editable copy of the processor's top-level metadata.
    pub fn format_metadata(&self) -> Option<FormatMetadata> {
        self.try_format_metadata().ok()
    }

    /// Copy the processor's top-level metadata, preserving native errors.
    pub fn try_format_metadata(&self) -> Result<FormatMetadata> {
        crate::clear_last_error();
        let h = unsafe {
            ocio_sys::ocio_processor_get_format_metadata(self.handle.as_ptr() as *mut c_void)
        };
        crate::handle_result(h).map(|handle| FormatMetadata { handle })
    }

    /// Return an editable metadata copy for the transform at `index`.
    pub fn transform_format_metadata(&self, index: i32) -> Option<FormatMetadata> {
        self.try_transform_format_metadata(index).ok()
    }

    /// Copy format metadata for the transform at `index`.
    pub fn try_transform_format_metadata(&self, index: i32) -> Result<FormatMetadata> {
        crate::clear_last_error();
        let h = unsafe {
            ocio_sys::ocio_processor_get_transform_format_metadata(self.handle.as_ptr(), index)
        };
        crate::handle_result(h).map(|handle| FormatMetadata { handle })
    }

    /// Return an independent editable copy of processor metadata.
    pub fn processor_metadata(&self) -> Option<ProcessorMetadata> {
        self.try_processor_metadata().ok()
    }

    /// Copy technical processor metadata, preserving native errors.
    pub fn try_processor_metadata(&self) -> Result<ProcessorMetadata> {
        crate::clear_last_error();
        let h = unsafe {
            ocio_sys::ocio_processor_get_processor_metadata(self.handle.as_ptr() as *mut c_void)
        };
        crate::handle_result(h).map(|handle| ProcessorMetadata { handle })
    }

    /// Share immutable native processor metadata without copying its entries.
    pub fn try_const_processor_metadata(&self) -> Result<Option<crate::ConstProcessorMetadata>> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_processor_get_const_metadata(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| crate::ConstProcessorMetadata { handle }))
    }

    /// Return whether the processor exposes a dynamic property of `prop_type`.
    pub fn has_dynamic_property_kind(&self, prop_type: DynamicPropertyType) -> bool {
        self.try_has_dynamic_property_kind(prop_type)
            .unwrap_or(false)
    }

    /// Fallible variant of [`Self::has_dynamic_property_kind`].
    pub fn try_has_dynamic_property_kind(&self, prop_type: DynamicPropertyType) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_processor_has_dynamic_property(self.handle.as_ptr(), prop_type as i32)
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer has_dynamic_property_kind with DynamicPropertyType"
    )]
    pub fn has_dynamic_property(&self, prop_type: i32) -> bool {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_processor_has_dynamic_property(self.handle.as_ptr(), prop_type)
        };
        let _ = crate::ocio_call_status();
        value
    }

    /// Return whether this processor contains any runtime-adjustable properties.
    pub fn is_dynamic(&self) -> bool {
        self.try_is_dynamic().unwrap_or(false)
    }

    /// Fallible variant of [`Self::is_dynamic`].
    pub fn try_is_dynamic(&self) -> Result<bool> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_processor_is_dynamic(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()?;
        Ok(value)
    }
}

impl Drop for Processor {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_processor_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

// --- CPUProcessor ---

/// CPU implementation of a [`Processor`].
///
/// Provides safe in-place pixel processing methods for `f32` RGB/RGBA
/// buffers and packed byte buffers at various OCIO bit depths. Each method
/// applies the color transform described by the originating `Processor`.
///
/// # Pixel layout
///
/// The scalar-stride methods ([`apply_rgba_pixels`](Self::apply_rgba_pixels),
/// [`apply_rgb_pixels`](Self::apply_rgb_pixels)) accept `&mut [f32]` buffers
/// and measure stride in `f32` elements, not bytes. Padding elements beyond
/// the pixel stride are left untouched. A stride of zero uses OCIO's native
/// three- or four-channel default; smaller strides are rejected.
///
/// The packed-byte methods ([`apply_rgba_packed_bit_depth`](Self::apply_rgba_packed_bit_depth),
/// [`apply_rgb_packed_bit_depth`](Self::apply_rgb_packed_bit_depth)) interpret
/// the buffer according to the provided [`BitDepth`], with stride measured in
/// channels (not bytes). A stride of zero uses the native channel count, and
/// smaller strides are rejected before calling OCIO. Multi-byte packed data
/// must be aligned to its storage width.
///
/// This handle is thread-confined; see [`Processor`] for the dynamic-property
/// synchronization rationale.
///
/// ```compile_fail
/// # use ocio_rs::CPUProcessor;
/// fn require_send<T: Send>() {}
/// require_send::<CPUProcessor>();
/// ```
///
/// [`BitDepth`]: crate::BitDepth
pub struct CPUProcessor {
    handle: NonNull<c_void>,
}

/// A CPU processor proven to contain no dynamic properties.
///
/// This wrapper can be shared across threads to process independent image
/// buffers. Its one-time validation does not add a lock or check to pixel
/// processing. OCIO native interop must not mutate or replace its underlying
/// processor concurrently.
///
/// ```no_run
/// # use ocio_rs::{CPUProcessor, Result};
/// # fn example(cpu: CPUProcessor) -> Result<()> {
/// let cpu = std::sync::Arc::new(cpu.try_into_static()?);
/// std::thread::scope(|scope| {
///     let cpu = cpu.clone();
///     scope.spawn(move || {
///         let mut rgba = [0.1, 0.2, 0.3, 1.0];
///         cpu.try_apply_rgba(&mut rgba).unwrap();
///     });
/// });
/// # Ok(())
/// # }
/// ```
#[repr(transparent)]
pub struct StaticCPUProcessor(CPUProcessor);

// SAFETY: Construction rejects dynamic processors. Native CPU processors are
// immutable after creation and OCIO supports concurrent processing on them.
// The only independently mutable CPU state OCIO exposes is dynamic properties,
// which cannot be obtained from a non-dynamic processor. The shared pointer
// remains alive until the last Rust owner is dropped. Unsafe external native
// aliases remain the responsibility of the caller creating them.
unsafe impl Send for StaticCPUProcessor {}
// SAFETY: As above, concurrent operations use immutable processor state and the
// safe processing APIs require exclusive access to each output buffer.
unsafe impl Sync for StaticCPUProcessor {}

impl StaticCPUProcessor {
    /// Returns the thread-confined CPU handle when the wrapper is owned.
    pub fn into_inner(self) -> CPUProcessor {
        self.0
    }
}

impl std::ops::Deref for StaticCPUProcessor {
    type Target = CPUProcessor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Positive F32 element strides for a planar image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanarF32Layout {
    /// Number of pixels in each row.
    pub width: usize,
    /// Number of rows.
    pub height: usize,
    /// Distance between adjacent pixels in F32 elements.
    pub x_stride: usize,
    /// Distance between adjacent rows in F32 elements.
    pub y_stride: usize,
}

/// Positive byte strides and native channel bit depth for a planar image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanarByteLayout {
    /// Number of pixels per row.
    pub width: usize,
    /// Number of rows.
    pub height: usize,
    /// Byte distance between adjacent pixels.
    pub x_stride: usize,
    /// Byte distance between adjacent rows; at least `width * x_stride`,
    /// including when the image has only one row.
    pub y_stride: usize,
    /// Native storage and interpretation of each channel.
    pub bit_depth: BitDepth,
}

/// Packed source and destination layout for native-depth conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedConversionLayout {
    /// Number of RGB or RGBA pixels.
    pub pixels: i64,
    /// Three for RGB, four for RGBA.
    pub channels: u8,
    /// Input sample bit depth.
    pub input_depth: BitDepth,
    /// Output sample bit depth.
    pub output_depth: BitDepth,
    /// Distance between input pixels in samples.
    pub input_stride: i64,
    /// Distance between output pixels in samples.
    pub output_stride: i64,
}

/// Packed image layout with signed pixel and row strides, in bytes.
///
/// `origin` points to the first stored channel of the first logical pixel.
/// Channel stride is positive. Slices must include pointer-step padding after
/// the final pixel, including before the lowest pixel for a negative x stride.
/// Strides and origin must be aligned to the sample size. The pixel stride's
/// magnitude must cover all stored channels, and the row stride's magnitude
/// must be at least `width * abs(x_stride)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedImageLayout {
    /// Pixels per row.
    pub width: usize,
    /// Number of rows.
    pub height: usize,
    /// Byte offset to the first stored channel of the first logical pixel.
    pub origin: usize,
    /// Positive byte distance between stored channels of a pixel.
    pub channel_stride: usize,
    /// Signed byte distance between adjacent pixels.
    pub x_stride: isize,
    /// Signed byte distance between adjacent rows.
    pub y_stride: isize,
    /// Native channel storage type, matching the processor bit depth.
    pub bit_depth: BitDepth,
    /// Ordering of the stored color and alpha channels.
    pub channel_order: ChannelOrdering,
}

fn packed_image_descriptor(
    layout: PackedImageLayout,
    data: &[u8],
) -> Result<ocio_sys::OcioPackedBuffer> {
    let invalid =
        || OcioError::InvalidInput("invalid, unaligned or overflowing packed image layout".into());
    let size = bit_depth_bytes_per_channel("CPUProcessor packed image", layout.bit_depth)?;
    let channels = match layout.channel_order {
        ChannelOrdering::Rgb | ChannelOrdering::Bgr => 3,
        _ => 4,
    };
    let pixel_size = (channels - 1usize)
        .checked_mul(layout.channel_stride)
        .and_then(|v| v.checked_add(size))
        .ok_or_else(invalid)?;
    let x = layout.x_stride.checked_abs().ok_or_else(invalid)? as usize;
    let y = layout.y_stride.checked_abs().ok_or_else(invalid)? as usize;
    if layout.width == 0
        || layout.height == 0
        || layout.channel_stride < size
        || x < pixel_size
        || y == 0
        || x > y / layout.width
        || !layout.channel_stride.is_multiple_of(size)
        || !x.is_multiple_of(size)
        || !y.is_multiple_of(size)
        || !layout.origin.is_multiple_of(size)
        || !(data.as_ptr() as usize).is_multiple_of(size)
        || layout.channel_stride > isize::MAX as usize
    {
        return Err(invalid());
    }
    let dx = (layout.width - 1).checked_mul(x).ok_or_else(invalid)?;
    let dy = (layout.height - 1).checked_mul(y).ok_or_else(invalid)?;
    let backward = (if layout.x_stride < 0 { dx } else { 0 })
        .checked_add(if layout.y_stride < 0 { dy } else { 0 })
        .ok_or_else(invalid)?;
    let forward = (if layout.x_stride > 0 { dx } else { 0 })
        .checked_add(if layout.y_stride > 0 { dy } else { 0 })
        .ok_or_else(invalid)?;
    let low = layout.origin.checked_sub(backward).ok_or_else(invalid)?;
    let high = layout.origin.checked_add(forward).ok_or_else(invalid)?;
    if (layout.x_stride < 0 && low < x)
        || high
            .checked_add(if layout.x_stride > 0 { x } else { pixel_size })
            .ok_or_else(invalid)?
            > data.len()
    {
        return Err(invalid());
    }
    let width = i64::try_from(layout.width).map_err(|_| invalid())?;
    let height = i64::try_from(layout.height).map_err(|_| invalid())?;
    Ok(ocio_sys::OcioPackedBuffer {
        // The native API treats the source descriptor as read-only during conversion.
        data: unsafe { data.as_ptr().add(layout.origin) }
            .cast_mut()
            .cast(),
        width,
        height,
        channel_stride: layout.channel_stride as i64,
        x_stride: layout.x_stride as i64,
        y_stride: layout.y_stride as i64,
        bit_depth: layout.bit_depth as i32,
        channel_order: layout.channel_order as i32,
    })
}

/// Borrowed source channels for a planar conversion.
pub struct PlanarBytes<'a> {
    pub red: &'a [u8],
    pub green: &'a [u8],
    pub blue: &'a [u8],
    pub alpha: Option<&'a [u8]>,
    pub layout: PlanarByteLayout,
}

/// Borrowed destination channels for a planar conversion.
pub struct PlanarBytesMut<'a> {
    pub red: &'a mut [u8],
    pub green: &'a mut [u8],
    pub blue: &'a mut [u8],
    pub alpha: Option<&'a mut [u8]>,
    pub layout: PlanarByteLayout,
}

/// A planar image layout with signed byte strides and an origin in each plane.
///
/// The origin is the byte offset of the first pixel to process in every slice.
/// Strides and origin must be aligned to the sample size. The row stride's
/// magnitude must be at least `width * abs(x_stride)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignedPlanarByteLayout {
    /// Number of pixels per row.
    pub width: usize,
    /// Number of rows.
    pub height: usize,
    /// Byte offset of the first logical pixel in each plane.
    pub origin: usize,
    /// Signed byte distance between horizontally adjacent pixels.
    pub x_stride: isize,
    /// Signed byte distance between adjacent rows.
    pub y_stride: isize,
    /// Native storage and interpretation of each channel.
    pub bit_depth: BitDepth,
}

/// Immutable planar input with signed byte strides.
pub struct SignedPlanarBytes<'a> {
    pub red: &'a [u8],
    pub green: &'a [u8],
    pub blue: &'a [u8],
    pub alpha: Option<&'a [u8]>,
    pub layout: SignedPlanarByteLayout,
}

/// Exclusively borrowed planar image with signed byte strides.
pub struct SignedPlanarBytesMut<'a> {
    pub red: &'a mut [u8],
    pub green: &'a mut [u8],
    pub blue: &'a mut [u8],
    pub alpha: Option<&'a mut [u8]>,
    pub layout: SignedPlanarByteLayout,
}

fn validate_signed_planar(layout: SignedPlanarByteLayout, planes: &[&[u8]]) -> Result<[i64; 4]> {
    let SignedPlanarByteLayout {
        width,
        height,
        origin,
        x_stride,
        y_stride,
        bit_depth,
    } = layout;
    let size = bit_depth_bytes_per_channel("CPUProcessor planar image", bit_depth)?;
    let invalid =
        || OcioError::InvalidInput("invalid, unaligned or overflowing signed planar layout".into());
    let x = x_stride.checked_abs().ok_or_else(invalid)? as usize;
    let y = y_stride.checked_abs().ok_or_else(invalid)? as usize;
    if width == 0
        || height == 0
        || x < size
        || y == 0
        || x > y / width
        || !x.is_multiple_of(size)
        || !y.is_multiple_of(size)
        || !origin.is_multiple_of(size)
    {
        return Err(invalid());
    }
    let dx = (width - 1).checked_mul(x).ok_or_else(invalid)?;
    let dy = (height - 1).checked_mul(y).ok_or_else(invalid)?;
    let backward = (if x_stride < 0 { dx } else { 0 })
        .checked_add(if y_stride < 0 { dy } else { 0 })
        .ok_or_else(invalid)?;
    let forward = (if x_stride > 0 { dx } else { 0 })
        .checked_add(if y_stride > 0 { dy } else { 0 })
        .ok_or_else(invalid)?;
    let low = origin.checked_sub(backward).ok_or_else(invalid)?;
    let high = origin.checked_add(forward).ok_or_else(invalid)?;
    // OCIO advances each pixel pointer once after processing the last sample.
    // Keep that pointer within, or one past, the borrowed allocation too.
    if x_stride < 0 && low < x {
        return Err(invalid());
    }
    let required = high
        .checked_add(if x_stride > 0 { x.max(size) } else { size })
        .ok_or_else(invalid)?;
    if planes
        .iter()
        .any(|p| p.len() < required || !(p.as_ptr() as usize).is_multiple_of(size))
    {
        return Err(invalid());
    }
    let [Ok(w), Ok(h)] = [width, height].map(i64::try_from) else {
        return Err(invalid());
    };
    Ok([w, h, x_stride as i64, y_stride as i64])
}

fn validate_planar_buffer(layout: PlanarByteLayout, planes: &[&[u8]]) -> Result<[i64; 4]> {
    let PlanarByteLayout {
        width,
        height,
        x_stride,
        y_stride,
        bit_depth,
    } = layout;
    let size = bit_depth_bytes_per_channel("CPUProcessor::try_apply_planar_convert", bit_depth)?;
    let invalid =
        || OcioError::InvalidInput("invalid, unaligned or overflowing planar layout".into());
    if width == 0
        || height == 0
        || x_stride < size
        || y_stride == 0
        || !x_stride.is_multiple_of(size)
        || !y_stride.is_multiple_of(size)
    {
        return Err(invalid());
    }
    let row_span = (width - 1)
        .checked_mul(x_stride)
        .and_then(|n| n.checked_add(size))
        .ok_or_else(invalid)?;
    if x_stride > y_stride / width {
        return Err(invalid());
    }
    let required = (height - 1)
        .checked_mul(y_stride)
        .and_then(|n| n.checked_add(row_span))
        .ok_or_else(invalid)?;
    if planes
        .iter()
        .any(|p| p.len() < required || !(p.as_ptr() as usize).is_multiple_of(size))
    {
        return Err(invalid());
    }
    let [Ok(w), Ok(h), Ok(x), Ok(y)] = [width, height, x_stride, y_stride].map(i64::try_from)
    else {
        return Err(invalid());
    };
    Ok([w, h, x, y])
}

fn signed_planar_source(image: &SignedPlanarBytes<'_>) -> Result<ocio_sys::OcioPlanarBuffer> {
    let [width, height, x_stride, y_stride] =
        validate_signed_planar(image.layout, &[image.red, image.green, image.blue])?;
    if let Some(alpha) = image.alpha {
        validate_signed_planar(image.layout, &[alpha])?;
    }
    let origin = image.layout.origin;
    Ok(ocio_sys::OcioPlanarBuffer {
        red: image.red.as_ptr().wrapping_add(origin).cast_mut().cast(),
        green: image.green.as_ptr().wrapping_add(origin).cast_mut().cast(),
        blue: image.blue.as_ptr().wrapping_add(origin).cast_mut().cast(),
        alpha: image.alpha.map_or(std::ptr::null_mut(), |p| {
            p.as_ptr().wrapping_add(origin).cast_mut().cast()
        }),
        width,
        height,
        x_stride,
        y_stride,
        bit_depth: image.layout.bit_depth as i32,
    })
}

fn signed_planar_destination(
    image: &mut SignedPlanarBytesMut<'_>,
) -> Result<ocio_sys::OcioPlanarBuffer> {
    let [width, height, x_stride, y_stride] =
        validate_signed_planar(image.layout, &[image.red, image.green, image.blue])?;
    if let Some(alpha) = image.alpha.as_deref() {
        validate_signed_planar(image.layout, &[alpha])?;
    }
    let origin = image.layout.origin;
    Ok(ocio_sys::OcioPlanarBuffer {
        red: image.red.as_mut_ptr().wrapping_add(origin).cast(),
        green: image.green.as_mut_ptr().wrapping_add(origin).cast(),
        blue: image.blue.as_mut_ptr().wrapping_add(origin).cast(),
        alpha: image
            .alpha
            .as_deref_mut()
            .map_or(std::ptr::null_mut(), |p| {
                p.as_mut_ptr().wrapping_add(origin).cast()
            }),
        width,
        height,
        x_stride,
        y_stride,
        bit_depth: image.layout.bit_depth as i32,
    })
}

impl CPUProcessor {
    /// Converts a non-dynamic processor into a thread-shareable processor.
    ///
    /// Returns an error for a dynamic processor or when the native query fails.
    /// This consumes the original handle even on error; query
    /// [`Self::try_is_dynamic`] first if it must be retained in that case.
    pub fn try_into_static(self) -> Result<StaticCPUProcessor> {
        if self.try_is_dynamic()? {
            return Err(OcioError::InvalidInput(
                "a dynamic CPU processor cannot be shared without synchronization".into(),
            ));
        }
        Ok(StaticCPUProcessor(self))
    }

    /// Process an exclusively borrowed packed image in place without pixel copies.
    pub fn try_apply_packed_image(&self, data: &mut [u8], layout: PackedImageLayout) -> Result<()> {
        let mut desc = packed_image_descriptor(layout, data)?;
        desc.data = unsafe { data.as_mut_ptr().add(layout.origin).cast() };
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_cpu_processor_apply_packed_image(self.handle.as_ptr(), &desc) };
        crate::ocio_call_status()
    }

    /// Convert independently laid-out packed images without pixel copies.
    /// Dimensions must match; bit depths and channel ordering may differ.
    pub fn try_convert_packed_image(
        &self,
        source: &[u8],
        source_layout: PackedImageLayout,
        destination: &mut [u8],
        destination_layout: PackedImageLayout,
    ) -> Result<()> {
        if source_layout.width != destination_layout.width
            || source_layout.height != destination_layout.height
        {
            return Err(OcioError::InvalidInput(
                "packed image dimensions differ".into(),
            ));
        }
        let src = packed_image_descriptor(source_layout, source)?;
        let mut dst = packed_image_descriptor(destination_layout, destination)?;
        dst.data = unsafe {
            destination
                .as_mut_ptr()
                .add(destination_layout.origin)
                .cast()
        };
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_convert_packed_image(self.handle.as_ptr(), &src, &dst)
        };
        crate::ocio_call_status()
    }

    /// Convert a packed source into separate signed-stride destination planes.
    ///
    /// The images must have matching dimensions and processor-compatible bit
    /// depths. Alpha may be present independently on either side. OCIO fills
    /// missing input alpha with zero in 2.4/2.5 and one in 2.6; it discards
    /// alpha if the output lacks it.
    /// Buffers are borrowed without copying; layout padding rules apply.
    pub fn try_convert_packed_to_planar(
        &self,
        source: &[u8],
        source_layout: PackedImageLayout,
        mut destination: SignedPlanarBytesMut<'_>,
    ) -> Result<()> {
        if source_layout.width != destination.layout.width
            || source_layout.height != destination.layout.height
        {
            return Err(OcioError::InvalidInput("image dimensions differ".into()));
        }
        let packed = packed_image_descriptor(source_layout, source)?;
        let planar = signed_planar_destination(&mut destination)?;
        crate::clear_last_error();
        // SAFETY: Both descriptors are validated and destination planes are
        // exclusively borrowed for the duration of the synchronous call.
        unsafe {
            ocio_sys::ocio_cpu_processor_convert_mixed_image(
                self.handle.as_ptr(),
                &packed,
                &planar,
                true,
            )
        };
        crate::ocio_call_status()
    }

    /// Convert signed-stride source planes into a packed destination.
    ///
    /// See [`Self::try_convert_packed_to_planar`] for dimensions, alpha,
    /// bit-depth and borrowed-buffer requirements.
    pub fn try_convert_planar_to_packed(
        &self,
        source: SignedPlanarBytes<'_>,
        destination: &mut [u8],
        destination_layout: PackedImageLayout,
    ) -> Result<()> {
        if source.layout.width != destination_layout.width
            || source.layout.height != destination_layout.height
        {
            return Err(OcioError::InvalidInput("image dimensions differ".into()));
        }
        let planar = signed_planar_source(&source)?;
        let mut packed = packed_image_descriptor(destination_layout, destination)?;
        packed.data = unsafe {
            destination
                .as_mut_ptr()
                .add(destination_layout.origin)
                .cast()
        };
        crate::clear_last_error();
        // SAFETY: Both descriptors are validated, and destination is exclusive.
        unsafe {
            ocio_sys::ocio_cpu_processor_convert_mixed_image(
                self.handle.as_ptr(),
                &packed,
                &planar,
                false,
            )
        };
        crate::ocio_call_status()
    }
    /// Process a planar image in place with signed byte strides without copying.
    ///
    /// OCIO advances pointers after the last pixel of each row. Therefore the
    /// slices must also contain that final pointer position (one-past is valid).
    /// In particular, a negative pixel stride requires preceding padding.
    pub fn try_apply_planar_signed(&self, mut image: SignedPlanarBytesMut<'_>) -> Result<()> {
        let [width, height, x_stride, y_stride] =
            validate_signed_planar(image.layout, &[image.red, image.green, image.blue])?;
        if let Some(alpha) = image.alpha.as_deref() {
            validate_signed_planar(image.layout, &[alpha])?;
        }
        let origin = image.layout.origin;
        crate::clear_last_error();
        // SAFETY: The exclusive slices have validated bounds and alignment,
        // including the original pointer position and the native strides.
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_planar_bytes(
                self.handle.as_ptr(),
                image.red.as_mut_ptr().add(origin).cast(),
                image.green.as_mut_ptr().add(origin).cast(),
                image.blue.as_mut_ptr().add(origin).cast(),
                image
                    .alpha
                    .as_deref_mut()
                    .map_or(std::ptr::null_mut(), |p| p.as_mut_ptr().add(origin).cast()),
                width,
                height,
                x_stride,
                y_stride,
                image.layout.bit_depth as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// Convert separate signed-stride planar images without copying pixel data.
    ///
    /// Source and destination must have equal dimensions. Alpha may be present
    /// independently on either side. Missing input alpha follows the native
    /// version's behavior, described by [`Self::try_convert_packed_to_planar`].
    /// Their layouts and bit depths may differ. As with
    /// [`Self::try_apply_planar_signed`], pointer-step padding is required.
    pub fn try_apply_planar_signed_convert(
        &self,
        source: SignedPlanarBytes<'_>,
        mut destination: SignedPlanarBytesMut<'_>,
    ) -> Result<()> {
        if source.layout.width != destination.layout.width
            || source.layout.height != destination.layout.height
        {
            return Err(OcioError::InvalidInput(
                "planar dimensions must match".into(),
            ));
        }
        let input = signed_planar_source(&source)?;
        let output = signed_planar_destination(&mut destination)?;
        crate::clear_last_error();
        // SAFETY: Bounds and alignment have been checked, the source is only
        // read, and Rust exclusively borrows all destination planes.
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_planar_convert(self.handle.as_ptr(), &input, &output)
        };
        crate::ocio_call_status()
    }

    /// Convert separate planar buffers using the processor's input and output depths.
    ///
    /// Layouts may have different strides and bit depths, but must have identical
    /// dimensions. Alpha may be present independently on either side; see
    /// [`Self::try_convert_packed_to_planar`] for missing-alpha behavior. Buffers are borrowed
    /// directly without copying; strides are in bytes and must be positive and aligned.
    pub fn try_apply_planar_convert(
        &self,
        source: PlanarBytes<'_>,
        mut destination: PlanarBytesMut<'_>,
    ) -> Result<()> {
        if source.layout.width != destination.layout.width
            || source.layout.height != destination.layout.height
        {
            return Err(OcioError::InvalidInput(
                "planar dimensions must match".into(),
            ));
        }
        let [iw, ih, ix, iy] =
            validate_planar_buffer(source.layout, &[source.red, source.green, source.blue])?;
        let [ow, oh, ox, oy] = validate_planar_buffer(
            destination.layout,
            &[destination.red, destination.green, destination.blue],
        )?;
        if let Some(input) = source.alpha {
            validate_planar_buffer(source.layout, &[input])?;
        }
        if let Some(output) = destination.alpha.as_deref() {
            validate_planar_buffer(destination.layout, &[output])?;
        }
        let input = ocio_sys::OcioPlanarBuffer {
            red: source.red.as_ptr().cast_mut().cast(),
            green: source.green.as_ptr().cast_mut().cast(),
            blue: source.blue.as_ptr().cast_mut().cast(),
            alpha: source
                .alpha
                .map_or(std::ptr::null_mut(), |p| p.as_ptr().cast_mut().cast()),
            width: iw,
            height: ih,
            x_stride: ix,
            y_stride: iy,
            bit_depth: source.layout.bit_depth as i32,
        };
        let output = ocio_sys::OcioPlanarBuffer {
            red: destination.red.as_mut_ptr().cast(),
            green: destination.green.as_mut_ptr().cast(),
            blue: destination.blue.as_mut_ptr().cast(),
            alpha: destination
                .alpha
                .as_deref_mut()
                .map_or(std::ptr::null_mut(), |p| p.as_mut_ptr().cast()),
            width: ow,
            height: oh,
            x_stride: ox,
            y_stride: oy,
            bit_depth: destination.layout.bit_depth as i32,
        };
        crate::clear_last_error();
        // SAFETY: Both descriptors have validated extents and alignment. The source
        // is read-only during apply, and the destination planes are exclusively borrowed.
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_planar_convert(self.handle.as_ptr(), &input, &output)
        };
        crate::ocio_call_status()
    }
    /// Convert separate packed buffers with the processor's input/output depths.
    ///
    /// RGB and RGBA are supported. Strides are measured in samples. Input and
    /// output slices are checked independently and must meet native alignment.
    pub fn try_apply_packed_convert(
        &self,
        input: &[u8],
        output: &mut [u8],
        layout: PackedConversionLayout,
    ) -> Result<()> {
        let PackedConversionLayout {
            pixels,
            channels,
            input_depth,
            output_depth,
            input_stride,
            output_stride,
        } = layout;
        if pixels <= 0
            || !matches!(channels, 3 | 4)
            || input_stride < i64::from(channels)
            || output_stride < i64::from(channels)
        {
            return Err(OcioError::InvalidInput(
                "invalid packed conversion layout".into(),
            ));
        }
        let api = "CPUProcessor::try_apply_packed_convert";
        validate_packed_buffer_len(
            api,
            input.len(),
            input_depth,
            pixels,
            input_stride,
            channels as usize,
        )?;
        validate_packed_buffer_len(
            api,
            output.len(),
            output_depth,
            pixels,
            output_stride,
            channels as usize,
        )?;
        validate_packed_buffer_alignment(api, input.as_ptr(), input_depth, pixels)?;
        validate_packed_buffer_alignment(api, output.as_ptr(), output_depth, pixels)?;
        let in_size = bit_depth_bytes_per_channel(api, input_depth)? as i64;
        let out_size = bit_depth_bytes_per_channel(api, output_depth)? as i64;
        let in_bytes = input_stride
            .checked_mul(in_size)
            .ok_or_else(|| OcioError::InvalidInput("input stride overflow".into()))?;
        let out_bytes = output_stride
            .checked_mul(out_size)
            .ok_or_else(|| OcioError::InvalidInput("output stride overflow".into()))?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_packed_convert(
                self.handle.as_ptr(),
                input.as_ptr().cast(),
                output.as_mut_ptr().cast(),
                pixels,
                channels.into(),
                input_depth as i32,
                output_depth as i32,
                in_bytes,
                out_bytes,
            )
        };
        crate::ocio_call_status()
    }
    /// Process planar byte buffers in place at the processor's native bit depth.
    ///
    /// All pointers and strides must be aligned for the selected storage type.
    /// The binding validates the layout and passes the original buffers to OCIO.
    pub fn try_apply_planar_bytes(
        &self,
        red: &mut [u8],
        green: &mut [u8],
        blue: &mut [u8],
        alpha: Option<&mut [u8]>,
        layout: PlanarByteLayout,
    ) -> Result<()> {
        let PlanarByteLayout {
            width,
            height,
            x_stride,
            y_stride,
            bit_depth,
        } = layout;
        let size = bit_depth_bytes_per_channel("CPUProcessor::try_apply_planar_bytes", bit_depth)?;
        let invalid =
            || OcioError::InvalidInput("invalid, unaligned or overflowing planar layout".into());
        if width == 0
            || height == 0
            || x_stride < size
            || !x_stride.is_multiple_of(size)
            || !y_stride.is_multiple_of(size)
        {
            return Err(invalid());
        }
        let row_span = (width - 1)
            .checked_mul(x_stride)
            .and_then(|n| n.checked_add(size))
            .ok_or_else(invalid)?;
        if x_stride > y_stride / width {
            return Err(invalid());
        }
        let required = (height - 1)
            .checked_mul(y_stride)
            .and_then(|n| n.checked_add(row_span))
            .ok_or_else(invalid)?;
        for plane in [&*red, &*green, &*blue].into_iter().chain(alpha.as_deref()) {
            if plane.len() < required || !(plane.as_ptr() as usize).is_multiple_of(size) {
                return Err(invalid());
            }
        }
        let [Ok(width), Ok(height), Ok(x_stride), Ok(y_stride)] =
            [width, height, x_stride, y_stride].map(i64::try_from)
        else {
            return Err(invalid());
        };
        if y_stride == 0 {
            return Err(invalid());
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_planar_bytes(
                self.handle.as_ptr(),
                red.as_mut_ptr().cast(),
                green.as_mut_ptr().cast(),
                blue.as_mut_ptr().cast(),
                alpha.map_or(std::ptr::null_mut(), |p| p.as_mut_ptr().cast()),
                width,
                height,
                x_stride,
                y_stride,
                bit_depth as i32,
            )
        };
        crate::ocio_call_status()
    }
    /// Process strided planar F32 channels in place without copying pixels.
    ///
    /// Strides are measured in F32 elements and must be positive. As required
    /// by OCIO, the row stride must be at least the pixel stride times the
    /// width, even for a single row. Alpha may be omitted.
    pub fn try_apply_planar_strided_f32(
        &self,
        red: &mut [f32],
        green: &mut [f32],
        blue: &mut [f32],
        alpha: Option<&mut [f32]>,
        layout: PlanarF32Layout,
    ) -> Result<()> {
        let PlanarF32Layout {
            width,
            height,
            x_stride,
            y_stride,
        } = layout;
        let invalid = || OcioError::InvalidInput("invalid or overflowing planar layout".into());
        if width == 0 || height == 0 || x_stride == 0 || y_stride == 0 {
            return Err(invalid());
        }
        let row_span = (width - 1)
            .checked_mul(x_stride)
            .and_then(|v| v.checked_add(1))
            .ok_or_else(invalid)?;
        if x_stride > y_stride / width {
            return Err(invalid());
        }
        let required = (height - 1)
            .checked_mul(y_stride)
            .and_then(|v| v.checked_add(row_span))
            .ok_or_else(invalid)?;
        if [red.len(), green.len(), blue.len()]
            .iter()
            .any(|&n| n < required)
            || alpha.as_ref().is_some_and(|plane| plane.len() < required)
        {
            return Err(invalid());
        }
        let values = [width, height, x_stride, y_stride].map(i64::try_from);
        let [Ok(width), Ok(height), Ok(x_stride), Ok(y_stride)] = values else {
            return Err(invalid());
        };
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_planar_strided_f32(
                self.handle.as_ptr(),
                red.as_mut_ptr(),
                green.as_mut_ptr(),
                blue.as_mut_ptr(),
                alpha.map_or(std::ptr::null_mut(), |plane| plane.as_mut_ptr()),
                width,
                height,
                x_stride,
                y_stride,
            )
        };
        crate::ocio_call_status()
    }
    /// Process contiguous row-major planar float channels in place.
    ///
    /// Each channel must contain exactly `width * height` samples. Alpha is
    /// optional. The native descriptor borrows these slices for this call and
    /// no pixel buffer is allocated or copied by the binding.
    pub fn try_apply_planar_f32(
        &self,
        red: &mut [f32],
        green: &mut [f32],
        blue: &mut [f32],
        alpha: Option<&mut [f32]>,
        width: usize,
        height: usize,
    ) -> Result<()> {
        let count = width
            .checked_mul(height)
            .ok_or_else(|| OcioError::InvalidInput("planar dimensions overflow".into()))?;
        if count == 0
            || [red.len(), green.len(), blue.len()]
                .iter()
                .any(|&len| len != count)
            || alpha.as_ref().is_some_and(|plane| plane.len() != count)
        {
            return Err(OcioError::InvalidInput(
                "planar channel lengths must match nonzero dimensions".into(),
            ));
        }
        let width = i64::try_from(width)
            .map_err(|_| OcioError::InvalidInput("planar width exceeds i64".into()))?;
        let height = i64::try_from(height)
            .map_err(|_| OcioError::InvalidInput("planar height exceeds i64".into()))?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_planar_f32(
                self.handle.as_ptr(),
                red.as_mut_ptr(),
                green.as_mut_ptr(),
                blue.as_mut_ptr(),
                alpha.map_or(std::ptr::null_mut(), |plane| plane.as_mut_ptr()),
                width,
                height,
            )
        };
        crate::ocio_call_status()
    }
    /// # Safety
    /// `img_desc` must point to a valid OCIO image descriptor for the active
    /// ABI. The pixel buffers referenced by the descriptor must remain
    /// allocated, writable, and correctly strided for the full call. The
    /// descriptor is borrowed only; this method does not take ownership.
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO image-descriptor entry point; prefer apply_rgb/apply_rgba/apply_*_pixels for Rust callers"
    )]
    pub unsafe fn apply(&self, img_desc: *mut c_void) {
        let _ = unsafe { self.try_apply_raw(img_desc) };
    }

    /// Apply a raw OCIO image descriptor and preserve an OCIO failure.
    ///
    /// # Safety
    /// `img_desc` must point to a valid OCIO image descriptor for the active
    /// ABI. The pixel buffers referenced by the descriptor must remain
    /// allocated, writable, and correctly strided for the full call. The
    /// descriptor is borrowed only; this method does not take ownership.
    #[doc(hidden)]
    pub unsafe fn try_apply_raw(&self, img_desc: *mut c_void) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply(self.handle.as_ptr(), img_desc);
        }
        crate::ocio_call_status()
    }

    /// # Safety
    /// Same invariants as [`apply`](CPUProcessor::apply). This is a
    /// compatibility alias for older ABI versions.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer apply() or the typed pixel helpers"
    )]
    pub unsafe fn apply_v1(&self, img_desc: *mut c_void) {
        let _ = unsafe { self.try_apply_raw_v1(img_desc) };
    }

    /// Fallible raw ABI compatibility entry point for single-descriptor apply.
    ///
    /// # Safety
    /// Same invariants as [`Self::try_apply_raw`].
    #[doc(hidden)]
    pub unsafe fn try_apply_raw_v1(&self, img_desc: *mut c_void) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_v1(self.handle.as_ptr(), img_desc);
        }
        crate::ocio_call_status()
    }

    /// # Safety
    /// `src_img_desc` and `dst_img_desc` must point to valid OCIO image
    /// descriptors for the active ABI. Their referenced buffers must remain
    /// allocated for the call; the destination buffer must be writable. Unless
    /// the upstream descriptor contract explicitly permits it, source and
    /// destination memory must not overlap. Both descriptors are borrowed only.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO image-descriptor entry point; prefer apply_rgb/apply_rgba/apply_*_pixels for Rust callers"
    )]
    pub unsafe fn apply_v2(&self, src_img_desc: *mut c_void, dst_img_desc: *mut c_void) {
        let _ = unsafe { self.try_apply_raw_v2(src_img_desc, dst_img_desc) };
    }

    /// Fallible raw ABI compatibility entry point for separate source and destination descriptors.
    ///
    /// # Safety
    /// Same invariants as [`Self::apply_v2`].
    #[doc(hidden)]
    pub unsafe fn try_apply_raw_v2(
        &self,
        src_img_desc: *mut c_void,
        dst_img_desc: *mut c_void,
    ) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_cpu_processor_apply_v2(self.handle.as_ptr(), src_img_desc, dst_img_desc);
        }
        crate::ocio_call_status()
    }

    /// Apply the processor in place to one RGBA pixel.
    pub fn apply_rgba(&self, rgba: &mut [f32; 4]) {
        self.try_apply_rgba(rgba)
            .expect("CPUProcessor::apply_rgba failed");
    }

    /// Apply the processor in place to one RGBA pixel.
    pub fn try_apply_rgba(&self, rgba: &mut [f32; 4]) -> Result<()> {
        let ok = unsafe {
            ocio_sys::ocio_cpu_processor_try_apply_rgba(
                self.handle.as_ptr(),
                rgba.as_mut_ptr() as *mut c_void,
            )
        };
        if ok {
            Ok(())
        } else {
            crate::ocio_call_status()?;
            Err(OcioError::Ocio(
                "OpenColorIO pixel processing failed without an error message".into(),
            ))
        }
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
            4,
        )?;
        let ok = unsafe {
            ocio_sys::ocio_cpu_processor_try_apply_rgba_pixels(
                self.handle.as_ptr(),
                rgba.as_mut_ptr(),
                num_pixels,
                stride,
            )
        };
        if ok {
            Ok(())
        } else {
            crate::ocio_call_status()?;
            Err(OcioError::Ocio(
                "OpenColorIO pixel processing failed without an error message".into(),
            ))
        }
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
            3,
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
    ///
    /// `bit_depth` must match the CPU processor's finalized bit-depth contract.
    /// In particular, a default CPU processor commonly expects `BitDepth::F32`.
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
    ///
    /// `bit_depth` must match the CPU processor's finalized bit-depth contract.
    /// In particular, a default CPU processor commonly expects `BitDepth::F32`.
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
            4,
        )?;
        validate_packed_buffer_alignment(
            "CPUProcessor::apply_rgba_packed_bit_depth",
            rgba.as_ptr(),
            bit_depth,
            num_pixels,
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
    ///
    /// `bit_depth` must match the CPU processor's finalized bit-depth contract.
    /// In particular, a default CPU processor commonly expects `BitDepth::F32`.
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
    ///
    /// `bit_depth` must match the CPU processor's finalized bit-depth contract.
    /// In particular, a default CPU processor commonly expects `BitDepth::F32`.
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
            3,
        )?;
        validate_packed_buffer_alignment(
            "CPUProcessor::apply_rgb_packed_bit_depth",
            rgb.as_ptr(),
            bit_depth,
            num_pixels,
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
    ///
    /// Returns `false` if OCIO reports an error. Use [`Self::try_is_no_op`] to
    /// distinguish that case from a non-identity processor.
    pub fn is_no_op(&self) -> bool {
        self.try_is_no_op().unwrap_or(false)
    }

    /// Fallible variant of [`Self::is_no_op`].
    ///
    /// Returns an error when the underlying OCIO query throws.
    pub fn try_is_no_op(&self) -> Result<bool> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_cpu_processor_is_no_op(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return whether the CPU path mixes color channels.
    ///
    /// Returns `false` if OCIO reports an error. Use
    /// [`Self::try_has_channel_crosstalk`] to distinguish that case from a
    /// lane-wise processor.
    pub fn has_channel_crosstalk(&self) -> bool {
        self.try_has_channel_crosstalk().unwrap_or(false)
    }

    /// Fallible variant of [`Self::has_channel_crosstalk`].
    ///
    /// Returns an error when the underlying OCIO query throws.
    pub fn try_has_channel_crosstalk(&self) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_cpu_processor_has_channel_crosstalk(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return OCIO's cache identifier for this CPU processor instance.
    pub fn cache_id(&self) -> Option<String> {
        self.try_cache_id().ok().flatten()
    }

    /// Return OCIO's cache identifier, preserving bridge errors.
    pub fn try_cache_id(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_cpu_processor_get_cache_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return the declared input bit depth for this CPU path.
    pub fn input_bit_depth(&self) -> i32 {
        self.try_input_bit_depth().unwrap_or(0)
    }

    /// Return the declared input bit depth, preserving bridge errors.
    pub fn try_input_bit_depth(&self) -> Result<i32> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_cpu_processor_get_input_bit_depth(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return the declared output bit depth for this CPU path.
    pub fn output_bit_depth(&self) -> i32 {
        self.try_output_bit_depth().unwrap_or(0)
    }

    /// Return the declared output bit depth, preserving bridge errors.
    pub fn try_output_bit_depth(&self) -> Result<i32> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_cpu_processor_get_output_bit_depth(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return whether this CPU path is functionally identity.
    ///
    /// Returns `false` if OCIO reports an error. Use [`Self::try_is_identity`]
    /// to distinguish that case from a non-identity processor.
    pub fn is_identity(&self) -> bool {
        self.try_is_identity().unwrap_or(false)
    }

    /// Fallible variant of [`Self::is_identity`].
    ///
    /// Returns an error when the underlying OCIO query throws.
    pub fn try_is_identity(&self) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_cpu_processor_is_identity(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    // ── v2.5.1 ──
    /// Borrow a runtime-adjustable dynamic property from the CPU processor.
    ///
    /// Returns the OCIO error when the requested property is not available.
    pub fn dynamic_property(&self, prop_type: DynamicPropertyType) -> Result<DynamicProperty> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_cpu_processor_get_dynamic_property(
                self.handle.as_ptr(),
                prop_type as i32,
            )
        };
        crate::handle_result(handle).map(|handle| DynamicProperty { handle })
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
        self.try_has_dynamic_property_kind(prop_type)
            .unwrap_or(false)
    }

    /// Fallible variant of [`Self::has_dynamic_property_kind`].
    pub fn try_has_dynamic_property_kind(&self, prop_type: DynamicPropertyType) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_cpu_processor_has_dynamic_property(
                self.handle.as_ptr(),
                prop_type as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer has_dynamic_property_kind with DynamicPropertyType"
    )]
    pub fn has_dynamic_property(&self, prop_type: i32) -> bool {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_cpu_processor_has_dynamic_property(self.handle.as_ptr(), prop_type)
        };
        let _ = crate::ocio_call_status();
        value
    }

    /// Return whether this CPU processor contains any runtime-adjustable properties.
    pub fn is_dynamic(&self) -> bool {
        self.try_is_dynamic().unwrap_or(false)
    }

    /// Fallible variant of [`Self::is_dynamic`].
    pub fn try_is_dynamic(&self) -> Result<bool> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_cpu_processor_is_dynamic(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()?;
        Ok(value)
    }
}

impl Drop for CPUProcessor {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_cpu_processor_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

// --- GPUProcessor ---

/// GPU implementation of a [`Processor`].
///
/// Wraps a GPU-ready color pipeline and provides [`extract_shader_info`] to
/// populate a [`GpuShaderDesc`] with OCIO-generated shader text, textures,
/// and uniforms.
///
/// The typical usage is:
///
/// ```rust,no_run
/// # use ocio_rs::{Config, GpuShaderDesc, GpuLanguage};
/// # fn example() -> ocio_rs::Result<()> {
/// let config = Config::from_file("config.ocio")?;
/// let processor = config.processor("scene_linear", "srgb_texture")?;
/// let gpu = processor.default_gpu_processor()?;
///
/// let mut desc = GpuShaderDesc::create()?;
/// desc.set_language(GpuLanguage::Glsl1_2)?;
///
/// gpu.try_extract_shader_info(&mut desc)?;
///
/// let shader_text = desc.shader_text().unwrap_or_default();
/// // Use shader_text in your rendering pipeline...
/// # Ok(())
/// # }
/// ```
///
/// This handle is thread-confined; see [`Processor`].
///
/// ```compile_fail
/// # use ocio_rs::GPUProcessor;
/// fn require_send<T: Send>() {}
/// require_send::<GPUProcessor>();
/// ```
///
/// [`extract_shader_info`]: Self::extract_shader_info
/// [`GpuShaderDesc`]: crate::GpuShaderDesc
pub struct GPUProcessor {
    handle: NonNull<c_void>,
}

/// Shader sections supplied by OCIO to a custom assembler during extraction.
pub struct ShaderSections<'a> {
    /// Parameter declarations (or all declarations on OCIO 2.4), without a Vulkan uniform-block wrapper.
    pub parameters: &'a str,
    /// Texture and sampler declarations; empty on OCIO 2.4 where these are merged into `parameters`.
    pub textures: &'a str,
    /// Helper function definitions.
    pub helpers: &'a str,
    /// OCIO function signature and opening code.
    pub header: &'a str,
    /// OCIO color-processing function body.
    pub body: &'a str,
    /// OCIO function closing code.
    pub footer: &'a str,
}

/// A texture supplied by OCIO during shader extraction. Data is borrowed only
/// for the duration of the callback and may be uploaded without a Rust copy.
pub struct ShaderTexture<'a> {
    /// Name of the generated texture resource.
    pub texture_name: &'a str,
    /// Name of its sampler in the generated shader.
    pub sampler_name: &'a str,
    /// Width in texels.
    pub width: u32,
    /// Height in texels.
    pub height: u32,
    /// One for a 1D/2D texture, otherwise the 3D LUT edge length.
    pub depth: u32,
    /// Channel layout of the native texels.
    pub channel: GpuTextureChannel,
    /// `None` represents a 3D texture.
    pub dimensions: Option<GpuTextureDimensions>,
    /// Interpolation requested by OCIO.
    pub interpolation: Interpolation,
    /// Borrowed texel data, valid only while the callback is running.
    pub values: &'a [f32],
    /// Native proposed binding on OCIO 2.5+, absent on OCIO 2.4.
    pub suggested_binding: Option<u32>,
}

/// Storage and optional binding choice for one native texture registration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderTextureAction {
    /// Copy the texture into the returned native descriptor.
    StoreInDescriptor { binding: Option<u32> },
    /// The caller copied or uploaded the texture; do not retain it in the descriptor.
    External { binding: Option<u32> },
}

/// A uniform registered by OCIO during custom shader extraction.
pub struct ShaderUniform<'a> {
    /// Name used by the generated shader.
    pub name: &'a str,
    /// Native uniform value type.
    pub uniform_type: GpuUniformType,
    /// Index for querying this uniform on the returned descriptor.
    pub index: u32,
    /// Declared vector capacity on OCIO 2.5+; unavailable on 2.4 and for scalars.
    pub max_size: Option<u32>,
    /// Byte offset in OCIO's uniform buffer layout on OCIO 2.5+.
    pub buffer_offset: Option<usize>,
}

/// An owned live native uniform getter received during custom registration.
///
/// The getter keeps its native creator and dynamic properties alive even if
/// extraction fails. It is thread-confined and performs no bridge allocation
/// while sampling; native getter implementations may themselves allocate.
///
/// ```compile_fail
/// # use ocio_rs::ShaderUniformGetter;
/// fn require_send<T: Send>() {}
/// require_send::<ShaderUniformGetter>();
/// ```
pub struct ShaderUniformGetter {
    cache: GpuUniformCache<'static>,
    uniform_type: GpuUniformType,
}

impl ShaderUniformGetter {
    /// Native value type of this getter.
    pub fn uniform_type(&self) -> GpuUniformType {
        self.uniform_type
    }

    /// Read a double without narrowing; `None` indicates another type.
    pub fn double(&mut self) -> Result<Option<f64>> {
        self.cache.double(0)
    }
    /// Read a boolean, or `None` for a different uniform type.
    pub fn boolean(&mut self) -> Result<Option<bool>> {
        if self.uniform_type != GpuUniformType::Bool {
            return Ok(None);
        }
        let mut value = [0.0];
        match self.copy_f32_into(&mut value)? {
            GpuUniformCopy::Written(1) => Ok(Some(value[0] != 0.0)),
            _ => Err(OcioError::Ocio(
                "native boolean getter is unavailable".into(),
            )),
        }
    }
    /// Read a float3, or `None` for a different uniform type.
    pub fn float3(&mut self) -> Result<Option<[f32; 3]>> {
        if self.uniform_type != GpuUniformType::Float3 {
            return Ok(None);
        }
        let mut value = [0.0; 3];
        match self.copy_f32_into(&mut value)? {
            GpuUniformCopy::Written(3) => Ok(Some(value)),
            _ => Err(OcioError::Ocio(
                "native float3 getter is unavailable".into(),
            )),
        }
    }
    /// Copy scalar, float3, or float-array values into caller-owned storage.
    pub fn copy_f32_into(&mut self, values: &mut [f32]) -> Result<GpuUniformCopy> {
        self.cache.copy_f32_into(0, values)
    }
    /// Copy integer-array values into caller-owned storage.
    pub fn copy_i32_into(&mut self, values: &mut [i32]) -> Result<GpuUniformCopy> {
        self.cache.copy_i32_into(0, values)
    }
}

/// A native uniform registration request and its independently owned getter.
pub struct ShaderUniformRegistration {
    /// Name supplied by OCIO.
    pub name: String,
    /// Native value type.
    pub uniform_type: GpuUniformType,
    /// Declared vector capacity on native 2.5+, otherwise unavailable.
    pub max_size: Option<u32>,
    /// Live getter. Move it out to retain it after the callback.
    pub getter: ShaderUniformGetter,
}

/// How a custom creator handles a native uniform registration request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderUniformAction {
    /// Let OCIO's descriptor store the getter and determine the return value.
    StoreInDescriptor,
    /// The caller stored a new uniform; report `true` to OCIO.
    ExternalAdded,
    /// The caller already had the uniform; report `false` to OCIO.
    ExternalDuplicate,
}

/// Options for extracting with custom creator resource handling.
#[derive(Default, Clone, Copy)]
pub struct ShaderCreatorOptions<'a> {
    /// Replace the creator's native cache ID with this value.
    ///
    /// OCIO uses this ID when deriving resource keys. The caller must ensure
    /// the ID changes whenever creator configuration or output changes,
    /// including after mutating the returned descriptor. Update it with
    /// [`GpuShaderDesc::set_custom_cache_id`]. The string is copied
    /// once into native ownership. `None` retains OCIO's default behavior.
    /// Native descriptor cloning creates a generic descriptor and does not
    /// preserve this override.
    pub cache_id_override: Option<&'a std::ffi::CStr>,
    /// Deliver textures to the supplied sink without retaining descriptor copies.
    pub stream_textures: bool,
}

/// Custom control over native uniform registration during shader extraction.
///
/// ```no_run
/// # use ocio_rs::{GPUProcessor, GpuShaderDesc, Result, ShaderExtractionHooks,
/// #     ShaderUniformAction, ShaderUniformControl, ShaderUniformGetter,
/// #     ShaderUniformRegistration};
/// struct Uniforms(Vec<(String, ShaderUniformGetter)>);
/// impl ShaderExtractionHooks for Uniforms {}
/// impl ShaderUniformControl for Uniforms {
///     fn register_uniform(&mut self, uniform: ShaderUniformRegistration) -> Result<ShaderUniformAction> {
///         self.0.push((uniform.name, uniform.getter));
///         Ok(ShaderUniformAction::ExternalAdded)
///     }
/// }
/// # fn example(gpu: &GPUProcessor, template: &GpuShaderDesc) -> Result<()> {
/// let mut uniforms = Uniforms(Vec::new());
/// let desc = gpu.try_extract_with_uniform_control(template, &mut uniforms)?;
/// // Allocate and populate external GPU storage from the retained getters.
/// # let _ = desc;
/// # Ok(()) }
/// ```
pub trait ShaderUniformControl: ShaderExtractionHooks {
    /// Decide where to store this uniform and which result to return to OCIO.
    fn register_uniform(
        &mut self,
        uniform: ShaderUniformRegistration,
    ) -> Result<ShaderUniformAction>;
}

/// Optional synchronous replacement for OCIO's native shader finalization stage.
///
/// Callbacks are detached when extraction returns; later descriptor operations
/// use the native implementation and do not call this object.
pub trait ShaderFinalizeControl: ShaderUniformControl {
    /// Return complete shader text to bypass OCIO's language wrapping and
    /// finalization, or `None` to delegate to OCIO. The implementation may
    /// collect the raw fragments delivered by [`ShaderExtractionHooks::code`].
    fn finalize_shader(&mut self) -> Result<Option<String>>;
}

/// Synchronous policy for OCIO's native texture-width and 1D-texture getters.
///
/// These methods can be called repeatedly during extraction. The policy is
/// detached before the resulting descriptor is returned. Its maximum width
/// must be at least two; supported OCIO versions have a separate LUT padding
/// bug for certain width/length pairs, so the caller must ensure its policies
/// are safe for every generated LUT. For a conservative probe see
/// [`GPUProcessor::try_extract_with_texture_policy_checked`]. No callback runs
/// in the GPU pixel loop.
/// Setter events update caller-owned policy state and are not rolled back if
/// extraction later fails. A checked extraction performs its probe before
/// forwarding the template's setter events.
///
/// ```
/// # use ocio_rs::{ShaderTexturePolicy, Result};
/// struct Policy;
/// impl ShaderTexturePolicy for Policy {
///     fn texture_max_width(&mut self) -> Result<u32> { Ok(4096) }
///     fn allow_texture_1d(&mut self) -> Result<bool> { Ok(false) }
/// }
/// ```
pub trait ShaderTexturePolicy {
    /// Observe or reject a native width setter. The default accepts it.
    /// Called with the template's setting before extraction begins.
    fn set_texture_max_width(&mut self, _width: u32) -> Result<()> {
        Ok(())
    }
    /// Observe or reject a native 1D setting. The default accepts it.
    /// Called with the template's setting before extraction begins.
    fn set_allow_texture_1d(&mut self, _allowed: bool) -> Result<()> {
        Ok(())
    }
    /// Maximum native texture width for the current query.
    fn texture_max_width(&mut self) -> Result<u32>;
    /// Whether OCIO may emit 1D textures for the current query.
    fn allow_texture_1d(&mut self) -> Result<bool>;
}

/// Lifecycle stages of a native shader-creator extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderLifecycle<'a> {
    /// OCIO has started collection with this generated resource identifier.
    Begin { unique_id: &'a str },
    /// OCIO has finalized and assembled the shader.
    Finalized,
    /// OCIO has ended collection, including after an extraction error once
    /// `Begin` returned successfully. A failed `Begin` has no `End` event.
    End,
}

/// Native shader code section currently being appended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderCodeSection {
    /// Combined parameter and texture declarations on OCIO 2.4.
    Declarations,
    /// Parameter declarations on OCIO 2.5 and newer.
    Parameters,
    /// Texture declarations on OCIO 2.5 and newer.
    Textures,
    /// Helper function definitions.
    Helpers,
    /// Function opening code.
    Header,
    /// Function body code.
    Body,
    /// Function closing code.
    Footer,
}

/// Synchronous customization hooks for OCIO shader extraction.
///
/// Defaults retain OCIO's resource handling, forward individual code
/// fragments, and use OCIO's native shader assembly.
/// Implementations may override selected callbacks; they must not reenter
/// extraction on the same processor. Callbacks must not retain borrowed inputs.
///
/// ```no_run
/// # use ocio_rs::{GPUProcessor, GpuShaderDesc, Result, ShaderCodeSection, ShaderExtractionHooks};
/// struct MarkBody;
/// impl ShaderExtractionHooks for MarkBody {
///     fn code(&mut self, section: ShaderCodeSection, code: &str) -> Result<Option<String>> {
///         if section == ShaderCodeSection::Body {
///             Ok(Some(format!("// application hook\n{code}")))
///         } else {
///             Ok(None)
///         }
///     }
/// }
/// # fn extract(gpu: &GPUProcessor, template: &GpuShaderDesc) -> Result<()> {
/// let desc = gpu.try_extract_with_hooks(template, &mut MarkBody)?;
/// # let _ = desc;
/// # Ok(()) }
/// ```
pub trait ShaderExtractionHooks {
    /// Inspect a lifecycle event.
    fn lifecycle(&mut self, _event: ShaderLifecycle<'_>) -> Result<()> {
        Ok(())
    }
    /// Inspect a texture and optionally override its binding on OCIO 2.5+.
    fn texture(&mut self, _texture: ShaderTexture<'_>) -> Result<Option<u32>> {
        Ok(None)
    }
    /// Inspect a registered uniform.
    fn uniform(&mut self, _uniform: ShaderUniform<'_>) -> Result<()> {
        Ok(())
    }
    /// Return replacement code, or `None` to forward the original without copying.
    fn code(&mut self, _section: ShaderCodeSection, _code: &str) -> Result<Option<String>> {
        Ok(None)
    }
    /// Return replacement shader text, or `None` to use OCIO's native assembly.
    /// A replacement must include any required language-specific uniform block.
    fn assemble(&mut self, _parts: ShaderSections<'_>) -> Result<Option<String>> {
        Ok(None)
    }
}

impl GPUProcessor {
    /// Extract a fresh descriptor while assembling its shader text in Rust.
    ///
    /// The template supplies language, resource names, binding settings and
    /// texture preferences; existing template resources are not copied. OCIO
    /// retains ownership of resources and live dynamic properties in the result.
    /// The callback runs synchronously and must return the complete shader text.
    /// Callback errors and panics are returned as errors without crossing FFI.
    /// OCIO holds its GPU processor lock during the callback; do not reenter
    /// extraction on the same processor. For Vulkan, the callback must include
    /// any desired uniform-buffer wrapper around `parameters` itself.
    ///
    /// ```no_run
    /// # use ocio_rs::{GPUProcessor, GpuShaderDesc, Result};
    /// # fn example(gpu: &GPUProcessor, template: &GpuShaderDesc) -> Result<()> {
    /// let desc = gpu.try_extract_with_assembler(template, |parts| {
    ///     Ok(format!("// custom\n{}{}{}{}{}{}", parts.parameters, parts.textures,
    ///         parts.helpers, parts.header, parts.body, parts.footer))
    /// })?;
    /// assert!(desc.shader_text().unwrap().starts_with("// custom"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_extract_with_assembler<F>(
        &self,
        template: &GpuShaderDesc,
        mut assemble: F,
    ) -> Result<GpuShaderDesc>
    where
        F: FnMut(ShaderSections<'_>) -> Result<String>,
    {
        self.try_extract_with_callbacks(
            template,
            false,
            None,
            None,
            &mut |_| Ok(ShaderUniformAction::StoreInDescriptor),
            false,
            |parts| assemble(parts).map(Some),
            |_| Ok(None),
            false,
            &mut |_| Ok(()),
            false,
            &mut |_| Ok(()),
            false,
            &mut |_, _| Ok(None),
            false,
            None,
        )
    }

    /// Extract a shader with synchronous Rust assembly and texture callbacks.
    ///
    /// The texture callback receives a temporary borrow of each native LUT and
    /// may return `Some(binding)` to override OCIO's proposed binding on 2.5+.
    /// Return `None` to keep the native binding; OCIO 2.4 requires `None`.
    /// Binding zero and duplicate bindings are rejected. Callback errors and
    /// panics abort extraction, but external uploads already performed by the
    /// callback remain the caller's responsibility. Do not reenter extraction
    /// on this GPU processor from either callback.
    ///
    /// Borrowed texture values cannot escape the callback:
    ///
    /// ```compile_fail
    /// # use ocio_rs::{GPUProcessor, GpuShaderDesc};
    /// # fn example(gpu: &GPUProcessor, template: &GpuShaderDesc) {
    /// let mut saved = None;
    /// let _ = gpu.try_extract_with_texture_callback(template,
    ///     |_| Ok(String::new()), |texture| { saved = Some(texture.values); Ok(None) });
    /// println!("{:?}", saved);
    /// # }
    /// ```
    pub fn try_extract_with_texture_callback<F, G>(
        &self,
        template: &GpuShaderDesc,
        mut assemble: F,
        texture: G,
    ) -> Result<GpuShaderDesc>
    where
        F: FnMut(ShaderSections<'_>) -> Result<String>,
        G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
    {
        self.try_extract_with_callbacks(
            template,
            false,
            None,
            None,
            &mut |_| Ok(ShaderUniformAction::StoreInDescriptor),
            false,
            |parts| assemble(parts).map(Some),
            texture,
            true,
            &mut |_| Ok(()),
            false,
            &mut |_| Ok(()),
            false,
            &mut |_, _| Ok(None),
            false,
            None,
        )
    }

    /// Extract with synchronous shader assembly, texture, and uniform callbacks.
    ///
    /// Uniform events are sent only for newly registered uniforms, before shader
    /// assembly. Use the reported index to query current dynamic values from the
    /// returned descriptor; OCIO retains the getters. Names are borrowed only
    /// for the callback. OCIO 2.4 does not expose vector maximum sizes or
    /// uniform-buffer offsets.
    /// Callback errors and panics abort extraction; already performed external
    /// resource allocations remain the caller's responsibility. Do not reenter
    /// extraction on this GPU processor from any callback.
    ///
    /// ```compile_fail
    /// # use ocio_rs::{GPUProcessor, GpuShaderDesc};
    /// # fn example(gpu: &GPUProcessor, desc: &GpuShaderDesc) {
    /// let mut saved = None;
    /// let _ = gpu.try_extract_with_resource_callbacks(desc, |_| Ok(String::new()),
    ///     |_| Ok(None), |uniform| { saved = Some(uniform.name); Ok(()) });
    /// println!("{:?}", saved);
    /// # }
    /// ```
    pub fn try_extract_with_resource_callbacks<F, G, H>(
        &self,
        template: &GpuShaderDesc,
        mut assemble: F,
        texture: G,
        mut uniform: H,
    ) -> Result<GpuShaderDesc>
    where
        F: FnMut(ShaderSections<'_>) -> Result<String>,
        G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
        H: FnMut(ShaderUniform<'_>) -> Result<()>,
    {
        self.try_extract_with_callbacks(
            template,
            false,
            None,
            None,
            &mut |_| Ok(ShaderUniformAction::StoreInDescriptor),
            false,
            |parts| assemble(parts).map(Some),
            texture,
            true,
            &mut uniform,
            true,
            &mut |_| Ok(()),
            false,
            &mut |_, _| Ok(None),
            false,
            None,
        )
    }

    /// Extract with shader, texture, uniform, and creator-lifecycle callbacks.
    ///
    /// Events occur synchronously in OCIO order: `Begin`, zero or more
    /// resource callbacks, shader assembly, `Finalized`, then `End`. If an
    /// extraction callback fails after `Begin`, OCIO still calls `End`;
    /// an error from `Begin` itself does not trigger `End`. The first callback
    /// error is preserved if cleanup also fails. IDs are borrowed for the
    /// callback only. Do not reenter extraction on this processor.
    ///
    /// ```compile_fail
    /// # use ocio_rs::{GPUProcessor, GpuShaderDesc, ShaderLifecycle};
    /// # fn example(gpu: &GPUProcessor, desc: &GpuShaderDesc) {
    /// let mut saved = None;
    /// let _ = gpu.try_extract_with_lifecycle_callbacks(desc, |_| Ok(String::new()),
    ///     |_| Ok(None), |_| Ok(()), |event| {
    ///         if let ShaderLifecycle::Begin { unique_id } = event { saved = Some(unique_id); }
    ///         Ok(())
    ///     });
    /// println!("{:?}", saved);
    /// # }
    /// ```
    pub fn try_extract_with_lifecycle_callbacks<F, G, H, L>(
        &self,
        template: &GpuShaderDesc,
        mut assemble: F,
        texture: G,
        mut uniform: H,
        mut lifecycle: L,
    ) -> Result<GpuShaderDesc>
    where
        F: FnMut(ShaderSections<'_>) -> Result<String>,
        G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
        H: FnMut(ShaderUniform<'_>) -> Result<()>,
        L: FnMut(ShaderLifecycle<'_>) -> Result<()>,
    {
        self.try_extract_with_callbacks(
            template,
            false,
            None,
            None,
            &mut |_| Ok(ShaderUniformAction::StoreInDescriptor),
            false,
            |parts| assemble(parts).map(Some),
            texture,
            true,
            &mut uniform,
            true,
            &mut lifecycle,
            true,
            &mut |_, _| Ok(None),
            false,
            None,
        )
    }

    /// Extract using a single hooks object for code, resources, and lifecycle.
    ///
    /// Events are synchronous; callback errors and panics abort extraction and
    /// any already-created external resources remain the caller's responsibility.
    /// On OCIO 2.4 declaration events are combined into `Declarations`.
    pub fn try_extract_with_hooks<H: ShaderExtractionHooks>(
        &self,
        template: &GpuShaderDesc,
        hooks: &mut H,
    ) -> Result<GpuShaderDesc> {
        let hooks = std::cell::RefCell::new(hooks);
        self.try_extract_with_callbacks(
            template,
            false,
            None,
            None,
            &mut |_| Ok(ShaderUniformAction::StoreInDescriptor),
            false,
            |parts| hooks.borrow_mut().assemble(parts),
            |texture| hooks.borrow_mut().texture(texture),
            true,
            &mut |uniform| hooks.borrow_mut().uniform(uniform),
            true,
            &mut |event| hooks.borrow_mut().lifecycle(event),
            true,
            &mut |section, code| hooks.borrow_mut().code(section, code),
            true,
            None,
        )
    }

    /// Extract using hooks while streaming texture data directly to the caller.
    ///
    /// The required texture sink must upload or copy the borrowed data before it
    /// returns; the returned descriptor does not retain or report these
    /// textures. Uniforms and shader text remain available on the descriptor.
    /// On failure, cleanup of any resources created by hooks is the caller's
    /// responsibility. The hooks object's `texture` method is not called;
    /// binding overrides come from the supplied sink.
    ///
    /// ```compile_fail
    /// # use ocio_rs::{GPUProcessor, GpuShaderDesc, ShaderExtractionHooks};
    /// # struct Hooks;
    /// # impl ShaderExtractionHooks for Hooks {}
    /// # fn example(gpu: &GPUProcessor, desc: &GpuShaderDesc) {
    /// let mut saved = None;
    /// let _ = gpu.try_extract_with_streamed_textures(desc, &mut Hooks, |texture| {
    ///     saved = Some(texture.values);
    ///     Ok(None)
    /// });
    /// println!("{saved:?}");
    /// # }
    /// ```
    pub fn try_extract_with_streamed_textures<H, T>(
        &self,
        template: &GpuShaderDesc,
        hooks: &mut H,
        texture: T,
    ) -> Result<GpuShaderDesc>
    where
        H: ShaderExtractionHooks,
        T: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
    {
        let hooks = std::cell::RefCell::new(hooks);
        self.try_extract_with_callbacks(
            template,
            true,
            None,
            None,
            &mut |_| Ok(ShaderUniformAction::StoreInDescriptor),
            false,
            |parts| hooks.borrow_mut().assemble(parts),
            texture,
            true,
            &mut |uniform| hooks.borrow_mut().uniform(uniform),
            true,
            &mut |event| hooks.borrow_mut().lifecycle(event),
            true,
            &mut |section, code| hooks.borrow_mut().code(section, code),
            true,
            None,
        )
    }

    /// Extract using hooks that control native uniform registration.
    ///
    /// External uniforms are not included in the descriptor's uniform list or
    /// buffer offsets. The caller must retain their getters and manage storage,
    /// layout and cleanup. Callback errors and panics abort extraction; owned
    /// getters already transferred to the caller remain usable. Do not reenter
    /// extraction on the same GPU processor from a callback.
    pub fn try_extract_with_uniform_control<H: ShaderUniformControl>(
        &self,
        template: &GpuShaderDesc,
        hooks: &mut H,
    ) -> Result<GpuShaderDesc> {
        let hooks = std::cell::RefCell::new(hooks);
        let mut registration = |uniform| hooks.borrow_mut().register_uniform(uniform);
        self.try_extract_with_callbacks(
            template,
            false,
            None,
            None,
            &mut registration,
            true,
            |parts| hooks.borrow_mut().assemble(parts),
            |texture| hooks.borrow_mut().texture(texture),
            true,
            &mut |uniform| hooks.borrow_mut().uniform(uniform),
            true,
            &mut |event| hooks.borrow_mut().lifecycle(event),
            true,
            &mut |section, code| hooks.borrow_mut().code(section, code),
            true,
            None,
        )
    }

    /// Control uniform registration and stream textures in the same extraction.
    ///
    /// External uniforms and streamed textures are not enumerated by the
    /// returned descriptor. The sink must copy or upload each texture before
    /// returning, and retained uniform getters keep their native owner alive.
    /// As with [`Self::try_extract_with_streamed_textures`], the hooks object's
    /// texture method is not called. Callers own cleanup on failure.
    pub fn try_extract_with_uniform_control_and_streamed_textures<H, T>(
        &self,
        template: &GpuShaderDesc,
        hooks: &mut H,
        texture: T,
    ) -> Result<GpuShaderDesc>
    where
        H: ShaderUniformControl,
        T: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
    {
        let hooks = std::cell::RefCell::new(hooks);
        let mut registration = |uniform| hooks.borrow_mut().register_uniform(uniform);
        self.try_extract_with_callbacks(
            template,
            true,
            None,
            None,
            &mut registration,
            true,
            |parts| hooks.borrow_mut().assemble(parts),
            texture,
            true,
            &mut |uniform| hooks.borrow_mut().uniform(uniform),
            true,
            &mut |event| hooks.borrow_mut().lifecycle(event),
            true,
            &mut |section, code| hooks.borrow_mut().code(section, code),
            true,
            None,
        )
    }

    /// Extract with custom registration, texture handling and creator options.
    ///
    /// The texture callback receives each LUT synchronously whether or not
    /// streaming is selected. In streaming mode it must copy or upload the
    /// borrowed data before returning. A custom cache ID remains in effect on
    /// the returned descriptor; see [`ShaderCreatorOptions`] for invalidation
    /// responsibility. Callback and external resource cleanup rules are the same
    /// as [`Self::try_extract_with_uniform_control_and_streamed_textures`].
    pub fn try_extract_with_creator_options<H, T>(
        &self,
        template: &GpuShaderDesc,
        hooks: &mut H,
        options: ShaderCreatorOptions<'_>,
        texture: T,
    ) -> Result<GpuShaderDesc>
    where
        H: ShaderUniformControl,
        T: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
    {
        let hooks = std::cell::RefCell::new(hooks);
        let mut registration = |uniform| hooks.borrow_mut().register_uniform(uniform);
        self.try_extract_with_callbacks(
            template,
            options.stream_textures,
            None,
            options.cache_id_override,
            &mut registration,
            true,
            |parts| hooks.borrow_mut().assemble(parts),
            texture,
            true,
            &mut |uniform| hooks.borrow_mut().uniform(uniform),
            true,
            &mut |event| hooks.borrow_mut().lifecycle(event),
            true,
            &mut |section, code| hooks.borrow_mut().code(section, code),
            true,
            None,
        )
    }

    /// Choose storage separately for each texture and control uniform registration.
    ///
    /// External textures are not returned by the descriptor; their data must be
    /// copied or uploaded before the callback returns. On 2.5+, bindings are
    /// shared across all texture types, including external textures. A binding
    /// override is not supported on 2.4. Callback errors and panics abort
    /// extraction; external resources already created remain the caller's responsibility.
    /// The hooks object's `texture` method is not called. Do not reenter extraction.
    pub fn try_extract_with_texture_control<H, T>(
        &self,
        template: &GpuShaderDesc,
        hooks: &mut H,
        cache_id_override: Option<&std::ffi::CStr>,
        mut texture: T,
    ) -> Result<GpuShaderDesc>
    where
        H: ShaderUniformControl,
        T: FnMut(ShaderTexture<'_>) -> Result<ShaderTextureAction>,
    {
        let hooks = std::cell::RefCell::new(hooks);
        let storage = std::cell::Cell::new(true);
        let mut registration = |uniform| hooks.borrow_mut().register_uniform(uniform);
        self.try_extract_with_callbacks(
            template,
            false,
            Some(&storage),
            cache_id_override,
            &mut registration,
            true,
            |parts| hooks.borrow_mut().assemble(parts),
            |event| match texture(event)? {
                ShaderTextureAction::StoreInDescriptor { binding } => {
                    storage.set(true);
                    Ok(binding)
                }
                ShaderTextureAction::External { binding } => {
                    storage.set(false);
                    Ok(binding)
                }
            },
            true,
            &mut |uniform| hooks.borrow_mut().uniform(uniform),
            true,
            &mut |event| hooks.borrow_mut().lifecycle(event),
            true,
            &mut |section, code| hooks.borrow_mut().code(section, code),
            true,
            None,
        )
    }

    /// Extract with independent texture/uniform storage and optional custom finalization.
    ///
    /// A custom finalizer receives control before OCIO performs its language
    /// wrapping. It must return a complete shader, including any required
    /// declarations and language-specific wrappers. Returning `None` delegates
    /// finalization and assembly to the ordinary native path. This control is
    /// scoped to the extraction and is not retained by the returned descriptor.
    /// Finalizer errors
    /// and panics abort extraction; already-created external resources remain
    /// the caller's responsibility. Do not reenter extraction from callbacks.
    pub fn try_extract_with_finalize_control<H, T>(
        &self,
        template: &GpuShaderDesc,
        hooks: &mut H,
        cache_id_override: Option<&std::ffi::CStr>,
        mut texture: T,
    ) -> Result<GpuShaderDesc>
    where
        H: ShaderFinalizeControl,
        T: FnMut(ShaderTexture<'_>) -> Result<ShaderTextureAction>,
    {
        let hooks = std::cell::RefCell::new(hooks);
        let storage = std::cell::Cell::new(true);
        let mut registration = |uniform| hooks.borrow_mut().register_uniform(uniform);
        let mut finalizer = || hooks.borrow_mut().finalize_shader();
        self.try_extract_with_callbacks(
            template,
            false,
            Some(&storage),
            cache_id_override,
            &mut registration,
            true,
            |parts| hooks.borrow_mut().assemble(parts),
            |event| match texture(event)? {
                ShaderTextureAction::StoreInDescriptor { binding } => {
                    storage.set(true);
                    Ok(binding)
                }
                ShaderTextureAction::External { binding } => {
                    storage.set(false);
                    Ok(binding)
                }
            },
            true,
            &mut |uniform| hooks.borrow_mut().uniform(uniform),
            true,
            &mut |event| hooks.borrow_mut().lifecycle(event),
            true,
            &mut |section, code| hooks.borrow_mut().code(section, code),
            true,
            Some(&mut finalizer),
        )
    }

    /// Extract with independently controlled texture policy, resource storage,
    /// uniforms and finalization. Policy callbacks are synchronous and detached
    /// on every return path. Do not reenter the same processor from callbacks.
    ///
    /// The native width getter is `noexcept`: on a callback failure or panic,
    /// extraction uses a conservative temporary fallback and then returns the
    /// original error. Other callbacks and external uploads may have occurred
    /// before failure and their cleanup remains the caller's responsibility.
    /// Policy values may vary during extraction, so a template-only layout probe
    /// cannot prove them safe; consider [`Self::try_extract_with_texture_policy_checked`].
    /// `cache_id` must be nonempty and uniquely represent the policy and output;
    /// update it whenever they change to avoid native resource-key collisions.
    pub fn try_extract_with_texture_policy<H, P, T>(
        &self,
        template: &GpuShaderDesc,
        hooks: &mut H,
        policy: &mut P,
        cache_id: &std::ffi::CStr,
        mut texture: T,
    ) -> Result<GpuShaderDesc>
    where
        H: ShaderFinalizeControl + ?Sized,
        P: ShaderTexturePolicy + ?Sized,
        T: FnMut(ShaderTexture<'_>) -> Result<ShaderTextureAction>,
    {
        if cache_id.to_bytes().is_empty() {
            return Err(OcioError::InvalidInput(
                "texture policy requires a nonempty cache ID".into(),
            ));
        }
        let hooks = std::cell::RefCell::new(hooks);
        let storage = std::cell::Cell::new(true);
        let mut registration = |uniform| hooks.borrow_mut().register_uniform(uniform);
        let mut finalizer = || hooks.borrow_mut().finalize_shader();
        let mut query = |kind, input| match kind {
            0 => policy.texture_max_width(),
            1 => policy.allow_texture_1d().map(u32::from),
            2 => policy.set_texture_max_width(input).map(|()| input),
            3 => policy.set_allow_texture_1d(input != 0).map(|()| input),
            _ => Err(OcioError::InvalidInput(
                "invalid texture policy query".into(),
            )),
        };
        self.try_extract_with_callbacks_and_policy(
            template,
            false,
            Some(&storage),
            Some(cache_id),
            &mut registration,
            true,
            |parts| hooks.borrow_mut().assemble(parts),
            |event| match texture(event)? {
                ShaderTextureAction::StoreInDescriptor { binding } => {
                    storage.set(true);
                    Ok(binding)
                }
                ShaderTextureAction::External { binding } => {
                    storage.set(false);
                    Ok(binding)
                }
            },
            true,
            &mut |uniform| hooks.borrow_mut().uniform(uniform),
            true,
            &mut |event| hooks.borrow_mut().lifecycle(event),
            true,
            &mut |section, code| hooks.borrow_mut().code(section, code),
            true,
            Some(&mut finalizer),
            Some(&mut query),
        )
    }

    /// Conservatively probe optimized texture lengths, then validate every
    /// width returned by `policy` against every probed length before OCIO uses it.
    ///
    /// This generates the shader twice and may reject non-LUT 1D/2D resources.
    /// It relies on the native 2.4–2.6 LUT length limit and does not validate
    /// newer versions or arbitrary native creators. The processor and any state
    /// capable of changing generated resources must remain unchanged across the
    /// probe and actual extraction, including inside user callbacks. External
    /// resources created before a later policy error remain caller-owned.
    /// `cache_id` has the same requirements as on
    /// [`Self::try_extract_with_texture_policy`].
    pub fn try_extract_with_texture_policy_checked<H, P, T>(
        &self,
        template: &GpuShaderDesc,
        hooks: &mut H,
        policy: &mut P,
        cache_id: &std::ffi::CStr,
        texture: T,
    ) -> Result<GpuShaderDesc>
    where
        H: ShaderFinalizeControl + ?Sized,
        P: ShaderTexturePolicy + ?Sized,
        T: FnMut(ShaderTexture<'_>) -> Result<ShaderTextureAction>,
    {
        struct CheckedPolicy<'a, P: ?Sized> {
            policy: &'a mut P,
            lengths: &'a [usize],
        }
        impl<P: ShaderTexturePolicy + ?Sized> ShaderTexturePolicy for CheckedPolicy<'_, P> {
            fn set_texture_max_width(&mut self, width: u32) -> Result<()> {
                self.policy.set_texture_max_width(width)
            }
            fn set_allow_texture_1d(&mut self, allowed: bool) -> Result<()> {
                self.policy.set_allow_texture_1d(allowed)
            }
            fn texture_max_width(&mut self) -> Result<u32> {
                let width = self.policy.texture_max_width()?;
                if width < 2 {
                    return Err(OcioError::InvalidInput(
                        "texture maximum width must be at least 2".into(),
                    ));
                }
                for &length in self.lengths {
                    GpuShaderDesc::validate_lut_texture_width(width, length)?;
                }
                Ok(width)
            }
            fn allow_texture_1d(&mut self) -> Result<bool> {
                self.policy.allow_texture_1d()
            }
        }
        if cache_id.to_bytes().is_empty() {
            return Err(OcioError::InvalidInput(
                "texture policy requires a nonempty cache ID".into(),
            ));
        }
        let (lengths, _) = self.probe_shader_texture_lengths(template)?;
        let mut checked = CheckedPolicy {
            policy,
            lengths: &lengths,
        };
        self.try_extract_with_texture_policy(template, hooks, &mut checked, cache_id, texture)
    }

    #[allow(clippy::too_many_arguments)]
    fn try_extract_with_callbacks<F, G>(
        &self,
        template: &GpuShaderDesc,
        stream_textures: bool,
        texture_storage: Option<&std::cell::Cell<bool>>,
        cache_id_override: Option<&std::ffi::CStr>,
        registration: &mut dyn FnMut(ShaderUniformRegistration) -> Result<ShaderUniformAction>,
        has_registration: bool,
        assemble: F,
        texture: G,
        notify_texture: bool,
        uniform: &mut dyn FnMut(ShaderUniform<'_>) -> Result<()>,
        notify_uniform: bool,
        lifecycle: &mut dyn FnMut(ShaderLifecycle<'_>) -> Result<()>,
        notify_lifecycle: bool,
        code: &mut dyn FnMut(ShaderCodeSection, &str) -> Result<Option<String>>,
        notify_code: bool,
        finalizer: Option<&mut dyn FnMut() -> Result<Option<String>>>,
    ) -> Result<GpuShaderDesc>
    where
        F: FnMut(ShaderSections<'_>) -> Result<Option<String>>,
        G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
    {
        self.try_extract_with_callbacks_and_policy(
            template,
            stream_textures,
            texture_storage,
            cache_id_override,
            registration,
            has_registration,
            assemble,
            texture,
            notify_texture,
            uniform,
            notify_uniform,
            lifecycle,
            notify_lifecycle,
            code,
            notify_code,
            finalizer,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn try_extract_with_callbacks_and_policy<F, G>(
        &self,
        template: &GpuShaderDesc,
        stream_textures: bool,
        texture_storage: Option<&std::cell::Cell<bool>>,
        cache_id_override: Option<&std::ffi::CStr>,
        registration: &mut dyn FnMut(ShaderUniformRegistration) -> Result<ShaderUniformAction>,
        has_registration: bool,
        mut assemble: F,
        mut texture: G,
        notify_texture: bool,
        uniform: &mut dyn FnMut(ShaderUniform<'_>) -> Result<()>,
        notify_uniform: bool,
        lifecycle: &mut dyn FnMut(ShaderLifecycle<'_>) -> Result<()>,
        notify_lifecycle: bool,
        code: &mut dyn FnMut(ShaderCodeSection, &str) -> Result<Option<String>>,
        notify_code: bool,
        mut finalizer: Option<&mut dyn FnMut() -> Result<Option<String>>>,
        mut policy: Option<&mut dyn FnMut(i32, u32) -> Result<u32>>,
    ) -> Result<GpuShaderDesc>
    where
        F: FnMut(ShaderSections<'_>) -> Result<Option<String>>,
        G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
    {
        struct State<'a, F, G> {
            function: &'a mut F,
            texture: &'a mut G,
            texture_storage: Option<&'a std::cell::Cell<bool>>,
            uniform: &'a mut dyn FnMut(ShaderUniform<'_>) -> Result<()>,
            lifecycle: &'a mut dyn FnMut(ShaderLifecycle<'_>) -> Result<()>,
            code: &'a mut dyn FnMut(ShaderCodeSection, &str) -> Result<Option<String>>,
            registration:
                &'a mut dyn FnMut(ShaderUniformRegistration) -> Result<ShaderUniformAction>,
            finalizer: Option<&'a mut dyn FnMut() -> Result<Option<String>>>,
            policy: Option<&'a mut dyn FnMut(i32, u32) -> Result<u32>>,
            output: Option<std::ffi::CString>,
            code_output: Option<std::ffi::CString>,
            finalizer_output: Option<std::ffi::CString>,
            error: Option<OcioError>,
        }
        unsafe extern "C" fn policy_callback<F, G>(
            context: *mut c_void,
            kind: i32,
            value: *mut u32,
        ) -> bool
        where
            F: FnMut(ShaderSections<'_>) -> Result<Option<String>>,
            G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
        {
            let state = unsafe { &mut *context.cast::<State<'_, F, G>>() };
            if state.error.is_some() {
                return false;
            }
            let result =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<u32> {
                    if value.is_null() || !(0..=3).contains(&kind) {
                        return Err(OcioError::InvalidInput(
                            "invalid texture policy event".into(),
                        ));
                    }
                    let policy = state
                        .policy
                        .as_mut()
                        .ok_or_else(|| OcioError::InvalidInput("missing texture policy".into()))?;
                    let input = unsafe { *value };
                    if kind == 3 && input > 1 {
                        return Err(OcioError::InvalidInput(
                            "invalid texture policy setter value".into(),
                        ));
                    }
                    let result = policy(kind, input)?;
                    if (kind == 0 && result < 2) || (kind == 1 && result > 1) {
                        return Err(OcioError::InvalidInput(
                            "invalid texture policy value".into(),
                        ));
                    }
                    Ok(result)
                }));
            match result {
                Ok(Ok(result)) => {
                    unsafe { *value = result };
                    true
                }
                Ok(Err(error)) => {
                    state.error.get_or_insert(error);
                    false
                }
                Err(_) => {
                    state.error.get_or_insert_with(|| {
                        OcioError::InvalidInput("texture policy callback panicked".into())
                    });
                    false
                }
            }
        }
        unsafe extern "C" fn finalize_callback<F, G>(
            context: *mut c_void,
            output: *mut *const i8,
        ) -> bool
        where
            F: FnMut(ShaderSections<'_>) -> Result<Option<String>>,
            G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
        {
            let state = unsafe { &mut *context.cast::<State<'_, F, G>>() };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || -> Result<Option<std::ffi::CString>> {
                    if output.is_null() {
                        return Err(OcioError::InvalidInput(
                            "null shader finalizer output".into(),
                        ));
                    }
                    state.finalizer.as_mut().ok_or_else(|| {
                        OcioError::InvalidInput("missing shader finalizer".into())
                    })?()?
                    .map(std::ffi::CString::new)
                    .transpose()
                    .map_err(|_| OcioError::InteriorNul)
                },
            ));
            match result {
                Ok(Ok(value)) => {
                    state.finalizer_output = value;
                    unsafe {
                        *output = state
                            .finalizer_output
                            .as_ref()
                            .map_or(std::ptr::null(), |s| s.as_ptr());
                    }
                    true
                }
                Ok(Err(error)) => {
                    state.error.get_or_insert(error);
                    false
                }
                Err(_) => {
                    state.error.get_or_insert_with(|| {
                        OcioError::InvalidInput("shader finalizer panicked".into())
                    });
                    false
                }
            }
        }
        unsafe extern "C" fn code_callback<F, G>(
            context: *mut c_void,
            section: i32,
            code: *const i8,
            replacement: *mut *const i8,
        ) -> bool
        where
            F: FnMut(ShaderSections<'_>) -> Result<Option<String>>,
            G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
        {
            let state = unsafe { &mut *context.cast::<State<'_, F, G>>() };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || -> Result<Option<std::ffi::CString>> {
                    let section = match section {
                        0 => ShaderCodeSection::Declarations,
                        1 => ShaderCodeSection::Parameters,
                        2 => ShaderCodeSection::Textures,
                        3 => ShaderCodeSection::Helpers,
                        4 => ShaderCodeSection::Header,
                        5 => ShaderCodeSection::Body,
                        6 => ShaderCodeSection::Footer,
                        _ => {
                            return Err(OcioError::InvalidInput(
                                "invalid shader code section".into(),
                            ))
                        }
                    };
                    if code.is_null() || replacement.is_null() {
                        return Err(OcioError::InvalidInput("null shader code event".into()));
                    }
                    let code = unsafe { std::ffi::CStr::from_ptr(code) }
                        .to_str()
                        .map_err(|_| OcioError::InvalidInput("shader code is not UTF-8".into()))?;
                    (state.code)(section, code)?
                        .map(std::ffi::CString::new)
                        .transpose()
                        .map_err(|_| OcioError::InteriorNul)
                },
            ));
            match result {
                Ok(Ok(value)) => {
                    state.code_output = value;
                    unsafe {
                        *replacement = state
                            .code_output
                            .as_ref()
                            .map_or(std::ptr::null(), |s| s.as_ptr());
                    }
                    true
                }
                Ok(Err(error)) => {
                    state.error.get_or_insert(error);
                    false
                }
                Err(_) => {
                    state.error.get_or_insert_with(|| {
                        OcioError::InvalidInput("shader code callback panicked".into())
                    });
                    false
                }
            }
        }
        unsafe extern "C" fn lifecycle_callback<F, G>(
            context: *mut c_void,
            event: i32,
            uid: *const i8,
        ) -> bool
        where
            F: FnMut(ShaderSections<'_>) -> Result<Option<String>>,
            G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
        {
            let state = unsafe { &mut *context.cast::<State<'_, F, G>>() };
            let result =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<()> {
                    let event = match event {
                        0 if !uid.is_null() => ShaderLifecycle::Begin {
                            unique_id: unsafe { std::ffi::CStr::from_ptr(uid) }.to_str().map_err(
                                |_| {
                                    OcioError::InvalidInput("shader identifier is not UTF-8".into())
                                },
                            )?,
                        },
                        1 => ShaderLifecycle::Finalized,
                        2 => ShaderLifecycle::End,
                        _ => {
                            return Err(OcioError::InvalidInput(
                                "invalid shader lifecycle event".into(),
                            ))
                        }
                    };
                    (state.lifecycle)(event)
                }));
            match result {
                Ok(Ok(())) => true,
                Ok(Err(error)) => {
                    state.error.get_or_insert(error);
                    false
                }
                Err(_) => {
                    state.error.get_or_insert_with(|| {
                        OcioError::InvalidInput("shader lifecycle callback panicked".into())
                    });
                    false
                }
            }
        }
        unsafe extern "C" fn uniform_callback<F, G>(
            context: *mut c_void,
            name: *const i8,
            uniform_type: i32,
            index: u32,
            has_max_size: bool,
            max_size: u32,
            has_buffer_offset: bool,
            buffer_offset: usize,
        ) -> bool
        where
            F: FnMut(ShaderSections<'_>) -> Result<Option<String>>,
            G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
        {
            let state = unsafe { &mut *context.cast::<State<'_, F, G>>() };
            let result =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<()> {
                    if name.is_null() || !(0..=4).contains(&uniform_type) {
                        return Err(OcioError::InvalidInput("invalid uniform event".into()));
                    }
                    let name = unsafe { std::ffi::CStr::from_ptr(name) }
                        .to_str()
                        .map_err(|_| OcioError::InvalidInput("uniform name is not UTF-8".into()))?;
                    (state.uniform)(ShaderUniform {
                        name,
                        uniform_type: GpuUniformType::from_raw(uniform_type),
                        index,
                        max_size: has_max_size.then_some(max_size),
                        buffer_offset: has_buffer_offset.then_some(buffer_offset),
                    })
                }));
            match result {
                Ok(Ok(())) => true,
                Ok(Err(error)) => {
                    state.error.get_or_insert(error);
                    false
                }
                Err(_) => {
                    state.error.get_or_insert(OcioError::InvalidInput(
                        "shader uniform callback panicked".into(),
                    ));
                    false
                }
            }
        }
        unsafe extern "C" fn callback<F, G>(
            context: *mut c_void,
            p: *const i8,
            t: *const i8,
            h: *const i8,
            head: *const i8,
            body: *const i8,
            foot: *const i8,
            output_ptr: *mut *const i8,
        ) -> bool
        where
            F: FnMut(ShaderSections<'_>) -> Result<Option<String>>,
            G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
        {
            let state = unsafe { &mut *context.cast::<State<'_, F, G>>() };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || -> Result<Option<std::ffi::CString>> {
                    let read = |ptr: *const i8| -> Result<&str> {
                        if ptr.is_null() {
                            return Err(OcioError::InvalidInput("null shader section".into()));
                        }
                        unsafe { std::ffi::CStr::from_ptr(ptr) }
                            .to_str()
                            .map_err(|_| {
                                OcioError::InvalidInput("shader section is not UTF-8".into())
                            })
                    };
                    let text = (state.function)(ShaderSections {
                        parameters: read(p)?,
                        textures: read(t)?,
                        helpers: read(h)?,
                        header: read(head)?,
                        body: read(body)?,
                        footer: read(foot)?,
                    })?;
                    text.map(|text| {
                        std::ffi::CString::new(text).map_err(|_| {
                            OcioError::InvalidInput("assembled shader contains a NUL byte".into())
                        })
                    })
                    .transpose()
                },
            ));
            match result {
                Ok(Ok(output)) => {
                    state.output = output;
                    unsafe {
                        *output_ptr = state
                            .output
                            .as_ref()
                            .map_or(std::ptr::null(), |text| text.as_ptr())
                    };
                    true
                }
                Ok(Err(error)) => {
                    state.error.get_or_insert(error);
                    false
                }
                Err(_) => {
                    state
                        .error
                        .get_or_insert(OcioError::InvalidInput("shader assembler panicked".into()));
                    false
                }
            }
        }
        unsafe extern "C" fn texture_callback<F, G>(
            context: *mut c_void,
            event: *const ocio_sys::OcioShaderTextureEvent,
            replace: *mut bool,
            binding: *mut u32,
        ) -> bool
        where
            F: FnMut(ShaderSections<'_>) -> Result<Option<String>>,
            G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
        {
            let state = unsafe { &mut *context.cast::<State<'_, F, G>>() };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || -> Result<Option<u32>> {
                    let event = unsafe { event.as_ref() }
                        .ok_or_else(|| OcioError::InvalidInput("null texture event".into()))?;
                    let read = |ptr: *const i8| -> Result<&str> {
                        if ptr.is_null() {
                            return Err(OcioError::InvalidInput("null texture name".into()));
                        }
                        unsafe { std::ffi::CStr::from_ptr(ptr) }
                            .to_str()
                            .map_err(|_| {
                                OcioError::InvalidInput("texture name is not UTF-8".into())
                            })
                    };
                    if event.values.is_null()
                        || event.value_count > isize::MAX as usize / std::mem::size_of::<f32>()
                    {
                        return Err(OcioError::InvalidInput("invalid texture data".into()));
                    }
                    let dimensions = match event.dimensions {
                        1 => Some(GpuTextureDimensions::Texture1D),
                        2 => Some(GpuTextureDimensions::Texture2D),
                        3 => None,
                        _ => {
                            return Err(OcioError::InvalidInput(
                                "unknown texture dimensions".into(),
                            ))
                        }
                    };
                    let channel = match event.channel {
                        0 => GpuTextureChannel::Red,
                        1 => GpuTextureChannel::Rgb,
                        _ => return Err(OcioError::InvalidInput("unknown texture channel".into())),
                    };
                    // SAFETY: OCIO owns the valid native float array during this
                    // synchronous callback; its checked length fits a Rust slice.
                    let values =
                        unsafe { std::slice::from_raw_parts(event.values, event.value_count) };
                    (state.texture)(ShaderTexture {
                        texture_name: read(event.texture_name)?,
                        sampler_name: read(event.sampler_name)?,
                        width: event.width,
                        height: event.height,
                        depth: event.depth,
                        channel,
                        dimensions,
                        interpolation: interpolation_from_raw(event.interpolation),
                        values,
                        suggested_binding: event.has_binding.then_some(event.binding),
                    })
                },
            ));
            match result {
                Ok(Ok(value)) => {
                    if let Some(value) = value {
                        unsafe {
                            *replace = true;
                            *binding = value;
                        }
                    }
                    true
                }
                Ok(Err(error)) => {
                    state.error.get_or_insert(error);
                    false
                }
                Err(_) => {
                    state.error.get_or_insert(OcioError::InvalidInput(
                        "shader texture callback panicked".into(),
                    ));
                    false
                }
            }
        }
        unsafe extern "C" fn texture_control_callback<F, G>(
            context: *mut c_void,
            event: *const ocio_sys::OcioShaderTextureEvent,
            store: *mut bool,
            replace: *mut bool,
            binding: *mut u32,
        ) -> bool
        where
            F: FnMut(ShaderSections<'_>) -> Result<Option<String>>,
            G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
        {
            if store.is_null() {
                return false;
            }
            if !unsafe { texture_callback::<F, G>(context, event, replace, binding) } {
                return false;
            }
            let state = unsafe { &*context.cast::<State<'_, F, G>>() };
            if let Some(storage) = state.texture_storage {
                unsafe { *store = storage.get() };
                true
            } else {
                false
            }
        }
        unsafe extern "C" fn registration_callback<F, G>(
            context: *mut c_void,
            name: *const i8,
            uniform_type: i32,
            has_max_size: bool,
            max_size: u32,
            getter: *mut c_void,
            action: *mut i32,
        ) -> bool
        where
            F: FnMut(ShaderSections<'_>) -> Result<Option<String>>,
            G: FnMut(ShaderTexture<'_>) -> Result<Option<u32>>,
        {
            let state = unsafe { &mut *context.cast::<State<'_, F, G>>() };
            let Some(handle) = NonNull::new(getter) else {
                state
                    .error
                    .get_or_insert_with(|| OcioError::InvalidInput("null uniform getter".into()));
                return false;
            };
            // SAFETY: Ownership of this native cache is transferred by the
            // bridge, which retained the native creator and its properties.
            let getter = ShaderUniformGetter {
                cache: GpuUniformCache {
                    handle,
                    _desc: std::marker::PhantomData,
                },
                uniform_type: GpuUniformType::from_raw(uniform_type),
            };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || -> Result<ShaderUniformAction> {
                    if name.is_null() || action.is_null() || !(0..=4).contains(&uniform_type) {
                        return Err(OcioError::InvalidInput(
                            "invalid uniform registration".into(),
                        ));
                    }
                    let name = unsafe { std::ffi::CStr::from_ptr(name) }
                        .to_str()
                        .map_err(|_| OcioError::InvalidInput("uniform name is not UTF-8".into()))?;
                    (state.registration)(ShaderUniformRegistration {
                        name: name.to_owned(),
                        uniform_type: GpuUniformType::from_raw(uniform_type),
                        max_size: has_max_size.then_some(max_size),
                        getter,
                    })
                },
            ));
            match result {
                Ok(Ok(choice)) => {
                    unsafe {
                        *action = match choice {
                            ShaderUniformAction::StoreInDescriptor => 0,
                            ShaderUniformAction::ExternalAdded => 1,
                            ShaderUniformAction::ExternalDuplicate => 2,
                        }
                    };
                    true
                }
                Ok(Err(error)) => {
                    state.error.get_or_insert(error);
                    false
                }
                Err(_) => {
                    state.error.get_or_insert_with(|| {
                        OcioError::InvalidInput("uniform registration callback panicked".into())
                    });
                    false
                }
            }
        }
        let has_finalizer = finalizer.is_some();
        let has_policy = policy.is_some();
        let mut state = State {
            function: &mut assemble,
            texture: &mut texture,
            texture_storage,
            uniform,
            lifecycle,
            code,
            registration,
            finalizer: finalizer
                .as_mut()
                .map(|f| &mut **f as &mut dyn FnMut() -> Result<Option<String>>),
            policy: policy
                .as_mut()
                .map(|p| &mut **p as &mut dyn FnMut(i32, u32) -> Result<u32>),
            output: None,
            code_output: None,
            finalizer_output: None,
            error: None,
        };
        crate::clear_last_error();
        let ptr = unsafe {
            let context = (&mut state as *mut State<'_, F, G>).cast();
            let extract = if stream_textures {
                ocio_sys::ocio_gpu_processor_extract_with_streamed_texture_hooks
                    as unsafe extern "C" fn(_, _, _, _, _, _, _, _) -> _
            } else {
                ocio_sys::ocio_gpu_processor_extract_with_hooks
            };
            if has_policy {
                debug_assert!(has_finalizer && texture_storage.is_some());
                ocio_sys::ocio_gpu_processor_extract_with_texture_policy(
                    self.handle.as_ptr(),
                    template.handle.as_ptr(),
                    callback::<F, G>,
                    texture_control_callback::<F, G>,
                    notify_uniform.then_some(uniform_callback::<F, G>),
                    notify_lifecycle.then_some(lifecycle_callback::<F, G>),
                    notify_code.then_some(code_callback::<F, G>),
                    has_registration.then_some(registration_callback::<F, G>),
                    finalize_callback::<F, G>,
                    policy_callback::<F, G>,
                    cache_id_override.map_or(std::ptr::null(), |id| id.as_ptr()),
                    context,
                )
            } else if has_finalizer {
                ocio_sys::ocio_gpu_processor_extract_with_finalize_control(
                    self.handle.as_ptr(),
                    template.handle.as_ptr(),
                    callback::<F, G>,
                    texture_control_callback::<F, G>,
                    notify_uniform.then_some(uniform_callback::<F, G>),
                    notify_lifecycle.then_some(lifecycle_callback::<F, G>),
                    notify_code.then_some(code_callback::<F, G>),
                    has_registration.then_some(registration_callback::<F, G>),
                    finalize_callback::<F, G>,
                    cache_id_override.map_or(std::ptr::null(), |id| id.as_ptr()),
                    context,
                )
            } else if texture_storage.is_some() {
                ocio_sys::ocio_gpu_processor_extract_with_texture_control(
                    self.handle.as_ptr(),
                    template.handle.as_ptr(),
                    callback::<F, G>,
                    texture_control_callback::<F, G>,
                    notify_uniform.then_some(uniform_callback::<F, G>),
                    notify_lifecycle.then_some(lifecycle_callback::<F, G>),
                    notify_code.then_some(code_callback::<F, G>),
                    has_registration.then_some(registration_callback::<F, G>),
                    cache_id_override.map_or(std::ptr::null(), |id| id.as_ptr()),
                    context,
                )
            } else if let Some(cache_id) = cache_id_override {
                debug_assert!(has_registration);
                ocio_sys::ocio_gpu_processor_extract_with_creator_options(
                    self.handle.as_ptr(),
                    template.handle.as_ptr(),
                    callback::<F, G>,
                    notify_texture.then_some(texture_callback::<F, G>),
                    notify_uniform.then_some(uniform_callback::<F, G>),
                    notify_lifecycle.then_some(lifecycle_callback::<F, G>),
                    notify_code.then_some(code_callback::<F, G>),
                    Some(registration_callback::<F, G>),
                    stream_textures,
                    cache_id.as_ptr(),
                    context,
                )
            } else if has_registration {
                ocio_sys::ocio_gpu_processor_extract_with_uniform_control(
                    self.handle.as_ptr(),
                    template.handle.as_ptr(),
                    callback::<F, G>,
                    notify_texture.then_some(texture_callback::<F, G>),
                    notify_uniform.then_some(uniform_callback::<F, G>),
                    notify_lifecycle.then_some(lifecycle_callback::<F, G>),
                    notify_code.then_some(code_callback::<F, G>),
                    Some(registration_callback::<F, G>),
                    stream_textures,
                    context,
                )
            } else {
                extract(
                    self.handle.as_ptr(),
                    template.handle.as_ptr(),
                    callback::<F, G>,
                    notify_texture.then_some(texture_callback::<F, G>),
                    notify_uniform.then_some(uniform_callback::<F, G>),
                    notify_lifecycle.then_some(lifecycle_callback::<F, G>),
                    notify_code.then_some(code_callback::<F, G>),
                    context,
                )
            }
        };
        let result = crate::handle_result(ptr);
        if let Some(error) = state.error {
            if let Ok(handle) = result {
                unsafe { ocio_sys::ocio_gpu_shader_desc_destroy(handle.as_ptr()) };
            }
            return Err(error);
        }
        Ok(GpuShaderDesc { handle: result? })
    }
    /// Return whether the GPU path is an identity/no-op transform.
    ///
    /// Returns `false` if OCIO reports an error. Use [`Self::try_is_no_op`] to
    /// distinguish that case from a non-identity processor.
    pub fn is_no_op(&self) -> bool {
        self.try_is_no_op().unwrap_or(false)
    }

    /// Fallible variant of [`Self::is_no_op`].
    ///
    /// Returns an error when the underlying OCIO query throws.
    pub fn try_is_no_op(&self) -> Result<bool> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_gpu_processor_is_no_op(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return whether the GPU path mixes color channels.
    ///
    /// Returns `false` if OCIO reports an error. Use
    /// [`Self::try_has_channel_crosstalk`] to distinguish that case from a
    /// lane-wise processor.
    pub fn has_channel_crosstalk(&self) -> bool {
        self.try_has_channel_crosstalk().unwrap_or(false)
    }

    /// Fallible variant of [`Self::has_channel_crosstalk`].
    ///
    /// Returns an error when the underlying OCIO query throws.
    pub fn try_has_channel_crosstalk(&self) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_gpu_processor_has_channel_crosstalk(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Return OCIO's cache identifier for this GPU processor instance.
    pub fn cache_id(&self) -> Option<String> {
        self.try_cache_id().ok().flatten()
    }

    /// Return OCIO's cache identifier, preserving bridge errors.
    pub fn try_cache_id(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_gpu_processor_get_cache_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Fill `shader_desc` with OCIO-generated shader text, uniforms, and textures.
    ///
    /// Panics when OCIO rejects extraction. Use [`Self::try_extract_shader_info`]
    /// when extraction failures should be handled by the caller.
    #[deprecated(
        since = "0.2.0",
        note = "panics on OCIO errors; prefer try_extract_shader_info()"
    )]
    pub fn extract_shader_info(&self, shader_desc: &mut GpuShaderDesc) {
        self.try_extract_shader_info(shader_desc)
            .expect("GPUProcessor::extract_shader_info failed");
    }

    /// Try to fill `shader_desc` with OCIO-generated shader text, uniforms, and textures.
    pub fn try_extract_shader_info(&self, shader_desc: &mut GpuShaderDesc) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_processor_extract_gpu_shader_info_v1(
                self.handle.as_ptr(),
                shader_desc.handle.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Probe optimized texture resources before extracting into `shader_desc`.
    ///
    /// OCIO 2.4–2.6 can underflow while packing some 1D LUT lengths into 2D
    /// textures. The probe uses a temporary descriptor wide enough for every
    /// 1D LUT accepted by these versions, streams its textures without storing
    /// them, and checks their lengths against the requested width before the
    /// real extraction. The target descriptor is untouched if probing fails.
    ///
    /// On OCIO 2.5+, it also checks the binding range needed by the probed
    /// textures and the target's existing textures before modifying the target.
    /// This assumes resource generation does not change between the probe and
    /// extraction; custom descriptors with noncontiguous bindings may be rejected
    /// conservatively. It does not validate arbitrary native creators.
    ///
    /// This performs shader generation twice and allocates OCIO's temporary
    /// texture data. It conservatively checks all 1D/2D resources with height
    /// one, including non-LUT resources, so it can reject a safe processor.
    /// It relies on the 1,048,576-entry limit of supported OCIO 2.4–2.6 and
    /// must be re-audited before supporting a newer native version.
    pub fn try_extract_shader_info_checked(&self, shader_desc: &mut GpuShaderDesc) -> Result<()> {
        let (lengths, texture_count) = self.probe_shader_texture_lengths(shader_desc)?;
        for length in lengths {
            shader_desc.validate_lut_texture_layout(length)?;
        }
        #[cfg(feature = "v2_5")]
        {
            let start = u64::from(shader_desc.try_texture_binding_start()?);
            let count = u64::from(shader_desc.try_num_textures()?)
                + u64::from(shader_desc.try_num_3d_textures()?)
                + texture_count;
            if count > 0 && start + count - 1 > u64::from(u32::MAX) {
                return Err(OcioError::InvalidInput(
                    "texture binding index overflow".into(),
                ));
            }
        }
        #[cfg(not(feature = "v2_5"))]
        let _ = texture_count;
        self.try_extract_shader_info(shader_desc)
    }

    /// Conservatively check the optimized processor's texture layouts before
    /// extraction with `template`. This performs a probe extraction, but does
    /// not invoke the caller's shader hooks or retain probe textures.
    ///
    /// The probe has the cost and version/false-positive limitations documented
    /// on [`Self::try_extract_shader_info_checked`]. It may be followed by any
    /// of this library's synchronous custom extraction APIs using the same
    /// processor and unchanged template settings. It does not validate arbitrary
    /// native creators with independently implemented texture-policy methods.
    ///
    /// ```no_run
    /// # use ocio_rs::{GPUProcessor, GpuShaderDesc, ShaderExtractionHooks, Result};
    /// # fn extract<H: ShaderExtractionHooks>(gpu: &GPUProcessor, template: &GpuShaderDesc, hooks: &mut H) -> Result<()> {
    /// gpu.validate_shader_texture_layout(template)?;
    /// let shader = gpu.try_extract_with_hooks(template, hooks)?;
    /// # let _ = shader;
    /// # Ok(()) }
    /// ```
    pub fn validate_shader_texture_layout(&self, shader_desc: &GpuShaderDesc) -> Result<()> {
        let (lengths, _) = self.probe_shader_texture_lengths(shader_desc)?;
        for length in lengths {
            shader_desc.validate_lut_texture_layout(length)?;
        }
        Ok(())
    }

    fn probe_shader_texture_lengths(
        &self,
        shader_desc: &GpuShaderDesc,
    ) -> Result<(Vec<usize>, u64)> {
        struct ProbeHooks;
        impl ShaderExtractionHooks for ProbeHooks {}
        let probe = shader_desc.try_clone_desc()?;
        probe.set_texture_max_width(1_048_577)?;
        probe.set_allow_texture_1d(shader_desc.try_allow_texture_1d()?)?;
        let mut lengths = Vec::new();
        let mut texture_count = 0u64;
        self.try_extract_with_streamed_textures(&probe, &mut ProbeHooks, |texture| {
            texture_count += 1;
            if texture.dimensions.is_some() && texture.height == 1 && texture.width >= 2 {
                lengths.push(texture.width as usize);
            }
            Ok(None)
        })?;
        Ok((lengths, texture_count))
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer try_extract_shader_info()"
    )]
    pub fn extract_gpu_shader_info(&self, shader_desc: &mut GpuShaderDesc) {
        self.try_extract_shader_info(shader_desc)
            .expect("GPUProcessor::extract_gpu_shader_info failed");
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "compat alias; prefer try_extract_shader_info()"
    )]
    pub fn extract_gpu_shader_info_v1(&self, shader_desc: &mut GpuShaderDesc) {
        self.try_extract_shader_info(shader_desc)
            .expect("GPUProcessor::extract_gpu_shader_info_v1 failed");
    }

    /// # Safety
    /// `shader_creator` must point to a valid mutable OCIO shader creator object
    /// for the active ABI and remain alive and exclusively accessible for the
    /// duration of the call. OCIO receives a temporary non-owning shared pointer;
    /// the object and its callbacks must not retain that shared pointer or depend
    /// on it sharing ownership with another control block. Neither this method
    /// nor the Rust binding destroys the object. Prefer
    /// [`Self::try_extract_shader_info`] with [`GpuShaderDesc`] for a typed,
    /// Rust-owned descriptor.
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.0",
        note = "raw OCIO shader-creator entry point; prefer extract_shader_info with GpuShaderDesc for Rust callers"
    )]
    pub unsafe fn extract_gpu_shader_info_v2(&self, shader_creator: *mut c_void) {
        let _ = unsafe { self.try_extract_shader_info_raw(shader_creator) };
    }

    /// Extract shader information through a raw `OCIO::GpuShaderCreator*`.
    ///
    /// # Safety
    /// `shader_creator` must meet the same ABI, exclusivity, lifetime, and ownership
    /// requirements as [`Self::extract_gpu_shader_info_v2`].
    #[doc(hidden)]
    pub unsafe fn try_extract_shader_info_raw(&self, shader_creator: *mut c_void) -> Result<()> {
        if shader_creator.is_null() {
            return Err(OcioError::InvalidInput(
                "native shader creator cannot be null".into(),
            ));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_processor_extract_gpu_shader_info_v2(
                self.handle.as_ptr(),
                shader_creator,
            );
        }
        crate::ocio_call_status()
    }
}

impl Drop for GPUProcessor {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_gpu_processor_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

// --- GpuShaderDesc ---

/// Collects parameters and emitted source for GPU shader extraction.
///
/// A `GpuShaderDesc` describes how OCIO should emit shader code for a given
/// target language. Before extraction, configure the descriptor with a
/// [`GpuLanguage`], entry-point name, pixel variable name, resource prefix,
/// and optional descriptor-set binding offsets.
///
/// After calling [`GPUProcessor::extract_shader_info`], the descriptor
/// contains the generated shader text (via [`shader_text`](Self::shader_text))
/// and any textures ([`textures_2d`](Self::textures_2d)) or uniforms
/// ([`uniforms`](Self::uniforms)) needed by the shader.
///
/// For manual shader assembly, the section-based helpers
/// ([`add_to_function_shader_code`](Self::add_to_function_shader_code),
/// `add_to_parameter_declare_shader_code` (`v2_5` feature), etc.) and
/// [`create_shader_text`](Self::create_shader_text) let you build the final
/// shader string from OCIO-provided fragments.
///
/// This handle is thread-confined; its dynamic property handles may outlive it.
///
/// ```compile_fail
/// # use ocio_rs::GpuShaderDesc;
/// fn require_send<T: Send>() {}
/// require_send::<GpuShaderDesc>();
/// ```
///
/// [`GpuLanguage`]: crate::GpuLanguage
/// [`GPUProcessor::extract_shader_info`]: crate::GPUProcessor::extract_shader_info
pub struct GpuShaderDesc {
    handle: NonNull<c_void>,
}

/// Result of copying a cached live uniform into caller-owned storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuUniformCopy {
    Written(usize),
    BufferTooSmall { required: usize },
    WrongType,
}

/// Cached OCIO uniform getters for repeated sampling without bridge allocations.
///
/// The descriptor remains exclusively borrowed because native getters may
/// reference properties owned by it. Values remain live; only the getters,
/// not their results, are cached. This type cannot be sent to another thread.
/// Getter implementations may themselves allocate or perform other work.
pub struct GpuUniformCache<'a> {
    handle: NonNull<c_void>,
    _desc: std::marker::PhantomData<&'a mut GpuShaderDesc>,
}

impl GpuUniformCache<'_> {
    fn copy_result(status: i32, actual: usize) -> Result<GpuUniformCopy> {
        crate::ocio_call_status()?;
        match status {
            0 => Ok(GpuUniformCopy::WrongType),
            1 => Ok(GpuUniformCopy::Written(actual)),
            2 => Ok(GpuUniformCopy::BufferTooSmall { required: actual }),
            _ => Err(OcioError::Ocio("uniform cache operation failed".into())),
        }
    }

    /// Copies a double, boolean, float3, or float array as `f32` values.
    /// An undersized buffer is left untouched.
    pub fn copy_f32_into(&mut self, index: u32, values: &mut [f32]) -> Result<GpuUniformCopy> {
        let mut actual = 0;
        crate::clear_last_error();
        let status = unsafe {
            ocio_sys::ocio_gpu_uniform_cache_copy_f32(
                self.handle.as_ptr(),
                index,
                values.as_mut_ptr(),
                values.len(),
                &mut actual,
            )
        };
        Self::copy_result(status, actual)
    }

    /// Copies an integer array. An undersized buffer is left untouched.
    pub fn copy_i32_into(&mut self, index: u32, values: &mut [i32]) -> Result<GpuUniformCopy> {
        let mut actual = 0;
        crate::clear_last_error();
        let status = unsafe {
            ocio_sys::ocio_gpu_uniform_cache_copy_i32(
                self.handle.as_ptr(),
                index,
                values.as_mut_ptr(),
                values.len(),
                &mut actual,
            )
        };
        Self::copy_result(status, actual)
    }

    /// Reads a double without narrowing to `f32`; returns `None` for other types.
    pub fn double(&mut self, index: u32) -> Result<Option<f64>> {
        let mut value = 0.0;
        crate::clear_last_error();
        let status = unsafe {
            ocio_sys::ocio_gpu_uniform_cache_double(self.handle.as_ptr(), index, &mut value)
        };
        crate::ocio_call_status()?;
        match status {
            0 => Ok(None),
            1 => Ok(Some(value)),
            _ => Err(OcioError::Ocio("uniform cache operation failed".into())),
        }
    }
}

impl Drop for GpuUniformCache<'_> {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_gpu_uniform_cache_destroy(self.handle.as_ptr()) }
    }
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
    Texture1D = 1,
    Texture2D = 2,
}

impl GpuTextureDimensions {
    fn from_raw(value: i32) -> Self {
        match value {
            2 => Self::Texture2D,
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
    /// API-facing binding slot reported by OCIO 2.5.1 and newer.
    ///
    /// OpenColorIO 2.4 does not expose binding indexes.
    pub binding_index: Option<u32>,
    /// Flattened texel payload in row-major order.
    pub values: Vec<f32>,
}

impl GpuTexture2D {
    /// Returns the number of `f32` values, saturating at `usize::MAX` on overflow.
    pub fn expected_value_count(&self) -> usize {
        self.checked_expected_value_count().unwrap_or(usize::MAX)
    }

    /// Returns the number of `f32` values, or `None` if it overflows.
    pub fn checked_expected_value_count(&self) -> Option<usize> {
        (self.width as usize)
            .checked_mul(self.height as usize)?
            .checked_mul(self.channel.channel_count())
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
    /// API-facing binding slot reported by OCIO 2.5.1 and newer.
    ///
    /// OpenColorIO 2.4 does not expose binding indexes.
    pub binding_index: Option<u32>,
    /// Flattened texel payload in OCIO's native 3D LUT ordering.
    pub values: Vec<f32>,
}

impl GpuTexture3D {
    /// Returns the number of `f32` values, saturating at `usize::MAX` on overflow.
    pub fn expected_value_count(&self) -> usize {
        self.checked_expected_value_count().unwrap_or(usize::MAX)
    }

    /// Returns the number of `f32` values, or `None` if it overflows.
    pub fn checked_expected_value_count(&self) -> Option<usize> {
        let edge = self.edge_len as usize;
        edge.checked_mul(edge)?.checked_mul(edge)?.checked_mul(3)
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
    /// Byte offset into the packed uniform buffer layout on OpenColorIO 2.5.1
    /// and newer.
    pub buffer_offset: Option<usize>,
    /// Logical scalar count for the current uniform payload.
    pub value_count: usize,
    /// Typed uniform payload copied into Rust-owned memory.
    pub value: GpuUniformValue,
}

impl GpuShaderDesc {
    /// Captures the current uniform getters for repeated reads without bridge allocations.
    ///
    /// Extract shader information before calling this method. Drop the cache
    /// before changing the descriptor or extracting shader information again.
    ///
    /// ```compile_fail
    /// # use ocio_rs::GpuShaderDesc;
    /// # fn example(mut desc: GpuShaderDesc) -> ocio_rs::Result<()> {
    /// let mut cache = desc.uniform_cache()?;
    /// desc.set_function_name("other")?;
    /// let _ = cache.double(0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn uniform_cache(&mut self) -> Result<GpuUniformCache<'_>> {
        crate::clear_last_error();
        let ptr = unsafe { ocio_sys::ocio_gpu_uniform_cache_create(self.handle.as_ptr()) };
        crate::handle_result(ptr).map(|handle| GpuUniformCache {
            handle,
            _desc: std::marker::PhantomData,
        })
    }
    /// Borrow this descriptor as a native `OCIO::GpuShaderCreator*` for C++ interop.
    ///
    /// The pointer is invalid after this descriptor is dropped. Native code
    /// must not delete it, retain it beyond that lifetime, or access the object
    /// concurrently or reentrantly with Rust operations on this descriptor.
    /// The returned raw pointer is only usable with a compatible OCIO ABI.
    pub fn borrow_native_creator(&mut self) -> Result<NonNull<c_void>> {
        crate::clear_last_error();
        let ptr =
            unsafe { ocio_sys::ocio_gpu_shader_desc_borrow_native_creator(self.handle.as_ptr()) };
        crate::handle_result(ptr)
    }

    /// Creates an empty OCIO GPU shader descriptor that can be configured and
    /// passed to `GPUProcessor::extract_shader_info`.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_gpu_shader_desc_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    fn bool_call_result(ok: bool) -> Result<bool> {
        if ok {
            Ok(true)
        } else {
            match crate::ocio_call_status() {
                Ok(()) => Ok(false),
                Err(err) => Err(err),
            }
        }
    }

    /// Returns the extracted shader source text, if any.
    pub fn shader_text(&self) -> Option<String> {
        self.try_shader_text().ok().flatten()
    }

    /// Return the extracted shader source text, if any.
    ///
    /// Unlike [`Self::shader_text`], this preserves OCIO query failures as
    /// [`OcioError`].
    pub fn try_shader_text(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let shader_text = unsafe {
            cstr_from_mut(ocio_sys::ocio_gpu_shader_desc_get_shader_text(
                self.handle.as_ptr() as *mut c_void,
            ))
        };
        crate::ocio_call_status()?;
        Ok(shader_text)
    }

    /// Borrow OCIO's shader text without allocating or converting its bytes.
    ///
    /// The exclusive borrow prevents mutation or re-extraction into this
    /// descriptor while the returned native string is in use. Raw native users
    /// must also avoid concurrent or reentrant mutation of the descriptor.
    /// The empty string is returned as `Some`; stub builds return `None`.
    ///
    /// ```compile_fail
    /// # use ocio_rs::GpuShaderDesc;
    /// # fn example(desc: &mut GpuShaderDesc) {
    /// let text = desc.shader_text_ref().unwrap().unwrap();
    /// desc.set_function_name("changed").unwrap();
    /// println!("{:?}", text);
    /// # }
    /// ```
    pub fn shader_text_ref(&mut self) -> Result<Option<&std::ffi::CStr>> {
        crate::clear_last_error();
        let ptr = unsafe { ocio_sys::ocio_gpu_shader_desc_get_shader_text(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        if ptr.is_null() {
            Ok(None)
        } else {
            // SAFETY: OCIO owns a NUL-terminated string until the descriptor is
            // mutated or dropped; the exclusive borrow prevents both in safe Rust.
            Ok(Some(unsafe { std::ffi::CStr::from_ptr(ptr.cast()) }))
        }
    }

    /// Returns the number of reported 1D/2D texture resources.
    pub fn num_textures(&self) -> u32 {
        self.try_num_textures().unwrap_or(0)
    }

    /// Return the number of reported 1D/2D texture resources.
    pub fn try_num_textures(&self) -> Result<u32> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_gpu_shader_desc_get_num_textures_u32(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
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
        self.try_texture_2d(index).ok().flatten()
    }

    /// Return a structured 1D/2D texture resource, preserving bridge failures.
    ///
    /// An out-of-range `index` is returned as an OCIO error. `Ok(None)` means
    /// OCIO did not report a usable texture resource without raising an error.
    pub fn try_texture_2d(&self, index: u32) -> Result<Option<GpuTexture2D>> {
        let mut info = ocio_sys::OcioGpuTexture2DInfo {
            texture_name: std::ptr::null(),
            sampler_name: std::ptr::null(),
            width: 0,
            height: 0,
            channel: 0,
            dimensions: 0,
            interpolation: 0,
            has_binding_index: false,
            binding_index: 0,
        };
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_texture_info(self.handle.as_ptr(), index, &mut info)
        };
        crate::ocio_call_status()?;
        if !ok {
            return Ok(None);
        }
        let channel = GpuTextureChannel::from_raw(info.channel);
        let value_count = (info.width as usize)
            .checked_mul(info.height as usize)
            .and_then(|count| count.checked_mul(channel.channel_count()))
            .ok_or_else(|| {
                OcioError::ValidationFailed("GPU texture payload size overflowed usize".into())
            })?;
        let mut values = vec![0.0f32; value_count];
        crate::clear_last_error();
        let values_ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_copy_texture_values(
                self.handle.as_ptr(),
                index,
                values.as_mut_ptr(),
                values.len(),
            )
        };
        crate::ocio_call_status()?;
        if !values_ok {
            return Err(OcioError::ValidationFailed(
                "OCIO could not copy the reported GPU texture payload".into(),
            ));
        }
        let Some(texture_name) = required_ocio_string(info.texture_name) else {
            return Ok(None);
        };
        let Some(sampler_name) = required_ocio_string(info.sampler_name) else {
            return Ok(None);
        };
        Ok(Some(GpuTexture2D {
            texture_name,
            sampler_name,
            width: info.width,
            height: info.height,
            channel,
            dimensions: GpuTextureDimensions::from_raw(info.dimensions),
            interpolation: interpolation_from_raw(info.interpolation),
            binding_index: info.has_binding_index.then_some(info.binding_index),
            values,
        }))
    }

    /// Returns all structured 1D/2D texture resources currently reported by OCIO.
    pub fn textures_2d(&self) -> Vec<GpuTexture2D> {
        (0..self.num_textures())
            .filter_map(|index| self.texture_2d(index))
            .collect()
    }

    fn gpu_language_from_raw(l: i32) -> GpuLanguage {
        match l {
            0 => GpuLanguage::Cg,
            1 => GpuLanguage::Glsl1_2,
            2 => GpuLanguage::Glsl1_3,
            3 => GpuLanguage::Glsl4_0,
            #[cfg(feature = "v2_5")]
            4 => GpuLanguage::GlslVk4_6,
            5 => GpuLanguage::HlslSm5_0,
            6 => GpuLanguage::Osl1,
            7 => GpuLanguage::GlslEs1_0,
            8 => GpuLanguage::GlslEs3_0,
            9 => GpuLanguage::Msl2_0,
            _ => GpuLanguage::Glsl1_2,
        }
    }

    /// Returns the shader language currently configured on the descriptor.
    pub fn language(&self) -> GpuLanguage {
        self.try_language().unwrap_or(GpuLanguage::Glsl1_2)
    }

    /// Return the configured shader language, preserving bridge failures.
    pub fn try_language(&self) -> Result<GpuLanguage> {
        crate::clear_last_error();
        let language = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_language(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        Ok(Self::gpu_language_from_raw(language))
    }

    /// Sets the shader language OCIO should target during extraction.
    pub fn set_language(&self, language: GpuLanguage) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_language(self.handle.as_ptr(), language as i32);
        }
        crate::ocio_call_status()
    }

    /// Returns the configured shader entry-point name, if any.
    pub fn function_name(&self) -> Option<String> {
        self.try_function_name().ok().flatten()
    }

    /// Return the configured shader entry-point name, preserving bridge errors.
    pub fn try_function_name(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_function_name(
                self.handle.as_ptr() as *mut c_void,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Sets the shader entry-point name used during extraction.
    pub fn set_function_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_function_name(
                self.handle.as_ptr(),
                n.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    /// Returns the configured pixel variable name, if any.
    pub fn pixel_name(&self) -> Option<String> {
        self.try_pixel_name().ok().flatten()
    }

    /// Return the configured pixel variable name, preserving bridge errors.
    pub fn try_pixel_name(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_pixel_name(
                self.handle.as_ptr() as *mut c_void,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Sets the pixel variable name used in emitted shader code.
    pub fn set_pixel_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_pixel_name(self.handle.as_ptr(), n.as_ptr().cast());
        }
        crate::ocio_call_status()
    }

    /// Returns the explicit unique identifier configured for shader extraction, if any.
    pub fn unique_id(&self) -> Option<String> {
        self.try_unique_id().ok().flatten()
    }

    /// Return the explicit unique identifier, preserving bridge errors.
    pub fn try_unique_id(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_unique_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Sets the unique identifier OCIO should use for generated shader resources.
    pub fn set_unique_id(&self, uid: impl AsRef<str>) -> Result<()> {
        let uid = cstring(uid)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_unique_id(self.handle.as_ptr(), uid.as_ptr().cast());
        }
        crate::ocio_call_status()
    }

    /// Returns the configured resource-name prefix, if any.
    pub fn resource_prefix(&self) -> Option<String> {
        self.try_resource_prefix().ok().flatten()
    }

    /// Return the configured resource-name prefix, preserving bridge errors.
    pub fn try_resource_prefix(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_resource_prefix(
                self.handle.as_ptr() as *mut c_void,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Sets the prefix OCIO uses for generated resource names.
    pub fn set_resource_prefix(&self, prefix: impl AsRef<str>) -> Result<()> {
        let p = cstring(prefix)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_resource_prefix(
                self.handle.as_ptr(),
                p.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    /// Configures the descriptor-set index and starting texture-binding slot used by OCIO.
    #[deprecated(
        since = "0.2.0",
        note = "panics on OCIO errors; prefer try_set_descriptor_set_index()"
    )]
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn set_descriptor_set_index(&self, index: u32, texture_binding_start: u32) {
        self.try_set_descriptor_set_index(index, texture_binding_start)
            .expect("GpuShaderDesc::set_descriptor_set_index failed");
    }

    /// Configures the descriptor-set index and starting texture-binding slot used by OCIO.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
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
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn descriptor_set_index(&self) -> u32 {
        self.try_descriptor_set_index().unwrap_or(0)
    }

    /// Return the configured descriptor-set index, preserving bridge failures.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn try_descriptor_set_index(&self) -> Result<u32> {
        crate::clear_last_error();
        let index = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_descriptor_set_index(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(index)
    }

    /// Returns the configured starting binding slot for extracted textures.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn texture_binding_start(&self) -> u32 {
        self.try_texture_binding_start().unwrap_or(0)
    }

    /// Return the starting texture binding slot, preserving bridge failures.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn try_texture_binding_start(&self) -> Result<u32> {
        crate::clear_last_error();
        let index = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_texture_binding_start(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(index)
    }

    /// Sets the maximum width OCIO may use when laying out extracted 1D textures.
    ///
    /// Values below 2 are rejected: native OCIO uses the width as a divisor,
    /// and uses width minus one as a loop increment. For widths of 2 or more,
    /// affected OCIO versions can still miscalculate padding for some LUT
    /// lengths; choose a width at least as large as the LUT when possible.
    pub fn set_texture_max_width(&self, max_width: u32) -> Result<()> {
        if max_width < 2 {
            return Err(OcioError::InvalidInput(
                "texture maximum width must be at least 2".into(),
            ));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_texture_max_width_u32(
                self.handle.as_ptr(),
                max_width,
            );
        }
        crate::ocio_call_status()
    }

    /// Check whether a known 1D LUT length fits OCIO's packed texture layout.
    ///
    /// OCIO 2.4–2.6 may underflow its padding count for some combinations of
    /// LUT length and maximum width. This check detects those combinations
    /// before extraction. It only checks the supplied length: optimization or
    /// other operations can introduce LUTs with different lengths, so success
    /// does not establish that an arbitrary GPU processor is safe to extract.
    /// A caller must validate every LUT length that reaches native extraction.
    pub fn validate_lut_texture_layout(&self, length: usize) -> Result<()> {
        Self::validate_lut_texture_width(self.try_texture_max_width()?, length)
    }

    fn validate_lut_texture_width(max_width: u32, length: usize) -> Result<()> {
        if length < 2 {
            return Err(OcioError::InvalidInput(
                "LUT length must be at least 2".into(),
            ));
        }
        let maximum = u128::from(max_width);
        if maximum < 2 {
            return Err(OcioError::InvalidInput(
                "texture maximum width must be at least 2".into(),
            ));
        }
        let length = length as u128;
        let width = length.min(maximum);
        let height = length / maximum + 1;
        // OCIO uses unsigned long for dimensions and RGB indexing, which is
        // only 32 bits on Windows even in a 64-bit process.
        let native_limit = std::os::raw::c_ulong::MAX as u128;
        if length > native_limit / 3 || width * height > native_limit / 3 {
            return Err(OcioError::InvalidInput(
                "LUT texture layout exceeds native arithmetic limits".into(),
            ));
        }
        let padded = if height > 1 {
            length + (length - 1) / (width - 1)
        } else {
            length
        };
        if padded > width * height {
            return Err(OcioError::InvalidInput(
                "OpenColorIO LUT texture padding exceeds its allocated layout".into(),
            ));
        }
        Ok(())
    }

    /// Controls whether OCIO may use native 1D textures instead of always promoting to 2D.
    pub fn set_allow_texture_1d(&self, allowed: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_allow_texture_1d(self.handle.as_ptr(), allowed);
        }
        crate::ocio_call_status()
    }

    /// Returns whether OCIO may use native 1D textures during extraction.
    pub fn allow_texture_1d(&self) -> bool {
        self.try_allow_texture_1d().unwrap_or(false)
    }

    /// Return whether OCIO may use native 1D textures, preserving bridge failures.
    pub fn try_allow_texture_1d(&self) -> Result<bool> {
        crate::clear_last_error();
        let allowed =
            unsafe { ocio_sys::ocio_gpu_shader_desc_get_allow_texture_1d(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(allowed)
    }

    /// Marks the beginning of shader-data collection with the provided OCIO resource UID.
    pub fn begin(&self, uid: impl AsRef<str>) -> Result<()> {
        let uid = cstring(uid)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_gpu_shader_desc_begin(self.handle.as_ptr(), uid.as_ptr()) };
        crate::ocio_call_status()
    }

    /// Marks the end of shader-data collection.
    pub fn end(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_gpu_shader_desc_end(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    /// Returns the next OCIO-managed resource index and advances the internal counter.
    pub fn next_resource_index(&self) -> u32 {
        self.try_next_resource_index().unwrap_or(0)
    }

    /// Return and advance the OCIO-managed resource index, preserving bridge failures.
    pub fn try_next_resource_index(&self) -> Result<u32> {
        crate::clear_last_error();
        let index =
            unsafe { ocio_sys::ocio_gpu_shader_desc_get_next_resource_index(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(index)
    }

    /// Append to OCIO 2.4's combined declaration section.
    ///
    /// On native OCIO 2.5 and later this uses the parameter declaration
    /// section for compatibility with non-Vulkan languages. Do not mix this
    /// method with the separate parameter/texture append methods, since the
    /// two native sections are assembled separately. Vulkan requires those
    /// separate methods and is rejected here.
    pub fn add_to_declare_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_declare_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Appends text to the shader's parameter-declaration section.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn add_to_parameter_declare_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_parameter_declare_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Appends text to the shader's texture-declaration section.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn add_to_texture_declare_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_texture_declare_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Appends text to the shader's helper-method section.
    pub fn add_to_helper_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_helper_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Appends text to the shader function's header section.
    pub fn add_to_function_header_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_function_header_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Appends text to the shader function's body section.
    pub fn add_to_function_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_function_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Appends text to the shader function's footer section.
    pub fn add_to_function_footer_shader_code(&self, shader_code: impl AsRef<str>) -> Result<()> {
        let shader_code = cstring(shader_code)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_to_function_footer_shader_code(
                self.handle.as_ptr(),
                shader_code.as_ptr(),
            );
        }
        crate::ocio_call_status()
    }

    /// Returns the copied texel payload for a 1D/2D texture resource.
    pub fn texture_values(&self, index: u32) -> Vec<f32> {
        self.texture_2d(index)
            .map(|texture| texture.values)
            .unwrap_or_default()
    }

    /// Borrow 1D/2D LUT values directly from OCIO without copying.
    ///
    /// The exclusive descriptor borrow prevents safe Rust code from mutating
    /// the descriptor while the returned slice is live. Native code using raw
    /// pointers must likewise avoid concurrent or reentrant mutation.
    ///
    /// ```compile_fail
    /// # use ocio_rs::GpuShaderDesc;
    /// # fn example(desc: &mut GpuShaderDesc) {
    /// let values = desc.texture_values_ref(0).unwrap();
    /// desc.set_function_name("changed").unwrap();
    /// println!("{}", values[0]);
    /// # }
    /// ```
    pub fn texture_values_ref(&mut self, index: u32) -> Result<&[f32]> {
        self.borrow_texture_values(index, false)
    }

    fn borrow_texture_values(&mut self, index: u32, is_3d: bool) -> Result<&[f32]> {
        let mut len = 0usize;
        crate::clear_last_error();
        let ptr = unsafe {
            ocio_sys::ocio_gpu_shader_desc_borrow_texture_values(
                self.handle.as_ptr(),
                index,
                is_3d,
                &mut len,
            )
        };
        crate::ocio_call_status()?;
        if ptr.is_null() || len == 0 {
            return Err(OcioError::ValidationFailed(
                "OCIO returned no GPU texture values".into(),
            ));
        }
        // SAFETY: The bridge validates the count and returns OCIO-owned f32
        // storage. The exclusive descriptor borrow keeps it alive and prevents
        // safe mutations for the lifetime of the returned slice.
        Ok(unsafe { std::slice::from_raw_parts(ptr, len) })
    }

    /// Adds a manual 1D/2D texture resource to the descriptor.
    ///
    /// The result contains the OCIO shader binding index on OpenColorIO 2.5.1
    /// and newer. OpenColorIO 2.4 performs the insertion but returns `None`
    /// because that release does not expose binding indexes.
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
    ) -> Result<Option<u32>> {
        let expected = (width as usize)
            .checked_mul(height as usize)
            .and_then(|count| count.checked_mul(channel.channel_count()))
            .ok_or_else(|| {
                OcioError::ValidationFailed("GPU texture payload size overflowed usize".into())
            })?;
        if values.len() != expected {
            return Err(OcioError::ValidationFailed(format!(
                "gpu texture value count mismatch: expected {expected}, got {}",
                values.len()
            )));
        }
        let texture_name = cstring(texture_name)?;
        let sampler_name = cstring(sampler_name)?;
        crate::clear_last_error();
        let mut has_binding_index = false;
        let mut binding_index = 0;
        let inserted = unsafe {
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
                &mut has_binding_index,
                &mut binding_index,
            )
        };
        crate::ocio_call_status()?;
        if !inserted {
            return Err(OcioError::ValidationFailed(
                "OpenColorIO rejected the 1D/2D texture".to_owned(),
            ));
        }
        Ok(has_binding_index.then_some(binding_index))
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
        crate::clear_last_error();
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
        crate::ocio_call_status()
    }

    /// Finalizes descriptor configuration before extraction when OCIO requires it.
    pub fn finalize(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_finalize(self.handle.as_ptr() as *mut c_void);
        }
        crate::ocio_call_status()
    }

    /// Returns the maximum width OCIO would like to use for GPU textures.
    pub fn texture_max_width(&self) -> u32 {
        self.try_texture_max_width().unwrap_or(0)
    }

    /// Return the maximum OCIO GPU texture width, preserving bridge failures.
    pub fn try_texture_max_width(&self) -> Result<u32> {
        crate::clear_last_error();
        let width =
            unsafe { ocio_sys::ocio_gpu_shader_desc_get_texture_max_width(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(width)
    }

    /// Returns OCIO's cache identifier for the current descriptor configuration.
    pub fn cache_id(&self) -> Option<String> {
        self.try_cache_id().ok().flatten()
    }

    /// Return OCIO's descriptor cache identifier, preserving bridge errors.
    pub fn try_cache_id(&self) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_cache_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Borrow the native cache identifier without copying or UTF-8 conversion.
    ///
    /// OCIO invalidates this string when descriptor settings change. The
    /// exclusive borrow prevents safe Rust from mutating the descriptor or
    /// replacing its custom ID while the returned string is in use. Native
    /// aliases must not mutate it concurrently. Stub builds return `None`.
    ///
    /// ```compile_fail
    /// # use ocio_rs::GpuShaderDesc;
    /// # fn example(desc: &mut GpuShaderDesc) {
    /// let id = desc.cache_id_ref().unwrap().unwrap();
    /// desc.set_custom_cache_id(Some(c"new-id")).unwrap();
    /// println!("{:?}", id);
    /// # }
    /// ```
    pub fn cache_id_ref(&mut self) -> Result<Option<&std::ffi::CStr>> {
        crate::clear_last_error();
        let ptr = unsafe { ocio_sys::ocio_gpu_shader_desc_get_cache_id(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        if ptr.is_null() {
            Ok(None)
        } else {
            // SAFETY: the descriptor owns this NUL-terminated string and the
            // exclusive borrow prevents mutation or destruction in safe Rust.
            Ok(Some(unsafe { std::ffi::CStr::from_ptr(ptr) }))
        }
    }

    /// Change or clear the cache ID override on a descriptor created by a
    /// custom extraction method. Clearing restores OCIO's native cache ID.
    ///
    /// This does not rename resources already generated during extraction;
    /// applications must manage any resource keys and external caches as well.
    /// Generic descriptors, including native clones, do not support this override.
    pub fn set_custom_cache_id(&mut self, id: Option<&std::ffi::CStr>) -> Result<()> {
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_set_custom_cache_id(
                self.handle.as_ptr(),
                id.map_or(std::ptr::null(), std::ffi::CStr::as_ptr),
            )
        };
        crate::ocio_call_status()?;
        if !ok {
            return Err(OcioError::Ocio(
                "Could not set custom shader cache ID".into(),
            ));
        }
        Ok(())
    }

    /// Returns the OCIO texture UID associated with a 1D/2D resource, if any.
    pub fn texture_uid(&self, index: i32) -> Option<String> {
        self.try_texture_uid(index).ok().flatten()
    }

    /// Return a texture UID, preserving bridge errors.
    ///
    /// `Ok(None)` means no texture exists at `index`.
    pub fn try_texture_uid(&self, index: i32) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_gpu_shader_desc_get_texture_uid(
                self.handle.as_ptr(),
                index,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    // ── v2.5.1 ──
    /// Clone the descriptor configuration.
    ///
    /// In real OCIO builds this preserves descriptor settings such as language,
    /// function name, pixel name, resource prefix, and descriptor-set settings.
    /// Lower-level creator implementation knobs such as 1D texture width and
    /// `allow_texture_1d` may fall back to OCIO defaults on clone, and
    /// extracted shader payloads are not guaranteed to be copied into the clone.
    /// A custom cache ID from [`ShaderCreatorOptions`] is not preserved.
    pub fn clone_desc(&self) -> Option<GpuShaderDesc> {
        self.try_clone_desc().ok()
    }

    /// Clone descriptor configuration and preserve native errors.
    /// See [`Self::clone_desc`] for which native settings are copied.
    pub fn try_clone_desc(&self) -> Result<GpuShaderDesc> {
        crate::clear_last_error();
        let h =
            unsafe { ocio_sys::ocio_gpu_shader_desc_clone(self.handle.as_ptr() as *mut c_void) };
        Ok(GpuShaderDesc {
            handle: crate::handle_result(h)?,
        })
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer clone_desc()")]
    #[allow(clippy::should_implement_trait)]
    pub fn clone(&self) -> Option<GpuShaderDesc> {
        self.clone_desc()
    }

    /// Adds an existing dynamic property to this shader creator's native store.
    ///
    /// OCIO retains shared ownership; changes through either handle affect the
    /// same property, even after the original owner is dropped. Only one
    /// property of a given type can be added; duplicates return an OCIO error.
    /// This does not itself add a uniform or shader code. Like the descriptor,
    /// the property is thread-confined; native aliases must not be accessed
    /// concurrently or reentrantly.
    pub fn add_dynamic_property(&mut self, property: &DynamicProperty) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_dynamic_property(
                self.handle.as_ptr(),
                property.handle.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    /// Registers a live double uniform backed by a property already in this
    /// descriptor. Supports exposure, contrast, and gamma properties.
    ///
    /// The getter owns the native property reference and does not invoke Rust
    /// when sampled. Returns `false` if the uniform name already exists.
    /// This does not emit a shader declaration; custom creators must emit it.
    pub fn add_uniform_dynamic_double(
        &mut self,
        name: impl AsRef<str>,
        property_type: DynamicPropertyType,
    ) -> Result<bool> {
        if !matches!(
            property_type,
            DynamicPropertyType::Exposure
                | DynamicPropertyType::Contrast
                | DynamicPropertyType::Gamma
        ) {
            return Err(OcioError::InvalidInput(
                "a double dynamic property is required".into(),
            ));
        }
        let name = cstring(name)?;
        crate::clear_last_error();
        let added = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_dynamic_double(
                self.handle.as_ptr(),
                name.as_ptr(),
                property_type as i32,
            )
        };
        Self::bool_call_result(added)
    }

    /// Returns the number of reported uniforms.
    pub fn num_uniforms(&self) -> u32 {
        self.try_num_uniforms().unwrap_or(0)
    }

    /// Return the number of reported uniforms.
    pub fn try_num_uniforms(&self) -> Result<u32> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_gpu_shader_desc_get_num_uniforms_u32(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer num_uniforms()")]
    pub fn get_num_uniforms_u32(&self) -> u32 {
        self.num_uniforms()
    }

    /// Returns the size in bytes of OCIO's packed uniform buffer layout.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn uniform_buffer_size(&self) -> usize {
        self.try_uniform_buffer_size().unwrap_or(0)
    }

    /// Return the packed uniform-buffer size, preserving bridge errors.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn try_uniform_buffer_size(&self) -> Result<usize> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_uniform_buffer_size_bytes(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer uniform_buffer_size()")]
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn get_uniform_buffer_size_bytes(&self) -> usize {
        self.uniform_buffer_size()
    }

    /// Adds a scalar floating-point uniform and returns `false` when the name already exists.
    pub fn add_uniform_f64(&self, name: impl AsRef<str>, value: f64) -> Result<bool> {
        let name = cstring(name)?;
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_double(
                self.handle.as_ptr(),
                name.as_ptr(),
                value,
            )
        };
        Self::bool_call_result(ok)
    }

    /// Adds a live double uniform backed by a thread-safe Rust getter.
    ///
    /// OCIO owns the getter until it no longer needs the uniform. It may invoke
    /// the getter multiple times and from native code; do not reenter the same
    /// descriptor from the getter. A panic is contained and reported as an
    /// OCIO error when the uniform is sampled. Returns `false` for a duplicate
    /// uniform name, in which case the supplied getter is dropped immediately.
    pub fn add_uniform_f64_getter<F>(&self, name: impl AsRef<str>, getter: F) -> Result<bool>
    where
        F: Fn() -> f64 + Send + Sync + 'static,
    {
        unsafe extern "C" fn invoke<F: Fn() -> f64>(context: *mut c_void, out: *mut f64) -> bool {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let getter = unsafe { &*context.cast::<F>() };
                unsafe { *out = getter() };
            })) {
                Ok(()) => true,
                Err(_) => false,
            }
        }
        unsafe extern "C" fn release<F>(context: *mut c_void) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                drop(Box::from_raw(context.cast::<F>()));
            }));
        }
        let name = cstring(name)?;
        let context = Box::into_raw(Box::new(getter)).cast();
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_double_getter(
                self.handle.as_ptr(),
                name.as_ptr(),
                context,
                invoke::<F>,
                release::<F>,
            )
        };
        Self::bool_call_result(ok)
    }

    /// Adds a boolean uniform and returns `false` when the name already exists.
    pub fn add_uniform_bool(&self, name: impl AsRef<str>, value: bool) -> Result<bool> {
        let name = cstring(name)?;
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_bool(
                self.handle.as_ptr(),
                name.as_ptr(),
                value,
            )
        };
        Self::bool_call_result(ok)
    }

    /// Adds a live boolean uniform; see [`Self::add_uniform_f64_getter`] for
    /// callback ownership, thread-safety, and panic behavior.
    pub fn add_uniform_bool_getter<F>(&self, name: impl AsRef<str>, getter: F) -> Result<bool>
    where
        F: Fn() -> bool + Send + Sync + 'static,
    {
        unsafe extern "C" fn invoke<F: Fn() -> bool>(context: *mut c_void, out: *mut bool) -> bool {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let getter = unsafe { &*context.cast::<F>() };
                unsafe { *out = getter() };
            }))
            .is_ok()
        }
        unsafe extern "C" fn release<F>(context: *mut c_void) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                drop(Box::from_raw(context.cast::<F>()));
            }));
        }
        let name = cstring(name)?;
        let context = Box::into_raw(Box::new(getter)).cast();
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_bool_getter(
                self.handle.as_ptr(),
                name.as_ptr(),
                context,
                invoke::<F>,
                release::<F>,
            )
        };
        Self::bool_call_result(ok)
    }

    /// Adds a three-component floating-point uniform and returns `false` when the name already exists.
    pub fn add_uniform_float3(&self, name: impl AsRef<str>, value: [f32; 3]) -> Result<bool> {
        let name = cstring(name)?;
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_float3(
                self.handle.as_ptr(),
                name.as_ptr(),
                value[0],
                value[1],
                value[2],
            )
        };
        Self::bool_call_result(ok)
    }

    /// Adds a live three-component uniform; see [`Self::add_uniform_f64_getter`]
    /// for callback ownership and panic behavior. Native callers must not sample
    /// the same descriptor concurrently or retain a returned native reference
    /// across another sample of this uniform.
    pub fn add_uniform_float3_getter<F>(&self, name: impl AsRef<str>, getter: F) -> Result<bool>
    where
        F: Fn() -> [f32; 3] + Send + Sync + 'static,
    {
        unsafe extern "C" fn invoke<F: Fn() -> [f32; 3]>(
            context: *mut c_void,
            out: *mut f32,
        ) -> bool {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let getter = unsafe { &*context.cast::<F>() };
                let value = getter();
                unsafe { std::ptr::copy_nonoverlapping(value.as_ptr(), out, 3) };
            }))
            .is_ok()
        }
        unsafe extern "C" fn release<F>(context: *mut c_void) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                drop(Box::from_raw(context.cast::<F>()));
            }));
        }
        let name = cstring(name)?;
        let context = Box::into_raw(Box::new(getter)).cast();
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_float3_getter(
                self.handle.as_ptr(),
                name.as_ptr(),
                context,
                invoke::<F>,
                release::<F>,
            )
        };
        Self::bool_call_result(ok)
    }

    /// Adds a floating-point array uniform and returns `false` when the name already exists.
    pub fn add_uniform_f32_array(
        &self,
        name: impl AsRef<str>,
        values: &[f32],
        max_size: u32,
    ) -> Result<bool> {
        if values.len() > i32::MAX as usize || values.len() > max_size as usize {
            return Err(OcioError::ValidationFailed(format!(
                "gpu uniform value count {} exceeds max_size {} or the native integer limit",
                values.len(),
                max_size
            )));
        }
        let name = cstring(name)?;
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_vector_float(
                self.handle.as_ptr(),
                name.as_ptr(),
                values.as_ptr(),
                values.len(),
                max_size,
            )
        };
        Self::bool_call_result(ok)
    }

    /// Adds a live float array uniform. The getter fills the provided buffer and
    /// returns the initialized length, which must not exceed `max_size`.
    ///
    /// Storage is allocated once at registration. Each native size query
    /// refreshes the snapshot; its pointer getter reads that same snapshot.
    /// Native callers must serialize descriptor access and consume the pointer
    /// before the next size query. Panics and excessive lengths are errors.
    pub fn add_uniform_f32_array_getter<F>(
        &self,
        name: impl AsRef<str>,
        max_size: u32,
        getter: F,
    ) -> Result<bool>
    where
        F: Fn(&mut [f32]) -> usize + Send + Sync + 'static,
    {
        unsafe extern "C" fn invoke<F: Fn(&mut [f32]) -> usize>(
            context: *mut c_void,
            data: *mut f32,
            capacity: usize,
            length: *mut usize,
        ) -> bool {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let getter = unsafe { &*context.cast::<F>() };
                let data = if capacity == 0 {
                    std::ptr::NonNull::<f32>::dangling().as_ptr()
                } else {
                    data
                };
                let buffer = unsafe { std::slice::from_raw_parts_mut(data, capacity) };
                unsafe { *length = getter(buffer) };
            }))
            .is_ok()
        }
        unsafe extern "C" fn release<F>(context: *mut c_void) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                drop(Box::from_raw(context.cast::<F>()));
            }));
        }
        if max_size > i32::MAX as u32
            || max_size as usize > isize::MAX as usize / std::mem::size_of::<f32>()
        {
            return Err(OcioError::ValidationFailed(
                "uniform max_size exceeds the native integer limit".into(),
            ));
        }
        let name = cstring(name)?;
        let context = Box::into_raw(Box::new(getter)).cast();
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_vector_float_getter(
                self.handle.as_ptr(),
                name.as_ptr(),
                max_size,
                context,
                invoke::<F>,
                release::<F>,
            )
        };
        Self::bool_call_result(ok)
    }

    /// Adds an integer array uniform and returns `false` when the name already exists.
    pub fn add_uniform_i32_array(
        &self,
        name: impl AsRef<str>,
        values: &[i32],
        max_size: u32,
    ) -> Result<bool> {
        if values.len() > i32::MAX as usize || values.len() > max_size as usize {
            return Err(OcioError::ValidationFailed(format!(
                "gpu uniform value count {} exceeds max_size {} or the native integer limit",
                values.len(),
                max_size
            )));
        }
        let name = cstring(name)?;
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_vector_int(
                self.handle.as_ptr(),
                name.as_ptr(),
                values.as_ptr(),
                values.len(),
                max_size,
            )
        };
        Self::bool_call_result(ok)
    }

    /// Adds a live integer array uniform with the same snapshot and lifetime
    /// rules as [`Self::add_uniform_f32_array_getter`].
    pub fn add_uniform_i32_array_getter<F>(
        &self,
        name: impl AsRef<str>,
        max_size: u32,
        getter: F,
    ) -> Result<bool>
    where
        F: Fn(&mut [i32]) -> usize + Send + Sync + 'static,
    {
        unsafe extern "C" fn invoke<F: Fn(&mut [i32]) -> usize>(
            context: *mut c_void,
            data: *mut i32,
            capacity: usize,
            length: *mut usize,
        ) -> bool {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let getter = unsafe { &*context.cast::<F>() };
                let data = if capacity == 0 {
                    std::ptr::NonNull::<i32>::dangling().as_ptr()
                } else {
                    data
                };
                let buffer = unsafe { std::slice::from_raw_parts_mut(data, capacity) };
                unsafe { *length = getter(buffer) };
            }))
            .is_ok()
        }
        unsafe extern "C" fn release<F>(context: *mut c_void) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                drop(Box::from_raw(context.cast::<F>()));
            }));
        }
        if max_size > i32::MAX as u32
            || max_size as usize > isize::MAX as usize / std::mem::size_of::<i32>()
        {
            return Err(OcioError::ValidationFailed(
                "uniform max_size exceeds the native integer limit".into(),
            ));
        }
        let name = cstring(name)?;
        let context = Box::into_raw(Box::new(getter)).cast();
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add_uniform_vector_int_getter(
                self.handle.as_ptr(),
                name.as_ptr(),
                max_size,
                context,
                invoke::<F>,
                release::<F>,
            )
        };
        Self::bool_call_result(ok)
    }

    /// Returns the number of dynamic properties attached to the descriptor.
    pub fn num_dynamic_properties(&self) -> u32 {
        self.try_num_dynamic_properties().unwrap_or(0)
    }

    /// Return the number of dynamic properties attached to the descriptor.
    pub fn try_num_dynamic_properties(&self) -> Result<u32> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_num_dynamic_properties_u32(self.handle.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Returns a dynamic property by ordinal index, if present.
    pub fn dynamic_property_by_index(&self, index: u32) -> Option<DynamicProperty> {
        self.try_dynamic_property_by_index(index).ok().flatten()
    }

    /// Return a dynamic property by ordinal index, preserving bridge failures.
    ///
    /// An out-of-range `index` is reported as an OCIO error. The legacy
    /// [`Self::dynamic_property_by_index`] helper maps that error to `None`.
    pub fn try_dynamic_property_by_index(&self, index: u32) -> Result<Option<DynamicProperty>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_dynamic_property_by_index(
                self.handle.as_ptr(),
                index,
            )
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| DynamicProperty { handle }))
    }

    /// Returns the dynamic property associated with the given OCIO property kind, if present.
    pub fn dynamic_property(&self, property_type: DynamicPropertyType) -> Option<DynamicProperty> {
        self.try_dynamic_property(property_type).ok().flatten()
    }

    /// Return the dynamic property for an OCIO property kind, preserving bridge failures.
    ///
    /// `Ok(None)` means the descriptor has no property of `property_type`.
    pub fn try_dynamic_property(
        &self,
        property_type: DynamicPropertyType,
    ) -> Result<Option<DynamicProperty>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_dynamic_property(
                self.handle.as_ptr(),
                property_type as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(NonNull::new(handle).map(|handle| DynamicProperty { handle }))
    }

    /// Returns whether the descriptor exposes the given OCIO dynamic property kind.
    pub fn has_dynamic_property_kind(&self, prop_type: DynamicPropertyType) -> bool {
        self.try_has_dynamic_property_kind(prop_type)
            .unwrap_or(false)
    }

    /// Fallible variant of [`Self::has_dynamic_property_kind`].
    pub fn try_has_dynamic_property_kind(&self, prop_type: DynamicPropertyType) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_gpu_shader_desc_has_dynamic_property(
                self.handle.as_ptr(),
                prop_type as i32,
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Returns a structured uniform record with Rust-owned payload values.
    pub fn uniform(&self, index: u32) -> Option<GpuUniform> {
        self.try_uniform(index).ok().flatten()
    }

    /// Return a structured uniform record, preserving bridge failures.
    ///
    /// An out-of-range `index` is returned as an OCIO error. `Ok(None)` means
    /// OCIO did not report a usable uniform without raising an error. A uniform
    /// with a payload that OCIO cannot expose through this ABI is returned with
    /// [`GpuUniformValue::Unsupported`].
    /// Dynamic arrays are sampled with their actual copied length. If an array
    /// repeatedly outgrows the buffer during sampling, an error is returned.
    pub fn try_uniform(&self, index: u32) -> Result<Option<GpuUniform>> {
        let mut info = ocio_sys::OcioGpuUniformInfo {
            name: std::ptr::null(),
            type_: 5,
            has_buffer_offset: false,
            buffer_offset: 0,
            value_count: 0,
        };
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_uniform_info(self.handle.as_ptr(), index, &mut info)
        };
        crate::ocio_call_status()?;
        if !ok {
            return Ok(None);
        }
        let uniform_type = GpuUniformType::from_raw(info.type_);
        let value = match uniform_type {
            GpuUniformType::VectorInt => {
                let mut values = vec![0i32; info.value_count];
                let mut copied = false;
                for attempt in 0..3 {
                    let mut actual = 0;
                    crate::clear_last_error();
                    let ok = unsafe {
                        ocio_sys::ocio_gpu_shader_desc_copy_uniform_i32_values_with_count(
                            self.handle.as_ptr(),
                            index,
                            values.as_mut_ptr(),
                            values.len(),
                            &mut actual,
                        )
                    };
                    crate::ocio_call_status()?;
                    if ok {
                        values.truncate(actual);
                        info.value_count = actual;
                        copied = true;
                        break;
                    }
                    if actual <= values.len() {
                        break;
                    }
                    if attempt == 2 {
                        return Err(OcioError::ValidationFailed(
                            "uniform array kept growing while sampled".into(),
                        ));
                    }
                    values.resize(actual, 0);
                }
                if copied {
                    GpuUniformValue::I32(values)
                } else {
                    GpuUniformValue::Unsupported
                }
            }
            GpuUniformType::Unknown => GpuUniformValue::Unsupported,
            _ => {
                let mut values = vec![0.0f32; info.value_count];
                let mut copied = false;
                for attempt in 0..3 {
                    let mut actual = 0;
                    crate::clear_last_error();
                    let ok = unsafe {
                        ocio_sys::ocio_gpu_shader_desc_copy_uniform_f32_values_with_count(
                            self.handle.as_ptr(),
                            index,
                            values.as_mut_ptr(),
                            values.len(),
                            &mut actual,
                        )
                    };
                    crate::ocio_call_status()?;
                    if ok {
                        values.truncate(actual);
                        info.value_count = actual;
                        copied = true;
                        break;
                    }
                    if actual <= values.len() {
                        break;
                    }
                    if attempt == 2 {
                        return Err(OcioError::ValidationFailed(
                            "uniform array kept growing while sampled".into(),
                        ));
                    }
                    values.resize(actual, 0.0);
                }
                if copied {
                    GpuUniformValue::F32(values)
                } else {
                    GpuUniformValue::Unsupported
                }
            }
        };
        let Some(name) = required_ocio_string(info.name) else {
            return Ok(None);
        };
        Ok(Some(GpuUniform {
            name,
            uniform_type,
            buffer_offset: info.has_buffer_offset.then_some(info.buffer_offset),
            value_count: info.value_count,
            value,
        }))
    }

    #[doc(hidden)]
    #[deprecated(since = "0.2.0", note = "compat alias; prefer uniform()")]
    pub fn get_uniform_info(&self, index: u32) -> Option<GpuUniform> {
        self.uniform(index)
    }

    /// Returns the scalar value count for a uniform, or `0` when absent.
    pub fn uniform_value_count(&self, index: u32) -> usize {
        self.try_uniform_value_count(index).unwrap_or(0)
    }

    /// Return the current scalar count without allocating or copying the payload.
    /// An invalid index or a native error is returned as an error.
    pub fn try_uniform_value_count(&self, index: u32) -> Result<usize> {
        crate::clear_last_error();
        let count = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_uniform_value_count(self.handle.as_ptr(), index)
        };
        crate::ocio_call_status()?;
        Ok(count)
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

    /// Reads the current native double uniform value without narrowing to `f32`.
    /// Returns `Ok(None)` for a uniform of another type; invalid indices and
    /// failures from the native or Rust getter are returned as errors.
    pub fn try_uniform_f64_value(&self, index: u32) -> Result<Option<f64>> {
        let mut value = 0.0;
        crate::clear_last_error();
        let found = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_uniform_double_value(
                self.handle.as_ptr(),
                index,
                &mut value,
            )
        };
        crate::ocio_call_status()?;
        Ok(found.then_some(value))
    }

    /// Read a native boolean uniform without converting it to a float.
    /// Returns `Ok(None)` for another uniform type; invalid indices and getter
    /// failures are returned as errors. `Some(false)` is a valid value.
    pub fn try_uniform_bool_value(&self, index: u32) -> Result<Option<bool>> {
        let mut value = false;
        crate::clear_last_error();
        let found = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get_uniform_bool_value(
                self.handle.as_ptr(),
                index,
                &mut value,
            )
        };
        crate::ocio_call_status()?;
        Ok(found.then_some(value))
    }

    /// Read the current floating-point uniform payload into caller-owned storage.
    ///
    /// Returns `Ok(false)` if the uniform is not representable as floats or
    /// `values` is too short. Double uniforms are converted to `f32`, and bool
    /// uniforms are represented as 0.0 or 1.0, matching OCIO GPU conventions.
    /// An invalid index or native failure is returned as an error. No Rust
    /// allocation is performed; the native getter is called on every invocation
    /// so updates to dynamic properties are observed.
    pub fn try_copy_uniform_f32_into(&self, index: u32, values: &mut [f32]) -> Result<bool> {
        crate::clear_last_error();
        // SAFETY: The bridge writes at most values.len() floats into the slice.
        let copied = unsafe {
            ocio_sys::ocio_gpu_shader_desc_copy_uniform_f32_values(
                self.handle.as_ptr(),
                index,
                values.as_mut_ptr(),
                values.len(),
            )
        };
        crate::ocio_call_status()?;
        Ok(copied)
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

    /// Read an integer-vector uniform into caller-owned storage without a Rust
    /// allocation. Returns `Ok(false)` for a different type or a short buffer;
    /// invalid indices and native failures are returned as errors.
    pub fn try_copy_uniform_i32_into(&self, index: u32, values: &mut [i32]) -> Result<bool> {
        crate::clear_last_error();
        // SAFETY: The bridge writes at most values.len() integers into the slice.
        let copied = unsafe {
            ocio_sys::ocio_gpu_shader_desc_copy_uniform_i32_values(
                self.handle.as_ptr(),
                index,
                values.as_mut_ptr(),
                values.len(),
            )
        };
        crate::ocio_call_status()?;
        Ok(copied)
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
        self.try_num_3d_textures().unwrap_or(0)
    }

    /// Return the number of reported 3D texture resources.
    pub fn try_num_3d_textures(&self) -> Result<u32> {
        crate::clear_last_error();
        let value =
            unsafe { ocio_sys::ocio_gpu_shader_desc_get_num3d_textures_u32(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(value)
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
        self.try_texture_3d(index).ok().flatten()
    }

    /// Return a structured 3D texture resource, preserving bridge failures.
    ///
    /// `Ok(None)` means no 3D texture exists at `index`.
    pub fn try_texture_3d(&self, index: u32) -> Result<Option<GpuTexture3D>> {
        let mut info = ocio_sys::OcioGpuTexture3DInfo {
            texture_name: std::ptr::null(),
            sampler_name: std::ptr::null(),
            edge_len: 0,
            interpolation: 0,
            has_binding_index: false,
            binding_index: 0,
        };
        crate::clear_last_error();
        let ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_get3d_texture_info(
                self.handle.as_ptr(),
                index,
                &mut info,
            )
        };
        crate::ocio_call_status()?;
        if !ok {
            return Ok(None);
        }
        let edge = info.edge_len as usize;
        let value_count = edge
            .checked_mul(edge)
            .and_then(|count| count.checked_mul(edge))
            .and_then(|count| count.checked_mul(3))
            .ok_or_else(|| {
                OcioError::ValidationFailed("GPU 3D texture payload size overflowed usize".into())
            })?;
        let mut values = vec![0.0f32; value_count];
        crate::clear_last_error();
        let values_ok = unsafe {
            ocio_sys::ocio_gpu_shader_desc_copy3d_texture_values(
                self.handle.as_ptr(),
                index,
                values.as_mut_ptr(),
                values.len(),
            )
        };
        crate::ocio_call_status()?;
        if !values_ok {
            return Err(OcioError::ValidationFailed(
                "OCIO could not copy the reported GPU 3D texture payload".into(),
            ));
        }
        let Some(texture_name) = required_ocio_string(info.texture_name) else {
            return Ok(None);
        };
        let Some(sampler_name) = required_ocio_string(info.sampler_name) else {
            return Ok(None);
        };
        Ok(Some(GpuTexture3D {
            texture_name,
            sampler_name,
            edge_len: info.edge_len,
            interpolation: interpolation_from_raw(info.interpolation),
            binding_index: info.has_binding_index.then_some(info.binding_index),
            values,
        }))
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

    /// Borrow the native 3D LUT values without allocating or copying.
    /// The returned slice is tied to the exclusive descriptor borrow.
    pub fn texture_3d_values_ref(&mut self, index: u32) -> Result<&[f32]> {
        self.borrow_texture_values(index, true)
    }

    /// Adds a manual 3D texture resource to the descriptor.
    ///
    /// The result contains the OCIO shader binding index on OpenColorIO 2.5.1
    /// and newer. OpenColorIO 2.4 performs the insertion but returns `None`
    /// because that release does not expose binding indexes.
    pub fn add_texture_3d(
        &self,
        texture_name: impl AsRef<str>,
        sampler_name: impl AsRef<str>,
        edge_len: u32,
        interpolation: Interpolation,
        values: &[f32],
    ) -> Result<Option<u32>> {
        let edge = edge_len as usize;
        let expected = edge
            .checked_mul(edge)
            .and_then(|count| count.checked_mul(edge))
            .and_then(|count| count.checked_mul(3))
            .ok_or_else(|| {
                OcioError::ValidationFailed("GPU 3D texture payload size overflowed usize".into())
            })?;
        if values.len() != expected {
            return Err(OcioError::ValidationFailed(format!(
                "gpu 3d texture value count mismatch: expected {expected}, got {}",
                values.len()
            )));
        }
        let texture_name = cstring(texture_name)?;
        let sampler_name = cstring(sampler_name)?;
        crate::clear_last_error();
        let mut has_binding_index = false;
        let mut binding_index = 0;
        let inserted = unsafe {
            ocio_sys::ocio_gpu_shader_desc_add3d_texture(
                self.handle.as_ptr(),
                texture_name.as_ptr(),
                sampler_name.as_ptr(),
                edge_len,
                interpolation as i32,
                values.as_ptr(),
                values.len(),
                &mut has_binding_index,
                &mut binding_index,
            )
        };
        crate::ocio_call_status()?;
        if !inserted {
            return Err(OcioError::ValidationFailed(
                "OpenColorIO rejected the 3D texture".to_owned(),
            ));
        }
        Ok(has_binding_index.then_some(binding_index))
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
        self.texture_3d(index)
            .and_then(|texture| texture.binding_index)
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
        self.texture_2d(index)
            .and_then(|texture| texture.binding_index)
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
        254 => Interpolation::Default,
        255 => Interpolation::Best,
        _ => Interpolation::Unknown,
    }
}

// --- DynamicProperty ---

/// References a processor property that may be adjusted dynamically at runtime.
pub struct DynamicProperty {
    handle: NonNull<c_void>,
}

impl DynamicProperty {
    /// Retain the current RGB curve value without a deep copy.
    pub fn grading_rgb_curve_shared(&self) -> Result<crate::SharedGradingRGBCurve> {
        self.require_property_type(
            DynamicPropertyType::GradingRgbCurve,
            "DynamicProperty::grading_rgb_curve_shared",
        )?;
        crate::SharedGradingRGBCurve::from_source(self.handle.as_ptr(), 1)
    }
    /// Retain the current hue curve value without a deep copy.
    #[cfg(feature = "v2_5")]
    pub fn grading_hue_curve_shared(&self) -> Result<crate::SharedGradingHueCurve> {
        self.require_property_type(
            DynamicPropertyType::GradingHueCurve,
            "DynamicProperty::grading_hue_curve_shared",
        )?;
        crate::SharedGradingHueCurve::from_source(self.handle.as_ptr(), 3)
    }
    /// Return an independent native RGB curve value, including its non-point state.
    pub fn grading_rgb_curve_object(&self) -> Result<crate::GradingRGBCurve> {
        self.require_property_type(
            DynamicPropertyType::GradingRgbCurve,
            "DynamicProperty::grading_rgb_curve_object",
        )?;
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_object_get(self.handle.as_ptr())
        };
        Ok(crate::GradingRGBCurve {
            handle: crate::handle_result(value)?,
        })
    }

    /// Replace the native RGB curve value in one update.
    pub fn set_grading_rgb_curve_object(&self, value: &crate::GradingRGBCurve) -> Result<()> {
        self.require_property_type(
            DynamicPropertyType::GradingRgbCurve,
            "DynamicProperty::set_grading_rgb_curve_object",
        )?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_object_set(
                self.handle.as_ptr(),
                value.handle.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    #[cfg(feature = "v2_5")]
    /// Return an independent native HUE curve value, including its non-point state.
    pub fn grading_hue_curve_object(&self) -> Result<crate::GradingHueCurve> {
        self.require_property_type(
            DynamicPropertyType::GradingHueCurve,
            "DynamicProperty::grading_hue_curve_object",
        )?;
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_object_get(self.handle.as_ptr())
        };
        Ok(crate::GradingHueCurve {
            handle: crate::handle_result(value)?,
        })
    }

    #[cfg(feature = "v2_5")]
    /// Replace the native HUE curve value in one update.
    pub fn set_grading_hue_curve_object(&self, value: &crate::GradingHueCurve) -> Result<()> {
        self.require_property_type(
            DynamicPropertyType::GradingHueCurve,
            "DynamicProperty::set_grading_hue_curve_object",
        )?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_object_set(
                self.handle.as_ptr(),
                value.handle.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

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

    #[cfg(feature = "v2_5")]
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
            #[cfg(feature = "v2_5")]
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

    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
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

    /// Snapshot all eight curves of a dynamic hue-curve property.
    ///
    /// This is a control-plane operation: it copies the native curve data.
    #[cfg(feature = "v2_5")]
    pub fn complete_grading_hue_curve_value(
        &self,
    ) -> Result<crate::grading::CompleteGradingHueCurveValue> {
        use crate::grading::{
            CompleteGradingHueCurveValue, GradingCurvePoint, GradingHueCurveValue,
        };
        let read = |kind| -> Result<Vec<GradingCurvePoint>> {
            let count = self.grading_hue_curve_num_control_points(kind)?;
            (0..count)
                .map(|index| {
                    let (x, y) = self.grading_hue_curve_control_point(kind, index)?;
                    let slope = self.grading_hue_curve_slope(kind, index)?;
                    Ok(GradingCurvePoint { x, y, slope })
                })
                .collect()
        };
        Ok(CompleteGradingHueCurveValue {
            base: GradingHueCurveValue {
                hue_hue: read(HueCurveType::HueHue)?,
                hue_sat: read(HueCurveType::HueSat)?,
                hue_lum: read(HueCurveType::HueLum)?,
                lum_sat: read(HueCurveType::LumSat)?,
            },
            sat_sat: read(HueCurveType::SatSat)?,
            lum_lum: read(HueCurveType::LumLum)?,
            sat_lum: read(HueCurveType::SatLum)?,
            hue_fx: read(HueCurveType::HueFx)?,
        })
    }

    /// Validate and replace all eight curves in one native update.
    ///
    /// If curve validation fails, the original property is left unchanged. The curve data is
    /// borrowed across the FFI boundary; no Rust-side point copy is made.
    #[cfg(feature = "v2_5")]
    pub fn set_complete_grading_hue_curve_value(
        &self,
        value: &crate::grading::CompleteGradingHueCurveValue,
    ) -> Result<()> {
        self.require_grading_hue_curve_property(
            "DynamicProperty::set_complete_grading_hue_curve_value",
        )?;
        let curves = [
            &value.base.hue_hue,
            &value.base.hue_sat,
            &value.base.hue_lum,
            &value.base.lum_sat,
            &value.sat_sat,
            &value.lum_lum,
            &value.sat_lum,
            &value.hue_fx,
        ];
        let pointers = curves.map(|points| points.as_ptr().cast::<std::ffi::c_void>());
        let counts = curves.map(|points| points.len());
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_dynamic_property_grading_hue_curve_set_complete(
                self.handle.as_ptr(),
                pointers.as_ptr(),
                counts.as_ptr(),
            )
        };
        crate::ocio_call_status()
    }

    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
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

    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
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

    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
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

    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
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

    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
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

    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
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

    #[test]
    fn packed_layout_rejects_out_of_bounds_and_overflow() {
        let bytes = [0u8; 24];
        let layout = PackedImageLayout {
            width: 2,
            height: 2,
            origin: 0,
            channel_stride: 1,
            x_stride: 4,
            y_stride: 8,
            bit_depth: BitDepth::Uint8,
            channel_order: ChannelOrdering::Rgba,
        };
        assert!(packed_image_descriptor(layout, &bytes[..16]).is_ok());
        assert!(packed_image_descriptor(layout, &bytes[..15]).is_err());
        assert!(packed_image_descriptor(
            PackedImageLayout {
                x_stride: isize::MIN,
                ..layout
            },
            &bytes
        )
        .is_err());
        assert!(packed_image_descriptor(
            PackedImageLayout {
                channel_stride: usize::MAX,
                ..layout
            },
            &bytes
        )
        .is_err());
        assert!(packed_image_descriptor(
            PackedImageLayout {
                x_stride: -4,
                origin: 4,
                ..layout
            },
            &bytes
        )
        .is_err());
        assert!(packed_image_descriptor(
            PackedImageLayout {
                x_stride: -4,
                origin: 8,
                ..layout
            },
            &bytes
        )
        .is_ok());
    }
    use crate::Config;

    #[test]
    fn gpu_texture_dimension_discriminants_match_native_headers() {
        assert_eq!(GpuTextureDimensions::Texture1D as i32, 1);
        assert_eq!(GpuTextureDimensions::Texture2D as i32, 2);
        assert_eq!(
            GpuTextureDimensions::from_raw(1),
            GpuTextureDimensions::Texture1D
        );
        assert_eq!(
            GpuTextureDimensions::from_raw(2),
            GpuTextureDimensions::Texture2D
        );
    }

    #[test]
    fn gpu_texture_dimensions_reject_overflow_without_panicking() {
        let desc = GpuShaderDesc::create().unwrap();
        assert!(desc
            .add_texture_3d("texture", "sampler", 1 << 22, Interpolation::Linear, &[])
            .is_err());
        assert!(desc
            .add_texture_2d(
                "texture",
                "sampler",
                u32::MAX,
                u32::MAX,
                GpuTextureChannel::Rgb,
                GpuTextureDimensions::Texture2D,
                Interpolation::Linear,
                &[]
            )
            .is_err());
    }

    #[test]
    fn signed_planar_layout_checks_all_corners_and_pointer_steps() {
        let plane = [0u8; 8];
        for x in [-1isize, 1] {
            for y in [-3isize, 3] {
                let layout = SignedPlanarByteLayout {
                    width: 2,
                    height: 2,
                    origin: 1 + if x < 0 { 1 } else { 0 } + if y < 0 { 3 } else { 0 },
                    x_stride: x,
                    y_stride: y,
                    bit_depth: BitDepth::Uint8,
                };
                assert!(validate_signed_planar(layout, &[&plane]).is_ok());
                assert!(validate_signed_planar(layout, &[&plane[..4]]).is_err());
            }
        }
        let underflow = SignedPlanarByteLayout {
            width: 2,
            height: 1,
            origin: 1,
            x_stride: -1,
            y_stride: 2,
            bit_depth: BitDepth::Uint8,
        };
        assert!(validate_signed_planar(underflow, &[&plane]).is_err());
        assert!(validate_signed_planar(
            SignedPlanarByteLayout {
                x_stride: isize::MIN,
                ..underflow
            },
            &[&plane]
        )
        .is_err());
    }

    #[test]
    fn planar_layout_requires_native_row_stride_even_for_one_row() {
        let plane = [0u8; 3];
        let layout = PlanarByteLayout {
            width: 2,
            height: 1,
            x_stride: 2,
            y_stride: 3,
            bit_depth: BitDepth::Uint8,
        };
        assert!(validate_planar_buffer(layout, &[&plane]).is_err());
        assert!(validate_planar_buffer(
            PlanarByteLayout {
                y_stride: 4,
                ..layout
            },
            &[&plane]
        )
        .is_ok());
        assert!(validate_planar_buffer(
            PlanarByteLayout {
                width: 2,
                x_stride: usize::MAX,
                y_stride: usize::MAX,
                ..layout
            },
            &[&plane]
        )
        .is_err());
    }

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
            let _ = cpu.try_is_no_op();
            let _ = cpu.has_channel_crosstalk();
            let _ = cpu.try_has_channel_crosstalk();
            let _ = cpu.is_identity();
            let _ = cpu.try_is_identity();
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
            let _ = gpu.try_is_no_op();
            let _ = gpu.has_channel_crosstalk();
            let _ = gpu.try_has_channel_crosstalk();
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
            desc.set_language(GpuLanguage::Glsl1_2).unwrap();
            let _ = desc.set_function_name("main");
            let _ = desc.set_pixel_name("outColor");
            let _ = desc.set_resource_prefix("ocio_");
            desc.finalize().unwrap();
        }
    }

    #[test]
    fn gpu_extract_shader_info_named_wrapper_no_crash() {
        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let (Ok(gpu), Ok(mut desc)) = (proc.default_gpu_processor(), GpuShaderDesc::create()) {
            gpu.try_extract_shader_info(&mut desc).unwrap();
        }
    }

    #[test]
    fn gpu_shader_desc_texture_max_no_crash() {
        if let Ok(desc) = GpuShaderDesc::create() {
            let _ = desc.texture_max_width();
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

    #[cfg(feature = "v2_5")]
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
    fn cpu_processor_pixel_layout_validation_covers_default_and_short_strides() {
        assert_eq!(
            required_scalar_len("test", 2, 0, 4).expect("default RGBA stride"),
            8
        );
        assert_eq!(
            required_scalar_len("test", 2, 6, 4).expect("padded RGBA stride"),
            10
        );
        assert!(required_scalar_len("test", 1, 3, 4).is_err());

        let config = Config::raw().unwrap();
        let proc = config.processor("raw", "raw").unwrap();
        if let Ok(cpu) = proc.default_cpu_processor() {
            let mut too_short_default_stride = vec![0.0f32; 3];
            assert!(cpu
                .try_apply_rgba_pixels(&mut too_short_default_stride, 1, 0)
                .is_err());

            let mut short_stride = vec![0.0f32; 4];
            assert!(cpu.try_apply_rgba_pixels(&mut short_stride, 1, 3).is_err());

            let mut packed_short_stride = vec![0u8; 4];
            assert!(cpu
                .try_apply_rgb_packed_bit_depth(&mut packed_short_stride, BitDepth::Uint8, 1, 2)
                .is_err());

            let mut overflowed_byte_stride = vec![0.0f32; 4];
            assert!(cpu
                .try_apply_rgba_pixels(&mut overflowed_byte_stride, 1, i64::MAX)
                .is_err());

            let mut unaligned_storage = [0u8; 17];
            let unaligned_f32 = &mut unaligned_storage[1..];
            assert!(cpu
                .try_apply_rgba_packed_bit_depth(unaligned_f32, BitDepth::F32, 1, 4)
                .is_err());
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

    #[cfg(feature = "v2_5")]
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

    #[cfg(feature = "v2_5")]
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
