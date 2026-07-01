//! GPU shader descriptor behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after unit-level smoke coverage.
//! In bundled/real mode they validate that shader descriptor configuration
//! round-trips and that extracted descriptor payloads are structurally
//! consistent with the reported uniform/texture metadata.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::MatrixTransform;
use ocio_rs::{
    DynamicPropertyType, ExposureContrastStyle, GpuLanguage, GpuShaderDesc, GpuTextureChannel,
    GpuTextureDimensions, GpuUniformType, GpuUniformValue, TransformDirection,
};
use ocio_rs::transform::ExposureContrastTransform;

fn gpu_shader_desc_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK
        .get_or_init(|| Mutex::new(()))
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
    desc.set_unique_id("ocio-test-uid")
        .expect("set_unique_id");
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

    let rebuilt = desc.shader_text().expect("shader_text after explicit build");
    assert!(rebuilt.contains("uniform float uManual;"));
    assert!(rebuilt.contains("uniform sampler2D texManual;"));
    assert!(rebuilt.contains("helperFn"));
    assert!(rebuilt.contains("ExplicitMain"));
}
