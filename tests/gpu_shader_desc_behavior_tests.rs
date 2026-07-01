//! GPU shader descriptor behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after unit-level smoke coverage.
//! In bundled/real mode they validate that shader descriptor configuration
//! round-trips and that extracted descriptor payloads are structurally
//! consistent with the reported uniform/texture metadata.

mod common;
use common::*;

use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::ExposureContrastTransform;
use ocio_rs::transform::MatrixTransform;
use ocio_rs::{
    DynamicPropertyType, ExposureContrastStyle, GpuLanguage, GpuShaderDesc, GpuTextureChannel,
    GpuTextureDimensions, GpuUniformType, GpuUniformValue, Interpolation, TransformDirection,
};

fn gpu_shader_desc_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn extracted_gpu_shader_desc() -> Option<GpuShaderDesc> {
    let config = create_test_config()?;
    let transform = MatrixTransform::scale(&[1.1, 0.9, 1.2, 1.0]).ok()?;
    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()?;
    let gpu = processor.default_gpu_processor().ok()?;
    let desc = GpuShaderDesc::create().ok()?;
    desc.set_language(GpuLanguage::Glsl4_0);
    desc.set_function_name("ocio_test_main").ok()?;
    desc.set_pixel_name("ocio_test_pixel").ok()?;
    desc.set_resource_prefix("ocio_test_").ok()?;
    let mut desc = desc;
    gpu.extract_shader_info(&mut desc);
    Some(desc)
}

fn extracted_dynamic_gpu_shader_desc() -> Option<(ocio_rs::Processor, GpuShaderDesc)> {
    let config = create_test_config()?;
    let transform = ExposureContrastTransform::create().ok()?;
    transform.set_style(ExposureContrastStyle::Linear);
    transform.set_exposure(0.0);
    transform.set_contrast(1.0);
    transform.set_gamma(1.0);
    transform.make_exposure_dynamic();

    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()?;
    let gpu = processor.default_gpu_processor().ok()?;
    let desc = GpuShaderDesc::create().ok()?;
    let mut desc = desc;
    gpu.extract_shader_info(&mut desc);
    Some((processor, desc))
}

#[test]
fn gpu_shader_desc_config_round_trip_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let desc = GpuShaderDesc::create().expect("gpu shader desc create");
    desc.set_language(GpuLanguage::Glsl4_0);
    desc.set_function_name("ocio_test_main")
        .expect("set_function_name");
    desc.set_pixel_name("ocio_test_pixel")
        .expect("set_pixel_name");
    desc.set_unique_id("ocio-test-uid").expect("set_unique_id");
    desc.set_resource_prefix("ocio_test_")
        .expect("set_resource_prefix");
    desc.set_descriptor_set_index(3, 7);
    desc.set_texture_max_width(64);
    desc.set_allow_texture_1d(false);

    assert_eq!(desc.language(), GpuLanguage::Glsl4_0);
    assert_eq!(desc.function_name().as_deref(), Some("ocio_test_main"));
    assert_eq!(desc.pixel_name().as_deref(), Some("ocio_test_pixel"));
    assert_eq!(desc.unique_id().as_deref(), Some("ocio-test-uid"));
    assert_eq!(desc.resource_prefix().as_deref(), Some("ocio_test_"));
    assert_eq!(desc.descriptor_set_index(), 3);
    assert_eq!(desc.texture_binding_start(), 7);
    assert_eq!(desc.texture_max_width(0), 64);
    assert!(!desc.allow_texture_1d());

    let cloned = desc.clone_desc().expect("clone_desc");
    assert_eq!(cloned.language(), GpuLanguage::Glsl4_0);
    assert_eq!(cloned.function_name().as_deref(), Some("ocio_test_main"));
    assert_eq!(cloned.pixel_name().as_deref(), Some("ocio_test_pixel"));
    assert_eq!(cloned.unique_id().as_deref(), Some("ocio-test-uid"));
    assert_eq!(cloned.resource_prefix().as_deref(), Some("ocio_test_"));
    assert_eq!(cloned.descriptor_set_index(), 3);
    assert_eq!(cloned.texture_binding_start(), 7);
    assert_eq!(cloned.texture_max_width(0), 4096);
    assert!(cloned.allow_texture_1d());
}

