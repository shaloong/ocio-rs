use ocio_rs::transform::{ExposureContrastTransform, Lut3DTransform, MatrixTransform};
use ocio_rs::{
    Config, DynamicPropertyType, GpuLanguage, GpuShaderDesc, GpuUniformType, Processor,
    TransformDirection,
};
use wgpu::util::DeviceExt;

fn validate_case(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    name: &str,
    processor: &Processor,
    exposure: Option<f64>,
    expect_3d_lut: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let expect_uniform = exposure.is_some();
    let cpu = processor.default_cpu_processor()?;
    let gpu = processor.default_gpu_processor()?;
    let mut desc = GpuShaderDesc::create()?;
    desc.set_language(GpuLanguage::GlslVk4_6)?;
    desc.set_function_name("ocio_transform")?;
    gpu.try_extract_shader_info(&mut desc)?;
    if let Some(value) = exposure {
        cpu.dynamic_property(DynamicPropertyType::Exposure)?
            .set_double_value(value)?;
        desc.try_dynamic_property(DynamicPropertyType::Exposure)?
            .ok_or("missing GPU exposure property")?
            .set_double_value(value)?;
    }
    if desc.num_textures() != 0
        || desc.num_3d_textures() > 1
        || (expect_uniform && desc.num_3d_textures() != 0)
    {
        return Err(format!("{name}: unsupported resource combination").into());
    }
    if (desc.num_3d_textures() == 1) != expect_3d_lut {
        return Err(format!(
            "{name}: unexpected 3D texture count {}",
            desc.num_3d_textures()
        )
        .into());
    }
    if (desc.num_uniforms() != 0) != expect_uniform {
        return Err(format!("{name}: unexpected uniform count {}", desc.num_uniforms()).into());
    }
    let mut uniform_bytes = vec![0u8; desc.try_uniform_buffer_size()?];
    for index in 0..desc.num_uniforms() {
        let uniform = desc.try_uniform(index)?.ok_or("missing uniform")?;
        if uniform.uniform_type != GpuUniformType::Double {
            return Err(
                format!("{name}: unexpected uniform type {:?}", uniform.uniform_type).into(),
            );
        }
        let offset = uniform.buffer_offset.ok_or("missing uniform offset")?;
        let value = desc
            .try_uniform_f64_value(index)?
            .ok_or("missing uniform value")? as f32;
        let end = offset.checked_add(4).ok_or("uniform offset overflow")?;
        uniform_bytes
            .get_mut(offset..end)
            .ok_or("uniform offset outside buffer")?
            .copy_from_slice(&value.to_ne_bytes());
    }
    let mut shader = desc.try_shader_text()?.ok_or("missing GPU shader")?;
    let lut = if desc.num_3d_textures() == 1 {
        Some(desc.try_texture_3d(0)?.ok_or("missing 3D texture")?)
    } else {
        None
    };
    if let Some(lut) = &lut {
        let binding = lut.binding_index.ok_or("missing 3D texture binding")?;
        let declaration = format!("uniform sampler3D {};", lut.sampler_name);
        if !shader.contains(&declaration) {
            return Err("missing expected OCIO sampler declaration".into());
        }
        let split = format!("uniform texture3D ocio_validation_texture;\nlayout(set=0,binding={}) uniform sampler ocio_validation_sampler;", binding + 1);
        shader = shader.replace(&declaration, &split);
        let call = format!("texture({},", lut.sampler_name);
        if shader.matches(&call).count() != 1 {
            return Err("unexpected number of OCIO LUT sampling calls".into());
        }
        shader = shader.replace(
            &call,
            "textureLod(sampler3D(ocio_validation_texture, ocio_validation_sampler),",
        );
        let start = shader.find("textureLod(").ok_or("missing texture call")?;
        let mut depth = 0usize;
        let mut closing = None;
        for (offset, ch) in shader[start..].char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        closing = Some(start + offset);
                        break;
                    }
                }
                _ => {}
            }
        }
        shader.insert_str(closing.ok_or("unbalanced texture call")?, ", 0.0");
    }
    let pixel_group = usize::from(expect_uniform || lut.is_some());
    let source = format!("#version 450\n{shader}\nlayout(local_size_x=1) in;\nlayout(set={pixel_group},binding=0,std430) buffer Pixels {{ vec4 value[]; }} pixels;\nvoid main() {{ uint i = gl_GlobalInvocationID.x; pixels.value[i] = ocio_transform(pixels.value[i]); }}\n");
    let input: [[f32; 4]; 4] = [
        [0.13, 0.47, 0.81, 0.75],
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 0.5, 0.25, 0.0],
        [0.8, 0.4, 0.1, 0.5],
    ];
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(name),
        source: wgpu::ShaderSource::Glsl {
            shader: source.into(),
            stage: wgpu::naga::ShaderStage::Compute,
            defines: &[],
        },
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some(name),
        layout: None,
        module: &module,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });
    let bytes = bytemuck::cast_slice(&input);
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("pixels"),
        contents: bytes,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: bytes.len() as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("pixel bindings"),
        layout: &pipeline.get_bind_group_layout(pixel_group as u32),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });
    let uniform_buffer = expect_uniform.then(|| {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("OCIO uniforms"),
            contents: &uniform_bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    });
    let uniform_bindings = uniform_buffer.as_ref().map(|buffer| {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("OCIO uniform bindings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        })
    });
    let mut lut_resources = None;
    if let Some(lut) = &lut {
        if lut.interpolation != ocio_rs::Interpolation::Linear {
            return Err("test requires a linear 3D LUT".into());
        }
        let extent = wgpu::Extent3d {
            width: lut.edge_len,
            height: lut.edge_len,
            depth_or_array_layers: lut.edge_len,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("OCIO 3D LUT"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let rgba: Vec<f32> = lut
            .values
            .chunks_exact(3)
            .flat_map(|rgb| [rgb[0], rgb[1], rgb[2], 1.0])
            .collect();
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(&rgba),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(lut.edge_len * 16),
                rows_per_image: Some(lut.edge_len),
            },
            extent,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let binding = lut.binding_index.unwrap();
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("OCIO 3D LUT bindings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: binding + 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        lut_resources = Some((texture, view, sampler, group));
    }
    for pass_number in 0..if expect_uniform { 2 } else { 1 } {
        if pass_number == 1 {
            let value = -0.75;
            cpu.dynamic_property(DynamicPropertyType::Exposure)?
                .set_double_value(value)?;
            desc.try_dynamic_property(DynamicPropertyType::Exposure)?
                .ok_or("missing GPU exposure property")?
                .set_double_value(value)?;
            for index in 0..desc.num_uniforms() {
                let uniform = desc.try_uniform(index)?.ok_or("missing uniform")?;
                let offset = uniform.buffer_offset.ok_or("missing uniform offset")?;
                let value = desc
                    .try_uniform_f64_value(index)?
                    .ok_or("missing uniform value")? as f32;
                uniform_bytes[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
            }
            queue.write_buffer(
                uniform_buffer.as_ref().ok_or("missing uniform buffer")?,
                0,
                &uniform_bytes,
            );
            queue.write_buffer(&buffer, 0, bytes);
        }
        let mut expected = input;
        for pixel in &mut expected {
            cpu.apply_rgba(pixel);
        }
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            pass.set_pipeline(&pipeline);
            if let Some(uniforms) = &uniform_bindings {
                pass.set_bind_group(0, uniforms, &[]);
            }
            if let Some((_, _, _, group)) = &lut_resources {
                pass.set_bind_group(0, group, &[]);
            }
            pass.set_bind_group(pixel_group as u32, &bindings, &[]);
            pass.dispatch_workgroups(input.len() as u32, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&buffer, 0, &readback, 0, bytes.len() as u64);
        queue.submit([encoder.finish()]);
        let slice = readback.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        device.poll(wgpu::PollType::wait_indefinitely())?;
        receiver.recv()??;
        let mapped = slice.get_mapped_range()?;
        let actual: &[f32] = bytemuck::cast_slice(&mapped);
        // Hardware 3D filtering has finite interpolation precision. Use a
        // separate bound (OCIO's upstream GPU suite also allows up to 1e-2
        // for some LUTs); analytic transforms retain the tighter 1e-5 bound.
        let tolerance = if lut.is_some() { 5e-3 } else { 1e-5 };
        let mut max_error = 0.0_f32;
        for (channel, (&gpu_value, &cpu_value)) in
            actual.iter().zip(expected.iter().flatten()).enumerate()
        {
            max_error = max_error.max((gpu_value - cpu_value).abs());
            if !gpu_value.is_finite() || (gpu_value - cpu_value).abs() > tolerance {
                return Err(format!(
                    "{name}: scalar {channel}: GPU {gpu_value} != CPU {cpu_value}"
                )
                .into());
            }
        }
        println!(
            "{name} pass {}: {} pixels match CPU (max error {max_error:.6})",
            pass_number + 1,
            input.len()
        );
        drop(mapped);
        readback.unmap();
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut instance_desc = wgpu::InstanceDescriptor::new_without_display_handle();
    instance_desc.backends = wgpu::Backends::VULKAN;
    let instance = wgpu::Instance::new(instance_desc);
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
        ..Default::default()
    }))?;
    let info = adapter.get_info();
    if !matches!(
        info.device_type,
        wgpu::DeviceType::DiscreteGpu | wgpu::DeviceType::IntegratedGpu
    ) {
        return Err(format!("adapter is not a physical GPU: {:?}", info.device_type).into());
    }
    if !adapter
        .features()
        .contains(wgpu::Features::FLOAT32_FILTERABLE)
    {
        return Err("adapter does not support filtered 32-bit float textures".into());
    }
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        required_features: wgpu::Features::FLOAT32_FILTERABLE,
        ..Default::default()
    }))?;
    println!("Hardware adapter: {} ({:?})", info.name, info.backend);
    let config = Config::raw()?;
    let matrix = MatrixTransform::scale(&[1.7, 0.6, 1.2, 1.0])?;
    matrix.set_offset(&[0.1, -0.2, 0.05, 0.0])?;
    let processor = config.processor_from_transform(&matrix, TransformDirection::Forward)?;
    validate_case(&device, &queue, "matrix", &processor, None, false)?;
    let exposure = ExposureContrastTransform::create()?;
    exposure.set_exposure(1.1);
    exposure.make_exposure_dynamic();
    let processor = config.processor_from_transform(&exposure, TransformDirection::Forward)?;
    validate_case(
        &device,
        &queue,
        "dynamic exposure +1.1",
        &processor,
        Some(1.1),
        false,
    )?;
    let lut = Lut3DTransform::create_with_grid_size(2)?;
    for r in 0..2 {
        for g in 0..2 {
            for b in 0..2 {
                lut.set_value(r, g, b, [r as f32 * 0.8, g as f32 * 0.7, b as f32 * 0.6])?;
            }
        }
    }
    let processor = config.processor_from_transform(&lut, TransformDirection::Forward)?;
    validate_case(&device, &queue, "3D LUT", &processor, None, true)?;
    Ok(())
}
