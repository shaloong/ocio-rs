#include "bridge.hpp"

#include <cstring>
#include <memory>
#include <stdexcept>
#include <string>
#include <fstream>

#ifndef OCIO_RS_STUB
#include <OpenColorIO/OpenColorIO.h>
#include <OpenColorIO/OpenColorTransforms.h>
namespace ocio = OCIO_NAMESPACE;
#endif

namespace ocio_rs_bridge {

// --- Handle types ---

struct ConfigHandle { std::shared_ptr<void> inner; };
struct ProcessorHandle { std::shared_ptr<void> inner; };
struct CPUProcessorHandle { std::shared_ptr<void> inner; };
struct GPUProcessorHandle { std::shared_ptr<void> inner; };
struct GpuShaderDescHandle { std::shared_ptr<void> inner; };

struct TransformHandleBase {
  virtual ~TransformHandleBase() = default;
  virtual int get_transform_type_tag() const = 0;
#ifndef OCIO_RS_STUB
  virtual OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() = 0;
#endif
};

struct AllocationTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 0; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct BuiltinTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 1; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct CDLTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 2; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct ColorSpaceTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 3; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct DisplayViewTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 4; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct ExponentTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 5; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct ExponentWithLinearTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 6; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct ExposureContrastTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 7; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct FileTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 8; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct FixedFunctionTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 9; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct GradingPrimaryTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 10; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct GradingRGBCurveTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 11; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct GradingHueCurveTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 12; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct GradingToneTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 13; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct GroupTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 14; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct LogAffineTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 15; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct LogCameraTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 16; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct LogTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 17; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct LookTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 18; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct Lut1DTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 19; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct Lut3DTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 20; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct MatrixTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 21; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct RangeTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 22; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};

struct BakerHandle { std::shared_ptr<void> inner; };
struct ContextHandle { std::shared_ptr<void> inner; };
struct ColorSpaceHandle { std::shared_ptr<void> inner; };
struct LookHandle { std::shared_ptr<void> inner; };
struct ViewTransformHandle { std::shared_ptr<void> inner; };
struct NamedTransformHandle { std::shared_ptr<void> inner; };
struct DynamicPropertyHandle { std::shared_ptr<void> inner; };
struct BuiltinConfigRegistryHandle { std::shared_ptr<void> inner; };
struct FileRulesHandle { std::shared_ptr<void> inner; };
struct ColorSpaceSetHandle { std::shared_ptr<void> inner; };
struct FormatMetadataHandle { std::shared_ptr<void> inner; };

// Handle types for referenced-only classes
struct TransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return -1; }
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct ConfigIOProxyHandle { std::shared_ptr<void> inner; };
struct ViewingRulesHandle { std::shared_ptr<void> inner; };
struct ProcessorMetadataHandle { std::shared_ptr<void> inner; };
struct GpuShaderCreatorHandle { std::shared_ptr<void> inner; };
struct GradingRGBCurveHandle { std::shared_ptr<void> inner; };
struct GradingHueCurveHandle { std::shared_ptr<void> inner; };
struct MixingSliderHandle { std::shared_ptr<void> inner; };
#ifdef OCIO_RS_STUB

// --- Stub wrapper structs ---
struct StubFileTransform {};
struct StubAllocationTransform {};
struct StubLook {};
struct StubGpuShaderDesc {};
struct StubNamedTransform {};
struct StubProcessor {};
struct StubContext {};
struct StubViewTransform {};
struct StubLut1DTransform {};
struct StubColorSpaceTransform {};
struct StubFileRules {};
struct StubGradingPrimaryTransform {};
struct StubLogTransform {};
struct StubExposureContrastTransform {};
struct StubGroupTransform {};
struct StubConfig {};
struct StubLogAffineTransform {};
struct StubLookTransform {};
struct StubGradingHueCurveTransform {};
struct StubRangeTransform {};
struct StubDynamicProperty {};
struct StubCPUProcessor {};
struct StubExponentTransform {};
struct StubDisplayViewTransform {};
struct StubExponentWithLinearTransform {};
struct StubMatrixTransform {};
struct StubBuiltinTransform {};
struct StubColorSpaceSet {};
struct StubBaker {};
struct StubGradingToneTransform {};
struct StubGPUProcessor {};
struct StubCDLTransform {};
struct StubFixedFunctionTransform {};
struct StubLogCameraTransform {};
struct StubLut3DTransform {};
struct StubColorSpace {};
struct StubGradingRGBCurveTransform {};
struct StubBuiltinConfigRegistry {
  int getNumBuiltinConfigs() { return 0; }
  const char* getBuiltinConfigName(int) { return nullptr; }
};

// --- Stub make functions ---
static std::unique_ptr<FileTransformHandle> make_stub_file_transform() {
  auto handle = std::make_unique<FileTransformHandle>();
  handle->inner = std::make_shared<StubFileTransform>();
  return handle;
}

static std::unique_ptr<AllocationTransformHandle> make_stub_allocation_transform() {
  auto handle = std::make_unique<AllocationTransformHandle>();
  handle->inner = std::make_shared<StubAllocationTransform>();
  return handle;
}

static std::unique_ptr<LookHandle> make_stub_look() {
  auto handle = std::make_unique<LookHandle>();
  handle->inner = std::make_shared<StubLook>();
  return handle;
}

static std::unique_ptr<GpuShaderDescHandle> make_stub_gpu_shader_desc() {
  auto handle = std::make_unique<GpuShaderDescHandle>();
  handle->inner = std::make_shared<StubGpuShaderDesc>();
  return handle;
}

static std::unique_ptr<NamedTransformHandle> make_stub_named_transform() {
  auto handle = std::make_unique<NamedTransformHandle>();
  handle->inner = std::make_shared<StubNamedTransform>();
  return handle;
}

static std::unique_ptr<ProcessorHandle> make_stub_processor() {
  auto handle = std::make_unique<ProcessorHandle>();
  handle->inner = std::make_shared<StubProcessor>();
  return handle;
}

static std::unique_ptr<ContextHandle> make_stub_context() {
  auto handle = std::make_unique<ContextHandle>();
  handle->inner = std::make_shared<StubContext>();
  return handle;
}

static std::unique_ptr<ViewTransformHandle> make_stub_view_transform() {
  auto handle = std::make_unique<ViewTransformHandle>();
  handle->inner = std::make_shared<StubViewTransform>();
  return handle;
}

static std::unique_ptr<Lut1DTransformHandle> make_stub_lut1d_transform() {
  auto handle = std::make_unique<Lut1DTransformHandle>();
  handle->inner = std::make_shared<StubLut1DTransform>();
  return handle;
}

static std::unique_ptr<ColorSpaceTransformHandle> make_stub_color_space_transform() {
  auto handle = std::make_unique<ColorSpaceTransformHandle>();
  handle->inner = std::make_shared<StubColorSpaceTransform>();
  return handle;
}

static std::unique_ptr<FileRulesHandle> make_stub_file_rules() {
  auto handle = std::make_unique<FileRulesHandle>();
  handle->inner = std::make_shared<StubFileRules>();
  return handle;
}

static std::unique_ptr<GradingPrimaryTransformHandle> make_stub_grading_primary_transform() {
  auto handle = std::make_unique<GradingPrimaryTransformHandle>();
  handle->inner = std::make_shared<StubGradingPrimaryTransform>();
  return handle;
}

static std::unique_ptr<LogTransformHandle> make_stub_log_transform() {
  auto handle = std::make_unique<LogTransformHandle>();
  handle->inner = std::make_shared<StubLogTransform>();
  return handle;
}

static std::unique_ptr<BuiltinConfigRegistryHandle> make_stub_builtin_config_registry() {
  auto handle = std::make_unique<BuiltinConfigRegistryHandle>();
  handle->inner = std::make_shared<StubBuiltinConfigRegistry>();
  return handle;
}

static std::unique_ptr<ExposureContrastTransformHandle> make_stub_exposure_contrast_transform() {
  auto handle = std::make_unique<ExposureContrastTransformHandle>();
  handle->inner = std::make_shared<StubExposureContrastTransform>();
  return handle;
}

static std::unique_ptr<GroupTransformHandle> make_stub_group_transform() {
  auto handle = std::make_unique<GroupTransformHandle>();
  handle->inner = std::make_shared<StubGroupTransform>();
  return handle;
}

static std::unique_ptr<ConfigHandle> make_stub_config() {
  auto handle = std::make_unique<ConfigHandle>();
  handle->inner = std::make_shared<StubConfig>();
  return handle;
}

static std::unique_ptr<LogAffineTransformHandle> make_stub_log_affine_transform() {
  auto handle = std::make_unique<LogAffineTransformHandle>();
  handle->inner = std::make_shared<StubLogAffineTransform>();
  return handle;
}

static std::unique_ptr<LookTransformHandle> make_stub_look_transform() {
  auto handle = std::make_unique<LookTransformHandle>();
  handle->inner = std::make_shared<StubLookTransform>();
  return handle;
}

static std::unique_ptr<GradingHueCurveTransformHandle> make_stub_grading_hue_curve_transform() {
  auto handle = std::make_unique<GradingHueCurveTransformHandle>();
  handle->inner = std::make_shared<StubGradingHueCurveTransform>();
  return handle;
}

static std::unique_ptr<RangeTransformHandle> make_stub_range_transform() {
  auto handle = std::make_unique<RangeTransformHandle>();
  handle->inner = std::make_shared<StubRangeTransform>();
  return handle;
}

static std::unique_ptr<DynamicPropertyHandle> make_stub_dynamic_property() {
  auto handle = std::make_unique<DynamicPropertyHandle>();
  handle->inner = std::make_shared<StubDynamicProperty>();
  return handle;
}

static std::unique_ptr<CPUProcessorHandle> make_stub_cpu_processor() {
  auto handle = std::make_unique<CPUProcessorHandle>();
  handle->inner = std::make_shared<StubCPUProcessor>();
  return handle;
}

static std::unique_ptr<ExponentTransformHandle> make_stub_exponent_transform() {
  auto handle = std::make_unique<ExponentTransformHandle>();
  handle->inner = std::make_shared<StubExponentTransform>();
  return handle;
}

static std::unique_ptr<DisplayViewTransformHandle> make_stub_display_view_transform() {
  auto handle = std::make_unique<DisplayViewTransformHandle>();
  handle->inner = std::make_shared<StubDisplayViewTransform>();
  return handle;
}

static std::unique_ptr<ExponentWithLinearTransformHandle> make_stub_exponent_with_linear_transform() {
  auto handle = std::make_unique<ExponentWithLinearTransformHandle>();
  handle->inner = std::make_shared<StubExponentWithLinearTransform>();
  return handle;
}

static std::unique_ptr<MatrixTransformHandle> make_stub_matrix_transform() {
  auto handle = std::make_unique<MatrixTransformHandle>();
  handle->inner = std::make_shared<StubMatrixTransform>();
  return handle;
}

static std::unique_ptr<BuiltinTransformHandle> make_stub_builtin_transform() {
  auto handle = std::make_unique<BuiltinTransformHandle>();
  handle->inner = std::make_shared<StubBuiltinTransform>();
  return handle;
}

static std::unique_ptr<ColorSpaceSetHandle> make_stub_color_space_set() {
  auto handle = std::make_unique<ColorSpaceSetHandle>();
  handle->inner = std::make_shared<StubColorSpaceSet>();
  return handle;
}

static std::unique_ptr<BakerHandle> make_stub_baker() {
  auto handle = std::make_unique<BakerHandle>();
  handle->inner = std::make_shared<StubBaker>();
  return handle;
}

static std::unique_ptr<GradingToneTransformHandle> make_stub_grading_tone_transform() {
  auto handle = std::make_unique<GradingToneTransformHandle>();
  handle->inner = std::make_shared<StubGradingToneTransform>();
  return handle;
}

static std::unique_ptr<GPUProcessorHandle> make_stub_gpu_processor() {
  auto handle = std::make_unique<GPUProcessorHandle>();
  handle->inner = std::make_shared<StubGPUProcessor>();
  return handle;
}

static std::unique_ptr<CDLTransformHandle> make_stub_cdl_transform() {
  auto handle = std::make_unique<CDLTransformHandle>();
  handle->inner = std::make_shared<StubCDLTransform>();
  return handle;
}

static std::unique_ptr<FixedFunctionTransformHandle> make_stub_fixed_function_transform() {
  auto handle = std::make_unique<FixedFunctionTransformHandle>();
  handle->inner = std::make_shared<StubFixedFunctionTransform>();
  return handle;
}

static std::unique_ptr<LogCameraTransformHandle> make_stub_log_camera_transform() {
  auto handle = std::make_unique<LogCameraTransformHandle>();
  handle->inner = std::make_shared<StubLogCameraTransform>();
  return handle;
}

static std::unique_ptr<Lut3DTransformHandle> make_stub_lut3d_transform() {
  auto handle = std::make_unique<Lut3DTransformHandle>();
  handle->inner = std::make_shared<StubLut3DTransform>();
  return handle;
}

static std::unique_ptr<ColorSpaceHandle> make_stub_color_space() {
  auto handle = std::make_unique<ColorSpaceHandle>();
  handle->inner = std::make_shared<StubColorSpace>();
  return handle;
}

static std::unique_ptr<GradingRGBCurveTransformHandle> make_stub_grading_rgb_curve_transform() {
  auto handle = std::make_unique<GradingRGBCurveTransformHandle>();
  handle->inner = std::make_shared<StubGradingRGBCurveTransform>();
  return handle;
}

static std::unique_ptr<ConfigHandle> make_stub_config_raw() {
  auto handle = std::make_unique<ConfigHandle>();
  handle->inner = std::make_shared<StubConfig>();
  return handle;
}

#else // real OCIO

// --- Real OCIO wrapper types ---
struct RealFileTransform {
  ocio::FileTransformRcPtr transform;
};
struct RealAllocationTransform {
  ocio::AllocationTransformRcPtr transform;
};
struct RealLook {
  ocio::LookRcPtr look;
};
struct RealGpuShaderDesc {
  ocio::GpuShaderDescRcPtr gpuShaderDesc;
};
struct RealNamedTransform {
  ocio::NamedTransformRcPtr transform;
};
struct RealProcessor {
  ocio::ProcessorRcPtr processor;
};
struct RealContext {
  ocio::ContextRcPtr context;
};
struct RealViewTransform {
  ocio::ViewTransformRcPtr transform;
};
struct RealLut1DTransform {
  ocio::Lut1DTransformRcPtr transform;
};
struct RealColorSpaceTransform {
  ocio::ColorSpaceTransformRcPtr transform;
};
struct RealFileRules {
  ocio::FileRulesRcPtr rules;
};
struct RealGradingPrimaryTransform {
  ocio::GradingPrimaryTransformRcPtr transform;
};
struct RealLogTransform {
  ocio::LogTransformRcPtr transform;
};
struct RealBuiltinConfigRegistry {
  const ocio::BuiltinConfigRegistry* registry;
};
struct RealExposureContrastTransform {
  ocio::ExposureContrastTransformRcPtr transform;
};
struct RealGroupTransform {
  ocio::GroupTransformRcPtr transform;
};
struct RealConfig {
  ocio::ConfigRcPtr config;
};
struct RealLogAffineTransform {
  ocio::LogAffineTransformRcPtr transform;
};
struct RealLookTransform {
  ocio::LookTransformRcPtr transform;
};
struct RealGradingHueCurveTransform {
  ocio::GradingHueCurveTransformRcPtr transform;
};
struct RealRangeTransform {
  ocio::RangeTransformRcPtr transform;
};
struct RealDynamicProperty {
  ocio::DynamicPropertyRcPtr prop;
};
struct RealCPUProcessor {
  ocio::CPUProcessorRcPtr cpu;
};
struct RealExponentTransform {
  ocio::ExponentTransformRcPtr transform;
};
struct RealDisplayViewTransform {
  ocio::DisplayViewTransformRcPtr transform;
};
struct RealExponentWithLinearTransform {
  ocio::ExponentWithLinearTransformRcPtr transform;
};
struct RealMatrixTransform {
  ocio::MatrixTransformRcPtr transform;
};
struct RealBuiltinTransform {
  ocio::BuiltinTransformRcPtr transform;
};
struct RealColorSpaceSet {
  ocio::ColorSpaceSetRcPtr set;
};
struct RealBaker {
  ocio::BakerRcPtr baker;
};
struct RealGradingToneTransform {
  ocio::GradingToneTransformRcPtr transform;
};
struct RealGPUProcessor {
  ocio::GPUProcessorRcPtr gpu;
};
struct RealCDLTransform {
  ocio::CDLTransformRcPtr transform;
};
struct RealFixedFunctionTransform {
  ocio::FixedFunctionTransformRcPtr transform;
};
struct RealLogCameraTransform {
  ocio::LogCameraTransformRcPtr transform;
};
struct RealLut3DTransform {
  ocio::Lut3DTransformRcPtr transform;
};
struct RealColorSpace {
  ocio::ColorSpaceRcPtr colorSpace;
};
struct RealGradingRGBCurveTransform {
  ocio::GradingRGBCurveTransformRcPtr transform;
};
// Real types for referenced-only classes
struct RealTransform {
  ocio::TransformRcPtr transform;
};
struct RealConfigIOProxy {
  ocio::ConfigIOProxyRcPtr proxy;
};
struct RealViewingRules {
  ocio::ViewingRulesRcPtr rules;
};
struct RealProcessorMetadata {
  ocio::ProcessorMetadataRcPtr metadata;
};
struct RealGpuShaderCreator {
  ocio::GpuShaderCreatorRcPtr shader;
};
struct RealGradingRGBCurve {
  ocio::GradingRGBCurveRcPtr curve;
};
struct RealGradingHueCurve {
  ocio::GradingHueCurveRcPtr curve;
};
struct RealMixingSlider {
  ocio::MixingSlider* slider;
};

// --- TransformHandleBase out-of-line ---
ocio::TransformRcPtr TransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealTransform>(inner)->transform;
}
ocio::TransformRcPtr AllocationTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealAllocationTransform>(inner)->transform;
}
ocio::TransformRcPtr BuiltinTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealBuiltinTransform>(inner)->transform;
}
ocio::TransformRcPtr CDLTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealCDLTransform>(inner)->transform;
}
ocio::TransformRcPtr ColorSpaceTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealColorSpaceTransform>(inner)->transform;
}
ocio::TransformRcPtr DisplayViewTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealDisplayViewTransform>(inner)->transform;
}
ocio::TransformRcPtr ExponentTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealExponentTransform>(inner)->transform;
}
ocio::TransformRcPtr ExponentWithLinearTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealExponentWithLinearTransform>(inner)->transform;
}
ocio::TransformRcPtr ExposureContrastTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealExposureContrastTransform>(inner)->transform;
}
ocio::TransformRcPtr FileTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealFileTransform>(inner)->transform;
}
ocio::TransformRcPtr FixedFunctionTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealFixedFunctionTransform>(inner)->transform;
}
ocio::TransformRcPtr GradingPrimaryTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealGradingPrimaryTransform>(inner)->transform;
}
ocio::TransformRcPtr GradingRGBCurveTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealGradingRGBCurveTransform>(inner)->transform;
}
ocio::TransformRcPtr GradingHueCurveTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealGradingHueCurveTransform>(inner)->transform;
}
ocio::TransformRcPtr GradingToneTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealGradingToneTransform>(inner)->transform;
}
ocio::TransformRcPtr GroupTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealGroupTransform>(inner)->transform;
}
ocio::TransformRcPtr LogAffineTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLogAffineTransform>(inner)->transform;
}
ocio::TransformRcPtr LogCameraTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLogCameraTransform>(inner)->transform;
}
ocio::TransformRcPtr LogTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLogTransform>(inner)->transform;
}
ocio::TransformRcPtr LookTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLookTransform>(inner)->transform;
}
ocio::TransformRcPtr Lut1DTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLut1DTransform>(inner)->transform;
}
ocio::TransformRcPtr Lut3DTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLut3DTransform>(inner)->transform;
}
ocio::TransformRcPtr MatrixTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealMatrixTransform>(inner)->transform;
}
ocio::TransformRcPtr RangeTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealRangeTransform>(inner)->transform;
}

