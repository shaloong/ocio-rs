use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;

/// Interleaved image with independent channel offsets within each pixel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomImageLayout {
    /// Dimensions, origin of the first logical pixel, signed strides and bit depth.
    pub layout: crate::SignedPlanarByteLayout,
    /// Byte offsets from each pixel origin, in R, G, B order.
    pub rgb_offsets: [usize; 3],
    /// Optional byte offset of alpha from each pixel origin.
    pub alpha_offset: Option<usize>,
}

/// Logical pixel channel queried by a Rust-defined native descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageDescChannel {
    Red,
    Green,
    Blue,
    Alpha,
}

/// Implements OCIO's virtual image descriptor getters in Rust.
///
/// For ordinary pixel buffers, prefer the checked fixed-layout constructors
/// on [NativeImageDesc].
///
/// # Safety
/// Getters must describe one coherent, valid layout for an entire native
/// operation, including repeated queries. RGB pointers must be non-null;
/// alpha may be null. Pixels must be initialized, aligned, correctly typed,
/// writable, and exclusively accessible to the operation. Pointer provenance
/// must permit writes; pointers derived from shared references to pixel
/// storage without interior mutability are not sufficient.
///
/// Dimensions, strides, and storage must permit native index arithmetic and
/// pointer stepping, including steps past the final sample. Source and
/// destination may not overlap when used for conversion. True packed and
/// float predicates must satisfy OCIO's corresponding fast-path contracts.
/// Getters must not reenter or destroy the descriptor.
pub unsafe trait ImageDescProvider {
    /// Address of this channel at the first logical pixel.
    fn channel_data(&self, channel: ImageDescChannel) -> *mut c_void;
    fn width(&self) -> i64;
    fn height(&self) -> i64;
    fn x_stride_bytes(&self) -> isize;
    fn y_stride_bytes(&self) -> isize;
    fn bit_depth(&self) -> crate::BitDepth;

    /// True only for tightly ordered RGBA storage as defined by OCIO.
    fn is_rgba_packed(&self) -> bool {
        false
    }

    fn is_float(&self) -> bool {
        self.bit_depth() == crate::BitDepth::F32
    }
}

unsafe extern "C" fn query_image_desc<P: ImageDescProvider>(
    context: *const c_void,
    query: ocio_sys::OcioImageDescQuery,
    out: *mut ocio_sys::OcioImageDescValue,
) -> bool {
    use ocio_sys::OcioImageDescQuery::*;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let provider = unsafe { &*context.cast::<P>() };
        let out = unsafe { &mut *out };
        let channel = match query {
            Red => ImageDescChannel::Red,
            Green => ImageDescChannel::Green,
            Blue => ImageDescChannel::Blue,
            Alpha => ImageDescChannel::Alpha,
            other => {
                out.scalar = match other {
                    Width => provider.width(),
                    Height => provider.height(),
                    XStride => provider.x_stride_bytes() as i64,
                    YStride => provider.y_stride_bytes() as i64,
                    BitDepth => i64::from(provider.bit_depth() as i32),
                    Packed => i64::from(provider.is_rgba_packed()),
                    Float => i64::from(provider.is_float()),
                    _ => unreachable!(),
                };
                return;
            }
        };
        out.pointer = provider.channel_data(channel);
    }));
    match result {
        Ok(()) => true,
        Err(payload) => {
            crate::discard_callback_panic(payload);
            false
        }
    }
}

unsafe extern "C" fn release_image_desc<P: ImageDescProvider>(context: *mut c_void) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        drop(unsafe { Box::from_raw(context.cast::<P>()) });
    }));
    if let Err(payload) = result {
        crate::discard_callback_panic(payload);
    }
}

/// An owned OCIO descriptor with exclusive access to its pixel storage.
/// Fixed-layout descriptors do not own their pixels. Callback descriptors
/// own their Rust provider, which may itself own or borrow the storage.
///
/// For ordinary CPU processing, prefer the slice-based processor methods:
/// they accept unpadded layouts that need special tail handling.
///
/// ```compile_fail
/// use ocio_rs::{NativeImageDesc, PackedImageLayout};
/// fn modify_while_borrowed(data: &mut [u8], layout: PackedImageLayout) {
///     let image = NativeImageDesc::packed(data, layout).unwrap();
///     data[0] = 0;
///     drop(image);
/// }
/// ```
pub struct NativeImageDesc<'a> {
    handle: NonNull<c_void>,
    _data: PhantomData<&'a mut [u8]>,
}

