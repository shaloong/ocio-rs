//! CPUProcessor behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after unit-level smoke coverage.
//! In bundled/real mode they validate packed/pixel execution paths and stride
//! handling for representative matrix processors.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::MatrixTransform;
use ocio_rs::{BitDepth, CPUProcessor, Processor, TransformDirection};

#[test]
#[cfg(ocio_stub)]
fn raw_pixel_batches_reject_unrepresentable_strides() {
    let mut pixel = [0.0f32; 4];
    let stride = i64::MAX / 4;
    let handle = std::ptr::null_mut();
    let ptr = pixel.as_mut_ptr();
    for mode in 0..3 {
        unsafe { ocio_sys::ocio_error_clear_last() };
        unsafe {
            match mode {
                0 => ocio_sys::ocio_cpu_processor_apply_rgb_pixels(handle, ptr, 2, stride),
                1 => {
                    ocio_sys::ocio_cpu_processor_apply_rgb_packed(handle, ptr.cast(), 8, 2, stride)
                }
                _ => {
                    ocio_sys::ocio_cpu_processor_apply_rgba_packed(handle, ptr.cast(), 8, 2, stride)
                }
            }
        }
        assert!(!unsafe { ocio_sys::ocio_error_get_last() }.is_null());
    }
    unsafe { ocio_sys::ocio_error_clear_last() };
}