// --- Real accessor functions ---
static ocio::ConfigRcPtr get_real_config(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ConfigHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealConfig>(h->inner)->config;
}

static ocio::FileRulesRcPtr get_real_file_rules(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
}

static ocio::ColorSpaceRcPtr get_real_color_space(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace;
}

static ocio::ColorSpaceSetRcPtr get_real_color_space_set(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceSet>(h->inner)->set;
}

static ocio::LookRcPtr get_real_look(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::LookHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner)->look;
}

static ocio::NamedTransformRcPtr get_real_named_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(h->inner)->transform;
}

static ocio::ViewTransformRcPtr get_real_view_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform;
}

static ocio::ProcessorRcPtr get_real_processor(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ProcessorHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealProcessor>(h->inner)->processor;
}

static ocio::CPUProcessorRcPtr get_real_cpu_processor(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::CPUProcessorHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealCPUProcessor>(h->inner)->cpu;
}

static ocio::GPUProcessorRcPtr get_real_gpu_processor(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::GPUProcessorHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealGPUProcessor>(h->inner)->gpu;
}

static ocio::GpuShaderDescRcPtr get_real_gpu_shader_desc(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealGpuShaderDesc>(h->inner)->gpuShaderDesc;
}

static ocio::BakerRcPtr get_real_baker(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker;
}

static ocio::ContextRcPtr get_real_context(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context;
}

static ocio::DynamicPropertyRcPtr get_real_dynamic_property(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(h->inner)->prop;
}

static const ocio::BuiltinConfigRegistry* get_real_builtin_config_registry(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::BuiltinConfigRegistryHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealBuiltinConfigRegistry>(h->inner)->registry;
}

static ocio::AllocationTransformRcPtr get_real_allocation_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::AllocationTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealAllocationTransform>(h->inner)->transform;
}

static ocio::BuiltinTransformRcPtr get_real_builtin_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::BuiltinTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealBuiltinTransform>(h->inner)->transform;
}

static ocio::CDLTransformRcPtr get_real_cdl_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform;
}

static ocio::ColorSpaceTransformRcPtr get_real_color_space_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceTransform>(h->inner)->transform;
}

static ocio::DisplayViewTransformRcPtr get_real_display_view_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(h->inner)->transform;
}

static ocio::ExponentTransformRcPtr get_real_exponent_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ExponentTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealExponentTransform>(h->inner)->transform;
}

static ocio::ExponentWithLinearTransformRcPtr get_real_exponent_with_linear_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealExponentWithLinearTransform>(h->inner)->transform;
}

static ocio::ExposureContrastTransformRcPtr get_real_exposure_contrast_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(h->inner)->transform;
}

static ocio::FileTransformRcPtr get_real_file_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform;
}

static ocio::FixedFunctionTransformRcPtr get_real_fixed_function_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealFixedFunctionTransform>(h->inner)->transform;
}

static ocio::GradingPrimaryTransformRcPtr get_real_grading_primary_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryTransform>(h->inner)->transform;
}

static ocio::GradingRGBCurveTransformRcPtr get_real_grading_rgb_curve_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(h->inner)->transform;
}

static ocio::GradingHueCurveTransformRcPtr get_real_grading_hue_curve_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(h->inner)->transform;
}

static ocio::GradingToneTransformRcPtr get_real_grading_tone_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealGradingToneTransform>(h->inner)->transform;
}

static ocio::GroupTransformRcPtr get_real_group_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::GroupTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(h->inner)->transform;
}

static ocio::LogAffineTransformRcPtr get_real_log_affine_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(h->inner)->transform;
}

static ocio::LogCameraTransformRcPtr get_real_log_camera_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(h->inner)->transform;
}

static ocio::LogTransformRcPtr get_real_log_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::LogTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealLogTransform>(h->inner)->transform;
}

static ocio::LookTransformRcPtr get_real_look_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::LookTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(h->inner)->transform;
}

static ocio::Lut1DTransformRcPtr get_real_lut1d_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
}

static ocio::Lut3DTransformRcPtr get_real_lut3d_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform;
}

static ocio::MatrixTransformRcPtr get_real_matrix_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform;
}

static ocio::RangeTransformRcPtr get_real_range_transform(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform;
}


// --- Real make functions ---
static std::unique_ptr<FileTransformHandle> make_real_file_transform() {
  try {
    auto handle = std::make_unique<FileTransformHandle>();
    auto obj = std::make_shared<RealFileTransform>();
    obj->transform = ocio::FileTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<AllocationTransformHandle> make_real_allocation_transform() {
  try {
    auto handle = std::make_unique<AllocationTransformHandle>();
    auto obj = std::make_shared<RealAllocationTransform>();
    obj->transform = ocio::AllocationTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<LookHandle> make_real_look() {
  try {
    auto handle = std::make_unique<LookHandle>();
    auto obj = std::make_shared<RealLook>();
    obj->look = ocio::Look::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<NamedTransformHandle> make_real_named_transform() {
  try {
    auto handle = std::make_unique<NamedTransformHandle>();
    auto obj = std::make_shared<RealNamedTransform>();
    obj->transform = ocio::NamedTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<ContextHandle> make_real_context() {
  try {
    auto handle = std::make_unique<ContextHandle>();
    auto obj = std::make_shared<RealContext>();
    obj->context = ocio::Context::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<Lut1DTransformHandle> make_real_lut1d_transform() {
  try {
    auto handle = std::make_unique<Lut1DTransformHandle>();
    auto obj = std::make_shared<RealLut1DTransform>();
    obj->transform = ocio::Lut1DTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<ColorSpaceTransformHandle> make_real_color_space_transform() {
  try {
    auto handle = std::make_unique<ColorSpaceTransformHandle>();
    auto obj = std::make_shared<RealColorSpaceTransform>();
    obj->transform = ocio::ColorSpaceTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<FileRulesHandle> make_real_file_rules() {
  try {
    auto handle = std::make_unique<FileRulesHandle>();
    auto obj = std::make_shared<RealFileRules>();
    obj->rules = ocio::FileRules::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<LogTransformHandle> make_real_log_transform() {
  try {
    auto handle = std::make_unique<LogTransformHandle>();
    auto obj = std::make_shared<RealLogTransform>();
    obj->transform = ocio::LogTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<ExposureContrastTransformHandle> make_real_exposure_contrast_transform() {
  try {
    auto handle = std::make_unique<ExposureContrastTransformHandle>();
    auto obj = std::make_shared<RealExposureContrastTransform>();
    obj->transform = ocio::ExposureContrastTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<GroupTransformHandle> make_real_group_transform() {
  try {
    auto handle = std::make_unique<GroupTransformHandle>();
    auto obj = std::make_shared<RealGroupTransform>();
    obj->transform = ocio::GroupTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<LogAffineTransformHandle> make_real_log_affine_transform() {
  try {
    auto handle = std::make_unique<LogAffineTransformHandle>();
    auto obj = std::make_shared<RealLogAffineTransform>();
    obj->transform = ocio::LogAffineTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<LookTransformHandle> make_real_look_transform() {
  try {
    auto handle = std::make_unique<LookTransformHandle>();
    auto obj = std::make_shared<RealLookTransform>();
    obj->transform = ocio::LookTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<RangeTransformHandle> make_real_range_transform() {
  try {
    auto handle = std::make_unique<RangeTransformHandle>();
    auto obj = std::make_shared<RealRangeTransform>();
    obj->transform = ocio::RangeTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<ExponentTransformHandle> make_real_exponent_transform() {
  try {
    auto handle = std::make_unique<ExponentTransformHandle>();
    auto obj = std::make_shared<RealExponentTransform>();
    obj->transform = ocio::ExponentTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<DisplayViewTransformHandle> make_real_display_view_transform() {
  try {
    auto handle = std::make_unique<DisplayViewTransformHandle>();
    auto obj = std::make_shared<RealDisplayViewTransform>();
    obj->transform = ocio::DisplayViewTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<ExponentWithLinearTransformHandle> make_real_exponent_with_linear_transform() {
  try {
    auto handle = std::make_unique<ExponentWithLinearTransformHandle>();
    auto obj = std::make_shared<RealExponentWithLinearTransform>();
    obj->transform = ocio::ExponentWithLinearTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<MatrixTransformHandle> make_real_matrix_transform() {
  try {
    auto handle = std::make_unique<MatrixTransformHandle>();
    auto obj = std::make_shared<RealMatrixTransform>();
    obj->transform = ocio::MatrixTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<BuiltinTransformHandle> make_real_builtin_transform() {
  try {
    auto handle = std::make_unique<BuiltinTransformHandle>();
    auto obj = std::make_shared<RealBuiltinTransform>();
    obj->transform = ocio::BuiltinTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<ColorSpaceSetHandle> make_real_color_space_set() {
  try {
    auto handle = std::make_unique<ColorSpaceSetHandle>();
    auto obj = std::make_shared<RealColorSpaceSet>();
    obj->set = ocio::ColorSpaceSet::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<BakerHandle> make_real_baker() {
  try {
    auto handle = std::make_unique<BakerHandle>();
    auto obj = std::make_shared<RealBaker>();
    obj->baker = ocio::Baker::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<CDLTransformHandle> make_real_cdl_transform() {
  try {
    auto handle = std::make_unique<CDLTransformHandle>();
    auto obj = std::make_shared<RealCDLTransform>();
    obj->transform = ocio::CDLTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<Lut3DTransformHandle> make_real_lut3d_transform() {
  try {
    auto handle = std::make_unique<Lut3DTransformHandle>();
    auto obj = std::make_shared<RealLut3DTransform>();
    obj->transform = ocio::Lut3DTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<ColorSpaceHandle> make_real_color_space() {
  try {
    auto handle = std::make_unique<ColorSpaceHandle>();
    auto obj = std::make_shared<RealColorSpace>();
    obj->colorSpace = ocio::ColorSpace::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<ConfigHandle> make_real_config_raw() {
  try {
    auto handle = std::make_unique<ConfigHandle>();
    auto config = std::make_shared<RealConfig>();
    config->config = std::const_pointer_cast<ocio::Config>(ocio::Config::CreateRaw());
    handle->inner = config;
    return handle;
  } catch (...) { return nullptr; }
}

static std::unique_ptr<ConfigHandle> make_real_config_from_file(const char* path) {
  try {
    auto handle = std::make_unique<ConfigHandle>();
    auto config = std::make_shared<RealConfig>();
    config->config = std::const_pointer_cast<ocio::Config>(ocio::Config::CreateFromFile(path));
    if (!config->config) return nullptr;
    handle->inner = config;
    return handle;
  } catch (...) { return nullptr; }
}

#endif // OCIO_RS_STUB

}  // namespace ocio_rs_bridge

// =====================================================================
// extern "C" implementations
// =====================================================================

extern "C" {

// --- Runtime ---
bool ocio_runtime_is_stub(void) {
#ifdef OCIO_RS_STUB
  return true;
#else
  return false;
#endif
}

// --- Global utility functions ---
const char* ocio_get_version(void) {
#ifdef OCIO_RS_STUB
  return "stub";
#else
  try { return ocio::GetVersion(); } catch (...) { return nullptr; }
#endif
}

int ocio_get_version_hex(void) {
#ifdef OCIO_RS_STUB
  return 0;
#else
  try { return ocio::GetVersionHex(); } catch (...) { return 0; }
#endif
}

int ocio_get_logging_level(void) {
#ifdef OCIO_RS_STUB
  return 0;
#else
  try { return static_cast<int>(ocio::GetLoggingLevel()); } catch (...) { return 0; }
#endif
}

void ocio_set_logging_level(int level) {
#ifdef OCIO_RS_STUB
  (void)level;
#else
  try { ocio::SetLoggingLevel(static_cast<ocio::LoggingLevel>(level)); } catch (...) {}
#endif
}

// --- Global config ---
void* ocio_get_current_config(void) {
#ifdef OCIO_RS_STUB
  return nullptr;
#else
  try {
    auto cfg = ocio::GetCurrentConfig();
    if (!cfg) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(ocio_rs_bridge::RealConfig{std::const_pointer_cast<ocio::Config>(cfg)});
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_set_current_config(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    ocio::SetCurrentConfig(ocio_rs_bridge::get_real_config(config));
  } catch (...) {}
#endif
}

void ocio_clear_all_caches(void) {
#ifdef OCIO_RS_STUB
#else
  try { ocio::ClearAllCaches(); } catch (...) {}
#endif
}

// --- BuiltinConfigRegistry ---
void* ocio_builtin_config_registry_get(void) {
#ifdef OCIO_RS_STUB
  return nullptr;
#else
  try {
    auto& registry = ocio::BuiltinConfigRegistry::Get();
    auto handle = std::make_unique<ocio_rs_bridge::BuiltinConfigRegistryHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealBuiltinConfigRegistry>(
      ocio_rs_bridge::RealBuiltinConfigRegistry{&registry});
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

size_t ocio_builtin_config_registry_get_num_builtin_configs(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_builtin_config_registry(handle)->getNumBuiltinConfigs();
  } catch (...) { return 0; }
#endif
}

void* ocio_builtin_config_registry_get_builtin_config_name(void* handle, size_t configIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)configIndex;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_builtin_config_registry(handle)->getBuiltinConfigName(configIndex);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_builtin_config_registry_get_builtin_config_ui_name(void* handle, size_t configIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)configIndex;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_builtin_config_registry(handle)->getBuiltinConfigUIName(configIndex);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_builtin_config_registry_get_builtin_config(void* handle, size_t configIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)configIndex;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_builtin_config_registry(handle)->getBuiltinConfig(configIndex);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_builtin_config_registry_get_builtin_config_by_name(void* handle, const char* configName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)configName;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_builtin_config_registry(handle)->getBuiltinConfigByName(configName);
  } catch (...) { return nullptr; }
#endif
}

bool ocio_builtin_config_registry_is_builtin_config_recommended(void* handle, size_t configIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)configIndex;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_builtin_config_registry(handle)->isBuiltinConfigRecommended(configIndex);
  } catch (...) { return false; }
#endif
}


// --- Config ---
void* ocio_config_create_raw(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_config().release();
#else
  auto handle = ocio_rs_bridge::make_real_config_raw();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void* ocio_config_create_from_file(const char* path) {
#ifdef OCIO_RS_STUB
  (void)path; return ocio_rs_bridge::make_stub_config().release();
#else
  auto handle = ocio_rs_bridge::make_real_config_from_file(path);
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_config_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ConfigHandle*>(handle);
}

int ocio_config_get_major_version(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getMajorVersion();
  } catch (...) { return 0; }
#endif
}

void ocio_config_set_major_version(void* handle, unsigned int major) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)major;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setMajorVersion(major);
  } catch (...) { return ; }
#endif
}

int ocio_config_get_minor_version(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getMinorVersion();
  } catch (...) { return 0; }
#endif
}

void ocio_config_set_minor_version(void* handle, unsigned int minor) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)minor;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setMinorVersion(minor);
  } catch (...) { return ; }
#endif
}

void ocio_config_set_version(void* handle, unsigned int major, unsigned int minor) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)major; (void)minor;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setVersion(major, minor);
  } catch (...) { return ; }
#endif
}

void ocio_config_upgrade_to_latest_version(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->upgradeToLatestVersion();
  } catch (...) { return ; }
#endif
}

void ocio_config_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->validate();
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_name(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getName();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_name(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setName(name);
  } catch (...) { return ; }
#endif
}

char ocio_config_get_family_separator(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return '\0';
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getFamilySeparator();
  } catch (...) { return '\0'; }
#endif
}

char ocio_config_get_default_family_separator(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return '\0';
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->GetDefaultFamilySeparator();
  } catch (...) { return '\0'; }
#endif
}

void ocio_config_set_family_separator(void* handle, char separator) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)separator;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setFamilySeparator(separator);
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDescription();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_description(void* handle, const char* description) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)description;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setDescription(description);
  } catch (...) { return ; }
#endif
}

void ocio_config_serialize(void* handle, void* os) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)os;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->serialize(*static_cast<std::ostream*>(os));
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_cache_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getCacheID();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_cache_id_n(void* handle, void* context) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context;
  return nullptr;
#else
  try {
    auto* _context_h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    auto context_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_context_h->inner)->context;
    return (void*)ocio_rs_bridge::get_real_config(handle)->getCacheID(context_ptr);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_current_context(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getCurrentContext();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ContextHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Context>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealContext>(ocio_rs_bridge::RealContext{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_add_environment_var(void* handle, const char* name, const char* defaultValue) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name; (void)defaultValue;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addEnvironmentVar(name, defaultValue);
  } catch (...) { return ; }
#endif
}

int ocio_config_get_num_environment_vars(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumEnvironmentVars();
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_environment_var_name_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getEnvironmentVarNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_environment_var_default(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getEnvironmentVarDefault(name);
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_clear_environment_vars(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearEnvironmentVars();
  } catch (...) { return ; }
#endif
}

void ocio_config_set_environment_mode(void* handle, int mode) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)mode;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setEnvironmentMode(static_cast<ocio::EnvironmentMode>(mode));
  } catch (...) { return ; }
#endif
}

int ocio_config_get_environment_mode(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getEnvironmentMode();
  } catch (...) { return 0; }
#endif
}

void ocio_config_load_environment(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->loadEnvironment();
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_search_path(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getSearchPath();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_search_path(void* handle, const char* path) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)path;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setSearchPath(path);
  } catch (...) { return ; }
#endif
}

int ocio_config_get_num_search_paths(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumSearchPaths();
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_search_path_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getSearchPath(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_clear_search_paths(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearSearchPaths();
  } catch (...) { return ; }
#endif
}

void ocio_config_add_search_path(void* handle, const char* path) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)path;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addSearchPath(path);
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_working_dir(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getWorkingDir();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_working_dir(void* handle, const char* dirname) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dirname;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setWorkingDir(dirname);
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_color_spaces(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getColorSpaces(category);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ColorSpaceSetHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceSet>(ocio_rs_bridge::RealColorSpaceSet{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_num_color_spaces(void* handle, int searchReferenceType, int visibility) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)searchReferenceType; (void)visibility;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumColorSpaces(static_cast<ocio::SearchReferenceSpaceType>(searchReferenceType), static_cast<ocio::ColorSpaceVisibility>(visibility));
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_color_space_name_by_index(void* handle, int searchReferenceType, int visibility, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)searchReferenceType; (void)visibility; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getColorSpaceNameByIndex(static_cast<ocio::SearchReferenceSpaceType>(searchReferenceType), static_cast<ocio::ColorSpaceVisibility>(visibility), index);
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_num_color_spaces_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumColorSpaces();
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_color_space_name_by_index_v1(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getColorSpaceNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_index_for_color_space(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getIndexForColorSpace(name);
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_color_space(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getColorSpace(name);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ColorSpaceHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::ColorSpace>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpace>(ocio_rs_bridge::RealColorSpace{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_canonical_name(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getCanonicalName(name);
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_add_color_space(void* handle, void* cs) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)cs;
  return;
#else
  try {
    auto* _cs_h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(cs);
    auto cs_ptr = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(_cs_h->inner)->colorSpace;
    ocio_rs_bridge::get_real_config(handle)->addColorSpace(cs_ptr);
  } catch (...) { return ; }
#endif
}

void ocio_config_remove_color_space(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeColorSpace(name);
  } catch (...) { return ; }
#endif
}

bool ocio_config_is_color_space_used(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->isColorSpaceUsed(name);
  } catch (...) { return false; }
#endif
}

void ocio_config_clear_color_spaces(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearColorSpaces();
  } catch (...) { return ; }
#endif
}

void ocio_config_set_inactive_color_spaces(void* handle, const char* inactiveColorSpaces) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)inactiveColorSpaces;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setInactiveColorSpaces(inactiveColorSpaces);
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_inactive_color_spaces(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getInactiveColorSpaces();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_config_is_inactive_color_space(void* handle, const char* colorspace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)colorspace;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->isInactiveColorSpace(colorspace);
  } catch (...) { return false; }
#endif
}

bool ocio_config_is_color_space_linear(void* handle, const char* colorSpace, int referenceSpaceType) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)colorSpace; (void)referenceSpaceType;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->isColorSpaceLinear(colorSpace, static_cast<ocio::ReferenceSpaceType>(referenceSpaceType));
  } catch (...) { return false; }
#endif
}

void* ocio_config_identify_builtin_color_space(void* handle, void* srcConfig, void* builtinConfig, const char* builtinColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcConfig; (void)builtinConfig; (void)builtinColorSpaceName;
  return nullptr;
#else
  try {
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto* _builtinConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(builtinConfig);
    auto builtinConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_builtinConfig_h->inner)->config;
    return ocio_rs_bridge::get_real_config(handle)->IdentifyBuiltinColorSpace(srcConfig_ptr, builtinConfig_ptr, builtinColorSpaceName);
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_identify_interchange_space(void* handle, void* srcInterchangeName, void* builtinInterchangeName, void* srcConfig, const char* srcColorSpaceName, void* builtinConfig, const char* builtinColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcInterchangeName; (void)builtinInterchangeName; (void)srcConfig; (void)srcColorSpaceName; (void)builtinConfig; (void)builtinColorSpaceName;
  return;
#else
  try {
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto* _builtinConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(builtinConfig);
    auto builtinConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_builtinConfig_h->inner)->config;
    ocio_rs_bridge::get_real_config(handle)->IdentifyInterchangeSpace(srcInterchangeName, builtinInterchangeName, srcConfig_ptr, srcColorSpaceName, builtinConfig_ptr, builtinColorSpaceName);
  } catch (...) { return ; }
#endif
}

void ocio_config_set_role(void* handle, const char* role, const char* colorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)role; (void)colorSpaceName;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setRole(role, colorSpaceName);
  } catch (...) { return ; }
#endif
}

int ocio_config_get_num_roles(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumRoles();
  } catch (...) { return 0; }
#endif
}

bool ocio_config_has_role(void* handle, const char* role) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)role;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->hasRole(role);
  } catch (...) { return false; }
#endif
}

void* ocio_config_get_role_name(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getRoleName(index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_role_color_space_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getRoleColorSpace(index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_role_color_space_by_name(void* handle, const char* roleName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)roleName;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getRoleColorSpace(roleName);
  } catch (...) { return nullptr; }
#endif
}

bool ocio_config_is_view_shared(void* handle, const char* dispName, const char* viewName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dispName; (void)viewName;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->isViewShared(dispName, viewName);
  } catch (...) { return false; }
#endif
}

void ocio_config_add_shared_view(void* handle, const char* view, const char* viewTransformName, const char* colorSpaceName, const char* looks, const char* ruleName, const char* description) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view; (void)viewTransformName; (void)colorSpaceName; (void)looks; (void)ruleName; (void)description;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addSharedView(view, viewTransformName, colorSpaceName, looks, ruleName, description);
  } catch (...) { return ; }
#endif
}

void ocio_config_remove_shared_view(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeSharedView(view);
  } catch (...) { return ; }
#endif
}

void ocio_config_clear_shared_views(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearSharedViews();
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_default_display(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDefaultDisplay();
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_num_displays(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumDisplays();
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_display(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDisplay(index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_default_view(void* handle, const char* display) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDefaultView(display);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_default_view_v1(void* handle, const char* display, const char* colorspaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)colorspaceName;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDefaultView(display, colorspaceName);
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_num_views(void* handle, const char* display) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumViews(display);
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_view(void* handle, const char* display, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getView(display, index);
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_num_views_v1(void* handle, const char* display, const char* colorspaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)colorspaceName;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumViews(display, colorspaceName);
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_view_v1(void* handle, const char* display, const char* colorspaceName, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)colorspaceName; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getView(display, colorspaceName, index);
  } catch (...) { return nullptr; }
#endif
}

bool ocio_config_are_views_equal(void* handle, void* first, void* second, const char* dispName, const char* viewName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)first; (void)second; (void)dispName; (void)viewName;
  return false;
#else
  try {
    auto* _first_h = static_cast<ocio_rs_bridge::ConfigHandle*>(first);
    auto first_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_first_h->inner)->config;
    auto* _second_h = static_cast<ocio_rs_bridge::ConfigHandle*>(second);
    auto second_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_second_h->inner)->config;
    return ocio_rs_bridge::get_real_config(handle)->AreViewsEqual(first_ptr, second_ptr, dispName, viewName);
  } catch (...) { return false; }
#endif
}

void* ocio_config_get_display_view_transform_name(void* handle, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)view;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDisplayViewTransformName(display, view);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_display_view_color_space_name(void* handle, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)view;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDisplayViewColorSpaceName(display, view);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_display_view_looks(void* handle, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)view;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDisplayViewLooks(display, view);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_display_view_rule(void* handle, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)view;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDisplayViewRule(display, view);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_display_view_description(void* handle, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)view;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDisplayViewDescription(display, view);
  } catch (...) { return nullptr; }
#endif
}

bool ocio_config_has_view(void* handle, const char* dispName, const char* viewName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dispName; (void)viewName;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->hasView(dispName, viewName);
  } catch (...) { return false; }
#endif
}

void ocio_config_add_display_view_v1(void* handle, const char* display, const char* view, const char* colorSpaceName, const char* looks) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)view; (void)colorSpaceName; (void)looks;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addDisplayView(display, view, colorSpaceName, looks);
  } catch (...) { return ; }
#endif
}

void ocio_config_add_display_view_v2(void* handle, const char* display, const char* view, const char* viewTransformName, const char* colorSpaceName, const char* looks, const char* ruleName, const char* description) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)view; (void)viewTransformName; (void)colorSpaceName; (void)looks; (void)ruleName; (void)description;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addDisplayView(display, view, viewTransformName, colorSpaceName, looks, ruleName, description);
  } catch (...) { return ; }
#endif
}

void ocio_config_add_display_shared_view(void* handle, const char* display, const char* sharedView) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)sharedView;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addDisplaySharedView(display, sharedView);
  } catch (...) { return ; }
#endif
}

void ocio_config_remove_display_view(void* handle, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)view;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeDisplayView(display, view);
  } catch (...) { return ; }
#endif
}

void ocio_config_clear_displays(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearDisplays();
  } catch (...) { return ; }
#endif
}

bool ocio_config_has_virtual_view(void* handle, const char* viewName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)viewName;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->hasVirtualView(viewName);
  } catch (...) { return false; }