#[test]
fn gpu_shader_desc_extraction_structural_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let desc = extracted_gpu_shader_desc().expect("extracted gpu shader desc");

    assert_eq!(desc.language(), GpuLanguage::Glsl4_0);
    assert_eq!(desc.function_name().as_deref(), Some("ocio_test_main"));
    assert_eq!(desc.pixel_name().as_deref(), Some("ocio_test_pixel"));
    assert_eq!(desc.resource_prefix().as_deref(), Some("ocio_test_"));

    let shader_text = desc.shader_text().expect("shader_text");
    assert!(!shader_text.trim().is_empty());
    assert!(shader_text.contains("ocio_test_main"));

    let uniforms = desc.uniforms();
    assert_eq!(desc.num_uniforms() as usize, uniforms.len());
    for (index, uniform) in uniforms.iter().enumerate() {
        assert!(!uniform.name.trim().is_empty());
        assert_eq!(
            desc.uniform(index as u32).as_ref().map(|u| &u.name),
            Some(&uniform.name)
        );
        assert_eq!(
            desc.uniform_name(index as u32).as_deref(),
            Some(uniform.name.as_str())
        );
        assert_eq!(desc.uniform_value_count(index as u32), uniform.value_count);
        match (&uniform.uniform_type, &uniform.value) {
            (GpuUniformType::VectorInt, GpuUniformValue::I32(values)) => {
                assert_eq!(values.len(), uniform.value_count);
                assert_eq!(desc.uniform_values_i32(index as u32), *values);
                assert!(desc.uniform_values_f32(index as u32).is_empty());
            }
            (GpuUniformType::Unknown, GpuUniformValue::Unsupported) => {
                assert!(desc.uniform_values_f32(index as u32).is_empty());
                assert!(desc.uniform_values_i32(index as u32).is_empty());
            }
            (_, GpuUniformValue::F32(values)) => {
                assert_eq!(values.len(), uniform.value_count);
                assert_eq!(desc.uniform_values_f32(index as u32), *values);
            }
            _ => panic!("uniform type/value mismatch at index {}", index),
        }
    }

    let textures_2d = desc.textures_2d();
    assert_eq!(desc.num_textures() as usize, textures_2d.len());
    for (index, texture) in textures_2d.iter().enumerate() {
        assert!(!texture.texture_name.trim().is_empty());
        assert!(!texture.sampler_name.trim().is_empty());
        assert_eq!(texture.values.len(), texture.expected_value_count());
        assert_eq!(
            desc.texture_value_count(index as u32),
            texture.expected_value_count()
        );
        assert_eq!(desc.texture_values(index as u32), texture.values);
        assert_eq!(
            desc.texture_shader_binding_index(index as u32),
            Some(texture.binding_index)
        );
        let info = desc.texture_info(index as u32).expect("texture_info");
        assert_eq!(info.texture_name, texture.texture_name);
        assert_eq!(info.sampler_name, texture.sampler_name);
        assert_eq!(info.width, texture.width);
        assert_eq!(info.height, texture.height);
        assert_eq!(info.channel, texture.channel as i32);
        assert_eq!(info.dimensions, texture.dimensions as i32);
        assert_eq!(info.interpolation, texture.interpolation as i32);
        assert!(matches!(
            texture.channel,
            GpuTextureChannel::Red | GpuTextureChannel::Rgb
        ));
        assert!(matches!(
            texture.dimensions,
            GpuTextureDimensions::Texture1D | GpuTextureDimensions::Texture2D
        ));
    }

    let textures_3d = desc.textures_3d();
    assert_eq!(desc.num_3d_textures() as usize, textures_3d.len());
    for (index, texture) in textures_3d.iter().enumerate() {
        assert!(!texture.texture_name.trim().is_empty());
        assert!(!texture.sampler_name.trim().is_empty());
        assert_eq!(texture.values.len(), texture.expected_value_count());
        assert_eq!(
            desc.texture_3d_value_count(index as u32),
            texture.expected_value_count()
        );
        assert_eq!(desc.texture_3d_values(index as u32), texture.values);
        assert_eq!(
            desc.texture_3d_shader_binding_index(index as u32),
            Some(texture.binding_index)
        );
    }

    let cloned = desc.clone_desc().expect("clone_desc");
    assert_eq!(cloned.language(), GpuLanguage::Glsl4_0);
    assert_eq!(cloned.function_name().as_deref(), Some("ocio_test_main"));
    assert_eq!(cloned.pixel_name().as_deref(), Some("ocio_test_pixel"));
    assert_eq!(cloned.resource_prefix().as_deref(), Some("ocio_test_"));
}

