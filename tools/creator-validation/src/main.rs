use ocio_rs::transform::{ExposureContrastTransform, MatrixTransform};
use ocio_rs::{
    Config, DynamicPropertyType, GpuLanguage, GpuShaderCreator, GpuTextureChannel,
    GpuTextureDimensions, Interpolation, TransformDirection,
};
use std::ffi::c_void;
use std::sync::Arc;

extern "C" {
    fn creator_fixture_create() -> *mut c_void;
    fn creator_fixture_create_alias() -> *mut c_void;
    fn creator_fixture_null_rgb() -> *mut c_void;
    fn creator_fixture_valid_rgb() -> *mut c_void;
    fn creator_fixture_custom_rgb() -> *mut c_void;
    fn creator_fixture_custom_rgb_copies() -> u32;
    fn creator_fixture_unknown_property() -> *mut c_void;
    fn creator_fixture_null_hue() -> *mut c_void;
    #[cfg(feature = "v2_5")]
    fn creator_fixture_custom_hue() -> *mut c_void;
    #[cfg(feature = "v2_5")]
    fn creator_fixture_custom_hue_copies() -> u32;
    fn creator_fixture_destroy(ptr: *mut c_void);
    fn creator_fixture_version() -> i32;
    fn creator_fixture_destroyed() -> u32;
    fn creator_fixture_reset_destroyed();
    fn creator_fixture_calls(index: u32) -> u32;
    fn creator_fixture_sections_match() -> bool;
    fn proxy_fixture_create() -> *mut c_void;
    fn proxy_fixture_create_config() -> *mut c_void;
    fn proxy_fixture_empty() -> *mut c_void;
    fn proxy_fixture_destroy(ptr: *mut c_void);
    fn proxy_fixture_destroyed() -> u32;
    fn grading_fixture_create(tone: bool, fail: bool) -> *mut c_void;
    fn grading_fixture_destroy(ptr: *mut c_void);
    fn grading_fixture_install_config() -> *mut c_void;
    fn grading_fixture_restore_config(ptr: *mut c_void);
    fn image_fixture_rgba(data: *mut f32) -> *mut c_void;
    fn image_fixture_planar(data: *mut f32) -> *mut c_void;
    fn image_fixture_custom(data: *mut f32) -> *mut c_void;
    fn image_fixture_throwing(data: *mut f32, failure: std::ffi::c_int) -> *mut c_void;
    fn image_fixture_destroy(ptr: *mut c_void);
}