#endif
}

bool ocio_config_is_virtual_view_shared(void* handle, const char* viewName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)viewName;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->isVirtualViewShared(viewName);
  } catch (...) { return false; }
#endif
}

void ocio_config_add_virtual_display_view(void* handle, const char* view, const char* viewTransformName, const char* colorSpaceName, const char* looks, const char* ruleName, const char* description) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view; (void)viewTransformName; (void)colorSpaceName; (void)looks; (void)ruleName; (void)description;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addVirtualDisplayView(view, viewTransformName, colorSpaceName, looks, ruleName, description);
  } catch (...) { return ; }
#endif
}

void ocio_config_add_virtual_display_shared_view(void* handle, const char* sharedView) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)sharedView;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addVirtualDisplaySharedView(sharedView);
  } catch (...) { return ; }
#endif
}

int ocio_config_get_virtual_display_num_views(void* handle, int type) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getVirtualDisplayNumViews(static_cast<ocio::ViewType>(type));
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_virtual_display_view(void* handle, int type, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getVirtualDisplayView(static_cast<ocio::ViewType>(type), index);
  } catch (...) { return nullptr; }
#endif
}

bool ocio_config_are_virtual_views_equal(void* handle, void* first, void* second, const char* viewName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)first; (void)second; (void)viewName;
  return false;
#else
  try {
    auto* _first_h = static_cast<ocio_rs_bridge::ConfigHandle*>(first);
    auto first_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_first_h->inner)->config;
    auto* _second_h = static_cast<ocio_rs_bridge::ConfigHandle*>(second);
    auto second_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_second_h->inner)->config;
    return ocio_rs_bridge::get_real_config(handle)->AreVirtualViewsEqual(first_ptr, second_ptr, viewName);
  } catch (...) { return false; }
#endif
}

void* ocio_config_get_virtual_display_view_transform_name(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getVirtualDisplayViewTransformName(view);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_virtual_display_view_color_space_name(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getVirtualDisplayViewColorSpaceName(view);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_virtual_display_view_looks(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getVirtualDisplayViewLooks(view);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_virtual_display_view_rule(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getVirtualDisplayViewRule(view);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_virtual_display_view_description(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getVirtualDisplayViewDescription(view);
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_remove_virtual_display_view(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeVirtualDisplayView(view);
  } catch (...) { return ; }
#endif
}

void ocio_config_clear_virtual_display(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearVirtualDisplay();
  } catch (...) { return ; }
#endif
}

int ocio_config_instantiate_display_from_monitor_name(void* handle, const char* monitorName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)monitorName;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->instantiateDisplayFromMonitorName(monitorName);
  } catch (...) { return 0; }
#endif
}

int ocio_config_instantiate_display_from_icc_profile(void* handle, const char* ICCProfileFilepath) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ICCProfileFilepath;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->instantiateDisplayFromICCProfile(ICCProfileFilepath);
  } catch (...) { return 0; }
#endif
}

void ocio_config_set_active_displays(void* handle, const char* displays) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)displays;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setActiveDisplays(displays);
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_active_displays(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getActiveDisplays();
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_num_active_displays(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumActiveDisplays();
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_active_display(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getActiveDisplay(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_add_active_display(void* handle, const char* display) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addActiveDisplay(display);
  } catch (...) { return ; }
#endif
}

void ocio_config_remove_active_display(void* handle, const char* display) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeActiveDisplay(display);
  } catch (...) { return ; }
#endif
}

void ocio_config_clear_active_displays(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearActiveDisplays();
  } catch (...) { return ; }
#endif
}

void ocio_config_set_active_views(void* handle, const char* views) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)views;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setActiveViews(views);
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_active_views(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getActiveViews();
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_num_active_views(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumActiveViews();
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_active_view(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getActiveView(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_add_active_view(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addActiveView(view);
  } catch (...) { return ; }
#endif
}

void ocio_config_remove_active_view(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeActiveView(view);
  } catch (...) { return ; }
#endif
}

void ocio_config_clear_active_views(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearActiveViews();
  } catch (...) { return ; }
#endif
}

int ocio_config_get_num_displays_all(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumDisplaysAll();
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_display_all(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDisplayAll(index);
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_display_all_by_name(void* handle, void* arg) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)arg;
  return 0;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDisplayAllByName(static_cast<const char*>(arg));
  } catch (...) { return 0; }
#endif
}

bool ocio_config_is_display_temporary(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->isDisplayTemporary(index);
  } catch (...) { return false; }
#endif
}

void ocio_config_set_display_temporary(void* handle, int index, bool isTemporary) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)isTemporary;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setDisplayTemporary(index, isTemporary);
  } catch (...) { return ; }
#endif
}