#[test]
#[cfg(all(ocio_stub, target_pointer_width = "32"))]
fn raw_rgba_rejects_row_stride_that_exceeds_ptrdiff() {
    let mut pixel = [0.0f32; 4];
    // The individual byte stride fits ptrdiff_t, but two strides do not.
    // Stub mode ensures the preflight is exercised without accessing pixels.
    let accepted = unsafe {
        ocio_sys::ocio_cpu_processor_try_apply_rgba_pixels(
            std::ptr::null_mut(),
            pixel.as_mut_ptr(),
            2,
            i64::from(i32::MAX) / 4,
        )
    };
    assert!(!accepted);
    let error = unsafe { ocio_sys::ocio_error_get_last() };
    assert!(!error.is_null());
    assert!(unsafe { std::ffi::CStr::from_ptr(error) }
        .to_bytes()
        .windows(b"row stride".len())
        .any(|window| window == b"row stride"));
    unsafe { ocio_sys::ocio_error_clear_last() };

    // The largest whole-float stride whose two-pixel row fits remains valid.
    assert!(unsafe {
        ocio_sys::ocio_cpu_processor_try_apply_rgba_pixels(
            std::ptr::null_mut(),
            pixel.as_mut_ptr(),
            2,
            i64::from(i32::MAX) / 8,
        )
    });

    for mode in 0..3 {
        unsafe {
            match mode {
                0 => ocio_sys::ocio_cpu_processor_apply_rgb_pixels(
                    std::ptr::null_mut(),
                    pixel.as_mut_ptr(),
                    2,
                    i64::from(i32::MAX) / 4,
                ),
                1 => ocio_sys::ocio_cpu_processor_apply_rgb_packed(
                    std::ptr::null_mut(),
                    pixel.as_mut_ptr().cast(),
                    8,
                    2,
                    i64::from(i32::MAX) / 4,
                ),
                _ => ocio_sys::ocio_cpu_processor_apply_rgba_packed(
                    std::ptr::null_mut(),
                    pixel.as_mut_ptr().cast(),
                    8,
                    2,
                    i64::from(i32::MAX) / 4,
                ),
            }
        }
        let error = unsafe { ocio_sys::ocio_error_get_last() };
        assert!(!error.is_null());
        assert!(unsafe { std::ffi::CStr::from_ptr(error) }
            .to_bytes()
            .windows(b"row stride".len())
            .any(|window| window == b"row stride"));
    }
    unsafe { ocio_sys::ocio_error_clear_last() };
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_buffers_can_back_owned_native_image_descriptors() {
    use ocio_rs::{
        ChannelOrdering, NativeImageDataPointers, NativeImageDesc, NativeImageLayout,
        PackedImageLayout, SignedPlanarByteLayout, SignedPlanarBytesMut,
    };
    let layout = PackedImageLayout {
        width: 2,
        height: 1,
        origin: 0,
        channel_stride: 1,
        x_stride: 3,
        y_stride: 6,
        bit_depth: BitDepth::Uint8,
        channel_order: ChannelOrdering::Bgr,
    };
    let mut unpadded = [0; 6];
    assert!(NativeImageDesc::packed(&mut unpadded, layout).is_err());
    let mut packed = [7; 8];
    let base = packed.as_ptr().addr();
    let mut descriptor = NativeImageDesc::packed(&mut packed, layout).unwrap();
    let info = descriptor.info().unwrap();
    assert_eq!((info.width, info.height, info.x_stride_bytes), (2, 1, 3));
    assert_eq!(info.bit_depth, BitDepth::Uint8 as i32);
    assert!(!info.has_alpha);
    assert_eq!(
        info.layout,
        NativeImageLayout::Packed {
            channel_order: ChannelOrdering::Bgr as i32,
            channels: 3,
            channel_stride_bytes: 1,
        }
    );
    let pointers = descriptor.data_ptrs().unwrap();
    assert_eq!(pointers.packed.addr(), base);
    assert_eq!(pointers.red.addr(), base + 2);
    assert_eq!(pointers.green.addr(), base + 1);
    assert_eq!(pointers.blue.addr(), base);
    assert!(pointers.alpha.is_null());
    let packed_text = descriptor.diagnostic_text().unwrap();
    assert!(packed_text.starts_with("<PackedImageDesc "));
    assert!(packed_text.contains("width=2"));
    drop(descriptor);
    assert_eq!(packed, [7; 8]);

    let (mut red, mut green, mut blue) = ([1; 3], [2; 3], [3; 3]);
    let (r, g, b) = (
        red.as_ptr().addr(),
        green.as_ptr().addr(),
        blue.as_ptr().addr(),
    );
    let mut descriptor = NativeImageDesc::planar(SignedPlanarBytesMut {
        red: &mut red,
        green: &mut green,
        blue: &mut blue,
        alpha: None,
        layout: SignedPlanarByteLayout {
            width: 2,
            height: 1,
            origin: 2,
            x_stride: -1,
            y_stride: 2,
            bit_depth: BitDepth::Uint8,
        },
    })
    .unwrap();
    let info = descriptor.info().unwrap();
    assert_eq!(info.layout, NativeImageLayout::Planar);
    assert_eq!((info.width, info.height), (2, 1));
    assert_eq!((info.x_stride_bytes, info.y_stride_bytes), (-1, 2));
    assert!(!info.has_alpha);
    let pointers = descriptor.data_ptrs().unwrap();
    assert_eq!(
        [
            pointers.red.addr(),
            pointers.green.addr(),
            pointers.blue.addr()
        ],
        [r + 2, g + 2, b + 2]
    );
    assert!(pointers.packed.is_null() && pointers.alpha.is_null());
    let planar_text = descriptor.diagnostic_text().unwrap();
    assert!(planar_text.starts_with("<PlanarImageDesc "));
    assert!(planar_text.contains("xStrideBytes=-1"));
    assert!(unsafe { NativeImageDataPointers::inspect(std::ptr::null()) }.is_err());
    assert!(unsafe { NativeImageDesc::diagnostic_text_from_ptr(std::ptr::null()) }.is_err());
}

fn cpu_processor_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn rust_image_provider_dispatches_and_owns_its_context() {
    use ocio_rs::{ImageDescChannel, ImageDescProvider, NativeImageDesc, NativeImageLayout};
    use std::cell::{Cell, UnsafeCell};
    use std::ffi::c_void;

    struct Provider<'a> {
        pixels: &'a mut [UnsafeCell<f32>; 4],
        panic_on_width: &'a Cell<bool>,
        drops: &'a Cell<usize>,
    }
    // One tightly ordered writable RGBA pixel, with an exclusive buffer
    // borrow for the descriptor lifetime and writable pointer provenance.
    unsafe impl ImageDescProvider for Provider<'_> {
        fn channel_data(&self, channel: ImageDescChannel) -> *mut c_void {
            self.pixels[match channel {
                ImageDescChannel::Red => 0,
                ImageDescChannel::Green => 1,
                ImageDescChannel::Blue => 2,
                ImageDescChannel::Alpha => 3,
            }]
            .get()
            .cast()
        }
        fn width(&self) -> i64 {
            assert!(!self.panic_on_width.get(), "width getter panicked");
            1
        }
        fn height(&self) -> i64 {
            1
        }
        fn x_stride_bytes(&self) -> isize {
            16
        }
        fn y_stride_bytes(&self) -> isize {
            16
        }
        fn bit_depth(&self) -> BitDepth {
            BitDepth::F32
        }
        fn is_rgba_packed(&self) -> bool {
            true
        }
    }
    impl Drop for Provider<'_> {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
        }
    }

    let mut pixels = [0.25, 0.125, 0.125, 0.75].map(UnsafeCell::new);
    let alpha = pixels[3].get();
    let drops = Cell::new(0);
    let panic_on_width = Cell::new(false);
    let result = NativeImageDesc::from_provider(Provider {
        pixels: &mut pixels,
        panic_on_width: &panic_on_width,
        drops: &drops,
    });
    if cfg!(ocio_stub) {
        assert!(result.is_err());
        assert_eq!(drops.get(), 1);
        return;
    }

    let mut image = result.unwrap();
    assert_eq!(drops.get(), 0);
    let info = image.info().unwrap();
    assert_eq!(info.layout, NativeImageLayout::Custom);
    assert_eq!((info.width, info.height), (1, 1));
    assert!(info.is_float && info.is_rgba_packed && info.has_alpha);
    assert_eq!(image.data_ptrs().unwrap().alpha, alpha.cast());
    assert!(image.diagnostic_text().unwrap().starts_with("<ImageDesc "));

    let _guard = cpu_processor_test_lock();
    let cpu = scaled_cpu_processor([2.0, 3.0, 4.0, 1.0]).unwrap();
    panic_on_width.set(true);
    assert!(image
        .info()
        .unwrap_err()
        .to_string()
        .contains("getter failed"));
    assert!(cpu.try_apply_image_desc(&mut image).is_err());
    panic_on_width.set(false);
    cpu.try_apply_image_desc(&mut image).unwrap();
    let mut output = [0.0; 4].map(UnsafeCell::new);
    let mut destination = NativeImageDesc::from_provider(Provider {
        pixels: &mut output,
        panic_on_width: &panic_on_width,
        drops: &drops,
    })
    .unwrap();
    cpu.try_convert_image_desc(&mut image, &mut destination)
        .unwrap();
    drop(destination);
    drop(image);
    assert_eq!(drops.get(), 2);
    assert_eq!(pixels.map(UnsafeCell::into_inner), [0.5, 0.375, 0.5, 0.75]);
    assert_eq!(output.map(UnsafeCell::into_inner), [1.0, 1.125, 2.0, 0.75]);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn custom_native_descriptor_handles_irregular_channel_offsets() {
    use ocio_rs::{CustomImageLayout, NativeImageDesc, NativeImageLayout, SignedPlanarByteLayout};
    fn bytes(values: &mut [f32]) -> &mut [u8] {
        // f32 storage is aligned and permits every bit pattern.
        unsafe {
            std::slice::from_raw_parts_mut(
                values.as_mut_ptr().cast(),
                std::mem::size_of_val(values),
            )
        }
    }
    let _guard = cpu_processor_test_lock();
    let cpu = scaled_cpu_processor([2.0, 3.0, 4.0, 1.0]).unwrap();
    let layout = CustomImageLayout {
        layout: SignedPlanarByteLayout {
            width: 2,
            height: 1,
            origin: 0,
            x_stride: 20,
            y_stride: 40,
            bit_depth: BitDepth::F32,
        },
        rgb_offsets: [12, 4, 0],
        alpha_offset: None,
    };
    let mut data = [
        0.25, 0.125, -1.0, 0.25, -1.0, 0.125, 0.25, -1.0, 0.5, -1.0, -1.0, -1.0, -1.0,
    ];
    assert!(NativeImageDesc::custom(bytes(&mut data[..12]), layout).is_err());
    for offsets in [[12, 4, 4], [12, 6, 0], [20, 4, 0]] {
        assert!(NativeImageDesc::custom(
            bytes(&mut data),
            CustomImageLayout {
                rgb_offsets: offsets,
                ..layout
            }
        )
        .is_err());
    }
    let base = data.as_ptr().addr();
    let mut desc = NativeImageDesc::custom(bytes(&mut data), layout).unwrap();
    let info = desc.info().unwrap();
    assert_eq!(info.layout, NativeImageLayout::Custom);
    assert_eq!((info.x_stride_bytes, info.y_stride_bytes), (20, 40));
    assert!(!info.has_alpha && !info.is_rgba_packed && info.is_float);
    let pointers = desc.data_ptrs().unwrap();
    assert_eq!(pointers.red.addr(), base + 12);
    assert_eq!(pointers.green.addr(), base + 4);
    assert_eq!(pointers.blue.addr(), base);
    assert!(pointers.packed.is_null() && pointers.alpha.is_null());
    assert!(desc.diagnostic_text().unwrap().starts_with("<ImageDesc "));
    cpu.try_apply_image_desc(&mut desc).unwrap();
    drop(desc);
    assert_eq!(
        data,
        [1.0, 0.375, -1.0, 0.5, -1.0, 0.5, 0.75, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0]
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn checked_native_descriptors_reuse_buffers_and_reject_mismatches() {
    use ocio_rs::{
        ChannelOrdering, NativeImageDesc, PackedImageLayout, SignedPlanarByteLayout,
        SignedPlanarBytesMut,
    };
    fn bytes(values: &mut [f32]) -> &mut [u8] {
        // f32 storage is aligned and every byte may be read or written.
        unsafe {
            std::slice::from_raw_parts_mut(
                values.as_mut_ptr().cast(),
                std::mem::size_of_val(values),
            )
        }
    }
    let _guard = cpu_processor_test_lock();
    let cpu = scaled_cpu_processor([2.0, 3.0, 4.0, 1.0]).unwrap();
    let layout = PackedImageLayout {
        width: 1,
        height: 1,
        origin: 0,
        channel_stride: 4,
        x_stride: 16,
        y_stride: 16,
        bit_depth: BitDepth::F32,
        channel_order: ChannelOrdering::Rgba,
    };
    let mut packed = [0.25, 0.125, 0.25, 0.75];
    let mut source = NativeImageDesc::packed(bytes(&mut packed), layout).unwrap();
    let mut wrong_depth = [123; 4];
    let mut wrong_size = [9.0; 8];
    {
        let mut wrong = NativeImageDesc::packed(
            &mut wrong_depth,
            PackedImageLayout {
                bit_depth: BitDepth::Uint8,
                channel_stride: 1,
                x_stride: 4,
                y_stride: 4,
                ..layout
            },
        )
        .unwrap();
        assert!(cpu.try_convert_image_desc(&mut source, &mut wrong).is_err());
        assert!(cpu.try_apply_image_desc(&mut wrong).is_err());
    }
    {
        let mut wrong = NativeImageDesc::packed(
            bytes(&mut wrong_size),
            PackedImageLayout {
                width: 2,
                y_stride: 32,
                ..layout
            },
        )
        .unwrap();
        assert!(cpu.try_convert_image_desc(&mut source, &mut wrong).is_err());
    }
    assert_eq!(wrong_depth, [123; 4]);
    assert_eq!(wrong_size, [9.0; 8]);
    let (mut r, mut g, mut b, mut a) = ([-1.0; 2], [-1.0; 2], [-1.0; 2], [-1.0; 2]);
    let mut dst = NativeImageDesc::planar(SignedPlanarBytesMut {
        red: bytes(&mut r),
        green: bytes(&mut g),
        blue: bytes(&mut b),
        alpha: Some(bytes(&mut a)),
        layout: SignedPlanarByteLayout {
            width: 1,
            height: 1,
            origin: 0,
            x_stride: 4,
            y_stride: 4,
            bit_depth: BitDepth::F32,
        },
    })
    .unwrap();
    cpu.try_convert_image_desc(&mut source, &mut dst).unwrap();
    cpu.try_apply_image_desc(&mut source).unwrap();
    drop(dst);
    drop(source);
    let expected = [0.5, 0.375, 1.0, 0.75];
    assert_eq!(packed, expected);
    assert_eq!([r[0], g[0], b[0], a[0]], expected);
    assert_eq!([r[1], g[1], b[1], a[1]], [-1.0; 4]);
}

fn scaled_cpu_processor(scale: [f64; 4]) -> Option<CPUProcessor> {
    let config = ocio_rs::Config::raw().ok()?;
    let transform = MatrixTransform::scale(&scale).ok()?;
    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()?;
    processor.default_cpu_processor().ok()
}

fn scaled_processor(scale: [f64; 4]) -> Option<Processor> {
    let config = ocio_rs::Config::raw().ok()?;
    let transform = MatrixTransform::scale(&scale).ok()?;
    config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()
}

fn f32s_to_bytes(values: &[f32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}

fn bytes_to_f32s(bytes: &[u8]) -> Vec<f32> {
    let (chunks, remainder) = bytes.as_chunks::<4>();
    assert!(
        remainder.is_empty(),
        "f32 byte input must be 4-byte aligned"
    );
    chunks.iter().copied().map(f32::from_ne_bytes).collect()
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rgba_fast_path_clears_stale_native_errors_and_validates_buffers() {
    let _guard = cpu_processor_test_lock();
    let cpu = scaled_cpu_processor([0.5, 1.0, 1.0, 1.0]).unwrap();
    let missing = std::ffi::CString::new("ocio-rs-nonexistent-fast-path-config.ocio").unwrap();
    let set_stale_error = || unsafe {
        ocio_rs::ocio_sys::ocio_error_clear_last();
        assert!(ocio_rs::ocio_sys::ocio_config_create_from_file(missing.as_ptr()).is_null());
        assert!(!ocio_rs::ocio_sys::ocio_error_get_last().is_null());
    };
    set_stale_error();
    let mut pixel = [0.8, 0.5, 0.25, 1.0];
    cpu.try_apply_rgba(&mut pixel).unwrap();
    assert_close(pixel[0] as f64, 0.4, 1e-6);
    assert!(unsafe { ocio_rs::ocio_sys::ocio_error_get_last().is_null() });
    set_stale_error();
    let mut pixels = [0.8, 0.5, 0.25, 1.0, 0.4, 0.5, 0.25, 1.0];
    cpu.try_apply_rgba_pixels(&mut pixels, 2, 4).unwrap();
    assert_close(pixels[0] as f64, 0.4, 1e-6);
    assert_close(pixels[4] as f64, 0.2, 1e-6);
    assert!(unsafe { ocio_rs::ocio_sys::ocio_error_get_last().is_null() });
    assert!(cpu.try_apply_rgba_pixels(&mut pixels[..3], 1, 4).is_err());
    cpu.try_apply_rgba_pixels(&mut pixels, 2, 4).unwrap();
    let integer_cpu = scaled_processor([0.5, 1.0, 1.0, 1.0])
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    let mut rgba_pixel = [0.8, 0.5, 0.25, 1.0];
    let original_rgba = rgba_pixel;
    assert!(integer_cpu.try_apply_rgba(&mut rgba_pixel).is_err());
    assert_eq!(rgba_pixel, original_rgba);
    let mut rgb_pixel = [0.8, 0.5, 0.25];
    let original_rgb = rgb_pixel;
    assert!(integer_cpu.try_apply_rgb(&mut rgb_pixel).is_err());
    assert_eq!(rgb_pixel, original_rgb);
    let check_batch_mismatch = |processor: &ocio_rs::CPUProcessor| {
        let mut rgba = [0.8, 0.5, 0.25, 1.0, 0.4, 0.3, 0.2, 1.0];
        let mut rgb = [0.8, 0.5, 0.25, 0.4, 0.3, 0.2];
        let expected_rgba = rgba;
        let expected_rgb = rgb;
        assert!(processor.try_apply_rgba_pixels(&mut rgba, 2, 4).is_err());
        assert_eq!(rgba, expected_rgba);
        assert!(processor.try_apply_rgb_pixels(&mut rgb, 2, 3).is_err());
        assert_eq!(rgb, expected_rgb);
    };
    check_batch_mismatch(&integer_cpu);
    for (input, output) in [
        (BitDepth::Uint8, BitDepth::F32),
        (BitDepth::F32, BitDepth::Uint8),
    ] {
        let mixed_cpu = scaled_processor([0.5, 1.0, 1.0, 1.0])
            .unwrap()
            .optimized_cpu_processor_bit_depth(input, output, 0)
            .unwrap();
        assert!(mixed_cpu.try_apply_rgba(&mut rgba_pixel).is_err());
        assert_eq!(rgba_pixel, original_rgba);
        assert!(mixed_cpu.try_apply_rgb(&mut rgb_pixel).is_err());
        assert_eq!(rgb_pixel, original_rgb);
        check_batch_mismatch(&mixed_cpu);
    }
    let explicit_f32 = scaled_processor([0.5, 1.0, 1.0, 1.0])
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::F32, BitDepth::F32, 0)
        .unwrap();
    explicit_f32.try_apply_rgba(&mut rgba_pixel).unwrap();
    explicit_f32.try_apply_rgb(&mut rgb_pixel).unwrap();
    assert_close(rgba_pixel[0] as f64, 0.4, 1e-6);
    assert_close(rgb_pixel[0] as f64, 0.4, 1e-6);
    assert!(integer_cpu
        .try_apply_rgba_pixels(&mut pixels, 2, 4)
        .is_err());
    assert!(unsafe { ocio_rs::ocio_sys::ocio_error_get_last().is_null() });
    cpu.try_apply_rgba_pixels(&mut pixels, 2, 4).unwrap();
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn packed_images_support_order_padding_and_signed_rows() {
    use ocio_rs::{ChannelOrdering, PackedImageLayout};
    let processor = scaled_processor([0.5, 1.0, 1.0, 1.0]).unwrap();
    let cpu = processor
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    // Two BGRA rows, one pixel each, with two-byte channel spacing.
    let layout = PackedImageLayout {
        width: 1,
        height: 2,
        origin: 8,
        channel_stride: 2,
        x_stride: 8,
        y_stride: -8,
        bit_depth: BitDepth::Uint8,
        channel_order: ChannelOrdering::Bgra,
    };
    let mut data = [
        10, 99, 20, 99, 100, 99, 255, 99, 30, 99, 40, 99, 200, 99, 255, 99, 99, 99, 99, 99, 99, 99,
    ];
    cpu.try_apply_packed_image(&mut data, layout).unwrap();
    assert_eq!(
        data,
        [
            10, 99, 20, 99, 50, 99, 255, 99, 30, 99, 40, 99, 100, 99, 255, 99, 99, 99, 99, 99, 99,
            99
        ]
    );
    let mut invalid = layout;
    invalid.origin = 0;
    assert!(cpu.try_apply_packed_image(&mut data, invalid).is_err());
    invalid = layout;
    invalid.channel_stride = usize::MAX;
    assert!(cpu.try_apply_packed_image(&mut data, invalid).is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn packed_images_convert_channel_order_and_bit_depth() {
    use ocio_rs::{ChannelOrdering, PackedImageLayout};
    let processor = scaled_processor([1.0; 4]).unwrap();
    let cpu = processor
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    let input = [10u8, 20, 30, 40, 50, 60, 99, 99]; // BGR with pointer-step padding
    let input_layout = PackedImageLayout {
        width: 2,
        height: 1,
        origin: 0,
        channel_stride: 1,
        x_stride: 3,
        y_stride: 6,
        bit_depth: BitDepth::Uint8,
        channel_order: ChannelOrdering::Bgr,
    };
    let mut output = [0u16; 8];
    let output_bytes = unsafe { std::slice::from_raw_parts_mut(output.as_mut_ptr().cast(), 16) };
    let output_layout = PackedImageLayout {
        width: 2,
        height: 1,
        origin: 0,
        channel_stride: 2,
        x_stride: 6,
        y_stride: 12,
        bit_depth: BitDepth::Uint16,
        channel_order: ChannelOrdering::Rgb,
    };
    cpu.try_convert_packed_image(&input, input_layout, output_bytes, output_layout)
        .unwrap();
    assert_eq!(output, [7710, 5140, 2570, 15420, 12850, 10280, 0, 0]);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn packed_conversion_unpadded_matches_padded_across_stride_signs() {
    use ocio_rs::{ChannelOrdering, PackedImageLayout};
    let cpu = scaled_processor([0.5, 1.0, 0.75, 1.0])
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    fn bytes(words: &mut [u16]) -> &mut [u8] {
        // SAFETY: the byte slice covers the same exclusively borrowed words.
        unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words))
        }
    }
    let source: Vec<u8> = (0..31).map(|n| 30 + n).collect();
    let mut padded_source = vec![99u8; 59];
    padded_source[14..45].copy_from_slice(&source);
    for ix in [-8isize, 8] {
        for iy in [-18isize, 18] {
            for ox in [-18isize, 18] {
                for oy in [-38isize, 38] {
                    let src = PackedImageLayout {
                        width: 2,
                        height: 2,
                        origin: (if ix < 0 { 8 } else { 0 }) + (if iy < 0 { 18 } else { 0 }),
                        channel_stride: 2,
                        x_stride: ix,
                        y_stride: iy,
                        bit_depth: BitDepth::Uint8,
                        channel_order: ChannelOrdering::Bgr,
                    };
                    let dst = PackedImageLayout {
                        width: 2,
                        height: 2,
                        origin: (if ox < 0 { 18 } else { 0 }) + (if oy < 0 { 38 } else { 0 }),
                        channel_stride: 4,
                        x_stride: ox,
                        y_stride: oy,
                        bit_depth: BitDepth::Uint16,
                        channel_order: ChannelOrdering::Abgr,
                    };
                    let mut actual = [0xaaaau16; 35];
                    let mut expected = [0xaaaau16; 69];
                    cpu.try_convert_packed_image(&source, src, bytes(&mut actual), dst)
                        .unwrap();
                    cpu.try_convert_packed_image(
                        &padded_source,
                        PackedImageLayout {
                            origin: src.origin + 14,
                            ..src
                        },
                        bytes(&mut expected),
                        PackedImageLayout {
                            origin: dst.origin + 34,
                            ..dst
                        },
                    )
                    .unwrap();
                    assert_eq!(&actual[..], &expected[17..52], "{ix},{iy} -> {ox},{oy}");
                    let mut source_short_only = [0xaaaau16; 69];
                    cpu.try_convert_packed_image(
                        &source,
                        src,
                        bytes(&mut source_short_only),
                        PackedImageLayout {
                            origin: dst.origin + 34,
                            ..dst
                        },
                    )
                    .unwrap();
                    assert_eq!(&source_short_only[17..52], &expected[17..52]);
                    let mut destination_short_only = [0xaaaau16; 35];
                    cpu.try_convert_packed_image(
                        &padded_source,
                        PackedImageLayout {
                            origin: src.origin + 14,
                            ..src
                        },
                        bytes(&mut destination_short_only),
                        dst,
                    )
                    .unwrap();
                    assert_eq!(&destination_short_only[..], &expected[17..52]);
                }
            }
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn packed_conversion_unpadded_single_row_column_and_pixel() {
    use ocio_rs::{ChannelOrdering, PackedImageLayout};
    let cpu = scaled_processor([0.5, 1.0, 0.75, 1.0])
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    for (width, height) in [(1, 1), (3, 1), (1, 3)] {
        for sx in [-1isize, 1] {
            for sy in [-1isize, 1] {
                let input_x = 4usize;
                let output_x = 5usize;
                let input_y = width * input_x + 1;
                let output_y = width * output_x + 1;
                let input_len = (width - 1) * input_x + (height - 1) * input_y + 3;
                let output_len = (width - 1) * output_x + (height - 1) * output_y + 4;
                let source = PackedImageLayout {
                    width,
                    height,
                    origin: if sx < 0 { (width - 1) * input_x } else { 0 }
                        + if sy < 0 { (height - 1) * input_y } else { 0 },
                    channel_stride: 1,
                    x_stride: sx * input_x as isize,
                    y_stride: sy * input_y as isize,
                    bit_depth: BitDepth::Uint8,
                    channel_order: ChannelOrdering::Rgb,
                };
                let target = PackedImageLayout {
                    width,
                    height,
                    origin: if sx > 0 { (width - 1) * output_x } else { 0 }
                        + if sy > 0 { (height - 1) * output_y } else { 0 },
                    channel_stride: 1,
                    x_stride: -sx * output_x as isize,
                    y_stride: -sy * output_y as isize,
                    bit_depth: BitDepth::Uint8,
                    channel_order: ChannelOrdering::Bgra,
                };
                let input = vec![60u8; input_len];
                let mut padded_input = vec![99u8; input_len + 32];
                padded_input[16..16 + input_len].copy_from_slice(&input);
                let mut output = vec![99u8; output_len];
                let mut padded_output = vec![99u8; output_len + 32];
                cpu.try_convert_packed_image(&input, source, &mut output, target)
                    .unwrap();
                cpu.try_convert_packed_image(
                    &padded_input,
                    PackedImageLayout {
                        origin: source.origin + 16,
                        ..source
                    },
                    &mut padded_output,
                    PackedImageLayout {
                        origin: target.origin + 16,
                        ..target
                    },
                )
                .unwrap();
                assert_eq!(
                    &output[..],
                    &padded_output[16..16 + output_len],
                    "{width}x{height} {sx}/{sy}"
                );
            }
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn mixed_unpadded_conversions_match_padded_across_stride_signs() {
    use ocio_rs::{
        ChannelOrdering, PackedImageLayout, SignedPlanarByteLayout, SignedPlanarBytes,
        SignedPlanarBytesMut,
    };
    let cpu = scaled_processor([0.5, 1., 0.75, 1.])
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    fn bytes(words: &mut [u16]) -> &mut [u8] {
        // SAFETY: same exclusively borrowed word allocation as bytes.
        unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words))
        }
    }
    for px in [-1isize, 1] {
        for py in [-1isize, 1] {
            for qx in [-1isize, 1] {
                for qy in [-1isize, 1] {
                    for alpha in [false, true] {
                        let packed_in = PackedImageLayout {
                            width: 2,
                            height: 2,
                            origin: if px < 0 { 8 } else { 0 } + if py < 0 { 18 } else { 0 },
                            channel_stride: 2,
                            x_stride: px * 8,
                            y_stride: py * 18,
                            bit_depth: BitDepth::Uint8,
                            channel_order: ChannelOrdering::Bgr,
                        };
                        let planar_out = SignedPlanarByteLayout {
                            width: 2,
                            height: 2,
                            origin: if qx < 0 { 4 } else { 0 } + if qy < 0 { 10 } else { 0 },
                            x_stride: qx * 4,
                            y_stride: qy * 10,
                            bit_depth: BitDepth::Uint16,
                        };
                        let packed_data: Vec<u8> = (0..31).map(|v| 30 + v).collect();
                        let mut padded_packed = vec![99u8; 59];
                        padded_packed[14..45].copy_from_slice(&packed_data);
                        let mut actual = [[0xaaaau16; 8]; 4];
                        let mut expected = [[0xaaaau16; 12]; 4];
                        let [r, g, b, a] = &mut actual;
                        cpu.try_convert_packed_to_planar(
                            &packed_data,
                            packed_in,
                            SignedPlanarBytesMut {
                                red: bytes(r),
                                green: bytes(g),
                                blue: bytes(b),
                                alpha: alpha.then(|| bytes(a)),
                                layout: planar_out,
                            },
                        )
                        .unwrap();
                        let [r, g, b, a] = &mut expected;
                        cpu.try_convert_packed_to_planar(
                            &padded_packed,
                            PackedImageLayout {
                                origin: packed_in.origin + 14,
                                ..packed_in
                            },
                            SignedPlanarBytesMut {
                                red: bytes(r),
                                green: bytes(g),
                                blue: bytes(b),
                                alpha: alpha.then(|| bytes(a)),
                                layout: SignedPlanarByteLayout {
                                    origin: planar_out.origin + 4,
                                    ..planar_out
                                },
                            },
                        )
                        .unwrap();
                        for (a, b) in actual.iter().zip(expected.iter()) {
                            assert_eq!(
                                &a[..],
                                &b[2..10],
                                "packed to planar: {px},{py}->{qx},{qy}, alpha={alpha}"
                            );
                        }
                        let mut packed_short_only = [[0xaaaau16; 12]; 4];
                        let [r, g, b, a] = &mut packed_short_only;
                        cpu.try_convert_packed_to_planar(
                            &packed_data,
                            packed_in,
                            SignedPlanarBytesMut {
                                red: bytes(r),
                                green: bytes(g),
                                blue: bytes(b),
                                alpha: alpha.then(|| bytes(a)),
                                layout: SignedPlanarByteLayout {
                                    origin: planar_out.origin + 4,
                                    ..planar_out
                                },
                            },
                        )
                        .unwrap();
                        for (a, b) in packed_short_only.iter().zip(expected.iter()) {
                            assert_eq!(&a[2..10], &b[2..10]);
                        }
                        let mut planar_short_only = [[0xaaaau16; 8]; 4];
                        let [r, g, b, a] = &mut planar_short_only;
                        cpu.try_convert_packed_to_planar(
                            &padded_packed,
                            PackedImageLayout {
                                origin: packed_in.origin + 14,
                                ..packed_in
                            },
                            SignedPlanarBytesMut {
                                red: bytes(r),
                                green: bytes(g),
                                blue: bytes(b),
                                alpha: alpha.then(|| bytes(a)),
                                layout: planar_out,
                            },
                        )
                        .unwrap();
                        for (a, b) in planar_short_only.iter().zip(expected.iter()) {
                            assert_eq!(&a[..], &b[2..10]);
                        }

                        let planar_in = SignedPlanarByteLayout {
                            width: 2,
                            height: 2,
                            origin: if qx < 0 { 2 } else { 0 } + if qy < 0 { 5 } else { 0 },
                            x_stride: qx * 2,
                            y_stride: qy * 5,
                            bit_depth: BitDepth::Uint8,
                        };
                        let packed_out = PackedImageLayout {
                            width: 2,
                            height: 2,
                            origin: if px < 0 { 18 } else { 0 } + if py < 0 { 38 } else { 0 },
                            channel_stride: 4,
                            x_stride: px * 18,
                            y_stride: py * 38,
                            bit_depth: BitDepth::Uint16,
                            channel_order: ChannelOrdering::Abgr,
                        };
                        let planes = [50u8; 8];
                        let padded_planes = [50u8; 12];
                        let mut actual = [0xaaaau16; 35];
                        let mut expected = [0xaaaau16; 69];
                        cpu.try_convert_planar_to_packed(
                            SignedPlanarBytes {
                                red: &planes,
                                green: &planes,
                                blue: &planes,
                                alpha: alpha.then_some(&planes[..]),
                                layout: planar_in,
                            },
                            bytes(&mut actual),
                            packed_out,
                        )
                        .unwrap();
                        cpu.try_convert_planar_to_packed(
                            SignedPlanarBytes {
                                red: &padded_planes,
                                green: &padded_planes,
                                blue: &padded_planes,
                                alpha: alpha.then_some(&padded_planes[..]),
                                layout: SignedPlanarByteLayout {
                                    origin: planar_in.origin + 2,
                                    ..planar_in
                                },
                            },
                            bytes(&mut expected),
                            PackedImageLayout {
                                origin: packed_out.origin + 34,
                                ..packed_out
                            },
                        )
                        .unwrap();
                        assert_eq!(
                            &actual[..],
                            &expected[17..52],
                            "planar to packed: {qx},{qy}->{px},{py}, alpha={alpha}"
                        );
                        let mut planar_short_only = [0xaaaau16; 69];
                        cpu.try_convert_planar_to_packed(
                            SignedPlanarBytes {
                                red: &planes,
                                green: &planes,
                                blue: &planes,
                                alpha: alpha.then_some(&planes[..]),
                                layout: planar_in,
                            },
                            bytes(&mut planar_short_only),
                            PackedImageLayout {
                                origin: packed_out.origin + 34,
                                ..packed_out
                            },
                        )
                        .unwrap();
                        assert_eq!(&planar_short_only[17..52], &expected[17..52]);
                        let mut packed_short_only = [0xaaaau16; 35];
                        cpu.try_convert_planar_to_packed(
                            SignedPlanarBytes {
                                red: &padded_planes,
                                green: &padded_planes,
                                blue: &padded_planes,
                                alpha: alpha.then_some(&padded_planes[..]),
                                layout: SignedPlanarByteLayout {
                                    origin: planar_in.origin + 2,
                                    ..planar_in
                                },
                            },
                            bytes(&mut packed_short_only),
                            packed_out,
                        )
                        .unwrap();
                        assert_eq!(&packed_short_only[..], &expected[17..52]);
                    }
                }
            }
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn mixed_unpadded_multibyte_depths_and_orders_match_native() {
    use ocio_rs::{
        ChannelOrdering, PackedImageLayout, SignedPlanarByteLayout, SignedPlanarBytes,
        SignedPlanarBytesMut,
    };
    #[repr(align(4))]
    #[derive(Clone, Copy)]
    struct Aligned([u8; 512]);
    let processor = scaled_processor([0.5, 1.0, 0.75, 1.0]).unwrap();
    for (input_depth, output_depth, output_size) in [
        (BitDepth::F16, BitDepth::F32, 4usize),
        (BitDepth::Uint16, BitDepth::Uint8, 1),
    ] {
        let cpu = processor
            .optimized_cpu_processor_bit_depth(input_depth, output_depth, 0)
            .unwrap();
        let input_size = 2usize;
        for (order, channels) in [
            (ChannelOrdering::Rgba, 4usize),
            (ChannelOrdering::Bgra, 4),
            (ChannelOrdering::Abgr, 4),
            (ChannelOrdering::Rgb, 3),
            (ChannelOrdering::Bgr, 3),
        ] {
            let input_channel_stride = input_size * 2;
            let input_x = channels * input_channel_stride + 2 * input_size;
            let input_y = 2 * input_x + 2 * input_size;
            let input_len = input_x + input_y + (channels - 1) * input_channel_stride + input_size;
            let packed_input_layout = PackedImageLayout {
                width: 2,
                height: 2,
                origin: 0,
                channel_stride: input_channel_stride,
                x_stride: input_x as isize,
                y_stride: input_y as isize,
                bit_depth: input_depth,
                channel_order: order,
            };
            let output_x = output_size * 2;
            let output_y = output_x * 2 + output_size;
            let output_len = output_x + output_y + output_size;
            let planar_output_layout = SignedPlanarByteLayout {
                width: 2,
                height: 2,
                origin: 0,
                x_stride: output_x as isize,
                y_stride: output_y as isize,
                bit_depth: output_depth,
            };
            let mut input = Aligned([0; 512]);
            for pair in input.0.as_chunks_mut::<2>().0 {
                pair.copy_from_slice(&0x3800u16.to_ne_bytes());
            }
            let mut padded_input = Aligned([0; 512]);
            padded_input.0[64..64 + input_len].copy_from_slice(&input.0[..input_len]);
            for alpha in [false, true] {
                let mut result = [Aligned([0xaa; 512]); 4];
                let mut reference = result;
                let [r, g, b, a] = &mut result;
                cpu.try_convert_packed_to_planar(
                    &input.0[..input_len],
                    packed_input_layout,
                    SignedPlanarBytesMut {
                        red: &mut r.0[..output_len],
                        green: &mut g.0[..output_len],
                        blue: &mut b.0[..output_len],
                        alpha: alpha.then(|| &mut a.0[..output_len]),
                        layout: planar_output_layout,
                    },
                )
                .unwrap();
                let [r, g, b, a] = &mut reference;
                cpu.try_convert_packed_to_planar(
                    &padded_input.0[..input_len + 128],
                    PackedImageLayout {
                        origin: 64,
                        ..packed_input_layout
                    },
                    SignedPlanarBytesMut {
                        red: &mut r.0[..output_len + 128],
                        green: &mut g.0[..output_len + 128],
                        blue: &mut b.0[..output_len + 128],
                        alpha: alpha.then(|| &mut a.0[..output_len + 128]),
                        layout: SignedPlanarByteLayout {
                            origin: 64,
                            ..planar_output_layout
                        },
                    },
                )
                .unwrap();
                for (a, b) in result.iter().zip(reference.iter()) {
                    assert_eq!(
                        &a.0[..output_len],
                        &b.0[64..64 + output_len],
                        "packed->planar {input_depth:?}->{output_depth:?} {order:?} alpha={alpha}"
                    );
                }

                let planar_x = input_size * 2;
                let planar_y = planar_x * 2 + input_size;
                let planar_len = planar_x + planar_y + input_size;
                let planar_input_layout = SignedPlanarByteLayout {
                    width: 2,
                    height: 2,
                    origin: 0,
                    x_stride: planar_x as isize,
                    y_stride: planar_y as isize,
                    bit_depth: input_depth,
                };
                let output_channel_stride = output_size * 2;
                let packed_x = channels * output_channel_stride + 2 * output_size;
                let packed_y = 2 * packed_x + 2 * output_size;
                let packed_len =
                    packed_x + packed_y + (channels - 1) * output_channel_stride + output_size;
                let packed_output_layout = PackedImageLayout {
                    width: 2,
                    height: 2,
                    origin: 0,
                    channel_stride: output_channel_stride,
                    x_stride: packed_x as isize,
                    y_stride: packed_y as isize,
                    bit_depth: output_depth,
                    channel_order: order,
                };
                let mut output = Aligned([0xaa; 512]);
                let mut expected = output;
                cpu.try_convert_planar_to_packed(
                    SignedPlanarBytes {
                        red: &input.0[..planar_len],
                        green: &input.0[..planar_len],
                        blue: &input.0[..planar_len],
                        alpha: alpha.then_some(&input.0[..planar_len]),
                        layout: planar_input_layout,
                    },
                    &mut output.0[..packed_len],
                    packed_output_layout,
                )
                .unwrap();
                cpu.try_convert_planar_to_packed(
                    SignedPlanarBytes {
                        red: &input.0[..planar_len + 128],
                        green: &input.0[..planar_len + 128],
                        blue: &input.0[..planar_len + 128],
                        alpha: alpha.then_some(&input.0[..planar_len + 128]),
                        layout: SignedPlanarByteLayout {
                            origin: 64,
                            ..planar_input_layout
                        },
                    },
                    &mut expected.0[..packed_len + 128],
                    PackedImageLayout {
                        origin: 64,
                        ..packed_output_layout
                    },
                )
                .unwrap();
                assert_eq!(
                    &output.0[..packed_len],
                    &expected.0[64..64 + packed_len],
                    "planar->packed {input_depth:?}->{output_depth:?} {order:?} alpha={alpha}"
                );
            }
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn mixed_unpadded_handles_single_pixels_rows_and_columns() {
    use ocio_rs::{
        ChannelOrdering, PackedImageLayout, SignedPlanarByteLayout, SignedPlanarBytes,
        SignedPlanarBytesMut,
    };
    let cpu = scaled_processor([0.5, 1., 0.75, 1.])
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    for (width, height) in [(1, 1), (3, 1), (1, 3)] {
        for sx in [-1isize, 1] {
            for sy in [-1isize, 1] {
                let px = 4usize;
                let py = width * px + 1;
                let qx = 2usize;
                let qy = width * qx + 1;
                let plen = (width - 1) * px + (height - 1) * py + 3;
                let qlen = (width - 1) * qx + (height - 1) * qy + 1;
                let packed = PackedImageLayout {
                    width,
                    height,
                    origin: if sx < 0 { (width - 1) * px } else { 0 }
                        + if sy < 0 { (height - 1) * py } else { 0 },
                    channel_stride: 1,
                    x_stride: sx * px as isize,
                    y_stride: sy * py as isize,
                    bit_depth: BitDepth::Uint8,
                    channel_order: ChannelOrdering::Bgr,
                };
                let planar = SignedPlanarByteLayout {
                    width,
                    height,
                    origin: if sx > 0 { (width - 1) * qx } else { 0 }
                        + if sy > 0 { (height - 1) * qy } else { 0 },
                    x_stride: -sx * qx as isize,
                    y_stride: -sy * qy as isize,
                    bit_depth: BitDepth::Uint8,
                };
                let input = vec![50u8; plen];
                let padded_input = vec![50u8; plen + 32];
                let mut actual = [vec![99u8; qlen], vec![99; qlen], vec![99; qlen]];
                let mut expected = [
                    vec![99u8; qlen + 32],
                    vec![99; qlen + 32],
                    vec![99; qlen + 32],
                ];
                let [r, g, b] = &mut actual;
                cpu.try_convert_packed_to_planar(
                    &input,
                    packed,
                    SignedPlanarBytesMut {
                        red: r,
                        green: g,
                        blue: b,
                        alpha: None,
                        layout: planar,
                    },
                )
                .unwrap();
                let [r, g, b] = &mut expected;
                cpu.try_convert_packed_to_planar(
                    &padded_input,
                    PackedImageLayout {
                        origin: packed.origin + 16,
                        ..packed
                    },
                    SignedPlanarBytesMut {
                        red: r,
                        green: g,
                        blue: b,
                        alpha: None,
                        layout: SignedPlanarByteLayout {
                            origin: planar.origin + 16,
                            ..planar
                        },
                    },
                )
                .unwrap();
                for (a, b) in actual.iter().zip(expected.iter()) {
                    assert_eq!(&a[..], &b[16..16 + qlen]);
                }

                let planes = vec![50u8; qlen];
                let padded_planes = vec![50u8; qlen + 32];
                let mut output = vec![99u8; plen];
                let mut padded_output = vec![99u8; plen + 32];
                cpu.try_convert_planar_to_packed(
                    SignedPlanarBytes {
                        red: &planes,
                        green: &planes,
                        blue: &planes,
                        alpha: None,
                        layout: planar,
                    },
                    &mut output,
                    packed,
                )
                .unwrap();
                cpu.try_convert_planar_to_packed(
                    SignedPlanarBytes {
                        red: &padded_planes,
                        green: &padded_planes,
                        blue: &padded_planes,
                        alpha: None,
                        layout: SignedPlanarByteLayout {
                            origin: planar.origin + 16,
                            ..planar
                        },
                    },
                    &mut padded_output,
                    PackedImageLayout {
                        origin: packed.origin + 16,
                        ..packed
                    },
                )
                .unwrap();
                assert_eq!(&output[..], &padded_output[16..16 + plen]);
            }
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn mixed_packed_planar_conversion_and_independent_alpha() {
    use ocio_rs::{
        ChannelOrdering, PackedImageLayout, SignedPlanarByteLayout, SignedPlanarBytes,
        SignedPlanarBytesMut,
    };
    let _guard = cpu_processor_test_lock();
    let cpu = scaled_processor([0.5, 1.0, 1.0, 1.0])
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    let packed_layout = PackedImageLayout {
        width: 2,
        height: 1,
        origin: 0,
        channel_stride: 1,
        x_stride: 4,
        y_stride: 8,
        bit_depth: BitDepth::Uint8,
        channel_order: ChannelOrdering::Rgba,
    };
    let planar_layout = SignedPlanarByteLayout {
        width: 2,
        height: 1,
        origin: 0,
        x_stride: 1,
        y_stride: 2,
        bit_depth: BitDepth::Uint8,
    };
    let input = [100, 50, 25, 128, 200, 80, 40, 64];
    let (mut red, mut green, mut blue) = ([0; 2], [0; 2], [0; 2]);
    cpu.try_convert_packed_to_planar(
        &input,
        packed_layout,
        SignedPlanarBytesMut {
            red: &mut red,
            green: &mut green,
            blue: &mut blue,
            alpha: None,
            layout: planar_layout,
        },
    )
    .unwrap();
    assert_eq!(red, [50, 100]);
    assert_eq!(green, [50, 80]);
    assert_eq!(blue, [25, 40]);

    let mut output = [0; 8];
    cpu.try_convert_planar_to_packed(
        SignedPlanarBytes {
            red: &red,
            green: &green,
            blue: &blue,
            alpha: None,
            layout: planar_layout,
        },
        &mut output,
        packed_layout,
    )
    .unwrap();
    let missing_alpha = if ocio_rs::version_hex() >= 0x02060000 {
        255
    } else {
        0
    };
    assert_eq!(
        output,
        [25, 50, 25, missing_alpha, 50, 80, 40, missing_alpha]
    );

    let mut alpha = [0; 2];
    let mut r = [0; 2];
    let mut g = [0; 2];
    let mut b = [0; 2];
    cpu.try_apply_planar_signed_convert(
        SignedPlanarBytes {
            red: &red,
            green: &green,
            blue: &blue,
            alpha: None,
            layout: planar_layout,
        },
        SignedPlanarBytesMut {
            red: &mut r,
            green: &mut g,
            blue: &mut b,
            alpha: Some(&mut alpha),
            layout: planar_layout,
        },
    )
    .unwrap();
    assert_eq!(alpha, [missing_alpha, missing_alpha]);
    let mut tiny_alpha = [0; 1];
    assert!(cpu
        .try_convert_packed_to_planar(
            &input,
            packed_layout,
            SignedPlanarBytesMut {
                red: &mut r,
                green: &mut g,
                blue: &mut b,
                alpha: Some(&mut tiny_alpha),
                layout: planar_layout,
            }
        )
        .is_err());
    assert!(cpu
        .try_convert_planar_to_packed(
            SignedPlanarBytes {
                red: &red,
                green: &green,
                blue: &blue,
                alpha: None,
                layout: planar_layout,
            },
            &mut output[..4],
            packed_layout
        )
        .is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn mixed_conversion_handles_negative_stride_and_bit_depth_change() {
    use ocio_rs::{ChannelOrdering, PackedImageLayout, SignedPlanarByteLayout, SignedPlanarBytes};
    let _guard = cpu_processor_test_lock();
    let cpu = scaled_processor([1.0; 4])
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    let layout = SignedPlanarByteLayout {
        width: 2,
        height: 1,
        origin: 2,
        x_stride: -1,
        y_stride: 2,
        bit_depth: BitDepth::Uint8,
    };
    let output_layout = PackedImageLayout {
        width: 2,
        height: 1,
        origin: 0,
        channel_stride: 2,
        x_stride: 8,
        y_stride: 16,
        bit_depth: BitDepth::Uint16,
        channel_order: ChannelOrdering::Abgr,
    };
    let mut output = [0u16; 11];
    // SAFETY: the byte slice spans exactly the exclusively borrowed words.
    let output_bytes = unsafe {
        std::slice::from_raw_parts_mut(output.as_mut_ptr().cast(), std::mem::size_of_val(&output))
    };
    cpu.try_convert_planar_to_packed(
        SignedPlanarBytes {
            red: &[99, 10, 20],
            green: &[99, 30, 40],
            blue: &[99, 3, 4],
            alpha: None,
            layout,
        },
        output_bytes,
        output_layout,
    )
    .unwrap();
    let missing_alpha = if ocio_rs::version_hex() >= 0x02060000 {
        65535
    } else {
        0
    };
    assert_eq!(
        &output[..8],
        [
            missing_alpha,
            4 * 257,
            40 * 257,
            20 * 257,
            missing_alpha,
            3 * 257,
            30 * 257,
            10 * 257
        ]
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn packed_abgr_negative_pixel_stride_keeps_padding() {
    use ocio_rs::{ChannelOrdering, PackedImageLayout};
    let processor = scaled_processor([0.5, 1.0, 1.0, 1.0]).unwrap();
    let cpu = processor
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    let layout = PackedImageLayout {
        width: 2,
        height: 1,
        origin: 8,
        channel_stride: 1,
        x_stride: -4,
        y_stride: 8,
        bit_depth: BitDepth::Uint8,
        channel_order: ChannelOrdering::Abgr,
    };
    let mut data = [99, 99, 99, 99, 255, 10, 20, 100, 128, 30, 40, 200];
    cpu.try_apply_packed_image(&mut data, layout).unwrap();
    assert_eq!(data, [99, 99, 99, 99, 255, 10, 20, 50, 128, 30, 40, 100]);
    assert!(cpu
        .try_apply_packed_image(
            &mut data[4..],
            PackedImageLayout {
                origin: 3,
                ..layout
            }
        )
        .is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn packed_unpadded_in_place_matches_padded_all_orders_and_signs() {
    use ocio_rs::{ChannelOrdering, PackedImageLayout};
    let cpu = scaled_processor([0.5, 1.0, 0.75, 1.0])
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    for (order, channels) in [
        (ChannelOrdering::Rgba, 4),
        (ChannelOrdering::Bgra, 4),
        (ChannelOrdering::Abgr, 4),
        (ChannelOrdering::Rgb, 3),
        (ChannelOrdering::Bgr, 3),
    ] {
        for (width, height) in [(1, 1), (3, 1), (1, 3), (2, 2)] {
            let channel_stride = 2;
            let xs = channels * channel_stride + 2;
            let ys = width * xs + 2;
            let dx = (width - 1) * xs;
            let dy = (height - 1) * ys;
            let pixel_size = (channels - 1) * channel_stride + 1;
            let len = dx + dy + pixel_size;
            let margin = xs + channels * channel_stride;
            for x_sign in [-1isize, 1] {
                for y_sign in [-1isize, 1] {
                    let origin = if x_sign < 0 { dx } else { 0 } + if y_sign < 0 { dy } else { 0 };
                    let layout = PackedImageLayout {
                        width,
                        height,
                        origin,
                        channel_stride,
                        x_stride: x_sign * xs as isize,
                        y_stride: y_sign * ys as isize,
                        bit_depth: BitDepth::Uint8,
                        channel_order: order,
                    };
                    let mut image = vec![80u8; len];
                    let mut padded = vec![99u8; len + 2 * margin];
                    padded[margin..margin + len].copy_from_slice(&image);
                    cpu.try_apply_packed_image(&mut image, layout).unwrap();
                    cpu.try_apply_packed_image(
                        &mut padded,
                        PackedImageLayout {
                            origin: origin + margin,
                            ..layout
                        },
                    )
                    .unwrap();
                    assert_eq!(
                        &image[..],
                        &padded[margin..margin + len],
                        "{order:?} {width}x{height} {x_sign}/{y_sign}"
                    );
                }
            }
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn nonlinear_packed_tail_matches_padded_native_result() {
    use ocio_rs::{
        transform::ExponentTransform, ChannelOrdering, Config, PackedImageLayout,
        TransformDirection,
    };
    let transform = ExponentTransform::create().unwrap();
    transform.set_value(&[2.2, 1.8, 1.4, 1.0]).unwrap();
    let cpu = Config::raw()
        .unwrap()
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    fn bytes(values: &mut [f32]) -> &mut [u8] {
        // SAFETY: the byte view covers the same exclusive float allocation.
        unsafe {
            std::slice::from_raw_parts_mut(
                values.as_mut_ptr().cast(),
                std::mem::size_of_val(values),
            )
        }
    }
    let layout = PackedImageLayout {
        width: 2,
        height: 2,
        origin: 0,
        channel_stride: 8,
        x_stride: 36,
        y_stride: 76,
        bit_depth: BitDepth::F32,
        channel_order: ChannelOrdering::Bgra,
    };
    for order in [ChannelOrdering::Bgra, ChannelOrdering::Rgba] {
        let layout = PackedImageLayout {
            channel_order: order,
            ..layout
        };
        let mut image = [0.5f32; 35];
        let mut reference = [0.5f32; 48];
        cpu.try_apply_packed_image(bytes(&mut image), layout)
            .unwrap();
        cpu.try_apply_packed_image(bytes(&mut reference), layout)
            .unwrap();
        for (index, (actual, expected)) in image.iter().zip(reference.iter()).enumerate() {
            assert_close(f64::from(*actual), f64::from(*expected), 1e-6);
            assert!(actual.is_finite(), "non-finite value at {index}, {order:?}");
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn packed_unpadded_in_place_preserves_u16_and_f16_alignment() {
    use ocio_rs::{ChannelOrdering, PackedImageLayout};
    let processor = scaled_processor([0.5, 1.0, 0.75, 1.0]).unwrap();
    fn bytes(words: &mut [u16]) -> &mut [u8] {
        // SAFETY: the byte view covers the same exclusive word slice.
        unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words))
        }
    }
    for depth in [BitDepth::Uint16, BitDepth::F16] {
        let cpu = processor
            .optimized_cpu_processor_bit_depth(depth, depth, 0)
            .unwrap();
        let layout = PackedImageLayout {
            width: 2,
            height: 2,
            origin: 0,
            channel_stride: 4,
            x_stride: 18,
            y_stride: 38,
            bit_depth: depth,
            channel_order: ChannelOrdering::Bgra,
        };
        let mut image = [0x3800u16; 35];
        let mut padded = [0xaaaau16; 69];
        padded[17..52].copy_from_slice(&image);
        cpu.try_apply_packed_image(bytes(&mut image), layout)
            .unwrap();
        cpu.try_apply_packed_image(
            bytes(&mut padded),
            PackedImageLayout {
                origin: 34,
                ..layout
            },
        )
        .unwrap();
        assert_eq!(&image[..], &padded[17..52], "{depth:?}");
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn packed_channel_orders_and_signed_strides_preserve_pixel_and_padding() {
    use ocio_rs::{ChannelOrdering, PackedImageLayout};
    let _guard = cpu_processor_test_lock();
    let cpu = scaled_processor([0.5, 1.0, 1.0, 1.0])
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    for (order, red_offset, channels) in [
        (ChannelOrdering::Rgba, 0usize, 4usize),
        (ChannelOrdering::Bgra, 2, 4),
        (ChannelOrdering::Abgr, 3, 4),
        (ChannelOrdering::Rgb, 0, 3),
        (ChannelOrdering::Bgr, 2, 3),
    ] {
        for x_sign in [-1isize, 1] {
            for y_sign in [-1isize, 1] {
                let x_stride = x_sign * 6;
                let y_stride = y_sign * 16;
                let origin = (24 + if x_sign < 0 { 6 } else { 0 } + if y_sign < 0 { 16 } else { 0 })
                    as usize;
                let layout = PackedImageLayout {
                    width: 2,
                    height: 2,
                    origin,
                    channel_stride: 1,
                    x_stride,
                    y_stride,
                    bit_depth: BitDepth::Uint8,
                    channel_order: order,
                };
                let mut data = [231u8; 80];
                let mut expected = data;
                for y in 0..2isize {
                    for x in 0..2isize {
                        let start = (origin as isize + x * x_stride + y * y_stride) as usize;
                        for c in 0..channels {
                            data[start + c] = if c == red_offset { 100 } else { 60 + c as u8 };
                            expected[start + c] = if c == red_offset { 50 } else { 60 + c as u8 };
                        }
                    }
                }
                cpu.try_apply_packed_image(&mut data, layout).unwrap();
                assert_eq!(
                    data, expected,
                    "order={order:?}, x={x_stride}, y={y_stride}"
                );
            }
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn cpu_rgba_pixels_stride_preserves_padding_behavior() {
    let _guard = cpu_processor_test_lock();
    if is_stub() {
        return;
    }

    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");

    // Two RGBA pixels with one padding float after each pixel (stride = 5).
    let mut rgba = vec![
        0.25f32, 0.5, 0.75, 1.0, 99.0, //
        0.4, 0.2, 0.8, 1.0, 77.0,
    ];

    cpu.apply_rgba_pixels(&mut rgba, 2, 5);

    assert_close(rgba[0] as f64, 0.5, 1e-6);
    assert_close(rgba[1] as f64, 0.5, 1e-6);
    assert_close(rgba[2] as f64, 0.375, 1e-6);
    assert_close(rgba[3] as f64, 1.0, 1e-6);
    assert_close(rgba[4] as f64, 99.0, 1e-6);

    assert_close(rgba[5] as f64, 0.8, 1e-6);
    assert_close(rgba[6] as f64, 0.2, 1e-6);
    assert_close(rgba[7] as f64, 0.4, 1e-6);
    assert_close(rgba[8] as f64, 1.0, 1e-6);
    assert_close(rgba[9] as f64, 77.0, 1e-6);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rgb_tail_without_padding_matches_native_alpha_semantics() {
    use ocio_rs::{transform::MatrixTransform, Config, TransformDirection};
    let matrix = MatrixTransform::create().unwrap();
    matrix
        .set_matrix(&[
            1., 0., 0., 1., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ])
        .unwrap();
    let cpu = Config::raw()
        .unwrap()
        .processor_from_transform(&matrix, TransformDirection::Forward)
        .unwrap()
        .default_cpu_processor()
        .unwrap();

    let mut short = [0.2, 0.3, 0.4, 99., 99., 0.5, 0.6, 0.7];
    let mut padded = [0.2, 0.3, 0.4, 99., 99., 0.5, 0.6, 0.7, 99., 99., 99., 99.];
    cpu.try_apply_rgb_pixels(&mut short, 2, 5).unwrap();
    cpu.try_apply_rgb_pixels(&mut padded, 2, 5).unwrap();
    assert_eq!(short, padded[..8]);
    assert_eq!(&padded[8..], &[99.; 4]);
    let alpha = if ocio_rs::version_hex() >= 0x02060000 {
        1.
    } else {
        0.
    };
    assert_close(short[0] as f64, 0.2 + alpha, 1e-6);
    assert_close(short[5] as f64, 0.5 + alpha, 1e-6);

    let mut single = [0.2, 0.3, 0.4];
    cpu.try_apply_rgb_pixels(&mut single, 1, 0).unwrap();
    assert_close(single[0] as f64, 0.2 + alpha, 1e-6);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn packed_rgb_short_tail_matches_padded_native_buffer() {
    use ocio_rs::{transform::MatrixTransform, BitDepth, Config, TransformDirection};
    let processor = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[0.5, 1.0, 0.75, 1.0]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap();
    let cpu = processor
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    let mut short = [50, 80, 120, 99, 160, 180, 200];
    let mut padded = [50, 80, 120, 99, 160, 180, 200, 99, 99, 99];
    cpu.try_apply_rgb_packed_bit_depth(&mut short, BitDepth::Uint8, 2, 4)
        .unwrap();
    cpu.try_apply_rgb_packed_bit_depth(&mut padded, BitDepth::Uint8, 2, 4)
        .unwrap();
    assert_eq!(short, padded[..7]);
    assert_eq!(&padded[7..], &[99; 3]);

    fn bytes(words: &mut [u16]) -> &mut [u8] {
        // SAFETY: the byte view covers the same exclusively borrowed storage.
        unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words))
        }
    }
    for depth in [BitDepth::Uint16, BitDepth::F16] {
        let cpu = processor
            .optimized_cpu_processor_bit_depth(depth, depth, 0)
            .unwrap();
        let mut short = [0x3800, 0x3a00, 0x3c00, 0x7bff, 0x3400, 0x3800, 0x3a00];
        let mut padded = [
            0x3800, 0x3a00, 0x3c00, 0x7bff, 0x3400, 0x3800, 0x3a00, 0x7bff, 0x7bff, 0x7bff,
        ];
        cpu.try_apply_rgb_packed_bit_depth(bytes(&mut short), depth, 2, 4)
            .unwrap();
        cpu.try_apply_rgb_packed_bit_depth(bytes(&mut padded), depth, 2, 4)
            .unwrap();
        assert_eq!(short, padded[..7], "{depth:?}");
        assert_eq!(&padded[7..], &[0x7bff; 3]);
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rgba_short_tail_matches_padded_for_float_and_integer() {
    use ocio_rs::{transform::MatrixTransform, BitDepth, Config, TransformDirection};
    let processor = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[0.5, 1.0, 0.75, 0.5]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap();
    let cpu = processor.default_cpu_processor().unwrap();
    let mut short = [0.2, 0.4, 0.6, 0.8, 99., 0.4, 0.6, 0.8, 1.0];
    let mut padded = [
        0.2, 0.4, 0.6, 0.8, 99., 0.4, 0.6, 0.8, 1.0, 99., 99., 99., 99.,
    ];
    cpu.try_apply_rgba_pixels(&mut short, 2, 5).unwrap();
    cpu.try_apply_rgba_pixels(&mut padded, 2, 5).unwrap();
    assert_eq!(short, padded[..9]);
    assert_eq!(&padded[9..], &[99.; 4]);
    let mut one = [0.2, 0.4, 0.6, 0.8];
    cpu.try_apply_rgba_pixels(&mut one, 1, 5).unwrap();
    assert_eq!(one, short[..4]);
    #[cfg(target_pointer_width = "64")]
    {
        let mut huge_stride = [0.2, 0.4, 0.6, 0.8];
        cpu.try_apply_rgba_pixels(&mut huge_stride, 1, i64::from(i32::MAX) / 4 + 1)
            .unwrap();
        assert_eq!(huge_stride, one);
        assert!(cpu.try_apply_rgba_pixels(&mut huge_stride, 1, 3).is_err());
    }

    let cpu = processor
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    let mut short = [20, 40, 60, 80, 99, 100, 120, 140, 160];
    let mut padded = [20, 40, 60, 80, 99, 100, 120, 140, 160, 99, 99, 99, 99];
    cpu.try_apply_rgba_packed_bit_depth(&mut short, BitDepth::Uint8, 2, 5)
        .unwrap();
    cpu.try_apply_rgba_packed_bit_depth(&mut padded, BitDepth::Uint8, 2, 5)
        .unwrap();
    assert_eq!(short, padded[..9]);
    assert_eq!(&padded[9..], &[99; 4]);
    #[cfg(target_pointer_width = "64")]
    {
        let mut huge_stride = [20, 40, 60, 80];
        cpu.try_apply_rgba_packed_bit_depth(
            &mut huge_stride,
            BitDepth::Uint8,
            1,
            i64::from(i32::MAX) + 1,
        )
        .unwrap();
        assert_eq!(huge_stride, short[..4]);
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn cpu_rgb_pixels_stride_preserves_padding_behavior() {
    let _guard = cpu_processor_test_lock();
    if is_stub() {
        return;
    }

    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");

    // Two RGB pixels with one padding float after each pixel (stride = 4).
    let mut rgb = vec![
        0.25f32, 0.5, 0.75, 99.0, //
        0.4, 0.2, 0.8, 77.0,
    ];

    cpu.apply_rgb_pixels(&mut rgb, 2, 4);

    assert_close(rgb[0] as f64, 0.5, 1e-6);
    assert_close(rgb[1] as f64, 0.5, 1e-6);
    assert_close(rgb[2] as f64, 0.375, 1e-6);
    assert_close(rgb[3] as f64, 99.0, 1e-6);

    assert_close(rgb[4] as f64, 0.8, 1e-6);
    assert_close(rgb[5] as f64, 0.2, 1e-6);
    assert_close(rgb[6] as f64, 0.4, 1e-6);
    assert_close(rgb[7] as f64, 77.0, 1e-6);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn cpu_rgba_packed_f32_matches_rgba_path_behavior() {
    let _guard = cpu_processor_test_lock();
    if is_stub() {
        return;
    }

    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");

    let packed_input = vec![0.25f32, 0.5, 0.75, 1.0, 0.4, 0.2, 0.8, 1.0];
    let mut packed_bytes = f32s_to_bytes(&packed_input);

    cpu.apply_rgba_packed_bit_depth(&mut packed_bytes, BitDepth::F32, 2, 4);

    let packed_output = bytes_to_f32s(&packed_bytes);
    assert_eq!(packed_output.len(), packed_input.len());
    assert_close(packed_output[0] as f64, 0.5, 1e-6);
    assert_close(packed_output[1] as f64, 0.5, 1e-6);
    assert_close(packed_output[2] as f64, 0.375, 1e-6);
    assert_close(packed_output[3] as f64, 1.0, 1e-6);
    assert_close(packed_output[4] as f64, 0.8, 1e-6);
    assert_close(packed_output[5] as f64, 0.2, 1e-6);
    assert_close(packed_output[6] as f64, 0.4, 1e-6);
    assert_close(packed_output[7] as f64, 1.0, 1e-6);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn cpu_rgb_packed_f32_matches_rgb_path_behavior() {
    let _guard = cpu_processor_test_lock();
    if is_stub() {
        return;
    }

    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");

    let packed_input = vec![0.25f32, 0.5, 0.75, 0.4, 0.2, 0.8];
    let mut packed_bytes = f32s_to_bytes(&packed_input);

    cpu.apply_rgb_packed_bit_depth(&mut packed_bytes, BitDepth::F32, 2, 3);

    let packed_output = bytes_to_f32s(&packed_bytes);
    assert_eq!(packed_output.len(), packed_input.len());
    assert_close(packed_output[0] as f64, 0.5, 1e-6);
    assert_close(packed_output[1] as f64, 0.5, 1e-6);
    assert_close(packed_output[2] as f64, 0.375, 1e-6);
    assert_close(packed_output[3] as f64, 0.8, 1e-6);
    assert_close(packed_output[4] as f64, 0.2, 1e-6);
    assert_close(packed_output[5] as f64, 0.4, 1e-6);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn cpu_packed_uint8_requires_matching_finalization_behavior() {
    let _guard = cpu_processor_test_lock();
    if is_stub() {
        return;
    }

    let processor = scaled_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled processor");
    let default_cpu = processor
        .default_cpu_processor()
        .expect("default CPU processor");

    let mut rgba = vec![64u8, 128, 192, 255, 102, 51, 204, 255];
    let default_error = default_cpu
        .try_apply_rgba_packed_bit_depth(&mut rgba, BitDepth::Uint8, 2, 4)
        .expect_err("F32-finalized processor must reject uint8 pixels");
    assert!(matches!(default_error, ocio_rs::OcioError::Ocio(_)));

    assert_eq!(rgba, vec![64, 128, 192, 255, 102, 51, 204, 255]);
}

#[test]
#[should_panic(expected = "CPUProcessor::apply_rgba_pixels: buffer too small")]
fn cpu_rgba_pixels_short_buffer_panics() {
    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");
    let mut rgba = vec![0.0f32; 7];
    cpu.apply_rgba_pixels(&mut rgba, 2, 4);
}

#[test]
#[should_panic(expected = "CPUProcessor::apply_rgb_packed_bit_depth: buffer too small")]
fn cpu_rgb_packed_short_buffer_panics() {
    let cpu = scaled_cpu_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled cpu processor");
    let mut packed_bytes = vec![0u8; 20];
    cpu.apply_rgb_packed_bit_depth(&mut packed_bytes, BitDepth::F32, 2, 3);
}

#[test]
#[should_panic(expected = "CPUProcessor::apply_rgba_pixels: buffer too small")]
fn processor_rgba_pixels_short_buffer_panics() {
    let processor = scaled_processor([2.0, 1.0, 0.5, 1.0]).expect("scaled processor");
    let cpu = processor
        .default_cpu_processor()
        .expect("default cpu processor");
    let mut rgba = vec![0.0f32; 7];
    cpu.apply_rgba_pixels(&mut rgba, 2, 4);
}
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn planar_float_channels_are_processed_in_place() {
    use ocio_rs::{transform::MatrixTransform, Config, TransformDirection};
    let transform = MatrixTransform::scale(&[2.0, 1.0, 0.5, 1.0]).unwrap();
    let cpu = Config::raw()
        .unwrap()
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    let mut red = [0.25, 0.5, 1.0, 0.1];
    let mut green = [0.5; 4];
    let mut blue = [0.75; 4];
    let mut alpha = [0.25, 0.5, 0.75, 1.0];
    cpu.try_apply_planar_f32(&mut red, &mut green, &mut blue, Some(&mut alpha), 2, 2)
        .unwrap();
    assert_eq!(red, [0.5, 1.0, 2.0, 0.2]);
    assert_eq!(green, [0.5; 4]);
    assert_eq!(blue, [0.375; 4]);
    assert_eq!(alpha, [0.25, 0.5, 0.75, 1.0]);
    assert!(cpu
        .try_apply_planar_f32(&mut red, &mut green[..3], &mut blue, None, 2, 2)
        .is_err());
    cpu.try_apply_planar_f32(&mut red, &mut green, &mut blue, None, 4, 1)
        .unwrap();
    assert_eq!(red, [1.0, 2.0, 4.0, 0.4]);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn planar_strides_preserve_padding_and_reject_overflow() {
    use ocio_rs::{transform::MatrixTransform, Config, PlanarF32Layout, TransformDirection};
    let cpu = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[2.0, 1.0, 0.5, 1.0]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    let layout = PlanarF32Layout {
        width: 2,
        height: 2,
        x_stride: 2,
        y_stride: 5,
    };
    let mut r = [1., 99., 2., 99., 99., 3., 99., 4., 99.];
    let mut g = [1., 99., 2., 99., 99., 3., 99., 4., 99.];
    let mut b = [1., 99., 2., 99., 99., 3., 99., 4., 99.];
    let mut short_r = r;
    let mut short_g = g;
    let mut short_b = b;
    cpu.try_apply_planar_strided_f32(
        &mut short_r[..8],
        &mut short_g[..8],
        &mut short_b[..8],
        None,
        layout,
    )
    .unwrap();
    cpu.try_apply_planar_strided_f32(&mut r, &mut g, &mut b, None, layout)
        .unwrap();
    assert_eq!(r, [2., 99., 4., 99., 99., 6., 99., 8., 99.]);
    assert_eq!(g, [1., 99., 2., 99., 99., 3., 99., 4., 99.]);
    assert_eq!(b, [0.5, 99., 1., 99., 99., 1.5, 99., 2., 99.]);
    assert_eq!(&short_r[..8], &r[..8]);
    assert_eq!(&short_g[..8], &g[..8]);
    assert_eq!(&short_b[..8], &b[..8]);
    for (width, height) in [(1, 1), (3, 1), (1, 3), (3, 2)] {
        let x = 3;
        let y = width * x + 2;
        let end = (height - 1) * y + (width - 1) * x + 1;
        let full = (height - 1) * y + width * x;
        let mut planes = [
            vec![0.4; full],
            vec![0.5; full],
            vec![0.6; full],
            vec![0.7; full],
        ];
        let mut expected = planes.clone();
        let [r, g, b, a] = &mut planes;
        let layout = PlanarF32Layout {
            width,
            height,
            x_stride: x,
            y_stride: y,
        };
        cpu.try_apply_planar_strided_f32(
            &mut r[..end],
            &mut g[..end],
            &mut b[..end],
            Some(&mut a[..end]),
            layout,
        )
        .unwrap();
        let [r, g, b, a] = &mut expected;
        cpu.try_apply_planar_strided_f32(r, g, b, Some(a), layout)
            .unwrap();
        for (actual, expected) in planes.iter().zip(expected.iter()) {
            assert_eq!(&actual[..end], &expected[..end], "{width}x{height}");
        }
    }
    assert!(cpu
        .try_apply_planar_strided_f32(
            &mut r,
            &mut g,
            &mut b,
            None,
            PlanarF32Layout {
                y_stride: 2,
                ..layout
            }
        )
        .is_err());
    assert!(cpu
        .try_apply_planar_strided_f32(
            &mut r,
            &mut g,
            &mut b,
            None,
            PlanarF32Layout {
                width: usize::MAX,
                ..layout
            }
        )
        .is_err());
    assert!(cpu
        .try_apply_planar_strided_f32(&mut r[..7], &mut g, &mut b, None, layout)
        .is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn planar_u8_uses_explicit_native_cpu_bit_depth() {
    use ocio_rs::{
        transform::MatrixTransform, BitDepth, Config, PlanarByteLayout, TransformDirection,
    };
    let processor = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[2.0, 1.0, 0.5, 1.0]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap();
    let cpu = processor
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    let layout = PlanarByteLayout {
        width: 2,
        height: 2,
        x_stride: 2,
        y_stride: 5,
        bit_depth: BitDepth::Uint8,
    };
    let mut red = [50, 255, 100, 255, 255, 25, 255, 60, 255];
    let mut green = [40, 255, 50, 255, 255, 60, 255, 70, 255];
    let mut blue = [100, 255, 200, 255, 255, 40, 255, 80, 255];
    let mut short_r = red;
    let mut short_g = green;
    let mut short_b = blue;
    cpu.try_apply_planar_bytes(
        &mut short_r[..8],
        &mut short_g[..8],
        &mut short_b[..8],
        None,
        layout,
    )
    .unwrap();
    cpu.try_apply_planar_bytes(&mut red, &mut green, &mut blue, None, layout)
        .unwrap();
    assert_eq!(red, [100, 255, 200, 255, 255, 50, 255, 120, 255]);
    assert_eq!(green, [40, 255, 50, 255, 255, 60, 255, 70, 255]);
    assert_eq!(blue, [50, 255, 100, 255, 255, 20, 255, 40, 255]);
    assert_eq!(&short_r[..8], &red[..8]);
    assert_eq!(&short_g[..8], &green[..8]);
    assert_eq!(&short_b[..8], &blue[..8]);
    for (width, height) in [(1, 1), (3, 1), (1, 3), (3, 2)] {
        let x = 3;
        let y = width * x + 2;
        let end = (height - 1) * y + (width - 1) * x + 1;
        let full = (height - 1) * y + width * x;
        let mut planes = [
            vec![40; full],
            vec![50; full],
            vec![60; full],
            vec![70; full],
        ];
        let mut expected = planes.clone();
        let [r, g, b, a] = &mut planes;
        let layout = PlanarByteLayout {
            width,
            height,
            x_stride: x,
            y_stride: y,
            bit_depth: BitDepth::Uint8,
        };
        cpu.try_apply_planar_bytes(
            &mut r[..end],
            &mut g[..end],
            &mut b[..end],
            Some(&mut a[..end]),
            layout,
        )
        .unwrap();
        let [r, g, b, a] = &mut expected;
        cpu.try_apply_planar_bytes(r, g, b, Some(a), layout)
            .unwrap();
        for (actual, expected) in planes.iter().zip(expected.iter()) {
            assert_eq!(&actual[..end], &expected[..end], "{width}x{height}");
        }
    }
    fn words_as_bytes(words: &mut [u16]) -> &mut [u8] {
        // SAFETY: the bytes cover the same exclusively borrowed word slice.
        unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words))
        }
    }
    for depth in [BitDepth::Uint16, BitDepth::F16] {
        let cpu = processor
            .optimized_cpu_processor_bit_depth(depth, depth, 0)
            .unwrap();
        let mut planes = [[0x3800u16; 9]; 4];
        let mut expected = planes;
        let layout = PlanarByteLayout {
            width: 2,
            height: 2,
            x_stride: 4,
            y_stride: 10,
            bit_depth: depth,
        };
        let [r, g, b, a] = &mut planes;
        cpu.try_apply_planar_bytes(
            words_as_bytes(&mut r[..8]),
            words_as_bytes(&mut g[..8]),
            words_as_bytes(&mut b[..8]),
            Some(words_as_bytes(&mut a[..8])),
            layout,
        )
        .unwrap();
        let [r, g, b, a] = &mut expected;
        cpu.try_apply_planar_bytes(
            words_as_bytes(r),
            words_as_bytes(g),
            words_as_bytes(b),
            Some(words_as_bytes(a)),
            layout,
        )
        .unwrap();
        for (actual, expected) in planes.iter().zip(expected.iter()) {
            assert_eq!(&actual[..8], &expected[..8], "{depth:?}");
        }
    }
    let incorrect = PlanarByteLayout {
        bit_depth: BitDepth::F32,
        x_stride: 4,
        y_stride: 8,
        width: 1,
        height: 1,
    };
    assert!(cpu
        .try_apply_planar_bytes(&mut red, &mut green, &mut blue, None, incorrect)
        .is_err());
    let f32_cpu = processor.default_cpu_processor().unwrap();
    assert!(f32_cpu
        .try_apply_planar_bytes(&mut red, &mut green, &mut blue, None, layout)
        .is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn signed_planar_unpadded_matches_padded_for_all_stride_signs() {
    use ocio_rs::{
        transform::MatrixTransform, BitDepth, Config, SignedPlanarByteLayout, SignedPlanarBytesMut,
        TransformDirection,
    };
    let cpu = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[0.5, 1.0, 0.75, 1.0]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    for (width, height) in [(1, 1), (3, 1), (1, 3), (3, 2)] {
        for sx in [2_isize, -2] {
            for sy in [(width * 2 + 1) as isize, -((width * 2 + 1) as isize)] {
                let dx = (width - 1) * 2;
                let dy = (height - 1) * sy.unsigned_abs();
                let origin = if sx < 0 { dx } else { 0 } + if sy < 0 { dy } else { 0 };
                let len = dx + dy + 1;
                let layout = SignedPlanarByteLayout {
                    width,
                    height,
                    origin,
                    x_stride: sx,
                    y_stride: sy,
                    bit_depth: BitDepth::Uint8,
                };
                let mut short = [vec![40; len], vec![60; len], vec![80; len], vec![100; len]];
                let mut padded = [
                    vec![99; len + 4],
                    vec![99; len + 4],
                    vec![99; len + 4],
                    vec![99; len + 4],
                ];
                for (dst, src) in padded.iter_mut().zip(short.iter()) {
                    dst[2..2 + len].copy_from_slice(src);
                }
                let [r, g, b, a] = &mut short;
                cpu.try_apply_planar_signed(SignedPlanarBytesMut {
                    red: r,
                    green: g,
                    blue: b,
                    alpha: Some(a),
                    layout,
                })
                .unwrap();
                let [r, g, b, a] = &mut padded;
                cpu.try_apply_planar_signed(SignedPlanarBytesMut {
                    red: r,
                    green: g,
                    blue: b,
                    alpha: Some(a),
                    layout: SignedPlanarByteLayout {
                        origin: origin + 2,
                        ..layout
                    },
                })
                .unwrap();
                for (actual, expected) in short.iter().zip(padded.iter()) {
                    assert_eq!(
                        &actual[..],
                        &expected[2..2 + len],
                        "{width}x{height}, {sx}, {sy}"
                    );
                }
            }
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn signed_planar_conversion_unpadded_matches_all_stride_signs() {
    use ocio_rs::{
        transform::MatrixTransform, BitDepth, Config, SignedPlanarByteLayout, SignedPlanarBytes,
        SignedPlanarBytesMut, TransformDirection,
    };
    let cpu = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[0.5, 1., 0.75, 1.]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    fn bytes(words: &mut [u16]) -> &mut [u8] {
        // SAFETY: same exclusively borrowed allocation viewed as bytes.
        unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words))
        }
    }
    for ix in [2_isize, -2] {
        for iy in [5_isize, -5] {
            for ox in [4_isize, -4] {
                for oy in [10_isize, -10] {
                    for has_input_alpha in [false, true] {
                        let source_layout = SignedPlanarByteLayout {
                            width: 2,
                            height: 2,
                            origin: (if ix < 0 { 2 } else { 0 }) + (if iy < 0 { 5 } else { 0 }),
                            x_stride: ix,
                            y_stride: iy,
                            bit_depth: BitDepth::Uint8,
                        };
                        let target_layout = SignedPlanarByteLayout {
                            width: 2,
                            height: 2,
                            origin: (if ox < 0 { 4 } else { 0 }) + (if oy < 0 { 10 } else { 0 }),
                            x_stride: ox,
                            y_stride: oy,
                            bit_depth: BitDepth::Uint16,
                        };
                        let input = [50u8; 8];
                        let mut padded_input = [99u8; 12];
                        padded_input[2..10].copy_from_slice(&input);
                        let mut actual = [[0xaaaau16; 8]; 4];
                        let mut expected = [[0xaaaau16; 12]; 4];
                        let [r, g, b, a] = &mut actual;
                        cpu.try_apply_planar_signed_convert(
                            SignedPlanarBytes {
                                red: &input,
                                green: &input,
                                blue: &input,
                                alpha: has_input_alpha.then_some(&input[..]),
                                layout: source_layout,
                            },
                            SignedPlanarBytesMut {
                                red: bytes(r),
                                green: bytes(g),
                                blue: bytes(b),
                                alpha: Some(bytes(a)),
                                layout: target_layout,
                            },
                        )
                        .unwrap();
                        let [r, g, b, a] = &mut expected;
                        cpu.try_apply_planar_signed_convert(
                            SignedPlanarBytes {
                                red: &padded_input,
                                green: &padded_input,
                                blue: &padded_input,
                                alpha: has_input_alpha.then_some(&padded_input[..]),
                                layout: SignedPlanarByteLayout {
                                    origin: source_layout.origin + 2,
                                    ..source_layout
                                },
                            },
                            SignedPlanarBytesMut {
                                red: bytes(r),
                                green: bytes(g),
                                blue: bytes(b),
                                alpha: Some(bytes(a)),
                                layout: SignedPlanarByteLayout {
                                    origin: target_layout.origin + 4,
                                    ..target_layout
                                },
                            },
                        )
                        .unwrap();
                        for (a, b) in actual.iter().zip(expected.iter()) {
                            assert_eq!(
                                &a[..],
                                &b[2..10],
                                "{ix},{iy} -> {ox},{oy}, alpha={has_input_alpha}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn signed_planar_conversion_asymmetric_padding_and_shapes() {
    use ocio_rs::{
        transform::MatrixTransform, BitDepth, Config, SignedPlanarByteLayout, SignedPlanarBytes,
        SignedPlanarBytesMut, TransformDirection,
    };
    let cpu = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[0.5, 1., 0.75, 1.]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    fn bytes(words: &mut [u16]) -> &mut [u8] {
        // SAFETY: same exclusive allocation viewed as bytes.
        unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words))
        }
    }
    for (width, height) in [(1, 1), (3, 1), (1, 3), (3, 2)] {
        let x = 2usize;
        let y = width * x + 1;
        let len = (width - 1) * x + (height - 1) * y + 1;
        for (ix_neg, iy_neg, ox_neg, oy_neg) in [
            (true, false, false, true),
            (false, true, false, false),
            (false, false, true, true),
            (true, true, true, false),
        ] {
            let ix = if ix_neg { -(x as isize) } else { x as isize };
            let iy = if iy_neg { -(y as isize) } else { y as isize };
            let ox = if ox_neg {
                -(2 * x as isize)
            } else {
                2 * x as isize
            };
            let oy = if oy_neg {
                -(2 * y as isize)
            } else {
                2 * y as isize
            };
            let src_origin = (if ix_neg { (width - 1) * x } else { 0 })
                + (if iy_neg { (height - 1) * y } else { 0 });
            let dst_origin = (if ox_neg { (width - 1) * 2 * x } else { 0 })
                + (if oy_neg { (height - 1) * 2 * y } else { 0 });
            let input = vec![50u8; len + 4];
            for (src_pad, dst_pad, src_alpha, dst_alpha) in [
                (false, true, false, true),
                (true, false, true, false),
                (false, false, true, true),
            ] {
                let src_range = if src_pad { 0..len + 4 } else { 2..len + 2 };
                let dst_range = if dst_pad { 0..len + 4 } else { 2..len + 2 };
                let src_layout = SignedPlanarByteLayout {
                    width,
                    height,
                    origin: src_origin + if src_pad { 2 } else { 0 },
                    x_stride: ix,
                    y_stride: iy,
                    bit_depth: BitDepth::Uint8,
                };
                let dst_layout = SignedPlanarByteLayout {
                    width,
                    height,
                    origin: dst_origin + if dst_pad { 4 } else { 0 },
                    x_stride: ox,
                    y_stride: oy,
                    bit_depth: BitDepth::Uint16,
                };
                let src = &input[src_range];
                let mut actual = [
                    vec![0xaaaau16; len + 4],
                    vec![0xaaaa; len + 4],
                    vec![0xaaaa; len + 4],
                    vec![0xaaaa; len + 4],
                ];
                let mut reference = actual.clone();
                let [r, g, b, a] = &mut actual;
                cpu.try_apply_planar_signed_convert(
                    SignedPlanarBytes {
                        red: src,
                        green: src,
                        blue: src,
                        alpha: src_alpha.then_some(src),
                        layout: src_layout,
                    },
                    SignedPlanarBytesMut {
                        red: bytes(&mut r[dst_range.clone()]),
                        green: bytes(&mut g[dst_range.clone()]),
                        blue: bytes(&mut b[dst_range.clone()]),
                        alpha: dst_alpha.then(|| bytes(&mut a[dst_range])),
                        layout: dst_layout,
                    },
                )
                .unwrap();
                let [r, g, b, a] = &mut reference;
                cpu.try_apply_planar_signed_convert(
                    SignedPlanarBytes {
                        red: &input,
                        green: &input,
                        blue: &input,
                        alpha: src_alpha.then_some(&input[..]),
                        layout: SignedPlanarByteLayout {
                            origin: src_origin + 2,
                            ..src_layout
                        },
                    },
                    SignedPlanarBytesMut {
                        red: bytes(r),
                        green: bytes(g),
                        blue: bytes(b),
                        alpha: dst_alpha.then(|| bytes(a)),
                        layout: SignedPlanarByteLayout {
                            origin: dst_origin + 4,
                            ..dst_layout
                        },
                    },
                )
                .unwrap();
                for (a, b) in actual.iter().zip(reference.iter()) {
                    assert_eq!(&a[2..len+2], &b[2..len+2], "{width}x{height}, {ix},{iy} -> {ox},{oy}, pad={src_pad}/{dst_pad}, alpha={src_alpha}/{dst_alpha}");
                }
            }
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn signed_planar_unpadded_conversion_rejects_bad_layout_before_writing() {
    use ocio_rs::{
        transform::MatrixTransform, BitDepth, Config, SignedPlanarByteLayout, SignedPlanarBytes,
        SignedPlanarBytesMut, TransformDirection,
    };
    let cpu = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[0.5, 1., 0.75, 1.]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    let layout = SignedPlanarByteLayout {
        width: 2,
        height: 2,
        origin: 0,
        x_stride: 2,
        y_stride: 5,
        bit_depth: BitDepth::Uint8,
    };
    let input = [50u8; 8];
    let mut output = [[99u8; 8]; 3];
    let [r, g, b] = &mut output;
    assert!(cpu
        .try_apply_planar_signed_convert(
            SignedPlanarBytes {
                red: &input,
                green: &input,
                blue: &input,
                alpha: None,
                layout
            },
            SignedPlanarBytesMut {
                red: r,
                green: g,
                blue: b,
                alpha: None,
                layout
            },
        )
        .is_err());
    assert_eq!(output, [[99u8; 8]; 3]);
    let [r, g, b] = &mut output;
    assert!(cpu
        .try_apply_planar_signed_convert(
            SignedPlanarBytes {
                red: &input[..7],
                green: &input,
                blue: &input,
                alpha: None,
                layout
            },
            SignedPlanarBytesMut {
                red: r,
                green: g,
                blue: b,
                alpha: None,
                layout
            },
        )
        .is_err());
    assert_eq!(output, [[99u8; 8]; 3]);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn planar_conversion_without_pointer_padding_matches_padded() {
    use ocio_rs::{
        transform::MatrixTransform, BitDepth, Config, PlanarByteLayout, PlanarBytes,
        PlanarBytesMut, TransformDirection,
    };
    let cpu = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[0.5, 1.0, 0.75, 1.0]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    fn bytes(words: &mut [u16]) -> &mut [u8] {
        // SAFETY: same exclusive word allocation viewed as bytes.
        unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words))
        }
    }
    let input_layout = PlanarByteLayout {
        width: 2,
        height: 2,
        x_stride: 2,
        y_stride: 5,
        bit_depth: BitDepth::Uint8,
    };
    let output_layout = PlanarByteLayout {
        width: 2,
        height: 2,
        x_stride: 4,
        y_stride: 10,
        bit_depth: BitDepth::Uint16,
    };
    let input = [20, 99, 40, 99, 99, 60, 99, 80, 99];
    let mut short = [[0xaaaau16; 9]; 4];
    let mut padded = short;
    let [r, g, b, a] = &mut short;
    cpu.try_apply_planar_convert(
        PlanarBytes {
            red: &input[..8],
            green: &input[..8],
            blue: &input[..8],
            alpha: Some(&input[..8]),
            layout: input_layout,
        },
        PlanarBytesMut {
            red: bytes(&mut r[..8]),
            green: bytes(&mut g[..8]),
            blue: bytes(&mut b[..8]),
            alpha: Some(bytes(&mut a[..8])),
            layout: output_layout,
        },
    )
    .unwrap();
    let [r, g, b, a] = &mut padded;
    cpu.try_apply_planar_convert(
        PlanarBytes {
            red: &input,
            green: &input,
            blue: &input,
            alpha: Some(&input),
            layout: input_layout,
        },
        PlanarBytesMut {
            red: bytes(r),
            green: bytes(g),
            blue: bytes(b),
            alpha: Some(bytes(a)),
            layout: output_layout,
        },
    )
    .unwrap();
    for (actual, expected) in short.iter().zip(padded.iter()) {
        assert_eq!(&actual[..8], &expected[..8]);
    }
    for (short_source, short_destination, source_alpha, destination_alpha) in [
        (true, false, false, true),
        (false, true, true, false),
        (true, true, false, false),
    ] {
        let mut result = [[0xaaaau16; 9]; 4];
        let mut reference = result;
        let src = if short_source {
            &input[..8]
        } else {
            &input[..]
        };
        let out_len = if short_destination { 8 } else { 9 };
        let [r, g, b, a] = &mut result;
        cpu.try_apply_planar_convert(
            PlanarBytes {
                red: src,
                green: src,
                blue: src,
                alpha: source_alpha.then_some(src),
                layout: input_layout,
            },
            PlanarBytesMut {
                red: bytes(&mut r[..out_len]),
                green: bytes(&mut g[..out_len]),
                blue: bytes(&mut b[..out_len]),
                alpha: destination_alpha.then(|| bytes(&mut a[..out_len])),
                layout: output_layout,
            },
        )
        .unwrap();
        let [r, g, b, a] = &mut reference;
        cpu.try_apply_planar_convert(
            PlanarBytes {
                red: &input,
                green: &input,
                blue: &input,
                alpha: source_alpha.then_some(&input[..]),
                layout: input_layout,
            },
            PlanarBytesMut {
                red: bytes(r),
                green: bytes(g),
                blue: bytes(b),
                alpha: destination_alpha.then(|| bytes(a)),
                layout: output_layout,
            },
        )
        .unwrap();
        for (actual, expected) in result.iter().zip(reference.iter()) {
            assert_eq!(&actual[..8], &expected[..8]);
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn packed_depth_conversion_unpadded_tail_matches_padded() {
    use ocio_rs::{
        transform::MatrixTransform, BitDepth, Config, PackedConversionLayout, TransformDirection,
    };
    let cpu = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[0.5, 1., 0.75, 1.]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap()
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    fn bytes(words: &mut [u16]) -> &mut [u8] {
        // SAFETY: same exclusively borrowed allocation as a byte slice.
        unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words))
        }
    }
    for channels in [3u8, 4] {
        let stride = channels as usize + 1;
        let input_len = 2 * stride + channels as usize;
        let output_len = 2 * stride + channels as usize;
        let full_len = 3 * stride + channels as usize - 1;
        let input = vec![80u8; full_len];
        let mut padded = vec![0xaaaau16; full_len];
        let layout = PackedConversionLayout {
            pixels: 3,
            channels,
            input_depth: BitDepth::Uint8,
            output_depth: BitDepth::Uint16,
            input_stride: stride as i64,
            output_stride: stride as i64,
        };
        cpu.try_apply_packed_convert(&input, bytes(&mut padded), layout)
            .unwrap();
        let mut single = vec![0xaaaau16; channels as usize];
        cpu.try_apply_packed_convert(
            &input[..channels as usize],
            bytes(&mut single),
            PackedConversionLayout {
                pixels: 1,
                ..layout
            },
        )
        .unwrap();
        assert_eq!(&single[..], &padded[..channels as usize]);
        for (short_input, short_output) in [(true, false), (false, true), (true, true)] {
            let mut result = vec![0xaaaau16; full_len];
            let src = if short_input {
                &input[..input_len]
            } else {
                &input[..]
            };
            let out = if short_output {
                &mut result[..output_len]
            } else {
                &mut result[..]
            };
            cpu.try_apply_packed_convert(src, bytes(out), layout)
                .unwrap();
            assert_eq!(
                &result[..output_len],
                &padded[..output_len],
                "channels={channels}, short={short_input}/{short_output}"
            );
            assert!(result[output_len..].iter().all(|&v| v == 0xaaaa));
        }
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn packed_conversion_changes_bit_depth_without_touching_padding() {
    use ocio_rs::{
        transform::MatrixTransform, BitDepth, Config, PackedConversionLayout, TransformDirection,
    };
    let processor = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[1.0, 1.0, 1.0, 1.0]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap();
    let cpu = processor
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    let input = [0u8, 128, 255, 77, 255, 64, 0, 77];
    let mut words = [0xaaaau16; 8];
    let output =
        unsafe { std::slice::from_raw_parts_mut(words.as_mut_ptr().cast::<u8>(), words.len() * 2) };
    let layout = PackedConversionLayout {
        pixels: 2,
        channels: 3,
        input_depth: BitDepth::Uint8,
        output_depth: BitDepth::Uint16,
        input_stride: 4,
        output_stride: 4,
    };
    cpu.try_apply_packed_convert(&input, output, layout)
        .unwrap();
    assert_eq!(words, [0, 32896, 65535, 0xaaaa, 65535, 16448, 0, 0xaaaa]);
    let mut short = [0u8; 5];
    assert!(cpu
        .try_apply_packed_convert(&input, &mut short, layout)
        .is_err());
    let mut unaligned = [0u8; 24];
    let offset = if (unaligned.as_ptr() as usize).is_multiple_of(2) {
        1
    } else {
        0
    };
    assert!(cpu
        .try_apply_packed_convert(&input, &mut unaligned[offset..], layout)
        .is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn planar_conversion_changes_bit_depth_without_touching_padding() {
    use ocio_rs::{
        transform::MatrixTransform, BitDepth, Config, PlanarByteLayout, PlanarBytes,
        PlanarBytesMut, TransformDirection,
    };
    let processor = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[1.0, 1.0, 1.0, 1.0]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap();
    let cpu = processor
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    let input_layout = PlanarByteLayout {
        width: 2,
        height: 2,
        x_stride: 1,
        y_stride: 3,
        bit_depth: BitDepth::Uint8,
    };
    let output_layout = PlanarByteLayout {
        width: 2,
        height: 2,
        x_stride: 2,
        y_stride: 6,
        bit_depth: BitDepth::Uint16,
    };
    let red = [0, 128, 99, 255, 64];
    let green = [255, 0, 99, 128, 64];
    let blue = [64, 255, 99, 0, 128];
    let alpha = [255, 128, 99, 64, 0];
    let mut r = [0xaaaau16; 5];
    let mut g = [0xaaaau16; 5];
    let mut b = [0xaaaau16; 5];
    let mut a = [0xaaaau16; 5];
    fn bytes(words: &mut [u16]) -> &mut [u8] {
        // SAFETY: u8 has no alignment or validity restrictions, and the returned
        // slice is limited to the lifetime and size of the exclusive word slice.
        unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words))
        }
    }
    let source = || PlanarBytes {
        red: &red,
        green: &green,
        blue: &blue,
        alpha: Some(&alpha),
        layout: input_layout,
    };
    cpu.try_apply_planar_convert(
        source(),
        PlanarBytesMut {
            red: bytes(&mut r),
            green: bytes(&mut g),
            blue: bytes(&mut b),
            alpha: Some(bytes(&mut a)),
            layout: output_layout,
        },
    )
    .unwrap();
    assert_eq!(a, [65535, 32896, 0xaaaa, 16448, 0]);
    cpu.try_apply_planar_convert(
        PlanarBytes {
            alpha: None,
            ..source()
        },
        PlanarBytesMut {
            red: bytes(&mut r),
            green: bytes(&mut g),
            blue: bytes(&mut b),
            alpha: Some(bytes(&mut a)),
            layout: output_layout,
        },
    )
    .unwrap();
    let missing_alpha = if ocio_rs::version_hex() >= 0x02060000 {
        65535
    } else {
        0
    };
    assert_eq!(
        a,
        [
            missing_alpha,
            missing_alpha,
            0xaaaa,
            missing_alpha,
            missing_alpha
        ]
    );
    assert!(cpu
        .try_apply_planar_convert(
            PlanarBytes {
                alpha: Some(&alpha[..1]),
                ..source()
            },
            PlanarBytesMut {
                red: bytes(&mut r),
                green: bytes(&mut g),
                blue: bytes(&mut b),
                alpha: None,
                layout: output_layout,
            },
        )
        .is_err());
    assert_eq!(r, [0, 32896, 0xaaaa, 65535, 16448]);
    assert_eq!(g, [65535, 0, 0xaaaa, 32896, 16448]);
    assert_eq!(b, [16448, 65535, 0xaaaa, 0, 32896]);
    cpu.try_apply_planar_convert(
        PlanarBytes {
            alpha: None,
            ..source()
        },
        PlanarBytesMut {
            red: bytes(&mut r),
            green: bytes(&mut g),
            blue: bytes(&mut b),
            alpha: None,
            layout: output_layout,
        },
    )
    .unwrap();
    assert_eq!(r, [0, 32896, 0xaaaa, 65535, 16448]);
    assert!(cpu
        .try_apply_planar_convert(
            source(),
            PlanarBytesMut {
                red: bytes(&mut r),
                green: bytes(&mut g),
                blue: bytes(&mut b),
                alpha: Some(bytes(&mut a)),
                layout: PlanarByteLayout {
                    bit_depth: BitDepth::Uint8,
                    ..output_layout
                }
            },
        )
        .is_err());
    cpu.try_apply_planar_convert(
        source(),
        PlanarBytesMut {
            red: bytes(&mut r),
            green: bytes(&mut g),
            blue: bytes(&mut b),
            alpha: None,
            layout: output_layout,
        },
    )
    .unwrap();
    assert!(cpu
        .try_apply_planar_convert(
            source(),
            PlanarBytesMut {
                red: &mut [],
                green: bytes(&mut g),
                blue: bytes(&mut b),
                alpha: Some(bytes(&mut a)),
                layout: output_layout
            }
        )
        .is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn signed_planar_strides_convert_and_process_in_place() {
    use ocio_rs::{
        transform::MatrixTransform, BitDepth, Config, SignedPlanarByteLayout, SignedPlanarBytes,
        SignedPlanarBytesMut, TransformDirection,
    };
    let processor = Config::raw()
        .unwrap()
        .processor_from_transform(
            &MatrixTransform::scale(&[0.5, 1.0, 1.0, 1.0]).unwrap(),
            TransformDirection::Forward,
        )
        .unwrap();
    let input_layout = SignedPlanarByteLayout {
        width: 2,
        height: 2,
        origin: 5,
        x_stride: -1,
        y_stride: -3,
        bit_depth: BitDepth::Uint8,
    };
    let output_layout = SignedPlanarByteLayout {
        width: 2,
        height: 2,
        origin: 0,
        x_stride: 2,
        y_stride: 6,
        bit_depth: BitDepth::Uint16,
    };
    let mut red = [99u8, 40, 80, 99, 120, 200, 99];
    let mut green = [99u8, 10, 20, 99, 30, 40, 99];
    let mut blue = [99u8, 50, 60, 99, 70, 80, 99];
    let mut alpha = [99u8, 50, 100, 99, 150, 200, 99];
    let mut r = [0xaaaau16; 5];
    let mut g = [0xaaaau16; 5];
    let mut b = [0xaaaau16; 5];
    let mut a = [0xaaaau16; 5];
    fn bytes(words: &mut [u16]) -> &mut [u8] {
        // SAFETY: The returned bytes cover only the exclusively borrowed words.
        unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words))
        }
    }
    let convert = processor
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint16, 0)
        .unwrap();
    convert
        .try_apply_planar_signed_convert(
            SignedPlanarBytes {
                red: &red,
                green: &green,
                blue: &blue,
                alpha: Some(&alpha),
                layout: input_layout,
            },
            SignedPlanarBytesMut {
                red: bytes(&mut r),
                green: bytes(&mut g),
                blue: bytes(&mut b),
                alpha: Some(bytes(&mut a)),
                layout: output_layout,
            },
        )
        .unwrap();
    assert_eq!(r, [25700, 15420, 0xaaaa, 10280, 5140]);
    assert_eq!(g, [10280, 7710, 0xaaaa, 5140, 2570]);
    assert_eq!(b, [20560, 17990, 0xaaaa, 15420, 12850]);
    assert_eq!(a, [51400, 38550, 0xaaaa, 25700, 12850]);
    let reversed_output = SignedPlanarByteLayout {
        origin: 10,
        x_stride: -2,
        y_stride: -6,
        ..output_layout
    };
    let mut rr = [0xaaaau16; 6];
    let mut gg = [0xaaaau16; 6];
    let mut bb = [0xaaaau16; 6];
    let mut aa = [0xaaaau16; 6];
    convert
        .try_apply_planar_signed_convert(
            SignedPlanarBytes {
                red: &red,
                green: &green,
                blue: &blue,
                alpha: Some(&alpha),
                layout: input_layout,
            },
            SignedPlanarBytesMut {
                red: bytes(&mut rr),
                green: bytes(&mut gg),
                blue: bytes(&mut bb),
                alpha: Some(bytes(&mut aa)),
                layout: reversed_output,
            },
        )
        .unwrap();
    assert_eq!(rr, [0xaaaa, 5140, 10280, 0xaaaa, 15420, 25700]);
    assert_eq!(aa, [0xaaaa, 12850, 25700, 0xaaaa, 38550, 51400]);
    let cpu = processor
        .optimized_cpu_processor_bit_depth(BitDepth::Uint8, BitDepth::Uint8, 0)
        .unwrap();
    cpu.try_apply_planar_signed(SignedPlanarBytesMut {
        red: &mut red,
        green: &mut green,
        blue: &mut blue,
        alpha: Some(&mut alpha),
        layout: input_layout,
    })
    .unwrap();
    assert_eq!(red, [99, 20, 40, 99, 60, 100, 99]);
    let invalid = SignedPlanarByteLayout {
        origin: 3,
        ..input_layout
    };
    assert!(cpu
        .try_apply_planar_signed(SignedPlanarBytesMut {
            red: &mut red,
            green: &mut green,
            blue: &mut blue,
            alpha: None,
            layout: invalid
        })
        .is_err());
}
