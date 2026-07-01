//! Processor and GPUProcessor behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke coverage in
//! `src/processor.rs`. In bundled/real mode they validate processor metadata,
//! CPU/GPU execution-path helpers, and `create_group_transform()` round trips.

mod common;
use common::*;

use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::transform::{MatrixTransform, Transform};
use ocio_rs::{GpuLanguage, GpuShaderDesc, TransformDirection};

fn processor_behavior_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn scaled_matrix_processor() -> Option<ocio_rs::Processor> {
    let config = create_test_config()?;
    let transform = MatrixTransform::scale(&[1.1, 0.9, 1.2, 1.0]).ok()?;
    config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()
}

fn extract_shader_text(
    gpu: &ocio_rs::GPUProcessor,
    function_name: &str,
    pixel_name: &str,
) -> Option<GpuShaderDesc> {
    let desc = GpuShaderDesc::create().ok()?;
    desc.set_language(GpuLanguage::Glsl4_0);
    desc.set_function_name(function_name).ok()?;
    desc.set_pixel_name(pixel_name).ok()?;
    desc.set_resource_prefix("ocio_processor_test_").ok()?;
    let mut desc = desc;
    gpu.extract_shader_info(&mut desc);
    Some(desc)
}

#[test]
fn processor_group_transform_and_metadata_behavior() {
    let _guard = processor_behavior_test_lock();
    if is_stub() {
        return;
    }

    let processor = scaled_matrix_processor().expect("scaled processor");

    assert!(!processor.is_no_op());
    assert!(!processor.has_channel_crosstalk());
    assert!(!processor.is_dynamic());
    assert!(processor.num_transforms() > 0);

    let cache_id = processor.cache_id().expect("processor cache_id");
    assert!(!cache_id.trim().is_empty());

    let format_metadata = processor
        .format_metadata()
        .expect("processor format metadata");
    assert!(format_metadata.num_children() >= 0);
    let processor_metadata = processor
        .processor_metadata()
        .expect("processor metadata handle");
    assert!(processor_metadata.num_files() >= 0);
    assert!(processor_metadata.num_looks() >= 0);
    assert!(
        processor.transform_format_metadata(0).is_some(),
        "expected transform metadata for first op"
    );

    let group = processor
        .create_group_transform()
        .expect("group transform from processor");
    assert!(group.num_transforms() > 0);
    assert_eq!(group.direction(), TransformDirection::Forward);

    let first = group.transform(0).expect("first group transform");
    assert!(
        matches!(
            first,
            Transform::Matrix(_) | Transform::Range(_) | Transform::Allocation(_)
        ),
        "unexpected first transform kind from processor group: {:?}",
        std::mem::discriminant(&first)
    );

    let config = create_test_config().expect("raw config");
    let group_processor = config
        .processor_from_transform(&group, TransformDirection::Forward)
        .expect("processor from group");
    let group_cpu = group_processor
        .default_cpu_processor()
        .expect("group cpu processor");
    let direct_cpu = processor
        .default_cpu_processor()
        .expect("direct cpu processor");

    let mut direct_pixel = [0.25f32, 0.5, 0.75, 1.0];
    let mut group_pixel = direct_pixel;
    direct_cpu.apply_rgba(&mut direct_pixel);
    group_cpu.apply_rgba(&mut group_pixel);

    assert_close(direct_pixel[0] as f64, group_pixel[0] as f64, 1e-6);
    assert_close(direct_pixel[1] as f64, group_pixel[1] as f64, 1e-6);
    assert_close(direct_pixel[2] as f64, group_pixel[2] as f64, 1e-6);
    assert_close(direct_pixel[3] as f64, group_pixel[3] as f64, 1e-6);
}

