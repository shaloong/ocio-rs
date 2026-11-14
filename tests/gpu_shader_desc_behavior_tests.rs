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

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn custom_shader_hooks_can_rewrite_sections_and_preserve_cleanup() {
    use ocio_rs::transform::{GroupTransform, Lut1DTransform};
    use ocio_rs::{
        OcioError, ShaderCodeSection, ShaderExtractionHooks, ShaderLifecycle, ShaderTexture,
        ShaderUniform,
    };
    struct Hooks {
        sections: Vec<ShaderCodeSection>,
        lifecycle: Vec<&'static str>,
        textures: usize,
        uniforms: usize,
        reject: bool,
    }
    impl ShaderExtractionHooks for Hooks {
        fn lifecycle(&mut self, event: ShaderLifecycle<'_>) -> ocio_rs::Result<()> {
            self.lifecycle.push(match event {
                ShaderLifecycle::Begin { .. } => "begin",
                ShaderLifecycle::Finalized => "finalized",
                ShaderLifecycle::End => "end",
            });
            Ok(())
        }
        fn texture(&mut self, _: ShaderTexture<'_>) -> ocio_rs::Result<Option<u32>> {
            self.textures += 1;
            Ok(None)
        }
        fn uniform(&mut self, _: ShaderUniform<'_>) -> ocio_rs::Result<()> {
            self.uniforms += 1;
            Ok(())
        }
        fn code(
            &mut self,
            section: ShaderCodeSection,
            code: &str,
        ) -> ocio_rs::Result<Option<String>> {
            self.sections.push(section);
            if section == ShaderCodeSection::Body {
                if self.reject {
                    return Err(OcioError::InvalidInput("rejected shader code".into()));
                }
                return Ok(Some(format!("// Rust code hook\n{code}")));
            }
            Ok(None)
        }
    }
    let _guard = gpu_shader_desc_test_lock();
    let group = GroupTransform::create().unwrap();
    let lut = Lut1DTransform::create().unwrap();
    lut.set_length(3).unwrap();
    lut.set_values(&[0.0, 0.0, 0.0, 0.2, 0.3, 0.4, 1.0, 1.0, 1.0])
        .unwrap();
    group.append_transform(&lut).unwrap();
    let exposure = ExposureContrastTransform::create().unwrap();
    exposure.make_exposure_dynamic();
    group.append_transform(&exposure).unwrap();
    let gpu = ocio_rs::Config::raw()
        .unwrap()
        .processor_from_transform(&group, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    let mut hooks = Hooks {
        sections: vec![],
        lifecycle: vec![],
        textures: 0,
        uniforms: 0,
        reject: false,
    };
    let desc = gpu.try_extract_with_hooks(&template, &mut hooks).unwrap();
    assert!(desc.shader_text().unwrap().contains("// Rust code hook"));
    assert!(hooks.sections.contains(&ShaderCodeSection::Body));
    assert!(hooks.sections.contains(&ShaderCodeSection::Header));
    assert!(hooks.sections.contains(&ShaderCodeSection::Footer));
    if ocio_rs::version_hex() < 0x0205_0000 {
        assert!(hooks.sections.contains(&ShaderCodeSection::Declarations));
    } else {
        assert!(hooks.sections.contains(&ShaderCodeSection::Textures));
        assert!(hooks.sections.contains(&ShaderCodeSection::Parameters));
    }
    assert!(hooks.textures > 0 && hooks.uniforms > 0);
    assert_eq!(hooks.lifecycle, ["begin", "finalized", "end"]);
    let mut reject = Hooks {
        sections: vec![],
        lifecycle: vec![],
        textures: 0,
        uniforms: 0,
        reject: true,
    };
    let result = gpu.try_extract_with_hooks(&template, &mut reject);
    assert!(matches!(result, Err(OcioError::InvalidInput(ref e)) if e == "rejected shader code"));
    assert_eq!(reject.lifecycle, ["begin", "end"]);
    struct InvalidCode {
        panic: bool,
        ended: bool,
    }
    impl ShaderExtractionHooks for InvalidCode {
        fn lifecycle(&mut self, event: ShaderLifecycle<'_>) -> ocio_rs::Result<()> {
            if event == ShaderLifecycle::End {
                self.ended = true;
            }
            Ok(())
        }
        fn code(&mut self, section: ShaderCodeSection, _: &str) -> ocio_rs::Result<Option<String>> {
            if section != ShaderCodeSection::Body {
                return Ok(None);
            }
            if self.panic {
                panic!("intentional shader code panic");
            }
            Ok(Some("bad\0code".into()))
        }
    }
    for panic in [false, true] {
        let mut invalid = InvalidCode {
            panic,
            ended: false,
        };
        let result = gpu.try_extract_with_hooks(&template, &mut invalid);
        if panic {
            assert!(
                matches!(result, Err(OcioError::InvalidInput(ref e)) if e.contains("panicked"))
            );
        } else {
            assert!(matches!(result, Err(OcioError::InteriorNul)));
        }
        assert!(invalid.ended);
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn custom_shader_lifecycle_preserves_order_and_first_error() {
    use ocio_rs::ShaderLifecycle;
    use std::cell::RefCell;
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let transform = ExposureContrastTransform::create().unwrap();
    transform.make_exposure_dynamic();
    let gpu = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    template.set_unique_id("rusttest_").unwrap();
    let events = RefCell::new(Vec::new());
    let desc = gpu
        .try_extract_with_lifecycle_callbacks(
            &template,
            |p| {
                events.borrow_mut().push("assembly");
                Ok(format!(
                    "{}{}{}{}{}{}",
                    p.parameters, p.textures, p.helpers, p.header, p.body, p.footer
                ))
            },
            |_| {
                events.borrow_mut().push("texture");
                Ok(None)
            },
            |_| {
                events.borrow_mut().push("uniform");
                Ok(())
            },
            |event| {
                match event {
                    ShaderLifecycle::Begin { unique_id } => {
                        assert!(unique_id.starts_with("rusttest_"));
                        events.borrow_mut().push("begin");
                    }
                    ShaderLifecycle::Finalized => events.borrow_mut().push("finalized"),
                    ShaderLifecycle::End => events.borrow_mut().push("end"),
                };
                Ok(())
            },
        )
        .unwrap();
    assert!(desc.shader_text().is_some());
    let events = events.into_inner();
    assert_eq!(events.first(), Some(&"begin"));
    assert!(events.contains(&"uniform"));
    let assembly = events
        .iter()
        .position(|event| *event == "assembly")
        .unwrap();
    assert!(events.iter().position(|event| *event == "uniform").unwrap() < assembly);
    assert_eq!(&events[events.len() - 2..], &["finalized", "end"]);

    let cleanup = RefCell::new(Vec::new());
    let error = gpu.try_extract_with_lifecycle_callbacks(
        &template,
        |_| Ok(String::new()),
        |_| Ok(None),
        |_| Err(ocio_rs::OcioError::InvalidInput("first error".into())),
        |event| {
            if matches!(event, ShaderLifecycle::End) {
                cleanup.borrow_mut().push("end");
                return Err(ocio_rs::OcioError::InvalidInput("cleanup error".into()));
            }
            Ok(())
        },
    );
    assert!(
        matches!(error, Err(ocio_rs::OcioError::InvalidInput(ref message)) if message == "first error")
    );
    assert_eq!(cleanup.into_inner(), ["end"]);

    let after_finalize = RefCell::new(Vec::new());
    let error = gpu.try_extract_with_lifecycle_callbacks(
        &template,
        |_| Ok(String::new()),
        |_| Ok(None),
        |_| Ok(()),
        |event| {
            match event {
                ShaderLifecycle::Begin { .. } => after_finalize.borrow_mut().push("begin"),
                ShaderLifecycle::Finalized => {
                    after_finalize.borrow_mut().push("finalized");
                    return Err(ocio_rs::OcioError::InvalidInput("finalize rejected".into()));
                }
                ShaderLifecycle::End => after_finalize.borrow_mut().push("end"),
            }
            Ok(())
        },
    );
    assert!(
        matches!(error, Err(ocio_rs::OcioError::InvalidInput(ref message)) if message == "finalize rejected")
    );
    assert_eq!(after_finalize.into_inner(), ["begin", "finalized", "end"]);

    let events = RefCell::new(Vec::new());
    let error = gpu.try_extract_with_lifecycle_callbacks(
        &template,
        |_| Ok(String::new()),
        |_| Ok(None),
        |_| Ok(()),
        |event| {
            events.borrow_mut().push(event == ShaderLifecycle::End);
            if matches!(event, ShaderLifecycle::Begin { .. }) {
                panic!("begin callback panic");
            }
            Ok(())
        },
    );
    assert!(
        matches!(error, Err(ocio_rs::OcioError::InvalidInput(ref message)) if message.contains("panicked"))
    );
    assert_eq!(events.into_inner(), [false]);
    assert!(gpu
        .try_extract_with_assembler(&template, |p| Ok(format!(
            "{}{}{}{}{}{}",
            p.parameters, p.textures, p.helpers, p.header, p.body, p.footer
        )))
        .is_ok());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn custom_uniform_callbacks_report_registration_and_propagate_errors() {
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let transform = ExposureContrastTransform::create().unwrap();
    transform.make_exposure_dynamic();
    let gpu = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    let assemble = |p: ocio_rs::ShaderSections<'_>| {
        Ok(format!(
            "{}{}{}{}{}{}",
            p.parameters, p.textures, p.helpers, p.header, p.body, p.footer
        ))
    };
    let mut uniforms = Vec::new();
    let desc = gpu
        .try_extract_with_resource_callbacks(
            &template,
            assemble,
            |_| Ok(None),
            |u| {
                uniforms.push((
                    u.name.to_owned(),
                    u.uniform_type,
                    u.index,
                    u.max_size,
                    u.buffer_offset,
                ));
                Ok(())
            },
        )
        .unwrap();
    assert!(!uniforms.is_empty());
    assert_eq!(uniforms.len(), desc.num_uniforms() as usize);
    for (index, (name, kind, event_index, max_size, offset)) in uniforms.iter().enumerate() {
        let native = desc.uniform(index as u32).unwrap();
        assert_eq!(*event_index, index as u32);
        assert_eq!(name, &native.name);
        assert_eq!(*kind, native.uniform_type);
        assert_eq!(*offset, native.buffer_offset);
        if *kind == GpuUniformType::Double {
            assert_eq!(*max_size, None);
        }
    }
    let index = uniforms
        .iter()
        .find(|u| u.1 == GpuUniformType::Double)
        .unwrap()
        .2;
    let before = desc.try_uniform_f64_value(index).unwrap().unwrap();
    let property = desc
        .try_dynamic_property(DynamicPropertyType::Exposure)
        .unwrap()
        .unwrap();
    property.set_double_value(before + 1.0).unwrap();
    assert_ne!(desc.try_uniform_f64_value(index).unwrap().unwrap(), before);
    let failed = gpu.try_extract_with_resource_callbacks(
        &template,
        assemble,
        |_| Ok(None),
        |_| Err(ocio_rs::OcioError::InvalidInput("uniform rejected".into())),
    );
    assert!(
        matches!(failed, Err(ocio_rs::OcioError::InvalidInput(ref message)) if message == "uniform rejected")
    );
    let panicked = gpu.try_extract_with_resource_callbacks(
        &template,
        assemble,
        |_| Ok(None),
        |_| panic!("uniform callback panic"),
    );
    assert!(
        matches!(panicked, Err(ocio_rs::OcioError::InvalidInput(ref message)) if message.contains("panicked"))
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn custom_uniform_callback_reports_vector_capacity_when_available() {
    use ocio_rs::transform::GradingRGBCurveTransform;
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let transform = GradingRGBCurveTransform::create(ocio_rs::GradingStyle::Log).unwrap();
    transform.try_make_dynamic().unwrap();
    let gpu = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    let mut vectors = Vec::new();
    let desc = gpu
        .try_extract_with_resource_callbacks(
            &template,
            |p| {
                Ok(format!(
                    "{}{}{}{}{}{}",
                    p.parameters, p.textures, p.helpers, p.header, p.body, p.footer
                ))
            },
            |_| Ok(None),
            |u| {
                if matches!(
                    u.uniform_type,
                    GpuUniformType::VectorFloat | GpuUniformType::VectorInt
                ) {
                    vectors.push((u.name.to_owned(), u.index, u.max_size, u.buffer_offset));
                }
                Ok(())
            },
        )
        .unwrap();
    assert!(!vectors.is_empty());
    for (name, index, max_size, offset) in vectors {
        let native = desc.uniform(index).unwrap();
        assert_eq!(native.name, name);
        assert_eq!(native.buffer_offset, offset);
        if ocio_rs::version_hex() >= 0x0205_0000 {
            assert!(max_size.is_some_and(|size| size as usize >= native.value_count));
        } else {
            assert_eq!(max_size, None);
        }
    }
}

#[test]
#[cfg(ocio_stub)]
fn shader_text_borrow_is_absent_in_stub_mode() {
    let mut desc = GpuShaderDesc::create().expect("stub descriptor");
    assert!(desc.shader_text_ref().unwrap().is_none());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn shader_text_borrow_matches_native_storage_and_owned_text() {
    let _guard = gpu_shader_desc_test_lock();
    let mut desc = GpuShaderDesc::create().expect("descriptor");
    assert_eq!(desc.shader_text_ref().unwrap().unwrap().to_bytes(), b"");
    desc.create_shader_text("", "", "", "void f() {\n", "  // borrowed\n", "}\n")
        .expect("shader text");
    let owned = desc.try_shader_text().unwrap().unwrap();
    let borrowed = desc.shader_text_ref().unwrap().unwrap();
    assert_eq!(borrowed.to_str().unwrap(), owned);
    assert!(borrowed.to_str().unwrap().contains("borrowed"));
    let first = borrowed.as_ptr();
    assert_eq!(desc.shader_text_ref().unwrap().unwrap().as_ptr(), first);
    desc.create_shader_text("", "", "", "void g() {\n", "  // changed\n", "}\n")
        .expect("replace shader text");
    assert!(desc
        .shader_text_ref()
        .unwrap()
        .unwrap()
        .to_str()
        .unwrap()
        .contains("changed"));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_shader_assembler_receives_sections_and_retains_dynamic_uniforms() {
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let transform = ExposureContrastTransform::create().unwrap();
    transform.make_exposure_dynamic();
    let gpu = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    template.set_language(GpuLanguage::Glsl4_0).unwrap();
    template.set_function_name("customAssembly").unwrap();
    let mut calls = 0;
    let desc = gpu
        .try_extract_with_assembler(&template, |parts| {
            calls += 1;
            Ok(format!(
                "// rust assembler\n{}{}{}{}{}{}",
                parts.parameters,
                parts.textures,
                parts.helpers,
                parts.header,
                parts.body,
                parts.footer
            ))
        })
        .unwrap();
    assert_eq!(calls, 1);
    let shader = desc.shader_text().unwrap();
    assert!(shader.contains("// rust assembler"));
    assert!(shader.contains("customAssembly"));
    let property = desc
        .try_dynamic_property(DynamicPropertyType::Exposure)
        .unwrap()
        .unwrap();
    let index = (0..desc.num_uniforms())
        .find(|&i| {
            desc.uniform(i)
                .is_some_and(|u| u.uniform_type == GpuUniformType::Double)
        })
        .unwrap();
    let mut before = [0.0];
    assert!(desc.try_copy_uniform_f32_into(index, &mut before).unwrap());
    property.set_double_value(1.0).unwrap();
    let mut after = [0.0];
    assert!(desc.try_copy_uniform_f32_into(index, &mut after).unwrap());
    assert_ne!(before, after);
    assert!(gpu
        .try_extract_with_assembler(&template, |_| Err(ocio_rs::OcioError::InvalidInput(
            "rejected".into()
        )))
        .is_err());
    assert!(gpu
        .try_extract_with_assembler(&template, |_| -> ocio_rs::Result<String> {
            panic!("callback panic")
        })
        .is_err());
    assert!(gpu
        .try_extract_with_assembler(&template, |_| Ok("bad\0shader".into()))
        .is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_shader_assembler_preserves_lut_across_native_versions() {
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let lut = ocio_rs::transform::Lut1DTransform::create().unwrap();
    lut.set_length(3).unwrap();
    lut.set_values(&[0.0, 0.0, 0.0, 0.2, 0.3, 0.4, 1.0, 1.0, 1.0])
        .unwrap();
    let gpu = config
        .processor_from_transform(&lut, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    template.set_language(GpuLanguage::Glsl4_0).unwrap();
    let mut saw_declaration = false;
    let mut desc = gpu
        .try_extract_with_assembler(&template, |p| {
            saw_declaration = if ocio_rs::version_hex() < 0x02050000 {
                p.textures.is_empty() && p.parameters.contains("sampler")
            } else {
                p.textures.contains("sampler")
            };
            Ok(format!(
                "{}{}{}{}{}{}",
                p.parameters, p.textures, p.helpers, p.header, p.body, p.footer
            ))
        })
        .unwrap();
    assert!(saw_declaration);
    assert!(desc.num_textures() > 0);
    assert!(!desc.texture_values_ref(0).unwrap().is_empty());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_shader_texture_callback_borrows_native_lut_and_propagates_errors() {
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let lut = ocio_rs::transform::Lut1DTransform::create().unwrap();
    lut.set_length(3).unwrap();
    lut.set_values(&[0.0, 0.0, 0.0, 0.2, 0.3, 0.4, 1.0, 1.0, 1.0])
        .unwrap();
    let gpu = config
        .processor_from_transform(&lut, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    template.set_language(GpuLanguage::Glsl4_0).unwrap();
    let mut observed = Vec::new();
    let desc = gpu
        .try_extract_with_texture_callback(
            &template,
            |p| {
                Ok(format!(
                    "{}{}{}{}{}{}",
                    p.parameters, p.textures, p.helpers, p.header, p.body, p.footer
                ))
            },
            |texture| {
                observed.push((texture.texture_name.to_owned(), texture.values.to_vec()));
                Ok(None)
            },
        )
        .unwrap();
    assert_eq!(observed.len(), desc.num_textures() as usize);
    assert!(!observed.is_empty());
    assert_eq!(observed[0].0, desc.texture_2d(0).unwrap().texture_name);
    assert_eq!(observed[0].1, desc.texture_values(0));
    assert!(gpu
        .try_extract_with_texture_callback(
            &template,
            |_| Ok(String::new()),
            |_| Err(ocio_rs::OcioError::InvalidInput("texture rejected".into()))
        )
        .is_err());
    assert!(gpu
        .try_extract_with_texture_callback(
            &template,
            |_| Ok(String::new()),
            |_| -> ocio_rs::Result<Option<u32>> { panic!("texture callback panic") }
        )
        .is_err());
    if ocio_rs::version_hex() < 0x02050000 {
        assert!(gpu
            .try_extract_with_texture_callback(
                &template,
                |_| Ok(String::new()),
                |texture| {
                    assert!(texture.suggested_binding.is_none());
                    Ok(Some(42))
                }
            )
            .is_err());
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_shader_texture_callback_receives_3d_lut() {
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let lut = ocio_rs::transform::Lut3DTransform::create().unwrap();
    lut.set_grid_size(2).unwrap();
    for r in 0..2 {
        for g in 0..2 {
            for b in 0..2 {
                lut.set_value(r, g, b, [r as f32 * 0.8, g as f32 * 0.7, b as f32 * 0.6])
                    .unwrap();
            }
        }
    }
    let gpu = config
        .processor_from_transform(&lut, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    let mut observed = Vec::new();
    let desc = gpu
        .try_extract_with_texture_callback(
            &template,
            |p| {
                Ok(format!(
                    "{}{}{}{}{}{}",
                    p.parameters, p.textures, p.helpers, p.header, p.body, p.footer
                ))
            },
            |texture| {
                if texture.dimensions.is_none() {
                    observed.push((texture.depth, texture.values.to_vec()));
                }
                Ok(None)
            },
        )
        .unwrap();
    assert_eq!(observed.len(), desc.num_3d_textures() as usize);
    assert!(!observed.is_empty());
    assert_eq!(observed[0].0, desc.texture_3d(0).unwrap().edge_len);
    assert_eq!(observed[0].1, desc.texture_3d_values(0));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn streamed_shader_textures_match_retained_resources_without_descriptor_copies() {
    use ocio_rs::ShaderExtractionHooks;
    struct Hooks;
    impl ShaderExtractionHooks for Hooks {}
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let lut1d = ocio_rs::transform::Lut1DTransform::create().unwrap();
    lut1d.set_length(3).unwrap();
    lut1d
        .set_values(&[0.0, 0.0, 0.0, 0.2, 0.3, 0.4, 1.0, 1.0, 1.0])
        .unwrap();
    let lut3d = ocio_rs::transform::Lut3DTransform::create().unwrap();
    lut3d.set_grid_size(2).unwrap();
    for r in 0..2 {
        for g in 0..2 {
            for b in 0..2 {
                lut3d
                    .set_value(r, g, b, [r as f32 * 0.8, g as f32 * 0.7, b as f32 * 0.6])
                    .unwrap();
            }
        }
    }
    let group = ocio_rs::transform::GroupTransform::create().unwrap();
    group.append_transform(&lut1d).unwrap();
    group.append_transform(&lut3d).unwrap();
    let gpu = config
        .processor_from_transform(&group, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    template.set_language(GpuLanguage::Glsl4_0).unwrap();
    let retained = gpu.try_extract_with_hooks(&template, &mut Hooks).unwrap();
    let mut seen = Vec::new();
    let streamed = gpu
        .try_extract_with_streamed_textures(&template, &mut Hooks, |tex| {
            seen.push((
                tex.texture_name.to_owned(),
                tex.dimensions,
                tex.values.to_vec(),
            ));
            Ok(None)
        })
        .unwrap();
    assert_eq!(streamed.num_textures(), 0);
    assert_eq!(streamed.num_3d_textures(), 0);
    assert_eq!(streamed.shader_text(), retained.shader_text());
    assert_eq!(
        seen.len(),
        (retained.num_textures() + retained.num_3d_textures()) as usize
    );
    for (name, dimensions, values) in seen {
        if dimensions.is_some() {
            let index = (0..retained.num_textures())
                .find(|&i| retained.texture_2d(i).unwrap().texture_name == name)
                .unwrap();
            assert_eq!(values, retained.texture_values(index));
        } else {
            let index = (0..retained.num_3d_textures())
                .find(|&i| retained.texture_3d(i).unwrap().texture_name == name)
                .unwrap();
            assert_eq!(values, retained.texture_3d_values(index));
        }
    }
    assert!(gpu
        .try_extract_with_streamed_textures(&template, &mut Hooks, |_| Err(
            ocio_rs::OcioError::InvalidInput("rejected".into())
        ))
        .is_err());
    assert!(gpu
        .try_extract_with_streamed_textures(
            &template,
            &mut Hooks,
            |_| -> ocio_rs::Result<Option<u32>> { panic!("sink panic") }
        )
        .is_err());
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn streamed_texture_bindings_are_global_and_control_shader_code() {
    use ocio_rs::ShaderExtractionHooks;
    struct Hooks;
    impl ShaderExtractionHooks for Hooks {}
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let lut1d = ocio_rs::transform::Lut1DTransform::create().unwrap();
    lut1d.set_length(3).unwrap();
    lut1d
        .set_values(&[0.0, 0.0, 0.0, 0.2, 0.3, 0.4, 1.0, 1.0, 1.0])
        .unwrap();
    let lut3d = ocio_rs::transform::Lut3DTransform::create().unwrap();
    lut3d.set_grid_size(2).unwrap();
    for r in 0..2 {
        for g in 0..2 {
            for b in 0..2 {
                lut3d
                    .set_value(r, g, b, [r as f32 * 0.8, g as f32 * 0.7, b as f32 * 0.6])
                    .unwrap();
            }
        }
    }
    let group = ocio_rs::transform::GroupTransform::create().unwrap();
    group.append_transform(&lut1d).unwrap();
    group.append_transform(&lut3d).unwrap();
    let gpu = config
        .processor_from_transform(&group, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    template.set_language(GpuLanguage::GlslVk4_6).unwrap();
    template.try_set_descriptor_set_index(2, 7).unwrap();
    let mut proposed = Vec::new();
    let desc = gpu
        .try_extract_with_streamed_textures(&template, &mut Hooks, |tex| {
            proposed.push(tex.suggested_binding.unwrap());
            Ok(Some(41 + proposed.len() as u32))
        })
        .unwrap();
    assert_eq!(proposed, [7, 8]);
    assert!(desc.shader_text().unwrap().contains("binding = 42"));
    assert!(desc.shader_text().unwrap().contains("binding = 43"));
    assert!(gpu
        .try_extract_with_streamed_textures(&template, &mut Hooks, |_| Ok(Some(42)))
        .is_err());
    assert!(gpu
        .try_extract_with_streamed_textures(&template, &mut Hooks, |_| Ok(Some(0)))
        .is_err());
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_shader_texture_callback_controls_vulkan_binding() {
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let lut = ocio_rs::transform::Lut1DTransform::create().unwrap();
    lut.set_length(3).unwrap();
    lut.set_values(&[0.0, 0.0, 0.0, 0.2, 0.3, 0.4, 1.0, 1.0, 1.0])
        .unwrap();
    let gpu = config
        .processor_from_transform(&lut, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    template.set_language(GpuLanguage::GlslVk4_6).unwrap();
    template.try_set_descriptor_set_index(2, 7).unwrap();
    let desc = gpu
        .try_extract_with_texture_callback(
            &template,
            |p| {
                Ok(format!(
                    "{}{}{}{}{}{}",
                    p.parameters, p.textures, p.helpers, p.header, p.body, p.footer
                ))
            },
            |texture| {
                assert_eq!(texture.suggested_binding, Some(7));
                Ok(Some(42))
            },
        )
        .unwrap();
    assert_eq!(desc.texture_shader_binding_index(0), Some(42));
    assert!(desc.shader_text().unwrap().contains("binding = 42"));
    assert!(gpu
        .try_extract_with_texture_callback(&template, |_| Ok(String::new()), |_| Ok(Some(0)))
        .is_err());
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_shader_texture_callback_checks_bindings_across_texture_types() {
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let lut1d = ocio_rs::transform::Lut1DTransform::create().unwrap();
    lut1d.set_length(3).unwrap();
    lut1d
        .set_values(&[0.0, 0.0, 0.0, 0.2, 0.3, 0.4, 1.0, 1.0, 1.0])
        .unwrap();
    let lut3d = ocio_rs::transform::Lut3DTransform::create().unwrap();
    lut3d.set_grid_size(2).unwrap();
    for r in 0..2 {
        for g in 0..2 {
            for b in 0..2 {
                lut3d
                    .set_value(r, g, b, [r as f32 * 0.8, g as f32 * 0.7, b as f32 * 0.6])
                    .unwrap();
            }
        }
    }
    let group = ocio_rs::transform::GroupTransform::create().unwrap();
    group.append_transform(&lut1d).unwrap();
    group.append_transform(&lut3d).unwrap();
    let gpu = config
        .processor_from_transform(&group, TransformDirection::Forward)
        .unwrap()
        .optimized_gpu_processor(0)
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    template.set_language(GpuLanguage::GlslVk4_6).unwrap();
    let mut next = 41;
    let desc = gpu
        .try_extract_with_texture_callback(
            &template,
            |p| {
                Ok(format!(
                    "{}{}{}{}{}{}",
                    p.parameters, p.textures, p.helpers, p.header, p.body, p.footer
                ))
            },
            |_| {
                next += 1;
                Ok(Some(next))
            },
        )
        .unwrap();
    assert_eq!(desc.num_textures(), 1);
    assert_eq!(desc.num_3d_textures(), 1);
    let a = desc.texture_shader_binding_index(0).unwrap();
    let b = desc.texture_3d_shader_binding_index(0).unwrap();
    assert_ne!(a, b);
    assert!([42, 43].contains(&a) && [42, 43].contains(&b));
    desc.try_set_descriptor_set_index(3, 40).unwrap();
    assert!(desc
        .add_texture_2d(
            "duplicate_texture",
            "duplicate_sampler",
            1,
            1,
            GpuTextureChannel::Rgb,
            GpuTextureDimensions::Texture2D,
            Interpolation::Linear,
            &[0.1, 0.2, 0.3],
        )
        .is_err());
    assert_eq!(desc.num_textures(), 1);
    desc.try_set_descriptor_set_index(3, 70).unwrap();
    let added = desc
        .add_texture_2d(
            "later_texture",
            "later_sampler",
            1,
            1,
            GpuTextureChannel::Rgb,
            GpuTextureDimensions::Texture2D,
            Interpolation::Linear,
            &[0.1, 0.2, 0.3],
        )
        .unwrap()
        .unwrap();
    assert!(added >= 70);
    assert_eq!(desc.texture_shader_binding_index(0), Some(a));
    assert_eq!(desc.texture_3d_shader_binding_index(0), Some(b));
    assert_eq!(desc.texture_shader_binding_index(1), Some(added));
    assert!(gpu
        .try_extract_with_texture_callback(&template, |_| Ok(String::new()), |_| Ok(Some(42)))
        .is_err());
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_shader_assembler_preserves_lut_resources_and_binding_indices() {
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let lut = ocio_rs::transform::Lut1DTransform::create().unwrap();
    lut.set_length(3).unwrap();
    lut.set_values(&[0.0, 0.0, 0.0, 0.2, 0.3, 0.4, 1.0, 1.0, 1.0])
        .unwrap();
    let gpu = config
        .processor_from_transform(&lut, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    let template = GpuShaderDesc::create().unwrap();
    template.set_language(GpuLanguage::GlslVk4_6).unwrap();
    template.try_set_descriptor_set_index(2, 7).unwrap();
    template.set_allow_texture_1d(false).unwrap();
    let mut desc = gpu
        .try_extract_with_assembler(&template, |p| {
            Ok(format!(
                "{}{}{}{}{}{}",
                p.parameters, p.textures, p.helpers, p.header, p.body, p.footer
            ))
        })
        .unwrap();
    assert!(desc.num_textures() > 0);
    let texture = desc.texture_2d(0).unwrap();
    assert_eq!(texture.dimensions, GpuTextureDimensions::Texture2D);
    assert_eq!(texture.binding_index, Some(7));
    assert_eq!(desc.texture_values_ref(0).unwrap(), texture.values);
    assert_eq!(desc.descriptor_set_index(), 2);
    assert_eq!(desc.texture_binding_start(), 7);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn native_shader_creator_interop_borrows_without_taking_ownership() {
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let transform = MatrixTransform::scale(&[1.1, 0.9, 1.2, 1.0]).unwrap();
    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap();
    let gpu = processor.default_gpu_processor().unwrap();
    let mut desc = GpuShaderDesc::create().unwrap();
    desc.set_function_name("customCreatorTest").unwrap();
    let creator = desc.borrow_native_creator().unwrap();
    // SAFETY: The descriptor owns this native creator and remains alive and
    // exclusively unused throughout the synchronous extraction.
    unsafe {
        gpu.try_extract_shader_info_raw(creator.as_ptr()).unwrap();
    }
    drop(gpu);
    assert!(desc.shader_text().unwrap().contains("customCreatorTest"));
    // A null native object is rejected without entering C++.
    let gpu = processor.default_gpu_processor().unwrap();
    assert!(unsafe { gpu.try_extract_shader_info_raw(std::ptr::null_mut()) }.is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn native_texture_bridge_rejects_overflow_before_reading_values() {
    let _guard = gpu_shader_desc_test_lock();
    unsafe {
        let desc = ocio_sys::ocio_gpu_shader_desc_create_shader_desc();
        assert!(!desc.is_null());
        let name = CString::new("texture").unwrap();
        let sample = [0.0f32];
        ocio_sys::ocio_error_clear_last();
        assert!(!ocio_sys::ocio_gpu_shader_desc_add3d_texture(
            desc,
            name.as_ptr(),
            name.as_ptr(),
            1 << 22,
            Interpolation::Linear as i32,
            sample.as_ptr(),
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut()
        ));
        let error = ocio_sys::ocio_error_get_last();
        assert!(!error.is_null());
        assert!(CStr::from_ptr(error)
            .to_string_lossy()
            .contains("payload size overflow"));
        ocio_sys::ocio_error_clear_last();
        assert!(!ocio_sys::ocio_gpu_shader_desc_add_texture(
            desc,
            name.as_ptr(),
            name.as_ptr(),
            u32::MAX,
            u32::MAX,
            1,
            1,
            Interpolation::Linear as i32,
            sample.as_ptr(),
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut()
        ));
        let error = ocio_sys::ocio_error_get_last();
        assert!(!error.is_null());
        assert!(CStr::from_ptr(error)
            .to_string_lossy()
            .contains("payload size overflow"));
        ocio_sys::ocio_gpu_shader_desc_destroy(desc);
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn gpu_texture_dimensions_match_native_resources() {
    let _guard = gpu_shader_desc_test_lock();
    let desc = GpuShaderDesc::create().unwrap();
    desc.add_texture_2d(
        "one",
        "oneSampler",
        2,
        1,
        GpuTextureChannel::Red,
        GpuTextureDimensions::Texture1D,
        Interpolation::Linear,
        &[0.0, 1.0],
    )
    .unwrap();
    desc.add_texture_2d(
        "two",
        "twoSampler",
        2,
        2,
        GpuTextureChannel::Red,
        GpuTextureDimensions::Texture2D,
        Interpolation::Linear,
        &[0.0, 0.3, 0.6, 1.0],
    )
    .unwrap();
    assert_eq!(
        desc.try_texture_2d(0).unwrap().unwrap().dimensions,
        GpuTextureDimensions::Texture1D
    );
    assert_eq!(
        desc.try_texture_2d(1).unwrap().unwrap().dimensions,
        GpuTextureDimensions::Texture2D
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn gpu_texture_values_borrow_native_storage() {
    let _guard = gpu_shader_desc_test_lock();
    let mut desc = GpuShaderDesc::create().unwrap();
    let samples = [0.0, 0.25, 0.5, 1.0];
    desc.add_texture_2d(
        "one",
        "oneSampler",
        4,
        1,
        GpuTextureChannel::Red,
        GpuTextureDimensions::Texture1D,
        Interpolation::Linear,
        &samples,
    )
    .unwrap();
    let cube = [0.5; 24];
    desc.add_texture_3d("cube", "cubeSampler", 2, Interpolation::Linear, &cube)
        .unwrap();
    assert_eq!(desc.texture_values_ref(0).unwrap(), &samples);
    assert_eq!(desc.texture_3d_values_ref(0).unwrap(), &cube);
    assert!(desc.texture_values_ref(99).is_err());
    assert!(desc.texture_3d_values_ref(99).is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn extracted_lut_texture_reports_native_dimension() {
    let _guard = gpu_shader_desc_test_lock();
    let config = ocio_rs::Config::raw().unwrap();
    let transform = ocio_rs::transform::Lut1DTransform::create().unwrap();
    transform.set_length(3).unwrap();
    transform
        .set_values(&[0.0, 0.0, 0.0, 0.2, 0.3, 0.4, 1.0, 1.0, 1.0])
        .unwrap();
    let gpu = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .unwrap()
        .default_gpu_processor()
        .unwrap();
    for (allow, dimension) in [
        (true, GpuTextureDimensions::Texture1D),
        (false, GpuTextureDimensions::Texture2D),
    ] {
        let mut desc = GpuShaderDesc::create().unwrap();
        desc.set_language(GpuLanguage::Glsl4_0).unwrap();
        desc.set_allow_texture_1d(allow).unwrap();
        gpu.try_extract_shader_info(&mut desc).unwrap();
        let textures = desc.textures_2d();
        assert!(!textures.is_empty());
        assert!(textures
            .iter()
            .all(|texture| texture.dimensions == dimension));
    }
}

fn extracted_gpu_shader_desc() -> Option<GpuShaderDesc> {
    let config = create_test_config()?;
    let transform = MatrixTransform::scale(&[1.1, 0.9, 1.2, 1.0]).ok()?;
    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .ok()?;
    let gpu = processor.default_gpu_processor().ok()?;
    let desc = GpuShaderDesc::create().ok()?;
    desc.set_language(GpuLanguage::Glsl4_0).ok()?;
    desc.set_function_name("ocio_test_main").ok()?;
    desc.set_pixel_name("ocio_test_pixel").ok()?;
    desc.set_resource_prefix("ocio_test_").ok()?;
    let mut desc = desc;
    gpu.try_extract_shader_info(&mut desc).ok()?;
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
    gpu.try_extract_shader_info(&mut desc).ok()?;
    Some((processor, desc))
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn gpu_shader_desc_config_round_trip_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let desc = GpuShaderDesc::create().expect("gpu shader desc create");
    desc.set_language(GpuLanguage::Glsl4_0)
        .expect("set_language");
    desc.set_function_name("ocio_test_main")
        .expect("set_function_name");
    desc.set_pixel_name("ocio_test_pixel")
        .expect("set_pixel_name");
    desc.set_unique_id("ocio-test-uid").expect("set_unique_id");
    desc.set_resource_prefix("ocio_test_")
        .expect("set_resource_prefix");
    desc.try_set_descriptor_set_index(3, 7)
        .expect("set descriptor set index");
    desc.set_texture_max_width(64)
        .expect("set_texture_max_width");
    desc.set_allow_texture_1d(false)
        .expect("set_allow_texture_1d");

    assert_eq!(desc.language(), GpuLanguage::Glsl4_0);
    assert_eq!(
        desc.try_language().expect("language query"),
        desc.language()
    );
    assert_eq!(desc.function_name().as_deref(), Some("ocio_test_main"));
    assert_eq!(desc.pixel_name().as_deref(), Some("ocio_test_pixel"));
    assert_eq!(desc.unique_id().as_deref(), Some("ocio-test-uid"));
    assert_eq!(desc.resource_prefix().as_deref(), Some("ocio_test_"));
    assert_eq!(
        desc.try_function_name().expect("function name query"),
        desc.function_name()
    );
    assert_eq!(
        desc.try_pixel_name().expect("pixel name query"),
        desc.pixel_name()
    );
    assert_eq!(
        desc.try_unique_id().expect("unique id query"),
        desc.unique_id()
    );
    assert_eq!(
        desc.try_resource_prefix().expect("resource prefix query"),
        desc.resource_prefix()
    );
    assert_eq!(desc.descriptor_set_index(), 3);
    assert_eq!(desc.texture_binding_start(), 7);
    assert_eq!(
        desc.try_descriptor_set_index()
            .expect("descriptor set index query"),
        desc.descriptor_set_index()
    );
    assert_eq!(
        desc.try_texture_binding_start()
            .expect("texture binding start query"),
        desc.texture_binding_start()
    );
    assert_eq!(desc.texture_max_width(), 64);
    assert_eq!(
        desc.try_texture_max_width()
            .expect("texture max width query"),
        desc.texture_max_width()
    );
    assert!(!desc.allow_texture_1d());
    assert_eq!(
        desc.try_allow_texture_1d().expect("allow texture 1d query"),
        desc.allow_texture_1d()
    );

    let cloned = desc.clone_desc().expect("clone_desc");
    assert_eq!(cloned.language(), GpuLanguage::Glsl4_0);
    assert_eq!(cloned.function_name().as_deref(), Some("ocio_test_main"));
    assert_eq!(cloned.pixel_name().as_deref(), Some("ocio_test_pixel"));
    assert_eq!(cloned.unique_id().as_deref(), Some("ocio-test-uid"));
    assert_eq!(cloned.resource_prefix().as_deref(), Some("ocio_test_"));
    assert_eq!(cloned.descriptor_set_index(), 3);
    assert_eq!(cloned.texture_binding_start(), 7);
    assert_eq!(cloned.texture_max_width(), 4096);
    assert!(cloned.allow_texture_1d());
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn gpu_shader_desc_invalid_descriptor_binding_reports_error_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let desc = GpuShaderDesc::create().expect("gpu shader desc create");
    let err = desc
        .try_set_descriptor_set_index(3, 0)
        .expect_err("zero texture binding start should fail");
    assert!(
        matches!(err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn gpu_shader_desc_result_wrappers_clear_prior_error_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let desc = GpuShaderDesc::create().expect("gpu shader desc create");
    let err = desc
        .try_set_descriptor_set_index(3, 0)
        .expect_err("zero texture binding start should fail");
    assert!(
        matches!(err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {err:?}"
    );

    desc.set_function_name("after_error_main")
        .expect("set_function_name after error");
    desc.set_pixel_name("after_error_pixel")
        .expect("set_pixel_name after error");
    desc.set_unique_id("after-error-uid")
        .expect("set_unique_id after error");
    desc.set_resource_prefix("after_error_")
        .expect("set_resource_prefix after error");

    desc.begin("after_error_uid").expect("begin after error");
    desc.end().expect("end shader collection");

    desc.add_to_parameter_declare_shader_code("uniform float uAfter;\n")
        .expect("parameter declarations after error");
    desc.add_to_texture_declare_shader_code("uniform sampler2D texAfter;\n")
        .expect("texture declarations after error");
    desc.add_to_helper_shader_code("float pass_after(float x) { return x; }\n")
        .expect("helper methods after error");
    desc.add_to_function_header_shader_code("vec4 AfterMain(vec4 inPixel) {\n")
        .expect("function header after error");
    desc.add_to_function_shader_code("  return vec4(pass_after(inPixel.r), inPixel.gba);\n")
        .expect("function body after error");
    desc.add_to_function_footer_shader_code("}\n")
        .expect("function footer after error");
    desc.create_shader_text(
        "uniform float uExplicit;\n",
        "uniform sampler2D texExplicit;\n",
        "float after_helper(float x) { return x; }\n",
        "vec4 ExplicitAfter(vec4 inPixel) {\n",
        "  return vec4(after_helper(inPixel.r), inPixel.g, inPixel.b, inPixel.a);\n",
        "}\n",
    )
    .expect("create_shader_text after error");

    assert!(desc
        .add_uniform_f64("uExposure", 1.25)
        .expect("add uniform after error"));
    assert!(!desc
        .add_uniform_f64("uExposure", 2.0)
        .expect("duplicate uniform should remain Ok(false)"));

    desc.try_set_descriptor_set_index(2, 5)
        .expect("set descriptor set index");

    let values_2d = vec![0.1f32, 0.2, 0.3, 0.4, 0.5, 0.6];
    let binding_2d = desc
        .add_texture_2d(
            "afterErrorTex2D",
            "afterErrorSampler2D",
            2,
            1,
            GpuTextureChannel::Rgb,
            GpuTextureDimensions::Texture1D,
            Interpolation::Linear,
            &values_2d,
        )
        .expect("add texture 2d after error");
    assert_eq!(binding_2d, Some(5));

    let values_3d = vec![
        0.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0,
        1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ];
    let binding_3d = desc
        .add_texture_3d(
            "afterErrorTex3D",
            "afterErrorSampler3D",
            2,
            Interpolation::Nearest,
            &values_3d,
        )
        .expect("add texture 3d after error");
    assert_eq!(binding_3d, Some(6));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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

    let shader_text = desc
        .try_shader_text()
        .expect("shader_text query")
        .expect("shader_text");
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
            texture.binding_index
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
            texture.binding_index
        );
    }

    let cloned = desc.clone_desc().expect("clone_desc");
    assert_eq!(cloned.language(), GpuLanguage::Glsl4_0);
    assert_eq!(cloned.function_name().as_deref(), Some("ocio_test_main"));
    assert_eq!(cloned.pixel_name().as_deref(), Some("ocio_test_pixel"));
    assert_eq!(cloned.resource_prefix().as_deref(), Some("ocio_test_"));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn gpu_shader_desc_extracts_multiple_target_languages_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let config = create_test_config().expect("raw config");
    let transform = MatrixTransform::scale(&[1.1, 0.9, 1.2, 1.0]).expect("matrix transform");
    let processor = config
        .processor_from_transform(&transform, TransformDirection::Forward)
        .expect("processor");
    let gpu = processor.default_gpu_processor().expect("gpu processor");

    for (language, suffix) in [
        (GpuLanguage::Glsl1_3, "glsl13"),
        (GpuLanguage::Glsl4_0, "glsl40"),
        (GpuLanguage::HlslSm5_0, "hlsl50"),
        (GpuLanguage::Msl2_0, "msl20"),
    ] {
        let mut desc = GpuShaderDesc::create().expect("gpu shader desc create");
        let function_name = format!("ocio_{suffix}_main");
        let pixel_name = format!("ocio_{suffix}_pixel");
        desc.set_language(language).expect("set language");
        desc.set_function_name(&function_name)
            .expect("set function name");
        desc.set_pixel_name(&pixel_name).expect("set pixel name");
        desc.set_resource_prefix("ocio_multi_")
            .expect("set resource prefix");
        gpu.try_extract_shader_info(&mut desc)
            .expect("extract shader info");

        let shader_text = desc.shader_text().expect("shader text");
        assert!(!shader_text.trim().is_empty(), "empty shader for {suffix}");
        assert!(
            shader_text.contains(&function_name),
            "shader for {suffix} omitted its entry point"
        );
        assert_eq!(desc.language(), language);
        assert_eq!(
            desc.function_name().as_deref(),
            Some(function_name.as_str())
        );
        assert_eq!(desc.pixel_name().as_deref(), Some(pixel_name.as_str()));
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn gpu_shader_desc_dynamic_property_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let (processor, desc) = extracted_dynamic_gpu_shader_desc().expect("dynamic gpu shader desc");
    assert!(processor.is_dynamic());
    assert!(desc.has_dynamic_property_kind(DynamicPropertyType::Exposure));
    assert_eq!(
        desc.try_has_dynamic_property_kind(DynamicPropertyType::Exposure)
            .expect("descriptor dynamic property query"),
        desc.has_dynamic_property_kind(DynamicPropertyType::Exposure)
    );
    let dynamic_property_count = desc
        .try_num_dynamic_properties()
        .expect("descriptor dynamic property count");
    assert_eq!(dynamic_property_count, desc.num_dynamic_properties());
    assert!(dynamic_property_count >= 1);

    let desc_prop = desc
        .try_dynamic_property(DynamicPropertyType::Exposure)
        .expect("descriptor dynamic exposure property query")
        .expect("desc dynamic exposure property");
    assert_eq!(desc_prop.property_type(), DynamicPropertyType::Exposure);
    assert_close(
        desc_prop.double_value().expect("desc prop double value"),
        0.0,
        1e-8,
    );
    let uniform_index = (0..desc.num_uniforms())
        .find(|&index| {
            desc.uniform(index)
                .is_some_and(|u| u.uniform_type == GpuUniformType::Double)
        })
        .expect("dynamic exposure uniform");
    let mut initial = [0.0f32];
    assert!(desc
        .try_copy_uniform_f32_into(uniform_index, &mut initial)
        .unwrap());

    let indexed_types: Vec<_> = (0..dynamic_property_count)
        .filter_map(|index| {
            desc.try_dynamic_property_by_index(index)
                .expect("descriptor indexed dynamic property query")
        })
        .map(|prop| prop.property_type())
        .collect();
    assert!(indexed_types.contains(&DynamicPropertyType::Exposure));
    assert!(desc
        .try_dynamic_property_by_index(dynamic_property_count)
        .is_err());
    assert!(desc
        .dynamic_property_by_index(dynamic_property_count)
        .is_none());

    let processor_prop = processor
        .dynamic_property(DynamicPropertyType::Exposure)
        .expect("processor dynamic exposure property");
    processor_prop
        .set_double_value(1.0)
        .expect("set processor prop double value");
    assert_close(
        desc_prop
            .double_value()
            .expect("desc prop double value after processor update"),
        0.0,
        1e-8,
    );

    desc_prop
        .set_double_value(-1.0)
        .expect("set desc prop double value");
    let mut updated = [0.0f32];
    assert!(desc
        .try_copy_uniform_f32_into(uniform_index, &mut updated)
        .unwrap());
    assert_ne!(initial, updated);
    assert_close(
        processor_prop
            .double_value()
            .expect("processor prop double value after desc update"),
        1.0,
        1e-8,
    );
    let desc_prop_after = desc
        .try_dynamic_property(DynamicPropertyType::Exposure)
        .expect("descriptor dynamic exposure property query after update")
        .expect("desc dynamic exposure property after update");
    assert_close(
        desc_prop_after
            .double_value()
            .expect("desc prop double value after desc update"),
        -1.0,
        1e-8,
    );
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn gpu_shader_desc_manual_shader_text_assembly_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let desc = GpuShaderDesc::create().expect("gpu shader desc create");
    desc.begin("manual_uid").expect("begin");
    assert_eq!(
        desc.try_next_resource_index()
            .expect("first resource index"),
        0
    );
    assert_eq!(desc.next_resource_index(), 1);
    desc.end().expect("end shader collection");

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
    desc.finalize().expect("finalize shader descriptor");

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

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn gpu_shader_desc_manual_texture_round_trip_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let desc = GpuShaderDesc::create().expect("gpu shader desc create");
    desc.try_set_descriptor_set_index(2, 5)
        .expect("set descriptor set index");

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
    assert_eq!(binding_2d, Some(5));

    let tex2d = desc.texture_2d(0).expect("texture_2d");
    assert_eq!(
        desc.try_num_textures().expect("texture count query"),
        desc.num_textures()
    );
    let queried_tex2d = desc
        .try_texture_2d(0)
        .expect("texture 2d query")
        .expect("texture 2d");
    assert_eq!(tex2d.texture_name, "manualTex2D");
    assert_eq!(tex2d.sampler_name, "manualSampler2D");
    assert_eq!(tex2d.width, 2);
    assert_eq!(tex2d.height, 1);
    assert_eq!(tex2d.channel, GpuTextureChannel::Rgb);
    assert_eq!(queried_tex2d.texture_name, tex2d.texture_name);
    assert_eq!(queried_tex2d.sampler_name, tex2d.sampler_name);
    assert_eq!(queried_tex2d.values, tex2d.values);
    assert!(matches!(
        desc.try_texture_2d(99),
        Err(ocio_rs::OcioError::Ocio(_))
    ));
    assert!(desc.texture_2d(99).is_none());
    assert_eq!(tex2d.dimensions, GpuTextureDimensions::Texture1D);
    assert_eq!(tex2d.interpolation, Interpolation::Linear);
    assert_eq!(tex2d.binding_index, Some(5));
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
    assert_eq!(binding_3d, Some(6));

    let tex3d = desc.texture_3d(0).expect("texture_3d");
    assert_eq!(
        desc.try_num_3d_textures().expect("3d texture count query"),
        desc.num_3d_textures()
    );
    let queried_tex3d = desc
        .try_texture_3d(0)
        .expect("texture 3d query")
        .expect("texture 3d");
    assert_eq!(tex3d.texture_name, "manualTex3D");
    assert_eq!(tex3d.sampler_name, "manualSampler3D");
    assert_eq!(tex3d.edge_len, 2);
    assert_eq!(tex3d.interpolation, Interpolation::Nearest);
    assert_eq!(tex3d.binding_index, Some(6));
    assert_eq!(tex3d.values, values_3d);
    assert_eq!(queried_tex3d.texture_name, tex3d.texture_name);
    assert_eq!(queried_tex3d.sampler_name, tex3d.sampler_name);
    assert_eq!(queried_tex3d.values, tex3d.values);
    assert!(desc.try_texture_3d(99).is_err());
    assert!(desc.texture_3d(99).is_none());
}

#[cfg(feature = "v2_5")]
#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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

    assert_eq!(
        desc.try_num_uniforms().expect("uniform count query"),
        desc.num_uniforms()
    );
    assert_eq!(desc.num_uniforms(), 5);
    let queried_uniform = desc
        .try_uniform(0)
        .expect("uniform query")
        .expect("uniform 0");
    let compat_uniform = desc.uniform(0).expect("compat uniform 0");
    assert_eq!(queried_uniform.name, compat_uniform.name);
    assert_eq!(queried_uniform.uniform_type, compat_uniform.uniform_type);
    assert_eq!(queried_uniform.value_count, compat_uniform.value_count);
    assert!(matches!(
        desc.try_uniform(99),
        Err(ocio_rs::OcioError::Ocio(_))
    ));
    assert!(desc.uniform(99).is_none());
    assert!(matches!(
        desc.try_texture_2d(99),
        Err(ocio_rs::OcioError::Ocio(_))
    ));
    assert!(desc.texture_2d(99).is_none());
    assert!(desc.uniform_buffer_size() > 0);
    assert_eq!(
        desc.try_uniform_buffer_size()
            .expect("uniform buffer size query"),
        desc.uniform_buffer_size()
    );
    assert_eq!(
        desc.try_texture_uid(-1).expect("missing texture uid query"),
        None
    );
    assert_eq!(
        desc.try_cache_id().expect("descriptor cache id query"),
        desc.cache_id()
    );

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
    assert_eq!(desc.try_uniform_value_count(3).unwrap(), 3);
    assert!(desc.try_uniform_value_count(99).is_err());
    assert_eq!(desc.uniform_values_f32(3), vec![0.0, 0.5, 1.0]);

    let indices = desc.uniform(4).expect("uniform 4");
    assert_eq!(indices.name, "uIndices");
    assert_eq!(indices.uniform_type, GpuUniformType::VectorInt);
    assert_eq!(indices.value_count, 3);
    assert_eq!(desc.uniform_values_i32(4), vec![1, 3, 5]);
    let mut floats = [9.0f32; 4];
    assert!(desc.try_copy_uniform_f32_into(3, &mut floats).unwrap());
    assert_eq!(floats, [0.0, 0.5, 1.0, 9.0]);
    assert!(!desc.try_copy_uniform_f32_into(3, &mut floats[..2]).unwrap());
    assert!(!desc.try_copy_uniform_f32_into(4, &mut floats).unwrap());
    let mut ints = [9i32; 4];
    assert!(desc.try_copy_uniform_i32_into(4, &mut ints).unwrap());
    assert_eq!(ints, [1, 3, 5, 9]);
    assert!(!desc.try_copy_uniform_i32_into(4, &mut ints[..2]).unwrap());
    assert!(!desc.try_copy_uniform_i32_into(0, &mut ints).unwrap());
    assert!(desc.try_copy_uniform_f32_into(99, &mut floats).is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn cached_uniforms_are_live_and_validate_buffers() {
    use ocio_rs::GpuUniformCopy;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    let _guard = gpu_shader_desc_test_lock();
    let value = Arc::new(AtomicUsize::new(2));
    let mut desc = GpuShaderDesc::create().unwrap();
    let getter_value = Arc::clone(&value);
    desc.add_uniform_f64_getter("live", move || {
        getter_value.load(Ordering::SeqCst) as f64 + 0.25
    })
    .unwrap();
    desc.add_uniform_bool("enabled", true).unwrap();
    desc.add_uniform_float3("color", [0.1, 0.2, 0.3]).unwrap();
    desc.add_uniform_f32_array("curve", &[1.0, 2.0, 3.0], 4)
        .unwrap();
    desc.add_uniform_i32_array("indices", &[4, 5], 4).unwrap();
    let mut cache = desc.uniform_cache().unwrap();
    assert_eq!(cache.double(0).unwrap(), Some(2.25));
    value.store(7, Ordering::SeqCst);
    assert_eq!(cache.double(0).unwrap(), Some(7.25));
    assert_eq!(cache.double(1).unwrap(), None);
    let mut floats = [9.0; 3];
    assert_eq!(
        cache.copy_f32_into(1, &mut floats).unwrap(),
        GpuUniformCopy::Written(1)
    );
    assert_eq!(floats, [1.0, 9.0, 9.0]);
    assert_eq!(
        cache.copy_f32_into(2, &mut floats).unwrap(),
        GpuUniformCopy::Written(3)
    );
    assert_eq!(floats, [0.1, 0.2, 0.3]);
    assert_eq!(
        cache.copy_f32_into(3, &mut floats[..2]).unwrap(),
        GpuUniformCopy::BufferTooSmall { required: 3 }
    );
    assert_eq!(floats, [0.1, 0.2, 0.3]);
    assert_eq!(
        cache.copy_f32_into(3, &mut floats).unwrap(),
        GpuUniformCopy::Written(3)
    );
    assert_eq!(floats, [1.0, 2.0, 3.0]);
    assert_eq!(
        cache.copy_f32_into(4, &mut floats).unwrap(),
        GpuUniformCopy::WrongType
    );
    let mut ints = [9; 2];
    assert_eq!(
        cache.copy_i32_into(4, &mut ints[..1]).unwrap(),
        GpuUniformCopy::BufferTooSmall { required: 2 }
    );
    assert_eq!(ints, [9, 9]);
    assert_eq!(
        cache.copy_i32_into(4, &mut ints).unwrap(),
        GpuUniformCopy::Written(2)
    );
    assert_eq!(ints, [4, 5]);
    assert!(cache.copy_i32_into(99, &mut ints).is_err());
    assert_eq!(cache.double(0).unwrap(), Some(7.25));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn cached_uniform_getter_panic_is_reported_and_recoverable() {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    let _guard = gpu_shader_desc_test_lock();
    let fail = Arc::new(AtomicBool::new(true));
    let getter_fail = Arc::clone(&fail);
    let mut desc = GpuShaderDesc::create().unwrap();
    desc.add_uniform_f64_getter("possibly_failing", move || {
        assert!(
            !getter_fail.load(Ordering::SeqCst),
            "intentional getter panic"
        );
        3.5
    })
    .unwrap();
    let mut cache = desc.uniform_cache().unwrap();
    assert!(cache.double(0).is_err());
    fail.store(false, Ordering::SeqCst);
    assert_eq!(cache.double(0).unwrap(), Some(3.5));
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn cached_native_dynamic_uniform_tracks_property() {
    let _guard = gpu_shader_desc_test_lock();
    let (_processor, mut desc) = extracted_dynamic_gpu_shader_desc().unwrap();
    let prop = desc
        .dynamic_property(DynamicPropertyType::Exposure)
        .unwrap();
    let index = (0..desc.num_uniforms())
        .find(|&index| {
            desc.uniform(index)
                .is_some_and(|u| u.uniform_type == GpuUniformType::Double)
        })
        .unwrap();
    let mut cache = desc.uniform_cache().unwrap();
    let before = cache.double(index).unwrap().unwrap();
    prop.set_double_value(-1.0).unwrap();
    let after = cache.double(index).unwrap().unwrap();
    assert_ne!(before, after);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn cached_array_uniform_tracks_size_and_recovers_from_invalid_getter() {
    use ocio_rs::GpuUniformCopy;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    let _guard = gpu_shader_desc_test_lock();
    let length = Arc::new(AtomicUsize::new(0));
    let getter_length = Arc::clone(&length);
    let mut desc = GpuShaderDesc::create().unwrap();
    desc.add_uniform_f32_array_getter("live_array", 3, move |out| {
        out.copy_from_slice(&[1.0, 2.0, 3.0]);
        getter_length.load(Ordering::SeqCst)
    })
    .unwrap();
    let mut cache = desc.uniform_cache().unwrap();
    let mut output = [9.0; 3];
    assert_eq!(
        cache.copy_f32_into(0, &mut []).unwrap(),
        GpuUniformCopy::Written(0)
    );
    length.store(3, Ordering::SeqCst);
    assert_eq!(
        cache.copy_f32_into(0, &mut output[..2]).unwrap(),
        GpuUniformCopy::BufferTooSmall { required: 3 }
    );
    assert_eq!(output, [9.0; 3]);
    assert_eq!(
        cache.copy_f32_into(0, &mut output).unwrap(),
        GpuUniformCopy::Written(3)
    );
    assert_eq!(output, [1.0, 2.0, 3.0]);
    length.store(4, Ordering::SeqCst);
    assert!(cache.copy_f32_into(0, &mut output).is_err());
    assert_eq!(output, [1.0, 2.0, 3.0]);
    length.store(1, Ordering::SeqCst);
    assert_eq!(
        cache.copy_f32_into(0, &mut output).unwrap(),
        GpuUniformCopy::Written(1)
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_uniform_array_getters_refresh_and_validate_snapshots() {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    let _guard = gpu_shader_desc_test_lock();
    let desc = GpuShaderDesc::create().unwrap();
    let count = Arc::new(AtomicUsize::new(2));
    let n = count.clone();
    assert!(desc
        .add_uniform_f32_array_getter("floats", 4, move |out| {
            for (i, x) in out.iter_mut().enumerate() {
                *x = i as f32 + 0.5;
            }
            n.load(Ordering::Relaxed)
        })
        .unwrap());
    assert!(desc
        .add_uniform_i32_array_getter("ints", 3, |out| {
            out.copy_from_slice(&[3, 4, 5]);
            3
        })
        .unwrap());
    let mut floats = [0.0; 4];
    let mut ints = [0; 3];
    assert!(desc.try_copy_uniform_f32_into(0, &mut floats).unwrap());
    assert_eq!(&floats[..2], &[0.5, 1.5]);
    assert!(desc.try_copy_uniform_i32_into(1, &mut ints).unwrap());
    assert_eq!(ints, [3, 4, 5]);
    count.store(4, Ordering::Relaxed);
    assert!(desc.try_copy_uniform_f32_into(0, &mut floats).unwrap());
    assert_eq!(floats, [0.5, 1.5, 2.5, 3.5]);
    count.store(5, Ordering::Relaxed);
    assert!(desc.try_copy_uniform_f32_into(0, &mut floats).is_err());
    assert!(desc
        .add_uniform_i32_array_getter("panic", 1, |_| panic!("array panic"))
        .unwrap());
    assert!(desc.try_copy_uniform_i32_into(2, &mut ints).is_err());
    assert!(desc
        .add_uniform_f32_array_getter("empty", 0, |out| {
            assert!(out.is_empty());
            0
        })
        .unwrap());
    assert_eq!(desc.try_uniform_value_count(3).unwrap(), 0);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn structured_uniform_uses_actual_dynamic_array_snapshot_length() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let _guard = gpu_shader_desc_test_lock();
    let desc = GpuShaderDesc::create().unwrap();
    let shrinking = AtomicUsize::new(0);
    assert!(desc
        .add_uniform_f32_array_getter("shrinking", 3, move |out| {
            out.copy_from_slice(&[1.0, 2.0, 3.0]);
            if shrinking.fetch_add(1, Ordering::Relaxed) == 0 {
                3
            } else {
                1
            }
        })
        .unwrap());
    let growing = AtomicUsize::new(0);
    assert!(desc
        .add_uniform_i32_array_getter("growing", 3, move |out| {
            out.copy_from_slice(&[4, 5, 6]);
            if growing.fetch_add(1, Ordering::Relaxed) == 0 {
                1
            } else {
                3
            }
        })
        .unwrap());
    let one = desc.try_uniform(0).unwrap().unwrap();
    assert_eq!(one.value_count, 1);
    assert!(matches!(one.value, GpuUniformValue::F32(ref v) if v == &[1.0]));
    let three = desc.try_uniform(1).unwrap().unwrap();
    assert_eq!(three.value_count, 3);
    assert!(matches!(three.value, GpuUniformValue::I32(ref v) if v == &[4, 5, 6]));
    let increasing = AtomicUsize::new(0);
    assert!(desc
        .add_uniform_f32_array_getter("increasing", 4, move |_| {
            increasing.fetch_add(1, Ordering::Relaxed) + 1
        })
        .unwrap());
    assert!(desc.try_uniform(2).is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_uniform_array_getter_recovers_after_partial_write_and_panic() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let _guard = gpu_shader_desc_test_lock();
    let desc = GpuShaderDesc::create().unwrap();
    let calls = AtomicUsize::new(0);
    assert!(desc
        .add_uniform_f32_array_getter("recover", 2, move |out| {
            let call = calls.fetch_add(1, Ordering::Relaxed);
            if call == 1 {
                out[0] = 999.0;
                panic!("interrupted update");
            }
            out.copy_from_slice(&[1.0, 2.0]);
            2
        })
        .unwrap());
    let mut out = [0.0; 2];
    assert!(desc.try_copy_uniform_f32_into(0, &mut out).unwrap());
    assert_eq!(out, [1.0, 2.0]);
    assert!(desc.try_copy_uniform_f32_into(0, &mut out).is_err());
    assert_eq!(out, [1.0, 2.0]);
    assert!(desc.try_copy_uniform_f32_into(0, &mut out).unwrap());
    assert_eq!(out, [1.0, 2.0]);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn uniform_array_sys_rejects_unrepresentable_lengths_and_accepts_empty() {
    let _guard = gpu_shader_desc_test_lock();
    let name = CString::new("empty").unwrap();
    let invalid = CString::new("invalid").unwrap();
    unsafe {
        let desc = ocio_sys::ocio_gpu_shader_desc_create();
        assert!(!desc.is_null());
        assert!(ocio_sys::ocio_gpu_shader_desc_add_uniform_vector_float(
            desc,
            name.as_ptr(),
            std::ptr::null(),
            0,
            0
        ));
        assert!(!ocio_sys::ocio_gpu_shader_desc_add_uniform_vector_int(
            desc,
            invalid.as_ptr(),
            &0i32,
            i32::MAX as usize + 1,
            u32::MAX
        ));
        ocio_sys::ocio_gpu_shader_desc_destroy(desc);
    }
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_uniform_double_getter_tracks_values_and_owns_closure() {
    use std::sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    };

    let _guard = gpu_shader_desc_test_lock();
    let desc = GpuShaderDesc::create().unwrap();
    let value = Arc::new(AtomicU64::new(1.25f64.to_bits()));
    let captured = value.clone();
    assert!(desc
        .add_uniform_f64_getter("dynamic", move || {
            f64::from_bits(captured.load(Ordering::Relaxed))
        })
        .unwrap());
    assert_eq!(Arc::strong_count(&value), 2);
    let duplicate = value.clone();
    assert!(!desc
        .add_uniform_f64_getter("dynamic", move || {
            f64::from_bits(duplicate.load(Ordering::Relaxed))
        })
        .unwrap());
    assert_eq!(Arc::strong_count(&value), 2);
    let mut out = [0.0];
    assert!(desc.try_copy_uniform_f32_into(0, &mut out).unwrap());
    assert_eq!(out, [1.25]);
    value.store(2.5f64.to_bits(), Ordering::Relaxed);
    assert!(desc.try_copy_uniform_f32_into(0, &mut out).unwrap());
    assert_eq!(out, [2.5]);
    let precise = 1.0f64 + f64::EPSILON;
    value.store(precise.to_bits(), Ordering::Relaxed);
    assert_eq!(desc.try_uniform_f64_value(0).unwrap(), Some(precise));
    assert!(desc.add_uniform_bool("flag", true).unwrap());
    assert_eq!(desc.try_uniform_f64_value(1).unwrap(), None);
    assert!(desc.try_uniform_f64_value(99).is_err());
    drop(desc);
    assert_eq!(Arc::strong_count(&value), 1);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_uniform_double_getter_contains_panic() {
    let _guard = gpu_shader_desc_test_lock();
    let desc = GpuShaderDesc::create().unwrap();
    assert!(desc
        .add_uniform_f64_getter("panics", || panic!("getter panic"))
        .unwrap());
    assert!(desc.try_copy_uniform_f32_into(0, &mut [0.0]).is_err());
    assert!(desc.try_uniform_f64_value(0).is_err());
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn rust_uniform_bool_and_float3_getters_are_live_and_contain_panics() {
    use std::sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    };

    let _guard = gpu_shader_desc_test_lock();
    let desc = GpuShaderDesc::create().unwrap();
    let enabled = Arc::new(AtomicBool::new(false));
    let component = Arc::new(AtomicU32::new(1.0f32.to_bits()));
    let flag = enabled.clone();
    let part = component.clone();
    assert!(desc
        .add_uniform_bool_getter("flag", move || flag.load(Ordering::Relaxed))
        .unwrap());
    assert!(desc
        .add_uniform_float3_getter("rgb", move || {
            [f32::from_bits(part.load(Ordering::Relaxed)), 2.0, 3.0]
        })
        .unwrap());
    let mut out = [0.0; 3];
    assert!(desc.try_copy_uniform_f32_into(0, &mut out).unwrap());
    assert_eq!(out[0], 0.0);
    assert_eq!(desc.try_uniform_bool_value(0).unwrap(), Some(false));
    assert_eq!(desc.try_uniform_bool_value(1).unwrap(), None);
    assert!(desc.try_uniform_bool_value(99).is_err());
    assert!(desc.try_copy_uniform_f32_into(1, &mut out).unwrap());
    assert_eq!(out, [1.0, 2.0, 3.0]);
    enabled.store(true, Ordering::Relaxed);
    component.store(4.0f32.to_bits(), Ordering::Relaxed);
    assert!(desc.try_copy_uniform_f32_into(0, &mut out).unwrap());
    assert_eq!(out[0], 1.0);
    assert_eq!(desc.try_uniform_bool_value(0).unwrap(), Some(true));
    assert!(desc.try_copy_uniform_f32_into(1, &mut out).unwrap());
    assert_eq!(out, [4.0, 2.0, 3.0]);
    assert!(desc
        .add_uniform_bool_getter("bad_bool", || panic!("bool panic"))
        .unwrap());
    assert!(desc
        .add_uniform_float3_getter("bad_rgb", || panic!("float3 panic"))
        .unwrap());
    assert!(desc.try_copy_uniform_f32_into(2, &mut out).is_err());
    assert!(desc.try_uniform_bool_value(2).is_err());
    assert!(desc.try_copy_uniform_f32_into(3, &mut out).is_err());
    drop(desc);
    assert_eq!(Arc::strong_count(&enabled), 1);
    assert_eq!(Arc::strong_count(&component), 1);
}

#[cfg(ocio_stub)]
#[test]
fn rust_uniform_double_getter_stub_releases_closure() {
    let value = std::sync::Arc::new(());
    let captured = value.clone();
    let desc = GpuShaderDesc::create().unwrap();
    assert!(desc
        .add_uniform_f64_getter("dynamic", move || {
            let _ = &captured;
            1.0
        })
        .is_err());
    assert_eq!(std::sync::Arc::strong_count(&value), 1);
    let captured = value.clone();
    assert!(desc
        .add_uniform_bool_getter("flag", move || {
            let _ = &captured;
            true
        })
        .is_err());
    assert_eq!(std::sync::Arc::strong_count(&value), 1);
    let captured = value.clone();
    assert!(desc
        .add_uniform_float3_getter("rgb", move || {
            let _ = &captured;
            [0.0; 3]
        })
        .is_err());
    assert_eq!(std::sync::Arc::strong_count(&value), 1);
    let captured = value.clone();
    assert!(desc
        .add_uniform_f32_array_getter("float_array", 2, move |_| {
            let _ = &captured;
            0
        })
        .is_err());
    assert_eq!(std::sync::Arc::strong_count(&value), 1);
    let captured = value.clone();
    assert!(desc
        .add_uniform_i32_array_getter("int_array", 2, move |_| {
            let _ = &captured;
            0
        })
        .is_err());
    assert_eq!(std::sync::Arc::strong_count(&value), 1);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
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

        let mut has_binding_2d = false;
        let mut binding_2d = 0;
        let inserted_2d = ocio_sys::ocio_gpu_shader_desc_add_texture(
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
            &mut has_binding_2d,
            &mut binding_2d,
        );

        let mut has_binding_3d = false;
        let mut binding_3d = 0;
        let inserted_3d = ocio_sys::ocio_gpu_shader_desc_add3d_texture(
            desc,
            texture3d_name.as_ptr(),
            sampler3d_name.as_ptr(),
            2,
            Interpolation::Nearest as i32,
            texture3d_values.as_ptr(),
            texture3d_values.len(),
            &mut has_binding_3d,
            &mut binding_3d,
        );
        assert!(inserted_2d);
        assert!(inserted_3d);
        if ocio_rs::version_hex() >= 0x02050000 {
            assert!(has_binding_2d);
            assert!(has_binding_3d);
            assert_eq!(binding_3d, binding_2d + 1);
        } else {
            assert!(!has_binding_2d);
            assert!(!has_binding_3d);
        }

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

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn gpu_shader_desc_real_config_extraction_behavior() {
    let _guard = gpu_shader_desc_test_lock();
    if is_stub() {
        return;
    }

    let config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("configs")
        .join("context_test1")
        .join("config.ocio");
    let working_dir = config_path.parent().expect("config parent");

    let config =
        ocio_rs::Config::from_file(config_path.to_string_lossy()).expect("load real config");
    config
        .set_working_dir(working_dir.to_string_lossy())
        .expect("set working dir");

    let processor = config
        .processor("plain_lut1_cs", "reference")
        .expect("create processor");

    let gpu_processor = processor
        .default_gpu_processor()
        .expect("default gpu processor");

    let desc = GpuShaderDesc::create().expect("gpu shader desc create");
    desc.set_language(GpuLanguage::Glsl4_0)
        .expect("set_language");
    desc.set_function_name("test_real_main")
        .expect("set_function_name");
    desc.set_pixel_name("test_real_pixel")
        .expect("set_pixel_name");
    desc.set_resource_prefix("test_real_")
        .expect("set_resource_prefix");

    let mut desc = desc;
    gpu_processor
        .try_extract_shader_info(&mut desc)
        .expect("extract shader info");

    // --- Shader text validation ---
    let shader_text = desc.shader_text().expect("shader_text should be present");
    assert!(
        !shader_text.trim().is_empty(),
        "shader text should not be empty for real config"
    );
    assert!(
        shader_text.contains("test_real_main"),
        "shader text should contain the function name"
    );
    // Real GLSL shaders typically contain type declarations
    assert!(
        shader_text.contains("vec4") || shader_text.contains("float"),
        "shader text should contain GLSL type keywords, got: {}",
        &shader_text[..shader_text.len().min(200)]
    );

    // --- Uniform validation ---
    let num_uniforms = desc.num_uniforms();
    let uniforms = desc.uniforms();
    assert_eq!(
        num_uniforms as usize,
        uniforms.len(),
        "uniform count mismatch"
    );
    for (idx, uniform) in uniforms.iter().enumerate() {
        assert!(
            !uniform.name.trim().is_empty(),
            "uniform name should not be empty at index {}",
            idx
        );
        // Verify round-trip consistency
        let queried = desc.uniform(idx as u32).expect("uniform by index");
        assert_eq!(queried.name, uniform.name);
        assert_eq!(queried.value_count, uniform.value_count);
    }

    // --- Texture 2D validation ---
    let textures_2d = desc.textures_2d();
    assert_eq!(
        desc.num_textures() as usize,
        textures_2d.len(),
        "texture 2D count mismatch"
    );
    for (idx, tex) in textures_2d.iter().enumerate() {
        assert!(
            !tex.texture_name.trim().is_empty(),
            "texture name should not be empty at index {}",
            idx
        );
        assert!(
            !tex.sampler_name.trim().is_empty(),
            "sampler name should not be empty at index {}",
            idx
        );
        let expected_len = tex.expected_value_count();
        assert_eq!(
            tex.values.len(),
            expected_len,
            "texture 2D values length mismatch at index {}: got {}, expected {}",
            idx,
            tex.values.len(),
            expected_len
        );
        // Verify values are within reasonable range (not NaN/Inf)
        for (vi, v) in tex.values.iter().enumerate() {
            assert!(
                (*v as f64).is_finite(),
                "texture 2D value at index {} texel {} is not finite: {}",
                idx,
                vi,
                v
            );
        }
    }

    // --- Texture 3D validation ---
    let textures_3d = desc.textures_3d();
    assert_eq!(
        desc.num_3d_textures() as usize,
        textures_3d.len(),
        "texture 3D count mismatch"
    );
    for (idx, tex) in textures_3d.iter().enumerate() {
        assert!(
            !tex.texture_name.trim().is_empty(),
            "3D texture name should not be empty at index {}",
            idx
        );
        let expected_len = tex.expected_value_count();
        assert_eq!(
            tex.values.len(),
            expected_len,
            "texture 3D values length mismatch at index {}: got {}, expected {}",
            idx,
            tex.values.len(),
            expected_len
        );
        for (vi, v) in tex.values.iter().enumerate() {
            assert!(
                (*v as f64).is_finite(),
                "texture 3D value at index {} texel {} is not finite: {}",
                idx,
                vi,
                v
            );
        }
    }

    // --- Metadata validation ---
    assert_eq!(desc.language(), ocio_rs::GpuLanguage::Glsl4_0);
    assert_eq!(desc.function_name().as_deref(), Some("test_real_main"));
    assert_eq!(desc.pixel_name().as_deref(), Some("test_real_pixel"));
    assert_eq!(desc.resource_prefix().as_deref(), Some("test_real_"));
}