int ocio_config_get_num_views_v2(void* handle, int type, const char* display) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type; (void)display;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumViews(static_cast<ocio::ViewType>(type), display);
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_view_v2(void* handle, int type, const char* display, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type; (void)display; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getView(static_cast<ocio::ViewType>(type), display, index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_viewing_rules(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getViewingRules();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ViewingRulesHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::ViewingRules>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealViewingRules>(ocio_rs_bridge::RealViewingRules{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_viewing_rules(void* handle, void* viewingRules) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)viewingRules;
  return;
#else
  try {
    auto* _viewingRules_h = static_cast<ocio_rs_bridge::ViewingRulesHandle*>(viewingRules);
    auto viewingRules_ptr = std::static_pointer_cast<ocio_rs_bridge::RealViewingRules>(_viewingRules_h->inner)->rules;
    ocio_rs_bridge::get_real_config(handle)->setViewingRules(viewingRules_ptr);
  } catch (...) { return ; }
#endif
}

void ocio_config_get_default_luma_coefs(void* handle, void* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->getDefaultLumaCoefs(rgb);
  } catch (...) { return ; }
#endif
}

void ocio_config_set_default_luma_coefs(void* handle, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setDefaultLumaCoefs(rgb);
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_look(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getLook(name);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::LookHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Look>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealLook>(ocio_rs_bridge::RealLook{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_num_looks(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumLooks();
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_look_name_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getLookNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_add_look(void* handle, void* look) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)look;
  return;
#else
  try {
    auto* _look_h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    auto look_ptr = std::static_pointer_cast<ocio_rs_bridge::RealLook>(_look_h->inner)->look;
    ocio_rs_bridge::get_real_config(handle)->addLook(look_ptr);
  } catch (...) { return ; }
#endif
}

void ocio_config_clear_looks(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearLooks();
  } catch (...) { return ; }
#endif
}

int ocio_config_get_num_view_transforms(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumViewTransforms();
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_view_transform(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getViewTransform(name);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ViewTransformHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::ViewTransform>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealViewTransform>(ocio_rs_bridge::RealViewTransform{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_view_transform_name_by_index(void* handle, int i) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)i;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getViewTransformNameByIndex(i);
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_add_view_transform(void* handle, void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)viewTransform;
  return;
#else
  try {
    auto* _viewTransform_h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    auto viewTransform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(_viewTransform_h->inner)->transform;
    ocio_rs_bridge::get_real_config(handle)->addViewTransform(viewTransform_ptr);
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_default_scene_to_display_view_transform(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getDefaultSceneToDisplayViewTransform();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ViewTransformHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::ViewTransform>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealViewTransform>(ocio_rs_bridge::RealViewTransform{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_default_view_transform_name(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getDefaultViewTransformName();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_default_view_transform_name(void* handle, const char* defaultName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)defaultName;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setDefaultViewTransformName(defaultName);
  } catch (...) { return ; }
#endif
}

void ocio_config_clear_view_transforms(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearViewTransforms();
  } catch (...) { return ; }
#endif
}

int ocio_config_get_num_named_transforms(void* handle, int visibility) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)visibility;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumNamedTransforms(static_cast<ocio::NamedTransformVisibility>(visibility));
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_named_transform_name_by_index(void* handle, int visibility, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)visibility; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getNamedTransformNameByIndex(static_cast<ocio::NamedTransformVisibility>(visibility), index);
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_num_named_transforms_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getNumNamedTransforms();
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_named_transform_name_by_index_v1(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return (void*)ocio_rs_bridge::get_real_config(handle)->getNamedTransformNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_index_for_named_transform(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getIndexForNamedTransform(name);
  } catch (...) { return 0; }
#endif
}

void* ocio_config_get_named_transform(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getNamedTransform(name);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::NamedTransformHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::NamedTransform>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealNamedTransform>(ocio_rs_bridge::RealNamedTransform{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_add_named_transform(void* handle, void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)namedTransform;
  return;
#else
  try {
    auto* _namedTransform_h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    auto namedTransform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(_namedTransform_h->inner)->transform;
    ocio_rs_bridge::get_real_config(handle)->addNamedTransform(namedTransform_ptr);
  } catch (...) { return ; }
#endif
}

void ocio_config_remove_named_transform(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeNamedTransform(name);
  } catch (...) { return ; }
#endif
}

void ocio_config_clear_named_transforms(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearNamedTransforms();
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_file_rules(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getFileRules();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::FileRulesHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::FileRules>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealFileRules>(ocio_rs_bridge::RealFileRules{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_file_rules(void* handle, void* fileRules) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)fileRules;
  return;
#else
  try {
    auto* _fileRules_h = static_cast<ocio_rs_bridge::FileRulesHandle*>(fileRules);
    auto fileRules_ptr = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(_fileRules_h->inner)->rules;
    ocio_rs_bridge::get_real_config(handle)->setFileRules(fileRules_ptr);
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_color_space_from_filepath(void* handle, const char* filePath) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filePath;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getColorSpaceFromFilepath(filePath);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_color_space_from_filepath_by_ref_type(void* handle, const char* filePath, void* ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filePath; (void)ruleIndex;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getColorSpaceFromFilepath(filePath, *static_cast<size_t*>(ruleIndex));
  } catch (...) { return nullptr; }
#endif
}

bool ocio_config_filepath_only_matches_default_rule(void* handle, const char* filePath) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filePath;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->filepathOnlyMatchesDefaultRule(filePath);
  } catch (...) { return false; }
#endif
}

void* ocio_config_parse_color_space_from_string(void* handle, const char* str) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)str;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->parseColorSpaceFromString(str);
  } catch (...) { return nullptr; }
#endif
}

bool ocio_config_is_strict_parsing_enabled(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->isStrictParsingEnabled();
  } catch (...) { return false; }
#endif
}

void ocio_config_set_strict_parsing_enabled(void* handle, bool enabled) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)enabled;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setStrictParsingEnabled(enabled);
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_processor(void* handle, void* context, void* srcColorSpace, void* dstColorSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)srcColorSpace; (void)dstColorSpace;
  return nullptr;
#else
  try {
    auto* _context_h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    auto context_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_context_h->inner)->context;
    auto* _srcColorSpace_h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(srcColorSpace);
    auto srcColorSpace_ptr = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(_srcColorSpace_h->inner)->colorSpace;
    auto* _dstColorSpace_h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(dstColorSpace);
    auto dstColorSpace_ptr = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(_dstColorSpace_h->inner)->colorSpace;
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(context_ptr, srcColorSpace_ptr, dstColorSpace_ptr);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v1(void* handle, void* srcColorSpace, void* dstColorSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcColorSpace; (void)dstColorSpace;
  return nullptr;
#else
  try {
    auto* _srcColorSpace_h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(srcColorSpace);
    auto srcColorSpace_ptr = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(_srcColorSpace_h->inner)->colorSpace;
    auto* _dstColorSpace_h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(dstColorSpace);
    auto dstColorSpace_ptr = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(_dstColorSpace_h->inner)->colorSpace;
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(srcColorSpace_ptr, dstColorSpace_ptr);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v2(void* handle, const char* srcColorSpaceName, const char* dstColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcColorSpaceName; (void)dstColorSpaceName;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(srcColorSpaceName, dstColorSpaceName);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v3(void* handle, void* context, const char* srcColorSpaceName, const char* dstColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)srcColorSpaceName; (void)dstColorSpaceName;
  return nullptr;
#else
  try {
    auto* _context_h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    auto context_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_context_h->inner)->context;
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(context_ptr, srcColorSpaceName, dstColorSpaceName);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v4(void* handle, const char* srcColorSpaceName, const char* display, const char* view, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcColorSpaceName; (void)display; (void)view; (void)direction;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(srcColorSpaceName, display, view, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v5(void* handle, void* context, const char* srcColorSpaceName, const char* display, const char* view, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)srcColorSpaceName; (void)display; (void)view; (void)direction;
  return nullptr;
#else
  try {
    auto* _context_h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    auto context_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_context_h->inner)->context;
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(context_ptr, srcColorSpaceName, display, view, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v6(void* handle, void* namedTransform, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)namedTransform; (void)direction;
  return nullptr;
#else
  try {
    auto* _namedTransform_h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    auto namedTransform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(_namedTransform_h->inner)->transform;
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(namedTransform_ptr, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v7(void* handle, void* context, void* namedTransform, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)namedTransform; (void)direction;
  return nullptr;
#else
  try {
    auto* _context_h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    auto context_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_context_h->inner)->context;
    auto* _namedTransform_h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    auto namedTransform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(_namedTransform_h->inner)->transform;
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(context_ptr, namedTransform_ptr, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v8(void* handle, const char* namedTransformName, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)namedTransformName; (void)direction;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(namedTransformName, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v9(void* handle, void* context, const char* namedTransformName, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)namedTransformName; (void)direction;
  return nullptr;
#else
  try {
    auto* _context_h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    auto context_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_context_h->inner)->context;
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(context_ptr, namedTransformName, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v10(void* handle, void* transform) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform;
  return nullptr;
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandle*>(transform);
    auto transform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealTransform>(_transform_h->inner)->transform;
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(transform_ptr);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v11(void* handle, void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform; (void)direction;
  return nullptr;
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandle*>(transform);
    auto transform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealTransform>(_transform_h->inner)->transform;
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(transform_ptr, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_v12(void* handle, void* context, void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)transform; (void)direction;
  return nullptr;
#else
  try {
    auto* _context_h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    auto context_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_context_h->inner)->context;
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandle*>(transform);
    auto transform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealTransform>(_transform_h->inner)->transform;
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(context_ptr, transform_ptr, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_to_builtin_color_space(void* handle, void* srcConfig, const char* srcColorSpaceName, const char* builtinColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcConfig; (void)srcColorSpaceName; (void)builtinColorSpaceName;
  return nullptr;
#else
  try {
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto result = ocio_rs_bridge::get_real_config(handle)->GetProcessorToBuiltinColorSpace(srcConfig_ptr, srcColorSpaceName, builtinColorSpaceName);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_from_builtin_color_space(void* handle, const char* builtinColorSpaceName, void* srcConfig, const char* srcColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)builtinColorSpaceName; (void)srcConfig; (void)srcColorSpaceName;
  return nullptr;
#else
  try {
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto result = ocio_rs_bridge::get_real_config(handle)->GetProcessorFromBuiltinColorSpace(builtinColorSpaceName, srcConfig_ptr, srcColorSpaceName);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_from_configs(void* handle, void* srcConfig, const char* srcColorSpaceName, void* dstConfig, const char* dstColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcConfig; (void)srcColorSpaceName; (void)dstConfig; (void)dstColorSpaceName;
  return nullptr;
#else
  try {
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto* _dstConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(dstConfig);
    auto dstConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_dstConfig_h->inner)->config;
    auto result = ocio_rs_bridge::get_real_config(handle)->GetProcessorFromConfigs(srcConfig_ptr, srcColorSpaceName, dstConfig_ptr, dstColorSpaceName);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_from_configs_v1(void* handle, void* srcContext, void* srcConfig, const char* srcColorSpaceName, void* dstContext, void* dstConfig, const char* dstColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcContext; (void)srcConfig; (void)srcColorSpaceName; (void)dstContext; (void)dstConfig; (void)dstColorSpaceName;
  return nullptr;
#else
  try {
    auto* _srcContext_h = static_cast<ocio_rs_bridge::ContextHandle*>(srcContext);
    auto srcContext_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_srcContext_h->inner)->context;
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto* _dstContext_h = static_cast<ocio_rs_bridge::ContextHandle*>(dstContext);
    auto dstContext_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_dstContext_h->inner)->context;
    auto* _dstConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(dstConfig);
    auto dstConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_dstConfig_h->inner)->config;
    auto result = ocio_rs_bridge::get_real_config(handle)->GetProcessorFromConfigs(srcContext_ptr, srcConfig_ptr, srcColorSpaceName, dstContext_ptr, dstConfig_ptr, dstColorSpaceName);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_from_configs_v2(void* handle, void* srcConfig, const char* srcColorSpaceName, const char* srcInterchangeName, void* dstConfig, const char* dstColorSpaceName, const char* dstInterchangeName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcConfig; (void)srcColorSpaceName; (void)srcInterchangeName; (void)dstConfig; (void)dstColorSpaceName; (void)dstInterchangeName;
  return nullptr;
#else
  try {
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto* _dstConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(dstConfig);
    auto dstConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_dstConfig_h->inner)->config;
    auto result = ocio_rs_bridge::get_real_config(handle)->GetProcessorFromConfigs(srcConfig_ptr, srcColorSpaceName, srcInterchangeName, dstConfig_ptr, dstColorSpaceName, dstInterchangeName);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_from_configs_v3(void* handle, void* srcContext, void* srcConfig, const char* srcColorSpaceName, const char* srcInterchangeName, void* dstContext, void* dstConfig, const char* dstColorSpaceName, const char* dstInterchangeName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcContext; (void)srcConfig; (void)srcColorSpaceName; (void)srcInterchangeName; (void)dstContext; (void)dstConfig; (void)dstColorSpaceName; (void)dstInterchangeName;
  return nullptr;
#else
  try {
    auto* _srcContext_h = static_cast<ocio_rs_bridge::ContextHandle*>(srcContext);
    auto srcContext_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_srcContext_h->inner)->context;
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto* _dstContext_h = static_cast<ocio_rs_bridge::ContextHandle*>(dstContext);
    auto dstContext_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_dstContext_h->inner)->context;
    auto* _dstConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(dstConfig);
    auto dstConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_dstConfig_h->inner)->config;
    auto result = ocio_rs_bridge::get_real_config(handle)->GetProcessorFromConfigs(srcContext_ptr, srcConfig_ptr, srcColorSpaceName, srcInterchangeName, dstContext_ptr, dstConfig_ptr, dstColorSpaceName, dstInterchangeName);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_from_configs_v4(void* handle, void* srcConfig, const char* srcColorSpaceName, void* dstConfig, const char* dstDisplay, const char* dstView, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcConfig; (void)srcColorSpaceName; (void)dstConfig; (void)dstDisplay; (void)dstView; (void)direction;
  return nullptr;
#else
  try {
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto* _dstConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(dstConfig);
    auto dstConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_dstConfig_h->inner)->config;
    auto result = ocio_rs_bridge::get_real_config(handle)->GetProcessorFromConfigs(srcConfig_ptr, srcColorSpaceName, dstConfig_ptr, dstDisplay, dstView, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_from_configs_v5(void* handle, void* srcContext, void* srcConfig, const char* srcColorSpaceName, void* dstContext, void* dstConfig, const char* dstDisplay, const char* dstView, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcContext; (void)srcConfig; (void)srcColorSpaceName; (void)dstContext; (void)dstConfig; (void)dstDisplay; (void)dstView; (void)direction;
  return nullptr;
#else
  try {
    auto* _srcContext_h = static_cast<ocio_rs_bridge::ContextHandle*>(srcContext);
    auto srcContext_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_srcContext_h->inner)->context;
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto* _dstContext_h = static_cast<ocio_rs_bridge::ContextHandle*>(dstContext);
    auto dstContext_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_dstContext_h->inner)->context;
    auto* _dstConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(dstConfig);
    auto dstConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_dstConfig_h->inner)->config;
    auto result = ocio_rs_bridge::get_real_config(handle)->GetProcessorFromConfigs(srcContext_ptr, srcConfig_ptr, srcColorSpaceName, dstContext_ptr, dstConfig_ptr, dstDisplay, dstView, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_from_configs_v6(void* handle, void* srcConfig, const char* srcColorSpaceName, const char* srcInterchangeName, void* dstConfig, const char* dstDisplay, const char* dstView, const char* dstInterchangeName, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcConfig; (void)srcColorSpaceName; (void)srcInterchangeName; (void)dstConfig; (void)dstDisplay; (void)dstView; (void)dstInterchangeName; (void)direction;
  return nullptr;
#else
  try {
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto* _dstConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(dstConfig);
    auto dstConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_dstConfig_h->inner)->config;
    auto result = ocio_rs_bridge::get_real_config(handle)->GetProcessorFromConfigs(srcConfig_ptr, srcColorSpaceName, srcInterchangeName, dstConfig_ptr, dstDisplay, dstView, dstInterchangeName, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_from_configs_v7(void* handle, void* srcContext, void* srcConfig, const char* srcColorSpaceName, const char* srcInterchangeName, void* dstContext, void* dstConfig, const char* dstDisplay, const char* dstView, const char* dstInterchangeName, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcContext; (void)srcConfig; (void)srcColorSpaceName; (void)srcInterchangeName; (void)dstContext; (void)dstConfig; (void)dstDisplay; (void)dstView; (void)dstInterchangeName; (void)direction;
  return nullptr;
#else
  try {
    auto* _srcContext_h = static_cast<ocio_rs_bridge::ContextHandle*>(srcContext);
    auto srcContext_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_srcContext_h->inner)->context;
    auto* _srcConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(srcConfig);
    auto srcConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_srcConfig_h->inner)->config;
    auto* _dstContext_h = static_cast<ocio_rs_bridge::ContextHandle*>(dstContext);
    auto dstContext_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_dstContext_h->inner)->context;
    auto* _dstConfig_h = static_cast<ocio_rs_bridge::ConfigHandle*>(dstConfig);
    auto dstConfig_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_dstConfig_h->inner)->config;
    auto result = ocio_rs_bridge::get_real_config(handle)->GetProcessorFromConfigs(srcContext_ptr, srcConfig_ptr, srcColorSpaceName, srcInterchangeName, dstContext_ptr, dstConfig_ptr, dstDisplay, dstView, dstInterchangeName, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_processor_cache_flags(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->getProcessorCacheFlags();
  } catch (...) { return 0; }
#endif
}

void ocio_config_set_processor_cache_flags(void* handle, int flags) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)flags;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setProcessorCacheFlags(static_cast<ocio::ProcessorCacheFlags>(flags));
  } catch (...) { return ; }
#endif
}

void ocio_config_clear_processor_cache(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearProcessorCache();
  } catch (...) { return ; }
#endif
}

void ocio_config_set_config_io_proxy(void* handle, void* ciop) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ciop;
  return;
#else
  try {
    auto* _ciop_h = static_cast<ocio_rs_bridge::ConfigIOProxyHandle*>(ciop);
    auto ciop_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfigIOProxy>(_ciop_h->inner)->proxy;
    ocio_rs_bridge::get_real_config(handle)->setConfigIOProxy(ciop_ptr);
  } catch (...) { return ; }
#endif
}

void* ocio_config_get_config_io_proxy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getConfigIOProxy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ConfigIOProxyHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealConfigIOProxy>(ocio_rs_bridge::RealConfigIOProxy{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_config_is_archivable(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(handle)->isArchivable();
  } catch (...) { return false; }
#endif
}

void ocio_config_archive(void* handle, void* ostream) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ostream;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->archive(*static_cast<std::ostream*>(ostream));
  } catch (...) { return ; }
#endif
}


// --- FileRules ---

void* ocio_file_rules_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_file_rules().release();
#else
  auto handle = ocio_rs_bridge::make_real_file_rules();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_file_rules_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::FileRulesHandle*>(handle);
}

size_t ocio_file_rules_get_num_entries(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_file_rules(handle)->getNumEntries();
  } catch (...) { return 0; }
#endif
}

size_t ocio_file_rules_get_index_for_rule(void* handle, const char* ruleName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleName;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_file_rules(handle)->getIndexForRule(ruleName);
  } catch (...) { return 0; }
#endif
}

void* ocio_file_rules_get_name(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_file_rules(handle)->getName(ruleIndex);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_file_rules_get_pattern(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_file_rules(handle)->getPattern(ruleIndex);
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_rules_set_pattern(void* handle, size_t ruleIndex, const char* pattern) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)pattern;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->setPattern(ruleIndex, pattern);
  } catch (...) { return ; }
#endif
}

void* ocio_file_rules_get_extension(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_file_rules(handle)->getExtension(ruleIndex);
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_rules_set_extension(void* handle, size_t ruleIndex, const char* extension) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)extension;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->setExtension(ruleIndex, extension);
  } catch (...) { return ; }
#endif
}

void* ocio_file_rules_get_regex(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_file_rules(handle)->getRegex(ruleIndex);
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_rules_set_regex(void* handle, size_t ruleIndex, const char* regex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)regex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->setRegex(ruleIndex, regex);
  } catch (...) { return ; }
#endif
}

void* ocio_file_rules_get_color_space(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_file_rules(handle)->getColorSpace(ruleIndex);
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_rules_set_color_space(void* handle, size_t ruleIndex, const char* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)colorSpace;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->setColorSpace(ruleIndex, colorSpace);
  } catch (...) { return ; }
#endif
}

size_t ocio_file_rules_get_num_custom_keys(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_file_rules(handle)->getNumCustomKeys(ruleIndex);
  } catch (...) { return 0; }
#endif
}

void* ocio_file_rules_get_custom_key_name(void* handle, size_t ruleIndex, size_t key) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)key;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_file_rules(handle)->getCustomKeyName(ruleIndex, key);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_file_rules_get_custom_key_value(void* handle, size_t ruleIndex, size_t key) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)key;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_file_rules(handle)->getCustomKeyValue(ruleIndex, key);
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_rules_set_custom_key(void* handle, size_t ruleIndex, const char* key, const char* value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)key; (void)value;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->setCustomKey(ruleIndex, key, value);
  } catch (...) { return ; }
#endif
}

void ocio_file_rules_insert_rule(void* handle, size_t ruleIndex, const char* name, const char* colorSpace, const char* pattern, const char* extension) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)name; (void)colorSpace; (void)pattern; (void)extension;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->insertRule(ruleIndex, name, colorSpace, pattern, extension);
  } catch (...) { return ; }
#endif
}

void ocio_file_rules_insert_rule_v1(void* handle, size_t ruleIndex, const char* name, const char* colorSpace, const char* regex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)name; (void)colorSpace; (void)regex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->insertRule(ruleIndex, name, colorSpace, regex);
  } catch (...) { return ; }
#endif
}

void ocio_file_rules_insert_path_search_rule(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->insertPathSearchRule(ruleIndex);
  } catch (...) { return ; }
#endif
}

void ocio_file_rules_set_default_rule_color_space(void* handle, const char* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)colorSpace;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->setDefaultRuleColorSpace(colorSpace);
  } catch (...) { return ; }
#endif
}

void ocio_file_rules_remove_rule(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->removeRule(ruleIndex);
  } catch (...) { return ; }
#endif
}

void ocio_file_rules_increase_rule_priority(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->increaseRulePriority(ruleIndex);
  } catch (...) { return ; }
#endif
}

void ocio_file_rules_decrease_rule_priority(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->decreaseRulePriority(ruleIndex);
  } catch (...) { return ; }
#endif
}

bool ocio_file_rules_is_default(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_file_rules(handle)->isDefault();
  } catch (...) { return false; }
#endif
}


// --- ColorSpace ---

void* ocio_color_space_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_color_space().release();
#else
  auto handle = ocio_rs_bridge::make_real_color_space();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_color_space_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ColorSpaceHandle*>(handle);
}

void* ocio_color_space_get_name(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getName();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_name(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setName(name);
  } catch (...) { return ; }
#endif
}

size_t ocio_color_space_get_num_aliases(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getNumAliases();
  } catch (...) { return 0; }
#endif
}

void* ocio_color_space_get_alias(void* handle, size_t idx) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)idx;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getAlias(idx);
  } catch (...) { return nullptr; }
#endif
}

bool ocio_color_space_has_alias(void* handle, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)alias;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->hasAlias(alias);
  } catch (...) { return false; }
#endif
}

void ocio_color_space_add_alias(void* handle, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)alias;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->addAlias(alias);
  } catch (...) { return ; }
#endif
}

void ocio_color_space_remove_alias(void* handle, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)alias;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->removeAlias(alias);
  } catch (...) { return ; }
#endif
}

void ocio_color_space_clear_aliases(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->clearAliases();
  } catch (...) { return ; }
#endif
}

void* ocio_color_space_get_family(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getFamily();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_family(void* handle, const char* family) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)family;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setFamily(family);
  } catch (...) { return ; }
#endif
}

void* ocio_color_space_get_equality_group(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getEqualityGroup();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_equality_group(void* handle, const char* equalityGroup) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)equalityGroup;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setEqualityGroup(equalityGroup);
  } catch (...) { return ; }
#endif
}

void* ocio_color_space_get_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getDescription();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_description(void* handle, const char* description) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)description;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setDescription(description);
  } catch (...) { return ; }
#endif
}

void* ocio_color_space_get_interop_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getInteropID();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_interop_id(void* handle, const char* interopID) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)interopID;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setInteropID(interopID);
  } catch (...) { return ; }
#endif
}

void ocio_color_space_set_interchange_attribute(void* handle, const char* attrName, void* value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)attrName; (void)value;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setInterchangeAttribute(attrName, value);
  } catch (...) { return ; }
#endif
}

int ocio_color_space_get_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getBitDepth();
  } catch (...) { return 0; }