#[test]
fn processor_cpu_and_gpu_helpers_match_scaled_matrix_behavior() {
    let _guard = processor_behavior_test_lock();
    if is_stub() {
        return;
    }

    let processor = scaled_matrix_processor().expect("scaled processor");

    let default_cpu = processor.default_cpu_processor().expect("default cpu");
    let optimized_cpu = processor.optimized_cpu_processor(0).expect("optimized cpu");

    assert!(!default_cpu.is_no_op());
    assert!(!optimized_cpu.is_no_op());
    assert!(!default_cpu.has_channel_crosstalk());
    assert!(!optimized_cpu.has_channel_crosstalk());
    assert!(!default_cpu.is_identity());
    assert!(!optimized_cpu.is_identity());
    assert_eq!(
        default_cpu.input_bit_depth(),
        optimized_cpu.input_bit_depth()
    );
    assert_eq!(
        default_cpu.output_bit_depth(),
        optimized_cpu.output_bit_depth()
    );
    assert!(!default_cpu
        .cache_id()
        .expect("default cpu cache id")
        .trim()
        .is_empty());
    assert!(!optimized_cpu
        .cache_id()
        .expect("optimized cpu cache id")
        .trim()
        .is_empty());

    let original = [0.25f32, 0.5, 0.75, 0.6];
    let mut via_default = original;
    let mut via_optimized = original;
    default_cpu.apply_rgba(&mut via_default);
    optimized_cpu.apply_rgba(&mut via_optimized);

    assert_close(via_default[0] as f64, 0.275, 1e-6);
    assert_close(via_default[1] as f64, 0.45, 1e-6);
    assert_close(via_default[2] as f64, 0.9, 1e-6);
    assert_close(via_default[3] as f64, 0.6, 1e-6);
    assert_close(via_optimized[0] as f64, via_default[0] as f64, 1e-6);
    assert_close(via_optimized[1] as f64, via_default[1] as f64, 1e-6);
    assert_close(via_optimized[2] as f64, via_default[2] as f64, 1e-6);
    assert_close(via_optimized[3] as f64, via_default[3] as f64, 1e-6);

    let default_gpu = processor.default_gpu_processor().expect("default gpu");
    let optimized_gpu = processor.optimized_gpu_processor(0).expect("optimized gpu");

    assert!(!default_gpu.is_no_op());
    assert!(!optimized_gpu.is_no_op());
    assert!(!default_gpu.has_channel_crosstalk());
    assert!(!optimized_gpu.has_channel_crosstalk());
    assert!(!default_gpu
        .cache_id()
        .expect("default gpu cache id")
        .trim()
        .is_empty());
    assert!(!optimized_gpu
        .cache_id()
        .expect("optimized gpu cache id")
        .trim()
        .is_empty());

    let default_desc = extract_shader_text(
        &default_gpu,
        "ocio_default_gpu_main",
        "ocio_default_gpu_pixel",
    )
    .expect("default gpu shader desc");
    let optimized_desc = extract_shader_text(
        &optimized_gpu,
        "ocio_optimized_gpu_main",
        "ocio_optimized_gpu_pixel",
    )
    .expect("optimized gpu shader desc");

    let default_shader = default_desc.shader_text().expect("default shader text");
    let optimized_shader = optimized_desc.shader_text().expect("optimized shader text");
    assert!(!default_shader.trim().is_empty());
    assert!(!optimized_shader.trim().is_empty());
    assert!(default_shader.contains("ocio_default_gpu_main"));
    assert!(optimized_shader.contains("ocio_optimized_gpu_main"));
    assert_eq!(default_desc.num_uniforms(), optimized_desc.num_uniforms());
    assert_eq!(default_desc.num_textures(), optimized_desc.num_textures());
    assert_eq!(
        default_desc.num_3d_textures(),
        optimized_desc.num_3d_textures()
    );
}

#[allow(deprecated)]
#[test]
fn processor_legacy_gpu_helper_emits_real_shader_behavior() {
    let _guard = processor_behavior_test_lock();
    if is_stub() {
        return;
    }

    let processor = scaled_matrix_processor().expect("scaled processor");
    let legacy_gpu = processor
        .optimized_legacy_gpu_processor(0, 16)
        .expect("legacy gpu");

    assert!(!legacy_gpu.is_no_op());
    assert!(!legacy_gpu.has_channel_crosstalk());
    assert!(!legacy_gpu
        .cache_id()
        .expect("legacy gpu cache id")
        .trim()
        .is_empty());

    let desc = extract_shader_text(&legacy_gpu, "ocio_legacy_gpu_main", "ocio_legacy_gpu_pixel")
        .expect("legacy gpu shader desc");
    let shader = desc.shader_text().expect("legacy shader text");

    assert!(!shader.trim().is_empty());
    assert!(shader.contains("ocio_legacy_gpu_main"));
    assert_eq!(
        desc.function_name().as_deref(),
        Some("ocio_legacy_gpu_main")
    );
    assert_eq!(desc.pixel_name().as_deref(), Some("ocio_legacy_gpu_pixel"));
}