#[test]
fn gpu_shader_desc_dynamic_property_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let (processor, desc) = extracted_dynamic_gpu_shader_desc().expect("dynamic gpu shader desc");
    assert!(processor.is_dynamic());
    assert!(desc.has_dynamic_property_kind(DynamicPropertyType::Exposure));
    assert!(desc.num_dynamic_properties() >= 1);

    let desc_prop = desc
        .dynamic_property(DynamicPropertyType::Exposure)
        .expect("desc dynamic exposure property");
    assert_eq!(desc_prop.property_type(), DynamicPropertyType::Exposure);
    assert_close(desc_prop.double_value(), 0.0, 1e-8);

    let indexed_types: Vec<_> = (0..desc.num_dynamic_properties())
        .filter_map(|index| desc.dynamic_property_by_index(index))
        .map(|prop| prop.property_type())
        .collect();
    assert!(indexed_types.contains(&DynamicPropertyType::Exposure));

    let processor_prop = processor
        .dynamic_property(DynamicPropertyType::Exposure)
        .expect("processor dynamic exposure property");
    processor_prop.set_double_value(1.0);
    assert_close(desc_prop.double_value(), 0.0, 1e-8);

    desc_prop.set_double_value(-1.0);
    assert_close(processor_prop.double_value(), 1.0, 1e-8);
    let desc_prop_after = desc
        .dynamic_property(DynamicPropertyType::Exposure)
        .expect("desc dynamic exposure property after update");
    assert_close(desc_prop_after.double_value(), -1.0, 1e-8);
}

#[test]
fn gpu_shader_desc_manual_shader_text_assembly_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let desc = GpuShaderDesc::create().expect("gpu shader desc create");
    desc.begin("manual_uid").expect("begin");
    assert_eq!(desc.next_resource_index(), 0);
    assert_eq!(desc.next_resource_index(), 1);
    desc.end();

    desc.add_to_parameter_declare_shader_code("uniform float uGain;\n")
        .expect("parameter declarations");
    desc.add_to_texture_declare_shader_code("uniform sampler3D texLut;\n")
        .expect("texture declarations");
    desc.add_to_helper_shader_code("vec3 passthrough(vec3 c) { return c; }\n")
        .expect("helper methods");
    desc.add_to_function_header_shader_code("vec4 ManualMain(vec4 inPixel) {\n")
        .expect("function header");
    desc.add_to_function_shader_code("  return vec4(passthrough(inPixel.rgb), inPixel.a);\n")
        .expect("function body");
    desc.add_to_function_footer_shader_code("}\n")
        .expect("function footer");
    desc.finalize();

    let shader_text = desc.shader_text().expect("shader_text after finalize");
    assert!(shader_text.contains("uniform float uGain;"));
    assert!(shader_text.contains("uniform sampler3D texLut;"));
    assert!(shader_text.contains("vec3 passthrough"));
    assert!(shader_text.contains("ManualMain"));

    desc.create_shader_text(
        "uniform float uManual;\n",
        "uniform sampler2D texManual;\n",
        "float helperFn(float x) { return x; }\n",
        "vec4 ExplicitMain(vec4 inPixel) {\n",
        "  return vec4(helperFn(inPixel.r), inPixel.g, inPixel.b, inPixel.a);\n",
        "}\n",
    )
    .expect("create_shader_text");

    let rebuilt = desc
        .shader_text()
        .expect("shader_text after explicit build");
    assert!(rebuilt.contains("uniform float uManual;"));
    assert!(rebuilt.contains("uniform sampler2D texManual;"));
    assert!(rebuilt.contains("helperFn"));
    assert!(rebuilt.contains("ExplicitMain"));
}

#[test]
fn gpu_shader_desc_manual_texture_round_trip_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let desc = GpuShaderDesc::create().expect("gpu shader desc create");
    desc.set_descriptor_set_index(2, 5);

    let values_2d = vec![0.1f32, 0.2, 0.3, 0.4, 0.5, 0.6];
    let binding_2d = desc
        .add_texture_2d(
            "manualTex2D",
            "manualSampler2D",
            2,
            1,
            GpuTextureChannel::Rgb,
            GpuTextureDimensions::Texture1D,
            Interpolation::Linear,
            &values_2d,
        )
        .expect("add texture 2d");
    assert_eq!(binding_2d, 5);

    let tex2d = desc.texture_2d(0).expect("texture_2d");
    assert_eq!(tex2d.texture_name, "manualTex2D");
    assert_eq!(tex2d.sampler_name, "manualSampler2D");
    assert_eq!(tex2d.width, 2);
    assert_eq!(tex2d.height, 1);
    assert_eq!(tex2d.channel, GpuTextureChannel::Rgb);
    assert_eq!(tex2d.dimensions, GpuTextureDimensions::Texture1D);
    assert_eq!(tex2d.interpolation, Interpolation::Linear);
    assert_eq!(tex2d.binding_index, 5);
    assert_eq!(tex2d.values, values_2d);

    let values_3d = vec![
        0.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0,
        1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ];
    let binding_3d = desc
        .add_texture_3d(
            "manualTex3D",
            "manualSampler3D",
            2,
            Interpolation::Nearest,
            &values_3d,
        )
        .expect("add texture 3d");
    assert_eq!(binding_3d, 6);

    let tex3d = desc.texture_3d(0).expect("texture_3d");
    assert_eq!(tex3d.texture_name, "manualTex3D");
    assert_eq!(tex3d.sampler_name, "manualSampler3D");
    assert_eq!(tex3d.edge_len, 2);
    assert_eq!(tex3d.interpolation, Interpolation::Nearest);
    assert_eq!(tex3d.binding_index, 6);
    assert_eq!(tex3d.values, values_3d);
}

