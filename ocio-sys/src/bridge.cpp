#include "bridge.hpp"

#include <cstring>
#include <cstdint>
#include <cstdlib>
#include <memory>
#include <stdexcept>
#include <string>
#include <fstream>
#include <sstream>
#include <unordered_map>
#include <vector>

#ifndef OCIO_RS_STUB
#include <OpenColorIO/OpenColorIO.h>
#include <OpenColorIO/OpenColorTransforms.h>
namespace ocio = OCIO_NAMESPACE;
#endif

namespace ocio_rs_bridge {

thread_local std::string g_serialized_text;
thread_local std::string g_last_error;

void clear_last_error() {
  g_last_error.clear();
}

void capture_error_message(const char* msg) {
  g_last_error = msg ? msg : "OpenColorIO bridge error";
}

void capture_current_exception() {
  try {
    throw;
  }
#ifndef OCIO_RS_STUB
  catch (const ocio::Exception& e) {
    capture_error_message(e.what());
  }
#endif
  catch (const std::exception& e) {
    capture_error_message(e.what());
  }
  catch (...) {
    capture_error_message("Unknown OpenColorIO bridge exception");
  }
}

// --- Handle types ---

struct ConfigHandle { std::shared_ptr<void> inner; };
struct ProcessorHandle { std::shared_ptr<void> inner; };
struct ProcessorMetadataHandle { std::shared_ptr<void> inner; };
struct FormatMetadataHandle { std::shared_ptr<void> owner; void* metadata = nullptr; };
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
struct BuiltinTransformRegistryHandle { std::shared_ptr<void> inner; };
struct FileRulesHandle { std::shared_ptr<void> inner; };
struct ColorSpaceSetHandle { std::shared_ptr<void> inner; };

// Handle types for referenced-only classes
struct TransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override;
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct ConfigIOProxyHandle { std::shared_ptr<void> inner; };
struct ViewingRulesHandle { std::shared_ptr<void> inner; };
struct GpuShaderCreatorHandle { std::shared_ptr<void> inner; };
struct GradingPrimaryValueHandle { std::shared_ptr<void> inner; };
struct GradingRGBCurveHandle { std::shared_ptr<void> inner; };
struct GradingHueCurveHandle { std::shared_ptr<void> inner; };
struct GradingToneValueHandle { std::shared_ptr<void> inner; };
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
struct StubViewingRules {};
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
struct StubProcessorMetadata {};
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

static std::unique_ptr<ViewingRulesHandle> make_stub_viewing_rules() {
  auto handle = std::make_unique<ViewingRulesHandle>();
  handle->inner = std::make_shared<StubViewingRules>();
  return handle;
}

static std::unique_ptr<ProcessorMetadataHandle> make_stub_processor_metadata() {
  auto handle = std::make_unique<ProcessorMetadataHandle>();
  handle->inner = std::make_shared<StubProcessorMetadata>();
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

static std::unique_ptr<BuiltinTransformRegistryHandle> make_stub_builtin_transform_registry() {
  auto handle = std::make_unique<BuiltinTransformRegistryHandle>();
  handle->inner = std::make_shared<StubBuiltinTransform>();
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

static std::unique_ptr<FormatMetadataHandle> make_stub_format_metadata() {
  return std::make_unique<FormatMetadataHandle>();
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
struct RealGradingPrimaryValue {
  std::shared_ptr<ocio::GradingPrimary> value;
};
struct RealLogTransform {
  ocio::LogTransformRcPtr transform;
};
struct RealBuiltinConfigRegistry {
  const ocio::BuiltinConfigRegistry* registry;
};
struct RealBuiltinTransformRegistry {
  ocio::ConstBuiltinTransformRegistryRcPtr registry;
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
struct RealGradingToneValue {
  std::shared_ptr<ocio::GradingTone> value;
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
  std::shared_ptr<void> owner = nullptr;
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

#ifndef OCIO_RS_STUB
class RustConfigIOProxy : public ocio::ConfigIOProxy
{
public:
  std::string configData;
  std::unordered_map<std::string, std::vector<uint8_t>> lutData;
  std::unordered_map<std::string, std::string> fastHashes;

  static std::string normalizePath(const char* filepath, char separator)
  {
    if (!filepath) return {};
    std::string path(filepath);
    for (char& ch : path) {
      if (ch == '/' || ch == '\\') ch = separator;
    }
    return path;
  }

  const std::vector<uint8_t>* findLutData(const char* filepath) const
  {
    if (!filepath) return nullptr;
    if (auto it = lutData.find(filepath); it != lutData.end()) return &it->second;
    const auto slash = normalizePath(filepath, '/');
    if (auto it = lutData.find(slash); it != lutData.end()) return &it->second;
    const auto backslash = normalizePath(filepath, '\\');
    if (auto it = lutData.find(backslash); it != lutData.end()) return &it->second;
    return nullptr;
  }

  std::string findFastHash(const char* filepath) const
  {
    if (!filepath) return {};
    if (auto it = fastHashes.find(filepath); it != fastHashes.end()) return it->second;
    const auto slash = normalizePath(filepath, '/');
    if (auto it = fastHashes.find(slash); it != fastHashes.end()) return it->second;
    const auto backslash = normalizePath(filepath, '\\');
    if (auto it = fastHashes.find(backslash); it != fastHashes.end()) return it->second;
    return {};
  }

  std::vector<uint8_t> getLutData(const char* filepath) const override
  {
    const auto* data = findLutData(filepath);
    return data ? *data : std::vector<uint8_t>{};
  }

  std::string getConfigData() const override
  {
    return configData;
  }

  std::string getFastLutFileHash(const char* filepath) const override
  {
    return findFastHash(filepath);
  }
};
#endif

template <typename MapLike>
std::string interchange_attribute_name_by_index(const MapLike& attrs, int index)
{
  if (index < 0 || static_cast<size_t>(index) >= attrs.size()) return {};
  auto it = attrs.begin();
  std::advance(it, index);
  return it->first;
}

template <typename MapLike>
std::string interchange_attribute_value_by_index(const MapLike& attrs, int index)
{
  if (index < 0 || static_cast<size_t>(index) >= attrs.size()) return {};
  auto it = attrs.begin();
  std::advance(it, index);
  return it->second;
}

// --- TransformHandleBase out-of-line ---
int TransformHandle::get_transform_type_tag() const {
#ifdef OCIO_RS_STUB
  return -1;
#else
  auto transform = std::static_pointer_cast<RealTransform>(inner)->transform;
  if (!transform) return -1;
  if (std::dynamic_pointer_cast<ocio::AllocationTransform>(transform)) return 0;
  if (std::dynamic_pointer_cast<ocio::BuiltinTransform>(transform)) return 1;
  if (std::dynamic_pointer_cast<ocio::CDLTransform>(transform)) return 2;
  if (std::dynamic_pointer_cast<ocio::ColorSpaceTransform>(transform)) return 3;
  if (std::dynamic_pointer_cast<ocio::DisplayViewTransform>(transform)) return 4;
  if (std::dynamic_pointer_cast<ocio::ExponentTransform>(transform)) return 5;
  if (std::dynamic_pointer_cast<ocio::ExponentWithLinearTransform>(transform)) return 6;
  if (std::dynamic_pointer_cast<ocio::ExposureContrastTransform>(transform)) return 7;
  if (std::dynamic_pointer_cast<ocio::FileTransform>(transform)) return 8;
  if (std::dynamic_pointer_cast<ocio::FixedFunctionTransform>(transform)) return 9;
  if (std::dynamic_pointer_cast<ocio::GradingHueCurveTransform>(transform)) return 10;
  if (std::dynamic_pointer_cast<ocio::GradingPrimaryTransform>(transform)) return 11;
  if (std::dynamic_pointer_cast<ocio::GradingRGBCurveTransform>(transform)) return 12;
  if (std::dynamic_pointer_cast<ocio::GradingToneTransform>(transform)) return 13;
  if (std::dynamic_pointer_cast<ocio::GroupTransform>(transform)) return 14;
  if (std::dynamic_pointer_cast<ocio::LogAffineTransform>(transform)) return 15;
  if (std::dynamic_pointer_cast<ocio::LogCameraTransform>(transform)) return 16;
  if (std::dynamic_pointer_cast<ocio::LogTransform>(transform)) return 17;
  if (std::dynamic_pointer_cast<ocio::LookTransform>(transform)) return 18;
  if (std::dynamic_pointer_cast<ocio::Lut1DTransform>(transform)) return 19;
  if (std::dynamic_pointer_cast<ocio::Lut3DTransform>(transform)) return 20;
  if (std::dynamic_pointer_cast<ocio::MatrixTransform>(transform)) return 21;
  if (std::dynamic_pointer_cast<ocio::RangeTransform>(transform)) return 22;
  return -1;
#endif
}

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

static ocio::ViewingRulesRcPtr get_real_viewing_rules(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ViewingRulesHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealViewingRules>(h->inner)->rules;
}

static ocio::ProcessorMetadataRcPtr get_real_processor_metadata(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ProcessorMetadataHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealProcessorMetadata>(h->inner)->metadata;
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

static std::shared_ptr<ocio::GradingPrimary> get_real_grading_primary_value(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::GradingPrimaryValueHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryValue>(h->inner)->value;
}

static ocio::FormatMetadata* get_real_format_metadata(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(handle);
  return h ? static_cast<ocio::FormatMetadata*>(h->metadata) : nullptr;
}

static void* make_format_metadata_handle(std::shared_ptr<void> owner, const ocio::FormatMetadata* metadata) {
  if (!metadata) return nullptr;
  auto out_handle = std::make_unique<ocio_rs_bridge::FormatMetadataHandle>();
  out_handle->owner = std::move(owner);
  out_handle->metadata = const_cast<ocio::FormatMetadata*>(metadata);
  return out_handle.release();
}

static const ocio::BuiltinConfigRegistry* get_real_builtin_config_registry(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::BuiltinConfigRegistryHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealBuiltinConfigRegistry>(h->inner)->registry;
}

static std::shared_ptr<ocio::GradingTone> get_real_grading_tone_value(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::GradingToneValueHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealGradingToneValue>(h->inner)->value;
}

static ocio::ConstBuiltinTransformRegistryRcPtr get_real_builtin_transform_registry(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::BuiltinTransformRegistryHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealBuiltinTransformRegistry>(h->inner)->registry;
}

static std::shared_ptr<ocio_rs_bridge::RealConfigIOProxy> get_real_config_io_proxy_handle(void* handle) {
  auto* h = static_cast<ocio_rs_bridge::ConfigIOProxyHandle*>(handle);
  return std::static_pointer_cast<ocio_rs_bridge::RealConfigIOProxy>(h->inner);
}

static const void* parse_color_space_from_string_deprecated(
    const ocio::ConstConfigRcPtr& config,
    const char* str) {
#if defined(__clang__)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#elif defined(__GNUC__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#endif
  const void* result = static_cast<const void*>(config->parseColorSpaceFromString(str));
#if defined(__clang__)
#pragma clang diagnostic pop
#elif defined(__GNUC__)
#pragma GCC diagnostic pop
#endif
  return result;
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
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<AllocationTransformHandle> make_real_allocation_transform() {
  try {
    auto handle = std::make_unique<AllocationTransformHandle>();
    auto obj = std::make_shared<RealAllocationTransform>();
    obj->transform = ocio::AllocationTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<LookHandle> make_real_look() {
  try {
    auto handle = std::make_unique<LookHandle>();
    auto obj = std::make_shared<RealLook>();
    obj->look = ocio::Look::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<NamedTransformHandle> make_real_named_transform() {
  try {
    auto handle = std::make_unique<NamedTransformHandle>();
    auto obj = std::make_shared<RealNamedTransform>();
    obj->transform = ocio::NamedTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<ContextHandle> make_real_context() {
  try {
    auto handle = std::make_unique<ContextHandle>();
    auto obj = std::make_shared<RealContext>();
    obj->context = ocio::Context::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<Lut1DTransformHandle> make_real_lut1d_transform() {
  try {
    auto handle = std::make_unique<Lut1DTransformHandle>();
    auto obj = std::make_shared<RealLut1DTransform>();
    obj->transform = ocio::Lut1DTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<ColorSpaceTransformHandle> make_real_color_space_transform() {
  try {
    auto handle = std::make_unique<ColorSpaceTransformHandle>();
    auto obj = std::make_shared<RealColorSpaceTransform>();
    obj->transform = ocio::ColorSpaceTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<FileRulesHandle> make_real_file_rules() {
  try {
    auto handle = std::make_unique<FileRulesHandle>();
    auto obj = std::make_shared<RealFileRules>();
    obj->rules = ocio::FileRules::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<LogTransformHandle> make_real_log_transform() {
  try {
    auto handle = std::make_unique<LogTransformHandle>();
    auto obj = std::make_shared<RealLogTransform>();
    obj->transform = ocio::LogTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<ExposureContrastTransformHandle> make_real_exposure_contrast_transform() {
  try {
    auto handle = std::make_unique<ExposureContrastTransformHandle>();
    auto obj = std::make_shared<RealExposureContrastTransform>();
    obj->transform = ocio::ExposureContrastTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<GroupTransformHandle> make_real_group_transform() {
  try {
    auto handle = std::make_unique<GroupTransformHandle>();
    auto obj = std::make_shared<RealGroupTransform>();
    obj->transform = ocio::GroupTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<LogAffineTransformHandle> make_real_log_affine_transform() {
  try {
    auto handle = std::make_unique<LogAffineTransformHandle>();
    auto obj = std::make_shared<RealLogAffineTransform>();
    obj->transform = ocio::LogAffineTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<LookTransformHandle> make_real_look_transform() {
  try {
    auto handle = std::make_unique<LookTransformHandle>();
    auto obj = std::make_shared<RealLookTransform>();
    obj->transform = ocio::LookTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<RangeTransformHandle> make_real_range_transform() {
  try {
    auto handle = std::make_unique<RangeTransformHandle>();
    auto obj = std::make_shared<RealRangeTransform>();
    obj->transform = ocio::RangeTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<ExponentTransformHandle> make_real_exponent_transform() {
  try {
    auto handle = std::make_unique<ExponentTransformHandle>();
    auto obj = std::make_shared<RealExponentTransform>();
    obj->transform = ocio::ExponentTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<DisplayViewTransformHandle> make_real_display_view_transform() {
  try {
    auto handle = std::make_unique<DisplayViewTransformHandle>();
    auto obj = std::make_shared<RealDisplayViewTransform>();
    obj->transform = ocio::DisplayViewTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<ExponentWithLinearTransformHandle> make_real_exponent_with_linear_transform() {
  try {
    auto handle = std::make_unique<ExponentWithLinearTransformHandle>();
    auto obj = std::make_shared<RealExponentWithLinearTransform>();
    obj->transform = ocio::ExponentWithLinearTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<MatrixTransformHandle> make_real_matrix_transform() {
  try {
    auto handle = std::make_unique<MatrixTransformHandle>();
    auto obj = std::make_shared<RealMatrixTransform>();
    obj->transform = ocio::MatrixTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<BuiltinTransformHandle> make_real_builtin_transform() {
  try {
    auto handle = std::make_unique<BuiltinTransformHandle>();
    auto obj = std::make_shared<RealBuiltinTransform>();
    obj->transform = ocio::BuiltinTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<ColorSpaceSetHandle> make_real_color_space_set() {
  try {
    auto handle = std::make_unique<ColorSpaceSetHandle>();
    auto obj = std::make_shared<RealColorSpaceSet>();
    obj->set = ocio::ColorSpaceSet::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<BakerHandle> make_real_baker() {
  try {
    auto handle = std::make_unique<BakerHandle>();
    auto obj = std::make_shared<RealBaker>();
    obj->baker = ocio::Baker::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<CDLTransformHandle> make_real_cdl_transform() {
  try {
    auto handle = std::make_unique<CDLTransformHandle>();
    auto obj = std::make_shared<RealCDLTransform>();
    obj->transform = ocio::CDLTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<Lut3DTransformHandle> make_real_lut3d_transform() {
  try {
    auto handle = std::make_unique<Lut3DTransformHandle>();
    auto obj = std::make_shared<RealLut3DTransform>();
    obj->transform = ocio::Lut3DTransform::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<ColorSpaceHandle> make_real_color_space() {
  try {
    auto handle = std::make_unique<ColorSpaceHandle>();
    auto obj = std::make_shared<RealColorSpace>();
    obj->colorSpace = ocio::ColorSpace::Create();
    handle->inner = obj;
    return handle;
  } catch (...) { capture_current_exception(); return nullptr; }
}

static std::unique_ptr<ConfigHandle> make_real_config_raw() {
  try {
    auto handle = std::make_unique<ConfigHandle>();
    auto config = std::make_shared<RealConfig>();
    config->config = std::const_pointer_cast<ocio::Config>(ocio::Config::CreateRaw());
    handle->inner = config;
    return handle;
  } catch (...) {
    capture_current_exception();
    return nullptr;
  }
}

static std::unique_ptr<ConfigHandle> make_real_config_from_file(const char* path) {
  try {
    auto handle = std::make_unique<ConfigHandle>();
    auto config = std::make_shared<RealConfig>();
    config->config = std::const_pointer_cast<ocio::Config>(ocio::Config::CreateFromFile(path));
    if (!config->config) return nullptr;
    handle->inner = config;
    return handle;
  } catch (...) {
    capture_current_exception();
    return nullptr;
  }
}

template <typename HandleT, typename RealT, typename TransformT>
static void* wrap_transform_copy_as(const ocio::TransformRcPtr& transform) {
  auto typed = std::dynamic_pointer_cast<TransformT>(transform);
  if (!typed) return nullptr;
  auto handle = std::make_unique<HandleT>();
  handle->inner = std::make_shared<RealT>(RealT{typed});
  return handle.release();
}

static void* wrap_editable_transform_copy(const ocio::TransformRcPtr& transform) {
  if (!transform) return nullptr;
  if (auto* h = wrap_transform_copy_as<AllocationTransformHandle, RealAllocationTransform, ocio::AllocationTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<BuiltinTransformHandle, RealBuiltinTransform, ocio::BuiltinTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<CDLTransformHandle, RealCDLTransform, ocio::CDLTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<ColorSpaceTransformHandle, RealColorSpaceTransform, ocio::ColorSpaceTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<DisplayViewTransformHandle, RealDisplayViewTransform, ocio::DisplayViewTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<ExponentTransformHandle, RealExponentTransform, ocio::ExponentTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<ExponentWithLinearTransformHandle, RealExponentWithLinearTransform, ocio::ExponentWithLinearTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<ExposureContrastTransformHandle, RealExposureContrastTransform, ocio::ExposureContrastTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<FileTransformHandle, RealFileTransform, ocio::FileTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<FixedFunctionTransformHandle, RealFixedFunctionTransform, ocio::FixedFunctionTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<GradingHueCurveTransformHandle, RealGradingHueCurveTransform, ocio::GradingHueCurveTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<GradingPrimaryTransformHandle, RealGradingPrimaryTransform, ocio::GradingPrimaryTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<GradingRGBCurveTransformHandle, RealGradingRGBCurveTransform, ocio::GradingRGBCurveTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<GradingToneTransformHandle, RealGradingToneTransform, ocio::GradingToneTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<GroupTransformHandle, RealGroupTransform, ocio::GroupTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<LogAffineTransformHandle, RealLogAffineTransform, ocio::LogAffineTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<LogCameraTransformHandle, RealLogCameraTransform, ocio::LogCameraTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<LogTransformHandle, RealLogTransform, ocio::LogTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<LookTransformHandle, RealLookTransform, ocio::LookTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<Lut1DTransformHandle, RealLut1DTransform, ocio::Lut1DTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<Lut3DTransformHandle, RealLut3DTransform, ocio::Lut3DTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<MatrixTransformHandle, RealMatrixTransform, ocio::MatrixTransform>(transform)) return h;
  if (auto* h = wrap_transform_copy_as<RangeTransformHandle, RealRangeTransform, ocio::RangeTransform>(transform)) return h;
  auto handle = std::make_unique<TransformHandle>();
  handle->inner = std::make_shared<RealTransform>(RealTransform{transform});
  return handle.release();
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
  try { ocio::SetLoggingLevel(static_cast<ocio::LoggingLevel>(level)); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_set_current_config(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    ocio::SetCurrentConfig(ocio_rs_bridge::get_real_config(config));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_clear_all_caches(void) {
#ifdef OCIO_RS_STUB
#else
  try { ocio::ClearAllCaches(); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

size_t ocio_builtin_config_registry_get_num_builtin_configs(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_builtin_config_registry(handle)->getNumBuiltinConfigs();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
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

// --- BuiltinTransformRegistry ---
void* ocio_builtin_transform_registry_get(void) {
#ifdef OCIO_RS_STUB
  return nullptr;
#else
  try {
    auto registry = ocio::BuiltinTransformRegistry::Get();
    if (!registry) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::BuiltinTransformRegistryHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealBuiltinTransformRegistry>(
      ocio_rs_bridge::RealBuiltinTransformRegistry{registry});
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_builtin_transform_registry_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::BuiltinTransformRegistryHandle*>(handle);
}

size_t ocio_builtin_transform_registry_get_num_builtins(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_builtin_transform_registry(handle)->getNumBuiltins();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

const char* ocio_builtin_transform_registry_get_builtin_style(void* handle, size_t index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    auto registry = ocio_rs_bridge::get_real_builtin_transform_registry(handle);
    if (!registry || index >= registry->getNumBuiltins()) return nullptr;
    return registry->getBuiltinStyle(index);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_builtin_transform_registry_get_builtin_description(void* handle, size_t index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    auto registry = ocio_rs_bridge::get_real_builtin_transform_registry(handle);
    if (!registry || index >= registry->getNumBuiltins()) return nullptr;
    return registry->getBuiltinDescription(index);
  } catch (...) { return nullptr; }
#endif
}

// --- ConfigIOProxy ---
void* ocio_config_io_proxy_create(void) {
#ifdef OCIO_RS_STUB
  return nullptr;
#else
  try {
    auto rustProxy = std::make_shared<ocio_rs_bridge::RustConfigIOProxy>();
    ocio::ConfigIOProxyRcPtr proxy = rustProxy;
    auto handle = std::make_unique<ocio_rs_bridge::ConfigIOProxyHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealConfigIOProxy>(
      ocio_rs_bridge::RealConfigIOProxy{proxy, rustProxy});
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_config_io_proxy_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ConfigIOProxyHandle*>(handle);
}

void ocio_config_io_proxy_set_config_data(void* handle, const char* data) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)data;
  return;
#else
  try {
    auto real = ocio_rs_bridge::get_real_config_io_proxy_handle(handle);
    auto rustProxy = std::dynamic_pointer_cast<ocio_rs_bridge::RustConfigIOProxy>(real->proxy);
    if (!rustProxy) return;
    rustProxy->configData = data ? data : "";
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

const char* ocio_config_io_proxy_get_config_data(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return nullptr;
#else
  try {
    auto real = ocio_rs_bridge::get_real_config_io_proxy_handle(handle);
    auto rustProxy = std::dynamic_pointer_cast<ocio_rs_bridge::RustConfigIOProxy>(real->proxy);
    if (!rustProxy) return nullptr;
    ocio_rs_bridge::g_serialized_text = rustProxy->configData;
    return ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_config_io_proxy_set_lut_data(
  void* handle,
  const char* filepath,
  const unsigned char* data,
  size_t len,
  const char* fastHash) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filepath; (void)data; (void)len; (void)fastHash;
  return false;
#else
  try {
    if (!filepath) return false;
    auto real = ocio_rs_bridge::get_real_config_io_proxy_handle(handle);
    auto rustProxy = std::dynamic_pointer_cast<ocio_rs_bridge::RustConfigIOProxy>(real->proxy);
    if (!rustProxy) return false;
    std::vector<uint8_t> bytes;
    if (data && len > 0) {
      bytes.assign(data, data + len);
    }
    rustProxy->lutData[filepath] = std::move(bytes);
    rustProxy->fastHashes[filepath] = fastHash ? fastHash : "";
    rustProxy->lutData[ocio_rs_bridge::RustConfigIOProxy::normalizePath(filepath, '/')] =
      rustProxy->lutData[filepath];
    rustProxy->lutData[ocio_rs_bridge::RustConfigIOProxy::normalizePath(filepath, '\\')] =
      rustProxy->lutData[filepath];
    rustProxy->fastHashes[ocio_rs_bridge::RustConfigIOProxy::normalizePath(filepath, '/')] =
      rustProxy->fastHashes[filepath];
    rustProxy->fastHashes[ocio_rs_bridge::RustConfigIOProxy::normalizePath(filepath, '\\')] =
      rustProxy->fastHashes[filepath];
    return true;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

size_t ocio_config_io_proxy_get_lut_data_size(void* handle, const char* filepath) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filepath;
  return 0;
#else
  try {
    if (!filepath) return 0;
    auto real = ocio_rs_bridge::get_real_config_io_proxy_handle(handle);
    auto rustProxy = std::dynamic_pointer_cast<ocio_rs_bridge::RustConfigIOProxy>(real->proxy);
    if (!rustProxy) return 0;
    const auto* data = rustProxy->findLutData(filepath);
    return data ? data->size() : 0;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

bool ocio_config_io_proxy_has_lut_data(void* handle, const char* filepath) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filepath;
  return false;
#else
  try {
    if (!filepath) return false;
    auto real = ocio_rs_bridge::get_real_config_io_proxy_handle(handle);
    auto rustProxy = std::dynamic_pointer_cast<ocio_rs_bridge::RustConfigIOProxy>(real->proxy);
    return rustProxy && rustProxy->findLutData(filepath);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_config_io_proxy_copy_lut_data(
  void* handle,
  const char* filepath,
  unsigned char* data,
  size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filepath; (void)data; (void)len;
  return false;
#else
  try {
    if (!filepath || !data) return false;
    auto real = ocio_rs_bridge::get_real_config_io_proxy_handle(handle);
    auto rustProxy = std::dynamic_pointer_cast<ocio_rs_bridge::RustConfigIOProxy>(real->proxy);
    if (!rustProxy) return false;
    const auto* bytes = rustProxy->findLutData(filepath);
    if (!bytes || len < bytes->size()) return false;
    if (!bytes->empty()) {
      std::memcpy(data, bytes->data(), bytes->size());
    }
    return true;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

const char* ocio_config_io_proxy_get_fast_lut_file_hash(void* handle, const char* filepath) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filepath;
  return nullptr;
#else
  try {
    auto real = ocio_rs_bridge::get_real_config_io_proxy_handle(handle);
    auto rustProxy = std::dynamic_pointer_cast<ocio_rs_bridge::RustConfigIOProxy>(real->proxy);
    if (!rustProxy || !filepath) return nullptr;
    ocio_rs_bridge::g_serialized_text = rustProxy->getFastLutFileHash(filepath);
    return ocio_rs_bridge::g_serialized_text.empty()
      ? nullptr
      : ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { return nullptr; }
#endif
}


// --- Config ---
void* ocio_config_raw(void) {
  return ocio_config_create_raw();
}

void* ocio_config_from_file(const char* path) {
  return ocio_config_create_from_file(path);
}

void* ocio_config_create_raw(void) {
#ifdef OCIO_RS_STUB
  ocio_rs_bridge::clear_last_error();
  return ocio_rs_bridge::make_stub_config().release();
#else
  ocio_rs_bridge::clear_last_error();
  auto handle = ocio_rs_bridge::make_real_config_raw();
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void* ocio_config_create_from_file(const char* path) {
#ifdef OCIO_RS_STUB
  ocio_rs_bridge::clear_last_error();
  if (!path) return nullptr;
  std::ifstream file(path);
  if (!file.good()) return nullptr;
  return ocio_rs_bridge::make_stub_config().release();
#else
  ocio_rs_bridge::clear_last_error();
  auto handle = ocio_rs_bridge::make_real_config_from_file(path);
  if (!handle) return nullptr;
  return handle.release();
#endif
}

void* ocio_config_create_from_builtin_config(const char* configName) {
#ifdef OCIO_RS_STUB
  (void)configName;
  ocio_rs_bridge::clear_last_error();
  return nullptr;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    auto result = ocio::Config::CreateFromBuiltinConfig(configName);
    if (!result) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(
      ocio_rs_bridge::RealConfig{std::const_pointer_cast<ocio::Config>(result)});
    return handle.release();
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return nullptr;
  }
#endif
}

void* ocio_config_create_from_env(void) {
#ifdef OCIO_RS_STUB
  ocio_rs_bridge::clear_last_error();
  const char* ocio_env = std::getenv("OCIO");
  if (!ocio_env || !*ocio_env) return nullptr;
  std::ifstream file(ocio_env);
  if (!file.good()) return nullptr;
  return ocio_rs_bridge::make_stub_config().release();
#else
  ocio_rs_bridge::clear_last_error();
  try {
    auto result = ocio::Config::CreateFromEnv();
    if (!result) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(
      ocio_rs_bridge::RealConfig{std::const_pointer_cast<ocio::Config>(result)});
    return handle.release();
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return nullptr;
  }
#endif
}

void* ocio_config_create_from_stream(const char* text) {
#ifdef OCIO_RS_STUB
  ocio_rs_bridge::clear_last_error();
  if (!text || !*text) return nullptr;
  return ocio_rs_bridge::make_stub_config().release();
#else
  ocio_rs_bridge::clear_last_error();
  try {
    if (!text) return nullptr;
    std::istringstream stream(text);
    auto result = ocio::Config::CreateFromStream(stream);
    if (!result) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(
      ocio_rs_bridge::RealConfig{std::const_pointer_cast<ocio::Config>(result)});
    return handle.release();
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return nullptr;
  }
#endif
}

void* ocio_config_create_from_config_io_proxy(void* ciop) {
#ifdef OCIO_RS_STUB
  (void)ciop;
  ocio_rs_bridge::clear_last_error();
  return nullptr;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    if (!ciop) return nullptr;
    auto real = ocio_rs_bridge::get_real_config_io_proxy_handle(ciop);
    if (!real || !real->proxy) return nullptr;
    auto result = ocio::Config::CreateFromConfigIOProxy(real->proxy);
    if (!result) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(
      ocio_rs_bridge::RealConfig{std::const_pointer_cast<ocio::Config>(result)});
    return handle.release();
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return nullptr;
  }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_set_version(void* handle, unsigned int major, unsigned int minor) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)major; (void)minor;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setVersion(major, minor);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_upgrade_to_latest_version(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->upgradeToLatestVersion();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_config_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    ocio_rs_bridge::get_real_config(handle)->validate();
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
    if (!context) {
      return (void*)ocio_rs_bridge::get_real_config(handle)->getCacheID();
    }
    auto* _context_h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    auto context_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_context_h->inner)->context;
    return (void*)ocio_rs_bridge::get_real_config(handle)->getCacheID(context_ptr);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_config_add_environment_var(void* handle, const char* name, const char* defaultValue) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name; (void)defaultValue;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addEnvironmentVar(name, defaultValue);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_set_environment_mode(void* handle, int mode) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)mode;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setEnvironmentMode(static_cast<ocio::EnvironmentMode>(mode));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_config_add_search_path(void* handle, const char* path) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)path;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addSearchPath(path);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_config_get_color_spaces(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return ocio_rs_bridge::make_stub_color_space_set().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getColorSpaces(category);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ColorSpaceSetHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceSet>(ocio_rs_bridge::RealColorSpaceSet{result});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_remove_color_space(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeColorSpace(name);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_config_set_inactive_color_spaces(void* handle, const char* inactiveColorSpaces) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)inactiveColorSpaces;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setInactiveColorSpaces(inactiveColorSpaces);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_config(handle)->IdentifyBuiltinColorSpace(srcConfig_ptr, builtinConfig_ptr, builtinColorSpaceName)));
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
    ocio_rs_bridge::get_real_config(handle)->IdentifyInterchangeSpace(static_cast<const char**>(srcInterchangeName), static_cast<const char**>(builtinInterchangeName), srcConfig_ptr, srcColorSpaceName, builtinConfig_ptr, builtinColorSpaceName);
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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

void* ocio_config_get_role_color_space(void* handle, int index) {
  return ocio_config_get_role_color_space_by_index(handle, index);
}

void* ocio_config_get_role_color_space_v1(void* handle, const char* roleName) {
  return ocio_config_get_role_color_space_by_name(handle, roleName);
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_remove_shared_view(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeSharedView(view);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_clear_shared_views(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearSharedViews();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_add_display_view_v2(void* handle, const char* display, const char* view, const char* viewTransformName, const char* colorSpaceName, const char* looks, const char* ruleName, const char* description) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)view; (void)viewTransformName; (void)colorSpaceName; (void)looks; (void)ruleName; (void)description;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addDisplayView(display, view, viewTransformName, colorSpaceName, looks, ruleName, description);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_add_display_shared_view(void* handle, const char* display, const char* sharedView) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)sharedView;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addDisplaySharedView(display, sharedView);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_remove_display_view(void* handle, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display; (void)view;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeDisplayView(display, view);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_clear_displays(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearDisplays();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_add_virtual_display_shared_view(void* handle, const char* sharedView) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)sharedView;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->addVirtualDisplaySharedView(sharedView);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_config(handle)->getVirtualDisplayView(static_cast<ocio::ViewType>(type), index)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_config_clear_virtual_display(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearVirtualDisplay();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_config_get_active_displays(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_config(handle)->getActiveDisplays()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_remove_active_display(void* handle, const char* display) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)display;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeActiveDisplay(display);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_clear_active_displays(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearActiveDisplays();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_config_set_active_views(void* handle, const char* views) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)views;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setActiveViews(views);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_remove_active_view(void* handle, const char* view) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)view;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeActiveView(view);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_clear_active_views(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearActiveViews();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
    return ocio_rs_bridge::get_real_config(handle)->getDisplayAllByName(static_cast<const char*>(arg));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_config(handle)->getView(static_cast<ocio::ViewType>(type), display, index)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_get_default_luma_coefs(void* handle, void* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->getDefaultLumaCoefs(static_cast<double*>(rgb));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_config_set_default_luma_coefs(void* handle, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->setDefaultLumaCoefs(rgb);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_clear_looks(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearLooks();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_config(handle)->getViewTransformNameByIndex(i)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_config_clear_view_transforms(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearViewTransforms();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_config_remove_named_transform(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->removeNamedTransform(name);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_config_clear_named_transforms(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearNamedTransforms();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_config_get_color_space_from_filepath(void* handle, const char* filePath) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filePath;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_config(handle)->getColorSpaceFromFilepath(filePath)));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_color_space_from_filepath_by_ref_type(void* handle, const char* filePath, void* ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filePath; (void)ruleIndex;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_config(handle)->getColorSpaceFromFilepath(filePath, *static_cast<size_t*>(ruleIndex))));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_color_space_from_filepath_with_rule_index(void* handle, const char* filePath, size_t* ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filePath; (void)ruleIndex;
  return nullptr;
#else
  try {
    if (!ruleIndex) return nullptr;
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_config(handle)->getColorSpaceFromFilepath(filePath, *ruleIndex)));
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
    return const_cast<void*>(
        ocio_rs_bridge::parse_color_space_from_string_deprecated(
            ocio_rs_bridge::get_real_config(handle),
            str));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_config_get_processor(void* handle, void* context, void* srcColorSpace, void* dstColorSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)srcColorSpace; (void)dstColorSpace;
  return ocio_rs_bridge::make_stub_processor().release();
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v1(void* handle, void* srcColorSpace, void* dstColorSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcColorSpace; (void)dstColorSpace;
  return ocio_rs_bridge::make_stub_processor().release();
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v2(void* handle, const char* srcColorSpaceName, const char* dstColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcColorSpaceName; (void)dstColorSpaceName;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(srcColorSpaceName, dstColorSpaceName);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v3(void* handle, void* context, const char* srcColorSpaceName, const char* dstColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)srcColorSpaceName; (void)dstColorSpaceName;
  return ocio_rs_bridge::make_stub_processor().release();
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v4(void* handle, const char* srcColorSpaceName, const char* display, const char* view, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcColorSpaceName; (void)display; (void)view; (void)direction;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(srcColorSpaceName, display, view, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v5(void* handle, void* context, const char* srcColorSpaceName, const char* display, const char* view, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)srcColorSpaceName; (void)display; (void)view; (void)direction;
  return ocio_rs_bridge::make_stub_processor().release();
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v6(void* handle, void* namedTransform, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)namedTransform; (void)direction;
  return ocio_rs_bridge::make_stub_processor().release();
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v7(void* handle, void* context, void* namedTransform, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)namedTransform; (void)direction;
  return ocio_rs_bridge::make_stub_processor().release();
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v8(void* handle, const char* namedTransformName, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)namedTransformName; (void)direction;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(namedTransformName, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v9(void* handle, void* context, const char* namedTransformName, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)namedTransformName; (void)direction;
  return ocio_rs_bridge::make_stub_processor().release();
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v10(void* handle, void* transform) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    auto transform_ptr = _transform_h->get_ocio_transform();
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(transform_ptr);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v11(void* handle, void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform; (void)direction;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    auto transform_ptr = _transform_h->get_ocio_transform();
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(transform_ptr, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_v12(void* handle, void* context, void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)context; (void)transform; (void)direction;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  try {
    auto* _context_h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    auto context_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_context_h->inner)->context;
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    auto transform_ptr = _transform_h->get_ocio_transform();
    auto result = ocio_rs_bridge::get_real_config(handle)->getProcessor(context_ptr, transform_ptr, static_cast<ocio::TransformDirection>(direction));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_to_builtin_color_space(void* handle, void* srcConfig, const char* srcColorSpaceName, const char* builtinColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcConfig; (void)srcColorSpaceName; (void)builtinColorSpaceName;
  return ocio_rs_bridge::make_stub_processor().release();
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_from_builtin_color_space(void* handle, const char* builtinColorSpaceName, void* srcConfig, const char* srcColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)builtinColorSpaceName; (void)srcConfig; (void)srcColorSpaceName;
  return ocio_rs_bridge::make_stub_processor().release();
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_get_processor_from_configs(void* handle, void* srcConfig, const char* srcColorSpaceName, void* dstConfig, const char* dstColorSpaceName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcConfig; (void)srcColorSpaceName; (void)dstConfig; (void)dstColorSpaceName;
  return ocio_rs_bridge::make_stub_processor().release();
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_config_clear_processor_cache(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_config(handle)->clearProcessorCache();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_config_set_config_io_proxy(void* handle, void* ciop) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ciop;
  return;
#else
  try {
    if (!ciop) {
      ocio_rs_bridge::get_real_config(handle)->setConfigIOProxy(ocio::ConfigIOProxyRcPtr());
      return;
    }
    auto* _ciop_h = static_cast<ocio_rs_bridge::ConfigIOProxyHandle*>(ciop);
    auto ciop_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfigIOProxy>(_ciop_h->inner)->proxy;
    ocio_rs_bridge::get_real_config(handle)->setConfigIOProxy(ciop_ptr);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_config_get_config_io_proxy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto config = ocio_rs_bridge::get_real_config(handle);
    auto result = config->getConfigIOProxy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ConfigIOProxyHandle>();
    auto owner = std::make_shared<ocio::ConfigRcPtr>(std::move(config));
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealConfigIOProxy>(
      ocio_rs_bridge::RealConfigIOProxy{result, owner});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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

void* ocio_config_serialize_to_string(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return nullptr;
#else
  try {
    std::ostringstream config_stream;
    ocio_rs_bridge::get_real_config(handle)->serialize(config_stream);
    ocio_rs_bridge::g_serialized_text = config_stream.str();
    return (void*)ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_config_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_config().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_config(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(ocio_rs_bridge::RealConfig{result});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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

void* ocio_config_archive_to_string(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return nullptr;
#else
  try {
    std::ostringstream archive_stream;
    ocio_rs_bridge::get_real_config(handle)->archive(archive_stream);
    ocio_rs_bridge::g_serialized_text = archive_stream.str();
    return (void*)ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_file_rules(handle)->getName(ruleIndex)));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_file_rules_get_pattern(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_file_rules(handle)->getPattern(ruleIndex)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_file_rules_get_extension(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_file_rules(handle)->getExtension(ruleIndex)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_file_rules_get_regex(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_file_rules(handle)->getRegex(ruleIndex)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_file_rules_get_color_space(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_file_rules(handle)->getColorSpace(ruleIndex)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_file_rules(handle)->getCustomKeyName(ruleIndex, key)));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_file_rules_get_custom_key_value(void* handle, size_t ruleIndex, size_t key) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)key;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_file_rules(handle)->getCustomKeyValue(ruleIndex, key)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_file_rules_insert_rule(void* handle, size_t ruleIndex, const char* name, const char* colorSpace, const char* pattern, const char* extension) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)name; (void)colorSpace; (void)pattern; (void)extension;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->insertRule(ruleIndex, name, colorSpace, pattern, extension);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_file_rules_insert_rule_v1(void* handle, size_t ruleIndex, const char* name, const char* colorSpace, const char* regex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)name; (void)colorSpace; (void)regex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->insertRule(ruleIndex, name, colorSpace, regex);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_file_rules_insert_path_search_rule(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->insertPathSearchRule(ruleIndex);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_file_rules_set_default_rule_color_space(void* handle, const char* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)colorSpace;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->setDefaultRuleColorSpace(colorSpace);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_file_rules_remove_rule(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->removeRule(ruleIndex);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_file_rules_increase_rule_priority(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->increaseRulePriority(ruleIndex);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_file_rules_decrease_rule_priority(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_file_rules(handle)->decreaseRulePriority(ruleIndex);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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

void* ocio_file_rules_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_file_rules().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_file_rules(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::FileRulesHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealFileRules>(ocio_rs_bridge::RealFileRules{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

// --- ViewingRules ---

void* ocio_viewing_rules_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_viewing_rules().release();
#else
  try {
    auto result = ocio::ViewingRules::Create();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ViewingRulesHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealViewingRules>(ocio_rs_bridge::RealViewingRules{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_viewing_rules_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_viewing_rules().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_viewing_rules(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ViewingRulesHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealViewingRules>(ocio_rs_bridge::RealViewingRules{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_viewing_rules_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ViewingRulesHandle*>(handle);
}

size_t ocio_viewing_rules_get_num_entries(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_viewing_rules(handle)->getNumEntries();
  } catch (...) { return 0; }
#endif
}

size_t ocio_viewing_rules_get_index_for_rule(void* handle, const char* ruleName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleName;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_viewing_rules(handle)->getIndexForRule(ruleName);
  } catch (...) { return 0; }
#endif
}

void* ocio_viewing_rules_get_name(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_viewing_rules(handle)->getName(ruleIndex)));
  } catch (...) { return nullptr; }
#endif
}

size_t ocio_viewing_rules_get_num_color_spaces(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_viewing_rules(handle)->getNumColorSpaces(ruleIndex);
  } catch (...) { return 0; }
#endif
}

void* ocio_viewing_rules_get_color_space(void* handle, size_t ruleIndex, size_t colorSpaceIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)colorSpaceIndex;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_viewing_rules(handle)->getColorSpace(ruleIndex, colorSpaceIndex)));
  } catch (...) { return nullptr; }
#endif
}

void ocio_viewing_rules_add_color_space(void* handle, size_t ruleIndex, const char* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)colorSpace;
  return;
#else
  try {
    ocio_rs_bridge::get_real_viewing_rules(handle)->addColorSpace(ruleIndex, colorSpace);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_viewing_rules_remove_color_space(void* handle, size_t ruleIndex, size_t colorSpaceIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)colorSpaceIndex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_viewing_rules(handle)->removeColorSpace(ruleIndex, colorSpaceIndex);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

size_t ocio_viewing_rules_get_num_encodings(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_viewing_rules(handle)->getNumEncodings(ruleIndex);
  } catch (...) { return 0; }
#endif
}

void* ocio_viewing_rules_get_encoding(void* handle, size_t ruleIndex, size_t encodingIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)encodingIndex;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_viewing_rules(handle)->getEncoding(ruleIndex, encodingIndex)));
  } catch (...) { return nullptr; }
#endif
}

void ocio_viewing_rules_add_encoding(void* handle, size_t ruleIndex, const char* encoding) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)encoding;
  return;
#else
  try {
    ocio_rs_bridge::get_real_viewing_rules(handle)->addEncoding(ruleIndex, encoding);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_viewing_rules_remove_encoding(void* handle, size_t ruleIndex, size_t encodingIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)encodingIndex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_viewing_rules(handle)->removeEncoding(ruleIndex, encodingIndex);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

size_t ocio_viewing_rules_get_num_custom_keys(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_viewing_rules(handle)->getNumCustomKeys(ruleIndex);
  } catch (...) { return 0; }
#endif
}

void* ocio_viewing_rules_get_custom_key_name(void* handle, size_t ruleIndex, size_t keyIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)keyIndex;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_viewing_rules(handle)->getCustomKeyName(ruleIndex, keyIndex)));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_viewing_rules_get_custom_key_value(void* handle, size_t ruleIndex, size_t keyIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)keyIndex;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_viewing_rules(handle)->getCustomKeyValue(ruleIndex, keyIndex)));
  } catch (...) { return nullptr; }
#endif
}

void ocio_viewing_rules_set_custom_key(void* handle, size_t ruleIndex, const char* key, const char* value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)key; (void)value;
  return;
#else
  try {
    ocio_rs_bridge::get_real_viewing_rules(handle)->setCustomKey(ruleIndex, key, value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_viewing_rules_insert_rule(void* handle, size_t ruleIndex, const char* ruleName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex; (void)ruleName;
  return;
#else
  try {
    ocio_rs_bridge::get_real_viewing_rules(handle)->insertRule(ruleIndex, ruleName);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_viewing_rules_remove_rule(void* handle, size_t ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ruleIndex;
  return;
#else
  try {
    ocio_rs_bridge::get_real_viewing_rules(handle)->removeRule(ruleIndex);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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

void* ocio_color_space_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_color_space().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_color_space(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ColorSpaceHandle>();
    out_handle->inner =
        std::make_shared<ocio_rs_bridge::RealColorSpace>(ocio_rs_bridge::RealColorSpace{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_color_space(handle)->getName()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_color_space(handle)->getAlias(idx)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_color_space_remove_alias(void* handle, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)alias;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->removeAlias(alias);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_color_space_clear_aliases(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->clearAliases();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_color_space_get_family(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_color_space(handle)->getFamily()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_color_space_get_equality_group(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_color_space(handle)->getEqualityGroup()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_color_space_get_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_color_space(handle)->getDescription()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_color_space_get_interop_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_color_space(handle)->getInteropID()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_color_space_set_interchange_attribute(void* handle, const char* attrName, const char* value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)attrName; (void)value;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setInterchangeAttribute(attrName, value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

const char* ocio_color_space_get_interchange_attribute(void* handle, const char* attrName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)attrName;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getInterchangeAttribute(attrName);
  } catch (...) { return nullptr; }
#endif
}

int ocio_color_space_get_num_interchange_attributes(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return static_cast<int>(ocio_rs_bridge::get_real_color_space(handle)->getInterchangeAttributes().size());
  } catch (...) { return 0; }
#endif
}

const char* ocio_color_space_get_interchange_attribute_name_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    const auto attrs = ocio_rs_bridge::get_real_color_space(handle)->getInterchangeAttributes();
    ocio_rs_bridge::g_serialized_text = ocio_rs_bridge::interchange_attribute_name_by_index(attrs, index);
    return ocio_rs_bridge::g_serialized_text.empty() ? nullptr : ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_color_space_get_interchange_attribute_value_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    const auto attrs = ocio_rs_bridge::get_real_color_space(handle)->getInterchangeAttributes();
    ocio_rs_bridge::g_serialized_text = ocio_rs_bridge::interchange_attribute_value_by_index(attrs, index);
    return ocio_rs_bridge::g_serialized_text.empty() ? nullptr : ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_color_space_remove_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->removeCategory(category);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_color_space(handle)->getCategory(index)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_color_space_get_encoding(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_color_space(handle)->getEncoding()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

int ocio_color_space_get_allocation(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getAllocation();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_color_space_set_allocation(void* handle, int allocation) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)allocation;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setAllocation(static_cast<ocio::Allocation>(allocation));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

int ocio_color_space_get_allocation_num_vars(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getAllocationNumVars();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_color_space_get_allocation_vars(void* handle, void* vars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)vars;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->getAllocationVars(static_cast<float*>(vars));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_color_space_set_allocation_vars(void* handle, int numvars, const float* vars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)numvars; (void)vars;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space(handle)->setAllocationVars(numvars, vars);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    auto transform_ptr = _transform_h->get_ocio_transform();
    ocio_rs_bridge::get_real_color_space(handle)->setTransform(transform_ptr, static_cast<ocio::ColorSpaceDirection>(dir));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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

void* ocio_color_space_set_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_color_space_set().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_color_space_set(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ColorSpaceSetHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceSet>(ocio_rs_bridge::RealColorSpaceSet{result});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_color_space_set(handle)->getColorSpaceNameByIndex(index)));
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
  return -1;
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_color_space_set_remove_color_space(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space_set(handle)->removeColorSpace(name);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_color_space_set_clear_color_spaces(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_color_space_set(handle)->clearColorSpaces();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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

void* ocio_look_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_look().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_look(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::LookHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealLook>(ocio_rs_bridge::RealLook{result});
    return out_handle.release();
  } catch (...) { return nullptr; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_look(handle)->getName()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_look_get_process_space(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_look(handle)->getProcessSpace()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    auto transform_ptr = _transform_h->get_ocio_transform();
    ocio_rs_bridge::get_real_look(handle)->setTransform(transform_ptr);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    auto transform_ptr = _transform_h->get_ocio_transform();
    ocio_rs_bridge::get_real_look(handle)->setInverseTransform(transform_ptr);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_look_get_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_look(handle)->getDescription()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_look_set_interchange_attribute(void* handle, const char* attrName, const char* value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)attrName; (void)value;
  return;
#else
  try {
    ocio_rs_bridge::get_real_look(handle)->setInterchangeAttribute(attrName, value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

const char* ocio_look_get_interchange_attribute(void* handle, const char* attrName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)attrName;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_look(handle)->getInterchangeAttribute(attrName);
  } catch (...) { return nullptr; }
#endif
}

int ocio_look_get_num_interchange_attributes(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return static_cast<int>(ocio_rs_bridge::get_real_look(handle)->getInterchangeAttributes().size());
  } catch (...) { return 0; }
#endif
}

const char* ocio_look_get_interchange_attribute_name_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    const auto attrs = ocio_rs_bridge::get_real_look(handle)->getInterchangeAttributes();
    ocio_rs_bridge::g_serialized_text = ocio_rs_bridge::interchange_attribute_name_by_index(attrs, index);
    return ocio_rs_bridge::g_serialized_text.empty() ? nullptr : ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_look_get_interchange_attribute_value_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    const auto attrs = ocio_rs_bridge::get_real_look(handle)->getInterchangeAttributes();
    ocio_rs_bridge::g_serialized_text = ocio_rs_bridge::interchange_attribute_value_by_index(attrs, index);
    return ocio_rs_bridge::g_serialized_text.empty() ? nullptr : ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { return nullptr; }
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

void* ocio_named_transform_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_named_transform().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_named_transform(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::NamedTransformHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealNamedTransform>(ocio_rs_bridge::RealNamedTransform{result});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_named_transform_get_name(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_named_transform(handle)->getName()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_named_transform(handle)->getAlias(idx)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_named_transform_remove_alias(void* handle, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)alias;
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->removeAlias(alias);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_named_transform_clear_aliases(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->clearAliases();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_named_transform_get_family(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_named_transform(handle)->getFamily()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_named_transform_get_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_named_transform(handle)->getDescription()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_named_transform_remove_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return;
#else
  try {
    ocio_rs_bridge::get_real_named_transform(handle)->removeCategory(category);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_named_transform(handle)->getCategory(index)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_named_transform_get_encoding(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_named_transform(handle)->getEncoding()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    auto transform_ptr = _transform_h->get_ocio_transform();
    ocio_rs_bridge::get_real_named_transform(handle)->setTransform(transform_ptr, static_cast<ocio::TransformDirection>(dir));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- ViewTransform ---

void* ocio_view_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_view_transform().release();
#else
  return ocio_view_transform_create_with_reference_space(static_cast<int>(ocio::REFERENCE_SPACE_SCENE));
#endif
}

void* ocio_view_transform_create_with_reference_space(int referenceSpace) {
#ifdef OCIO_RS_STUB
  (void)referenceSpace;
  return ocio_rs_bridge::make_stub_view_transform().release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::ViewTransformHandle>();
    auto obj = std::make_shared<ocio_rs_bridge::RealViewTransform>();
    obj->transform = ocio::ViewTransform::Create(static_cast<ocio::ReferenceSpaceType>(referenceSpace));
    handle->inner = obj;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_view_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ViewTransformHandle*>(handle);
}

void* ocio_view_transform_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_view_transform().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_view_transform(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ViewTransformHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealViewTransform>(
        ocio_rs_bridge::RealViewTransform{result});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_view_transform_get_name(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_view_transform(handle)->getName()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_view_transform_get_family(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_view_transform(handle)->getFamily()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_view_transform_get_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_view_transform(handle)->getDescription()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_view_transform_set_interchange_attribute(void* handle, const char* attrName, const char* value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)attrName; (void)value;
  return;
#else
  try {
    ocio_rs_bridge::get_real_view_transform(handle)->setInterchangeAttribute(attrName, value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

const char* ocio_view_transform_get_interchange_attribute(void* handle, const char* attrName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)attrName;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_view_transform(handle)->getInterchangeAttribute(attrName);
  } catch (...) { return nullptr; }
#endif
}

int ocio_view_transform_get_num_interchange_attributes(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return static_cast<int>(ocio_rs_bridge::get_real_view_transform(handle)->getInterchangeAttributes().size());
  } catch (...) { return 0; }
#endif
}

const char* ocio_view_transform_get_interchange_attribute_name_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    const auto attrs = ocio_rs_bridge::get_real_view_transform(handle)->getInterchangeAttributes();
    ocio_rs_bridge::g_serialized_text = ocio_rs_bridge::interchange_attribute_name_by_index(attrs, index);
    return ocio_rs_bridge::g_serialized_text.empty() ? nullptr : ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_view_transform_get_interchange_attribute_value_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    const auto attrs = ocio_rs_bridge::get_real_view_transform(handle)->getInterchangeAttributes();
    ocio_rs_bridge::g_serialized_text = ocio_rs_bridge::interchange_attribute_value_by_index(attrs, index);
    return ocio_rs_bridge::g_serialized_text.empty() ? nullptr : ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_view_transform_remove_category(void* handle, const char* category) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)category;
  return;
#else
  try {
    ocio_rs_bridge::get_real_view_transform(handle)->removeCategory(category);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_view_transform(handle)->getCategory(index)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
    ocio::TransformRcPtr transform_ptr;
    if (transform) {
      auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
      transform_ptr = _transform_h->get_ocio_transform();
    }
    ocio_rs_bridge::get_real_view_transform(handle)->setTransform(transform_ptr, static_cast<ocio::ViewTransformDirection>(dir));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

bool ocio_color_space_is_transform_defined(void* handle, int direction) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)direction;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_color_space(handle)->getTransform(static_cast<ocio::ColorSpaceDirection>(direction)) != nullptr;
  } catch (...) { return false; }
#endif
}

void* ocio_transform_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return nullptr;
#else
  try {
    if (!handle) return nullptr;
    auto* transform_handle = static_cast<ocio_rs_bridge::TransformHandleBase*>(handle);
    auto transform = transform_handle->get_ocio_transform();
    if (!transform) return nullptr;
    return ocio_rs_bridge::wrap_editable_transform_copy(transform->createEditableCopy());
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

int ocio_transform_get_transform_type(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return -1;
#else
  try {
    if (!handle) return -1;
    auto* transform = static_cast<ocio_rs_bridge::TransformHandleBase*>(handle);
    return transform->get_transform_type_tag();
  } catch (...) { return -1; }
#endif
}


// --- Processor ---

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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_processor_has_channel_crosstalk(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_processor(handle)->hasChannelCrosstalk();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

void* ocio_processor_get_cache_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_processor(handle)->getCacheID()));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_processor_get_processor_metadata(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return ocio_rs_bridge::make_stub_processor_metadata().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getProcessorMetadata();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorMetadataHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::ProcessorMetadata>(result);
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessorMetadata>(ocio_rs_bridge::RealProcessorMetadata{result_unconst});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_processor_get_format_metadata(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return ocio_rs_bridge::make_stub_format_metadata().release();
#else
  try {
    auto processor = ocio_rs_bridge::get_real_processor(handle);
    auto owner = std::make_shared<ocio::ProcessorRcPtr>(processor);
    return ocio_rs_bridge::make_format_metadata_handle(owner, &((*owner)->getFormatMetadata()));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  return ocio_rs_bridge::make_stub_format_metadata().release();
#else
  try {
    auto processor = ocio_rs_bridge::get_real_processor(handle);
    auto owner = std::make_shared<ocio::ProcessorRcPtr>(processor);
    return ocio_rs_bridge::make_format_metadata_handle(
      owner,
      &((*owner)->getTransformFormatMetadata(index)));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_processor_create_group_transform(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return ocio_rs_bridge::make_stub_group_transform().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->createGroupTransform();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::GroupTransformHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGroupTransform>(ocio_rs_bridge::RealGroupTransform{result});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

bool ocio_processor_has_dynamic_property(void* handle, int type) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_processor(handle)->hasDynamicProperty(static_cast<ocio::DynamicPropertyType>(type));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_processor_is_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_processor(handle)->isDynamic();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

void ocio_processor_apply_rgba(void* handle, float* rgba, size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgba; (void)len;
  return;
#else
  try {
    if (!rgba || len < 4) return;
    auto cpu = ocio_rs_bridge::get_real_processor(handle)->getDefaultCPUProcessor();
    if (cpu) cpu->applyRGBA(rgba);
  } catch (...) { return; }
#endif
}

void ocio_processor_apply_rgba_pixels(void* handle, float* rgba, int64_t numPixels, int64_t stride) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgba; (void)numPixels; (void)stride;
  return;
#else
  try {
    if (!rgba || numPixels <= 0) return;
    auto cpu = ocio_rs_bridge::get_real_processor(handle)->getDefaultCPUProcessor();
    if (!cpu) return;
    const ptrdiff_t channelStride = static_cast<ptrdiff_t>(sizeof(float));
    const ptrdiff_t xStride = static_cast<ptrdiff_t>((stride > 0 ? stride : 4) * static_cast<int64_t>(sizeof(float)));
    ocio::PackedImageDesc img(rgba, static_cast<long>(numPixels), 1L, ocio::CHANNEL_ORDERING_RGBA,
                              ocio::BIT_DEPTH_F32, channelStride, xStride, xStride * numPixels);
    cpu->apply(img);
  } catch (...) { return; }
#endif
}

void* ocio_processor_get_optimized_processor_v1(void* handle, int oFlags) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)oFlags;
  return ocio_rs_bridge::make_stub_processor().release();
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
  return ocio_rs_bridge::make_stub_processor().release();
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

void* ocio_processor_optimized_processor(void* handle, int oFlags) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)oFlags;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getOptimizedProcessor(
        static_cast<ocio::OptimizationFlags>(oFlags));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    auto result_unconst = std::const_pointer_cast<ocio::Processor>(result);
    out_handle->inner =
        std::make_shared<ocio_rs_bridge::RealProcessor>(ocio_rs_bridge::RealProcessor{result_unconst});
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_default_gpu_processor(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return ocio_rs_bridge::make_stub_gpu_processor().release();
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
  return ocio_rs_bridge::make_stub_gpu_processor().release();
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

void* ocio_processor_get_optimized_legacy_gpu_processor(void* handle, int oFlags, unsigned edgelen) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)oFlags; (void)edgelen;
  return ocio_rs_bridge::make_stub_gpu_processor().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_processor(handle)->getOptimizedLegacyGPUProcessor(
        static_cast<ocio::OptimizationFlags>(oFlags), edgelen);
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
  return ocio_rs_bridge::make_stub_cpu_processor().release();
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
  return ocio_rs_bridge::make_stub_cpu_processor().release();
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

// --- ProcessorMetadata ---

void* ocio_processor_metadata_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_processor_metadata().release();
#else
  try {
    auto result = ocio::ProcessorMetadata::Create();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ProcessorMetadataHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealProcessorMetadata>(ocio_rs_bridge::RealProcessorMetadata{result});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_processor_metadata_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ProcessorMetadataHandle*>(handle);
}

int ocio_processor_metadata_get_num_files(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_processor_metadata(handle)->getNumFiles();
  } catch (...) { return 0; }
#endif
}

void* ocio_processor_metadata_get_file(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_processor_metadata(handle)->getFile(index)));
  } catch (...) { return nullptr; }
#endif
}

int ocio_processor_metadata_get_num_looks(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_processor_metadata(handle)->getNumLooks();
  } catch (...) { return 0; }
#endif
}

void* ocio_processor_metadata_get_look(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_processor_metadata(handle)->getLook(index)));
  } catch (...) { return nullptr; }
#endif
}

void ocio_processor_metadata_add_file(void* handle, const char* fileName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)fileName;
  return;
#else
  try {
    ocio_rs_bridge::get_real_processor_metadata(handle)->addFile(fileName);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_processor_metadata_add_look(void* handle, const char* look) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)look;
  return;
#else
  try {
    ocio_rs_bridge::get_real_processor_metadata(handle)->addLook(look);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

// --- CPUProcessor ---

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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_cpu_processor_is_identity(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->isIdentity();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_cpu_processor_has_channel_crosstalk(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->hasChannelCrosstalk();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

void* ocio_cpu_processor_get_cache_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_cpu_processor(handle)->getCacheID()));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

int ocio_cpu_processor_get_input_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->getInputBitDepth();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

int ocio_cpu_processor_get_output_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->getOutputBitDepth();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

bool ocio_cpu_processor_has_dynamic_property(void* handle, int type) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->hasDynamicProperty(static_cast<ocio::DynamicPropertyType>(type));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_cpu_processor_is_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(handle)->isDynamic();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

void ocio_cpu_processor_apply(void* handle, void* imgDesc) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)imgDesc;
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    ocio_rs_bridge::get_real_cpu_processor(handle)->apply(*static_cast<const ocio::ImageDesc*>(imgDesc));
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void ocio_cpu_processor_apply_v1(void* handle, void* imgDesc) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)imgDesc;
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    ocio_rs_bridge::get_real_cpu_processor(handle)->apply(*static_cast<const ocio::ImageDesc*>(imgDesc));
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void ocio_cpu_processor_apply_v2(void* handle, void* srcImgDesc, void* dstImgDesc) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)srcImgDesc; (void)dstImgDesc;
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    ocio_rs_bridge::get_real_cpu_processor(handle)->apply(*static_cast<const ocio::ImageDesc*>(srcImgDesc), *static_cast<ocio::ImageDesc*>(dstImgDesc));
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void ocio_cpu_processor_apply_rgb(void* handle, void* pixel) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)pixel;
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    ocio_rs_bridge::get_real_cpu_processor(handle)->applyRGB(static_cast<float*>(pixel));
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void ocio_cpu_processor_apply_rgba(void* handle, void* pixel) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)pixel;
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    ocio_rs_bridge::get_real_cpu_processor(handle)->applyRGBA(static_cast<float*>(pixel));
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void ocio_cpu_processor_apply_rgba_pixels(void* handle, float* rgba, int64_t numPixels, int64_t stride) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgba; (void)numPixels; (void)stride;
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    if (!rgba || numPixels <= 0) return;
    const ptrdiff_t channelStride = static_cast<ptrdiff_t>(sizeof(float));
    const ptrdiff_t xStride = static_cast<ptrdiff_t>((stride > 0 ? stride : 4) * static_cast<int64_t>(sizeof(float)));
    ocio::PackedImageDesc img(rgba, static_cast<long>(numPixels), 1L, ocio::CHANNEL_ORDERING_RGBA,
                              ocio::BIT_DEPTH_F32, channelStride, xStride, xStride * numPixels);
    ocio_rs_bridge::get_real_cpu_processor(handle)->apply(img);
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void ocio_cpu_processor_apply_rgb_pixels(void* handle, float* rgb, int64_t numPixels, int64_t stride) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb; (void)numPixels; (void)stride;
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    if (!rgb || numPixels <= 0) return;
    const ptrdiff_t channelStride = static_cast<ptrdiff_t>(sizeof(float));
    const ptrdiff_t xStride = static_cast<ptrdiff_t>((stride > 0 ? stride : 3) * static_cast<int64_t>(sizeof(float)));
    ocio::PackedImageDesc img(rgb, static_cast<long>(numPixels), 1L, ocio::CHANNEL_ORDERING_RGB,
                              ocio::BIT_DEPTH_F32, channelStride, xStride, xStride * numPixels);
    ocio_rs_bridge::get_real_cpu_processor(handle)->apply(img);
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void ocio_cpu_processor_apply_rgba_packed(void* handle, void* rgba, int bitDepth, int64_t numPixels, int64_t stride) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgba; (void)bitDepth; (void)numPixels; (void)stride;
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    if (!rgba || numPixels <= 0) return;
    const size_t bytesPerChannel = (bitDepth == ocio::BIT_DEPTH_F32) ? sizeof(float)
      : (bitDepth == ocio::BIT_DEPTH_F16 || bitDepth == ocio::BIT_DEPTH_UINT16
         || bitDepth == ocio::BIT_DEPTH_UINT10 || bitDepth == ocio::BIT_DEPTH_UINT12
         || bitDepth == ocio::BIT_DEPTH_UINT14) ? 2u
      : (bitDepth == ocio::BIT_DEPTH_UINT32) ? 4u : 1u;
    const ptrdiff_t channelStride = static_cast<ptrdiff_t>(bytesPerChannel);
    const ptrdiff_t xStride = static_cast<ptrdiff_t>((stride > 0 ? stride : 4) * static_cast<int64_t>(bytesPerChannel));
    ocio::PackedImageDesc img(rgba, static_cast<long>(numPixels), 1L, ocio::CHANNEL_ORDERING_RGBA,
                              static_cast<ocio::BitDepth>(bitDepth), channelStride, xStride, xStride * numPixels);
    ocio_rs_bridge::get_real_cpu_processor(handle)->apply(img);
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void ocio_cpu_processor_apply_rgb_packed(void* handle, void* rgb, int bitDepth, int64_t numPixels, int64_t stride) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb; (void)bitDepth; (void)numPixels; (void)stride;
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    if (!rgb || numPixels <= 0) return;
    const size_t bytesPerChannel = (bitDepth == ocio::BIT_DEPTH_F32) ? sizeof(float)
      : (bitDepth == ocio::BIT_DEPTH_F16 || bitDepth == ocio::BIT_DEPTH_UINT16
         || bitDepth == ocio::BIT_DEPTH_UINT10 || bitDepth == ocio::BIT_DEPTH_UINT12
         || bitDepth == ocio::BIT_DEPTH_UINT14) ? 2u
      : (bitDepth == ocio::BIT_DEPTH_UINT32) ? 4u : 1u;
    const ptrdiff_t channelStride = static_cast<ptrdiff_t>(bytesPerChannel);
    const ptrdiff_t xStride = static_cast<ptrdiff_t>((stride > 0 ? stride : 3) * static_cast<int64_t>(bytesPerChannel));
    ocio::PackedImageDesc img(rgb, static_cast<long>(numPixels), 1L, ocio::CHANNEL_ORDERING_RGB,
                              static_cast<ocio::BitDepth>(bitDepth), channelStride, xStride, xStride * numPixels);
    ocio_rs_bridge::get_real_cpu_processor(handle)->apply(img);
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}


// --- GPUProcessor ---

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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_gpu_processor_has_channel_crosstalk(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_processor(handle)->hasChannelCrosstalk();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

void* ocio_gpu_processor_get_cache_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_gpu_processor(handle)->getCacheID()));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_gpu_processor_extract_gpu_shader_info(void* handle, void* shaderDesc) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shaderDesc;
  return;
#else
  try {
    auto* _shaderDesc_h = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shaderDesc);
    auto shaderDesc_ptr = std::static_pointer_cast<ocio_rs_bridge::RealGpuShaderDesc>(_shaderDesc_h->inner)->gpuShaderDesc;
    ocio_rs_bridge::get_real_gpu_processor(handle)->extractGpuShaderInfo(shaderDesc_ptr);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}


// --- GpuShaderDesc ---

void* ocio_gpu_shader_desc_create_shader_desc(void) {
  return ocio_gpu_shader_desc_create();
}

void* ocio_gpu_shader_desc_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_gpu_shader_desc().release();
#else
  try {
    auto result = ocio::GpuShaderDesc::CreateShaderDesc();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::GpuShaderDescHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGpuShaderDesc>(ocio_rs_bridge::RealGpuShaderDesc{result});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
    auto desc = std::dynamic_pointer_cast<ocio::GpuShaderDesc>(result);
    if (!desc) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::GpuShaderDescHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGpuShaderDesc>(ocio_rs_bridge::RealGpuShaderDesc{desc});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

unsigned ocio_gpu_shader_desc_get_num_uniforms_u32(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getNumUniforms();
  } catch (...) { return 0; }
#endif
}

bool ocio_gpu_shader_desc_get_uniform_info(void* handle, unsigned index, OcioGpuUniformInfo* out) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)out;
  return false;
#else
  try {
    if (!out) return false;
    ocio::GpuShaderDesc::UniformData data;
    const char* name = ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getUniform(index, data);
    out->name = name;
    out->type = static_cast<int>(data.m_type);
    out->buffer_offset = data.m_bufferOffset;
    out->value_count = 0;
    switch (data.m_type) {
      case ocio::UNIFORM_DOUBLE:
      case ocio::UNIFORM_BOOL:
        out->value_count = 1;
        break;
      case ocio::UNIFORM_FLOAT3:
        out->value_count = 3;
        break;
      case ocio::UNIFORM_VECTOR_FLOAT:
        out->value_count = data.m_vectorFloat.m_getSize ? static_cast<size_t>(data.m_vectorFloat.m_getSize()) : 0;
        break;
      case ocio::UNIFORM_VECTOR_INT:
        out->value_count = data.m_vectorInt.m_getSize ? static_cast<size_t>(data.m_vectorInt.m_getSize()) : 0;
        break;
      default:
        out->value_count = 0;
        break;
    }
    return name != nullptr;
  } catch (...) { return false; }
#endif
}

size_t ocio_gpu_shader_desc_get_uniform_value_count(void* handle, unsigned index) {
  OcioGpuUniformInfo info{};
  return ocio_gpu_shader_desc_get_uniform_info(handle, index, &info) ? info.value_count : 0;
}

bool ocio_gpu_shader_desc_copy_uniform_f32_values(void* handle, unsigned index, float* values, size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)values; (void)len;
  return false;
#else
  try {
    if (!values) return false;
    ocio::GpuShaderDesc::UniformData data;
    const char* name = ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getUniform(index, data);
    if (!name) return false;
    switch (data.m_type) {
      case ocio::UNIFORM_DOUBLE:
        if (len < 1 || !data.m_getDouble) return false;
        values[0] = static_cast<float>(data.m_getDouble());
        return true;
      case ocio::UNIFORM_BOOL:
        if (len < 1 || !data.m_getBool) return false;
        values[0] = data.m_getBool() ? 1.0f : 0.0f;
        return true;
      case ocio::UNIFORM_FLOAT3: {
        if (len < 3 || !data.m_getFloat3) return false;
        const auto& rgb = data.m_getFloat3();
        values[0] = rgb[0];
        values[1] = rgb[1];
        values[2] = rgb[2];
        return true;
      }
      case ocio::UNIFORM_VECTOR_FLOAT: {
        if (!data.m_vectorFloat.m_getSize || !data.m_vectorFloat.m_getVector) return false;
        const size_t count = static_cast<size_t>(data.m_vectorFloat.m_getSize());
        if (len < count) return false;
        const float* src = data.m_vectorFloat.m_getVector();
        if (!src && count > 0) return false;
        for (size_t i = 0; i < count; ++i) values[i] = src[i];
        return true;
      }
      default:
        return false;
    }
  } catch (...) { return false; }
#endif
}

bool ocio_gpu_shader_desc_copy_uniform_i32_values(void* handle, unsigned index, int* values, size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)values; (void)len;
  return false;
#else
  try {
    if (!values) return false;
    ocio::GpuShaderDesc::UniformData data;
    const char* name = ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getUniform(index, data);
    if (!name || data.m_type != ocio::UNIFORM_VECTOR_INT) return false;
    if (!data.m_vectorInt.m_getSize || !data.m_vectorInt.m_getVector) return false;
    const size_t count = static_cast<size_t>(data.m_vectorInt.m_getSize());
    if (len < count) return false;
    const int* src = data.m_vectorInt.m_getVector();
    if (!src && count > 0) return false;
    for (size_t i = 0; i < count; ++i) values[i] = src[i];
    return true;
  } catch (...) { return false; }
#endif
}

size_t ocio_gpu_shader_desc_get_uniform_buffer_size_bytes(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getUniformBufferSize();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

bool ocio_gpu_shader_desc_add_uniform_double(void* handle, const char* name, double value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name; (void)value;
  return false;
#else
  try {
    auto owned = std::make_shared<double>(value);
    ocio::GpuShaderCreator::DoubleGetter getter = [owned]() { return *owned; };
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addUniform(
      name,
      getter);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_gpu_shader_desc_add_uniform_bool(void* handle, const char* name, bool value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name; (void)value;
  return false;
#else
  try {
    auto owned = std::make_shared<bool>(value);
    ocio::GpuShaderCreator::BoolGetter getter = [owned]() { return *owned; };
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addUniform(
      name,
      getter);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_gpu_shader_desc_add_uniform_float3(void* handle, const char* name, float x, float y, float z) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name; (void)x; (void)y; (void)z;
  return false;
#else
  try {
    auto owned = std::make_shared<ocio::Float3>();
    (*owned)[0] = x;
    (*owned)[1] = y;
    (*owned)[2] = z;
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addUniform(
      name,
      [owned]() -> const ocio::Float3& { return *owned; });
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_gpu_shader_desc_add_uniform_vector_float(
    void* handle,
    const char* name,
    const float* values,
    size_t len,
    uint32_t maxSize) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name; (void)values; (void)len; (void)maxSize;
  return false;
#else
  try {
    if ((!values && len > 0) || maxSize < len) return false;
    auto owned = std::make_shared<std::vector<float>>(values, values + len);
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addUniform(
      name,
      [owned]() { return static_cast<int>(owned->size()); },
      [owned]() { return owned->empty() ? nullptr : owned->data(); },
      maxSize);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_gpu_shader_desc_add_uniform_vector_int(
    void* handle,
    const char* name,
    const int* values,
    size_t len,
    uint32_t maxSize) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name; (void)values; (void)len; (void)maxSize;
  return false;
#else
  try {
    if ((!values && len > 0) || maxSize < len) return false;
    auto owned = std::make_shared<std::vector<int>>(values, values + len);
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addUniform(
      name,
      [owned]() { return static_cast<int>(owned->size()); },
      [owned]() { return owned->empty() ? nullptr : owned->data(); },
      maxSize);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

unsigned ocio_gpu_shader_desc_get_num_dynamic_properties_u32(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getNumDynamicProperties();
  } catch (...) { return 0; }
#endif
}

void* ocio_gpu_shader_desc_get_dynamic_property_by_index(void* handle, unsigned index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getDynamicProperty(index);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::DynamicPropertyHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealDynamicProperty>(
        ocio_rs_bridge::RealDynamicProperty{result});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_dynamic_property(void* handle, int type) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type;
  return nullptr;
#else
  try {
    auto result = ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getDynamicProperty(
        static_cast<ocio::DynamicPropertyType>(type));
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::DynamicPropertyHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealDynamicProperty>(
        ocio_rs_bridge::RealDynamicProperty{result});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

bool ocio_gpu_shader_desc_has_dynamic_property(void* handle, int type) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)type;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->hasDynamicProperty(
        static_cast<ocio::DynamicPropertyType>(type));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

uint32_t ocio_gpu_shader_desc_add_texture(
    void* handle,
    const char* textureName,
    const char* samplerName,
    uint32_t width,
    uint32_t height,
    int channel,
    int dimensions,
    int interpolation,
    const float* values,
    size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)textureName; (void)samplerName; (void)width; (void)height;
  (void)channel; (void)dimensions; (void)interpolation; (void)values; (void)len;
  return 0;
#else
  try {
    const size_t channels = channel == static_cast<int>(ocio::GpuShaderDesc::TEXTURE_RED_CHANNEL) ? 1 : 3;
    const size_t expected = static_cast<size_t>(width) * static_cast<size_t>(height) * channels;
    if (!values || len < expected) return 0;
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addTexture(
      textureName,
      samplerName,
      width,
      height,
      static_cast<ocio::GpuShaderDesc::TextureType>(channel),
      static_cast<ocio::GpuShaderDesc::TextureDimensions>(dimensions),
      static_cast<ocio::Interpolation>(interpolation),
      values);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

unsigned ocio_gpu_shader_desc_get_num_textures_u32(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getNumTextures();
  } catch (...) { return 0; }
#endif
}

bool ocio_gpu_shader_desc_get_texture_info(void* handle, unsigned index, OcioGpuTexture2DInfo* out) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)out;
  return false;
#else
  try {
    if (!out) return false;
    const char* textureName = nullptr;
    const char* samplerName = nullptr;
    unsigned width = 0;
    unsigned height = 0;
    ocio::GpuShaderDesc::TextureType channel = ocio::GpuShaderDesc::TEXTURE_RGB_CHANNEL;
    ocio::GpuShaderDesc::TextureDimensions dimensions = ocio::GpuShaderDesc::TEXTURE_1D;
    ocio::Interpolation interpolation = ocio::INTERP_LINEAR;
    auto desc = ocio_rs_bridge::get_real_gpu_shader_desc(handle);
    desc->getTexture(index, textureName, samplerName, width, height, channel, dimensions, interpolation);
    out->texture_name = textureName;
    out->sampler_name = samplerName;
    out->width = width;
    out->height = height;
    out->channel = static_cast<int>(channel);
    out->dimensions = static_cast<int>(dimensions);
    out->interpolation = static_cast<int>(interpolation);
    out->binding_index = desc->getTextureShaderBindingIndex(index);
    return textureName && samplerName && width > 0;
  } catch (...) { return false; }
#endif
}

size_t ocio_gpu_shader_desc_get_texture_value_count(void* handle, unsigned index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return 0;
#else
  OcioGpuTexture2DInfo info{};
  if (!ocio_gpu_shader_desc_get_texture_info(handle, index, &info)) return 0;
  const size_t channels = info.channel == ocio::GpuShaderDesc::TEXTURE_RED_CHANNEL ? 1 : 3;
  return static_cast<size_t>(info.width) * static_cast<size_t>(info.height) * channels;
#endif
}

bool ocio_gpu_shader_desc_copy_texture_values(void* handle, unsigned index, float* values, size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)values; (void)len;
  return false;
#else
  try {
    if (!values) return false;
    const size_t expected = ocio_gpu_shader_desc_get_texture_value_count(handle, index);
    if (expected == 0 || len < expected) return false;
    const float* src = nullptr;
    ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getTextureValues(index, src);
    if (!src) return false;
    for (size_t i = 0; i < expected; ++i) values[i] = src[i];
    return true;
  } catch (...) { return false; }
#endif
}

uint32_t ocio_gpu_shader_desc_add3d_texture(
    void* handle,
    const char* textureName,
    const char* samplerName,
    uint32_t edgeLen,
    int interpolation,
    const float* values,
    size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)textureName; (void)samplerName; (void)edgeLen;
  (void)interpolation; (void)values; (void)len;
  return 0;
#else
  try {
    const size_t edge = static_cast<size_t>(edgeLen);
    const size_t expected = edge * edge * edge * 3;
    if (!values || len < expected) return 0;
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->add3DTexture(
      textureName,
      samplerName,
      edgeLen,
      static_cast<ocio::Interpolation>(interpolation),
      values);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

unsigned ocio_gpu_shader_desc_get_num3d_textures_u32(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getNum3DTextures();
  } catch (...) { return 0; }
#endif
}

bool ocio_gpu_shader_desc_get3d_texture_info(void* handle, unsigned index, OcioGpuTexture3DInfo* out) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)out;
  return false;
#else
  try {
    if (!out) return false;
    const char* textureName = nullptr;
    const char* samplerName = nullptr;
    unsigned edgeLen = 0;
    ocio::Interpolation interpolation = ocio::INTERP_LINEAR;
    auto desc = ocio_rs_bridge::get_real_gpu_shader_desc(handle);
    desc->get3DTexture(index, textureName, samplerName, edgeLen, interpolation);
    out->texture_name = textureName;
    out->sampler_name = samplerName;
    out->edge_len = edgeLen;
    out->interpolation = static_cast<int>(interpolation);
    out->binding_index = desc->get3DTextureShaderBindingIndex(index);
    return textureName && samplerName && edgeLen > 0;
  } catch (...) { return false; }
#endif
}

size_t ocio_gpu_shader_desc_get3d_texture_value_count(void* handle, unsigned index) {
  OcioGpuTexture3DInfo info{};
  if (!ocio_gpu_shader_desc_get3d_texture_info(handle, index, &info)) return 0;
  const size_t edge = static_cast<size_t>(info.edge_len);
  return edge * edge * edge * 3;
}

bool ocio_gpu_shader_desc_copy3d_texture_values(void* handle, unsigned index, float* values, size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)values; (void)len;
  return false;
#else
  try {
    if (!values) return false;
    const size_t expected = ocio_gpu_shader_desc_get3d_texture_value_count(handle, index);
    if (expected == 0 || len < expected) return false;
    const float* src = nullptr;
    ocio_rs_bridge::get_real_gpu_shader_desc(handle)->get3DTextureValues(index, src);
    if (!src) return false;
    for (size_t i = 0; i < expected; ++i) values[i] = src[i];
    return true;
  } catch (...) { return false; }
#endif
}

void* ocio_gpu_shader_desc_get_num_uniforms(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return reinterpret_cast<void*>(static_cast<uintptr_t>(
        ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getNumUniforms()));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_uniform(void* handle, void* index, void* data) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)data;
  return nullptr;
#else
  try {
    return const_cast<char*>(ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getUniform(
        static_cast<unsigned>(reinterpret_cast<uintptr_t>(index)),
        *static_cast<ocio::GpuShaderDesc::UniformData*>(data)));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_uniform_buffer_size(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return reinterpret_cast<void*>(static_cast<uintptr_t>(
        ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getUniformBufferSize()));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_num_textures(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return reinterpret_cast<void*>(static_cast<uintptr_t>(
        ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getNumTextures()));
  } catch (...) { return nullptr; }
#endif
}

void ocio_gpu_shader_desc_get_texture(void* handle, void* index, const char* textureName, const char* samplerName, void* width, void* height, void* channel, void* dimensions, void* interpolation) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)textureName; (void)samplerName; (void)width; (void)height; (void)channel; (void)dimensions; (void)interpolation;
  return;
#else
  try {
    if (!width || !height || !channel || !dimensions || !interpolation) return;
    const char* localTextureName = nullptr;
    const char* localSamplerName = nullptr;
    ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getTexture(
        static_cast<unsigned>(reinterpret_cast<uintptr_t>(index)),
        localTextureName,
        localSamplerName,
        *static_cast<unsigned*>(width),
        *static_cast<unsigned*>(height),
        *static_cast<ocio::GpuShaderDesc::TextureType*>(channel),
        *static_cast<ocio::GpuShaderDesc::TextureDimensions*>(dimensions),
        *static_cast<ocio::Interpolation*>(interpolation));
    if (textureName) {
      *reinterpret_cast<const char**>(const_cast<char*>(textureName)) = localTextureName;
    }
    if (samplerName) {
      *reinterpret_cast<const char**>(const_cast<char*>(samplerName)) = localSamplerName;
    }
  } catch (...) { return ; }
#endif
}

void ocio_gpu_shader_desc_get_texture_values(void* handle, void* index, const float* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)values;
  return;
#else
  try {
    const float* localValues = nullptr;
    ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getTextureValues(
        static_cast<unsigned>(reinterpret_cast<uintptr_t>(index)), localValues);
    if (values) {
      *reinterpret_cast<const float**>(const_cast<float*>(values)) = localValues;
    }
  } catch (...) { return ; }
#endif
}

void* ocio_gpu_shader_desc_get_texture_shader_binding_index(void* handle, void* index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return reinterpret_cast<void*>(static_cast<uintptr_t>(
        ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getTextureShaderBindingIndex(
            static_cast<unsigned>(reinterpret_cast<uintptr_t>(index)))));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_num3d_textures(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return reinterpret_cast<void*>(static_cast<uintptr_t>(
        ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getNum3DTextures()));
  } catch (...) { return nullptr; }
#endif
}

void ocio_gpu_shader_desc_get3d_texture(void* handle, void* index, const char* textureName, const char* samplerName, void* edgelen, void* interpolation) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)textureName; (void)samplerName; (void)edgelen; (void)interpolation;
  return;
#else
  try {
    if (!edgelen || !interpolation) return;
    const char* localTextureName = nullptr;
    const char* localSamplerName = nullptr;
    ocio_rs_bridge::get_real_gpu_shader_desc(handle)->get3DTexture(
        static_cast<unsigned>(reinterpret_cast<uintptr_t>(index)),
        localTextureName,
        localSamplerName,
        *static_cast<unsigned*>(edgelen),
        *static_cast<ocio::Interpolation*>(interpolation));
    if (textureName) {
      *reinterpret_cast<const char**>(const_cast<char*>(textureName)) = localTextureName;
    }
    if (samplerName) {
      *reinterpret_cast<const char**>(const_cast<char*>(samplerName)) = localSamplerName;
    }
  } catch (...) { return ; }
#endif
}

void ocio_gpu_shader_desc_get3d_texture_values(void* handle, void* index, const float* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)values;
  return;
#else
  try {
    const float* localValues = nullptr;
    ocio_rs_bridge::get_real_gpu_shader_desc(handle)->get3DTextureValues(
        static_cast<unsigned>(reinterpret_cast<uintptr_t>(index)), localValues);
    if (values) {
      *reinterpret_cast<const float**>(const_cast<float*>(values)) = localValues;
    }
  } catch (...) { return ; }
#endif
}

void* ocio_gpu_shader_desc_get3d_texture_shader_binding_index(void* handle, void* index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return reinterpret_cast<void*>(static_cast<uintptr_t>(
        ocio_rs_bridge::get_real_gpu_shader_desc(handle)->get3DTextureShaderBindingIndex(
            static_cast<unsigned>(reinterpret_cast<uintptr_t>(index)))));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_gpu_shader_desc_get_shader_text(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getShaderText()));
  } catch (...) { return nullptr; }
#endif
}

int ocio_gpu_shader_desc_get_language(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try { return static_cast<int>(ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getLanguage()); }
  catch (...) { return 0; }
#endif
}

void ocio_gpu_shader_desc_set_language(void* handle, int language) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)language;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->setLanguage(static_cast<ocio::GpuLanguage>(language)); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

const char* ocio_gpu_shader_desc_get_function_name(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return nullptr;
#else
  try { return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getFunctionName(); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_gpu_shader_desc_set_function_name(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->setFunctionName(name); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

const char* ocio_gpu_shader_desc_get_pixel_name(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return nullptr;
#else
  try { return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getPixelName(); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_gpu_shader_desc_set_pixel_name(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->setPixelName(name); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

const char* ocio_gpu_shader_desc_get_unique_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return nullptr;
#else
  try { return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getUniqueID(); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_gpu_shader_desc_set_unique_id(void* handle, const char* uid) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)uid;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->setUniqueID(uid); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

const char* ocio_gpu_shader_desc_get_resource_prefix(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return nullptr;
#else
  try { return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getResourcePrefix(); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_gpu_shader_desc_set_resource_prefix(void* handle, const char* prefix) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)prefix;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->setResourcePrefix(prefix); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_gpu_shader_desc_set_descriptor_set_index(void* handle, uint32_t index, uint32_t textureBindingStart) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)textureBindingStart;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->setDescriptorSetIndex(index, textureBindingStart); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

uint32_t ocio_gpu_shader_desc_get_descriptor_set_index(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try { return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getDescriptorSetIndex(); }
  catch (...) { return 0; }
#endif
}

uint32_t ocio_gpu_shader_desc_get_texture_binding_start(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try { return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getTextureBindingStart(); }
  catch (...) { return 0; }
#endif
}

void ocio_gpu_shader_desc_set_texture_max_width_u32(void* handle, uint32_t maxWidth) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)maxWidth;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->setTextureMaxWidth(maxWidth); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_gpu_shader_desc_set_allow_texture_1d(void* handle, bool allowed) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)allowed;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->setAllowTexture1D(allowed); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

bool ocio_gpu_shader_desc_get_allow_texture_1d(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return false;
#else
  try { return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getAllowTexture1D(); }
  catch (...) { return false; }
#endif
}

const char* ocio_gpu_shader_desc_get_cache_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return nullptr;
#else
  try { return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getCacheID(); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_gpu_shader_desc_begin(void* handle, const char* uid) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)uid;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->begin(uid); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_gpu_shader_desc_end(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->end(); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

uint32_t ocio_gpu_shader_desc_get_next_resource_index(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try { return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getNextResourceIndex(); }
  catch (...) { return 0; }
#endif
}

void ocio_gpu_shader_desc_add_to_parameter_declare_shader_code(void* handle, const char* shaderCode) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shaderCode;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addToParameterDeclareShaderCode(shaderCode); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_gpu_shader_desc_add_to_texture_declare_shader_code(void* handle, const char* shaderCode) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shaderCode;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addToTextureDeclareShaderCode(shaderCode); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_gpu_shader_desc_add_to_helper_shader_code(void* handle, const char* shaderCode) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shaderCode;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addToHelperShaderCode(shaderCode); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_gpu_shader_desc_add_to_function_header_shader_code(void* handle, const char* shaderCode) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shaderCode;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addToFunctionHeaderShaderCode(shaderCode); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_gpu_shader_desc_add_to_function_shader_code(void* handle, const char* shaderCode) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shaderCode;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addToFunctionShaderCode(shaderCode); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_gpu_shader_desc_add_to_function_footer_shader_code(void* handle, const char* shaderCode) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shaderCode;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->addToFunctionFooterShaderCode(shaderCode); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_gpu_shader_desc_create_shader_text(
    void* handle,
    const char* shaderParameterDeclarations,
    const char* shaderTextureDeclarations,
    const char* shaderHelperMethods,
    const char* shaderFunctionHeader,
    const char* shaderFunctionBody,
    const char* shaderFunctionFooter) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)shaderParameterDeclarations; (void)shaderTextureDeclarations;
  (void)shaderHelperMethods; (void)shaderFunctionHeader; (void)shaderFunctionBody; (void)shaderFunctionFooter;
#else
  try {
    ocio_rs_bridge::get_real_gpu_shader_desc(handle)->createShaderText(
      shaderParameterDeclarations,
      shaderTextureDeclarations,
      shaderHelperMethods,
      shaderFunctionHeader,
      shaderFunctionBody,
      shaderFunctionFooter);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_gpu_shader_desc_finalize(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
#else
  try { ocio_rs_bridge::get_real_gpu_shader_desc(handle)->finalize(); }
  catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

uint32_t ocio_gpu_shader_desc_get_texture_max_width(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return 0;
#else
  try {
    (void)index;
    return ocio_rs_bridge::get_real_gpu_shader_desc(handle)->getTextureMaxWidth();
  } catch (...) { return 0; }
#endif
}

uint32_t ocio_gpu_shader_desc_get_texture_max_height(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return 0;
#else
  (void)handle; (void)index;
  return 0;
#endif
}

const char* ocio_gpu_shader_desc_get_texture_uid(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    if (index < 0) return nullptr;
    OcioGpuTexture2DInfo info{};
    return ocio_gpu_shader_desc_get_texture_info(handle, static_cast<unsigned>(index), &info)
      ? info.texture_name : nullptr;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_baker_get_format(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_baker(handle)->getFormat()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_baker_get_format_metadata(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return ocio_rs_bridge::make_stub_format_metadata().release();
#else
  try {
    auto baker = ocio_rs_bridge::get_real_baker(handle);
    auto owner = std::make_shared<ocio::BakerRcPtr>(baker);
    return ocio_rs_bridge::make_format_metadata_handle(owner, &((*owner)->getFormatMetadata()));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_baker_get_format_metadata_v1(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return ocio_rs_bridge::make_stub_format_metadata().release();
#else
  return ocio_baker_get_format_metadata(handle);
#endif
}

void* ocio_baker_create_editable_copy(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker;
  return ocio_rs_bridge::make_stub_baker().release();
#else
  try {
    auto copy = ocio_rs_bridge::get_real_baker(baker)->createEditableCopy();
    if (!copy) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::BakerHandle>();
    auto obj = std::make_shared<ocio_rs_bridge::RealBaker>();
    obj->baker = copy;
    out_handle->inner = obj;
    return out_handle.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_baker_get_num_formats(void) {
#ifdef OCIO_RS_STUB
  return 0;
#else
  try { return ocio::Baker::getNumFormats(); } catch (...) { return 0; }
#endif
}

const char* ocio_baker_get_format_name_by_index(int index) {
#ifdef OCIO_RS_STUB
  (void)index;
  return nullptr;
#else
  try { return ocio::Baker::getFormatNameByIndex(index); } catch (...) { return nullptr; }
#endif
}

const char* ocio_baker_get_format_extension_by_index(int index) {
#ifdef OCIO_RS_STUB
  (void)index;
  return nullptr;
#else
  try { return ocio::Baker::getFormatExtensionByIndex(index); } catch (...) { return nullptr; }
#endif
}

void* ocio_baker_get_input_space(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_baker(handle)->getInputSpace()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_baker_get_shaper_space(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_baker(handle)->getShaperSpace()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_baker_get_looks(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_baker(handle)->getLooks()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_baker_get_target_space(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_baker(handle)->getTargetSpace()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_baker_get_display(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_baker(handle)->getDisplay()));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_baker_get_view(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_baker(handle)->getView()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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

void* ocio_baker_bake_to_string(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return nullptr;
#else
  try {
    std::ostringstream baker_stream;
    ocio_rs_bridge::get_real_baker(handle)->bake(baker_stream);
    ocio_rs_bridge::g_serialized_text = baker_stream.str();
    return (void*)ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->getCacheID()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_context_get_search_path(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->getSearchPath()));
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->getSearchPath(index)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_context_add_search_path(void* handle, const char* path) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)path;
  return;
#else
  try {
    ocio_rs_bridge::get_real_context(handle)->addSearchPath(path);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_context_set_working_dir(void* handle, const char* dirname) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dirname;
  return;
#else
  try {
    ocio_rs_bridge::get_real_context(handle)->setWorkingDir(dirname);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_context_get_working_dir(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->getWorkingDir()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_context_get_string_var(void* handle, const char* name) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)name;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->getStringVar(name)));
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->getStringVarNameByIndex(index)));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_context_get_string_var_by_index(void* handle, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->getStringVarByIndex(index)));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_context_set_environment_mode(void* handle, int mode) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)mode;
  return;
#else
  try {
    ocio_rs_bridge::get_real_context(handle)->setEnvironmentMode(static_cast<ocio::EnvironmentMode>(mode));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_context_resolve_string_var(void* handle, const char* string) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)string;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->resolveStringVar(string)));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_context_resolve_string_var_v1(void* handle, const char* string, void* usedContextVars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)string; (void)usedContextVars;
  return nullptr;
#else
  try {
    if (!usedContextVars) {
      return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->resolveStringVar(string)));
    }
    auto* _usedContextVars_h = static_cast<ocio_rs_bridge::ContextHandle*>(usedContextVars);
    auto usedContextVars_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_usedContextVars_h->inner)->context;
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->resolveStringVar(string, usedContextVars_ptr)));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_context_resolve_file_location(void* handle, const char* filename) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filename;
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->resolveFileLocation(filename)));
  } catch (...) { return nullptr; }
#endif
}

void* ocio_context_resolve_file_location_v1(void* handle, const char* filename, void* usedContextVars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)filename; (void)usedContextVars;
  return nullptr;
#else
  try {
    if (!usedContextVars) {
      return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->resolveFileLocation(filename)));
    }
    auto* _usedContextVars_h = static_cast<ocio_rs_bridge::ContextHandle*>(usedContextVars);
    auto usedContextVars_ptr = std::static_pointer_cast<ocio_rs_bridge::RealContext>(_usedContextVars_h->inner)->context;
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_context(handle)->resolveFileLocation(filename, usedContextVars_ptr)));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_context_set_config_io_proxy(void* handle, void* ciop) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)ciop;
  return;
#else
  try {
    if (!ciop) {
      ocio_rs_bridge::get_real_context(handle)->setConfigIOProxy(ocio::ConfigIOProxyRcPtr());
      return;
    }
    auto* _ciop_h = static_cast<ocio_rs_bridge::ConfigIOProxyHandle*>(ciop);
    auto ciop_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfigIOProxy>(_ciop_h->inner)->proxy;
    ocio_rs_bridge::get_real_context(handle)->setConfigIOProxy(ciop_ptr);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_context_get_config_io_proxy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto context = ocio_rs_bridge::get_real_context(handle);
    auto result = context->getConfigIOProxy();
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ConfigIOProxyHandle>();
    auto owner = std::make_shared<ocio::ContextRcPtr>(std::move(context));
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealConfigIOProxy>(
      ocio_rs_bridge::RealConfigIOProxy{result, owner});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}


// --- AllocationTransform ---

void* ocio_allocation_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_allocation_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_allocation_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_allocation_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::AllocationTransformHandle*>(handle);
}

void* ocio_allocation_transform_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_allocation_transform().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_allocation_transform(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto typed = std::dynamic_pointer_cast<ocio::AllocationTransform>(result);
    if (!typed) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::AllocationTransformHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealAllocationTransform>(
        ocio_rs_bridge::RealAllocationTransform{typed});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

int ocio_allocation_transform_get_direction(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_allocation_transform(handle)->getDirection();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_allocation_transform_set_direction(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return;
#else
  try {
    ocio_rs_bridge::get_real_allocation_transform(handle)->setDirection(static_cast<ocio::TransformDirection>(dir));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_allocation_transform_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    ocio_rs_bridge::get_real_allocation_transform(handle)->validate();
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

int ocio_allocation_transform_get_allocation(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_allocation_transform(handle)->getAllocation();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_allocation_transform_set_allocation(void* handle, int allocation) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)allocation;
  return;
#else
  try {
    ocio_rs_bridge::get_real_allocation_transform(handle)->setAllocation(static_cast<ocio::Allocation>(allocation));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

int ocio_allocation_transform_get_num_vars(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_allocation_transform(handle)->getNumVars();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_allocation_transform_get_vars(void* handle, void* vars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)vars;
  return;
#else
  try {
    ocio_rs_bridge::get_real_allocation_transform(handle)->getVars(static_cast<float*>(vars));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_allocation_transform_set_vars(void* handle, int numvars, const float* vars) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)numvars; (void)vars;
  return;
#else
  try {
    ocio_rs_bridge::get_real_allocation_transform(handle)->setVars(numvars, vars);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- BuiltinTransform ---

void* ocio_builtin_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_builtin_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_builtin_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_builtin_transform(handle)->getStyle()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_builtin_transform_get_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_builtin_transform(handle)->getDescription()));
  } catch (...) { return nullptr; }
#endif
}

int ocio_builtin_transform_get_num_styles(void) {
#ifdef OCIO_RS_STUB
  return 0;
#else
  try {
    auto registry = ocio::BuiltinTransformRegistry::Get();
    return registry ? static_cast<int>(registry->getNumBuiltins()) : 0;
  } catch (...) { return 0; }
#endif
}

const char* ocio_builtin_transform_get_style_by_index(int index) {
#ifdef OCIO_RS_STUB
  (void)index;
  return nullptr;
#else
  try {
    if (index < 0) return nullptr;
    auto registry = ocio::BuiltinTransformRegistry::Get();
    if (!registry || static_cast<size_t>(index) >= registry->getNumBuiltins()) return nullptr;
    return registry->getBuiltinStyle(static_cast<size_t>(index));
  } catch (...) { return nullptr; }
#endif
}

bool ocio_builtin_transform_is_valid_style(const char* style) {
#ifdef OCIO_RS_STUB
  (void)style;
  return false;
#else
  try {
    if (!style) return false;
    auto registry = ocio::BuiltinTransformRegistry::Get();
    if (!registry) return false;
    const size_t count = registry->getNumBuiltins();
    for (size_t i = 0; i < count; ++i) {
      const char* candidate = registry->getBuiltinStyle(i);
      if (candidate && std::strcmp(candidate, style) == 0) return true;
    }
    return false;
  } catch (...) { return false; }
#endif
}


// --- CDLTransform ---

void* ocio_cdl_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_cdl_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_cdl_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_cdl_transform_create_from_file(const char* src, const char* cccId) {
#ifdef OCIO_RS_STUB
  (void)src; (void)cccId;
  return ocio_rs_bridge::make_stub_cdl_transform().release();
#else
  try {
    auto result = ocio::CDLTransform::CreateFromFile(src, cccId);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::CDLTransformHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealCDLTransform>(
      ocio_rs_bridge::RealCDLTransform{ result }
    );
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_cdl_transform_from_file(const char* src, const char* cccId) {
  return ocio_cdl_transform_create_from_file(src, cccId);
}

void* ocio_cdl_transform_create_group_from_file(const char* src) {
#ifdef OCIO_RS_STUB
  (void)src;
  return ocio_rs_bridge::make_stub_group_transform().release();
#else
  try {
    auto result = ocio::CDLTransform::CreateGroupFromFile(src);
    if (!result) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::GroupTransformHandle>();
    out_handle->inner =
        std::make_shared<ocio_rs_bridge::RealGroupTransform>(ocio_rs_bridge::RealGroupTransform{result});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_cdl_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::CDLTransformHandle*>(handle);
}

void* ocio_cdl_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_cdl_transform_get_format_metadata_v1(void* handle) {
  return ocio_cdl_transform_get_format_metadata(handle);
}

void* ocio_cdl_transform_get_format_metadata_v2(void* handle) {
  return ocio_cdl_transform_get_format_metadata(handle);
}

bool ocio_cdl_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cdl_transform(handle)->equals(
      *ocio_rs_bridge::get_real_cdl_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_cdl_transform_get_slope(void* handle, void* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->getSlope(static_cast<double*>(rgb));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_cdl_transform_set_slope(void* handle, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setSlope(rgb);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_cdl_transform_get_offset(void* handle, void* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->getOffset(static_cast<double*>(rgb));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_cdl_transform_set_offset(void* handle, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setOffset(rgb);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_cdl_transform_get_power(void* handle, void* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->getPower(static_cast<double*>(rgb));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_cdl_transform_set_power(void* handle, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setPower(rgb);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_cdl_transform_get_sop(void* handle, void* vec9) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)vec9;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->getSOP(static_cast<double*>(vec9));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_cdl_transform_set_sop(void* handle, const double* vec9) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)vec9;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setSOP(vec9);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

double ocio_cdl_transform_get_sat(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_cdl_transform(handle)->getSat();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0.0; }
#endif
}

void ocio_cdl_transform_set_sat(void* handle, double sat) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)sat;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->setSat(sat);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_cdl_transform_get_sat_luma_coefs(void* handle, void* rgb) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)rgb;
  return;
#else
  try {
    ocio_rs_bridge::get_real_cdl_transform(handle)->getSatLumaCoefs(static_cast<double*>(rgb));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_cdl_transform_get_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_cdl_transform(handle)->getID()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_cdl_transform_get_first_sop_description(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_cdl_transform(handle)->getFirstSOPDescription()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- ColorSpaceTransform ---

void* ocio_color_space_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_color_space_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_color_space_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_color_space_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(handle);
}

void* ocio_color_space_transform_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_color_space_transform().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_color_space_transform(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto typed = std::dynamic_pointer_cast<ocio::ColorSpaceTransform>(result);
    if (!typed) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ColorSpaceTransformHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceTransform>(
        ocio_rs_bridge::RealColorSpaceTransform{typed});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_color_space_transform_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    ocio_rs_bridge::get_real_color_space_transform(handle)->validate();
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void* ocio_color_space_transform_get_src(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_color_space_transform(handle)->getSrc()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_color_space_transform_get_dst(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_color_space_transform(handle)->getDst()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- DisplayViewTransform ---

void* ocio_display_view_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_display_view_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_display_view_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_display_view_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(handle);
}

void* ocio_display_view_transform_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_display_view_transform().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_display_view_transform(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto typed = std::dynamic_pointer_cast<ocio::DisplayViewTransform>(result);
    if (!typed) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::DisplayViewTransformHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealDisplayViewTransform>(
        ocio_rs_bridge::RealDisplayViewTransform{typed});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_display_view_transform_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    ocio_rs_bridge::get_real_display_view_transform(handle)->validate();
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void* ocio_display_view_transform_get_src(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_display_view_transform(handle)->getSrc()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_display_view_transform_get_display(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_display_view_transform(handle)->getDisplay()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_display_view_transform_get_view(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_display_view_transform(handle)->getView()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- ExponentTransform ---

void* ocio_exponent_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_exponent_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_exponent_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_exponent_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ExponentTransformHandle*>(handle);
}

void* ocio_exponent_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_exponent_transform_get_format_metadata_v1(void* handle) {
  return ocio_exponent_transform_get_format_metadata(handle);
}

void* ocio_exponent_transform_get_format_metadata_v2(void* handle) {
  return ocio_exponent_transform_get_format_metadata(handle);
}

bool ocio_exponent_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_exponent_transform(handle)->equals(
      *ocio_rs_bridge::get_real_exponent_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}


// --- ExponentWithLinearTransform ---

void* ocio_exponent_with_linear_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_exponent_with_linear_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_exponent_with_linear_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_exponent_with_linear_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(handle);
}

void* ocio_exponent_with_linear_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_exponent_with_linear_transform_get_format_metadata_v1(void* handle) {
  return ocio_exponent_with_linear_transform_get_format_metadata(handle);
}

void* ocio_exponent_with_linear_transform_get_format_metadata_v2(void* handle) {
  return ocio_exponent_with_linear_transform_get_format_metadata(handle);
}

bool ocio_exponent_with_linear_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_exponent_with_linear_transform(handle)->equals(
      *ocio_rs_bridge::get_real_exponent_with_linear_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}


// --- ExposureContrastTransform ---

void* ocio_exposure_contrast_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_exposure_contrast_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_exposure_contrast_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_exposure_contrast_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(handle);
}

void* ocio_exposure_contrast_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_exposure_contrast_transform_get_format_metadata_v1(void* handle) {
  return ocio_exposure_contrast_transform_get_format_metadata(handle);
}

void* ocio_exposure_contrast_transform_get_format_metadata_v2(void* handle) {
  return ocio_exposure_contrast_transform_get_format_metadata(handle);
}

bool ocio_exposure_contrast_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->equals(
      *ocio_rs_bridge::get_real_exposure_contrast_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_exposure_contrast_transform_make_exposure_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->makeExposureNonDynamic();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_exposure_contrast_transform_make_contrast_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->makeContrastNonDynamic();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_exposure_contrast_transform_make_gamma_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_exposure_contrast_transform(handle)->makeGammaNonDynamic();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- FileTransform ---

void* ocio_file_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_file_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_file_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_file_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::FileTransformHandle*>(handle);
}

void* ocio_file_transform_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_file_transform().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_file_transform(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto typed = std::dynamic_pointer_cast<ocio::FileTransform>(result);
    if (!typed) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::FileTransformHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealFileTransform>(
        ocio_rs_bridge::RealFileTransform{typed});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_file_transform_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    ocio_rs_bridge::get_real_file_transform(handle)->validate();
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void* ocio_file_transform_get_src(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_file_transform(handle)->getSrc()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_file_transform_get_ccc_id(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_file_transform(handle)->getCCCId()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- FixedFunctionTransform ---

void* ocio_fixed_function_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_fixed_function_transform().release();
#else
  try {
    auto result = ocio::FixedFunctionTransform::Create(ocio::FIXED_FUNCTION_ACES_RED_MOD_03);
    if (!result) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::FixedFunctionTransformHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealFixedFunctionTransform>(ocio_rs_bridge::RealFixedFunctionTransform{result});
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_fixed_function_transform_create_with_params(int style, const double* params, size_t num) {
#ifdef OCIO_RS_STUB
  (void)style; (void)params; (void)num;
  return ocio_rs_bridge::make_stub_fixed_function_transform().release();
#else
  try {
    auto result = ocio::FixedFunctionTransform::Create(static_cast<ocio::FixedFunctionStyle>(style), params, num);
    if (!result) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::FixedFunctionTransformHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealFixedFunctionTransform>(ocio_rs_bridge::RealFixedFunctionTransform{result});
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_fixed_function_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(handle);
}

void* ocio_fixed_function_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_fixed_function_transform_get_format_metadata_v1(void* handle) {
  return ocio_fixed_function_transform_get_format_metadata(handle);
}

void* ocio_fixed_function_transform_get_format_metadata_v2(void* handle) {
  return ocio_fixed_function_transform_get_format_metadata(handle);
}

bool ocio_fixed_function_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_fixed_function_transform(handle)->equals(
      *ocio_rs_bridge::get_real_fixed_function_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

int ocio_fixed_function_transform_get_direction(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_fixed_function_transform(handle)->getDirection();
  } catch (...) { return 0; }
#endif
}

void ocio_fixed_function_transform_set_direction(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return;
#else
  try {
    ocio_rs_bridge::get_real_fixed_function_transform(handle)->setDirection(static_cast<ocio::TransformDirection>(dir));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

int ocio_file_transform_get_num_formats(void) {
#ifdef OCIO_RS_STUB
  return 0;
#else
  try {
    return ocio::FileTransform::GetNumFormats();
  } catch (...) { return 0; }
#endif
}

const char* ocio_file_transform_get_format_name_by_index(int index) {
#ifdef OCIO_RS_STUB
  (void)index;
  return nullptr;
#else
  try {
    return ocio::FileTransform::GetFormatNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_file_transform_get_format_extension_by_index(int index) {
#ifdef OCIO_RS_STUB
  (void)index;
  return nullptr;
#else
  try {
    return ocio::FileTransform::GetFormatExtensionByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

bool ocio_file_transform_is_format_extension_supported(const char* extension) {
#ifdef OCIO_RS_STUB
  (void)extension;
  return false;
#else
  try {
    return ocio::FileTransform::IsFormatExtensionSupported(extension);
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

size_t ocio_fixed_function_transform_get_num_params(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_fixed_function_transform(handle)->getNumParams();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_fixed_function_transform_get_params(void* handle, void* params) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)params;
  return;
#else
  try {
    ocio_rs_bridge::get_real_fixed_function_transform(handle)->getParams(static_cast<double*>(params));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_fixed_function_transform_set_params(void* handle, const double* params, size_t num) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)params; (void)num;
  return;
#else
  try {
    ocio_rs_bridge::get_real_fixed_function_transform(handle)->setParams(params, num);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- GradingPrimaryTransform ---

void* ocio_grading_primary_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_grading_primary_transform().release();
#else
  return ocio_grading_primary_transform_create_with_style(static_cast<int>(ocio::GRADING_LOG));
#endif
}

void* ocio_grading_primary_transform_create_with_style(int style) {
#ifdef OCIO_RS_STUB
  (void)style;
  return ocio_rs_bridge::make_stub_grading_primary_transform().release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::GradingPrimaryTransformHandle>();
    auto obj = std::make_shared<ocio_rs_bridge::RealGradingPrimaryTransform>();
    obj->transform = ocio::GradingPrimaryTransform::Create(static_cast<ocio::GradingStyle>(style));
    handle->inner = obj;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_grading_primary_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(handle);
}

void* ocio_grading_primary_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_grading_primary_transform_get_format_metadata_v1(void* handle) {
  return ocio_grading_primary_transform_get_format_metadata(handle);
}

void* ocio_grading_primary_transform_get_format_metadata_v2(void* handle) {
  return ocio_grading_primary_transform_get_format_metadata(handle);
}

bool ocio_grading_primary_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_primary_transform(handle)->equals(
      *ocio_rs_bridge::get_real_grading_primary_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_grading_primary_transform_get_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto out_handle = std::make_unique<ocio_rs_bridge::GradingPrimaryValueHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGradingPrimaryValue>(
        ocio_rs_bridge::RealGradingPrimaryValue{
            std::make_shared<ocio::GradingPrimary>(
                ocio_rs_bridge::get_real_grading_primary_transform(handle)->getValue())});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_grading_primary_transform_set_value(void* handle, void* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_primary_transform(handle)->setValue(
        *ocio_rs_bridge::get_real_grading_primary_value(values));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_grading_primary_value_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingPrimaryValueHandle*>(handle);
}

bool ocio_grading_primary_transform_copy_value(void* handle, double* values, size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values; (void)len;
  return false;
#else
  try {
    if (!values || len < 34) return false;
    const auto& v = ocio_rs_bridge::get_real_grading_primary_transform(handle)->getValue();
    size_t off = 0;
    auto write_rgbm = [&](const ocio::GradingRGBM& rgbm) {
      values[off++] = rgbm.m_red;
      values[off++] = rgbm.m_green;
      values[off++] = rgbm.m_blue;
      values[off++] = rgbm.m_master;
    };
    write_rgbm(v.m_brightness);
    write_rgbm(v.m_contrast);
    write_rgbm(v.m_gamma);
    write_rgbm(v.m_offset);
    write_rgbm(v.m_exposure);
    write_rgbm(v.m_lift);
    write_rgbm(v.m_gain);
    values[off++] = v.m_saturation;
    values[off++] = v.m_pivot;
    values[off++] = v.m_pivotBlack;
    values[off++] = v.m_pivotWhite;
    values[off++] = v.m_clampBlack;
    values[off++] = v.m_clampWhite;
    return true;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_grading_primary_transform_set_value_from_f64(void* handle, const double* values, size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values; (void)len;
  return true;
#else
  try {
    if (!values || len < 34) return false;
    auto transform = ocio_rs_bridge::get_real_grading_primary_transform(handle);
    ocio::GradingPrimary v(static_cast<ocio::GradingStyle>(transform->getStyle()));
    size_t off = 0;
    auto read_rgbm = [&]() {
      ocio::GradingRGBM rgbm;
      rgbm.m_red = values[off++];
      rgbm.m_green = values[off++];
      rgbm.m_blue = values[off++];
      rgbm.m_master = values[off++];
      return rgbm;
    };
    v.m_brightness = read_rgbm();
    v.m_contrast = read_rgbm();
    v.m_gamma = read_rgbm();
    v.m_offset = read_rgbm();
    v.m_exposure = read_rgbm();
    v.m_lift = read_rgbm();
    v.m_gain = read_rgbm();
    v.m_saturation = values[off++];
    v.m_pivot = values[off++];
    v.m_pivotBlack = values[off++];
    v.m_pivotWhite = values[off++];
    v.m_clampBlack = values[off++];
    v.m_clampWhite = values[off++];
    transform->setValue(v);
    return true;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_grading_primary_transform_make_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_primary_transform(handle)->makeNonDynamic();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- GradingRGBCurveTransform ---

void* ocio_grading_rgb_curve_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_grading_rgb_curve_transform().release();
#else
  return ocio_grading_rgb_curve_transform_create_with_style(static_cast<int>(ocio::GRADING_LOG));
#endif
}

void* ocio_grading_rgb_curve_transform_create_with_style(int style) {
#ifdef OCIO_RS_STUB
  (void)style;
  return ocio_rs_bridge::make_stub_grading_rgb_curve_transform().release();
#else
  try {
    auto result = ocio::GradingRGBCurveTransform::Create(static_cast<ocio::GradingStyle>(style));
    if (!result) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::GradingRGBCurveTransformHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealGradingRGBCurveTransform>(ocio_rs_bridge::RealGradingRGBCurveTransform{result});
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_grading_rgb_curve_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(handle);
}

void* ocio_grading_rgb_curve_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_grading_rgb_curve_transform_get_format_metadata_v1(void* handle) {
  return ocio_grading_rgb_curve_transform_get_format_metadata(handle);
}

void* ocio_grading_rgb_curve_transform_get_format_metadata_v2(void* handle) {
  return ocio_grading_rgb_curve_transform_get_format_metadata(handle);
}

bool ocio_grading_rgb_curve_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->equals(
      *ocio_rs_bridge::get_real_grading_rgb_curve_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

int ocio_grading_rgb_curve_transform_get_direction(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->getDirection();
  } catch (...) { return 0; }
#endif
}

void ocio_grading_rgb_curve_transform_set_direction(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->setDirection(static_cast<ocio::TransformDirection>(dir));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_grading_rgb_curve_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingRGBCurveHandle*>(handle);
}

int ocio_grading_rgb_curve_transform_get_num_control_points(void* handle, int c) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c;
  return 0;
#else
  try {
    auto curve = ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->getValue()->getCurve(static_cast<ocio::RGBCurveType>(c));
    if (!curve) return 0;
    return static_cast<int>(curve->getNumControlPoints());
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_grading_rgb_curve_transform_get_control_point(void* handle, int c, int index, float* x, float* y) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index; (void)x; (void)y;
  return;
#else
  try {
    if (!x || !y) return;
    auto curve = ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->getValue()->getCurve(static_cast<ocio::RGBCurveType>(c));
    if (!curve) return;
    const auto& point = curve->getControlPoint(static_cast<size_t>(index));
    *x = point.m_x;
    *y = point.m_y;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_grading_rgb_curve_transform_set_num_control_points(void* handle, int c, int num) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)num;
  return;
#else
  try {
    auto transform = ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle);
    auto value = ocio::GradingRGBCurve::Create(transform->getValue());
    auto curve = value->getCurve(static_cast<ocio::RGBCurveType>(c));
    if (!curve) return;
    curve->setNumControlPoints(static_cast<size_t>(num));
    transform->setValue(value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_grading_rgb_curve_transform_set_control_point(void* handle, int c, int index, float x, float y) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index; (void)x; (void)y;
  return;
#else
  try {
    auto transform = ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle);
    auto value = ocio::GradingRGBCurve::Create(transform->getValue());
    auto curve = value->getCurve(static_cast<ocio::RGBCurveType>(c));
    if (!curve) return;
    auto& point = curve->getControlPoint(static_cast<size_t>(index));
    point.m_x = x;
    point.m_y = y;
    transform->setValue(value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

float ocio_grading_rgb_curve_transform_get_slope(void* handle, int c, size_t index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index;
  return 0.0f;
#else
  try {
    return ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->getSlope(static_cast<ocio::RGBCurveType>(c), index);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0.0f; }
#endif
}

void ocio_grading_rgb_curve_transform_set_slope(void* handle, int c, size_t index, float slope) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index; (void)slope;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->setSlope(static_cast<ocio::RGBCurveType>(c), index, slope);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

bool ocio_grading_rgb_curve_transform_slopes_are_default(void* handle, int c) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->slopesAreDefault(static_cast<ocio::RGBCurveType>(c));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_grading_rgb_curve_transform_make_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_rgb_curve_transform(handle)->makeNonDynamic();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}


// --- GradingHueCurveTransform ---

void* ocio_grading_hue_curve_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_grading_hue_curve_transform().release();
#else
  return ocio_grading_hue_curve_transform_create_with_style(static_cast<int>(ocio::GRADING_LOG));
#endif
}

void* ocio_grading_hue_curve_transform_create_with_style(int style) {
#ifdef OCIO_RS_STUB
  (void)style;
  return ocio_rs_bridge::make_stub_grading_hue_curve_transform().release();
#else
  try {
    auto result = ocio::GradingHueCurveTransform::Create(static_cast<ocio::GradingStyle>(style));
    if (!result) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::GradingHueCurveTransformHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealGradingHueCurveTransform>(ocio_rs_bridge::RealGradingHueCurveTransform{result});
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_grading_hue_curve_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(handle);
}

void* ocio_grading_hue_curve_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_grading_hue_curve_transform_get_format_metadata_v1(void* handle) {
  return ocio_grading_hue_curve_transform_get_format_metadata(handle);
}

void* ocio_grading_hue_curve_transform_get_format_metadata_v2(void* handle) {
  return ocio_grading_hue_curve_transform_get_format_metadata(handle);
}

bool ocio_grading_hue_curve_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->equals(
      *ocio_rs_bridge::get_real_grading_hue_curve_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

int ocio_grading_hue_curve_transform_get_direction(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->getDirection();
  } catch (...) { return 0; }
#endif
}

void ocio_grading_hue_curve_transform_set_direction(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->setDirection(static_cast<ocio::TransformDirection>(dir));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_grading_hue_curve_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingHueCurveHandle*>(handle);
}

int ocio_grading_hue_curve_transform_get_num_control_points(void* handle, int c) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c;
  return 0;
#else
  try {
    auto curve = ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->getValue()->getCurve(static_cast<ocio::HueCurveType>(c));
    if (!curve) return 0;
    return static_cast<int>(curve->getNumControlPoints());
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_grading_hue_curve_transform_get_control_point(void* handle, int c, int index, float* x, float* y) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index; (void)x; (void)y;
  return;
#else
  try {
    if (!x || !y) return;
    auto curve = ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->getValue()->getCurve(static_cast<ocio::HueCurveType>(c));
    if (!curve) return;
    const auto& point = curve->getControlPoint(static_cast<size_t>(index));
    *x = point.m_x;
    *y = point.m_y;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_grading_hue_curve_transform_set_num_control_points(void* handle, int c, int num) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)num;
  return;
#else
  try {
    auto transform = ocio_rs_bridge::get_real_grading_hue_curve_transform(handle);
    auto value = ocio::GradingHueCurve::Create(transform->getValue());
    auto curve = value->getCurve(static_cast<ocio::HueCurveType>(c));
    if (!curve) return;
    curve->setNumControlPoints(static_cast<size_t>(num));
    transform->setValue(value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_grading_hue_curve_transform_set_control_point(void* handle, int c, int index, float x, float y) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index; (void)x; (void)y;
  return;
#else
  try {
    auto transform = ocio_rs_bridge::get_real_grading_hue_curve_transform(handle);
    auto value = ocio::GradingHueCurve::Create(transform->getValue());
    auto curve = value->getCurve(static_cast<ocio::HueCurveType>(c));
    if (!curve) return;
    auto& point = curve->getControlPoint(static_cast<size_t>(index));
    point.m_x = x;
    point.m_y = y;
    transform->setValue(value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

float ocio_grading_hue_curve_transform_get_slope(void* handle, int c, size_t index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index;
  return 0.0f;
#else
  try {
    return ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->getSlope(static_cast<ocio::HueCurveType>(c), index);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0.0f; }
#endif
}

void ocio_grading_hue_curve_transform_set_slope(void* handle, int c, size_t index, float slope) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c; (void)index; (void)slope;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->setSlope(static_cast<ocio::HueCurveType>(c), index, slope);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

bool ocio_grading_hue_curve_transform_slopes_are_default(void* handle, int c) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)c;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->slopesAreDefault(static_cast<ocio::HueCurveType>(c));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_grading_hue_curve_transform_make_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_hue_curve_transform(handle)->makeNonDynamic();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}


// --- GradingToneTransform ---

void* ocio_grading_tone_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_grading_tone_transform().release();
#else
  return ocio_grading_tone_transform_create_with_style(static_cast<int>(ocio::GRADING_LOG));
#endif
}

void* ocio_grading_tone_transform_create_with_style(int style) {
#ifdef OCIO_RS_STUB
  (void)style;
  return ocio_rs_bridge::make_stub_grading_tone_transform().release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::GradingToneTransformHandle>();
    auto obj = std::make_shared<ocio_rs_bridge::RealGradingToneTransform>();
    obj->transform = ocio::GradingToneTransform::Create(static_cast<ocio::GradingStyle>(style));
    handle->inner = obj;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_grading_tone_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(handle);
}

void* ocio_grading_tone_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_grading_tone_transform_get_format_metadata_v1(void* handle) {
  return ocio_grading_tone_transform_get_format_metadata(handle);
}

void* ocio_grading_tone_transform_get_format_metadata_v2(void* handle) {
  return ocio_grading_tone_transform_get_format_metadata(handle);
}

bool ocio_grading_tone_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_grading_tone_transform(handle)->equals(
      *ocio_rs_bridge::get_real_grading_tone_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_grading_tone_transform_get_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto out_handle = std::make_unique<ocio_rs_bridge::GradingToneValueHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealGradingToneValue>(
        ocio_rs_bridge::RealGradingToneValue{
            std::make_shared<ocio::GradingTone>(
                ocio_rs_bridge::get_real_grading_tone_transform(handle)->getValue())});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_grading_tone_transform_set_value(void* handle, void* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_tone_transform(handle)->setValue(
        *ocio_rs_bridge::get_real_grading_tone_value(values));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_grading_tone_value_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingToneValueHandle*>(handle);
}

bool ocio_grading_tone_transform_copy_value(void* handle, double* values, size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values; (void)len;
  return false;
#else
  try {
    if (!values || len < 31) return false;
    const auto& v = ocio_rs_bridge::get_real_grading_tone_transform(handle)->getValue();
    size_t off = 0;
    auto write_rgbmsw = [&](const ocio::GradingRGBMSW& rgbmsw) {
      values[off++] = rgbmsw.m_red;
      values[off++] = rgbmsw.m_green;
      values[off++] = rgbmsw.m_blue;
      values[off++] = rgbmsw.m_master;
      values[off++] = rgbmsw.m_start;
      values[off++] = rgbmsw.m_width;
    };
    write_rgbmsw(v.m_blacks);
    write_rgbmsw(v.m_shadows);
    write_rgbmsw(v.m_midtones);
    write_rgbmsw(v.m_highlights);
    write_rgbmsw(v.m_whites);
    values[off++] = v.m_scontrast;
    return true;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

bool ocio_grading_tone_transform_set_value_from_f64(void* handle, const double* values, size_t len) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values; (void)len;
  return true;
#else
  try {
    if (!values || len < 31) return false;
    auto transform = ocio_rs_bridge::get_real_grading_tone_transform(handle);
    ocio::GradingTone v(static_cast<ocio::GradingStyle>(transform->getStyle()));
    size_t off = 0;
    auto read_rgbmsw = [&]() {
      ocio::GradingRGBMSW rgbmsw;
      rgbmsw.m_red = values[off++];
      rgbmsw.m_green = values[off++];
      rgbmsw.m_blue = values[off++];
      rgbmsw.m_master = values[off++];
      rgbmsw.m_start = values[off++];
      rgbmsw.m_width = values[off++];
      return rgbmsw;
    };
    v.m_blacks = read_rgbmsw();
    v.m_shadows = read_rgbmsw();
    v.m_midtones = read_rgbmsw();
    v.m_highlights = read_rgbmsw();
    v.m_whites = read_rgbmsw();
    v.m_scontrast = values[off++];
    transform->setValue(v);
    return true;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_grading_tone_transform_make_non_dynamic(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_grading_tone_transform(handle)->makeNonDynamic();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}


// --- GroupTransform ---

void* ocio_group_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_group_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_group_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_group_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GroupTransformHandle*>(handle);
}

void* ocio_group_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_group_transform_get_format_metadata_v1(void* handle) {
  return ocio_group_transform_get_format_metadata(handle);
}

void* ocio_group_transform_get_format_metadata_v2(void* handle) {
  return ocio_group_transform_get_format_metadata(handle);
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
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    auto transform_ptr = _transform_h->get_ocio_transform();
    ocio_rs_bridge::get_real_group_transform(handle)->appendTransform(transform_ptr);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_group_transform_prepend_transform(void* handle, void* transform) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)transform;
  return;
#else
  try {
    auto* _transform_h = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    auto transform_ptr = _transform_h->get_ocio_transform();
    ocio_rs_bridge::get_real_group_transform(handle)->prependTransform(transform_ptr);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void* ocio_group_transform_write_to_string(void* handle, void* config, const char* formatName) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)config; (void)formatName;
  return nullptr;
#else
  try {
    auto* _config_h = static_cast<ocio_rs_bridge::ConfigHandle*>(config);
    auto config_ptr = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(_config_h->inner)->config;
    std::ostringstream stream;
    ocio_rs_bridge::get_real_group_transform(handle)->write(config_ptr, formatName, stream);
    ocio_rs_bridge::g_serialized_text = stream.str();
    return (void*)ocio_rs_bridge::g_serialized_text.c_str();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

int ocio_group_transform_get_num_write_formats(void) {
#ifdef OCIO_RS_STUB
  return 0;
#else
  try {
    return ocio::GroupTransform::GetNumWriteFormats();
  } catch (...) { return 0; }
#endif
}

const char* ocio_group_transform_get_format_name_by_index(int index) {
#ifdef OCIO_RS_STUB
  (void)index;
  return nullptr;
#else
  try {
    return ocio::GroupTransform::GetFormatNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_group_transform_get_format_extension_by_index(int index) {
#ifdef OCIO_RS_STUB
  (void)index;
  return nullptr;
#else
  try {
    return ocio::GroupTransform::GetFormatExtensionByIndex(index);
  } catch (...) { return nullptr; }
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

void* ocio_log_affine_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_log_affine_transform_get_format_metadata_v1(void* handle) {
  return ocio_log_affine_transform_get_format_metadata(handle);
}

void* ocio_log_affine_transform_get_format_metadata_v2(void* handle) {
  return ocio_log_affine_transform_get_format_metadata(handle);
}

bool ocio_log_affine_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_log_affine_transform(handle)->equals(
      *ocio_rs_bridge::get_real_log_affine_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

double ocio_log_affine_transform_get_base(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_log_affine_transform(handle)->getBase();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0.0; }
#endif
}

void ocio_log_affine_transform_set_base(void* handle, double base) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)base;
  return;
#else
  try {
    ocio_rs_bridge::get_real_log_affine_transform(handle)->setBase(base);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}


// --- LogCameraTransform ---

void* ocio_log_camera_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_log_camera_transform().release();
#else
  const double values[3] = {0.01, 0.01, 0.01};
  return ocio_log_camera_transform_create_with_lin_side_break(values);
#endif
}

void* ocio_log_camera_transform_create_with_lin_side_break(const double* values) {
#ifdef OCIO_RS_STUB
  (void)values;
  return ocio_rs_bridge::make_stub_log_camera_transform().release();
#else
  try {
    if (!values) return nullptr;
    double lin_side_break[3] = { values[0], values[1], values[2] };
    auto result = ocio::LogCameraTransform::Create(lin_side_break);
    if (!result) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::LogCameraTransformHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealLogCameraTransform>(ocio_rs_bridge::RealLogCameraTransform{result});
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_log_camera_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(handle);
}

void* ocio_log_camera_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_log_camera_transform_get_format_metadata_v1(void* handle) {
  return ocio_log_camera_transform_get_format_metadata(handle);
}

void* ocio_log_camera_transform_get_format_metadata_v2(void* handle) {
  return ocio_log_camera_transform_get_format_metadata(handle);
}

bool ocio_log_camera_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_log_camera_transform(handle)->equals(
      *ocio_rs_bridge::get_real_log_camera_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

int ocio_log_camera_transform_get_direction(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_log_camera_transform(handle)->getDirection();
  } catch (...) { return 0; }
#endif
}

void ocio_log_camera_transform_set_direction(void* handle, int dir) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)dir;
  return;
#else
  try {
    ocio_rs_bridge::get_real_log_camera_transform(handle)->setDirection(static_cast<ocio::TransformDirection>(dir));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

double ocio_log_camera_transform_get_base(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0.0;
#else
  try {
    return ocio_rs_bridge::get_real_log_camera_transform(handle)->getBase();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0.0; }
#endif
}

void ocio_log_camera_transform_set_base(void* handle, double base) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)base;
  return;
#else
  try {
    ocio_rs_bridge::get_real_log_camera_transform(handle)->setBase(base);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_log_camera_transform_get_log_side_slope_value(void* handle, double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    if (!values) return;
    double tmp[3];
    ocio_rs_bridge::get_real_log_camera_transform(handle)->getLogSideSlopeValue(tmp);
    values[0] = tmp[0]; values[1] = tmp[1]; values[2] = tmp[2];
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_log_camera_transform_set_log_side_slope_value(void* handle, const double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    if (!values) return;
    double tmp[3] = { values[0], values[1], values[2] };
    ocio_rs_bridge::get_real_log_camera_transform(handle)->setLogSideSlopeValue(tmp);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_log_camera_transform_get_log_side_offset_value(void* handle, double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    if (!values) return;
    double tmp[3];
    ocio_rs_bridge::get_real_log_camera_transform(handle)->getLogSideOffsetValue(tmp);
    values[0] = tmp[0]; values[1] = tmp[1]; values[2] = tmp[2];
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_log_camera_transform_set_log_side_offset_value(void* handle, const double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    if (!values) return;
    double tmp[3] = { values[0], values[1], values[2] };
    ocio_rs_bridge::get_real_log_camera_transform(handle)->setLogSideOffsetValue(tmp);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_log_camera_transform_get_lin_side_slope_value(void* handle, double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    if (!values) return;
    double tmp[3];
    ocio_rs_bridge::get_real_log_camera_transform(handle)->getLinSideSlopeValue(tmp);
    values[0] = tmp[0]; values[1] = tmp[1]; values[2] = tmp[2];
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_log_camera_transform_set_lin_side_slope_value(void* handle, const double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    if (!values) return;
    double tmp[3] = { values[0], values[1], values[2] };
    ocio_rs_bridge::get_real_log_camera_transform(handle)->setLinSideSlopeValue(tmp);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_log_camera_transform_get_lin_side_offset_value(void* handle, double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    if (!values) return;
    double tmp[3];
    ocio_rs_bridge::get_real_log_camera_transform(handle)->getLinSideOffsetValue(tmp);
    values[0] = tmp[0]; values[1] = tmp[1]; values[2] = tmp[2];
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_log_camera_transform_set_lin_side_offset_value(void* handle, const double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    if (!values) return;
    double tmp[3] = { values[0], values[1], values[2] };
    ocio_rs_bridge::get_real_log_camera_transform(handle)->setLinSideOffsetValue(tmp);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_log_camera_transform_get_lin_side_break_value(void* handle, double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    if (!values) return;
    double tmp[3];
    ocio_rs_bridge::get_real_log_camera_transform(handle)->getLinSideBreakValue(tmp);
    values[0] = tmp[0]; values[1] = tmp[1]; values[2] = tmp[2];
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_log_camera_transform_set_lin_side_break_value(void* handle, const double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    if (!values) return;
    double tmp[3] = { values[0], values[1], values[2] };
    ocio_rs_bridge::get_real_log_camera_transform(handle)->setLinSideBreakValue(tmp);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

bool ocio_log_camera_transform_get_linear_slope_value(void* handle, double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return false;
#else
  try {
    if (!values) return false;
    double tmp[3];
    bool result = ocio_rs_bridge::get_real_log_camera_transform(handle)->getLinearSlopeValue(tmp);
    values[0] = tmp[0]; values[1] = tmp[1]; values[2] = tmp[2];
    return result;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

void ocio_log_camera_transform_set_linear_slope_value(void* handle, const double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
  return;
#else
  try {
    if (!values) return;
    double tmp[3] = { values[0], values[1], values[2] };
    ocio_rs_bridge::get_real_log_camera_transform(handle)->setLinearSlopeValue(tmp);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_log_camera_transform_unset_linear_slope_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return;
#else
  try {
    ocio_rs_bridge::get_real_log_camera_transform(handle)->unsetLinearSlopeValue();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}


// --- LogTransform ---

void* ocio_log_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_log_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_log_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_log_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LogTransformHandle*>(handle);
}

void* ocio_log_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_log_transform_get_format_metadata_v1(void* handle) {
  return ocio_log_transform_get_format_metadata(handle);
}

void* ocio_log_transform_get_format_metadata_v2(void* handle) {
  return ocio_log_transform_get_format_metadata(handle);
}

bool ocio_log_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_log_transform(handle)->equals(
      *ocio_rs_bridge::get_real_log_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- LookTransform ---

void* ocio_look_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_look_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_look_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_look_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LookTransformHandle*>(handle);
}

void* ocio_look_transform_create_editable_copy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return ocio_rs_bridge::make_stub_look_transform().release();
#else
  try {
    auto result = ocio_rs_bridge::get_real_look_transform(handle)->createEditableCopy();
    if (!result) return nullptr;
    auto typed = std::dynamic_pointer_cast<ocio::LookTransform>(result);
    if (!typed) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::LookTransformHandle>();
    out_handle->inner = std::make_shared<ocio_rs_bridge::RealLookTransform>(
        ocio_rs_bridge::RealLookTransform{typed});
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_look_transform_validate(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  ocio_rs_bridge::clear_last_error();
  return;
#else
  ocio_rs_bridge::clear_last_error();
  try {
    ocio_rs_bridge::get_real_look_transform(handle)->validate();
  } catch (...) {
    ocio_rs_bridge::capture_current_exception();
    return;
  }
#endif
}

void* ocio_look_transform_get_src(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_look_transform(handle)->getSrc()));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_look_transform_set_src(void* handle, const char* src) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)src;
  return;
#else
  try {
    ocio_rs_bridge::get_real_look_transform(handle)->setSrc(src);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_look_transform_get_dst(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_look_transform(handle)->getDst()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_look_transform_get_looks(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    return const_cast<void*>(static_cast<const void*>(ocio_rs_bridge::get_real_look_transform(handle)->getLooks()));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- Lut1DTransform ---

void* ocio_lut1d_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_lut1d_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_lut1d_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_lut1d_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(handle);
}

void* ocio_lut1d_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

int ocio_lut1d_transform_get_file_output_bit_depth(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_lut1d_transform(handle)->getFileOutputBitDepth();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_lut1d_transform_set_file_output_bit_depth(void* handle, int bitDepth) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)bitDepth;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->setFileOutputBitDepth(static_cast<ocio::BitDepth>(bitDepth));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_lut1d_transform_get_format_metadata_v1(void* handle) {
  return ocio_lut1d_transform_get_format_metadata(handle);
}

void* ocio_lut1d_transform_get_format_metadata_v2(void* handle) {
  return ocio_lut1d_transform_get_format_metadata(handle);
}

bool ocio_lut1d_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_lut1d_transform(handle)->equals(
      *ocio_rs_bridge::get_real_lut1d_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

void* ocio_lut1d_transform_get_length(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto length = ocio_rs_bridge::get_real_lut1d_transform(handle)->getLength();
    return reinterpret_cast<void*>(static_cast<uintptr_t>(length));
  } catch (...) { return nullptr; }
#endif
}

void ocio_lut1d_transform_set_length(void* handle, void* length) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)length;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->setLength(
        static_cast<unsigned long>(reinterpret_cast<uintptr_t>(length)));
  } catch (...) { return ; }
#endif
}

uint64_t ocio_lut1d_transform_get_length_u64(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return static_cast<uint64_t>(ocio_rs_bridge::get_real_lut1d_transform(handle)->getLength());
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_lut1d_transform_set_length_u64(void* handle, uint64_t length) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)length;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->setLength(static_cast<unsigned long>(length));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_lut1d_transform_get_value(void* handle, void* index, void* r, void* g, void* b) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)r; (void)g; (void)b;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->getValue(
        static_cast<unsigned long>(reinterpret_cast<uintptr_t>(index)),
        *static_cast<float*>(r), *static_cast<float*>(g), *static_cast<float*>(b));
  } catch (...) { return ; }
#endif
}

void ocio_lut1d_transform_set_value(void* handle, void* index, float r, float g, float b) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)index; (void)r; (void)g; (void)b;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut1d_transform(handle)->setValue(
        static_cast<unsigned long>(reinterpret_cast<uintptr_t>(index)), r, g, b);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- Lut3DTransform ---

void* ocio_lut3d_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_lut3d_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_lut3d_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_lut3d_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(handle);
}

void* ocio_lut3d_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_lut3d_transform_get_format_metadata_v1(void* handle) {
  return ocio_lut3d_transform_get_format_metadata(handle);
}

void* ocio_lut3d_transform_get_format_metadata_v2(void* handle) {
  return ocio_lut3d_transform_get_format_metadata(handle);
}

bool ocio_lut3d_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_lut3d_transform(handle)->equals(
      *ocio_rs_bridge::get_real_lut3d_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

void* ocio_lut3d_transform_get_grid_size(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle; 
  return nullptr;
#else
  try {
    auto gridSize = ocio_rs_bridge::get_real_lut3d_transform(handle)->getGridSize();
    return reinterpret_cast<void*>(static_cast<uintptr_t>(gridSize));
  } catch (...) { return nullptr; }
#endif
}

void ocio_lut3d_transform_set_grid_size(void* handle, void* gridSize) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)gridSize;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut3d_transform(handle)->setGridSize(
        static_cast<unsigned long>(reinterpret_cast<uintptr_t>(gridSize)));
  } catch (...) { return ; }
#endif
}

uint64_t ocio_lut3d_transform_get_grid_size_u64(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0;
#else
  try {
    return static_cast<uint64_t>(ocio_rs_bridge::get_real_lut3d_transform(handle)->getGridSize());
  } catch (...) { return 0; }
#endif
}

void ocio_lut3d_transform_set_grid_size_u64(void* handle, uint64_t gridSize) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)gridSize;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut3d_transform(handle)->setGridSize(static_cast<unsigned long>(gridSize));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_lut3d_transform_get_value(void* handle, void* indexR, void* indexG, void* indexB, void* r, void* g, void* b) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)indexR; (void)indexG; (void)indexB; (void)r; (void)g; (void)b;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut3d_transform(handle)->getValue(
        static_cast<unsigned long>(reinterpret_cast<uintptr_t>(indexR)),
        static_cast<unsigned long>(reinterpret_cast<uintptr_t>(indexG)),
        static_cast<unsigned long>(reinterpret_cast<uintptr_t>(indexB)),
        *static_cast<float*>(r), *static_cast<float*>(g), *static_cast<float*>(b));
  } catch (...) { return ; }
#endif
}

void ocio_lut3d_transform_set_value(void* handle, void* indexR, void* indexG, void* indexB, float r, float g, float b) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)indexR; (void)indexG; (void)indexB; (void)r; (void)g; (void)b;
  return;
#else
  try {
    ocio_rs_bridge::get_real_lut3d_transform(handle)->setValue(
        static_cast<unsigned long>(reinterpret_cast<uintptr_t>(indexR)),
        static_cast<unsigned long>(reinterpret_cast<uintptr_t>(indexG)),
        static_cast<unsigned long>(reinterpret_cast<uintptr_t>(indexB)),
        r, g, b);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- MatrixTransform ---

void* ocio_matrix_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_matrix_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_matrix_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_matrix_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::MatrixTransformHandle*>(handle);
}

void* ocio_matrix_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
}

void* ocio_matrix_transform_get_format_metadata_v1(void* handle) {
  return ocio_matrix_transform_get_format_metadata(handle);
}

void* ocio_matrix_transform_get_format_metadata_v2(void* handle) {
  return ocio_matrix_transform_get_format_metadata(handle);
}

bool ocio_matrix_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_matrix_transform(handle)->equals(
      *ocio_rs_bridge::get_real_matrix_transform(other));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return false; }
#endif
}

void ocio_matrix_transform_get_matrix(void* handle, void* m44) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)m44;
  return;
#else
  try {
    ocio_rs_bridge::get_real_matrix_transform(handle)->getMatrix(static_cast<double*>(m44));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_matrix_transform_set_matrix(void* handle, const double* m44) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)m44;
  return;
#else
  try {
    ocio_rs_bridge::get_real_matrix_transform(handle)->setMatrix(m44);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_matrix_transform_get_offset(void* handle, void* offset4) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)offset4;
  return;
#else
  try {
    ocio_rs_bridge::get_real_matrix_transform(handle)->getOffset(static_cast<double*>(offset4));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return ; }
#endif
}

void ocio_matrix_transform_set_offset(void* handle, const double* offset4) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)offset4;
  return;
#else
  try {
    ocio_rs_bridge::get_real_matrix_transform(handle)->setOffset(offset4);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- RangeTransform ---

void* ocio_range_transform_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_range_transform().release();
#else
  try {
    auto handle = ocio_rs_bridge::make_real_range_transform();
    if (!handle) return nullptr;
    return handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_range_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::RangeTransformHandle*>(handle);
}

void* ocio_range_transform_get_format_metadata(void* handle) {
  return ocio_transform_get_format_metadata(handle);
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void* ocio_range_transform_get_format_metadata_v1(void* handle) {
  return ocio_range_transform_get_format_metadata(handle);
}

void* ocio_range_transform_get_format_metadata_v2(void* handle) {
  return ocio_range_transform_get_format_metadata(handle);
}

bool ocio_range_transform_equals(void* handle, void* other) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)other;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_range_transform(handle)->equals(
      *ocio_rs_bridge::get_real_range_transform(other));
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_range_transform_set_max_in_value(void* handle, double val) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)val;
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->setMaxInValue(val);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_range_transform_set_min_out_value(void* handle, double val) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)val;
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->setMinOutValue(val);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}

void ocio_range_transform_set_max_out_value(void* handle, double val) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)val;
  return;
#else
  try {
    ocio_rs_bridge::get_real_range_transform(handle)->setMaxOutValue(val);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
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
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#endif
}


// --- Consolidated real compatibility ABI ---

void* ocio_context_create_editable_copy(void* context) {
#ifdef OCIO_RS_STUB
  (void)context;
  return ocio_rs_bridge::make_stub_context().release();
#else
  try {
    auto copy = ocio_rs_bridge::get_real_context(context)->createEditableCopy();
    if (!copy) return nullptr;
    auto out_handle = std::make_unique<ocio_rs_bridge::ContextHandle>();
    auto obj = std::make_shared<ocio_rs_bridge::RealContext>();
    obj->context = copy;
    out_handle->inner = obj;
    return out_handle.release();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void* ocio_format_metadata_get_child_element(void* metadata, int i) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)i;
  return nullptr;
#else
  try {
    auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata);
    if (!format_metadata) return nullptr;
    auto* parent_handle = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    return ocio_rs_bridge::make_format_metadata_handle(
      parent_handle ? parent_handle->owner : std::shared_ptr<void>{},
      &format_metadata->getChildElement(i));
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_format_metadata_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::FormatMetadataHandle*>(handle);
}

#ifndef OCIO_RS_STUB
static ocio::FormatMetadata* get_format_metadata_from_transform(const ocio::TransformRcPtr& transform) {
  if (auto t = std::dynamic_pointer_cast<ocio::CDLTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::ExponentTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::ExponentWithLinearTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::ExposureContrastTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::FixedFunctionTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::GradingPrimaryTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::GradingRGBCurveTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::GradingHueCurveTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::GradingToneTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::GroupTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::LogAffineTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::LogCameraTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::LogTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::Lut1DTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::Lut3DTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::MatrixTransform>(transform)) return &t->getFormatMetadata();
  if (auto t = std::dynamic_pointer_cast<ocio::RangeTransform>(transform)) return &t->getFormatMetadata();
  return nullptr;
}
#endif

void* ocio_transform_get_format_metadata(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return ocio_rs_bridge::make_stub_format_metadata().release();
#else
  try {
    auto* base = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    if (!base) return nullptr;
    auto transform_ptr = base->get_ocio_transform();
    if (!transform_ptr) return nullptr;
    return ocio_rs_bridge::make_format_metadata_handle(
      std::make_shared<ocio::TransformRcPtr>(transform_ptr),
      get_format_metadata_from_transform(transform_ptr));
  }
  catch (...) { return nullptr; }
#endif
}

const char* ocio_format_metadata_get_element_name(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return nullptr;
#else
  try {
    auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata);
    return format_metadata ? format_metadata->getElementName() : nullptr;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_format_metadata_set_element_name(void* metadata, const char* name) {
#ifndef OCIO_RS_STUB
  try {
    if (auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata)) {
      format_metadata->setElementName(name);
    }
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)metadata; (void)name;
#endif
}

const char* ocio_format_metadata_get_element_value(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return nullptr;
#else
  try {
    auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata);
    return format_metadata ? format_metadata->getElementValue() : nullptr;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

void ocio_format_metadata_set_element_value(void* metadata, const char* value) {
#ifndef OCIO_RS_STUB
  try {
    if (auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata)) {
      format_metadata->setElementValue(value);
    }
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)metadata; (void)value;
#endif
}

int ocio_format_metadata_get_num_attributes(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return 0;
#else
  try {
    auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata);
    return format_metadata ? format_metadata->getNumAttributes() : 0;
  } catch (...) { return 0; }
#endif
}

const char* ocio_format_metadata_get_attribute_name(void* metadata, int i) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)i; return nullptr;
#else
  try {
    auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata);
    return format_metadata ? format_metadata->getAttributeName(i) : nullptr;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return nullptr; }
#endif
}

const char* ocio_format_metadata_get_attribute_value_by_index(void* metadata, int i) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)i; return nullptr;
#else
  try {
    auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata);
    return format_metadata ? format_metadata->getAttributeValue(i) : nullptr;
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_format_metadata_get_attribute_value(void* metadata, const char* name) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)name; return nullptr;
#else
  try {
    auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata);
    return format_metadata ? format_metadata->getAttributeValue(name) : nullptr;
  } catch (...) { return nullptr; }
#endif
}

void ocio_format_metadata_add_attribute(void* metadata, const char* name, const char* value) {
#ifndef OCIO_RS_STUB
  try {
    if (auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata)) {
      format_metadata->addAttribute(name, value);
    }
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)metadata; (void)name; (void)value;
#endif
}

int ocio_format_metadata_get_num_children_elements(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return 0;
#else
  try {
    auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata);
    return format_metadata ? format_metadata->getNumChildrenElements() : 0;
  } catch (...) { return 0; }
#endif
}

void ocio_format_metadata_add_child_element(void* metadata, const char* name, const char* value) {
#ifndef OCIO_RS_STUB
  try {
    if (auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata)) {
      format_metadata->addChildElement(name, value);
    }
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)metadata; (void)name; (void)value;
#endif
}

void ocio_format_metadata_clear(void* metadata) {
#ifndef OCIO_RS_STUB
  try {
    if (auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata)) {
      format_metadata->clear();
    }
  } catch (...) { return; }
#else
  (void)metadata;
#endif
}

const char* ocio_format_metadata_get_name(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return nullptr;
#else
  try {
    auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata);
    return format_metadata ? format_metadata->getName() : nullptr;
  } catch (...) { return nullptr; }
#endif
}

void ocio_format_metadata_set_name(void* metadata, const char* name) {
#ifndef OCIO_RS_STUB
  try {
    if (auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata)) {
      format_metadata->setName(name);
    }
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)metadata; (void)name;
#endif
}

const char* ocio_format_metadata_get_id(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return nullptr;
#else
  try {
    auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata);
    return format_metadata ? format_metadata->getID() : nullptr;
  } catch (...) { return nullptr; }
#endif
}

void ocio_format_metadata_set_id(void* metadata, const char* id) {
#ifndef OCIO_RS_STUB
  try {
    if (auto* format_metadata = ocio_rs_bridge::get_real_format_metadata(metadata)) {
      format_metadata->setID(id);
    }
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)metadata; (void)id;
#endif
}

#ifdef OCIO_RS_STUB
#define OCIO_RS_DEFINE_DIRECTION_ABI(prefix, getter) \
int prefix##_get_direction(void* transform) { (void)transform; return 0; } \
void prefix##_set_direction(void* transform, int direction) { (void)transform; (void)direction; }
#else
#define OCIO_RS_DEFINE_DIRECTION_ABI(prefix, getter) \
int prefix##_get_direction(void* transform) { \
  try { return static_cast<int>(ocio_rs_bridge::getter(transform)->getDirection()); } catch (...) { return 0; } \
} \
void prefix##_set_direction(void* transform, int direction) { \
  try { ocio_rs_bridge::getter(transform)->setDirection(static_cast<ocio::TransformDirection>(direction)); } catch (...) { ocio_rs_bridge::capture_current_exception(); return; } \
}
#endif

OCIO_RS_DEFINE_DIRECTION_ABI(ocio_builtin_transform, get_real_builtin_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_cdl_transform, get_real_cdl_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_exponent_transform, get_real_exponent_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_exponent_with_linear_transform, get_real_exponent_with_linear_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_exposure_contrast_transform, get_real_exposure_contrast_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_grading_primary_transform, get_real_grading_primary_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_grading_tone_transform, get_real_grading_tone_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_group_transform, get_real_group_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_log_affine_transform, get_real_log_affine_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_log_transform, get_real_log_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_lut1d_transform, get_real_lut1d_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_lut3d_transform, get_real_lut3d_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_matrix_transform, get_real_matrix_transform)
OCIO_RS_DEFINE_DIRECTION_ABI(ocio_range_transform, get_real_range_transform)

#undef OCIO_RS_DEFINE_DIRECTION_ABI

void ocio_exponent_transform_get_value(void* transform, double* vec4) {
#ifndef OCIO_RS_STUB
  try { double values[4]{}; ocio_rs_bridge::get_real_exponent_transform(transform)->getValue(values); std::memcpy(vec4, values, sizeof(values)); } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform; (void)vec4;
#endif
}

void ocio_exponent_transform_set_value(void* transform, const double* vec4) {
#ifndef OCIO_RS_STUB
  try { double values[4]{}; std::memcpy(values, vec4, sizeof(values)); ocio_rs_bridge::get_real_exponent_transform(transform)->setValue(values); } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform; (void)vec4;
#endif
}

void ocio_exponent_with_linear_transform_get_gamma(void* transform, double* vec4) {
#ifndef OCIO_RS_STUB
  try { double values[4]{}; ocio_rs_bridge::get_real_exponent_with_linear_transform(transform)->getGamma(values); std::memcpy(vec4, values, sizeof(values)); } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform; (void)vec4;
#endif
}

void ocio_exponent_with_linear_transform_set_gamma(void* transform, const double* vec4) {
#ifndef OCIO_RS_STUB
  try { double values[4]{}; std::memcpy(values, vec4, sizeof(values)); ocio_rs_bridge::get_real_exponent_with_linear_transform(transform)->setGamma(values); } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform; (void)vec4;
#endif
}

void ocio_exponent_with_linear_transform_get_offset(void* transform, double* vec4) {
#ifndef OCIO_RS_STUB
  try { double values[4]{}; ocio_rs_bridge::get_real_exponent_with_linear_transform(transform)->getOffset(values); std::memcpy(vec4, values, sizeof(values)); } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform; (void)vec4;
#endif
}

void ocio_exponent_with_linear_transform_set_offset(void* transform, const double* vec4) {
#ifndef OCIO_RS_STUB
  try { double values[4]{}; std::memcpy(values, vec4, sizeof(values)); ocio_rs_bridge::get_real_exponent_with_linear_transform(transform)->setOffset(values); } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform; (void)vec4;
#endif
}

#ifdef OCIO_RS_STUB
#define OCIO_RS_LOG_AFFINE_VEC3_ABI(suffix, getter_name, setter_name) \
void ocio_log_affine_transform_get_##suffix(void* transform, double* values) { (void)transform; (void)values; } \
void ocio_log_affine_transform_set_##suffix(void* transform, const double* values) { (void)transform; (void)values; }
#else
#define OCIO_RS_LOG_AFFINE_VEC3_ABI(suffix, getter_name, setter_name) \
void ocio_log_affine_transform_get_##suffix(void* transform, double* values) { \
  try { if (!values) return; double local[3]{}; ocio_rs_bridge::get_real_log_affine_transform(transform)->getter_name(local); std::memcpy(values, local, sizeof(local)); } catch (...) { ocio_rs_bridge::capture_current_exception(); return; } \
} \
void ocio_log_affine_transform_set_##suffix(void* transform, const double* values) { \
  try { if (!values) return; double local[3]{}; std::memcpy(local, values, sizeof(local)); ocio_rs_bridge::get_real_log_affine_transform(transform)->setter_name(local); } catch (...) { ocio_rs_bridge::capture_current_exception(); return; } \
}
#endif

OCIO_RS_LOG_AFFINE_VEC3_ABI(log_side_slope_value, getLogSideSlopeValue, setLogSideSlopeValue)
OCIO_RS_LOG_AFFINE_VEC3_ABI(log_side_offset_value, getLogSideOffsetValue, setLogSideOffsetValue)
OCIO_RS_LOG_AFFINE_VEC3_ABI(lin_side_slope_value, getLinSideSlopeValue, setLinSideSlopeValue)
OCIO_RS_LOG_AFFINE_VEC3_ABI(lin_side_offset_value, getLinSideOffsetValue, setLinSideOffsetValue)

#undef OCIO_RS_LOG_AFFINE_VEC3_ABI

void ocio_lut1d_transform_get_values(void* transform, double* data) {
#ifndef OCIO_RS_STUB
  try {
    auto lut = ocio_rs_bridge::get_real_lut1d_transform(transform);
    const auto length = lut->getLength();
    for (unsigned long i = 0; i < length; ++i) {
      float r = 0.f, g = 0.f, b = 0.f;
      lut->getValue(i, r, g, b);
      data[i * 3 + 0] = r;
      data[i * 3 + 1] = g;
      data[i * 3 + 2] = b;
    }
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform; (void)data;
#endif
}

void ocio_lut1d_transform_set_values(void* transform, const double* data) {
#ifndef OCIO_RS_STUB
  try {
    auto lut = ocio_rs_bridge::get_real_lut1d_transform(transform);
    const auto length = lut->getLength();
    for (unsigned long i = 0; i < length; ++i) {
      lut->setValue(i, static_cast<float>(data[i * 3 + 0]), static_cast<float>(data[i * 3 + 1]), static_cast<float>(data[i * 3 + 2]));
    }
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform; (void)data;
#endif
}

void ocio_lut3d_transform_get_values(void* transform, double* data) {
#ifndef OCIO_RS_STUB
  try {
    auto lut = ocio_rs_bridge::get_real_lut3d_transform(transform);
    const auto size = lut->getGridSize();
    size_t out = 0;
    for (unsigned long b = 0; b < size; ++b) {
      for (unsigned long g = 0; g < size; ++g) {
        for (unsigned long r = 0; r < size; ++r) {
          float rv = 0.f, gv = 0.f, bv = 0.f;
          lut->getValue(r, g, b, rv, gv, bv);
          data[out++] = rv;
          data[out++] = gv;
          data[out++] = bv;
        }
      }
    }
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform; (void)data;
#endif
}

void ocio_lut3d_transform_set_values(void* transform, const double* data) {
#ifndef OCIO_RS_STUB
  try {
    auto lut = ocio_rs_bridge::get_real_lut3d_transform(transform);
    const auto size = lut->getGridSize();
    size_t in = 0;
    for (unsigned long b = 0; b < size; ++b) {
      for (unsigned long g = 0; g < size; ++g) {
        for (unsigned long r = 0; r < size; ++r) {
          lut->setValue(r, g, b, static_cast<float>(data[in + 0]), static_cast<float>(data[in + 1]), static_cast<float>(data[in + 2]));
          in += 3;
        }
      }
    }
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform; (void)data;
#endif
}

#ifndef OCIO_RS_STUB
static void* make_matrix_transform_from_values(const double* m44, const double* offset4) {
  auto out_handle = std::make_unique<ocio_rs_bridge::MatrixTransformHandle>();
  auto obj = std::make_shared<ocio_rs_bridge::RealMatrixTransform>();
  obj->transform = ocio::MatrixTransform::Create();
  obj->transform->setMatrix(m44);
  obj->transform->setOffset(offset4);
  out_handle->inner = obj;
  return out_handle.release();
}
#endif

void* ocio_matrix_transform_create_identity(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_matrix_transform().release();
#else
  try {
    double m44[16]{};
    double offset4[4]{};
    ocio::MatrixTransform::Identity(m44, offset4);
    return make_matrix_transform_from_values(m44, offset4);
  } catch (...) { return nullptr; }
#endif
}

void ocio_matrix_transform_identity(double* m44, double* offset4) {
#ifdef OCIO_RS_STUB
  (void)m44; (void)offset4;
  return;
#else
  try {
    ocio::MatrixTransform::Identity(m44, offset4);
  } catch (...) { return; }
#endif
}

void* ocio_matrix_transform_create_sat(double sat, const double* luma) {
#ifdef OCIO_RS_STUB
  (void)sat; (void)luma;
  return ocio_rs_bridge::make_stub_matrix_transform().release();
#else
  try {
    double m44[16]{};
    double offset4[4]{};
    ocio::MatrixTransform::Sat(m44, offset4, sat, luma);
    return make_matrix_transform_from_values(m44, offset4);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_matrix_transform_create_scale(const double* scale) {
#ifdef OCIO_RS_STUB
  (void)scale;
  return ocio_rs_bridge::make_stub_matrix_transform().release();
#else
  try {
    double m44[16]{};
    double offset4[4]{};
    ocio::MatrixTransform::Scale(m44, offset4, scale);
    return make_matrix_transform_from_values(m44, offset4);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_matrix_transform_create_fit(const double* oldMin4, const double* oldMax4, const double* newMin4, const double* newMax4) {
#ifdef OCIO_RS_STUB
  (void)oldMin4; (void)oldMax4; (void)newMin4; (void)newMax4;
  return ocio_rs_bridge::make_stub_matrix_transform().release();
#else
  try {
    double m44[16]{};
    double offset4[4]{};
    ocio::MatrixTransform::Fit(m44, offset4, oldMin4, oldMax4, newMin4, newMax4);
    return make_matrix_transform_from_values(m44, offset4);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_matrix_transform_create_view(int* channels, const double* luma) {
#ifdef OCIO_RS_STUB
  (void)channels; (void)luma;
  return ocio_rs_bridge::make_stub_matrix_transform().release();
#else
  try {
    double m44[16]{};
    double offset4[4]{};
    ocio::MatrixTransform::View(m44, offset4, channels, luma);
    return make_matrix_transform_from_values(m44, offset4);
  } catch (...) { return nullptr; }
#endif
}

void ocio_group_transform_remove_transform(void* transform, uint64_t index) {
#ifndef OCIO_RS_STUB
  try {
    auto* handle = static_cast<ocio_rs_bridge::GroupTransformHandle*>(transform);
    auto obj = std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(handle->inner);
    if (index >= static_cast<uint64_t>(obj->transform->getNumTransforms())) {
      throw std::out_of_range("GroupTransform child index is out of range");
    }
    auto replacement = ocio::GroupTransform::Create();
    replacement->setDirection(obj->transform->getDirection());
    replacement->getFormatMetadata() = obj->transform->getFormatMetadata();
    const int remove_index = static_cast<int>(index);
    for (int i = 0; i < obj->transform->getNumTransforms(); ++i) {
      if (i == remove_index) continue;
      auto child = obj->transform->getTransform(i);
      if (child) replacement->appendTransform(child);
    }
    obj->transform = replacement;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform; (void)index;
#endif
}

void ocio_group_transform_clear_transforms(void* transform) {
#ifndef OCIO_RS_STUB
  try {
    auto* handle = static_cast<ocio_rs_bridge::GroupTransformHandle*>(transform);
    auto obj = std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(handle->inner);
    auto replacement = ocio::GroupTransform::Create();
    replacement->setDirection(obj->transform->getDirection());
    replacement->getFormatMetadata() = obj->transform->getFormatMetadata();
    obj->transform = replacement;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return; }
#else
  (void)transform;
#endif
}

// --- DynamicProperty ---

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

double ocio_dynamic_property_double_get_value(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
  return 0.0;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    return ocio::DynamicPropertyValue::AsDouble(prop)->getValue();
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0.0; }
#endif
}

void ocio_dynamic_property_double_set_value(void* handle, double value) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)value;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    ocio::DynamicPropertyValue::AsDouble(prop)->setValue(value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_dynamic_property_grading_primary_get_value(void* handle, double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
#else
  try {
    if (!values) return;
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    const auto& v = ocio::DynamicPropertyValue::AsGradingPrimary(prop)->getValue();
    size_t off = 0;
    auto write_rgbm = [&](const ocio::GradingRGBM& rgbm) {
      values[off++] = rgbm.m_red;
      values[off++] = rgbm.m_green;
      values[off++] = rgbm.m_blue;
      values[off++] = rgbm.m_master;
    };
    write_rgbm(v.m_brightness);
    write_rgbm(v.m_contrast);
    write_rgbm(v.m_gamma);
    write_rgbm(v.m_offset);
    write_rgbm(v.m_exposure);
    write_rgbm(v.m_lift);
    write_rgbm(v.m_gain);
    values[off++] = v.m_saturation;
    values[off++] = v.m_pivot;
    values[off++] = v.m_pivotBlack;
    values[off++] = v.m_pivotWhite;
    values[off++] = v.m_clampBlack;
    values[off++] = v.m_clampWhite;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_dynamic_property_grading_primary_set_value(void* handle, const double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
#else
  try {
    if (!values) return;
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto typed = ocio::DynamicPropertyValue::AsGradingPrimary(prop);
    auto v = typed->getValue();
    size_t off = 0;
    auto read_rgbm = [&]() {
      ocio::GradingRGBM rgbm;
      rgbm.m_red = values[off++];
      rgbm.m_green = values[off++];
      rgbm.m_blue = values[off++];
      rgbm.m_master = values[off++];
      return rgbm;
    };
    v.m_brightness = read_rgbm();
    v.m_contrast = read_rgbm();
    v.m_gamma = read_rgbm();
    v.m_offset = read_rgbm();
    v.m_exposure = read_rgbm();
    v.m_lift = read_rgbm();
    v.m_gain = read_rgbm();
    v.m_saturation = values[off++];
    v.m_pivot = values[off++];
    v.m_pivotBlack = values[off++];
    v.m_pivotWhite = values[off++];
    v.m_clampBlack = values[off++];
    v.m_clampWhite = values[off++];
    typed->setValue(v);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_dynamic_property_grading_tone_get_value(void* handle, double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
#else
  try {
    if (!values) return;
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    const auto& v = ocio::DynamicPropertyValue::AsGradingTone(prop)->getValue();
    size_t off = 0;
    auto write_rgbmsw = [&](const ocio::GradingRGBMSW& rgbmsw) {
      values[off++] = rgbmsw.m_red;
      values[off++] = rgbmsw.m_green;
      values[off++] = rgbmsw.m_blue;
      values[off++] = rgbmsw.m_master;
      values[off++] = rgbmsw.m_start;
      values[off++] = rgbmsw.m_width;
    };
    write_rgbmsw(v.m_blacks);
    write_rgbmsw(v.m_shadows);
    write_rgbmsw(v.m_midtones);
    write_rgbmsw(v.m_highlights);
    write_rgbmsw(v.m_whites);
    values[off++] = v.m_scontrast;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_dynamic_property_grading_tone_set_value(void* handle, const double* values) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)values;
#else
  try {
    if (!values) return;
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto typed = ocio::DynamicPropertyValue::AsGradingTone(prop);
    auto v = typed->getValue();
    size_t off = 0;
    auto read_rgbmsw = [&]() {
      ocio::GradingRGBMSW rgbmsw;
      rgbmsw.m_red = values[off++];
      rgbmsw.m_green = values[off++];
      rgbmsw.m_blue = values[off++];
      rgbmsw.m_master = values[off++];
      rgbmsw.m_start = values[off++];
      rgbmsw.m_width = values[off++];
      return rgbmsw;
    };
    v.m_blacks = read_rgbmsw();
    v.m_shadows = read_rgbmsw();
    v.m_midtones = read_rgbmsw();
    v.m_highlights = read_rgbmsw();
    v.m_whites = read_rgbmsw();
    v.m_scontrast = values[off++];
    typed->setValue(v);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

int ocio_dynamic_property_grading_rgb_curve_get_num_control_points(void* handle, int curveType) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType;
  return 0;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto value = ocio::DynamicPropertyValue::AsGradingRGBCurve(prop)->getValue();
    auto curve = value->getCurve(static_cast<ocio::RGBCurveType>(curveType));
    return curve ? static_cast<int>(curve->getNumControlPoints()) : 0;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_dynamic_property_grading_rgb_curve_set_num_control_points(void* handle, int curveType, int num) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType; (void)num;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto typed = ocio::DynamicPropertyValue::AsGradingRGBCurve(prop);
    auto value = ocio::GradingRGBCurve::Create(typed->getValue());
    auto curve = value->getCurve(static_cast<ocio::RGBCurveType>(curveType));
    if (!curve) return;
    curve->setNumControlPoints(static_cast<size_t>(num));
    typed->setValue(value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_dynamic_property_grading_rgb_curve_get_control_point(void* handle, int curveType, int index, float* x, float* y) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType; (void)index; (void)x; (void)y;
#else
  try {
    if (!x || !y) return;
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto value = ocio::DynamicPropertyValue::AsGradingRGBCurve(prop)->getValue();
    auto curve = value->getCurve(static_cast<ocio::RGBCurveType>(curveType));
    if (!curve) return;
    const auto& point = curve->getControlPoint(static_cast<size_t>(index));
    *x = point.m_x;
    *y = point.m_y;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_dynamic_property_grading_rgb_curve_set_control_point(void* handle, int curveType, int index, float x, float y) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType; (void)index; (void)x; (void)y;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto typed = ocio::DynamicPropertyValue::AsGradingRGBCurve(prop);
    auto value = ocio::GradingRGBCurve::Create(typed->getValue());
    auto curve = value->getCurve(static_cast<ocio::RGBCurveType>(curveType));
    if (!curve) return;
    auto& point = curve->getControlPoint(static_cast<size_t>(index));
    point.m_x = x;
    point.m_y = y;
    typed->setValue(value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

float ocio_dynamic_property_grading_rgb_curve_get_slope(void* handle, int curveType, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType; (void)index;
  return 0.0f;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto value = ocio::DynamicPropertyValue::AsGradingRGBCurve(prop)->getValue();
    auto curve = value->getCurve(static_cast<ocio::RGBCurveType>(curveType));
    return curve ? curve->getSlope(static_cast<size_t>(index)) : 0.0f;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0.0f; }
#endif
}

void ocio_dynamic_property_grading_rgb_curve_set_slope(void* handle, int curveType, int index, float slope) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType; (void)index; (void)slope;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto typed = ocio::DynamicPropertyValue::AsGradingRGBCurve(prop);
    auto value = ocio::GradingRGBCurve::Create(typed->getValue());
    auto curve = value->getCurve(static_cast<ocio::RGBCurveType>(curveType));
    if (!curve) return;
    curve->setSlope(static_cast<size_t>(index), slope);
    typed->setValue(value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

bool ocio_dynamic_property_grading_rgb_curve_slopes_are_default(void* handle, int curveType) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType;
  return true;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto value = ocio::DynamicPropertyValue::AsGradingRGBCurve(prop)->getValue();
    auto curve = value->getCurve(static_cast<ocio::RGBCurveType>(curveType));
    return curve ? curve->slopesAreDefault() : true;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return true; }
#endif
}

int ocio_dynamic_property_grading_hue_curve_get_num_control_points(void* handle, int curveType) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType;
  return 0;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto value = ocio::DynamicPropertyValue::AsGradingHueCurve(prop)->getValue();
    auto curve = value->getCurve(static_cast<ocio::HueCurveType>(curveType));
    return curve ? static_cast<int>(curve->getNumControlPoints()) : 0;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0; }
#endif
}

void ocio_dynamic_property_grading_hue_curve_set_num_control_points(void* handle, int curveType, int num) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType; (void)num;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto typed = ocio::DynamicPropertyValue::AsGradingHueCurve(prop);
    auto value = ocio::GradingHueCurve::Create(typed->getValue());
    auto curve = value->getCurve(static_cast<ocio::HueCurveType>(curveType));
    if (!curve) return;
    curve->setNumControlPoints(static_cast<size_t>(num));
    typed->setValue(value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_dynamic_property_grading_hue_curve_get_control_point(void* handle, int curveType, int index, float* x, float* y) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType; (void)index; (void)x; (void)y;
#else
  try {
    if (!x || !y) return;
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto value = ocio::DynamicPropertyValue::AsGradingHueCurve(prop)->getValue();
    auto curve = value->getCurve(static_cast<ocio::HueCurveType>(curveType));
    if (!curve) return;
    const auto& point = curve->getControlPoint(static_cast<size_t>(index));
    *x = point.m_x;
    *y = point.m_y;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

void ocio_dynamic_property_grading_hue_curve_set_control_point(void* handle, int curveType, int index, float x, float y) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType; (void)index; (void)x; (void)y;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto typed = ocio::DynamicPropertyValue::AsGradingHueCurve(prop);
    auto value = ocio::GradingHueCurve::Create(typed->getValue());
    auto curve = value->getCurve(static_cast<ocio::HueCurveType>(curveType));
    if (!curve) return;
    auto& point = curve->getControlPoint(static_cast<size_t>(index));
    point.m_x = x;
    point.m_y = y;
    typed->setValue(value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

float ocio_dynamic_property_grading_hue_curve_get_slope(void* handle, int curveType, int index) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType; (void)index;
  return 0.0f;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto value = ocio::DynamicPropertyValue::AsGradingHueCurve(prop)->getValue();
    auto curve = value->getCurve(static_cast<ocio::HueCurveType>(curveType));
    return curve ? curve->getSlope(static_cast<size_t>(index)) : 0.0f;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return 0.0f; }
#endif
}

void ocio_dynamic_property_grading_hue_curve_set_slope(void* handle, int curveType, int index, float slope) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType; (void)index; (void)slope;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto typed = ocio::DynamicPropertyValue::AsGradingHueCurve(prop);
    auto value = ocio::GradingHueCurve::Create(typed->getValue());
    auto curve = value->getCurve(static_cast<ocio::HueCurveType>(curveType));
    if (!curve) return;
    curve->setSlope(static_cast<size_t>(index), slope);
    typed->setValue(value);
  } catch (...) { ocio_rs_bridge::capture_current_exception(); }
#endif
}

bool ocio_dynamic_property_grading_hue_curve_slopes_are_default(void* handle, int curveType) {
#ifdef OCIO_RS_STUB
  (void)handle; (void)curveType;
  return true;
#else
  try {
    auto prop = ocio_rs_bridge::get_real_dynamic_property(handle);
    auto value = ocio::DynamicPropertyValue::AsGradingHueCurve(prop)->getValue();
    auto curve = value->getCurve(static_cast<ocio::HueCurveType>(curveType));
    return curve ? curve->slopesAreDefault() : true;
  } catch (...) { ocio_rs_bridge::capture_current_exception(); return true; }
#endif
}


const char* ocio_error_get_last(void) {
  return ocio_rs_bridge::g_last_error.empty() ? nullptr : ocio_rs_bridge::g_last_error.c_str();
}

void ocio_error_clear_last(void) {
  ocio_rs_bridge::clear_last_error();
}

}  // extern "C"