impl<'a> NativeImageDesc<'a> {
    /// Create a native descriptor that owns a Rust virtual getter provider.
    ///
    /// OCIO queries the provider for descriptor data, not once per pixel.
    /// A panicking getter aborts that native operation with an error. A
    /// provider destructor cannot unwind through the native bridge.
    pub fn from_provider<P: ImageDescProvider + 'a>(provider: P) -> crate::Result<Self> {
        let context = Box::into_raw(Box::new(provider));
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_image_desc_create_callback(
                context.cast(),
                query_image_desc::<P>,
                release_image_desc::<P>,
            )
        };
        let result = Self::from_created_handle(handle);
        if handle.is_null() {
            // Native ownership transfers only on successful construction.
            drop(unsafe { Box::from_raw(context) });
        }
        result
    }

    /// Construct a native packed descriptor from a checked byte layout.
    /// Layouts requiring a separate tail are rejected.
    pub fn packed(data: &'a mut [u8], layout: crate::PackedImageLayout) -> crate::Result<Self> {
        let (mut raw, split) = crate::processor::packed_image_samples(layout, data)?;
        Self::require_native_layout(split)?;
        raw.data = data.as_mut_ptr().wrapping_add(layout.origin).cast();
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_image_desc_create_packed(&raw) };
        Self::from_created_handle(handle)
    }

    /// Construct a native planar descriptor from checked channel slices.
    /// Layouts requiring a separate tail are rejected.
    pub fn planar(mut image: crate::SignedPlanarBytesMut<'a>) -> crate::Result<Self> {
        let (raw, split) = crate::processor::signed_planar_destination(&mut image)?;
        Self::require_native_layout(split)?;
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_image_desc_create_planar(&raw) };
        Self::from_created_handle(handle)
    }

    /// Construct an OCIO custom descriptor with irregular channel placement.
    ///
    /// Offsets must be distinct, sample-aligned and fit within the absolute
    /// pixel stride. This uses OCIO's generic packing path, so any required
    /// pointer-step padding must be present in the slice.
    pub fn custom(data: &'a mut [u8], layout: CustomImageLayout) -> crate::Result<Self> {
        let base = layout.layout;
        let size = crate::processor::bit_depth_bytes_per_channel(
            "NativeImageDesc::custom",
            base.bit_depth,
        )?;
        let invalid =
            || crate::OcioError::InvalidInput("invalid custom image channel offsets".into());
        let offsets = layout.rgb_offsets;
        let offsets = [
            Some(offsets[0]),
            Some(offsets[1]),
            Some(offsets[2]),
            layout.alpha_offset,
        ];
        let pixel_stride = base.x_stride.unsigned_abs();
        let mut origins = [None; 4];
        let mut dimensions = [0; 4];
        for (index, offset) in offsets.into_iter().enumerate() {
            let Some(offset) = offset else { continue };
            if !offset.is_multiple_of(size)
                || offset
                    .checked_add(size)
                    .is_none_or(|end| end > pixel_stride)
                || offsets[..index].contains(&Some(offset))
            {
                return Err(invalid());
            }
            let origin = base.origin.checked_add(offset).ok_or_else(invalid)?;
            let (dims, split) = crate::processor::validate_signed_planar_samples(
                crate::SignedPlanarByteLayout { origin, ..base },
                &[data],
            )?;
            Self::require_native_layout(split)?;
            dimensions = dims;
            origins[index] = Some(origin);
        }
        let ptr = data.as_mut_ptr();
        let [red, green, blue, alpha] = origins.map(|origin| {
            origin.map_or(std::ptr::null_mut(), |origin| {
                ptr.wrapping_add(origin).cast()
            })
        });
        let [width, height, x_stride, y_stride] = dimensions;
        let raw = ocio_sys::OcioPlanarBuffer {
            red,
            green,
            blue,
            alpha,
            width,
            height,
            x_stride,
            y_stride,
            bit_depth: base.bit_depth as i32,
        };
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_image_desc_create_custom(&raw) };
        Self::from_created_handle(handle)
    }

    fn require_native_layout(split: bool) -> crate::Result<()> {
        if split {
            Err(crate::OcioError::InvalidInput(
                "image needs native pointer-step padding; use a slice-based processor method"
                    .into(),
            ))
        } else {
            Ok(())
        }
    }

    fn from_created_handle(handle: *mut c_void) -> crate::Result<Self> {
        Ok(Self {
            handle: crate::handle_result(handle)?,
            _data: PhantomData,
        })
    }

    /// Query the native descriptor without accessing pixel contents.
    pub fn info(&self) -> crate::Result<NativeImageDescInfo> {
        unsafe { NativeImageDescInfo::inspect(self.handle.as_ptr()) }
    }

    /// Get the native first-pixel pointers for raw buffer interop.
    /// The pointers do not grant access beyond the original buffer borrows.
    pub fn data_ptrs(&mut self) -> crate::Result<NativeImageDataPointers> {
        unsafe { NativeImageDataPointers::inspect(self.handle.as_ptr()) }
    }

    /// Format the descriptor using OCIO's stream operator.
    /// This includes buffer addresses; it is not serialization.
    pub fn diagnostic_text(&self) -> crate::Result<String> {
        unsafe { Self::diagnostic_text_from_ptr(self.handle.as_ptr()) }
    }

    /// Format an externally owned descriptor using OCIO's stream operator.
    /// The diagnostic contains buffer addresses; avoid logging it publicly.
    ///
    /// # Safety
    /// Except for null, which returns an error, `image` must point to a live
    /// OCIO `ImageDesc` base compiled for the active ABI. Its getters must be
    /// callable and may not race with mutation or destruction.
    pub unsafe fn diagnostic_text_from_ptr(image: *const c_void) -> crate::Result<String> {
        crate::clear_last_error();
        let text = unsafe { ocio_sys::ocio_image_desc_diagnostic_text(image) };
        crate::ocio_call_status()?;
        unsafe { crate::cstr_to_opt_string(text.cast()) }
            .ok_or_else(|| crate::OcioError::Ocio("null image descriptor diagnostic text".into()))
    }

    /// Return the borrowed native `ImageDesc` base pointer for C++ interop.
    /// Do not retain or destroy it. Any use must respect the descriptor and
    /// pixel buffer lifetimes and the native API's preconditions.
    pub fn as_mut_ptr(&mut self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

impl Drop for NativeImageDesc<'_> {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_image_desc_destroy(self.handle.as_ptr()) }
    }
}