#[test]
fn gpu_shader_desc_manual_uniform_round_trip_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let desc = GpuShaderDesc::create().expect("gpu shader desc create");
    assert!(desc
        .add_uniform_f64("uExposure", 1.25)
        .expect("add uniform f64"));
    assert!(desc
        .add_uniform_bool("uEnabled", true)
        .expect("add uniform bool"));
    assert!(desc
        .add_uniform_float3("uTint", [0.1, 0.2, 0.3])
        .expect("add uniform float3"));
    assert!(desc
        .add_uniform_f32_array("uCurve", &[0.0, 0.5, 1.0], 4)
        .expect("add uniform f32 array"));
    assert!(desc
        .add_uniform_i32_array("uIndices", &[1, 3, 5], 4)
        .expect("add uniform i32 array"));
    assert!(!desc
        .add_uniform_f64("uExposure", 2.0)
        .expect("duplicate uniform returns false"));

    assert_eq!(desc.num_uniforms(), 5);
    assert!(desc.uniform_buffer_size() > 0);

    let exposure = desc.uniform(0).expect("uniform 0");
    assert_eq!(exposure.name, "uExposure");
    assert_eq!(exposure.uniform_type, GpuUniformType::Double);
    assert_eq!(desc.uniform_values_f32(0), vec![1.25]);

    let enabled = desc.uniform(1).expect("uniform 1");
    assert_eq!(enabled.name, "uEnabled");
    assert_eq!(enabled.uniform_type, GpuUniformType::Bool);
    assert_eq!(desc.uniform_values_f32(1), vec![1.0]);

    let tint = desc.uniform(2).expect("uniform 2");
    assert_eq!(tint.name, "uTint");
    assert_eq!(tint.uniform_type, GpuUniformType::Float3);
    assert_eq!(desc.uniform_values_f32(2), vec![0.1, 0.2, 0.3]);

    let curve = desc.uniform(3).expect("uniform 3");
    assert_eq!(curve.name, "uCurve");
    assert_eq!(curve.uniform_type, GpuUniformType::VectorFloat);
    assert_eq!(curve.value_count, 3);
    assert_eq!(desc.uniform_values_f32(3), vec![0.0, 0.5, 1.0]);

    let indices = desc.uniform(4).expect("uniform 4");
    assert_eq!(indices.name, "uIndices");
    assert_eq!(indices.uniform_type, GpuUniformType::VectorInt);
    assert_eq!(indices.value_count, 3);
    assert_eq!(desc.uniform_values_i32(4), vec![1, 3, 5]);
}