fn check_native_image_descriptors() -> ocio_rs::Result<()> {
    use ocio_rs::{NativeImageDataPointers, NativeImageDesc};

    let config = Config::raw()?;
    let matrix = MatrixTransform::scale(&[2.0, 3.0, 4.0, 1.0])?;
    let cpu = config
        .processor_from_transform(&matrix, TransformDirection::Forward)?
        .default_cpu_processor()?;
    let mut packed = [0.25, 0.125, 0.25, 0.75];
    let mut planar = [-1.0; 16];
    unsafe {
        let packed_base = packed.as_mut_ptr();
        let planar_base = planar.as_mut_ptr();
        let src = image_fixture_rgba(packed_base);
        let dst = image_fixture_planar(planar_base);
        assert!(!src.is_null() && !dst.is_null());
        let packed_info = ocio_rs::NativeImageDescInfo::inspect(src);
        let planar_info = ocio_rs::NativeImageDescInfo::inspect(dst);
        let packed_ptrs = NativeImageDataPointers::inspect(src);
        let planar_ptrs = NativeImageDataPointers::inspect(dst);
        let diagnostic = NativeImageDesc::diagnostic_text_from_ptr(dst);
        let converted = cpu.try_apply_raw_v2(src, dst);
        let applied = cpu.try_apply_raw(src);
        image_fixture_destroy(dst);
        image_fixture_destroy(src);
        let packed_info = packed_info?;
        let planar_info = planar_info?;
        let packed_ptrs = packed_ptrs?;
        let planar_ptrs = planar_ptrs?;
        assert_eq!(packed_ptrs.packed, packed_base.cast());
        assert_eq!(packed_ptrs.alpha, packed_base.add(3).cast());
        assert_eq!(planar_ptrs.alpha, planar_base.add(12).cast());
        assert!(planar_ptrs.packed.is_null());
        assert!(diagnostic?.starts_with("<PlanarImageDesc "));
        use ocio_rs::{BitDepth, ChannelOrdering, NativeImageLayout};
        assert_eq!(
            packed_info.layout,
            NativeImageLayout::Packed {
                channel_order: ChannelOrdering::Rgba as i32,
                channels: 4,
                channel_stride_bytes: 4,
            }
        );
        assert_eq!(planar_info.layout, NativeImageLayout::Planar);
        for (info, stride, is_rgba_packed) in [(packed_info, 16, true), (planar_info, 4, false)] {
            assert_eq!((info.width, info.height), (1, 1));
            assert_eq!((info.x_stride_bytes, info.y_stride_bytes), (stride, stride));
            assert_eq!(info.bit_depth, BitDepth::F32 as i32);
            assert!(info.has_alpha && info.is_float);
            assert_eq!(info.is_rgba_packed, is_rgba_packed);
        }
        assert!(ocio_rs::NativeImageDescInfo::inspect(std::ptr::null()).is_err());
        converted?;
        applied?;
    }
    let expected = [0.5, 0.375, 1.0, 0.75];
    assert_eq!(packed, expected);
    assert_eq!([planar[0], planar[4], planar[8], planar[12]], expected);
    assert!(planar
        .iter()
        .enumerate()
        .all(|(index, value)| index % 4 == 0 || *value == -1.0));
    // Two RGB pixels with irregular channel placement and padding for native
    // pointer stepping (the final R pointer steps to element 13).
    let mut custom = [
        0.25, 0.125, -1.0, 0.25, -1.0, 0.125, 0.25, -1.0, 0.5, -1.0, -1.0, -1.0, -1.0,
    ];
    let info = unsafe {
        let descriptor = image_fixture_custom(custom.as_mut_ptr());
        assert!(!descriptor.is_null());
        let info = ocio_rs::NativeImageDescInfo::inspect(descriptor);
        let applied = cpu.try_apply_raw(descriptor);
        image_fixture_destroy(descriptor);
        applied?;
        info?
    };
    assert_eq!(info.layout, ocio_rs::NativeImageLayout::Custom);
    assert_eq!((info.width, info.height), (2, 1));
    assert_eq!((info.x_stride_bytes, info.y_stride_bytes), (20, 40));
    assert_eq!(info.bit_depth, ocio_rs::BitDepth::F32 as i32);
    assert!(!info.has_alpha && !info.is_rgba_packed && info.is_float);
    assert_eq!(
        custom,
        [1.0, 0.375, -1.0, 0.5, -1.0, 0.5, 0.75, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0]
    );
    let expected_unmodified = custom;
    for kind in [1, 2] {
        let (info, formatted, applied) = unsafe {
            let descriptor = image_fixture_throwing(custom.as_mut_ptr(), kind);
            assert!(!descriptor.is_null());
            let info = ocio_rs::NativeImageDescInfo::inspect(descriptor).map(|_| ());
            let formatted = NativeImageDesc::diagnostic_text_from_ptr(descriptor).map(|_| ());
            let applied = cpu.try_apply_raw(descriptor);
            image_fixture_destroy(descriptor);
            (info, formatted, applied)
        };
        for result in [info, formatted, applied] {
            match (kind, result) {
                (1, Err(ocio_rs::OcioError::Ocio(message)))
                | (2, Err(ocio_rs::OcioError::MissingFile(message))) => assert!(message.is_empty()),
                (_, other) => panic!("wrong classification for native exception {kind}: {other:?}"),
            }
        }
        assert_eq!(custom, expected_unmodified);
        assert!(unsafe { ocio_rs::ocio_sys::ocio_error_get_last() }.is_null());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        unsafe { creator_fixture_version() },
        ocio_rs::version_hex(),
        "headers and runtime must match"
    );
    for tone in [false, true] {
        for fail in [true, false] {
            unsafe {
                let native = grading_fixture_create(tone, fail);
                let handle = ocio_sys::ocio_const_transform_create_readonly_handle(native);
                grading_fixture_destroy(native);
                assert!(!handle.is_null());
                let mut values = [123.0_f64; 34];
                ocio_sys::ocio_error_clear_last();
                let copied = if tone {
                    ocio_sys::ocio_grading_tone_transform_copy_value(
                        handle,
                        values.as_mut_ptr(),
                        31,
                    )
                } else {
                    ocio_sys::ocio_grading_primary_transform_copy_value(
                        handle,
                        values.as_mut_ptr(),
                        34,
                    )
                };
                if fail {
                    assert!(!copied);
                    assert_eq!(values, [123.0; 34]);
                    let error = ocio_sys::ocio_error_get_last();
                    assert!(!error.is_null());
                    assert_eq!(
                        std::ffi::CStr::from_ptr(error).to_bytes(),
                        b"grading fixture failure"
                    );
                } else {
                    assert!(copied);
                    assert!(ocio_sys::ocio_error_get_last().is_null());
                }
                ocio_sys::ocio_error_clear_last();
                ocio_sys::ocio_transform_destroy(handle);
            }
        }
    }
    {
        struct RestoreConfig(*mut c_void);
        impl Drop for RestoreConfig {
            fn drop(&mut self) {
                unsafe { grading_fixture_restore_config(self.0) };
            }
        }
        let saved = unsafe { grading_fixture_install_config() };
        assert!(!saved.is_null());
        let _restore = RestoreConfig(saved);
        // This fixture is a standalone, single-threaded process and controls
        // all native accesses to the temporary global config and its children.
        let config = unsafe { ocio_rs::try_current_config_native_copy() }?.unwrap();
        for (name, tone) in [("fixture-primary", false), ("fixture-tone", true)] {
            let space = config.try_color_space(name)?.unwrap();
            let transform = space
                .try_transform(ocio_rs::ColorSpaceDirection::ToReference)?
                .unwrap();
            let error = match transform {
                ocio_rs::transform::Transform::GradingPrimary(value) if !tone => {
                    value.try_value().unwrap_err()
                }
                ocio_rs::transform::Transform::GradingTone(value) if tone => {
                    value.try_value().unwrap_err()
                }
                _ => panic!("unexpected grading transform"),
            };
            assert!(error.to_string().contains("grading fixture failure"));
        }
        let space = config.try_color_space("fixture-exposure")?.unwrap();
        let transform = space
            .try_transform(ocio_rs::ColorSpaceDirection::ToReference)?
            .unwrap();
        let ocio_rs::transform::Transform::ExposureContrast(exposure) = transform else {
            panic!("unexpected exposure transform");
        };
        for error in [
            exposure.try_style().unwrap_err(),
            exposure.try_exposure().unwrap_err(),
            exposure.try_contrast().unwrap_err(),
            exposure.try_gamma().unwrap_err(),
            exposure.try_pivot().unwrap_err(),
            exposure.try_log_exposure_step().unwrap_err(),
            exposure.try_log_mid_gray().unwrap_err(),
            exposure.try_is_exposure_dynamic().unwrap_err(),
            exposure.try_is_contrast_dynamic().unwrap_err(),
            exposure.try_is_gamma_dynamic().unwrap_err(),
        ] {
            assert!(error.to_string().contains("exposure fixture failure"));
        }
        assert!(ExposureContrastTransform::create()?.try_exposure().is_ok());
    }
    {
        use std::ffi::CStr;
        assert!(
            unsafe { ocio_rs::ConfigIOProxy::from_native_shared_ptr(std::ptr::null()) }.is_err()
        );
        let empty = unsafe { proxy_fixture_empty() };
        assert!(unsafe { ocio_rs::ConfigIOProxy::from_native_shared_ptr(empty) }.is_err());
        unsafe { proxy_fixture_destroy(empty) };
        let native = unsafe { proxy_fixture_create() };
        let proxy = unsafe { ocio_rs::ConfigIOProxy::from_native_shared_ptr(native) }?;
        unsafe { proxy_fixture_destroy(native) };
        let path = CStr::from_bytes_with_nul(b"asset.lut\0")?;
        assert_eq!(proxy.try_config_data_bytes()?.unwrap(), b"cfg\0bytes");
        assert_eq!(proxy.read_lut_data_cstr(path)?, [0, 1, 255]);
        assert_eq!(proxy.read_fast_lut_file_hash_cstr(path)?, b"native-hash");
        let failing_path = CStr::from_bytes_with_nul(b"raise\0")?;
        assert!(proxy.read_lut_data_cstr(failing_path).is_err());
        assert_eq!(proxy.read_lut_data_cstr(path)?, [0, 1, 255]);
        let context = ocio_rs::Context::create()?;
        context.set_config_io_proxy_object(&proxy)?;
        let config = Config::raw()?;
        config.set_config_io_proxy_object(&proxy)?;
        drop(proxy);
        assert_eq!(unsafe { proxy_fixture_destroyed() }, 0);
        let retained = context.try_config_io_proxy_object()?.unwrap();
        drop(context);
        assert_eq!(retained.read_lut_data_cstr(path)?, [0, 1, 255]);
        drop(retained);
        assert_eq!(unsafe { proxy_fixture_destroyed() }, 0);
        let retained = config.try_config_io_proxy_object()?.unwrap();
        drop(config);
        assert_eq!(retained.read_fast_lut_file_hash_cstr(path)?, b"native-hash");
        drop(retained);
        assert_eq!(unsafe { proxy_fixture_destroyed() }, 1);

        let native = unsafe { proxy_fixture_create_config() };
        let proxy = unsafe { ocio_rs::ConfigIOProxy::from_native_shared_ptr(native) }?;
        unsafe { proxy_fixture_destroy(native) };
        let loaded = Config::from_config_io_proxy(&proxy)?;
        drop(proxy);
        assert_eq!(unsafe { proxy_fixture_destroyed() }, 1);
        loaded.validate()?;
        let retained = loaded.try_config_io_proxy_object()?.unwrap();
        drop(loaded);
        assert_eq!(retained.read_lut_data_cstr(path)?, [0, 1, 255]);
        drop(retained);
        assert_eq!(unsafe { proxy_fixture_destroyed() }, 2);
    }
    check_native_image_descriptors()?;
    {
        let native = unsafe { creator_fixture_unknown_property() };
        assert!(!native.is_null());
        let creator = unsafe { GpuShaderCreator::from_native_shared_ptr(native) }?;
        unsafe { creator_fixture_destroy(native) };
        let property = creator.try_dynamic_property_by_index(0)?.unwrap();
        assert!(property.try_property_type().is_err());
        assert!(property.double_value().is_err());
        assert!(property.set_double_value(1.0).is_err());
    }
    // Exercise malformed native implementations through the raw bridge: a
    // null curve must become an error, never a null dereference.
    unsafe {
        let native = creator_fixture_custom_rgb();
        assert!(!native.is_null());
        let creator = ocio_sys::ocio_gpu_shader_creator_copy_native_shared(native);
        assert!(!creator.is_null());
        let property = ocio_sys::ocio_gpu_shader_creator_dynamic_property(creator, 0, true);
        assert!(!property.is_null());
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_slope(property, 0, 0, 0.37);
        assert!(ocio_sys::ocio_error_get_last().is_null());
        assert_eq!(creator_fixture_custom_rgb_copies(), 1);
        assert!(
            (ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_slope(property, 0, 0) - 0.37)
                .abs()
                < 1e-6
        );
        assert!(ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_dynamic_property_destroy(property);
        ocio_sys::ocio_gpu_shader_creator_destroy(creator);
        creator_fixture_destroy(native);
    }
    unsafe {
        let native = creator_fixture_valid_rgb();
        assert!(!native.is_null());
        let creator = ocio_sys::ocio_gpu_shader_creator_copy_native_shared(native);
        assert!(!creator.is_null());
        let property = ocio_sys::ocio_gpu_shader_creator_dynamic_property(creator, 0, true);
        assert!(!property.is_null());
        ocio_sys::ocio_error_clear_last();
        // RGB_NUM_CURVES is an actual native enum sentinel, not a curve.
        assert_eq!(
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_num_control_points(property, 4),
            0
        );
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_dynamic_property_destroy(property);
        ocio_sys::ocio_gpu_shader_creator_destroy(creator);
        creator_fixture_destroy(native);
    }
    unsafe {
        let native = creator_fixture_null_rgb();
        assert!(!native.is_null());
        let creator = ocio_sys::ocio_gpu_shader_creator_copy_native_shared(native);
        assert!(!creator.is_null());
        let property = ocio_sys::ocio_gpu_shader_creator_dynamic_property(creator, 0, true);
        assert!(!property.is_null());
        ocio_sys::ocio_error_clear_last();
        assert_eq!(
            ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_num_control_points(property, 0),
            0
        );
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_num_control_points(property, 0, 2);
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_num_control_points(property, 0, -1);
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        assert!(std::ffi::CStr::from_ptr(ocio_sys::ocio_error_get_last())
            .to_bytes()
            .starts_with(b"negative dynamic RGB"));
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_slope(property, 0, -1);
        let error = ocio_sys::ocio_error_get_last();
        assert!(!error.is_null());
        assert!(std::ffi::CStr::from_ptr(error)
            .to_bytes()
            .starts_with(b"negative dynamic curve index"));
        let mut x = 0.0;
        let mut y = 0.0;
        for operation in 0..3 {
            ocio_sys::ocio_error_clear_last();
            match operation {
                0 => ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_control_point(
                    property, 0, -1, &mut x, &mut y,
                ),
                1 => ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_control_point(
                    property, 0, -1, 0.0, 0.0,
                ),
                _ => ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_slope(
                    property, 0, -1, 0.0,
                ),
            }
            let error = ocio_sys::ocio_error_get_last();
            assert!(!error.is_null());
            assert!(std::ffi::CStr::from_ptr(error)
                .to_bytes()
                .starts_with(b"negative dynamic curve index"));
        }
        for operation in 0..5 {
            ocio_sys::ocio_error_clear_last();
            match operation {
                0 => ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_control_point(
                    property, 0, 0, &mut x, &mut y,
                ),
                1 => ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_control_point(
                    property, 0, 0, 0.0, 0.0,
                ),
                2 => {
                    ocio_sys::ocio_dynamic_property_grading_rgb_curve_get_slope(property, 0, 0);
                }
                3 => {
                    ocio_sys::ocio_dynamic_property_grading_rgb_curve_set_slope(property, 0, 0, 0.0)
                }
                _ => {
                    ocio_sys::ocio_dynamic_property_grading_rgb_curve_slopes_are_default(
                        property, 0,
                    );
                }
            }
            assert!(
                !ocio_sys::ocio_error_get_last().is_null(),
                "RGB operation {operation}"
            );
        }
        ocio_sys::ocio_error_clear_last();
        assert!(ocio_sys::ocio_dynamic_property_grading_rgb_curve_object_get(property).is_null());
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
        assert!(ocio_sys::ocio_grading_curve_value_shared(property, 1).is_null());
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_dynamic_property_destroy(property);
        ocio_sys::ocio_gpu_shader_creator_destroy(creator);
        creator_fixture_destroy(native);
    }
    #[cfg(feature = "v2_5")]
    unsafe {
        let native = creator_fixture_custom_hue();
        assert!(!native.is_null());
        let creator = ocio_sys::ocio_gpu_shader_creator_copy_native_shared(native);
        assert!(!creator.is_null());
        let property = ocio_sys::ocio_gpu_shader_creator_dynamic_property(creator, 0, true);
        assert!(!property.is_null());
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_dynamic_property_grading_hue_curve_set_slope(property, 0, 0, 0.37);
        assert!(ocio_sys::ocio_error_get_last().is_null());
        assert_eq!(creator_fixture_custom_hue_copies(), 1);
        assert!(
            (ocio_sys::ocio_dynamic_property_grading_hue_curve_get_slope(property, 0, 0) - 0.37)
                .abs()
                < 1e-6
        );
        let value = ocio_sys::ocio_dynamic_property_grading_hue_curve_object_get(property);
        assert!(!value.is_null());
        assert!(ocio_sys::ocio_grading_hue_curve_object_get_draw_curve_only(
            value
        ));
        ocio_sys::ocio_grading_hue_curve_object_destroy(value);
        ocio_sys::ocio_dynamic_property_destroy(property);
        ocio_sys::ocio_gpu_shader_creator_destroy(creator);
        creator_fixture_destroy(native);
    }
    #[cfg(feature = "v2_5")]
    unsafe {
        let native = creator_fixture_null_hue();
        assert!(!native.is_null());
        let creator = ocio_sys::ocio_gpu_shader_creator_copy_native_shared(native);
        assert!(!creator.is_null());
        let property = ocio_sys::ocio_gpu_shader_creator_dynamic_property(creator, 0, true);
        assert!(!property.is_null());
        ocio_sys::ocio_error_clear_last();
        assert_eq!(
            ocio_sys::ocio_dynamic_property_grading_hue_curve_get_num_control_points(property, 0),
            0
        );
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_dynamic_property_grading_hue_curve_set_num_control_points(property, 0, 2);
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_dynamic_property_grading_hue_curve_set_num_control_points(property, 0, -1);
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        assert!(std::ffi::CStr::from_ptr(ocio_sys::ocio_error_get_last())
            .to_bytes()
            .starts_with(b"negative dynamic hue"));
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_dynamic_property_grading_hue_curve_get_slope(property, 0, -1);
        let error = ocio_sys::ocio_error_get_last();
        assert!(!error.is_null());
        assert!(std::ffi::CStr::from_ptr(error)
            .to_bytes()
            .starts_with(b"negative dynamic curve index"));
        let mut x = 0.0;
        let mut y = 0.0;
        for operation in 0..3 {
            ocio_sys::ocio_error_clear_last();
            match operation {
                0 => ocio_sys::ocio_dynamic_property_grading_hue_curve_get_control_point(
                    property, 0, -1, &mut x, &mut y,
                ),
                1 => ocio_sys::ocio_dynamic_property_grading_hue_curve_set_control_point(
                    property, 0, -1, 0.0, 0.0,
                ),
                _ => ocio_sys::ocio_dynamic_property_grading_hue_curve_set_slope(
                    property, 0, -1, 0.0,
                ),
            }
            let error = ocio_sys::ocio_error_get_last();
            assert!(!error.is_null());
            assert!(std::ffi::CStr::from_ptr(error)
                .to_bytes()
                .starts_with(b"negative dynamic curve index"));
        }
        for operation in 0..5 {
            ocio_sys::ocio_error_clear_last();
            match operation {
                0 => ocio_sys::ocio_dynamic_property_grading_hue_curve_get_control_point(
                    property, 0, 0, &mut x, &mut y,
                ),
                1 => ocio_sys::ocio_dynamic_property_grading_hue_curve_set_control_point(
                    property, 0, 0, 0.0, 0.0,
                ),
                2 => {
                    ocio_sys::ocio_dynamic_property_grading_hue_curve_get_slope(property, 0, 0);
                }
                3 => {
                    ocio_sys::ocio_dynamic_property_grading_hue_curve_set_slope(property, 0, 0, 0.0)
                }
                _ => {
                    ocio_sys::ocio_dynamic_property_grading_hue_curve_slopes_are_default(
                        property, 0,
                    );
                }
            }
            assert!(
                !ocio_sys::ocio_error_get_last().is_null(),
                "hue operation {operation}"
            );
        }
        ocio_sys::ocio_error_clear_last();
        assert!(ocio_sys::ocio_dynamic_property_grading_hue_curve_object_get(property).is_null());
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
        assert!(ocio_sys::ocio_grading_curve_value_shared(property, 3).is_null());
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
        let curves = [std::ptr::null::<c_void>(); 8];
        let counts = [0usize; 8];
        ocio_sys::ocio_dynamic_property_grading_hue_curve_set_complete(
            property,
            curves.as_ptr(),
            counts.as_ptr(),
        );
        assert!(!ocio_sys::ocio_error_get_last().is_null());
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_dynamic_property_destroy(property);
        ocio_sys::ocio_gpu_shader_creator_destroy(creator);
        creator_fixture_destroy(native);
    }
    #[cfg(not(feature = "v2_5"))]
    if unsafe { creator_fixture_version() } >= 0x02050000 {
        let native = unsafe { creator_fixture_null_hue() };
        assert!(!native.is_null());
        let creator = unsafe { GpuShaderCreator::from_native_shared_ptr(native) }?;
        unsafe { creator_fixture_destroy(native) };
        let property = creator.try_dynamic_property_by_index(0)?.unwrap();
        assert!(property.try_property_type().is_err());
        assert!(property.double_value().is_err());
        assert!(property.set_double_value(1.0).is_err());
    }
    // Deliberately malformed clone is tested through the unsafe sys layer,
    // not the safe wrapper's native import contract.
    let alias = unsafe { creator_fixture_create_alias() };
    assert!(!alias.is_null());
    let alias_handle = unsafe { ocio_sys::ocio_gpu_shader_creator_copy_native_shared(alias) };
    assert!(!alias_handle.is_null());
    let result = unsafe { ocio_sys::ocio_gpu_shader_creator_clone(alias_handle) };
    assert!(result.is_null());
    let error_ptr = unsafe { ocio_sys::ocio_error_get_last() };
    assert!(!error_ptr.is_null());
    let error = unsafe { std::ffi::CStr::from_ptr(error_ptr) };
    assert!(error
        .to_bytes()
        .windows(b"returned the original object".len())
        .any(|part| part == b"returned the original object"));
    unsafe {
        ocio_sys::ocio_error_clear_last();
        ocio_sys::ocio_gpu_shader_creator_destroy(alias_handle);
        creator_fixture_destroy(alias);
        creator_fixture_reset_destroyed();
    }
    let native = unsafe { creator_fixture_create() };
    assert!(!native.is_null());
    let mut creator = unsafe { GpuShaderCreator::from_native_shared_ptr(native) }?;
    unsafe { creator_fixture_destroy(native) };
    assert!(creator.try_borrow_desc()?.is_none());
    creator.set_function_name("custom_shader")?;
    creator.set_language(GpuLanguage::Glsl4_0)?;
    creator.try_set_texture_max_width(128)?;
    #[cfg(feature = "v2_5")]
    {
        creator.try_set_descriptor_set_index(3, 7)?;
        assert_eq!(creator.try_descriptor_set_index()?, 3);
        assert_eq!(creator.try_texture_binding_start()?, 7);
        assert!(creator.try_set_descriptor_set_index(4, 0).is_err());
    }
    assert_eq!(
        creator.try_function_name()?.as_deref(),
        Some("custom_shader")
    );
    assert_eq!(
        creator.function_name_ref()?.unwrap().to_bytes(),
        b"custom_shader"
    );
    assert!(!creator.cache_id_ref()?.unwrap().to_bytes().is_empty());
    creator.begin("manual")?;
    creator.add_to_function_shader_code("body")?;
    assert!(creator.add_to_function_shader_code("reject").is_err());
    creator.create_shader_text("params", "textures", "helpers", "header", "body", "footer")?;
    assert!(unsafe { creator_fixture_sections_match() });
    creator.finalize()?;
    creator.end()?;
    assert_eq!(unsafe { creator_fixture_calls(0) }, 1);
    assert_eq!(unsafe { creator_fixture_calls(1) }, 1);
    assert_eq!(unsafe { creator_fixture_calls(2) }, 1);
    assert_eq!(unsafe { creator_fixture_calls(3) }, 1);
    assert!(unsafe { creator_fixture_calls(4) } >= 1);
    assert!(creator
        .add_texture_2d(
            "bad",
            "bad_sampler",
            2,
            1,
            GpuTextureChannel::Red,
            GpuTextureDimensions::Texture1D,
            Interpolation::Linear,
            &[0.0]
        )
        .is_err());
    assert!(creator
        .add_texture_3d("bad3", "bad3_sampler", 2, Interpolation::Linear, &[0.0])
        .is_err());
    assert_eq!(unsafe { creator_fixture_calls(5) }, 0);
    assert_eq!(unsafe { creator_fixture_calls(6) }, 0);
    let one = creator.add_texture_2d(
        "one",
        "one_sampler",
        2,
        1,
        GpuTextureChannel::Red,
        GpuTextureDimensions::Texture1D,
        Interpolation::Linear,
        &[0.0, 1.0],
    )?;
    let cube =
        creator.add_texture_3d("cube", "cube_sampler", 2, Interpolation::Linear, &[0.0; 24])?;
    assert_eq!(one.is_some(), ocio_rs::version_hex() >= 0x02050100);
    assert_eq!(cube.is_some(), ocio_rs::version_hex() >= 0x02050100);
    assert_eq!(unsafe { creator_fixture_calls(5) }, 1);
    assert_eq!(unsafe { creator_fixture_calls(6) }, 1);
    assert!(creator.add_uniform_f64("manual_double", 1.5)?);
    assert!(!creator.add_uniform_f64("manual_double", 7.0)?);
    assert!(creator.add_uniform_bool("manual_bool", true)?);
    let owned = Arc::new(7);
    let captured = Arc::clone(&owned);
    assert!(creator.add_uniform_f64_getter("live_double", move || *captured as f64)?);
    assert!(creator.add_uniform_bool_getter("live_bool", || true)?);
    assert!(creator.add_uniform_float3_getter("live_float3", || [1.0, 2.0, 3.0])?);
    assert!(creator.add_uniform_float3("manual_float3", [1.0, 2.0, 3.0])?);
    assert!(creator
        .add_uniform_f32_array("manual_f32", &[1.0, 2.0], 1)
        .is_err());
    assert!(creator.add_uniform_f32_array("manual_f32", &[1.0, 2.0], 4)?);
    assert!(creator.add_uniform_i32_array("manual_i32", &[3, 4], 4)?);
    assert!(creator.add_uniform_f32_array_getter("live_f32", 4, |data| {
        data[0] = 1.0;
        1
    })?);
    assert!(creator.add_uniform_i32_array_getter("live_i32", 4, |data| {
        data[0] = 2;
        1
    })?);
    let transform = MatrixTransform::scale(&[1.1, 0.9, 1.2, 1.0])?;
    let processor =
        Config::raw()?.processor_from_transform(&transform, TransformDirection::Forward)?;
    processor
        .default_gpu_processor()?
        .try_extract_to_creator(&mut creator)?;
    let exposure = ExposureContrastTransform::create()?;
    exposure.set_exposure(0.5);
    exposure.make_exposure_dynamic();
    Config::raw()?
        .processor_from_transform(&exposure, TransformDirection::Forward)?
        .default_gpu_processor()?
        .try_extract_to_creator(&mut creator)?;
    assert_eq!(creator.try_num_dynamic_properties()?, 1);
    assert!(creator.try_has_dynamic_property_kind(DynamicPropertyType::Exposure)?);
    assert!(creator.try_dynamic_property_by_index(1).is_err());
    let property = creator.try_dynamic_property_by_index(0)?.unwrap();
    assert_eq!(property.double_value()?, 0.5);
    let same = creator
        .try_dynamic_property(DynamicPropertyType::Exposure)?
        .unwrap();
    same.set_double_value(1.25)?;
    assert_eq!(property.double_value()?, 1.25);
    assert!(creator.add_uniform_dynamic_double("manual_dynamic", DynamicPropertyType::Exposure)?);
    let mut cloned = creator.try_clone_creator()?;
    cloned.add_dynamic_property(&property)?;
    assert!(cloned.add_dynamic_property(&property).is_err());
    assert_eq!(cloned.try_num_dynamic_properties()?, 1);
    drop(creator);
    assert_eq!(Arc::strong_count(&owned), 1);
    assert_eq!(unsafe { creator_fixture_destroyed() }, 1);
    assert!(cloned.try_borrow_desc()?.is_none());
    assert_eq!(
        cloned.try_function_name()?.as_deref(),
        Some("custom_shader")
    );
    assert_eq!(cloned.try_texture_max_width()?, 128);
    assert_eq!(cloned.try_language()?, GpuLanguage::Glsl4_0);
    drop(cloned);
    assert_eq!(unsafe { creator_fixture_destroyed() }, 2);
    property.set_double_value(2.0)?;
    assert_eq!(property.double_value()?, 2.0);
    println!(
        "Non-descriptor native creator and clone passed; header version {:#x}",
        unsafe { creator_fixture_version() }
    );
    Ok(())
}