#endif
}

void ocio_color_space_set_bit_depth(void* handle, int bitDepth) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)bitDepth;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setBitDepth(static_cast<ocio::BitDepth>(bitDepth));
  } catch (...) { return ; }
#endif
}

int ocio_color_space_get_reference_space_type(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getReferenceSpaceType();
  } catch (...) { return 0; }
#endif
}

bool ocio_color_space_has_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->hasCategory(category);
  } catch (...) { return false; }
#endif
}

void ocio_color_space_add_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->addCategory(category);
  } catch (...) { return ; }
#endif
}

void ocio_color_space_remove_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->removeCategory(category);
  } catch (...) { return ; }
#endif
}

int ocio_color_space_get_num_categories(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getNumCategories();
  } catch (...) { return 0; }
#endif
}

void* ocio_color_space_get_category(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getCategory(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_clear_categories(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->clearCategories();
  } catch (...) { return ; }
#endif
}

void* ocio_color_space_get_encoding(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getEncoding();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_encoding(void* handle, const char* encoding) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)encoding;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setEncoding(encoding);
  } catch (...) { return ; }
#endif
}

bool ocio_color_space_is_data(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->isData();
  } catch (...) { return false; }
#endif
}

void ocio_color_space_set_is_data(void* handle, bool isData) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)isData;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setIsData(isData);
  } catch (...) { return ; }
#endif
}

int ocio_color_space_get_allocation(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getAllocation();
  } catch (...) { return 0; }
#endif
}

void ocio_color_space_set_allocation(void* handle, int allocation) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)allocation;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setAllocation(static_cast<ocio::Allocation>(allocation));
  } catch (...) { return ; }
#endif
}

int ocio_color_space_get_allocation_num_vars(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getAllocationNumVars();
  } catch (...) { return 0; }
#endif
}

void ocio_color_space_get_allocation_vars(void* handle, void* vars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)vars;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->getAllocationVars(vars);
  } catch (...) { return ; }
#endif
}

void ocio_color_space_set_allocation_vars(void* handle, int numvars, const float* vars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)numvars; (void)vars;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setAllocationVars(numvars, vars);
  } catch (...) { return ; }
#endif
}

void* ocio_color_space_get_transform(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_color_space(handle)->getTransform(static_cast<ocio::ColorSpaceDirection>(dir));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::TransformHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Transform>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealTransform>(ocio_rs_bridge::RealTransform{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_transform(void* handle, void* transform, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform; (void)dir;
  return;
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandle*>(transform);
    auto transform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealTransform>(_transform_h->inner)->transform;
    ocio_rs_bridge::get_real_color_space(handle)->setTransform(transform_ptr, static_cast<ocio::ColorSpaceDirection>(dir));
  } catch (...) { return ; }
#endif
}


// --- ColorSpaceSet ---

void* ocio_color_space_set_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_color_space_set().release();
#else
  auto handle = ocio_rs_bridge::make_real_color_space_set();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_color_space_set_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(handle);
}

int ocio_color_space_set_get_num_color_spaces(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_color_space_set(handle)->getNumColorSpaces();
  } catch (...) { return 0; }
#endif
}

void* ocio_color_space_set_get_color_space_name_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space_set(handle)->getColorSpaceNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_color_space_set_get_color_space_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_color_space_set(handle)->getColorSpaceByIndex(index);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ColorSpaceHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::ColorSpace>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpace>(ocio_rs_bridge::RealColorSpace{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_color_space_set_get_color_space(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_color_space_set(handle)->getColorSpace(name);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ColorSpaceHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::ColorSpace>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpace>(ocio_rs_bridge::RealColorSpace{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_color_space_set_get_color_space_index(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_color_space_set(handle)->getColorSpaceIndex(name);
  } catch (...) { return 0; }
#endif
}

bool ocio_color_space_set_has_color_space(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_color_space_set(handle)->hasColorSpace(name);
  } catch (...) { return false; }
#endif
}

void ocio_color_space_set_add_color_space(void* handle, void* cs) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)cs;
  return;
#else
  try {
    auto* _cs_h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(cs);
    auto cs_ptr = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(_cs_h->inner)->colorSpace;
    ocio_rs_bridge::get_real_color_space_set(handle)->addColorSpace(cs_ptr);
  } catch (...) { return ; }
#endif
}

void ocio_color_space_set_add_color_spaces(void* handle, void* cs) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)cs;
  return;
#else
  try {
    auto* _cs_h = static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(cs);
    auto cs_ptr = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceSet>(_cs_h->inner)->set;
    ocio_rs_bridge::get_real_color_space_set(handle)->addColorSpaces(cs_ptr);
  } catch (...) { return ; }
#endif
}

void ocio_color_space_set_remove_color_space(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space_set(handle)->removeColorSpace(name);
  } catch (...) { return ; }
#endif
}

void ocio_color_space_set_remove_color_spaces(void* handle, void* cs) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)cs;
  return;
#else
  try {
    auto* _cs_h = static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(cs);
    auto cs_ptr = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceSet>(_cs_h->inner)->set;
    ocio_rs_bridge::get_real_color_space_set(handle)->removeColorSpaces(cs_ptr);
  } catch (...) { return ; }
#endif
}

void ocio_color_space_set_clear_color_spaces(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space_set(handle)->clearColorSpaces();
  } catch (...) { return ; }
#endif
}


// --- Look ---

void* ocio_look_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_look().release();
#else
  auto handle = ocio_rs_bridge::make_real_look();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_look_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LookHandle*>(handle);
}

void* ocio_look_get_name(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_look(handle)->getName();
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_set_name(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_look(handle)->setName(name);
  } catch (...) { return ; }
#endif
}

void* ocio_look_get_process_space(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_look(handle)->getProcessSpace();
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_set_process_space(void* handle, const char* processSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)processSpace;
  return;
#else
  try {
    ocio_rs_bridge::get_real_look(handle)->setProcessSpace(processSpace);
  } catch (...) { return ; }
#endif
}

void* ocio_look_get_transform(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_look(handle)->getTransform();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::TransformHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Transform>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealTransform>(ocio_rs_bridge::RealTransform{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_set_transform(void* handle, void* transform) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform;
  return;
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandle*>(transform);
    auto transform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealTransform>(_transform_h->inner)->transform;
    ocio_rs_bridge::get_real_look(handle)->setTransform(transform_ptr);
  } catch (...) { return ; }
#endif
}

void* ocio_look_get_inverse_transform(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_look(handle)->getInverseTransform();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::TransformHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Transform>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealTransform>(ocio_rs_bridge::RealTransform{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_set_inverse_transform(void* handle, void* transform) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform;
  return;
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandle*>(transform);
    auto transform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealTransform>(_transform_h->inner)->transform;
    ocio_rs_bridge::get_real_look(handle)->setInverseTransform(transform_ptr);
  } catch (...) { return ; }
#endif
}

void* ocio_look_get_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_look(handle)->getDescription();
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_set_description(void* handle, const char* description) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)description;
  return;
#else
  try {
    ocio_rs_bridge::get_real_look(handle)->setDescription(description);
  } catch (...) { return ; }
#endif
}

void ocio_look_set_interchange_attribute(void* handle, const char* attrName, void* value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)attrName; (void)value;
  return;
#else
  try {
    ocio_rs_bridge::get_real_look(handle)->setInterchangeAttribute(attrName, value);
  } catch (...) { return ; }
#endif
}


// --- NamedTransform ---

void* ocio_named_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_named_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_named_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_named_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::NamedTransformHandle*>(handle);
}

void* ocio_named_transform_get_name(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_named_transform(handle)->getName();
  } catch (...) { return nullptr; }
#endif
}

void ocio_named_transform_set_name(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->setName(name);
  } catch (...) { return ; }
#endif
}

size_t ocio_named_transform_get_num_aliases(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_named_transform(handle)->getNumAliases();
  } catch (...) { return 0; }
#endif
}

void* ocio_named_transform_get_alias(void* handle, size_t idx) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)idx;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_named_transform(handle)->getAlias(idx);
  } catch (...) { return nullptr; }
#endif
}

bool ocio_named_transform_has_alias(void* handle, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)alias;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_named_transform(handle)->hasAlias(alias);
  } catch (...) { return false; }
#endif
}

void ocio_named_transform_add_alias(void* handle, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)alias;
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->addAlias(alias);
  } catch (...) { return ; }
#endif
}

void ocio_named_transform_remove_alias(void* handle, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)alias;
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->removeAlias(alias);
  } catch (...) { return ; }
#endif
}

void ocio_named_transform_clear_aliases(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->clearAliases();
  } catch (...) { return ; }
#endif
}

void* ocio_named_transform_get_family(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_named_transform(handle)->getFamily();
  } catch (...) { return nullptr; }
#endif
}

void ocio_named_transform_set_family(void* handle, const char* family) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)family;
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->setFamily(family);
  } catch (...) { return ; }
#endif
}

void* ocio_named_transform_get_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_named_transform(handle)->getDescription();
  } catch (...) { return nullptr; }
#endif
}

void ocio_named_transform_set_description(void* handle, const char* description) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)description;
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->setDescription(description);
  } catch (...) { return ; }
#endif
}

bool ocio_named_transform_has_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_named_transform(handle)->hasCategory(category);
  } catch (...) { return false; }
#endif
}

void ocio_named_transform_add_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->addCategory(category);
  } catch (...) { return ; }
#endif
}

void ocio_named_transform_remove_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->removeCategory(category);
  } catch (...) { return ; }
#endif
}

int ocio_named_transform_get_num_categories(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_named_transform(handle)->getNumCategories();
  } catch (...) { return 0; }
#endif
}

void* ocio_named_transform_get_category(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_named_transform(handle)->getCategory(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_named_transform_clear_categories(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->clearCategories();
  } catch (...) { return ; }
#endif
}

void* ocio_named_transform_get_encoding(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_named_transform(handle)->getEncoding();
  } catch (...) { return nullptr; }
#endif
}

void ocio_named_transform_set_encoding(void* handle, const char* encoding) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)encoding;
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->setEncoding(encoding);
  } catch (...) { return ; }
#endif
}

void* ocio_named_transform_get_transform(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_named_transform(handle)->getTransform(static_cast<ocio::TransformDirection>(dir));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::TransformHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Transform>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealTransform>(ocio_rs_bridge::RealTransform{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_named_transform_set_transform(void* handle, void* transform, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform; (void)dir;
  return;
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandle*>(transform);
    auto transform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealTransform>(_transform_h->inner)->transform;
    ocio_rs_bridge::get_real_named_transform(handle)->setTransform(transform_ptr, static_cast<ocio::TransformDirection>(dir));
  } catch (...) { return ; }
#endif
}


// --- ViewTransform ---

void* ocio_view_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_view_transform().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_view_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ViewTransformHandle*>(handle);
}

void* ocio_view_transform_get_name(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_view_transform(handle)->getName();
  } catch (...) { return nullptr; }
#endif
}

void ocio_view_transform_set_name(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_view_transform(handle)->setName(name);
  } catch (...) { return ; }
#endif
}

void* ocio_view_transform_get_family(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_view_transform(handle)->getFamily();
  } catch (...) { return nullptr; }
#endif
}

void ocio_view_transform_set_family(void* handle, const char* family) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)family;
  return;
#else
  try {
    ocio_rs_bridge::get_real_view_transform(handle)->setFamily(family);
  } catch (...) { return ; }
#endif
}

void* ocio_view_transform_get_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_view_transform(handle)->getDescription();
  } catch (...) { return nullptr; }
#endif
}

void ocio_view_transform_set_description(void* handle, const char* description) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)description;
  return;
#else
  try {
    ocio_rs_bridge::get_real_view_transform(handle)->setDescription(description);
  } catch (...) { return ; }
#endif
}

void ocio_view_transform_set_interchange_attribute(void* handle, const char* attrName, void* value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)attrName; (void)value;
  return;
#else
  try {
    ocio_rs_bridge::get_real_view_transform(handle)->setInterchangeAttribute(attrName, value);
  } catch (...) { return ; }
#endif
}

bool ocio_view_transform_has_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_view_transform(handle)->hasCategory(category);
  } catch (...) { return false; }
#endif
}

void ocio_view_transform_add_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return;
#else
  try {
    ocio_rs_bridge::get_real_view_transform(handle)->addCategory(category);
  } catch (...) { return ; }
#endif
}

void ocio_view_transform_remove_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return;
#else
  try {
    ocio_rs_bridge::get_real_view_transform(handle)->removeCategory(category);
  } catch (...) { return ; }
#endif
}

int ocio_view_transform_get_num_categories(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_view_transform(handle)->getNumCategories();
  } catch (...) { return 0; }
#endif
}

void* ocio_view_transform_get_category(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_view_transform(handle)->getCategory(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_view_transform_clear_categories(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_view_transform(handle)->clearCategories();
  } catch (...) { return ; }
#endif
}

int ocio_view_transform_get_reference_space_type(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_view_transform(handle)->getReferenceSpaceType();
  } catch (...) { return 0; }
#endif
}

void* ocio_view_transform_get_transform(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_view_transform(handle)->getTransform(static_cast<ocio::ViewTransformDirection>(dir));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::TransformHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Transform>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealTransform>(ocio_rs_bridge::RealTransform{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_view_transform_set_transform(void* handle, void* transform, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform; (void)dir;
  return;
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandle*>(transform);
    auto transform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealTransform>(_transform_h->inner)->transform;
    ocio_rs_bridge::get_real_view_transform(handle)->setTransform(transform_ptr, static_cast<ocio::ViewTransformDirection>(dir));
  } catch (...) { return ; }
#endif
}


// --- Processor ---

void* ocio_processor_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_processor().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_processor_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ProcessorHandle*>(handle);
}

bool ocio_processor_is_no_op(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_processor(handle)->isNoOp();
  } catch (...) { return false; }
#endif
}

bool ocio_processor_has_channel_crosstalk(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_processor(handle)->hasChannelCrosstalk();
  } catch (...) { return false; }
#endif
}

void* ocio_processor_get_cache_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_processor(handle)->getCacheID();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_processor_metadata(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getProcessorMetadata();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorMetadataHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::ProcessorMetadata>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessorMetadata>(ocio_rs_bridge::RealProcessorMetadata{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_format_metadata(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_processor(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

int ocio_processor_get_num_transforms(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_processor(handle)->getNumTransforms();
  } catch (...) { return 0; }
#endif
}

void* ocio_processor_get_transform_format_metadata(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_processor(handle)->getTransformFormatMetadata(index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_create_group_transform(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->createGroupTransform();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::GroupTransformHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGroupTransform>(ocio_rs_bridge::RealGroupTransform{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_dynamic_property(void* handle, int type) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getDynamicProperty(static_cast<ocio::DynamicPropertyType>(type));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::DynamicPropertyHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealDynamicProperty>(ocio_rs_bridge::RealDynamicProperty{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_processor_has_dynamic_property(void* handle, int type) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_processor(handle)->hasDynamicProperty(static_cast<ocio::DynamicPropertyType>(type));
  } catch (...) { return false; }
#endif
}

bool ocio_processor_is_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_processor(handle)->isDynamic();
  } catch (...) { return false; }
#endif
}

void* ocio_processor_get_optimized_processor_v1(void* handle, int oFlags) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)oFlags;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getOptimizedProcessor(static_cast<ocio::OptimizationFlags>(oFlags));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_optimized_processor_v2(void* handle, int inBD, int outBD, int oFlags) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)inBD; (void)outBD; (void)oFlags;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getOptimizedProcessor(static_cast<ocio::BitDepth>(inBD), static_cast<ocio::BitDepth>(outBD), static_cast<ocio::OptimizationFlags>(oFlags));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_default_gpu_processor(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getDefaultGPUProcessor();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::GPUProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::GPUProcessor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGPUProcessor>(ocio_rs_bridge::RealGPUProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_optimized_gpu_processor(void* handle, int oFlags) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)oFlags;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getOptimizedGPUProcessor(static_cast<ocio::OptimizationFlags>(oFlags));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::GPUProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::GPUProcessor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGPUProcessor>(ocio_rs_bridge::RealGPUProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_optimized_legacy_gpu_processor(void* handle, int oFlags, void* edgelen) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)oFlags; (void)edgelen;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getOptimizedLegacyGPUProcessor(static_cast<ocio::OptimizationFlags>(oFlags), edgelen);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::GPUProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::GPUProcessor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGPUProcessor>(ocio_rs_bridge::RealGPUProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_default_cpu_processor(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getDefaultCPUProcessor();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::CPUProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::CPUProcessor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealCPUProcessor>(ocio_rs_bridge::RealCPUProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_optimized_cpu_processor(void* handle, int oFlags) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)oFlags;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getOptimizedCPUProcessor(static_cast<ocio::OptimizationFlags>(oFlags));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::CPUProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::CPUProcessor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealCPUProcessor>(ocio_rs_bridge::RealCPUProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_optimized_cpu_processor_v1(void* handle, int inBitDepth, int outBitDepth, int oFlags) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)inBitDepth; (void)outBitDepth; (void)oFlags;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getOptimizedCPUProcessor(static_cast<ocio::BitDepth>(inBitDepth), static_cast<ocio::BitDepth>(outBitDepth), static_cast<ocio::OptimizationFlags>(oFlags));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::CPUProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::CPUProcessor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealCPUProcessor>(ocio_rs_bridge::RealCPUProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}


// --- CPUProcessor ---

void* ocio_cpu_processor_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_cpu_processor().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_cpu_processor_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::CPUProcessorHandle*>(handle);
}

bool ocio_cpu_processor_is_no_op(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->isNoOp();
  } catch (...) { return false; }
#endif
}

bool ocio_cpu_processor_is_identity(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->isIdentity();
  } catch (...) { return false; }
#endif
}

bool ocio_cpu_processor_has_channel_crosstalk(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->hasChannelCrosstalk();
  } catch (...) { return false; }
#endif
}

void* ocio_cpu_processor_get_cache_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->getCacheID();
  } catch (...) { return nullptr; }
#endif
}

int ocio_cpu_processor_get_input_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->getInputBitDepth();
  } catch (...) { return 0; }
#endif
}

int ocio_cpu_processor_get_output_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->getOutputBitDepth();
  } catch (...) { return 0; }
#endif
}

void* ocio_cpu_processor_get_dynamic_property(void* handle, int type) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_cpu_processor(handle)->getDynamicProperty(static_cast<ocio::DynamicPropertyType>(type));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::DynamicPropertyHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealDynamicProperty>(ocio_rs_bridge::RealDynamicProperty{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_cpu_processor_has_dynamic_property(void* handle, int type) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->hasDynamicProperty(static_cast<ocio::DynamicPropertyType>(type));
  } catch (...) { return false; }
#endif
}

bool ocio_cpu_processor_is_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->isDynamic();
  } catch (...) { return false; }
#endif
}

void ocio_cpu_processor_apply_v1(void* handle, void* imgDesc) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)imgDesc;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cpu_processor(handle)->apply(*static_cast<const ocio::ImageDesc*>(imgDesc));
  } catch (...) { return ; }
