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
use ocio_rs::{GpuLanguage, GpuShaderDesc, TransformDirection};

fn gpu_shader_desc_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
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
    desc.set_resource_prefix("ocio_test_")
        .expect("set_resource_prefix");

    assert_eq!(desc.language(), GpuLanguage::Glsl4_0);
    assert_eq!(desc.function_name().as_deref(), Some("ocio_test_main"));
    assert_eq!(desc.pixel_name().as_deref(), Some("ocio_test_pixel"));
    assert_eq!(desc.resource_prefix().as_deref(), Some("ocio_test_"));

    let cloned = desc.clone_desc().expect("clone_desc");
    assert_eq!(cloned.language(), GpuLanguage::Glsl4_0);
    assert_eq!(cloned.function_name().as_deref(), Some("ocio_test_main"));
    assert_eq!(cloned.pixel_name().as_deref(), Some("ocio_test_pixel"));
    assert_eq!(cloned.resource_prefix().as_deref(), Some("ocio_test_"));
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
        let f32_values = desc.uniform_values_f32(index as u32);
        let i32_values = desc.uniform_values_i32(index as u32);
        if !i32_values.is_empty() {
            assert_eq!(i32_values.len(), uniform.value_count);
            assert!(f32_values.is_empty());
        } else if !f32_values.is_empty() {
            assert_eq!(f32_values.len(), uniform.value_count);
        } else {
            assert_eq!(uniform.value_count, 0);
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