#[test]
fn legacy_gpu_shader_desc_sys_texture_getters_return_real_outputs() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let texture_name = CString::new("legacyTex2D").expect("texture name");
    let sampler_name = CString::new("legacySampler2D").expect("sampler name");
    let texture_values = [0.1f32, 0.2, 0.3, 0.4, 0.5, 0.6];

    let texture3d_name = CString::new("legacyTex3D").expect("3d texture name");
    let sampler3d_name = CString::new("legacySampler3D").expect("3d sampler name");
    let texture3d_values = [
        0.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0,
        1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ];

    unsafe {
        let desc = ocio_sys::ocio_gpu_shader_desc_create();
        assert!(!desc.is_null(), "gpu shader desc handle");

        let binding_2d = ocio_sys::ocio_gpu_shader_desc_add_texture(
            desc,
            texture_name.as_ptr(),
            sampler_name.as_ptr(),
            2,
            1,
            GpuTextureChannel::Rgb as i32,
            GpuTextureDimensions::Texture1D as i32,
            Interpolation::Linear as i32,
            texture_values.as_ptr(),
            texture_values.len(),
        );
        assert!(binding_2d > 0);

        let binding_3d = ocio_sys::ocio_gpu_shader_desc_add3d_texture(
            desc,
            texture3d_name.as_ptr(),
            sampler3d_name.as_ptr(),
            2,
            Interpolation::Nearest as i32,
            texture3d_values.as_ptr(),
            texture3d_values.len(),
        );
        assert_eq!(binding_3d, binding_2d + 1);

        let mut raw_texture_name: *const i8 = std::ptr::null();
        let mut raw_sampler_name: *const i8 = std::ptr::null();
        let mut width = 0u32;
        let mut height = 0u32;
        let mut channel = -1i32;
        let mut dimensions = 0u8;
        let mut interpolation = -1i32;
        ocio_sys::ocio_gpu_shader_desc_get_texture(
            desc,
            std::ptr::null_mut(),
            (&mut raw_texture_name as *mut *const i8).cast(),
            (&mut raw_sampler_name as *mut *const i8).cast(),
            (&mut width as *mut u32).cast::<c_void>(),
            (&mut height as *mut u32).cast::<c_void>(),
            (&mut channel as *mut i32).cast::<c_void>(),
            (&mut dimensions as *mut u8).cast::<c_void>(),
            (&mut interpolation as *mut i32).cast::<c_void>(),
        );

        assert!(!raw_texture_name.is_null(), "legacy texture name ptr");
        assert!(!raw_sampler_name.is_null(), "legacy sampler name ptr");
        assert_eq!(
            CStr::from_ptr(raw_texture_name)
                .to_str()
                .expect("utf8 texture name"),
            "legacyTex2D"
        );
        assert_eq!(
            CStr::from_ptr(raw_sampler_name)
                .to_str()
                .expect("utf8 sampler name"),
            "legacySampler2D"
        );
        assert_eq!(width, 2);
        assert_eq!(height, 1);
        assert_eq!(channel, GpuTextureChannel::Rgb as i32);
        assert_eq!(dimensions, GpuTextureDimensions::Texture1D as u8);
        assert_eq!(interpolation, Interpolation::Linear as i32);

        let mut raw_texture_values: *const f32 = std::ptr::null();
        ocio_sys::ocio_gpu_shader_desc_get_texture_values(
            desc,
            std::ptr::null_mut(),
            (&mut raw_texture_values as *mut *const f32).cast::<c_void>(),
        );
        assert!(!raw_texture_values.is_null(), "legacy texture values ptr");
        assert_eq!(
            std::slice::from_raw_parts(raw_texture_values, texture_values.len()),
            texture_values
        );

        let mut raw_texture3d_name: *const i8 = std::ptr::null();
        let mut raw_sampler3d_name: *const i8 = std::ptr::null();
        let mut edge_len = 0u32;
        let mut interpolation3d = -1i32;
        ocio_sys::ocio_gpu_shader_desc_get3d_texture(
            desc,
            std::ptr::null_mut(),
            (&mut raw_texture3d_name as *mut *const i8).cast(),
            (&mut raw_sampler3d_name as *mut *const i8).cast(),
            (&mut edge_len as *mut u32).cast::<c_void>(),
            (&mut interpolation3d as *mut i32).cast::<c_void>(),
        );

        assert!(!raw_texture3d_name.is_null(), "legacy 3d texture name ptr");
        assert!(!raw_sampler3d_name.is_null(), "legacy 3d sampler name ptr");
        assert_eq!(
            CStr::from_ptr(raw_texture3d_name)
                .to_str()
                .expect("utf8 3d texture name"),
            "legacyTex3D"
        );
        assert_eq!(
            CStr::from_ptr(raw_sampler3d_name)
                .to_str()
                .expect("utf8 3d sampler name"),
            "legacySampler3D"
        );
        assert_eq!(edge_len, 2);
        assert_eq!(interpolation3d, Interpolation::Nearest as i32);

        let mut raw_texture3d_values: *const f32 = std::ptr::null();
        ocio_sys::ocio_gpu_shader_desc_get3d_texture_values(
            desc,
            std::ptr::null_mut(),
            (&mut raw_texture3d_values as *mut *const f32).cast::<c_void>(),
        );
        assert!(
            !raw_texture3d_values.is_null(),
            "legacy 3d texture values ptr"
        );
        assert_eq!(
            std::slice::from_raw_parts(raw_texture3d_values, texture3d_values.len()),
            texture3d_values
        );

        ocio_sys::ocio_gpu_shader_desc_destroy(desc);
    }
}