#endif
}

void ocio_cpu_processor_apply_v2(void* handle, void* srcImgDesc, void* dstImgDesc) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcImgDesc; (void)dstImgDesc;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cpu_processor(handle)->apply(*static_cast<const ocio::ImageDesc*>(srcImgDesc), *static_cast<ocio::ImageDesc*>(dstImgDesc));
  } catch (...) { return ; }
#endif
}

void ocio_cpu_processor_apply_rgb(void* handle, void* pixel) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)pixel;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cpu_processor(handle)->applyRGB(pixel);
  } catch (...) { return ; }
#endif
}

void ocio_cpu_processor_apply_rgba(void* handle, void* pixel) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)pixel;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cpu_processor(handle)->applyRGBA(pixel);
  } catch (...) { return ; }
#endif
}


// --- GPUProcessor ---

void* ocio_gpu_processor_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_gpu_processor().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_gpu_processor_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GPUProcessorHandle*>(handle);
}

bool ocio_gpu_processor_is_no_op(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_processor(handle)->isNoOp();
  } catch (...) { return false; }
#endif
}

bool ocio_gpu_processor_has_channel_crosstalk(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_processor(handle)->hasChannelCrosstalk();
  } catch (...) { return false; }
#endif
}

void* ocio_gpu_processor_get_cache_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_processor(handle)->getCacheID();
  } catch (...) { return nullptr; }
#endif
}

void ocio_gpu_processor_extract_gpu_shader_info_v1(void* handle, void* shaderDesc) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shaderDesc;
  return;
#else
  try {
    auto* _shaderDesc_h = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shaderDesc);
    auto shaderDesc_ptr = std::static_pointer_cast<ocio_rs_bridge::RealGpuShaderDesc>(_shaderDesc_h->inner)->gpuShaderDesc;
    ocio_rs_bridge::get_real_gpu_processor(handle)->extractGpuShaderInfo(shaderDesc_ptr);
  } catch (...) { return ; }
#endif
}

void ocio_gpu_processor_extract_gpu_shader_info_v2(void* handle, void* shaderCreator) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shaderCreator;
  return;
#else
  try {
    auto* _shaderCreator_h = static_cast<ocio_rs_bridge::GpuShaderCreatorHandle*>(shaderCreator);
    auto shaderCreator_ptr = std::static_pointer_cast<ocio_rs_bridge::RealGpuShaderCreator>(_shaderCreator_h->inner)->shader;
    ocio_rs_bridge::get_real_gpu_processor(handle)->extractGpuShaderInfo(shaderCreator_ptr);
  } catch (...) { return ; }
#endif
}


// --- GpuShaderDesc ---

void* ocio_gpu_shader_desc_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_gpu_shader_desc().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_gpu_shader_desc_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(handle);
}

void* ocio_gpu_shader_desc_clone(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_gpu_shader_desc(handle)->clone();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::GpuShaderCreatorHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGpuShaderCreator>(ocio_rs_bridge::RealGpuShaderCreator{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_num_uniforms(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getNumUniforms();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_uniform(void* handle, void* index, void* data) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)data;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getUniform(index, *static_cast<ocio::GpuShaderDesc::UniformData*>(data));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_uniform_buffer_size(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getUniformBufferSize();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_num_textures(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getNumTextures();
  } catch (...) { return nullptr; }
#endif
}

void ocio_gpu_shader_desc_get_texture(void* handle, void* index, const char* textureName, const char* samplerName, void* width, void* height, void* channel, void* dimensions, void* interpolation) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)textureName; (void)samplerName; (void)width; (void)height; (void)channel; (void)dimensions; (void)interpolation;
  return;
#else
  try {
    ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getTexture(index, textureName, samplerName, *static_cast<unsigned*>(width), *static_cast<unsigned*>(height), *static_cast<ocio::GpuShaderCreator::TextureType*>(channel), *static_cast<ocio::GpuShaderDesc::TextureDimensions*>(dimensions), *static_cast<ocio::Interpolation*>(interpolation));
  } catch (...) { return ; }
#endif
}

void ocio_gpu_shader_desc_get_texture_values(void* handle, void* index, const float* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)values;
  return;
#else
  try {
    ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getTextureValues(index, values);
  } catch (...) { return ; }
#endif
}

void* ocio_gpu_shader_desc_get_texture_shader_binding_index(void* handle, void* index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getTextureShaderBindingIndex(index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_num3d_textures(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getNum3DTextures();
  } catch (...) { return nullptr; }
#endif
}

void ocio_gpu_shader_desc_get3d_texture(void* handle, void* index, const char* textureName, const char* samplerName, void* edgelen, void* interpolation) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)textureName; (void)samplerName; (void)edgelen; (void)interpolation;
  return;
#else
  try {
    ocio_rs_bridge::get_real_gpu_shader_desc(handle)->get3DTexture(index, textureName, samplerName, *static_cast<unsigned*>(edgelen), *static_cast<ocio::Interpolation*>(interpolation));
  } catch (...) { return ; }
#endif
}

void ocio_gpu_shader_desc_get3d_texture_values(void* handle, void* index, const float* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)values;
  return;
#else
  try {
    ocio_rs_bridge::get_real_gpu_shader_desc(handle)->get3DTextureValues(index, values);
  } catch (...) { return ; }
#endif
}

void* ocio_gpu_shader_desc_get3d_texture_shader_binding_index(void* handle, void* index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->get3DTextureShaderBindingIndex(index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_shader_text(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getShaderText();
  } catch (...) { return nullptr; }
#endif
}


// --- Baker ---

void* ocio_baker_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_baker().release();
#else
  auto handle = ocio_rs_bridge::make_real_baker();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_baker_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::BakerHandle*>(handle);
}

void* ocio_baker_get_config(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_baker(handle)->getConfig();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Config>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(ocio_rs_bridge::RealConfig{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_config(void* handle, void* config) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)config;
  return;
#else
  try {
    auto* _config_h = static_cast<ocio_rs_bridge::ConfigHandle*>(config);
    auto config_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_config_h->inner)->config;
    ocio_rs_bridge::get_real_baker(handle)->setConfig(config_ptr);
  } catch (...) { return ; }
#endif
}

void* ocio_baker_get_format(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_baker(handle)->getFormat();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_format(void* handle, const char* formatName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)formatName;
  return;
#else
  try {
    ocio_rs_bridge::get_real_baker(handle)->setFormat(formatName);
  } catch (...) { return ; }
#endif
}

void* ocio_baker_get_format_metadata(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_baker(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_baker_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_baker(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_baker_get_input_space(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_baker(handle)->getInputSpace();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_input_space(void* handle, const char* inputSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)inputSpace;
  return;
#else
  try {
    ocio_rs_bridge::get_real_baker(handle)->setInputSpace(inputSpace);
  } catch (...) { return ; }
#endif
}

void* ocio_baker_get_shaper_space(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_baker(handle)->getShaperSpace();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_shaper_space(void* handle, const char* shaperSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shaperSpace;
  return;
#else
  try {
    ocio_rs_bridge::get_real_baker(handle)->setShaperSpace(shaperSpace);
  } catch (...) { return ; }
#endif
}

void* ocio_baker_get_looks(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_baker(handle)->getLooks();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_looks(void* handle, const char* looks) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)looks;
  return;
#else
  try {
    ocio_rs_bridge::get_real_baker(handle)->setLooks(looks);
  } catch (...) { return ; }
#endif
}

void* ocio_baker_get_target_space(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_baker(handle)->getTargetSpace();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_target_space(void* handle, const char* targetSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)targetSpace;
  return;
#else
  try {
    ocio_rs_bridge::get_real_baker(handle)->setTargetSpace(targetSpace);
  } catch (...) { return ; }
#endif
}

void* ocio_baker_get_display(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_baker(handle)->getDisplay();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_baker_get_view(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_baker(handle)->getView();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_display_view(void* handle, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)view;
  return;
#else
  try {
    ocio_rs_bridge::get_real_baker(handle)->setDisplayView(display, view);
  } catch (...) { return ; }
#endif
}

int ocio_baker_get_shaper_size(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_baker(handle)->getShaperSize();
  } catch (...) { return 0; }
#endif
}

void ocio_baker_set_shaper_size(void* handle, int shapersize) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shapersize;
  return;
#else
  try {
    ocio_rs_bridge::get_real_baker(handle)->setShaperSize(shapersize);
  } catch (...) { return ; }
#endif
}

int ocio_baker_get_cube_size(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_baker(handle)->getCubeSize();
  } catch (...) { return 0; }
#endif
}

void ocio_baker_set_cube_size(void* handle, int cubesize) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)cubesize;
  return;
#else
  try {
    ocio_rs_bridge::get_real_baker(handle)->setCubeSize(cubesize);
  } catch (...) { return ; }
#endif
}

void ocio_baker_bake(void* handle, void* os) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)os;
  return;
#else
  try {
    ocio_rs_bridge::get_real_baker(handle)->bake(*static_cast<std::ostream*>(os));
  } catch (...) { return ; }
#endif
}


// --- Context ---

void* ocio_context_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_context().release();
#else
  auto handle = ocio_rs_bridge::make_real_context();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_context_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ContextHandle*>(handle);
}

void* ocio_context_get_cache_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->getCacheID();
  } catch (...) { return nullptr; }
#endif
}

void ocio_context_set_search_path(void* handle, const char* path) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)path;
  return;
#else
  try {
    ocio_rs_bridge::get_real_context(handle)->setSearchPath(path);
  } catch (...) { return ; }
#endif
}

void* ocio_context_get_search_path(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->getSearchPath();
  } catch (...) { return nullptr; }
#endif
}

int ocio_context_get_num_search_paths(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->getNumSearchPaths();
  } catch (...) { return 0; }
#endif
}

void* ocio_context_get_search_path_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->getSearchPath(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_context_clear_search_paths(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_context(handle)->clearSearchPaths();
  } catch (...) { return ; }
#endif
}

void ocio_context_add_search_path(void* handle, const char* path) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)path;
  return;
#else
  try {
    ocio_rs_bridge::get_real_context(handle)->addSearchPath(path);
  } catch (...) { return ; }
#endif
}

void ocio_context_set_working_dir(void* handle, const char* dirname) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dirname;
  return;
#else
  try {
    ocio_rs_bridge::get_real_context(handle)->setWorkingDir(dirname);
  } catch (...) { return ; }
#endif
}

void* ocio_context_get_working_dir(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->getWorkingDir();
  } catch (...) { return nullptr; }
#endif
}

void ocio_context_set_string_var(void* handle, const char* name, const char* value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name; (void)value;
  return;
#else
  try {
    ocio_rs_bridge::get_real_context(handle)->setStringVar(name, value);
  } catch (...) { return ; }
#endif
}

void* ocio_context_get_string_var(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->getStringVar(name);
  } catch (...) { return nullptr; }
#endif
}

int ocio_context_get_num_string_vars(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->getNumStringVars();
  } catch (...) { return 0; }
#endif
}

void* ocio_context_get_string_var_name_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->getStringVarNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_context_get_string_var_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->getStringVarByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_context_clear_string_vars(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_context(handle)->clearStringVars();
  } catch (...) { return ; }
#endif
}

void ocio_context_add_string_vars(void* handle, void* ctx) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ctx;
  return;
#else
  try {
    auto* _ctx_h = static_cast<ocio_rs_bridge::ContextHandle*>(ctx);
    auto ctx_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_ctx_h->inner)->context;
    ocio_rs_bridge::get_real_context(handle)->addStringVars(ctx_ptr);
  } catch (...) { return ; }
#endif
}

void ocio_context_set_environment_mode(void* handle, int mode) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)mode;
  return;
#else
  try {
    ocio_rs_bridge::get_real_context(handle)->setEnvironmentMode(static_cast<ocio::EnvironmentMode>(mode));
  } catch (...) { return ; }
#endif
}

int ocio_context_get_environment_mode(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->getEnvironmentMode();
  } catch (...) { return 0; }
#endif
}

void ocio_context_load_environment(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_context(handle)->loadEnvironment();
  } catch (...) { return ; }
#endif
}

void* ocio_context_resolve_string_var(void* handle, const char* string) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)string;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->resolveStringVar(string);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_context_resolve_string_var_v1(void* handle, const char* string, void* usedContextVars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)string; (void)usedContextVars;
  return nullptr;
#else
  try {
    auto* _usedContextVars_h = static_cast<ocio_rs_bridge::ContextHandle*>(usedContextVars);
    auto usedContextVars_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_usedContextVars_h->inner)->context;
    return ocio_rs_bridge::get_real_context(handle)->resolveStringVar(string, usedContextVars_ptr);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_context_resolve_file_location(void* handle, const char* filename) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filename;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_context(handle)->resolveFileLocation(filename);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_context_resolve_file_location_v1(void* handle, const char* filename, void* usedContextVars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filename; (void)usedContextVars;
  return nullptr;
#else
  try {
    auto* _usedContextVars_h = static_cast<ocio_rs_bridge::ContextHandle*>(usedContextVars);
    auto usedContextVars_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_usedContextVars_h->inner)->context;
    return ocio_rs_bridge::get_real_context(handle)->resolveFileLocation(filename, usedContextVars_ptr);
  } catch (...) { return nullptr; }
#endif
}

void ocio_context_set_config_io_proxy(void* handle, void* ciop) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ciop;
  return;
#else
  try {
    auto* _ciop_h = static_cast<ocio_rs_bridge::ConfigIOProxyHandle*>(ciop);
    auto ciop_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfigIOProxy>(_ciop_h->inner)->proxy;
    ocio_rs_bridge::get_real_context(handle)->setConfigIOProxy(ciop_ptr);
  } catch (...) { return ; }
#endif
}

void* ocio_context_get_config_io_proxy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_context(handle)->getConfigIOProxy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ConfigIOProxyHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealConfigIOProxy>(ocio_rs_bridge::RealConfigIOProxy{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}


// --- AllocationTransform ---

void* ocio_allocation_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_allocation_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_allocation_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_allocation_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::AllocationTransformHandle*>(handle);
}

int ocio_allocation_transform_get_direction(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_allocation_transform(handle)->getDirection();
  } catch (...) { return 0; }
#endif
}

void ocio_allocation_transform_set_direction(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return;
#else
  try {
    ocio_rs_bridge::get_real_allocation_transform(handle)->setDirection(static_cast<ocio::TransformDirection>(dir));
  } catch (...) { return ; }
#endif
}

void ocio_allocation_transform_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_allocation_transform(handle)->validate();
  } catch (...) { return ; }
#endif
}

int ocio_allocation_transform_get_allocation(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_allocation_transform(handle)->getAllocation();
  } catch (...) { return 0; }
#endif
}

void ocio_allocation_transform_set_allocation(void* handle, int allocation) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)allocation;
  return;
#else
  try {
    ocio_rs_bridge::get_real_allocation_transform(handle)->setAllocation(static_cast<ocio::Allocation>(allocation));
  } catch (...) { return ; }
#endif
}

int ocio_allocation_transform_get_num_vars(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_allocation_transform(handle)->getNumVars();
  } catch (...) { return 0; }
#endif
}

void ocio_allocation_transform_get_vars(void* handle, void* vars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)vars;
  return;
#else
  try {
    ocio_rs_bridge::get_real_allocation_transform(handle)->getVars(vars);
  } catch (...) { return ; }
#endif
}

void ocio_allocation_transform_set_vars(void* handle, int numvars, const float* vars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)numvars; (void)vars;
  return;
#else
  try {
    ocio_rs_bridge::get_real_allocation_transform(handle)->setVars(numvars, vars);
  } catch (...) { return ; }
#endif
}


// --- BuiltinTransform ---

void* ocio_builtin_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_builtin_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_builtin_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_builtin_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::BuiltinTransformHandle*>(handle);
}

void* ocio_builtin_transform_get_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_builtin_transform(handle)->getStyle();
  } catch (...) { return nullptr; }
#endif
}

void ocio_builtin_transform_set_style(void* handle, const char* style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_builtin_transform(handle)->setStyle(style);
  } catch (...) { return ; }
#endif
}

void* ocio_builtin_transform_get_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_builtin_transform(handle)->getDescription();
  } catch (...) { return nullptr; }
#endif
}


// --- CDLTransform ---

void* ocio_cdl_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_cdl_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_cdl_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_cdl_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::CDLTransformHandle*>(handle);
}

void* ocio_cdl_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_cdl_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_cdl_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_cdl_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_cdl_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cdl_transform(handle)->equals(*static_cast<const ocio::CDLTransform*>(other));
  } catch (...) { return false; }
#endif
}

int ocio_cdl_transform_get_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_cdl_transform(handle)->getStyle();
  } catch (...) { return 0; }
#endif
}

void ocio_cdl_transform_set_style(void* handle, int style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setStyle(static_cast<ocio::CDLStyle>(style));
  } catch (...) { return ; }
#endif
}

void ocio_cdl_transform_get_slope(void* handle, void* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->getSlope(rgb);
  } catch (...) { return ; }
#endif
}

void ocio_cdl_transform_set_slope(void* handle, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setSlope(rgb);
  } catch (...) { return ; }
#endif
}

void ocio_cdl_transform_get_offset(void* handle, void* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->getOffset(rgb);
  } catch (...) { return ; }
#endif
}

void ocio_cdl_transform_set_offset(void* handle, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setOffset(rgb);
  } catch (...) { return ; }
#endif
}

void ocio_cdl_transform_get_power(void* handle, void* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->getPower(rgb);
  } catch (...) { return ; }
#endif
}

void ocio_cdl_transform_set_power(void* handle, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setPower(rgb);
  } catch (...) { return ; }
#endif
}

void ocio_cdl_transform_get_sop(void* handle, void* vec9) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)vec9;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->getSOP(vec9);
  } catch (...) { return ; }
#endif
}

void ocio_cdl_transform_set_sop(void* handle, const double* vec9) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)vec9;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setSOP(vec9);
  } catch (...) { return ; }
#endif
}

double ocio_cdl_transform_get_sat(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_cdl_transform(handle)->getSat();
  } catch (...) { return 0.0; }
#endif
}

void ocio_cdl_transform_set_sat(void* handle, double sat) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)sat;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setSat(sat);
  } catch (...) { return ; }
#endif
}

void ocio_cdl_transform_get_sat_luma_coefs(void* handle, void* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->getSatLumaCoefs(rgb);
  } catch (...) { return ; }
#endif
}

void* ocio_cdl_transform_get_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_cdl_transform(handle)->getID();
  } catch (...) { return nullptr; }
#endif
}

void ocio_cdl_transform_set_id(void* handle, const char* id) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)id;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setID(id);
  } catch (...) { return ; }
#endif
}

void* ocio_cdl_transform_get_first_sop_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_cdl_transform(handle)->getFirstSOPDescription();
  } catch (...) { return nullptr; }
#endif
}

void ocio_cdl_transform_set_first_sop_description(void* handle, const char* description) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)description;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setFirstSOPDescription(description);
  } catch (...) { return ; }
#endif
}


// --- ColorSpaceTransform ---

void* ocio_color_space_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_color_space_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_color_space_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_color_space_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(handle);
}

