use ocio_rs::transform::{
    ExposureContrastTransform, FixedFunctionTransform, Lut1DTransform, Lut3DTransform,
    MatrixTransform,
};
use ocio_rs::{
    Config, DynamicPropertyType, FixedFunctionStyle, GpuLanguage, GpuShaderDesc, GpuTextureChannel,
    GpuTextureDimensions, GpuUniformType, Interpolation, Processor, ShaderCodeSection,
    ShaderExtractionHooks, TransformDirection,
};
use wgpu::util::DeviceExt;

#[derive(Clone, Copy, PartialEq, Eq)]
enum TextureKind {
    None,
    OneD,
    TwoD,
    ThreeD,
}

struct LutTexture {
    sampler: String,
    binding: u32,
    width: u32,
    height: u32,
    depth: u32,
    kind: TextureKind,
    channel: GpuTextureChannel,
    interpolation: Interpolation,
    values: Vec<f32>,
}

#[allow(clippy::too_many_arguments)]
fn validate_case(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    name: &str,
    processor: &Processor,
    exposure: Option<f64>,
    expected_texture: TextureKind,
    tolerance: f32,
    input_override: Option<&[[f32; 4]]>,
    use_hooks: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let expect_uniform = exposure.is_some();
    let cpu = processor.default_cpu_processor()?;
    let gpu = processor.default_gpu_processor()?;
    let mut desc = GpuShaderDesc::create()?;
    desc.set_language(GpuLanguage::GlslVk4_6)?;
    desc.set_function_name("ocio_transform")?;
    desc.set_allow_texture_1d(expected_texture == TextureKind::OneD)?;
    if use_hooks {
        struct MarkBody {
            seen: bool,
            modified_footer: bool,
        }
        impl ShaderExtractionHooks for MarkBody {
            fn code(
                &mut self,
                section: ShaderCodeSection,
                code: &str,
            ) -> ocio_rs::Result<Option<String>> {
                if section == ShaderCodeSection::Body {
                    self.seen = true;
                    Ok(Some(format!("// GPU validation hook\n{code}")))
                } else if section == ShaderCodeSection::Footer && code.contains("return outColor;")
                {
                    self.modified_footer = true;
                    Ok(Some(code.replace(
                        "return outColor;",
                        "outColor.rgb *= 0.5;\n  return outColor;",
                    )))
                } else {
                    Ok(None)
                }
            }
        }
        let mut hooks = MarkBody {
            seen: false,
            modified_footer: false,
        };
        desc = gpu.try_extract_with_hooks(&desc, &mut hooks)?;
        if !hooks.seen
            || !hooks.modified_footer
            || !desc
                .try_shader_text()?
                .is_some_and(|text| text.contains("// GPU validation hook"))
        {
            return Err("shader code hook did not modify generated text".into());
        }
    } else {
        gpu.try_extract_shader_info(&mut desc)?;
    }
    if let Some(value) = exposure {
        cpu.dynamic_property(DynamicPropertyType::Exposure)?
            .set_double_value(value)?;
        desc.try_dynamic_property(DynamicPropertyType::Exposure)?
            .ok_or("missing GPU exposure property")?
            .set_double_value(value)?;
    }
    let expected_counts = match expected_texture {
        TextureKind::None => (0, 0),
        TextureKind::OneD | TextureKind::TwoD => (1, 0),
        TextureKind::ThreeD => (0, 1),
    };
    if (desc.num_textures(), desc.num_3d_textures()) != expected_counts
        || (expect_uniform && expected_texture != TextureKind::None)
    {
        return Err(format!("{name}: unexpected or unsupported texture resources").into());
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
    let lut = match expected_texture {
        TextureKind::None => None,
        TextureKind::OneD | TextureKind::TwoD => {
            let t = desc.try_texture_2d(0)?.ok_or("missing 1D LUT texture")?;
            let actual = match t.dimensions {
                GpuTextureDimensions::Texture1D => TextureKind::OneD,
                GpuTextureDimensions::Texture2D => TextureKind::TwoD,
            };
            if actual != expected_texture {
                return Err("unexpected LUT texture dimension".into());
            }
            Some(LutTexture {
                sampler: t.sampler_name,
                binding: t.binding_index.ok_or("missing LUT binding")?,
                width: t.width,
                height: t.height,
                depth: 1,
                kind: actual,
                channel: t.channel,
                interpolation: t.interpolation,
                values: t.values,
            })
        }
        TextureKind::ThreeD => {
            let t = desc.try_texture_3d(0)?.ok_or("missing 3D texture")?;
            Some(LutTexture {
                sampler: t.sampler_name,
                binding: t.binding_index.ok_or("missing LUT binding")?,
                width: t.edge_len,
                height: t.edge_len,
                depth: t.edge_len,
                kind: TextureKind::ThreeD,
                channel: GpuTextureChannel::Rgb,
                interpolation: t.interpolation,
                values: t.values,
            })
        }
    };
    if let Some(lut) = &lut {
        let dimension = match lut.kind {
            TextureKind::OneD => "1D",
            TextureKind::TwoD => "2D",
            TextureKind::ThreeD => "3D",
            TextureKind::None => unreachable!(),
        };
        let declaration = format!("uniform sampler{dimension} {};", lut.sampler);
        if !shader.contains(&declaration) {
            return Err("missing expected OCIO sampler declaration".into());
        }
        let split = format!("uniform texture{dimension} ocio_validation_texture;\nlayout(set=0,binding={}) uniform sampler ocio_validation_sampler;", lut.binding.checked_add(1).ok_or("binding overflow")?);
        shader = shader.replace(&declaration, &split);
        let call = format!("texture({},", lut.sampler);
        let count = shader.matches(&call).count();
        let valid_count = match (lut.kind, lut.interpolation) {
            (TextureKind::ThreeD, Interpolation::Linear) => count == 1,
            (TextureKind::ThreeD, Interpolation::Nearest) => count >= 4,
            (TextureKind::OneD | TextureKind::TwoD, Interpolation::Linear) => count == 3,
            _ => false,
        };
        if !valid_count {
            return Err("unexpected number of OCIO LUT sampling calls".into());
        }
        shader = shader.replace(
            &call,
            &format!(
                "textureLod(sampler{dimension}(ocio_validation_texture, ocio_validation_sampler),"
            ),
        );
        let mut search_from = 0;
        for _ in 0..count {
            let start = search_from
                + shader[search_from..]
                    .find("textureLod(")
                    .ok_or("missing texture call")?;
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
            let close = closing.ok_or("unbalanced texture call")?;
            shader.insert_str(close, ", 0.0");
            search_from = close + 6;
        }
    }
    let pixel_group = usize::from(expect_uniform || lut.is_some());
    let source = format!("#version 450\n{shader}\nlayout(local_size_x=1) in;\nlayout(set={pixel_group},binding=0,std430) buffer Pixels {{ vec4 value[]; }} pixels;\nvoid main() {{ uint i = gl_GlobalInvocationID.x; pixels.value[i] = ocio_transform(pixels.value[i]); }}\n");
    let default_input: [[f32; 4]; 4] = [
        [0.13, 0.47, 0.81, 0.75],
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 0.5, 0.25, 0.0],
        [0.8, 0.4, 0.1, 0.5],
    ];
    let input = input_override.unwrap_or(&default_input);
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
    let bytes = bytemuck::cast_slice(input);
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
        let filter = match lut.interpolation {
            Interpolation::Linear => wgpu::FilterMode::Linear,
            Interpolation::Nearest => wgpu::FilterMode::Nearest,
            _ => return Err("unsupported native LUT sampler interpolation".into()),
        };
        let extent = wgpu::Extent3d {
            width: lut.width,
            height: lut.height,
            depth_or_array_layers: lut.depth,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("OCIO 3D LUT"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: match lut.kind {
                TextureKind::OneD => wgpu::TextureDimension::D1,
                TextureKind::TwoD => wgpu::TextureDimension::D2,
                TextureKind::ThreeD => wgpu::TextureDimension::D3,
                TextureKind::None => unreachable!(),
            },
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let rgba: Vec<f32> = match lut.channel {
            GpuTextureChannel::Rgb => lut
                .values
                .chunks_exact(3)
                .flat_map(|rgb| [rgb[0], rgb[1], rgb[2], 1.0])
                .collect(),
            GpuTextureChannel::Red => lut
                .values
                .iter()
                .flat_map(|&r| [r, 0.0, 0.0, 1.0])
                .collect(),
        };
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
                bytes_per_row: Some(lut.width * 16),
                rows_per_image: Some(lut.height),
            },
            extent,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: filter,
            min_filter: filter,
            ..Default::default()
        });
        let binding = lut.binding;
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
        let mut expected = input.to_vec();
        for pixel in &mut expected {
            cpu.apply_rgba(pixel);
            if use_hooks {
                for channel in &mut pixel[..3] {
                    *channel *= 0.5;
                }
            }
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
    validate_case(
        &device,
        &queue,
        "matrix",
        &processor,
        None,
        TextureKind::None,
        1e-5,
        None,
        false,
    )?;
    validate_case(
        &device,
        &queue,
        "matrix with shader hooks",
        &processor,
        None,
        TextureKind::None,
        1e-5,
        None,
        true,
    )?;
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
        TextureKind::None,
        1e-5,
        None,
        false,
    )?;
    let lut = Lut3DTransform::create_with_grid_size(2)?;
    for r in 0..2 {
        for g in 0..2 {
            for b in 0..2 {
                let (r, g, b) = (r as f32, g as f32, b as f32);
                lut.set_value(
                    r as u64,
                    g as u64,
                    b as u64,
                    [
                        r * 0.8 + g * b * 0.1,
                        g * 0.7 + r * b * 0.1,
                        b * 0.6 + r * g * 0.1,
                    ],
                )?;
            }
        }
    }
    let processor = config.processor_from_transform(&lut, TransformDirection::Forward)?;
    let linear_cpu = processor.default_cpu_processor()?;
    validate_case(
        &device,
        &queue,
        "3D LUT",
        &processor,
        None,
        TextureKind::ThreeD,
        5e-3,
        None,
        false,
    )?;
    lut.try_set_interpolation(Interpolation::Tetrahedral)?;
    let processor = config.processor_from_transform(&lut, TransformDirection::Forward)?;
    let tetra_pixels = [
        [0.8, 0.5, 0.2, 1.0],
        [0.8, 0.2, 0.5, 0.5],
        [0.5, 0.2, 0.8, 0.0],
        [0.2, 0.5, 0.8, 1.0],
        [0.2, 0.8, 0.5, 0.75],
        [0.5, 0.8, 0.2, 1.0],
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    ];
    let mut linear_value = tetra_pixels[0];
    let mut tetra_value = tetra_pixels[0];
    linear_cpu.try_apply_rgba(&mut linear_value)?;
    processor
        .default_cpu_processor()?
        .try_apply_rgba(&mut tetra_value)?;
    if linear_value
        .iter()
        .zip(tetra_value)
        .all(|(&a, b)| (a - b).abs() < 1e-3)
    {
        return Err(
            "3D LUT fixture does not distinguish trilinear from tetrahedral interpolation".into(),
        );
    }
    validate_case(
        &device,
        &queue,
        "3D LUT tetrahedral",
        &processor,
        None,
        TextureKind::ThreeD,
        1e-6,
        Some(&tetra_pixels),
        true,
    )?;
    let lut = Lut1DTransform::create()?;
    lut.set_length(3)?;
    lut.set_values(&[0.0, 0.0, 0.0, 0.2, 0.3, 0.4, 1.0, 1.0, 1.0])?;
    let processor = config.processor_from_transform(&lut, TransformDirection::Forward)?;
    validate_case(
        &device,
        &queue,
        "1D LUT in 2D texture",
        &processor,
        None,
        TextureKind::TwoD,
        5e-3,
        None,
        false,
    )?;
    validate_case(
        &device,
        &queue,
        "1D LUT in 1D texture",
        &processor,
        None,
        TextureKind::OneD,
        5e-3,
        None,
        false,
    )?;
    let hmj = FixedFunctionTransform::create_with_params(
        FixedFunctionStyle::AcesRgbToHmj20,
        &[0.7347, 0.2653, 0.0, 1.0, 0.0001, -0.0770, 0.32168, 0.33767],
    )?;
    let rgb = [
        [2.781809, 0.17917825, -0.02210353, 1.0],
        [3.3445237, 3.6178627, -0.006002689, 1.0],
        [0.11, 0.02, 0.04, 0.5],
        [0.43, 0.82, 0.71, 0.0],
    ];
    // Corresponding ACES test values in the documented [h/360, M/200, J/100] order.
    let hmj_input = [
        [25.02511 / 360.0, 206.8273 / 200.0, 107.48064 / 100.0, 1.0],
        [106.18345 / 360.0, 133.33089 / 200.0, 173.19408 / 100.0, 1.0],
        [4.173158 / 360.0, 42.523605 / 200.0, 26.112514 / 100.0, 0.5],
        [182.92575 / 360.0, 39.75481 / 200.0, 81.91256 / 100.0, 0.0],
    ];
    for (name, direction, tolerance, pixels) in [
        ("ACES RGB to HMJ", TransformDirection::Forward, 1e-5, rgb),
        (
            "ACES HMJ to RGB",
            TransformDirection::Inverse,
            2e-5,
            hmj_input,
        ),
    ] {
        let processor = config.processor_from_transform(&hmj, direction)?;
        validate_case(
            &device,
            &queue,
            name,
            &processor,
            None,
            TextureKind::None,
            tolerance,
            Some(&pixels),
            false,
        )?;
    }
    Ok(())
}