/// First-pixel addresses returned by OCIO without reading pixel contents.
///
/// These are raw pointers, not Rust references. In particular, they do not
/// establish buffer bounds, validity, lifetime, or exclusivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeImageDataPointers {
    pub red: *mut c_void,
    pub green: *mut c_void,
    pub blue: *mut c_void,
    /// Null when no alpha channel is present.
    pub alpha: *mut c_void,
    /// First channel in storage order, or null for a non-packed descriptor.
    pub packed: *mut c_void,
}

impl NativeImageDataPointers {
    /// Query the native channel and packed-data getters.
    ///
    /// # Safety
    /// Except for null, which returns an error, `image` must point to a live
    /// OCIO `ImageDesc` base compiled for the active ABI. Its getters must be
    /// callable and may not race with mutation or destruction.
    pub unsafe fn inspect(image: *const c_void) -> crate::Result<Self> {
        let mut raw = ocio_sys::OcioImageDescPointers::default();
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_image_desc_pointers(image, &mut raw) };
        crate::ocio_call_status()?;
        Ok(Self {
            red: raw.red,
            green: raw.green,
            blue: raw.blue,
            alpha: raw.alpha,
            packed: raw.packed,
        })
    }
}

/// Concrete layout of an existing OCIO image descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeImageLayout {
    Packed {
        /// Native discriminant corresponding to [`crate::ChannelOrdering`].
        channel_order: i32,
        channels: i64,
        channel_stride_bytes: i64,
    },
    Planar,
    Custom,
}

/// Snapshot of a native image descriptor. This does not expose buffer
/// addresses or certify that the descriptor is safe to process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeImageDescInfo {
    pub width: i64,
    pub height: i64,
    pub x_stride_bytes: i64,
    pub y_stride_bytes: i64,
    /// Native discriminant corresponding to [`crate::BitDepth`].
    pub bit_depth: i32,
    pub has_alpha: bool,
    /// OCIO's tight-RGBA predicate, not a test for the packed descriptor class.
    pub is_rgba_packed: bool,
    /// OCIO's 32-bit-float predicate.
    pub is_float: bool,
    pub layout: NativeImageLayout,
}

impl NativeImageDescInfo {
    /// Query metadata from a C++-owned descriptor without transferring ownership.
    /// No pixels are read by this binding.
    ///
    /// # Safety
    /// Except for null, which returns an error, `image` must point to a live
    /// OCIO `ImageDesc` base compiled for the active ABI. Its getters must be
    /// callable for the duration of this operation and may not race with
    /// mutation or destruction. For custom implementations this includes any
    /// state the virtual getters access.
    pub unsafe fn inspect(image: *const c_void) -> crate::Result<Self> {
        let mut raw = ocio_sys::OcioImageDescInfo::default();
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_image_desc_info(image, &mut raw) };
        crate::ocio_call_status()?;
        let layout = if raw.is_packed {
            NativeImageLayout::Packed {
                channel_order: raw.channel_order,
                channels: raw.num_channels,
                channel_stride_bytes: raw.channel_stride,
            }
        } else if raw.is_planar {
            NativeImageLayout::Planar
        } else {
            NativeImageLayout::Custom
        };
        Ok(Self {
            width: raw.width,
            height: raw.height,
            x_stride_bytes: raw.x_stride,
            y_stride_bytes: raw.y_stride,
            bit_depth: raw.bit_depth,
            has_alpha: raw.has_alpha,
            is_rgba_packed: raw.is_rgba_packed,
            is_float: raw.is_float,
            layout,
        })
    }
}