int ocio_color_space_transform_get_direction(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_color_space_transform(handle)->getDirection();
  } catch (...) { return 0; }
#endif
}

void ocio_color_space_transform_set_direction(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space_transform(handle)->setDirection(static_cast<ocio::TransformDirection>(dir));
  } catch (...) { return ; }
#endif
}

void ocio_color_space_transform_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space_transform(handle)->validate();
  } catch (...) { return ; }
#endif
}

void* ocio_color_space_transform_get_src(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space_transform(handle)->getSrc();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_transform_set_src(void* handle, const char* src) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)src;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space_transform(handle)->setSrc(src);
  } catch (...) { return ; }
#endif
}

void* ocio_color_space_transform_get_dst(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space_transform(handle)->getDst();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_transform_set_dst(void* handle, const char* dst) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dst;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space_transform(handle)->setDst(dst);
  } catch (...) { return ; }
#endif
}

bool ocio_color_space_transform_get_data_bypass(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_color_space_transform(handle)->getDataBypass();
  } catch (...) { return false; }
#endif
}

void ocio_color_space_transform_set_data_bypass(void* handle, bool enabled) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)enabled;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space_transform(handle)->setDataBypass(enabled);
  } catch (...) { return ; }
#endif
}


// --- DisplayViewTransform ---

void* ocio_display_view_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_display_view_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_display_view_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_display_view_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(handle);
}

int ocio_display_view_transform_get_direction(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_display_view_transform(handle)->getDirection();
  } catch (...) { return 0; }
#endif
}

void ocio_display_view_transform_set_direction(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return;
#else
  try {
    ocio_rs_bridge::get_real_display_view_transform(handle)->setDirection(static_cast<ocio::TransformDirection>(dir));
  } catch (...) { return ; }
#endif
}

void ocio_display_view_transform_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_display_view_transform(handle)->validate();
  } catch (...) { return ; }
#endif
}

void* ocio_display_view_transform_get_src(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_display_view_transform(handle)->getSrc();
  } catch (...) { return nullptr; }
#endif
}

void ocio_display_view_transform_set_src(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_display_view_transform(handle)->setSrc(name);
  } catch (...) { return ; }
#endif
}

void* ocio_display_view_transform_get_display(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_display_view_transform(handle)->getDisplay();
  } catch (...) { return nullptr; }
#endif
}

void ocio_display_view_transform_set_display(void* handle, const char* display) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display;
  return;
#else
  try {
    ocio_rs_bridge::get_real_display_view_transform(handle)->setDisplay(display);
  } catch (...) { return ; }
#endif
}

void* ocio_display_view_transform_get_view(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_display_view_transform(handle)->getView();
  } catch (...) { return nullptr; }
#endif
}

void ocio_display_view_transform_set_view(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return;
#else
  try {
    ocio_rs_bridge::get_real_display_view_transform(handle)->setView(view);
  } catch (...) { return ; }
#endif
}

bool ocio_display_view_transform_get_looks_bypass(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_display_view_transform(handle)->getLooksBypass();
  } catch (...) { return false; }
#endif
}

void ocio_display_view_transform_set_looks_bypass(void* handle, bool bypass) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)bypass;
  return;
#else
  try {
    ocio_rs_bridge::get_real_display_view_transform(handle)->setLooksBypass(bypass);
  } catch (...) { return ; }
#endif
}

bool ocio_display_view_transform_get_data_bypass(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_display_view_transform(handle)->getDataBypass();
  } catch (...) { return false; }
#endif
}

void ocio_display_view_transform_set_data_bypass(void* handle, bool bypass) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)bypass;
  return;
#else
  try {
    ocio_rs_bridge::get_real_display_view_transform(handle)->setDataBypass(bypass);
  } catch (...) { return ; }
#endif
}


// --- ExponentTransform ---

void* ocio_exponent_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_exponent_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_exponent_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_exponent_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ExponentTransformHandle*>(handle);
}

void* ocio_exponent_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_exponent_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_exponent_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_exponent_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_exponent_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_exponent_transform(handle)->equals(*static_cast<const ocio::ExponentTransform*>(other));
  } catch (...) { return false; }
#endif
}

int ocio_exponent_transform_get_negative_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_exponent_transform(handle)->getNegativeStyle();
  } catch (...) { return 0; }
#endif
}

void ocio_exponent_transform_set_negative_style(void* handle, int style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_exponent_transform(handle)->setNegativeStyle(static_cast<ocio::NegativeStyle>(style));
  } catch (...) { return ; }
#endif
}


// --- ExponentWithLinearTransform ---

void* ocio_exponent_with_linear_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_exponent_with_linear_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_exponent_with_linear_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_exponent_with_linear_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(handle);
}

void* ocio_exponent_with_linear_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_exponent_with_linear_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_exponent_with_linear_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_exponent_with_linear_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_exponent_with_linear_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_exponent_with_linear_transform(handle)->equals(*static_cast<const ocio::ExponentWithLinearTransform*>(other));
  } catch (...) { return false; }
#endif
}

int ocio_exponent_with_linear_transform_get_negative_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_exponent_with_linear_transform(handle)->getNegativeStyle();
  } catch (...) { return 0; }
#endif
}

void ocio_exponent_with_linear_transform_set_negative_style(void* handle, int style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_exponent_with_linear_transform(handle)->setNegativeStyle(static_cast<ocio::NegativeStyle>(style));
  } catch (...) { return ; }
#endif
}


// --- ExposureContrastTransform ---

void* ocio_exposure_contrast_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_exposure_contrast_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_exposure_contrast_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_exposure_contrast_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(handle);
}

void* ocio_exposure_contrast_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_exposure_contrast_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_exposure_contrast_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->equals(*static_cast<const ocio::ExposureContrastTransform*>(other));
  } catch (...) { return false; }
#endif
}

int ocio_exposure_contrast_transform_get_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->getStyle();
  } catch (...) { return 0; }
#endif
}

void ocio_exposure_contrast_transform_set_style(void* handle, int style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->setStyle(static_cast<ocio::ExposureContrastStyle>(style));
  } catch (...) { return ; }
#endif
}

double ocio_exposure_contrast_transform_get_exposure(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->getExposure();
  } catch (...) { return 0.0; }
#endif
}

void ocio_exposure_contrast_transform_set_exposure(void* handle, double exposure) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)exposure;
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->setExposure(exposure);
  } catch (...) { return ; }
#endif
}

bool ocio_exposure_contrast_transform_is_exposure_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->isExposureDynamic();
  } catch (...) { return false; }
#endif
}

void ocio_exposure_contrast_transform_make_exposure_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->makeExposureDynamic();
  } catch (...) { return ; }
#endif
}

void ocio_exposure_contrast_transform_make_exposure_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->makeExposureNonDynamic();
  } catch (...) { return ; }
#endif
}

double ocio_exposure_contrast_transform_get_contrast(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->getContrast();
  } catch (...) { return 0.0; }
#endif
}

void ocio_exposure_contrast_transform_set_contrast(void* handle, double contrast) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)contrast;
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->setContrast(contrast);
  } catch (...) { return ; }
#endif
}

bool ocio_exposure_contrast_transform_is_contrast_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->isContrastDynamic();
  } catch (...) { return false; }
#endif
}

void ocio_exposure_contrast_transform_make_contrast_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->makeContrastDynamic();
  } catch (...) { return ; }
#endif
}

void ocio_exposure_contrast_transform_make_contrast_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->makeContrastNonDynamic();
  } catch (...) { return ; }
#endif
}

double ocio_exposure_contrast_transform_get_gamma(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->getGamma();
  } catch (...) { return 0.0; }
#endif
}

void ocio_exposure_contrast_transform_set_gamma(void* handle, double gamma) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)gamma;
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->setGamma(gamma);
  } catch (...) { return ; }
#endif
}

bool ocio_exposure_contrast_transform_is_gamma_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->isGammaDynamic();
  } catch (...) { return false; }
#endif
}

void ocio_exposure_contrast_transform_make_gamma_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->makeGammaDynamic();
  } catch (...) { return ; }
#endif
}

void ocio_exposure_contrast_transform_make_gamma_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->makeGammaNonDynamic();
  } catch (...) { return ; }
#endif
}

double ocio_exposure_contrast_transform_get_pivot(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->getPivot();
  } catch (...) { return 0.0; }
#endif
}

void ocio_exposure_contrast_transform_set_pivot(void* handle, double pivot) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)pivot;
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->setPivot(pivot);
  } catch (...) { return ; }
#endif
}

double ocio_exposure_contrast_transform_get_log_exposure_step(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->getLogExposureStep();
  } catch (...) { return 0.0; }
#endif
}

void ocio_exposure_contrast_transform_set_log_exposure_step(void* handle, double logExposureStep) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)logExposureStep;
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->setLogExposureStep(logExposureStep);
  } catch (...) { return ; }
#endif
}

double ocio_exposure_contrast_transform_get_log_mid_gray(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->getLogMidGray();
  } catch (...) { return 0.0; }
#endif
}

void ocio_exposure_contrast_transform_set_log_mid_gray(void* handle, double logMidGray) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)logMidGray;
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->setLogMidGray(logMidGray);
  } catch (...) { return ; }
#endif
}


// --- FileTransform ---

void* ocio_file_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_file_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_file_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_file_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::FileTransformHandle*>(handle);
}

int ocio_file_transform_get_direction(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_file_transform(handle)->getDirection();
  } catch (...) { return 0; }
#endif
}

void ocio_file_transform_set_direction(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_transform(handle)->setDirection(static_cast<ocio::TransformDirection>(dir));
  } catch (...) { return ; }
#endif
}

void ocio_file_transform_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_transform(handle)->validate();
  } catch (...) { return ; }
#endif
}

void* ocio_file_transform_get_src(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_file_transform(handle)->getSrc();
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_transform_set_src(void* handle, const char* src) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)src;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_transform(handle)->setSrc(src);
  } catch (...) { return ; }
#endif
}

void* ocio_file_transform_get_ccc_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_file_transform(handle)->getCCCId();
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_transform_set_ccc_id(void* handle, const char* id) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)id;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_transform(handle)->setCCCId(id);
  } catch (...) { return ; }
#endif
}

int ocio_file_transform_get_cdl_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_file_transform(handle)->getCDLStyle();
  } catch (...) { return 0; }
#endif
}

void ocio_file_transform_set_cdl_style(void* handle, int arg) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)arg;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_transform(handle)->setCDLStyle(static_cast<ocio::CDLStyle>(arg));
  } catch (...) { return ; }
#endif
}

int ocio_file_transform_get_interpolation(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_file_transform(handle)->getInterpolation();
  } catch (...) { return 0; }
#endif
}

void ocio_file_transform_set_interpolation(void* handle, int interp) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)interp;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_transform(handle)->setInterpolation(static_cast<ocio::Interpolation>(interp));
  } catch (...) { return ; }
#endif
}


// --- FixedFunctionTransform ---

void* ocio_fixed_function_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_fixed_function_transform().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_fixed_function_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(handle);
}

void* ocio_fixed_function_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_fixed_function_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_fixed_function_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_fixed_function_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_fixed_function_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_fixed_function_transform(handle)->equals(*static_cast<const ocio::FixedFunctionTransform*>(other));
  } catch (...) { return false; }
#endif
}

int ocio_fixed_function_transform_get_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_fixed_function_transform(handle)->getStyle();
  } catch (...) { return 0; }
#endif
}

void ocio_fixed_function_transform_set_style(void* handle, int style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_fixed_function_transform(handle)->setStyle(static_cast<ocio::FixedFunctionStyle>(style));
  } catch (...) { return ; }
#endif
}

size_t ocio_fixed_function_transform_get_num_params(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_fixed_function_transform(handle)->getNumParams();
  } catch (...) { return 0; }
#endif
}

void ocio_fixed_function_transform_get_params(void* handle, void* params) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)params;
  return;
#else
  try {
    ocio_rs_bridge::get_real_fixed_function_transform(handle)->getParams(params);
  } catch (...) { return ; }
#endif
}

void ocio_fixed_function_transform_set_params(void* handle, const double* params, size_t num) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)params; (void)num;
  return;
#else
  try {
    ocio_rs_bridge::get_real_fixed_function_transform(handle)->setParams(params, num);
  } catch (...) { return ; }
#endif
}


// --- GradingPrimaryTransform ---

void* ocio_grading_primary_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_grading_primary_transform().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_grading_primary_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(handle);
}

void* ocio_grading_primary_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_grading_primary_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_grading_primary_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_grading_primary_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_grading_primary_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_primary_transform(handle)->equals(*static_cast<const ocio::GradingPrimaryTransform*>(other));
  } catch (...) { return false; }
#endif
}

int ocio_grading_primary_transform_get_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_grading_primary_transform(handle)->getStyle();
  } catch (...) { return 0; }
#endif
}

void ocio_grading_primary_transform_set_style(void* handle, int style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_primary_transform(handle)->setStyle(static_cast<ocio::GradingStyle>(style));
  } catch (...) { return ; }
#endif
}

void* ocio_grading_primary_transform_get_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_grading_primary_transform(handle)->getValue();
  } catch (...) { return nullptr; }
#endif
}

void ocio_grading_primary_transform_set_value(void* handle, void* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_primary_transform(handle)->setValue(*static_cast<const ocio::GradingPrimary*>(values));
  } catch (...) { return ; }
#endif
}

bool ocio_grading_primary_transform_is_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_primary_transform(handle)->isDynamic();
  } catch (...) { return false; }
#endif
}

void ocio_grading_primary_transform_make_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_primary_transform(handle)->makeDynamic();
  } catch (...) { return ; }
#endif
}

void ocio_grading_primary_transform_make_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_primary_transform(handle)->makeNonDynamic();
  } catch (...) { return ; }
#endif
}


// --- GradingRGBCurveTransform ---

void* ocio_grading_rgb_curve_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_grading_rgb_curve_transform().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_grading_rgb_curve_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(handle);
}

void* ocio_grading_rgb_curve_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_grading_rgb_curve_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_grading_rgb_curve_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->equals(*static_cast<const ocio::GradingRGBCurveTransform*>(other));
  } catch (...) { return false; }
#endif
}

int ocio_grading_rgb_curve_transform_get_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->getStyle();
  } catch (...) { return 0; }
#endif
}

void ocio_grading_rgb_curve_transform_set_style(void* handle, int style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->setStyle(static_cast<ocio::GradingStyle>(style));
  } catch (...) { return ; }
#endif
}

void* ocio_grading_rgb_curve_transform_get_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->getValue();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::GradingRGBCurveHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::GradingRGBCurve>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGradingRGBCurve>(ocio_rs_bridge::RealGradingRGBCurve{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_grading_rgb_curve_transform_set_value(void* handle, void* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    auto* _values_h = static_cast<ocio_rs_bridge::GradingRGBCurveHandle*>(values);
    auto values_ptr = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurve>(_values_h->inner)->curve;
    ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->setValue(values_ptr);
  } catch (...) { return ; }
#endif
}

float ocio_grading_rgb_curve_transform_get_slope(void* handle, int c, size_t index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index;
  return 0.0f;
#else
  try {
    return ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->getSlope(static_cast<ocio::RGBCurveType>(c), index);
  } catch (...) { return 0.0f; }
#endif
}

void ocio_grading_rgb_curve_transform_set_slope(void* handle, int c, size_t index, float slope) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index; (void)slope;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->setSlope(static_cast<ocio::RGBCurveType>(c), index, slope);
  } catch (...) { return ; }
#endif
}

bool ocio_grading_rgb_curve_transform_slopes_are_default(void* handle, int c) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->slopesAreDefault(static_cast<ocio::RGBCurveType>(c));
  } catch (...) { return false; }
#endif
}

bool ocio_grading_rgb_curve_transform_get_bypass_lin_to_log(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->getBypassLinToLog();
  } catch (...) { return false; }
#endif
}

void ocio_grading_rgb_curve_transform_set_bypass_lin_to_log(void* handle, bool bypass) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)bypass;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->setBypassLinToLog(bypass);
  } catch (...) { return ; }
#endif
}

bool ocio_grading_rgb_curve_transform_is_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->isDynamic();
  } catch (...) { return false; }
#endif
}

void ocio_grading_rgb_curve_transform_make_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->makeDynamic();
  } catch (...) { return ; }
#endif
}

void ocio_grading_rgb_curve_transform_make_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->makeNonDynamic();
  } catch (...) { return ; }
#endif
}


// --- GradingHueCurveTransform ---

void* ocio_grading_hue_curve_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_grading_hue_curve_transform().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_grading_hue_curve_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(handle);
}

void* ocio_grading_hue_curve_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_grading_hue_curve_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_grading_hue_curve_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->equals(*static_cast<const ocio::GradingHueCurveTransform*>(other));
  } catch (...) { return false; }
#endif
}

int ocio_grading_hue_curve_transform_get_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->getStyle();
  } catch (...) { return 0; }
#endif
}

void ocio_grading_hue_curve_transform_set_style(void* handle, int style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->setStyle(static_cast<ocio::GradingStyle>(style));
  } catch (...) { return ; }
#endif
}

void* ocio_grading_hue_curve_transform_get_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->getValue();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::GradingHueCurveHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::GradingHueCurve>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGradingHueCurve>(ocio_rs_bridge::RealGradingHueCurve{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_grading_hue_curve_transform_set_value(void* handle, void* value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)value;
  return;
#else
  try {
    auto* _value_h = static_cast<ocio_rs_bridge::GradingHueCurveHandle*>(value);
    auto value_ptr = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurve>(_value_h->inner)->curve;
    ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->setValue(value_ptr);
  } catch (...) { return ; }
#endif
}

float ocio_grading_hue_curve_transform_get_slope(void* handle, int c, size_t index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index;
  return 0.0f;
#else
  try {
    return ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->getSlope(static_cast<ocio::HueCurveType>(c), index);
  } catch (...) { return 0.0f; }
#endif
}

void ocio_grading_hue_curve_transform_set_slope(void* handle, int c, size_t index, float slope) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index; (void)slope;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->setSlope(static_cast<ocio::HueCurveType>(c), index, slope);
  } catch (...) { return ; }
#endif
}

bool ocio_grading_hue_curve_transform_slopes_are_default(void* handle, int c) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->slopesAreDefault(static_cast<ocio::HueCurveType>(c));
  } catch (...) { return false; }
#endif
}

int ocio_grading_hue_curve_transform_get_rgb_to_hsy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->getRGBToHSY();
  } catch (...) { return 0; }
#endif
}

void ocio_grading_hue_curve_transform_set_rgb_to_hsy(void* handle, int style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->setRGBToHSY(static_cast<ocio::HSYTransformStyle>(style));
  } catch (...) { return ; }
#endif
}

bool ocio_grading_hue_curve_transform_is_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->isDynamic();
  } catch (...) { return false; }
#endif
}

void ocio_grading_hue_curve_transform_make_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->makeDynamic();
  } catch (...) { return ; }
#endif
}

void ocio_grading_hue_curve_transform_make_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->makeNonDynamic();
  } catch (...) { return ; }
#endif
}


// --- GradingToneTransform ---

void* ocio_grading_tone_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_grading_tone_transform().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_grading_tone_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(handle);
}

void* ocio_grading_tone_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_grading_tone_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_grading_tone_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_grading_tone_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_grading_tone_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_tone_transform(handle)->equals(*static_cast<const ocio::GradingToneTransform*>(other));
  } catch (...) { return false; }
#endif
}

int ocio_grading_tone_transform_get_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_grading_tone_transform(handle)->getStyle();
  } catch (...) { return 0; }
#endif
}

void ocio_grading_tone_transform_set_style(void* handle, int style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_tone_transform(handle)->setStyle(static_cast<ocio::GradingStyle>(style));
  } catch (...) { return ; }
#endif
}

void* ocio_grading_tone_transform_get_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_grading_tone_transform(handle)->getValue();
  } catch (...) { return nullptr; }
#endif
}

void ocio_grading_tone_transform_set_value(void* handle, void* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_tone_transform(handle)->setValue(*static_cast<const ocio::GradingTone*>(values));
  } catch (...) { return ; }
#endif
}

bool ocio_grading_tone_transform_is_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_tone_transform(handle)->isDynamic();
  } catch (...) { return false; }
#endif
}

void ocio_grading_tone_transform_make_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_tone_transform(handle)->makeDynamic();
  } catch (...) { return ; }
#endif
}

void ocio_grading_tone_transform_make_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_tone_transform(handle)->makeNonDynamic();
  } catch (...) { return ; }
#endif
}


// --- GroupTransform ---

void* ocio_group_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_group_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_group_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_group_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GroupTransformHandle*>(handle);
}

void* ocio_group_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_group_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_group_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_group_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_group_transform_get_transform(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_group_transform(handle)->getTransform(index);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::TransformHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Transform>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealTransform>(ocio_rs_bridge::RealTransform{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_group_transform_get_transform_v1(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_group_transform(handle)->getTransform(index);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::TransformHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealTransform>(ocio_rs_bridge::RealTransform{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_group_transform_get_num_transforms(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_group_transform(handle)->getNumTransforms();
  } catch (...) { return 0; }
#endif
}

void ocio_group_transform_append_transform(void* handle, void* transform) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform;
  return;
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandle*>(transform);
    auto transform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealTransform>(_transform_h->inner)->transform;
    ocio_rs_bridge::get_real_group_transform(handle)->appendTransform(transform_ptr);
  } catch (...) { return ; }
#endif
}

void ocio_group_transform_prepend_transform(void* handle, void* transform) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform;
  return;
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandle*>(transform);
    auto transform_ptr = std::static_pointer_cast<ocio_rs_bridge::RealTransform>(_transform_h->inner)->transform;
    ocio_rs_bridge::get_real_group_transform(handle)->prependTransform(transform_ptr);
  } catch (...) { return ; }
#endif
}

void ocio_group_transform_write(void* handle, void* config, const char* formatName, void* os) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)config; (void)formatName; (void)os;
  return;
#else
  try {
    auto* _config_h = static_cast<ocio_rs_bridge::ConfigHandle*>(config);
    auto config_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_config_h->inner)->config;
    ocio_rs_bridge::get_real_group_transform(handle)->write(config_ptr, formatName, *static_cast<std::ostream*>(os));
  } catch (...) { return ; }
#endif
}


// --- LogAffineTransform ---

void* ocio_log_affine_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_log_affine_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_log_affine_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_log_affine_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(handle);
}

void* ocio_log_affine_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_log_affine_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_log_affine_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_log_affine_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_log_affine_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_log_affine_transform(handle)->equals(*static_cast<const ocio::LogAffineTransform*>(other));
  } catch (...) { return false; }
#endif
}

double ocio_log_affine_transform_get_base(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_log_affine_transform(handle)->getBase();
  } catch (...) { return 0.0; }
#endif
}

void ocio_log_affine_transform_set_base(void* handle, double base) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)base;
  return;
#else
  try {
    ocio_rs_bridge::get_real_log_affine_transform(handle)->setBase(base);
  } catch (...) { return ; }
#endif
}


// --- LogCameraTransform ---

void* ocio_log_camera_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_log_camera_transform().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_log_camera_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(handle);
}

void* ocio_log_camera_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_log_camera_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_log_camera_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_log_camera_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_log_camera_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_log_camera_transform(handle)->equals(*static_cast<const ocio::LogCameraTransform*>(other));
  } catch (...) { return false; }
#endif
}

double ocio_log_camera_transform_get_base(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_log_camera_transform(handle)->getBase();
  } catch (...) { return 0.0; }
#endif
}

void ocio_log_camera_transform_set_base(void* handle, double base) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)base;
  return;
#else
  try {
    ocio_rs_bridge::get_real_log_camera_transform(handle)->setBase(base);
  } catch (...) { return ; }
#endif
}

void ocio_log_camera_transform_unset_linear_slope_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_log_camera_transform(handle)->unsetLinearSlopeValue();
  } catch (...) { return ; }
#endif
}


// --- LogTransform ---

void* ocio_log_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_log_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_log_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_log_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LogTransformHandle*>(handle);
}

void* ocio_log_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_log_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_log_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_log_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_log_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_log_transform(handle)->equals(*static_cast<const ocio::LogTransform*>(other));
  } catch (...) { return false; }
#endif
}

double ocio_log_transform_get_base(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_log_transform(handle)->getBase();
  } catch (...) { return 0.0; }
#endif
}

void ocio_log_transform_set_base(void* handle, double val) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)val;
  return;
#else
  try {
    ocio_rs_bridge::get_real_log_transform(handle)->setBase(val);
  } catch (...) { return ; }
#endif
}


// --- LookTransform ---

void* ocio_look_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_look_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_look_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_look_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LookTransformHandle*>(handle);
}

int ocio_look_transform_get_direction(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_look_transform(handle)->getDirection();
  } catch (...) { return 0; }
#endif
}

void ocio_look_transform_set_direction(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return;
#else
  try {
    ocio_rs_bridge::get_real_look_transform(handle)->setDirection(static_cast<ocio::TransformDirection>(dir));
  } catch (...) { return ; }
#endif
}

void ocio_look_transform_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_look_transform(handle)->validate();
  } catch (...) { return ; }
#endif
}

void* ocio_look_transform_get_src(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_look_transform(handle)->getSrc();
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_transform_set_src(void* handle, const char* src) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)src;
  return;
#else
  try {
    ocio_rs_bridge::get_real_look_transform(handle)->setSrc(src);
  } catch (...) { return ; }
#endif
}

void* ocio_look_transform_get_dst(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_look_transform(handle)->getDst();
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_transform_set_dst(void* handle, const char* dst) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dst;
  return;
#else
  try {
    ocio_rs_bridge::get_real_look_transform(handle)->setDst(dst);
  } catch (...) { return ; }
#endif
}

void* ocio_look_transform_get_looks(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_look_transform(handle)->getLooks();
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_transform_set_looks(void* handle, const char* looks) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)looks;
  return;
#else
  try {
    ocio_rs_bridge::get_real_look_transform(handle)->setLooks(looks);
  } catch (...) { return ; }
#endif
}

bool ocio_look_transform_get_skip_color_space_conversion(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_look_transform(handle)->getSkipColorSpaceConversion();
  } catch (...) { return false; }
#endif
}

void ocio_look_transform_set_skip_color_space_conversion(void* handle, bool skip) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)skip;
  return;
#else
  try {
    ocio_rs_bridge::get_real_look_transform(handle)->setSkipColorSpaceConversion(skip);
  } catch (...) { return ; }
#endif
}


// --- Lut1DTransform ---

void* ocio_lut1d_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_lut1d_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_lut1d_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_lut1d_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(handle);
}

int ocio_lut1d_transform_get_file_output_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_lut1d_transform(handle)->getFileOutputBitDepth();
  } catch (...) { return 0; }
#endif
}

void ocio_lut1d_transform_set_file_output_bit_depth(void* handle, int bitDepth) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)bitDepth;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->setFileOutputBitDepth(static_cast<ocio::BitDepth>(bitDepth));
  } catch (...) { return ; }
#endif
}

void* ocio_lut1d_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_lut1d_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_lut1d_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_lut1d_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_lut1d_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_lut1d_transform(handle)->equals(*static_cast<const ocio::Lut1DTransform*>(other));
  } catch (...) { return false; }
#endif
}

void* ocio_lut1d_transform_get_length(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_lut1d_transform(handle)->getLength();
  } catch (...) { return nullptr; }
#endif
}

void ocio_lut1d_transform_set_length(void* handle, void* length) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)length;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->setLength(length);
  } catch (...) { return ; }
#endif
}

void ocio_lut1d_transform_get_value(void* handle, void* index, void* r, void* g, void* b) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)r; (void)g; (void)b;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->getValue(index, *static_cast<float*>(r), *static_cast<float*>(g), *static_cast<float*>(b));
  } catch (...) { return ; }
#endif
}

void ocio_lut1d_transform_set_value(void* handle, void* index, float r, float g, float b) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)r; (void)g; (void)b;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->setValue(index, r, g, b);
  } catch (...) { return ; }
#endif
}

bool ocio_lut1d_transform_get_input_half_domain(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_lut1d_transform(handle)->getInputHalfDomain();
  } catch (...) { return false; }
#endif
}

void ocio_lut1d_transform_set_input_half_domain(void* handle, bool isHalfDomain) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)isHalfDomain;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->setInputHalfDomain(isHalfDomain);
  } catch (...) { return ; }
#endif
}

bool ocio_lut1d_transform_get_output_raw_halfs(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_lut1d_transform(handle)->getOutputRawHalfs();
  } catch (...) { return false; }
#endif
}

void ocio_lut1d_transform_set_output_raw_halfs(void* handle, bool isRawHalfs) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)isRawHalfs;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->setOutputRawHalfs(isRawHalfs);
  } catch (...) { return ; }
#endif
}

int ocio_lut1d_transform_get_hue_adjust(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_lut1d_transform(handle)->getHueAdjust();
  } catch (...) { return 0; }
#endif
}

void ocio_lut1d_transform_set_hue_adjust(void* handle, int algo) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)algo;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->setHueAdjust(static_cast<ocio::Lut1DHueAdjust>(algo));
  } catch (...) { return ; }
#endif
}

int ocio_lut1d_transform_get_interpolation(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_lut1d_transform(handle)->getInterpolation();
  } catch (...) { return 0; }
#endif
}

void ocio_lut1d_transform_set_interpolation(void* handle, int algo) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)algo;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->setInterpolation(static_cast<ocio::Interpolation>(algo));
  } catch (...) { return ; }
#endif
}


// --- Lut3DTransform ---

void* ocio_lut3d_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_lut3d_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_lut3d_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_lut3d_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(handle);
}

int ocio_lut3d_transform_get_file_output_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_lut3d_transform(handle)->getFileOutputBitDepth();
  } catch (...) { return 0; }
#endif
}

void ocio_lut3d_transform_set_file_output_bit_depth(void* handle, int bitDepth) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)bitDepth;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut3d_transform(handle)->setFileOutputBitDepth(static_cast<ocio::BitDepth>(bitDepth));
  } catch (...) { return ; }
#endif
}

void* ocio_lut3d_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_lut3d_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_lut3d_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_lut3d_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_lut3d_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_lut3d_transform(handle)->equals(*static_cast<const ocio::Lut3DTransform*>(other));
  } catch (...) { return false; }
#endif
}

void* ocio_lut3d_transform_get_grid_size(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_lut3d_transform(handle)->getGridSize();
  } catch (...) { return nullptr; }
#endif
}

void ocio_lut3d_transform_set_grid_size(void* handle, void* gridSize) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)gridSize;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut3d_transform(handle)->setGridSize(gridSize);
  } catch (...) { return ; }
#endif
}

void ocio_lut3d_transform_get_value(void* handle, void* indexR, void* indexG, void* indexB, void* r, void* g, void* b) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)indexR; (void)indexG; (void)indexB; (void)r; (void)g; (void)b;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut3d_transform(handle)->getValue(indexR, indexG, indexB, *static_cast<float*>(r), *static_cast<float*>(g), *static_cast<float*>(b));
  } catch (...) { return ; }
#endif
}

void ocio_lut3d_transform_set_value(void* handle, void* indexR, void* indexG, void* indexB, float r, float g, float b) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)indexR; (void)indexG; (void)indexB; (void)r; (void)g; (void)b;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut3d_transform(handle)->setValue(indexR, indexG, indexB, r, g, b);
  } catch (...) { return ; }
#endif
}

int ocio_lut3d_transform_get_interpolation(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_lut3d_transform(handle)->getInterpolation();
  } catch (...) { return 0; }
#endif
}

void ocio_lut3d_transform_set_interpolation(void* handle, int algo) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)algo;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut3d_transform(handle)->setInterpolation(static_cast<ocio::Interpolation>(algo));
  } catch (...) { return ; }
#endif
}


// --- MatrixTransform ---

void* ocio_matrix_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_matrix_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_matrix_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_matrix_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::MatrixTransformHandle*>(handle);
}

void* ocio_matrix_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_matrix_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_matrix_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_matrix_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_matrix_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_matrix_transform(handle)->equals(*static_cast<const ocio::MatrixTransform*>(other));
  } catch (...) { return false; }
#endif
}

void ocio_matrix_transform_get_matrix(void* handle, void* m44) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)m44;
  return;
#else
  try {
    ocio_rs_bridge::get_real_matrix_transform(handle)->getMatrix(m44);
  } catch (...) { return ; }
#endif
}

void ocio_matrix_transform_set_matrix(void* handle, const double* m44) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)m44;
  return;
#else
  try {
    ocio_rs_bridge::get_real_matrix_transform(handle)->setMatrix(m44);
  } catch (...) { return ; }
#endif
}

void ocio_matrix_transform_get_offset(void* handle, void* offset4) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)offset4;
  return;
#else
  try {
    ocio_rs_bridge::get_real_matrix_transform(handle)->getOffset(offset4);
  } catch (...) { return ; }
#endif
}

void ocio_matrix_transform_set_offset(void* handle, const double* offset4) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)offset4;
  return;
#else
  try {
    ocio_rs_bridge::get_real_matrix_transform(handle)->setOffset(offset4);
  } catch (...) { return ; }
#endif
}

int ocio_matrix_transform_get_file_input_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_matrix_transform(handle)->getFileInputBitDepth();
  } catch (...) { return 0; }
#endif
}

void ocio_matrix_transform_set_file_input_bit_depth(void* handle, int bitDepth) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)bitDepth;
  return;
#else
  try {
    ocio_rs_bridge::get_real_matrix_transform(handle)->setFileInputBitDepth(static_cast<ocio::BitDepth>(bitDepth));
  } catch (...) { return ; }
#endif
}

int ocio_matrix_transform_get_file_output_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_matrix_transform(handle)->getFileOutputBitDepth();
  } catch (...) { return 0; }
#endif
}

void ocio_matrix_transform_set_file_output_bit_depth(void* handle, int bitDepth) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)bitDepth;
  return;
#else
  try {
    ocio_rs_bridge::get_real_matrix_transform(handle)->setFileOutputBitDepth(static_cast<ocio::BitDepth>(bitDepth));
  } catch (...) { return ; }
#endif
}


// --- RangeTransform ---

void* ocio_range_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_range_transform().release();
#else
  auto handle = ocio_rs_bridge::make_real_range_transform();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void ocio_range_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::RangeTransformHandle*>(handle);
}

int ocio_range_transform_get_style(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->getStyle();
  } catch (...) { return 0; }
#endif
}

void ocio_range_transform_set_style(void* handle, int style) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)style;
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->setStyle(static_cast<ocio::RangeStyle>(style));
  } catch (...) { return ; }
#endif
}

void* ocio_range_transform_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_range_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_range_transform_get_format_metadata_v2(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return &ocio_rs_bridge::get_real_range_transform(handle)->getFormatMetadata();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_range_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->equals(*static_cast<const ocio::RangeTransform*>(other));
  } catch (...) { return false; }
#endif
}

int ocio_range_transform_get_file_input_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->getFileInputBitDepth();
  } catch (...) { return 0; }
#endif
}

void ocio_range_transform_set_file_input_bit_depth(void* handle, int bitDepth) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)bitDepth;
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->setFileInputBitDepth(static_cast<ocio::BitDepth>(bitDepth));
  } catch (...) { return ; }
#endif
}

int ocio_range_transform_get_file_output_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->getFileOutputBitDepth();
  } catch (...) { return 0; }
#endif
}

void ocio_range_transform_set_file_output_bit_depth(void* handle, int bitDepth) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)bitDepth;
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->setFileOutputBitDepth(static_cast<ocio::BitDepth>(bitDepth));
  } catch (...) { return ; }
#endif
}

double ocio_range_transform_get_min_in_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->getMinInValue();
  } catch (...) { return 0.0; }
#endif
}

void ocio_range_transform_set_min_in_value(void* handle, double val) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)val;
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->setMinInValue(val);
  } catch (...) { return ; }
#endif
}

bool ocio_range_transform_has_min_in_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->hasMinInValue();
  } catch (...) { return false; }
#endif
}

void ocio_range_transform_unset_min_in_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->unsetMinInValue();
  } catch (...) { return ; }
#endif
}

void ocio_range_transform_set_max_in_value(void* handle, double val) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)val;
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->setMaxInValue(val);
  } catch (...) { return ; }
#endif
}

double ocio_range_transform_get_max_in_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->getMaxInValue();
  } catch (...) { return 0.0; }
#endif
}

bool ocio_range_transform_has_max_in_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->hasMaxInValue();
  } catch (...) { return false; }
#endif
}

void ocio_range_transform_unset_max_in_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->unsetMaxInValue();
  } catch (...) { return ; }
#endif
}

void ocio_range_transform_set_min_out_value(void* handle, double val) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)val;
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->setMinOutValue(val);
  } catch (...) { return ; }
#endif
}

double ocio_range_transform_get_min_out_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->getMinOutValue();
  } catch (...) { return 0.0; }
#endif
}

bool ocio_range_transform_has_min_out_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->hasMinOutValue();
  } catch (...) { return false; }
#endif
}

void ocio_range_transform_unset_min_out_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->unsetMinOutValue();
  } catch (...) { return ; }
#endif
}

void ocio_range_transform_set_max_out_value(void* handle, double val) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)val;
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->setMaxOutValue(val);
  } catch (...) { return ; }
#endif
}

double ocio_range_transform_get_max_out_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->getMaxOutValue();
  } catch (...) { return 0.0; }
#endif
}

bool ocio_range_transform_has_max_out_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->hasMaxOutValue();
  } catch (...) { return false; }
#endif
}

void ocio_range_transform_unset_max_out_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->unsetMaxOutValue();
  } catch (...) { return ; }
#endif
}


// --- DynamicProperty ---

void* ocio_dynamic_property_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_dynamic_property().release();
#else
  return nullptr; // @TODO: implement make_real
#endif
}

void ocio_dynamic_property_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(handle);
}

int ocio_dynamic_property_get_type(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_dynamic_property(handle)->getType();
  } catch (...) { return 0; }
#endif
}


}  // extern "C"
