#include "bridge.hpp"

#include <cstring>
#include <memory>
#include <stdexcept>
#include <string>
#include <fstream>

#ifndef OCIO_RS_STUB
#include <OpenColorIO/OpenColorIO.h>
#include <OpenColorIO/OpenColorTransforms.h>
#endif

namespace ocio_rs_bridge {

// --- Handle types ---

struct ConfigHandle {
  std::shared_ptr<void> inner;
};

struct ProcessorHandle {
  std::shared_ptr<void> inner;
};

struct CPUProcessorHandle {
  std::shared_ptr<void> inner;
};

struct GPUProcessorHandle {
  std::shared_ptr<void> inner;
};

struct GpuShaderDescHandle {
  std::shared_ptr<void> inner;
};

// Transform base for cross-type dispatch
struct TransformHandleBase {
  virtual ~TransformHandleBase() = default;
  virtual int get_transform_type_tag() const = 0;
#ifndef OCIO_RS_STUB
  virtual OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() = 0;
#endif
};

// Transform handles — all inherit from TransformHandleBase for GroupTransform dispatch
struct FileTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 8; } // TRANSFORM_TYPE_FILE
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct CDLTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 2; } // TRANSFORM_TYPE_CDL
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct ExponentTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 5; } // TRANSFORM_TYPE_EXPONENT
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct ExponentWithLinearTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 6; } // TRANSFORM_TYPE_EXPONENT_WITH_LINEAR
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct MatrixTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 21; } // TRANSFORM_TYPE_MATRIX
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct LogTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 17; } // TRANSFORM_TYPE_LOG
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct RangeTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 22; } // TRANSFORM_TYPE_RANGE
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};
struct GroupTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 14; } // TRANSFORM_TYPE_GROUP
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};

struct BuiltinTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 1; } // TRANSFORM_TYPE_BUILTIN
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};

struct FixedFunctionTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 9; } // TRANSFORM_TYPE_FIXED_FUNCTION
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};

struct Lut1DTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 19; } // TRANSFORM_TYPE_LUT1D
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};

struct Lut3DTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 20; } // TRANSFORM_TYPE_LUT3D
#ifndef OCIO_RS_STUB
  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;
#endif
};

// Baker / Context / ColorSpace handles
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
struct ExposureContrastTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 7; }
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};
struct ColorSpaceTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 3; }
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};
struct LookTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 18; }
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};
struct GradingRGBCurveTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 12; }
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};
struct GradingHueCurveTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 10; }
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};
struct GradingPrimaryTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 11; }
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};
struct GradingToneTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 13; }
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};
struct AllocationTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 0; }
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};
struct LogAffineTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 15; }
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};
struct LogCameraTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 16; }
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};
struct DisplayViewTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 4; } // TRANSFORM_TYPE_DISPLAY_VIEW
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};

#ifdef OCIO_RS_STUB

// Stub types
struct StubConfig {};
struct StubProcessor {};
struct StubCPUProcessor {};
struct StubGPUProcessor {};
struct StubGpuShaderDesc {};
struct StubFileTransform {};
struct StubCDLTransform {};
struct StubExponentTransform {};
struct StubExponentWithLinearTransform {};
struct StubMatrixTransform {};
struct StubLogTransform {};
struct StubRangeTransform {};
struct StubGroupTransform {};
struct StubBaker {};
struct StubContext {};
struct StubColorSpace {};
struct StubFileRules {};
struct StubColorSpaceSet {};
struct StubFormatMetadata {};

// Stub factories
static std::unique_ptr<ConfigHandle> make_stub_config() {
  auto handle = std::make_unique<ConfigHandle>();
  handle->inner = std::make_shared<StubConfig>();
  return handle;
}

static std::unique_ptr<ProcessorHandle> make_stub_processor() {
  auto handle = std::make_unique<ProcessorHandle>();
  handle->inner = std::make_shared<StubProcessor>();
  return handle;
}

static std::unique_ptr<CPUProcessorHandle> make_stub_cpu_processor() {
  auto handle = std::make_unique<CPUProcessorHandle>();
  handle->inner = std::make_shared<StubCPUProcessor>();
  return handle;
}

static std::unique_ptr<GPUProcessorHandle> make_stub_gpu_processor() {
  auto handle = std::make_unique<GPUProcessorHandle>();
  handle->inner = std::make_shared<StubGPUProcessor>();
  return handle;
}

static std::unique_ptr<GpuShaderDescHandle> make_stub_gpu_shader_desc() {
  auto handle = std::make_unique<GpuShaderDescHandle>();
  handle->inner = std::make_shared<StubGpuShaderDesc>();
  return handle;
}

static std::unique_ptr<FileRulesHandle> make_stub_file_rules() {
  auto handle = std::make_unique<FileRulesHandle>();
  handle->inner = std::make_shared<StubFileRules>();
  return handle;
}

static std::unique_ptr<ColorSpaceSetHandle> make_stub_color_space_set() {
  auto handle = std::make_unique<ColorSpaceSetHandle>();
  handle->inner = std::make_shared<StubColorSpaceSet>();
  return handle;
}

static std::unique_ptr<FormatMetadataHandle> make_stub_format_metadata() {
  auto handle = std::make_unique<FormatMetadataHandle>();
  handle->inner = std::make_shared<StubFormatMetadata>();
  return handle;
}

#else

namespace ocio = OCIO_NAMESPACE;

// Real OCIO wrapper types
struct RealConfig {
  ocio::ConstConfigRcPtr config;
};

struct RealProcessor {
  ocio::ConstProcessorRcPtr processor;
};

struct RealCPUProcessor {
  ocio::ConstCPUProcessorRcPtr cpu;
};

struct RealGPUProcessor {
  ocio::ConstGPUProcessorRcPtr gpu;
};

struct RealFileTransform {
  ocio::FileTransformRcPtr transform;
};

struct RealCDLTransform {
  ocio::CDLTransformRcPtr transform;
};

struct RealExponentTransform {
  ocio::ExponentTransformRcPtr transform;
};

struct RealExponentWithLinearTransform {
  ocio::ExponentWithLinearTransformRcPtr transform;
};

struct RealMatrixTransform {
  ocio::MatrixTransformRcPtr transform;
};

struct RealLogTransform {
  ocio::LogTransformRcPtr transform;
};

struct RealRangeTransform {
  ocio::RangeTransformRcPtr transform;
};

struct RealGroupTransform {
  ocio::GroupTransformRcPtr transform;
};

struct RealBuiltinTransform {
  ocio::BuiltinTransformRcPtr transform;
};

struct RealFixedFunctionTransform {
  ocio::FixedFunctionTransformRcPtr transform;
};

struct RealLut1DTransform {
  ocio::Lut1DTransformRcPtr transform;
};

struct RealLut3DTransform {
  ocio::Lut3DTransformRcPtr transform;
};

struct RealBaker {
  ocio::BakerRcPtr baker;
};

struct RealContext {
  ocio::ContextRcPtr context;
};

struct RealColorSpace {
  ocio::ColorSpaceRcPtr colorSpace;
};

struct RealLook {
  ocio::LookRcPtr look;
};
struct RealViewTransform {
  ocio::ViewTransformRcPtr transform;
};
struct RealNamedTransform {
  ocio::NamedTransformRcPtr transform;
};
struct RealDynamicProperty {
  ocio::DynamicPropertyRcPtr prop;
};
struct RealBuiltinConfigRegistry {
  ocio::BuiltinConfigRegistry* registry;
};
struct RealExposureContrastTransform {
  ocio::ExposureContrastTransformRcPtr transform;
};
struct RealColorSpaceTransform {
  ocio::ColorSpaceTransformRcPtr transform;
};
struct RealLookTransform {
  ocio::LookTransformRcPtr transform;
};
struct RealGradingRGBCurveTransform {
  ocio::GradingRGBCurveTransformRcPtr transform;
};
struct RealGradingHueCurveTransform {
  ocio::GradingHueCurveTransformRcPtr transform;
};
struct RealGradingPrimaryTransform {
  ocio::GradingPrimaryTransformRcPtr transform;
};
struct RealGradingToneTransform {
  ocio::GradingToneTransformRcPtr transform;
};
struct RealAllocationTransform {
  ocio::AllocationTransformRcPtr transform;
};
struct RealLogAffineTransform {
  ocio::LogAffineTransformRcPtr transform;
};
struct RealLogCameraTransform {
  ocio::LogCameraTransformRcPtr transform;
};
struct RealDisplayViewTransform {
  ocio::DisplayViewTransformRcPtr transform;
};
struct RealFileRules {
  ocio::FileRulesRcPtr rules;
};

struct RealColorSpaceSet {
  ocio::ColorSpaceSetRcPtr set;
};

struct RealFormatMetadata {
  ocio::FormatMetadata* metadata;
  bool owned; // always false - FormatMetadata is never owned by the handle
};

// --- TransformHandleBase out-of-line implementations ---

ocio::TransformRcPtr FileTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealFileTransform>(inner)->transform;
}
ocio::TransformRcPtr CDLTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealCDLTransform>(inner)->transform;
}
ocio::TransformRcPtr ExponentTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealExponentTransform>(inner)->transform;
}
ocio::TransformRcPtr ExponentWithLinearTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealExponentWithLinearTransform>(inner)->transform;
}
ocio::TransformRcPtr MatrixTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealMatrixTransform>(inner)->transform;
}
ocio::TransformRcPtr LogTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLogTransform>(inner)->transform;
}
ocio::TransformRcPtr RangeTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealRangeTransform>(inner)->transform;
}
ocio::TransformRcPtr GroupTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealGroupTransform>(inner)->transform;
}
ocio::TransformRcPtr BuiltinTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealBuiltinTransform>(inner)->transform;
}
ocio::TransformRcPtr FixedFunctionTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealFixedFunctionTransform>(inner)->transform;
}
ocio::TransformRcPtr Lut1DTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLut1DTransform>(inner)->transform;
}
ocio::TransformRcPtr Lut3DTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLut3DTransform>(inner)->transform;
}

ocio::TransformRcPtr ExposureContrastTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealExposureContrastTransform>(inner)->transform;
}

ocio::TransformRcPtr ColorSpaceTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealColorSpaceTransform>(inner)->transform;
}

ocio::TransformRcPtr LookTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLookTransform>(inner)->transform;
}

ocio::TransformRcPtr GradingRGBCurveTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealGradingRGBCurveTransform>(inner)->transform;
}
ocio::TransformRcPtr GradingHueCurveTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealGradingHueCurveTransform>(inner)->transform;
}
ocio::TransformRcPtr GradingPrimaryTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealGradingPrimaryTransform>(inner)->transform;
}
ocio::TransformRcPtr GradingToneTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealGradingToneTransform>(inner)->transform;
}
ocio::TransformRcPtr AllocationTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealAllocationTransform>(inner)->transform;
}
ocio::TransformRcPtr LogAffineTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLogAffineTransform>(inner)->transform;
}
ocio::TransformRcPtr LogCameraTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealLogCameraTransform>(inner)->transform;
}
ocio::TransformRcPtr DisplayViewTransformHandle::get_ocio_transform() {
  return std::static_pointer_cast<RealDisplayViewTransform>(inner)->transform;
}

// --- Config real implementations ---

static std::unique_ptr<ConfigHandle> make_real_config_raw() {
  try {
    auto handle = std::make_unique<ConfigHandle>();
    auto config = std::make_shared<RealConfig>();
    config->config = ocio::Config::CreateRaw();
    handle->inner = config;
    return handle;
  } catch (...) {
    return nullptr;
  }
}

static std::unique_ptr<ConfigHandle> make_real_config_from_file(const char* path) {
  try {
    auto handle = std::make_unique<ConfigHandle>();
    auto config = std::make_shared<RealConfig>();
    config->config = ocio::Config::CreateFromFile(path);
    if (!config->config) {
      return nullptr;
    }
    handle->inner = config;
    return handle;
  } catch (...) {
    return nullptr;
  }
}

static ocio::ConstConfigRcPtr get_real_config(void* config_handle) {
  auto* handle = static_cast<ConfigHandle*>(config_handle);
  return std::static_pointer_cast<RealConfig>(handle->inner)->config;
}

// --- Processor real implementations ---

static std::unique_ptr<ProcessorHandle> make_real_processor(
    void* config_handle, const char* src, const char* dst) {
  try {
    auto config = get_real_config(config_handle);
    auto handle = std::make_unique<ProcessorHandle>();
    auto processor = std::make_shared<RealProcessor>();
    processor->processor = config->getProcessor(src, dst);
    handle->inner = processor;
    return handle;
  } catch (...) {
    return nullptr;
  }
}

static std::unique_ptr<ProcessorHandle> make_real_processor_display(
    void* config_handle, const char* src, const char* display, const char* view, int direction) {
  try {
    auto config = get_real_config(config_handle);
    auto handle = std::make_unique<ProcessorHandle>();
    auto processor = std::make_shared<RealProcessor>();
    processor->processor = config->getProcessor(
        src, display, view,
        static_cast<ocio::TransformDirection>(direction));
    handle->inner = processor;
    return handle;
  } catch (...) {
    return nullptr;
  }
}

static std::unique_ptr<ProcessorHandle> make_real_processor_from_transform(
    void* config_handle, void* transform_handle, int direction) {
  try {
    auto config = get_real_config(config_handle);
    auto* th = static_cast<GroupTransformHandle*>(transform_handle);
    // For transform-based processors, the transform could be any type.
    // We pass it as a raw pointer that OCIO can type-erase.
    // The caller is responsible for passing a valid transform.
    (void)th;
    return nullptr; // implemented when Transform base class bridging is added
  } catch (...) {
    return nullptr;
  }
}

static ocio::ConstProcessorRcPtr get_real_processor(void* processor_handle) {
  auto* handle = static_cast<ProcessorHandle*>(processor_handle);
  return std::static_pointer_cast<RealProcessor>(handle->inner)->processor;
}

// --- CPUProcessor real implementations ---

static std::unique_ptr<CPUProcessorHandle> make_real_cpu_processor(void* processor_handle) {
  try {
    auto proc = get_real_processor(processor_handle);
    auto handle = std::make_unique<CPUProcessorHandle>();
    auto cpu = std::make_shared<RealCPUProcessor>();
    cpu->cpu = proc->getDefaultCPUProcessor();
    handle->inner = cpu;
    return handle;
  } catch (...) {
    return nullptr;
  }
}

static std::unique_ptr<CPUProcessorHandle> make_real_optimized_cpu_processor(
    void* processor_handle, unsigned long flags) {
  try {
    auto proc = get_real_processor(processor_handle);
    auto handle = std::make_unique<CPUProcessorHandle>();
    auto cpu = std::make_shared<RealCPUProcessor>();
    cpu->cpu = proc->getOptimizedCPUProcessor(
        static_cast<ocio::OptimizationFlags>(flags));
    handle->inner = cpu;
    return handle;
  } catch (...) {
    return nullptr;
  }
}

static ocio::ConstCPUProcessorRcPtr get_real_cpu_processor(void* cpu_handle) {
  auto* handle = static_cast<CPUProcessorHandle*>(cpu_handle);
  return std::static_pointer_cast<RealCPUProcessor>(handle->inner)->cpu;
}

// --- GPUProcessor real implementations ---

static std::unique_ptr<GPUProcessorHandle> make_real_gpu_processor(void* processor_handle) {
  try {
    auto proc = get_real_processor(processor_handle);
    auto handle = std::make_unique<GPUProcessorHandle>();
    auto gpu = std::make_shared<RealGPUProcessor>();
    gpu->gpu = proc->getDefaultGPUProcessor();
    handle->inner = gpu;
    return handle;
  } catch (...) {
    return nullptr;
  }
}

static std::unique_ptr<GPUProcessorHandle> make_real_optimized_gpu_processor(
    void* processor_handle, unsigned long flags) {
  try {
    auto proc = get_real_processor(processor_handle);
    auto handle = std::make_unique<GPUProcessorHandle>();
    auto gpu = std::make_shared<RealGPUProcessor>();
    gpu->gpu = proc->getOptimizedGPUProcessor(
        static_cast<ocio::OptimizationFlags>(flags));
    handle->inner = gpu;
    return handle;
  } catch (...) {
    return nullptr;
  }
}

static ocio::ConstGPUProcessorRcPtr get_real_gpu_processor(void* gpu_handle) {
  auto* handle = static_cast<GPUProcessorHandle*>(gpu_handle);
  return std::static_pointer_cast<RealGPUProcessor>(handle->inner)->gpu;
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
  try {
    return ocio::GetVersion();
  } catch (...) { return nullptr; }
#endif
}

int ocio_get_version_hex(void) {
#ifdef OCIO_RS_STUB
  return 0;
#else
  try {
    return ocio::GetVersionHex();
  } catch (...) { return 0; }
#endif
}

int ocio_get_logging_level(void) {
#ifdef OCIO_RS_STUB
  return 0;
#else
  try {
    return static_cast<int>(ocio::GetLoggingLevel());
  } catch (...) { return 0; }
#endif
}

void ocio_set_logging_level(int level) {
#ifdef OCIO_RS_STUB
  (void)level;
#else
  try {
    ocio::SetLoggingLevel(static_cast<ocio::LoggingLevel>(level));
  } catch (...) {}
#endif
}

void ocio_set_logging_level_to_override(int level) {
#ifdef OCIO_RS_STUB
  (void)level;
#else
  try {
    ocio::SetLoggingLevelToOverride(static_cast<ocio::LoggingLevel>(level));
  } catch (...) {}
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
      ocio_rs_bridge::RealBuiltinConfigRegistry{&registry}
    );
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_builtin_config_registry_get_num_builtin_configs(void* registry) {
#ifdef OCIO_RS_STUB
  (void)registry; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BuiltinConfigRegistryHandle*>(registry);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinConfigRegistry>(h->inner);
    return static_cast<int>(real->registry->getNumBuiltinConfigs());
  } catch (...) { return 0; }
#endif
}

const char* ocio_builtin_config_registry_get_config_name(void* registry, int index) {
#ifdef OCIO_RS_STUB
  (void)registry; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BuiltinConfigRegistryHandle*>(registry);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinConfigRegistry>(h->inner);
    return real->registry->getBuiltinConfigNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_builtin_config_registry_get_config_ui_name(void* registry, int index) {
#ifdef OCIO_RS_STUB
  (void)registry; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BuiltinConfigRegistryHandle*>(registry);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinConfigRegistry>(h->inner);
    return real->registry->getBuiltinConfigUiNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

bool ocio_builtin_config_registry_is_config_recommended(void* registry, int index) {
#ifdef OCIO_RS_STUB
  (void)registry; (void)index; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BuiltinConfigRegistryHandle*>(registry);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinConfigRegistry>(h->inner);
    return real->registry->isBuiltinConfigRecommended(index);
  } catch (...) { return false; }
#endif
}

void* ocio_builtin_config_registry_get_config_by_index(void* registry, int index) {
#ifdef OCIO_RS_STUB
  (void)registry; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BuiltinConfigRegistryHandle*>(registry);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinConfigRegistry>(h->inner);
    auto cfg = real->registry->getBuiltinConfigByIndex(index);
    if (!cfg) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(ocio_rs_bridge::RealConfig{cfg});
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_builtin_config_registry_get_config_by_name(void* registry, const char* name) {
#ifdef OCIO_RS_STUB
  (void)registry; (void)name; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BuiltinConfigRegistryHandle*>(registry);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinConfigRegistry>(h->inner);
    auto cfg = real->registry->getBuiltinConfig(name);
    if (!cfg) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(ocio_rs_bridge::RealConfig{cfg});
    return handle.release();
  } catch (...) { return nullptr; }
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
    handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(ocio_rs_bridge::RealConfig{cfg});
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_set_current_config(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    if (config) {
      auto* h = static_cast<ocio_rs_bridge::ConfigHandle*>(config);
      auto real = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(h->inner);
      ocio::SetCurrentConfig(real->config);
    }
  } catch (...) {}
#endif
}

void ocio_clear_all_caches(void) {
#ifndef OCIO_RS_STUB
  try {
    ocio::ClearAllCaches();
  } catch (...) {}
#endif
}

// --- Config: construction ---

void* ocio_config_create_raw(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_config().release();
#else
  return ocio_rs_bridge::make_real_config_raw().release();
#endif
}

void* ocio_config_create_from_file(const char* path) {
#ifdef OCIO_RS_STUB
  (void)path;
  return nullptr;
#else
  auto handle = ocio_rs_bridge::make_real_config_from_file(path);
  if (!handle) return nullptr;
  auto* real_config = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(handle->inner).get();
  if (!real_config || !real_config->config) return nullptr;
  return handle.release();
#endif
}

// --- Config: name & metadata ---

const char* ocio_config_get_name(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getName();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_name(void* config, const char* name) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setName(name);
  } catch (...) {}
#endif
}

const char* ocio_config_get_description(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getDescription();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_description(void* config, const char* desc) {
#ifdef OCIO_RS_STUB
  (void)config; (void)desc;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setDescription(desc);
  } catch (...) {}
#endif
}

const char* ocio_config_get_cache_id(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getCacheID();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_config_get_cache_id_n(void* config, const char* contextKey) {
#ifdef OCIO_RS_STUB
  (void)config; (void)contextKey;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getCacheID(contextKey);
  } catch (...) { return nullptr; }
#endif
}

// --- Config: version ---

unsigned int ocio_config_get_major_version(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getMajorVersion();
  } catch (...) { return 0; }
#endif
}

unsigned int ocio_config_get_minor_version(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getMinorVersion();
  } catch (...) { return 0; }
#endif
}

char ocio_config_get_family_separator(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return '/';
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getFamilySeparator();
  } catch (...) { return '/'; }
#endif
}

// --- Config: color spaces ---

int ocio_config_get_num_color_spaces(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getNumColorSpaces();
  } catch (...) { return 0; }
#endif
}

const char* ocio_config_get_color_space_name_by_index(void* config, int index) {
#ifdef OCIO_RS_STUB
  (void)config; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getColorSpaceNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_config_get_canonical_name(void* config, const char* name) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getCanonicalName(name);
  } catch (...) { return nullptr; }
#endif
}

bool ocio_config_is_color_space_linear(void* config, const char* colorSpace, int referenceSpaceType) {
#ifdef OCIO_RS_STUB
  (void)config; (void)colorSpace; (void)referenceSpaceType;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->isColorSpaceLinear(
        colorSpace,
        static_cast<ocio::ReferenceSpaceType>(referenceSpaceType));
  } catch (...) { return false; }
#endif
}

const char* ocio_config_get_color_space_from_filepath(void* config, const char* filePath) {
#ifdef OCIO_RS_STUB
  (void)config; (void)filePath;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getColorSpaceFromFilepath(filePath);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_color_space_by_ref_type(void* config, const char* name, int refType) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name; (void)refType;
  return nullptr;
#else
  try {
    auto cs = ocio_rs_bridge::get_real_config(config)->getColorSpace(
        name, static_cast<ocio::SearchReferenceSpaceType>(refType));
    if (!cs) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ColorSpaceHandle>();
    auto wrapper = std::make_shared<ocio_rs_bridge::RealColorSpace>();
    wrapper->colorSpace = cs->createEditableCopy();
    handle->inner = wrapper;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_color_space_from_filepath_by_ref_type(void* config, const char* filePath, int refType) {
#ifdef OCIO_RS_STUB
  (void)config; (void)filePath; (void)refType;
  return nullptr;
#else
  try {
    auto cs = ocio_rs_bridge::get_real_config(config)->getColorSpaceFromFilepath(
        filePath, static_cast<ocio::SearchReferenceSpaceType>(refType));
    if (!cs) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ColorSpaceHandle>();
    auto wrapper = std::make_shared<ocio_rs_bridge::RealColorSpace>();
    wrapper->colorSpace = cs->createEditableCopy();
    handle->inner = wrapper;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

// --- Config: displays ---

const char* ocio_config_get_default_display(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getDefaultDisplay();
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_num_displays(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getNumDisplays();
  } catch (...) { return 0; }
#endif
}

const char* ocio_config_get_display(void* config, int index) {
#ifdef OCIO_RS_STUB
  (void)config; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getDisplay(index);
  } catch (...) { return nullptr; }
#endif
}

// --- Config: views ---

const char* ocio_config_get_default_view(void* config, const char* display) {
#ifdef OCIO_RS_STUB
  (void)config; (void)display;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getDefaultView(display);
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_num_views(void* config, const char* display) {
#ifdef OCIO_RS_STUB
  (void)config; (void)display;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getNumViews(display);
  } catch (...) { return 0; }
#endif
}

const char* ocio_config_get_view(void* config, const char* display, int index) {
#ifdef OCIO_RS_STUB
  (void)config; (void)display; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getView(display, index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_default_display(void* config, const char* display) {
#ifdef OCIO_RS_STUB
  (void)config; (void)display;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setDefaultDisplay(display);
  } catch (...) {}
#endif
}

void ocio_config_set_default_view(void* config, const char* view) {
#ifdef OCIO_RS_STUB
  (void)config; (void)view;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setDefaultView(view);
  } catch (...) {}
#endif
}

// --- Config: looks ---

int ocio_config_get_num_looks(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getNumLooks();
  } catch (...) { return 0; }
#endif
}

const char* ocio_config_get_look_name_by_index(void* config, int index) {
#ifdef OCIO_RS_STUB
  (void)config; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getLookNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_config_get_color_spaces(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return nullptr;
#else
  try {
    auto* real = ocio_rs_bridge::get_real_config(config);
    static thread_local std::string cached;
    cached.clear();
    int n = real->getNumColorSpaces();
    for (int i = 0; i < n; ++i) {
      if (i > 0) cached += ", ";
      cached += real->getColorSpaceNameByIndex(i);
    }
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_config_get_looks(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return nullptr;
#else
  try {
    auto* real = ocio_rs_bridge::get_real_config(config);
    static thread_local std::string cached;
    cached.clear();
    int n = real->getNumLooks();
    for (int i = 0; i < n; ++i) {
      if (i > 0) cached += ", ";
      cached += real->getLookNameByIndex(i);
    }
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

// --- Config: luma coefs ---

void ocio_config_get_default_luma_coefs(void* config, double* rgb) {
  rgb[0] = 0.0; rgb[1] = 0.0; rgb[2] = 0.0;
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->getDefaultLumaCoefs(rgb);
  } catch (...) {}
#endif
}

// --- Config: roles ---

int ocio_config_get_num_roles(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getNumRoles();
  } catch (...) { return 0; }
#endif
}

bool ocio_config_has_role(void* config, const char* role) {
#ifdef OCIO_RS_STUB
  (void)config; (void)role;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->hasRole(role);
  } catch (...) { return false; }
#endif
}

const char* ocio_config_get_role_name(void* config, int index) {
#ifdef OCIO_RS_STUB
  (void)config; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getRoleName(index);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_config_get_role_color_space_by_index(void* config, int index) {
#ifdef OCIO_RS_STUB
  (void)config; (void)index;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getRoleColorSpace(index);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_config_get_role_color_space_by_name(void* config, const char* roleName) {
#ifdef OCIO_RS_STUB
  (void)config; (void)roleName;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getRoleColorSpace(roleName);
  } catch (...) { return nullptr; }
#endif
}

// --- Config: active displays / views ---

const char* ocio_config_get_active_displays(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getActiveDisplays();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_config_get_active_views(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getActiveViews();
  } catch (...) { return nullptr; }
#endif
}

// --- Config: search paths & strict parsing ---

const char* ocio_config_get_search_path(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getSearchPath();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_search_path(void* config, const char* path) {
#ifdef OCIO_RS_STUB
  (void)config; (void)path;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setSearchPath(path);
  } catch (...) {}
#endif
}

bool ocio_config_is_strict_parsing_enabled(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->isStrictParsingEnabled();
  } catch (...) { return false; }
#endif
}

void ocio_config_set_strict_parsing_enabled(void* config, bool enabled) {
#ifdef OCIO_RS_STUB
  (void)config; (void)enabled;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setStrictParsingEnabled(enabled);
  } catch (...) {}
#endif
}

int ocio_config_get_num_search_paths(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getNumSearchPaths();
  } catch (...) { return 0; }
#endif
}

const char* ocio_config_get_search_path_by_index(void* config, int index) {
#ifdef OCIO_RS_STUB
  (void)config; (void)index; return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getSearchPath(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_clear_search_paths(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->clearSearchPaths();
  } catch (...) {}
#endif
}

void ocio_config_add_search_path(void* config, const char* path) {
#ifdef OCIO_RS_STUB
  (void)config; (void)path;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->addSearchPath(path);
  } catch (...) {}
#endif
}

// --- Config: roles (mutable) ---

void ocio_config_set_role(void* config, const char* role, const char* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)config; (void)role; (void)colorSpace;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setRole(role, colorSpace);
  } catch (...) {}
#endif
}

// --- Config: family separator ---

void ocio_config_set_family_separator(void* config, char separator) {
#ifdef OCIO_RS_STUB
  (void)config; (void)separator;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setFamilySeparator(separator);
  } catch (...) {}
#endif
}

// --- Config: validate & serialize ---

const char* ocio_config_validate(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return nullptr;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->validate();
    return nullptr;
  } catch (const std::exception& e) {
    // Return error message on validation failure
    return strdup(e.what());
  } catch (...) { return "Unknown validation error"; }
#endif
}

const char* ocio_config_serialize(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->serialize();
  } catch (...) { return nullptr; }
#endif
}

// --- Config: editable copy ---

void* ocio_config_create_editable_copy(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return nullptr;
#else
  try {
    auto cfg = ocio_rs_bridge::get_real_config(config)->createEditableCopy();
    if (!cfg) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(ocio_rs_bridge::RealConfig{cfg});
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

// --- Config: context ---

void* ocio_config_get_current_context(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return nullptr;
#else
  try {
    auto ctx = ocio_rs_bridge::get_real_config(config)->getCurrentContext();
    if (!ctx) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ContextHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealContext>(ocio_rs_bridge::RealContext{ctx});
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_current_context(void* config, void* context) {
#ifdef OCIO_RS_STUB
  (void)config; (void)context;
#else
  try {
    if (context) {
      auto* ctx = static_cast<ocio_rs_bridge::ContextHandle*>(context);
      auto real = std::static_pointer_cast<ocio_rs_bridge::RealContext>(ctx->inner);
      ocio_rs_bridge::get_real_config(config)->setCurrentContext(real->context);
    }
  } catch (...) {}
#endif
}

// --- Config: active display/view setters ---

void ocio_config_set_active_displays(void* config, const char* displays) {
#ifdef OCIO_RS_STUB
  (void)config; (void)displays;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setActiveDisplays(displays);
  } catch (...) {}
#endif
}

void ocio_config_set_active_views(void* config, const char* views) {
#ifdef OCIO_RS_STUB
  (void)config; (void)views;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setActiveViews(views);
  } catch (...) {}
#endif
}

// --- Config: display/view transform name queries ---

const char* ocio_config_get_display_view_transform_name(void* config, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)config; (void)display; (void)view; return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getDisplayViewTransformName(display, view);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_config_get_display_view_color_space_name(void* config, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)config; (void)display; (void)view; return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getDisplayViewColorSpaceName(display, view);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_config_get_display_view_looks(void* config, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)config; (void)display; (void)view; return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getDisplayViewLooks(display, view);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_default_scene_to_display_view_transform(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
  return nullptr;
#else
  try {
    auto t = ocio_rs_bridge::get_real_config(config)->getDefaultSceneToDisplayViewTransform();
    if (!t) return nullptr;
    switch (t->getTransformType()) {
      case ocio::TRANSFORM_TYPE_BUILTIN: {
        auto builtin = ocio::DynamicPtrCast<ocio::BuiltinTransform>(t);
        auto handle = new ocio_rs_bridge::BuiltinTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealBuiltinTransform>(ocio_rs_bridge::RealBuiltinTransform{builtin});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_CDL: {
        auto cdl = ocio::DynamicPtrCast<ocio::CDLTransform>(t);
        auto handle = new ocio_rs_bridge::CDLTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealCDLTransform>(ocio_rs_bridge::RealCDLTransform{cdl});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_COLOR_SPACE: {
        auto cs = ocio::DynamicPtrCast<ocio::ColorSpaceTransform>(t);
        auto handle = new ocio_rs_bridge::ColorSpaceTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceTransform>(ocio_rs_bridge::RealColorSpaceTransform{cs});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_EXPONENT: {
        auto exp = ocio::DynamicPtrCast<ocio::ExponentTransform>(t);
        auto handle = new ocio_rs_bridge::ExponentTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealExponentTransform>(ocio_rs_bridge::RealExponentTransform{exp});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_EXPOSURE_CONTRAST: {
        auto ec = ocio::DynamicPtrCast<ocio::ExposureContrastTransform>(t);
        auto handle = new ocio_rs_bridge::ExposureContrastTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealExposureContrastTransform>(ocio_rs_bridge::RealExposureContrastTransform{ec});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_FILE: {
        auto file = ocio::DynamicPtrCast<ocio::FileTransform>(t);
        auto handle = new ocio_rs_bridge::FileTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealFileTransform>(ocio_rs_bridge::RealFileTransform{file});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_FIXED_FUNCTION: {
        auto ff = ocio::DynamicPtrCast<ocio::FixedFunctionTransform>(t);
        auto handle = new ocio_rs_bridge::FixedFunctionHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealFixedFunctionTransform>(ocio_rs_bridge::RealFixedFunctionTransform{ff});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_GRADING_HUE_CURVE: {
        auto ghc = ocio::DynamicPtrCast<ocio::GradingHueCurveTransform>(t);
        auto handle = new ocio_rs_bridge::GradingHueCurveTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealGradingHueCurveTransform>(ocio_rs_bridge::RealGradingHueCurveTransform{ghc});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_GRADING_PRIMARY: {
        auto gp = ocio::DynamicPtrCast<ocio::GradingPrimaryTransform>(t);
        auto handle = new ocio_rs_bridge::GradingPrimaryTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealGradingPrimaryTransform>(ocio_rs_bridge::RealGradingPrimaryTransform{gp});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_GRADING_RGB_CURVE: {
        auto gc = ocio::DynamicPtrCast<ocio::GradingRGBCurveTransform>(t);
        auto handle = new ocio_rs_bridge::GradingRGBCurveTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealGradingRGBCurveTransform>(ocio_rs_bridge::RealGradingRGBCurveTransform{gc});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_GRADING_TONE: {
        auto gt = ocio::DynamicPtrCast<ocio::GradingToneTransform>(t);
        auto handle = new ocio_rs_bridge::GradingToneTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealGradingToneTransform>(ocio_rs_bridge::RealGradingToneTransform{gt});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_GROUP: {
        auto grp = ocio::DynamicPtrCast<ocio::GroupTransform>(t);
        auto handle = new ocio_rs_bridge::GroupTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealGroupTransform>(ocio_rs_bridge::RealGroupTransform{grp});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_LOG: {
        auto log = ocio::DynamicPtrCast<ocio::LogTransform>(t);
        auto handle = new ocio_rs_bridge::LogTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealLogTransform>(ocio_rs_bridge::RealLogTransform{log});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_LOG_AFFINE: {
        auto la = ocio::DynamicPtrCast<ocio::LogAffineTransform>(t);
        auto handle = new ocio_rs_bridge::LogAffineTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealLogAffineTransform>(ocio_rs_bridge::RealLogAffineTransform{la});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_LOG_CAMERA: {
        auto lc = ocio::DynamicPtrCast<ocio::LogCameraTransform>(t);
        auto handle = new ocio_rs_bridge::LogCameraTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealLogCameraTransform>(ocio_rs_bridge::RealLogCameraTransform{lc});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_LOOK: {
        auto lk = ocio::DynamicPtrCast<ocio::LookTransform>(t);
        auto handle = new ocio_rs_bridge::LookTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealLookTransform>(ocio_rs_bridge::RealLookTransform{lk});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_LUT1D: {
        auto lut1d = ocio::DynamicPtrCast<ocio::Lut1DTransform>(t);
        auto handle = new ocio_rs_bridge::Lut1DHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealLut1DTransform>(ocio_rs_bridge::RealLut1DTransform{lut1d});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_LUT3D: {
        auto lut3d = ocio::DynamicPtrCast<ocio::Lut3DTransform>(t);
        auto handle = new ocio_rs_bridge::Lut3DHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealLut3DTransform>(ocio_rs_bridge::RealLut3DTransform{lut3d});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_MATRIX: {
        auto mat = ocio::DynamicPtrCast<ocio::MatrixTransform>(t);
        auto handle = new ocio_rs_bridge::MatrixTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealMatrixTransform>(ocio_rs_bridge::RealMatrixTransform{mat});
        return handle;
      }
      case ocio::TRANSFORM_TYPE_RANGE: {
        auto range = ocio::DynamicPtrCast<ocio::RangeTransform>(t);
        auto handle = new ocio_rs_bridge::RangeTransformHandle{};
        handle->inner = std::make_shared<ocio_rs_bridge::RealRangeTransform>(ocio_rs_bridge::RealRangeTransform{range});
        return handle;
      }
      default: return nullptr;
    }
  } catch (...) { return nullptr; }
#endif
}

// --- Config: clear collections ---

void ocio_config_clear_color_spaces(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->clearColorSpaces();
  } catch (...) {}
#endif
}

void ocio_config_clear_looks(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->clearLooks();
  } catch (...) {}
#endif
}

void ocio_config_clear_named_transforms(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    auto* real = ocio_rs_bridge::get_real_config(config);
    std::vector<std::string> names;
    int n = real->getNumNamedTransforms();
    for (int i = 0; i < n; ++i) {
      names.push_back(real->getNamedTransformNameByIndex(i));
    }
    for (const auto& name : names) {
      real->removeNamedTransform(name.c_str());
    }
  } catch (...) {}
#endif
}

void ocio_config_clear_view_transforms(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    auto* real = ocio_rs_bridge::get_real_config(config);
    std::vector<std::string> names;
    int n = real->getNumViewTransforms();
    for (int i = 0; i < n; ++i) {
      names.push_back(real->getViewTransformNameByIndex(i));
    }
    for (const auto& name : names) {
      real->removeViewTransform(name.c_str());
    }
  } catch (...) {}
#endif
}

// --- Config: default luma setter ---

void ocio_config_set_default_luma_coefs(void* config, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)config; (void)rgb;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setDefaultLumaCoefs(ocio::LumaCoef3{rgb[0], rgb[1], rgb[2]});
  } catch (...) {}
#endif
}

// --- Config: display/view management ---

void ocio_config_add_display(void* config, const char* display, const char* view, const char* transformName, const char* rule) {
#ifdef OCIO_RS_STUB
  (void)config; (void)display; (void)view; (void)transformName; (void)rule;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->addDisplay(display, view, transformName, rule);
  } catch (...) {}
#endif
}

void ocio_config_add_shared_view(void* config, const char* display, const char* view, const char* transformName, const char* rule) {
#ifdef OCIO_RS_STUB
  (void)config; (void)display; (void)view; (void)transformName; (void)rule;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->addSharedView(display, view, transformName, rule);
  } catch (...) {}
#endif
}

void ocio_config_remove_display(void* config, const char* display) {
#ifdef OCIO_RS_STUB
  (void)config; (void)display;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->removeDisplay(display);
  } catch (...) {}
#endif
}

void ocio_config_remove_view(void* config, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)config; (void)display; (void)view;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->removeView(display, view);
  } catch (...) {}
#endif
}

// --- Config: named transforms ---

int ocio_config_get_num_named_transforms(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getNumNamedTransforms();
  } catch (...) { return 0; }
#endif
}

const char* ocio_config_get_named_transform_name_by_index(void* config, int index) {
#ifdef OCIO_RS_STUB
  (void)config; (void)index; return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getNamedTransformNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_named_transform(void* config, const char* name) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name; return nullptr;
#else
  try {
    auto nt = ocio_rs_bridge::get_real_config(config)->getNamedTransform(name);
    if (!nt) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::NamedTransformHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealNamedTransform>(ocio_rs_bridge::RealNamedTransform{nt});
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_add_named_transform(void* config, void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)config; (void)namedTransform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(h->inner);
    ocio_rs_bridge::get_real_config(config)->addNamedTransform(real->transform);
  } catch (...) {}
#endif
}

void ocio_config_remove_named_transform(void* config, const char* name) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->removeNamedTransform(name);
  } catch (...) {}
#endif
}

// --- Config: view transforms ---

int ocio_config_get_num_view_transforms(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return 0;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getNumViewTransforms();
  } catch (...) { return 0; }
#endif
}

const char* ocio_config_get_view_transform_name_by_index(void* config, int index) {
#ifdef OCIO_RS_STUB
  (void)config; (void)index; return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getViewTransformNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_view_transform(void* config, const char* name) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name; return nullptr;
#else
  try {
    auto vt = ocio_rs_bridge::get_real_config(config)->getViewTransform(name);
    if (!vt) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ViewTransformHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealViewTransform>(ocio_rs_bridge::RealViewTransform{vt});
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_add_view_transform(void* config, void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)config; (void)viewTransform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner);
    ocio_rs_bridge::get_real_config(config)->addViewTransform(real->transform);
  } catch (...) {}
#endif
}

void ocio_config_remove_view_transform(void* config, const char* name) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->removeViewTransform(name);
  } catch (...) {}
#endif
}

// --- Config: processors ---

void* ocio_config_get_processor(void* config, const char* src, const char* dst) {
#ifdef OCIO_RS_STUB
  (void)config; (void)src; (void)dst;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  auto handle = ocio_rs_bridge::make_real_processor(config, src, dst);
  if (!handle) return nullptr;
  auto* real_proc = std::static_pointer_cast<ocio_rs_bridge::RealProcessor>(handle->inner).get();
  if (!real_proc || !real_proc->processor) return nullptr;
  return handle.release();
#endif
}

void* ocio_config_get_processor_display(
    void* config, const char* src, const char* display, const char* view, int direction) {
#ifdef OCIO_RS_STUB
  (void)config; (void)src; (void)display; (void)view; (void)direction;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  auto handle = ocio_rs_bridge::make_real_processor_display(config, src, display, view, direction);
  if (!handle) return nullptr;
  auto* real_proc = std::static_pointer_cast<ocio_rs_bridge::RealProcessor>(handle->inner).get();
  if (!real_proc || !real_proc->processor) return nullptr;
  return handle.release();
#endif
}

void* ocio_config_get_processor_transform(void* config, void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)config; (void)transform; (void)direction;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  try {
    auto cfg = ocio_rs_bridge::get_real_config(config);
    auto* base = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    auto ocio_transform = base->get_ocio_transform();
    if (!ocio_transform) return nullptr;
    auto processor = cfg->getProcessor(
        ocio_transform, static_cast<ocio::TransformDirection>(direction));
    auto hdl = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    hdl->inner = std::make_shared<ocio_rs_bridge::RealProcessor>();
    std::static_pointer_cast<ocio_rs_bridge::RealProcessor>(hdl->inner)->processor = processor;
    return hdl.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_with_context(
    void* config, const char* src, const char* dst, void* context) {
#ifdef OCIO_RS_STUB
  (void)config; (void)src; (void)dst; (void)context;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  try {
    auto cfg = ocio_rs_bridge::get_real_config(config);
    auto* ctx_handle = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    auto ocio_ctx = std::static_pointer_cast<ocio_rs_bridge::RealContext>(ctx_handle->inner)->context;
    auto processor = cfg->getProcessor(src, dst, ocio_ctx);
    auto hdl = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    hdl->inner = std::make_shared<ocio_rs_bridge::RealProcessor>();
    std::static_pointer_cast<ocio_rs_bridge::RealProcessor>(hdl->inner)->processor = processor;
    return hdl.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_config_get_processor_from_configs(
    void* srcConfig, const char* srcName, void* dstConfig, const char* dstName) {
#ifdef OCIO_RS_STUB
  (void)srcConfig; (void)srcName; (void)dstConfig; (void)dstName;
  return ocio_rs_bridge::make_stub_processor().release();
#else
  try {
    auto src = ocio_rs_bridge::get_real_config(srcConfig);
    auto dst = ocio_rs_bridge::get_real_config(dstConfig);
    auto processor = ocio::Config::GetProcessorFromConfigs(src, srcName, dst, dstName);
    auto hdl = std::make_unique<ocio_rs_bridge::ProcessorHandle>();
    hdl->inner = std::make_shared<ocio_rs_bridge::RealProcessor>();
    std::static_pointer_cast<ocio_rs_bridge::RealProcessor>(hdl->inner)->processor = processor;
    return hdl.release();
  } catch (...) { return nullptr; }
#endif
}

// --- Processor ---

void* ocio_processor_get_default_cpu_processor(void* processor) {
#ifdef OCIO_RS_STUB
  (void)processor;
  return ocio_rs_bridge::make_stub_cpu_processor().release();
#else
  auto handle = ocio_rs_bridge::make_real_cpu_processor(processor);
  if (!handle) return nullptr;
  auto* real_cpu = std::static_pointer_cast<ocio_rs_bridge::RealCPUProcessor>(handle->inner).get();
  if (!real_cpu || !real_cpu->cpu) return nullptr;
  return handle.release();
#endif
}

void* ocio_processor_get_optimized_cpu_processor(void* processor, unsigned long flags) {
#ifdef OCIO_RS_STUB
  (void)processor; (void)flags;
  return ocio_rs_bridge::make_stub_cpu_processor().release();
#else
  auto handle = ocio_rs_bridge::make_real_optimized_cpu_processor(processor, flags);
  if (!handle) return nullptr;
  auto* real_cpu = std::static_pointer_cast<ocio_rs_bridge::RealCPUProcessor>(handle->inner).get();
  if (!real_cpu || !real_cpu->cpu) return nullptr;
  return handle.release();
#endif
}

bool ocio_processor_is_no_op(void* processor) {
#ifdef OCIO_RS_STUB
  (void)processor;
  return true;
#else
  try {
    return ocio_rs_bridge::get_real_processor(processor)->isNoOp();
  } catch (...) { return true; }
#endif
}

bool ocio_processor_has_channel_crosstalk(void* processor) {
#ifdef OCIO_RS_STUB
  (void)processor;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_processor(processor)->hasChannelCrosstalk();
  } catch (...) { return false; }
#endif
}

const char* ocio_processor_get_cache_id(void* processor) {
#ifdef OCIO_RS_STUB
  (void)processor;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_processor(processor)->getCacheID();
  } catch (...) { return nullptr; }
#endif
}

void ocio_processor_apply_rgba(void* processor, float* rgba, size_t len) {
  if (len != 4) return;
#ifdef OCIO_RS_STUB
  (void)processor; (void)rgba;
#else
  try {
    auto proc = ocio_rs_bridge::get_real_processor(processor);
    auto cpu = proc->getDefaultCPUProcessor();
    cpu->applyRGBA(rgba);
  } catch (...) {}
#endif
}

// --- Processor: bit-depth CPU/GPU processor access ---

void* ocio_processor_get_default_cpu_processor_bitdepth(void* processor, int inBitDepth, int outBitDepth) {
#ifdef OCIO_RS_STUB
  (void)processor; (void)inBitDepth; (void)outBitDepth;
  return ocio_rs_bridge::make_stub_cpu_processor().release();
#else
  try {
    auto proc = ocio_rs_bridge::get_real_processor(processor);
    auto handle = std::make_unique<CPUProcessorHandle>();
    auto cpu = std::make_shared<RealCPUProcessor>();
    cpu->cpu = proc->getDefaultCPUProcessor(
        static_cast<ocio::BitDepth>(inBitDepth),
        static_cast<ocio::BitDepth>(outBitDepth));
    handle->inner = cpu;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_optimized_cpu_processor_bitdepth(void* processor, int inBitDepth, int outBitDepth, unsigned long flags) {
#ifdef OCIO_RS_STUB
  (void)processor; (void)inBitDepth; (void)outBitDepth; (void)flags;
  return ocio_rs_bridge::make_stub_cpu_processor().release();
#else
  try {
    auto proc = ocio_rs_bridge::get_real_processor(processor);
    auto handle = std::make_unique<CPUProcessorHandle>();
    auto cpu = std::make_shared<RealCPUProcessor>();
    cpu->cpu = proc->getOptimizedCPUProcessor(
        static_cast<ocio::BitDepth>(inBitDepth),
        static_cast<ocio::BitDepth>(outBitDepth),
        static_cast<ocio::OptimizationFlags>(flags));
    handle->inner = cpu;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_default_gpu_processor_bitdepth(void* processor, int inBitDepth, int outBitDepth) {
#ifdef OCIO_RS_STUB
  (void)processor; (void)inBitDepth; (void)outBitDepth;
  return ocio_rs_bridge::make_stub_gpu_processor().release();
#else
  try {
    auto proc = ocio_rs_bridge::get_real_processor(processor);
    auto handle = std::make_unique<GPUProcessorHandle>();
    auto gpu = std::make_shared<RealGPUProcessor>();
    gpu->gpu = proc->getDefaultGPUProcessor(
        static_cast<ocio::BitDepth>(inBitDepth),
        static_cast<ocio::BitDepth>(outBitDepth));
    handle->inner = gpu;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_processor_get_optimized_gpu_processor_bitdepth(void* processor, int inBitDepth, int outBitDepth, unsigned long flags) {
#ifdef OCIO_RS_STUB
  (void)processor; (void)inBitDepth; (void)outBitDepth; (void)flags;
  return ocio_rs_bridge::make_stub_gpu_processor().release();
#else
  try {
    auto proc = ocio_rs_bridge::get_real_processor(processor);
    auto handle = std::make_unique<GPUProcessorHandle>();
    auto gpu = std::make_shared<RealGPUProcessor>();
    gpu->gpu = proc->getOptimizedGPUProcessor(
        static_cast<ocio::BitDepth>(inBitDepth),
        static_cast<ocio::BitDepth>(outBitDepth),
        static_cast<ocio::OptimizationFlags>(flags));
    handle->inner = gpu;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

// --- CPUProcessor ---

void ocio_cpu_processor_apply_rgba(void* cpu_processor, float* rgba) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor; (void)rgba;
#else
  try {
    ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->applyRGBA(rgba);
  } catch (...) {}
#endif
}

void ocio_cpu_processor_apply_rgb(void* cpu_processor, float* rgb) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor; (void)rgb;
#else
  try {
    ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->applyRGB(rgb);
  } catch (...) {}
#endif
}

bool ocio_cpu_processor_is_no_op(void* cpu_processor) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor;
  return true;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->isNoOp();
  } catch (...) { return true; }
#endif
}

bool ocio_cpu_processor_has_channel_crosstalk(void* cpu_processor) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->hasChannelCrosstalk();
  } catch (...) { return false; }
#endif
}

const char* ocio_cpu_processor_get_cache_id(void* cpu_processor) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->getCacheID();
  } catch (...) { return nullptr; }
#endif
}

int ocio_cpu_processor_get_input_bit_depth(void* cpu_processor) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor;
  return 8; // BIT_DEPTH_F32
#else
  try {
    return static_cast<int>(ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->getInputBitDepth());
  } catch (...) { return 0; }
#endif
}

int ocio_cpu_processor_get_output_bit_depth(void* cpu_processor) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor;
  return 8; // BIT_DEPTH_F32
#else
  try {
    return static_cast<int>(ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->getOutputBitDepth());
  } catch (...) { return 0; }
#endif
}

bool ocio_cpu_processor_is_identity(void* cpu_processor) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->isIdentity();
  } catch (...) { return false; }
#endif
}

void ocio_cpu_processor_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::CPUProcessorHandle*>(handle);
}

// --- CPUProcessor: packed pixel processing ---

void ocio_cpu_processor_apply_rgba_packed(void* cpu_processor, void* rgba, int bitDepth, long numPixels, long stride) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor; (void)rgba; (void)bitDepth; (void)numPixels; (void)stride;
#else
  try {
    ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->applyRGBA(rgba, static_cast<ocio::BitDepth>(bitDepth), numPixels, stride);
  } catch (...) {}
#endif
}

void ocio_cpu_processor_apply_rgb_packed(void* cpu_processor, void* rgb, int bitDepth, long numPixels, long stride) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor; (void)rgb; (void)bitDepth; (void)numPixels; (void)stride;
#else
  try {
    ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->applyRGB(rgb, static_cast<ocio::BitDepth>(bitDepth), numPixels, stride);
  } catch (...) {}
#endif
}

// --- GPUProcessor ---

void* ocio_processor_get_default_gpu_processor(void* processor) {
#ifdef OCIO_RS_STUB
  (void)processor;
  return ocio_rs_bridge::make_stub_gpu_processor().release();
#else
  auto handle = ocio_rs_bridge::make_real_gpu_processor(processor);
  if (!handle) return nullptr;
  auto* real_gpu = std::static_pointer_cast<ocio_rs_bridge::RealGPUProcessor>(handle->inner).get();
  if (!real_gpu || !real_gpu->gpu) return nullptr;
  return handle.release();
#endif
}

void* ocio_processor_get_optimized_gpu_processor(void* processor, unsigned long flags) {
#ifdef OCIO_RS_STUB
  (void)processor; (void)flags;
  return ocio_rs_bridge::make_stub_gpu_processor().release();
#else
  auto handle = ocio_rs_bridge::make_real_optimized_gpu_processor(processor, flags);
  if (!handle) return nullptr;
  auto* real_gpu = std::static_pointer_cast<ocio_rs_bridge::RealGPUProcessor>(handle->inner).get();
  if (!real_gpu || !real_gpu->gpu) return nullptr;
  return handle.release();
#endif
}

bool ocio_gpu_processor_is_no_op(void* gpu_processor) {
#ifdef OCIO_RS_STUB
  (void)gpu_processor;
  return true;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_processor(gpu_processor)->isNoOp();
  } catch (...) { return true; }
#endif
}

bool ocio_gpu_processor_has_channel_crosstalk(void* gpu_processor) {
#ifdef OCIO_RS_STUB
  (void)gpu_processor;
  return false;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_processor(gpu_processor)->hasChannelCrosstalk();
  } catch (...) { return false; }
#endif
}

const char* ocio_gpu_processor_get_cache_id(void* gpu_processor) {
#ifdef OCIO_RS_STUB
  (void)gpu_processor;
  return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_gpu_processor(gpu_processor)->getCacheID();
  } catch (...) { return nullptr; }
#endif
}

void ocio_gpu_processor_extract_shader_info(void* gpu_processor, void* shader_desc) {
#ifdef OCIO_RS_STUB
  (void)gpu_processor; (void)shader_desc;
#else
  try {
    auto gpu = ocio_rs_bridge::get_real_gpu_processor(gpu_processor);
    auto* sd = static_cast<ocio::GpuShaderDesc*>(shader_desc);
    gpu->extractGpuShaderInfo(*sd);
  } catch (...) {}
#endif
}

const char* ocio_gpu_processor_extract_gpu_shader_info_cache_id(void* gpu_processor, void* shader_desc) {
#ifdef OCIO_RS_STUB
  (void)gpu_processor; (void)shader_desc; return nullptr;
#else
  try {
    auto gpu = ocio_rs_bridge::get_real_gpu_processor(gpu_processor);
    auto* sd = static_cast<ocio::GpuShaderDesc*>(shader_desc);
    static thread_local std::string cached;
    cached = gpu->extractGpuShaderInfoCacheID(*sd);
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_gpu_processor_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GPUProcessorHandle*>(handle);
}

// --- GpuShaderDesc ---

void* ocio_gpu_shader_desc_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_gpu_shader_desc().release();
#else
  try {
    auto desc = ocio::GpuShaderDesc::CreateShaderDesc();
    auto handle = std::make_unique<ocio_rs_bridge::GpuShaderDescHandle>();
    handle->inner = std::make_shared<ocio::GpuShaderDescRcPtr>(desc);
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_gpu_shader_desc_get_shader_text(void* shader_desc) {
#ifdef OCIO_RS_STUB
  (void)shader_desc;
  return nullptr;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    return (*desc)->getShaderText();
  } catch (...) { return nullptr; }
#endif
}

unsigned int ocio_gpu_shader_desc_get_num_textures(void* shader_desc) {
#ifdef OCIO_RS_STUB
  (void)shader_desc;
  return 0;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    return (*desc)->getNumTextures();
  } catch (...) { return 0; }
#endif
}

void ocio_gpu_shader_desc_get_texture(
    void* shader_desc, unsigned int index,
    const char** textureName, const char** samplerName,
    unsigned int* width, unsigned int* height,
    int* channel, int* dimensions, int* interpolation) {
#ifdef OCIO_RS_STUB
  (void)shader_desc; (void)index;
  *textureName = nullptr; *samplerName = nullptr;
  *width = 0; *height = 0;
  *channel = 0; *dimensions = 0; *interpolation = 0;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    ocio::GpuShaderDesc::TextureType channel_;
    ocio::GpuShaderCreator::TextureDimensions dims_;
    ocio::Interpolation interp_;
    (*desc)->getTexture(index, *textureName, *samplerName, *width, *height, channel_, dims_, interp_);
    *channel = static_cast<int>(channel_);
    *dimensions = static_cast<int>(dims_);
    *interpolation = static_cast<int>(interp_);
  } catch (...) {
    *textureName = nullptr; *samplerName = nullptr;
    *width = 0; *height = 0;
    *channel = 0; *dimensions = 0; *interpolation = 0;
  }
#endif
}

const float* ocio_gpu_shader_desc_get_texture_values(void* shader_desc, unsigned int index) {
#ifdef OCIO_RS_STUB
  (void)shader_desc; (void)index;
  return nullptr;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    const float* values = nullptr;
    (*desc)->getTextureValues(index, values);
    return values;
  } catch (...) { return nullptr; }
#endif
}

int ocio_gpu_shader_desc_get_language(void* shader_desc) {
#ifdef OCIO_RS_STUB
  (void)shader_desc; return 0;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    return static_cast<int>((*desc)->getLanguage());
  } catch (...) { return 0; }
#endif
}

void ocio_gpu_shader_desc_set_language(void* shader_desc, int language) {
#ifdef OCIO_RS_STUB
  (void)shader_desc; (void)language;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    (*desc)->setLanguage(static_cast<ocio::GpuLanguage>(language));
  } catch (...) {}
#endif
}

const char* ocio_gpu_shader_desc_get_function_name(void* shader_desc) {
#ifdef OCIO_RS_STUB
  (void)shader_desc; return nullptr;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    static thread_local std::string cached;
    cached = (*desc)->getFunctionName();
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_gpu_shader_desc_set_function_name(void* shader_desc, const char* name) {
#ifdef OCIO_RS_STUB
  (void)shader_desc; (void)name;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    (*desc)->setFunctionName(name);
  } catch (...) {}
#endif
}

const char* ocio_gpu_shader_desc_get_pixel_name(void* shader_desc) {
#ifdef OCIO_RS_STUB
  (void)shader_desc; return nullptr;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    static thread_local std::string cached;
    cached = (*desc)->getPixelName();
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_gpu_shader_desc_set_pixel_name(void* shader_desc, const char* name) {
#ifdef OCIO_RS_STUB
  (void)shader_desc; (void)name;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    (*desc)->setPixelName(name);
  } catch (...) {}
#endif
}

const char* ocio_gpu_shader_desc_get_resource_prefix(void* shader_desc) {
#ifdef OCIO_RS_STUB
  (void)shader_desc; return nullptr;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    static thread_local std::string cached;
    cached = (*desc)->getResourcePrefix();
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_gpu_shader_desc_set_resource_prefix(void* shader_desc, const char* prefix) {
#ifdef OCIO_RS_STUB
  (void)shader_desc; (void)prefix;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    (*desc)->setResourcePrefix(prefix);
  } catch (...) {}
#endif
}

void ocio_gpu_shader_desc_finalize(void* shader_desc) {
#ifdef OCIO_RS_STUB
  (void)shader_desc;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(shader_desc);
    auto desc = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    (*desc)->finalize();
  } catch (...) {}
#endif
}

void ocio_gpu_shader_desc_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(handle);
}

// --- GpuShaderDesc: texture dimensions & cache ---

unsigned int ocio_gpu_shader_desc_get_texture_max_width(void* desc, int index) {
#ifdef OCIO_RS_STUB
  (void)desc; (void)index; return 0;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(desc);
    auto d = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    return (*d)->getTextureMaxWidth(index);
  } catch (...) { return 0; }
#endif
}

unsigned int ocio_gpu_shader_desc_get_texture_max_height(void* desc, int index) {
#ifdef OCIO_RS_STUB
  (void)desc; (void)index; return 0;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(desc);
    auto d = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    return (*d)->getTextureMaxHeight(index);
  } catch (...) { return 0; }
#endif
}

const char* ocio_gpu_shader_desc_get_cache_id(void* desc) {
#ifdef OCIO_RS_STUB
  (void)desc; return nullptr;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(desc);
    auto d = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    static thread_local std::string cached;
    cached = (*d)->getCacheID();
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_gpu_shader_desc_get_texture_uid(void* desc, int index) {
#ifdef OCIO_RS_STUB
  (void)desc; (void)index; return nullptr;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::GpuShaderDescHandle*>(desc);
    auto d = std::static_pointer_cast<ocio::GpuShaderDescRcPtr>(handle->inner);
    static thread_local std::string cached;
    cached = (*d)->getTextureUID(index);
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

// --- Transform base ---

int ocio_transform_get_transform_type(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  return static_cast<ocio_rs_bridge::TransformHandleBase*>(transform)->get_transform_type_tag();
#endif
}

void* ocio_transform_create_editable_copy(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return nullptr;
#else
  if (!transform) return nullptr;
  auto* base = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
  auto type = base->get_transform_type_tag();
  TransformHandleBase* out = nullptr;
  switch (type) {
    case 2: { // CDLTransform
      auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealCDLTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::CDLTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 8: { // FileTransform
      auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealFileTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::FileTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 5: { // ExponentTransform
      auto* h = static_cast<ocio_rs_bridge::ExponentTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealExponentTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealExponentTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::ExponentTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 6: { // ExponentWithLinearTransform
      auto* h = static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealExponentWithLinearTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealExponentWithLinearTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::ExponentWithLinearTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 17: { // LogTransform
      auto* h = static_cast<ocio_rs_bridge::LogTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealLogTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealLogTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::LogTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 21: { // MatrixTransform
      auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealMatrixTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::MatrixTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 22: { // RangeTransform
      auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealRangeTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::RangeTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 7: { // ExposureContrastTransform
      auto* h = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealExposureContrastTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::ExposureContrastTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 4: { // ColorSpaceTransform
      auto* h = static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealColorSpaceTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::ColorSpaceTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 18: { // LookTransform
      auto* h = static_cast<ocio_rs_bridge::LookTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealLookTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::LookTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 1: { // BuiltinTransform
      auto* h = static_cast<ocio_rs_bridge::BuiltinTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealBuiltinTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::BuiltinTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 9: { // FixedFunctionTransform
      auto* h = static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealFixedFunctionTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealFixedFunctionTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::FixedFunctionTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 19: { // Lut1DTransform
      auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealLut1DTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::Lut1DTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 20: { // Lut3DTransform
      auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealLut3DTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::Lut3DTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 14: { // GroupTransform
      auto* h = static_cast<ocio_rs_bridge::GroupTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealGroupTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::GroupTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 12: { // GradingRGBCurveTransform
      auto* h = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealGradingRGBCurveTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::GradingRGBCurveTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 10: { // GradingHueCurveTransform
      auto* h = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealGradingHueCurveTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::GradingHueCurveTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 11: { // GradingPrimaryTransform
      auto* h = static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealGradingPrimaryTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::GradingPrimaryTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 13: { // GradingToneTransform
      auto* h = static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealGradingToneTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealGradingToneTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::GradingToneTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 0: { // AllocationTransform
      auto* h = static_cast<ocio_rs_bridge::AllocationTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealAllocationTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealAllocationTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::AllocationTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 15: { // LogAffineTransform
      auto* h = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealLogAffineTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::LogAffineTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
    case 16: { // LogCameraTransform
      auto* h = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(base);
      auto r = std::make_shared<ocio_rs_bridge::RealLogCameraTransform>();
      r->transform = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(h->inner)->transform->createEditableCopy();
      auto hdl = std::make_unique<ocio_rs_bridge::LogCameraTransformHandle>();
      hdl->inner = r;
      out = hdl.release();
      break;
    }
  }
  return out;
#endif
}


// --- Config destruction ---

void ocio_config_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ConfigHandle*>(handle);
}

// --- Processor additional methods ---

int ocio_processor_get_num_transforms(void* processor) {
#ifdef OCIO_RS_STUB
  (void)processor; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ProcessorHandle*>(processor);
    return std::static_pointer_cast<ocio_rs_bridge::RealProcessor>(h->inner)->processor->getNumTransforms();
  } catch (...) { return 0; }
#endif
}

void* ocio_processor_create_group_transform(void* processor) {
#ifdef OCIO_RS_STUB
  (void)processor; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ProcessorHandle*>(processor);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealProcessor>(h->inner);
    auto gt = real->processor->createGroupTransform();
    if (!gt) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::GroupTransformHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealGroupTransform>(ocio_rs_bridge::RealGroupTransform{gt});
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

// --- Processor destruction ---

void ocio_processor_write(void* processor, const char* formatName, const char* fileName) {
#ifdef OCIO_RS_STUB
  (void)processor; (void)formatName; (void)fileName;
#else
  try {
    auto proc = ocio_rs_bridge::get_real_processor(processor);
    proc->write(formatName, fileName);
  } catch (...) {}
#endif
}

void ocio_processor_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ProcessorHandle*>(handle);
}

// --- FileTransform ---

void* ocio_file_transform_create(void) {
#ifdef OCIO_RS_STUB
  auto handle = std::make_unique<ocio_rs_bridge::FileTransformHandle>();
  handle->inner = std::make_shared<ocio_rs_bridge::StubFileTransform>();
  return handle.release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::FileTransformHandle>();
    auto ft = std::make_shared<ocio_rs_bridge::RealFileTransform>();
    ft->transform = ocio::FileTransform::Create();
    handle->inner = ft;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_file_transform_get_src(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform->getSrc();
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_transform_set_src(void* transform, const char* src) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)src;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform->setSrc(src);
  } catch (...) {}
#endif
}

const char* ocio_file_transform_get_ccc_id(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform->getCCCId();
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_transform_set_ccc_id(void* transform, const char* id) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)id;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform->setCCCId(id);
  } catch (...) {}
#endif
}

int ocio_file_transform_get_interpolation(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform->getInterpolation());
  } catch (...) { return 0; }
#endif
}

void ocio_file_transform_set_interpolation(void* transform, int interp) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)interp;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform->setInterpolation(
        static_cast<ocio::Interpolation>(interp));
  } catch (...) {}
#endif
}

int ocio_file_transform_get_cdl_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform->getCDLStyle());
  } catch (...) { return 0; }
#endif
}

void ocio_file_transform_set_cdl_style(void* transform, int style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform->setCDLStyle(
        static_cast<ocio::CDLStyle>(style));
  } catch (...) {}
#endif
}

int ocio_file_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_file_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealFileTransform>(h->inner)->transform->setDirection(
        static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_file_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::FileTransformHandle*>(handle);
}

// --- CDLTransform ---

void* ocio_cdl_transform_create(void) {
#ifdef OCIO_RS_STUB
  auto handle = std::make_unique<ocio_rs_bridge::CDLTransformHandle>();
  handle->inner = std::make_shared<ocio_rs_bridge::StubCDLTransform>();
  return handle.release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::CDLTransformHandle>();
    auto t = std::make_shared<ocio_rs_bridge::RealCDLTransform>();
    t->transform = ocio::CDLTransform::Create();
    handle->inner = t;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_cdl_transform_create_from_file(const char* src, const char* cccId) {
#ifdef OCIO_RS_STUB
  (void)src; (void)cccId; return nullptr;
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::CDLTransformHandle>();
    auto t = std::make_shared<ocio_rs_bridge::RealCDLTransform>();
    t->transform = ocio::CDLTransform::CreateFromFile(src, cccId);
    handle->inner = t;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_cdl_transform_get_slope(void* transform, double* rgb) {
  rgb[0] = rgb[1] = rgb[2] = 1.0;
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->getSlope(rgb);
  } catch (...) {}
#endif
}

void ocio_cdl_transform_set_slope(void* transform, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)rgb;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->setSlope(rgb);
  } catch (...) {}
#endif
}

void ocio_cdl_transform_get_offset(void* transform, double* rgb) {
  rgb[0] = rgb[1] = rgb[2] = 0.0;
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->getOffset(rgb);
  } catch (...) {}
#endif
}

void ocio_cdl_transform_set_offset(void* transform, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)rgb;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->setOffset(rgb);
  } catch (...) {}
#endif
}

void ocio_cdl_transform_get_power(void* transform, double* rgb) {
  rgb[0] = rgb[1] = rgb[2] = 1.0;
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->getPower(rgb);
  } catch (...) {}
#endif
}

void ocio_cdl_transform_set_power(void* transform, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)rgb;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->setPower(rgb);
  } catch (...) {}
#endif
}

double ocio_cdl_transform_get_sat(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 1.0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->getSat();
  } catch (...) { return 1.0; }
#endif
}

void ocio_cdl_transform_set_sat(void* transform, double sat) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)sat;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->setSat(static_cast<float>(sat));
  } catch (...) {}
#endif
}

void ocio_cdl_transform_get_sat_luma_coefs(void* transform, double* rgb) {
  rgb[0] = rgb[1] = rgb[2] = 0.0;
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->getSatLumaCoefs(rgb);
  } catch (...) {}
#endif
}

void ocio_cdl_transform_set_sat_luma_coefs(void* transform, const double* rgb) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)rgb;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->setSatLumaCoefs(rgb);
  } catch (...) {}
#endif
}

int ocio_cdl_transform_get_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->getStyle());
  } catch (...) { return 0; }
#endif
}

void ocio_cdl_transform_set_style(void* transform, int style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->setStyle(
        static_cast<ocio::CDLStyle>(style));
  } catch (...) {}
#endif
}

const char* ocio_cdl_transform_get_id(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->getID();
  } catch (...) { return nullptr; }
#endif
}

void ocio_cdl_transform_set_id(void* transform, const char* id) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)id;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->setID(id);
  } catch (...) {}
#endif
}

int ocio_cdl_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_cdl_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->setDirection(
        static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_cdl_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::CDLTransformHandle*>(handle);
}

// --- ExponentTransform ---

void* ocio_exponent_transform_create(void) {
#ifdef OCIO_RS_STUB
  auto handle = std::make_unique<ocio_rs_bridge::ExponentTransformHandle>();
  handle->inner = std::make_shared<ocio_rs_bridge::StubExponentTransform>();
  return handle.release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::ExponentTransformHandle>();
    auto t = std::make_shared<ocio_rs_bridge::RealExponentTransform>();
    t->transform = ocio::ExponentTransform::Create();
    handle->inner = t;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_exponent_transform_get_value(void* transform, double* vec4) {
  vec4[0] = vec4[1] = vec4[2] = vec4[3] = 1.0;
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealExponentTransform>(h->inner)->transform->getValue(vec4);
  } catch (...) {}
#endif
}

void ocio_exponent_transform_set_value(void* transform, const double* vec4) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)vec4;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealExponentTransform>(h->inner)->transform->setValue(vec4);
  } catch (...) {}
#endif
}

int ocio_exponent_transform_get_negative_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealExponentTransform>(h->inner)->transform->getNegativeStyle());
  } catch (...) { return 0; }
#endif
}

void ocio_exponent_transform_set_negative_style(void* transform, int style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealExponentTransform>(h->inner)->transform->setNegativeStyle(
        static_cast<ocio::NegativeStyle>(style));
  } catch (...) {}
#endif
}

int ocio_exponent_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealExponentTransform>(h->inner)->transform->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_exponent_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealExponentTransform>(h->inner)->transform->setDirection(
        static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_exponent_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ExponentTransformHandle*>(handle);
}

// --- ExponentWithLinearTransform ---

void* ocio_exponent_with_linear_transform_create(void) {
#ifdef OCIO_RS_STUB
  auto handle = std::make_unique<ocio_rs_bridge::ExponentWithLinearTransformHandle>();
  handle->inner = std::make_shared<ocio_rs_bridge::StubExponentWithLinearTransform>();
  return handle.release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::ExponentWithLinearTransformHandle>();
    auto t = std::make_shared<ocio_rs_bridge::RealExponentWithLinearTransform>();
    t->transform = ocio::ExponentWithLinearTransform::Create();
    handle->inner = t;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_exponent_with_linear_transform_get_gamma(void* transform, double* vec4) {
  vec4[0] = vec4[1] = vec4[2] = vec4[3] = 1.0;
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealExponentWithLinearTransform>(h->inner)->transform->getGamma(vec4);
  } catch (...) {}
#endif
}

void ocio_exponent_with_linear_transform_set_gamma(void* transform, const double* vec4) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)vec4;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealExponentWithLinearTransform>(h->inner)->transform->setGamma(vec4);
  } catch (...) {}
#endif
}

void ocio_exponent_with_linear_transform_get_offset(void* transform, double* vec4) {
  vec4[0] = vec4[1] = vec4[2] = vec4[3] = 0.0;
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealExponentWithLinearTransform>(h->inner)->transform->getOffset(vec4);
  } catch (...) {}
#endif
}

void ocio_exponent_with_linear_transform_set_offset(void* transform, const double* vec4) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)vec4;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealExponentWithLinearTransform>(h->inner)->transform->setOffset(vec4);
  } catch (...) {}
#endif
}

int ocio_exponent_with_linear_transform_get_negative_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealExponentWithLinearTransform>(h->inner)->transform->getNegativeStyle());
  } catch (...) { return 0; }
#endif
}

void ocio_exponent_with_linear_transform_set_negative_style(void* transform, int style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealExponentWithLinearTransform>(h->inner)->transform->setNegativeStyle(
        static_cast<ocio::NegativeStyle>(style));
  } catch (...) {}
#endif
}

int ocio_exponent_with_linear_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealExponentWithLinearTransform>(h->inner)->transform->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_exponent_with_linear_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealExponentWithLinearTransform>(h->inner)->transform->setDirection(
        static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_exponent_with_linear_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ExponentWithLinearTransformHandle*>(handle);
}

// --- MatrixTransform ---

void* ocio_matrix_transform_create(void) {
#ifdef OCIO_RS_STUB
  auto handle = std::make_unique<ocio_rs_bridge::MatrixTransformHandle>();
  handle->inner = std::make_shared<ocio_rs_bridge::StubMatrixTransform>();
  return handle.release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::MatrixTransformHandle>();
    auto t = std::make_shared<ocio_rs_bridge::RealMatrixTransform>();
    t->transform = ocio::MatrixTransform::Create();
    handle->inner = t;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_matrix_transform_get_matrix(void* transform, double* m44) {
  for (int i = 0; i < 16; i++) m44[i] = (i % 5 == 0) ? 1.0 : 0.0;
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform->getMatrix(m44);
  } catch (...) {}
#endif
}

void ocio_matrix_transform_set_matrix(void* transform, const double* m44) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)m44;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform->setMatrix(m44);
  } catch (...) {}
#endif
}

void ocio_matrix_transform_get_offset(void* transform, double* offset4) {
  offset4[0] = offset4[1] = offset4[2] = offset4[3] = 0.0;
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform->getOffset(offset4);
  } catch (...) {}
#endif
}

void ocio_matrix_transform_set_offset(void* transform, const double* offset4) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)offset4;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform->setOffset(offset4);
  } catch (...) {}
#endif
}

int ocio_matrix_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_matrix_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform->setDirection(
        static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_matrix_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::MatrixTransformHandle*>(handle);
}

// --- LogTransform ---

void* ocio_log_transform_create(void) {
#ifdef OCIO_RS_STUB
  auto handle = std::make_unique<ocio_rs_bridge::LogTransformHandle>();
  handle->inner = std::make_shared<ocio_rs_bridge::StubLogTransform>();
  return handle.release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::LogTransformHandle>();
    auto t = std::make_shared<ocio_rs_bridge::RealLogTransform>();
    t->transform = ocio::LogTransform::Create();
    handle->inner = t;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

double ocio_log_transform_get_base(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 2.0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LogTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealLogTransform>(h->inner)->transform->getBase();
  } catch (...) { return 2.0; }
#endif
}

void ocio_log_transform_set_base(void* transform, double base) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)base;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LogTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealLogTransform>(h->inner)->transform->setBase(base);
  } catch (...) {}
#endif
}

int ocio_log_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LogTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealLogTransform>(h->inner)->transform->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_log_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LogTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealLogTransform>(h->inner)->transform->setDirection(
        static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_log_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LogTransformHandle*>(handle);
}

// --- RangeTransform ---

void* ocio_range_transform_create(void) {
#ifdef OCIO_RS_STUB
  auto handle = std::make_unique<ocio_rs_bridge::RangeTransformHandle>();
  handle->inner = std::make_shared<ocio_rs_bridge::StubRangeTransform>();
  return handle.release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::RangeTransformHandle>();
    auto t = std::make_shared<ocio_rs_bridge::RealRangeTransform>();
    t->transform = ocio::RangeTransform::Create();
    handle->inner = t;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_range_transform_get_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->getStyle());
  } catch (...) { return 0; }
#endif
}

void ocio_range_transform_set_style(void* transform, int style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->setStyle(
        static_cast<ocio::RangeStyle>(style));
  } catch (...) {}
#endif
}

double ocio_range_transform_get_min_in_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0.0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->getMinInValue();
  } catch (...) { return 0.0; }
#endif
}

void ocio_range_transform_set_min_in_value(void* transform, double value) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)value;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->setMinInValue(value);
  } catch (...) {}
#endif
}

double ocio_range_transform_get_max_in_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 1.0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->getMaxInValue();
  } catch (...) { return 1.0; }
#endif
}

void ocio_range_transform_set_max_in_value(void* transform, double value) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)value;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->setMaxInValue(value);
  } catch (...) {}
#endif
}

double ocio_range_transform_get_min_out_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0.0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->getMinOutValue();
  } catch (...) { return 0.0; }
#endif
}

void ocio_range_transform_set_min_out_value(void* transform, double value) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)value;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->setMinOutValue(value);
  } catch (...) {}
#endif
}

double ocio_range_transform_get_max_out_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 1.0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->getMaxOutValue();
  } catch (...) { return 1.0; }
#endif
}

void ocio_range_transform_set_max_out_value(void* transform, double value) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)value;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->setMaxOutValue(value);
  } catch (...) {}
#endif
}

int ocio_range_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_range_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->setDirection(
        static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_range_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::RangeTransformHandle*>(handle);
}

// --- GroupTransform ---

void* ocio_group_transform_create(void) {
#ifdef OCIO_RS_STUB
  auto handle = std::make_unique<ocio_rs_bridge::GroupTransformHandle>();
  handle->inner = std::make_shared<ocio_rs_bridge::StubGroupTransform>();
  return handle.release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::GroupTransformHandle>();
    auto t = std::make_shared<ocio_rs_bridge::RealGroupTransform>();
    t->transform = ocio::GroupTransform::Create();
    handle->inner = t;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_group_transform_get_num_transforms(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::GroupTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(h->inner)->transform->getNumTransforms();
  } catch (...) { return 0; }
#endif
}

void* ocio_group_transform_get_transform(void* transform, int index) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::GroupTransformHandle*>(transform);
    auto ut = std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(h->inner)->transform;
    auto child = ut->getTransform(index);
    if (!child) return nullptr;
    int type = static_cast<int>(child->getTransformType());
    TransformHandleBase* out = nullptr;
    switch (type) {
      case 1: { // Builtin
        auto t = ocio::DynamicPtrCast<const ocio::BuiltinTransform>(child);
        auto hdl = std::make_unique<BuiltinTransformHandle>();
        auto r = std::make_shared<RealBuiltinTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 2: { // CDL
        auto t = ocio::DynamicPtrCast<const ocio::CDLTransform>(child);
        auto hdl = std::make_unique<CDLTransformHandle>();
        auto r = std::make_shared<RealCDLTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 5: { // Exponent
        auto t = ocio::DynamicPtrCast<const ocio::ExponentTransform>(child);
        auto hdl = std::make_unique<ExponentTransformHandle>();
        auto r = std::make_shared<RealExponentTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 8: { // File
        auto t = ocio::DynamicPtrCast<const ocio::FileTransform>(child);
        auto hdl = std::make_unique<FileTransformHandle>();
        auto r = std::make_shared<RealFileTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 9: { // FixedFunction
        auto t = ocio::DynamicPtrCast<const ocio::FixedFunctionTransform>(child);
        auto hdl = std::make_unique<FixedFunctionTransformHandle>();
        auto r = std::make_shared<RealFixedFunctionTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 19: { // Lut1D
        auto t = ocio::DynamicPtrCast<const ocio::Lut1DTransform>(child);
        auto hdl = std::make_unique<Lut1DTransformHandle>();
        auto r = std::make_shared<RealLut1DTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 20: { // Lut3D
        auto t = ocio::DynamicPtrCast<const ocio::Lut3DTransform>(child);
        auto hdl = std::make_unique<Lut3DTransformHandle>();
        auto r = std::make_shared<RealLut3DTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 14: { // Group
        auto t = ocio::DynamicPtrCast<const ocio::GroupTransform>(child);
        auto hdl = std::make_unique<GroupTransformHandle>();
        auto r = std::make_shared<RealGroupTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 17: { // Log
        auto t = ocio::DynamicPtrCast<const ocio::LogTransform>(child);
        auto hdl = std::make_unique<LogTransformHandle>();
        auto r = std::make_shared<RealLogTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 21: { // Matrix
        auto t = ocio::DynamicPtrCast<const ocio::MatrixTransform>(child);
        auto hdl = std::make_unique<MatrixTransformHandle>();
        auto r = std::make_shared<RealMatrixTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 22: { // Range
        auto t = ocio::DynamicPtrCast<const ocio::RangeTransform>(child);
        auto hdl = std::make_unique<RangeTransformHandle>();
        auto r = std::make_shared<RealRangeTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 7: { // ExposureContrast
        auto t = ocio::DynamicPtrCast<const ocio::ExposureContrastTransform>(child);
        auto hdl = std::make_unique<ExposureContrastTransformHandle>();
        auto r = std::make_shared<RealExposureContrastTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 4: { // ColorSpace
        auto t = ocio::DynamicPtrCast<const ocio::ColorSpaceTransform>(child);
        auto hdl = std::make_unique<ColorSpaceTransformHandle>();
        auto r = std::make_shared<RealColorSpaceTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 18: { // Look
        auto t = ocio::DynamicPtrCast<const ocio::LookTransform>(child);
        auto hdl = std::make_unique<LookTransformHandle>();
        auto r = std::make_shared<RealLookTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 0: { // Allocation
        auto t = ocio::DynamicPtrCast<const ocio::AllocationTransform>(child);
        auto hdl = std::make_unique<AllocationTransformHandle>();
        auto r = std::make_shared<RealAllocationTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 15: { // LogAffine
        auto t = ocio::DynamicPtrCast<const ocio::LogAffineTransform>(child);
        auto hdl = std::make_unique<LogAffineTransformHandle>();
        auto r = std::make_shared<RealLogAffineTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 16: { // LogCamera
        auto t = ocio::DynamicPtrCast<const ocio::LogCameraTransform>(child);
        auto hdl = std::make_unique<LogCameraTransformHandle>();
        auto r = std::make_shared<RealLogCameraTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 11: { // GradingPrimary
        auto t = ocio::DynamicPtrCast<const ocio::GradingPrimaryTransform>(child);
        auto hdl = std::make_unique<GradingPrimaryTransformHandle>();
        auto r = std::make_shared<RealGradingPrimaryTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 12: { // GradingRGBCurve
        auto t = ocio::DynamicPtrCast<const ocio::GradingRGBCurveTransform>(child);
        auto hdl = std::make_unique<GradingRGBCurveTransformHandle>();
        auto r = std::make_shared<RealGradingRGBCurveTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 13: { // GradingTone
        auto t = ocio::DynamicPtrCast<const ocio::GradingToneTransform>(child);
        auto hdl = std::make_unique<GradingToneTransformHandle>();
        auto r = std::make_shared<RealGradingToneTransform>();
        r->transform = t->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      default: return nullptr;
    }
    return out;
  } catch (...) { return nullptr; }
#endif
}

void ocio_group_transform_append_transform(void* transform, void* child) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)child;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::GroupTransformHandle*>(transform);
    auto group = std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(h->inner);
    auto* child_base = static_cast<ocio_rs_bridge::TransformHandleBase*>(child);
    auto child_t = child_base->get_ocio_transform();
    if (child_t) {
      group->transform->appendTransform(child_t);
    }
  } catch (...) {}
#endif
}

void ocio_group_transform_prepend_transform(void* transform, void* child) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)child;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::GroupTransformHandle*>(transform);
    auto group = std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(h->inner);
    auto* child_base = static_cast<ocio_rs_bridge::TransformHandleBase*>(child);
    auto child_t = child_base->get_ocio_transform();
    if (child_t) {
      group->transform->prependTransform(child_t);
    }
  } catch (...) {}
#endif
}

int ocio_group_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::GroupTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(h->inner)->transform->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_group_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::GroupTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(h->inner)->transform->setDirection(
        static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_group_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GroupTransformHandle*>(handle);
}

// --- BuiltinTransform ---

void* ocio_builtin_transform_create(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::BuiltinTransformHandle{};
#else
  try {
    auto r = std::make_shared<ocio_rs_bridge::RealBuiltinTransform>();
    r->transform = ocio::BuiltinTransform::Create();
    auto hdl = std::make_unique<ocio_rs_bridge::BuiltinTransformHandle>();
    hdl->inner = r;
    return hdl.release();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_builtin_transform_get_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BuiltinTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinTransform>(h->inner)->transform;
    // Call getStyle() and cache the result to avoid returning a dangling pointer
    static thread_local std::string cached;
    cached = t->getStyle();
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_builtin_transform_set_style(void* transform, const char* style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BuiltinTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinTransform>(h->inner)->transform;
    t->setStyle(style);
  } catch (...) {}
#endif
}

int ocio_builtin_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BuiltinTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinTransform>(h->inner)->transform;
    return static_cast<int>(t->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_builtin_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BuiltinTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinTransform>(h->inner)->transform;
    t->setDirection(static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_builtin_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::BuiltinTransformHandle*>(handle);
}

// --- BuiltinTransform: static methods ---

int ocio_builtin_transform_get_num_styles(void) {
#ifdef OCIO_RS_STUB
  return 0;
#else
  try {
    return ocio::BuiltinTransform::GetNumBuiltinStyles();
  } catch (...) { return 0; }
#endif
}

const char* ocio_builtin_transform_get_style_by_index(int index) {
#ifdef OCIO_RS_STUB
  (void)index; return nullptr;
#else
  try {
    static thread_local std::string cached;
    cached = ocio::BuiltinTransform::GetBuiltinStyle(index);
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_builtin_transform_is_valid_style(const char* style) {
#ifdef OCIO_RS_STUB
  (void)style; return false;
#else
  try {
    return ocio::BuiltinTransform::IsValidBuiltinStyle(style);
  } catch (...) { return false; }
#endif
}

// --- FixedFunctionTransform ---

void* ocio_fixed_function_transform_create(int style) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::FixedFunctionTransformHandle{};
#else
  try {
    auto r = std::make_shared<ocio_rs_bridge::RealFixedFunctionTransform>();
    r->transform = ocio::FixedFunctionTransform::Create(
        static_cast<ocio::FixedFunctionStyle>(style));
    auto hdl = std::make_unique<ocio_rs_bridge::FixedFunctionTransformHandle>();
    hdl->inner = r;
    return hdl.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_fixed_function_transform_create_with_params(int style, const double* params, int num_params) {
#ifdef OCIO_RS_STUB
  (void)params; (void)num_params;
  return new ocio_rs_bridge::FixedFunctionTransformHandle{};
#else
  try {
    auto r = std::make_shared<ocio_rs_bridge::RealFixedFunctionTransform>();
    r->transform = ocio::FixedFunctionTransform::Create(
        static_cast<ocio::FixedFunctionStyle>(style), params, num_params);
    auto hdl = std::make_unique<ocio_rs_bridge::FixedFunctionTransformHandle>();
    hdl->inner = r;
    return hdl.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_fixed_function_transform_get_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealFixedFunctionTransform>(h->inner)->transform;
    return static_cast<int>(t->getStyle());
  } catch (...) { return 0; }
#endif
}

void ocio_fixed_function_transform_set_style(void* transform, int style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealFixedFunctionTransform>(h->inner)->transform;
    t->setStyle(static_cast<ocio::FixedFunctionStyle>(style));
  } catch (...) {}
#endif
}

int ocio_fixed_function_transform_get_num_params(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealFixedFunctionTransform>(h->inner)->transform;
    return static_cast<int>(t->getNumParams());
  } catch (...) { return 0; }
#endif
}

void ocio_fixed_function_transform_get_params(void* transform, double* params) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)params;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealFixedFunctionTransform>(h->inner)->transform;
    t->getParams(params);
  } catch (...) {}
#endif
}

void ocio_fixed_function_transform_set_params(void* transform, const double* params, int num_params) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)params; (void)num_params;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealFixedFunctionTransform>(h->inner)->transform;
    t->setParams(params, num_params);
  } catch (...) {}
#endif
}

int ocio_fixed_function_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealFixedFunctionTransform>(h->inner)->transform;
    return static_cast<int>(t->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_fixed_function_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealFixedFunctionTransform>(h->inner)->transform;
    t->setDirection(static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_fixed_function_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::FixedFunctionTransformHandle*>(handle);
}

// --- Lut1DTransform ---

void* ocio_lut1d_transform_create(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::Lut1DTransformHandle{};
#else
  try {
    auto r = std::make_shared<ocio_rs_bridge::RealLut1DTransform>();
    r->transform = ocio::Lut1DTransform::Create();
    auto hdl = std::make_unique<ocio_rs_bridge::Lut1DTransformHandle>();
    hdl->inner = r;
    return hdl.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_lut1d_transform_get_interpolation(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    return static_cast<int>(t->getInterpolation());
  } catch (...) { return 0; }
#endif
}

void ocio_lut1d_transform_set_interpolation(void* transform, int interpolation) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)interpolation;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    t->setInterpolation(static_cast<ocio::Interpolation>(interpolation));
  } catch (...) {}
#endif
}

int ocio_lut1d_transform_get_file_output_bit_depth(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    return static_cast<int>(t->getFileOutputBitDepth());
  } catch (...) { return 0; }
#endif
}

void ocio_lut1d_transform_set_file_output_bit_depth(void* transform, int bit_depth) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)bit_depth;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    t->setFileOutputBitDepth(static_cast<ocio::BitDepth>(bit_depth));
  } catch (...) {}
#endif
}

int ocio_lut1d_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    return static_cast<int>(t->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_lut1d_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    t->setDirection(static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_lut1d_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(handle);
}

// --- Lut3DTransform ---

void* ocio_lut3d_transform_create(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::Lut3DTransformHandle{};
#else
  try {
    auto r = std::make_shared<ocio_rs_bridge::RealLut3DTransform>();
    r->transform = ocio::Lut3DTransform::Create();
    auto hdl = std::make_unique<ocio_rs_bridge::Lut3DTransformHandle>();
    hdl->inner = r;
    return hdl.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_lut3d_transform_get_interpolation(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform;
    return static_cast<int>(t->getInterpolation());
  } catch (...) { return 0; }
#endif
}

void ocio_lut3d_transform_set_interpolation(void* transform, int interpolation) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)interpolation;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform;
    t->setInterpolation(static_cast<ocio::Interpolation>(interpolation));
  } catch (...) {}
#endif
}

int ocio_lut3d_transform_get_file_output_bit_depth(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform;
    return static_cast<int>(t->getFileOutputBitDepth());
  } catch (...) { return 0; }
#endif
}

void ocio_lut3d_transform_set_file_output_bit_depth(void* transform, int bit_depth) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)bit_depth;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform;
    t->setFileOutputBitDepth(static_cast<ocio::BitDepth>(bit_depth));
  } catch (...) {}
#endif
}

int ocio_lut3d_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform;
    return static_cast<int>(t->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_lut3d_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform;
    t->setDirection(static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

void ocio_lut3d_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(handle);
}

// --- Baker ---

void* ocio_baker_create(void) {
#ifdef OCIO_RS_STUB
  auto handle = std::make_unique<ocio_rs_bridge::BakerHandle>();
  handle->inner = std::make_shared<ocio_rs_bridge::StubBaker>();
  return handle.release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::BakerHandle>();
    auto baker = std::make_shared<ocio_rs_bridge::RealBaker>();
    baker->baker = ocio::Baker::Create();
    handle->inner = baker;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_baker_create_editable_copy(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    auto orig = std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner);
    auto handle = std::make_unique<ocio_rs_bridge::BakerHandle>();
    auto copy = std::make_shared<ocio_rs_bridge::RealBaker>();
    copy->baker = orig->baker->createEditableCopy();
    handle->inner = copy;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_config(void* baker, void* config) {
#ifdef OCIO_RS_STUB
  (void)baker; (void)config;
#else
  try {
    auto* bh = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    auto b = std::static_pointer_cast<ocio_rs_bridge::RealBaker>(bh->inner);
    auto cfg = ocio_rs_bridge::get_real_config(config);
    b->baker->setConfig(cfg);
  } catch (...) {}
#endif
}

void* ocio_baker_get_config(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return nullptr;
#else
  try {
    auto* bh = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    auto b = std::static_pointer_cast<ocio_rs_bridge::RealBaker>(bh->inner);
    auto handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();
    auto config = std::make_shared<ocio_rs_bridge::RealConfig>();
    config->config = b->baker->getConfig();
    handle->inner = config;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_baker_get_format(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    return std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->getFormat();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_format(void* baker, const char* formatName) {
#ifdef OCIO_RS_STUB
  (void)baker; (void)formatName;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->setFormat(formatName);
  } catch (...) {}
#endif
}

const char* ocio_baker_get_input_space(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    return std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->getInputSpace();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_input_space(void* baker, const char* inputSpace) {
#ifdef OCIO_RS_STUB
  (void)baker; (void)inputSpace;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->setInputSpace(inputSpace);
  } catch (...) {}
#endif
}

const char* ocio_baker_get_shaper_space(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    return std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->getShaperSpace();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_shaper_space(void* baker, const char* shaperSpace) {
#ifdef OCIO_RS_STUB
  (void)baker; (void)shaperSpace;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->setShaperSpace(shaperSpace);
  } catch (...) {}
#endif
}

const char* ocio_baker_get_looks(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    return std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->getLooks();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_looks(void* baker, const char* looks) {
#ifdef OCIO_RS_STUB
  (void)baker; (void)looks;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->setLooks(looks);
  } catch (...) {}
#endif
}

const char* ocio_baker_get_target_space(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    return std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->getTargetSpace();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_target_space(void* baker, const char* targetSpace) {
#ifdef OCIO_RS_STUB
  (void)baker; (void)targetSpace;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->setTargetSpace(targetSpace);
  } catch (...) {}
#endif
}

const char* ocio_baker_get_display(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    return std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->getDisplay();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_baker_get_view(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    return std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->getView();
  } catch (...) { return nullptr; }
#endif
}

void ocio_baker_set_display_view(void* baker, const char* display, const char* view) {
#ifdef OCIO_RS_STUB
  (void)baker; (void)display; (void)view;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->setDisplayView(display, view);
  } catch (...) {}
#endif
}

int ocio_baker_get_shaper_size(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    return std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->getShaperSize();
  } catch (...) { return 0; }
#endif
}

void ocio_baker_set_shaper_size(void* baker, int size) {
#ifdef OCIO_RS_STUB
  (void)baker; (void)size;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->setShaperSize(size);
  } catch (...) {}
#endif
}

int ocio_baker_get_cube_size(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    return std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->getCubeSize();
  } catch (...) { return 0; }
#endif
}

void ocio_baker_set_cube_size(void* baker, int size) {
#ifdef OCIO_RS_STUB
  (void)baker; (void)size;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->setCubeSize(size);
  } catch (...) {}
#endif
}

int ocio_baker_get_target_bit_depth(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    return std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->getTargetBitDepth();
  } catch (...) { return 0; }
#endif
}

void ocio_baker_set_target_bit_depth(void* baker, int bitDepth) {
#ifdef OCIO_RS_STUB
  (void)baker; (void)bitDepth;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner)->baker->setTargetBitDepth(bitDepth);
  } catch (...) {}
#endif
}

void ocio_baker_bake(void* baker, const char* outputPath) {
#ifdef OCIO_RS_STUB
  (void)baker; (void)outputPath;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BakerHandle*>(baker);
    auto b = std::static_pointer_cast<ocio_rs_bridge::RealBaker>(h->inner);
    std::ofstream out(outputPath, std::ios::binary);
    b->baker->bake(out);
  } catch (...) {}
#endif
}

void ocio_baker_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::BakerHandle*>(handle);
}

// --- Baker: static format metadata ---

int ocio_baker_get_num_formats(void) {
#ifdef OCIO_RS_STUB
  return 0;
#else
  try {
    return OCIO::Baker::getNumFormats();
  } catch (...) { return 0; }
#endif
}

const char* ocio_baker_get_format_name_by_index(int index) {
#ifdef OCIO_RS_STUB
  (void)index;
  static const char* empty = "";
  return empty;
#else
  try {
    return OCIO::Baker::getFormatNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_baker_get_format_extension_by_index(int index) {
#ifdef OCIO_RS_STUB
  (void)index;
  static const char* empty = "";
  return empty;
#else
  try {
    return OCIO::Baker::getFormatExtensionByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

// --- Context ---

void* ocio_context_create(void) {
#ifdef OCIO_RS_STUB
  auto handle = std::make_unique<ocio_rs_bridge::ContextHandle>();
  handle->inner = std::make_shared<ocio_rs_bridge::StubContext>();
  return handle.release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::ContextHandle>();
    auto ctx = std::make_shared<ocio_rs_bridge::RealContext>();
    ctx->context = ocio::Context::Create();
    handle->inner = ctx;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_context_create_editable_copy(void* context) {
#ifdef OCIO_RS_STUB
  (void)context; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    auto orig = std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner);
    auto handle = std::make_unique<ocio_rs_bridge::ContextHandle>();
    auto copy = std::make_shared<ocio_rs_bridge::RealContext>();
    copy->context = orig->context->createEditableCopy();
    handle->inner = copy;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_context_get_cache_id(void* context) {
#ifdef OCIO_RS_STUB
  (void)context; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->getCacheID();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_context_get_search_path(void* context) {
#ifdef OCIO_RS_STUB
  (void)context; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->getSearchPath();
  } catch (...) { return nullptr; }
#endif
}

void ocio_context_set_search_path(void* context, const char* path) {
#ifdef OCIO_RS_STUB
  (void)context; (void)path;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->setSearchPath(path);
  } catch (...) {}
#endif
}

int ocio_context_get_num_search_paths(void* context) {
#ifdef OCIO_RS_STUB
  (void)context; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->getNumSearchPaths();
  } catch (...) { return 0; }
#endif
}

const char* ocio_context_get_search_path_by_index(void* context, int index) {
#ifdef OCIO_RS_STUB
  (void)context; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->getSearchPath(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_context_clear_search_paths(void* context) {
#ifdef OCIO_RS_STUB
  (void)context;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->clearSearchPaths();
  } catch (...) {}
#endif
}

void ocio_context_add_search_path(void* context, const char* path) {
#ifdef OCIO_RS_STUB
  (void)context; (void)path;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->addSearchPath(path);
  } catch (...) {}
#endif
}

const char* ocio_context_get_working_dir(void* context) {
#ifdef OCIO_RS_STUB
  (void)context; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->getWorkingDir();
  } catch (...) { return nullptr; }
#endif
}

void ocio_context_set_working_dir(void* context, const char* dirname) {
#ifdef OCIO_RS_STUB
  (void)context; (void)dirname;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->setWorkingDir(dirname);
  } catch (...) {}
#endif
}

const char* ocio_context_get_string_var(void* context, const char* name) {
#ifdef OCIO_RS_STUB
  (void)context; (void)name; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->getStringVar(name);
  } catch (...) { return nullptr; }
#endif
}

void ocio_context_set_string_var(void* context, const char* name, const char* value) {
#ifdef OCIO_RS_STUB
  (void)context; (void)name; (void)value;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->setStringVar(name, value);
  } catch (...) {}
#endif
}

int ocio_context_get_num_string_vars(void* context) {
#ifdef OCIO_RS_STUB
  (void)context; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->getNumStringVars();
  } catch (...) { return 0; }
#endif
}

const char* ocio_context_get_string_var_name_by_index(void* context, int index) {
#ifdef OCIO_RS_STUB
  (void)context; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->getStringVarNameByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_context_get_string_var_by_index(void* context, int index) {
#ifdef OCIO_RS_STUB
  (void)context; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->getStringVarByIndex(index);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_context_resolve_string_var(void* context, const char* string) {
#ifdef OCIO_RS_STUB
  (void)context; (void)string; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->resolveStringVar(string);
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_context_resolve_file_location(void* context, const char* filename) {
#ifdef OCIO_RS_STUB
  (void)context; (void)filename; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->resolveFileLocation(filename);
  } catch (...) { return nullptr; }
#endif
}

void ocio_context_clear_string_vars(void* context) {
#ifdef OCIO_RS_STUB
  (void)context;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->clearStringVars();
  } catch (...) {}
#endif
}

void ocio_context_set_environment_mode(void* context, int mode) {
#ifdef OCIO_RS_STUB
  (void)context; (void)mode;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->setEnvironmentMode(
        static_cast<ocio::EnvironmentMode>(mode));
  } catch (...) {}
#endif
}

int ocio_context_get_environment_mode(void* context) {
#ifdef OCIO_RS_STUB
  (void)context; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->getEnvironmentMode());
  } catch (...) { return 0; }
#endif
}

void ocio_context_load_environment(void* context) {
#ifdef OCIO_RS_STUB
  (void)context;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ContextHandle*>(context);
    std::static_pointer_cast<ocio_rs_bridge::RealContext>(h->inner)->context->loadEnvironment();
  } catch (...) {}
#endif
}

void ocio_context_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ContextHandle*>(handle);
}

// --- ColorSpace ---

void* ocio_color_space_create(void) {
#ifdef OCIO_RS_STUB
  auto handle = std::make_unique<ocio_rs_bridge::ColorSpaceHandle>();
  handle->inner = std::make_shared<ocio_rs_bridge::StubColorSpace>();
  return handle.release();
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::ColorSpaceHandle>();
    auto cs = std::make_shared<ocio_rs_bridge::RealColorSpace>();
    cs->colorSpace = ocio::ColorSpace::Create();
    handle->inner = cs;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_color_space_create_editable_copy(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    auto orig = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner);
    auto handle = std::make_unique<ocio_rs_bridge::ColorSpaceHandle>();
    auto copy = std::make_shared<ocio_rs_bridge::RealColorSpace>();
    copy->colorSpace = orig->colorSpace->createEditableCopy();
    handle->inner = copy;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_color_space_get_name(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getName();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_name(void* colorSpace, const char* name) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)name;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->setName(name);
  } catch (...) {}
#endif
}

const char* ocio_color_space_get_family(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getFamily();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_family(void* colorSpace, const char* family) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)family;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->setFamily(family);
  } catch (...) {}
#endif
}

const char* ocio_color_space_get_equality_group(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getEqualityGroup();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_equality_group(void* colorSpace, const char* group) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)group;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->setEqualityGroup(group);
  } catch (...) {}
#endif
}

const char* ocio_color_space_get_description(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getDescription();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_description(void* colorSpace, const char* description) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)description;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->setDescription(description);
  } catch (...) {}
#endif
}

int ocio_color_space_get_bit_depth(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return 8;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getBitDepth());
  } catch (...) { return 0; }
#endif
}

void ocio_color_space_set_bit_depth(void* colorSpace, int bitDepth) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)bitDepth;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->setBitDepth(
        static_cast<ocio::BitDepth>(bitDepth));
  } catch (...) {}
#endif
}

int ocio_color_space_get_reference_space_type(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getReferenceSpaceType());
  } catch (...) { return 0; }
#endif
}

bool ocio_color_space_is_data(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->isData();
  } catch (...) { return false; }
#endif
}

void ocio_color_space_set_is_data(void* colorSpace, bool isData) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)isData;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->setIsData(isData);
  } catch (...) {}
#endif
}

const char* ocio_color_space_get_category(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getCategory();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_category(void* colorSpace, const char* category) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)category;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->setCategory(category);
  } catch (...) {}
#endif
}

int ocio_color_space_get_allocation(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getAllocation());
  } catch (...) { return 0; }
#endif
}

void ocio_color_space_set_allocation(void* colorSpace, int allocation) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)allocation;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->setAllocation(
        static_cast<ocio::Allocation>(allocation));
  } catch (...) {}
#endif
}

int ocio_color_space_get_allocation_num_vars(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getAllocationNumVars();
  } catch (...) { return 0; }
#endif
}

void ocio_color_space_get_allocation_vars(void* colorSpace, float* vars) {
#ifdef OCIO_RS_STUB
  (void)colorSpace;
  vars[0] = 0.0f;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getAllocationVars(vars);
  } catch (...) {}
#endif
}

void ocio_color_space_set_allocation_vars(void* colorSpace, int numVars, const float* vars) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)numVars; (void)vars;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->setAllocationVars(numVars, vars);
  } catch (...) {}
#endif
}

void* ocio_color_space_get_transform(void* colorSpace, int direction) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)direction; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getTransform(
        static_cast<ocio::ColorSpaceDirection>(direction));
    if (!t) return nullptr;
    int type = static_cast<int>(t->getTransformType());
    TransformHandleBase* out = nullptr;
    switch (type) {
      case 1: { // Builtin
        auto ct = ocio::DynamicPtrCast<const ocio::BuiltinTransform>(t);
        auto hdl = std::make_unique<BuiltinTransformHandle>();
        auto r = std::make_shared<RealBuiltinTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 2: { // CDL
        auto ct = ocio::DynamicPtrCast<const ocio::CDLTransform>(t);
        auto hdl = std::make_unique<CDLTransformHandle>();
        auto r = std::make_shared<RealCDLTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 5: { // Exponent
        auto ct = ocio::DynamicPtrCast<const ocio::ExponentTransform>(t);
        auto hdl = std::make_unique<ExponentTransformHandle>();
        auto r = std::make_shared<RealExponentTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 8: { // File
        auto ct = ocio::DynamicPtrCast<const ocio::FileTransform>(t);
        auto hdl = std::make_unique<FileTransformHandle>();
        auto r = std::make_shared<RealFileTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 9: { // FixedFunction
        auto ct = ocio::DynamicPtrCast<const ocio::FixedFunctionTransform>(t);
        auto hdl = std::make_unique<FixedFunctionTransformHandle>();
        auto r = std::make_shared<RealFixedFunctionTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 14: { // Group
        auto ct = ocio::DynamicPtrCast<const ocio::GroupTransform>(t);
        auto hdl = std::make_unique<GroupTransformHandle>();
        auto r = std::make_shared<RealGroupTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 17: { // Log
        auto ct = ocio::DynamicPtrCast<const ocio::LogTransform>(t);
        auto hdl = std::make_unique<LogTransformHandle>();
        auto r = std::make_shared<RealLogTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 19: { // Lut1D
        auto ct = ocio::DynamicPtrCast<const ocio::Lut1DTransform>(t);
        auto hdl = std::make_unique<Lut1DTransformHandle>();
        auto r = std::make_shared<RealLut1DTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 20: { // Lut3D
        auto ct = ocio::DynamicPtrCast<const ocio::Lut3DTransform>(t);
        auto hdl = std::make_unique<Lut3DTransformHandle>();
        auto r = std::make_shared<RealLut3DTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 21: { // Matrix
        auto ct = ocio::DynamicPtrCast<const ocio::MatrixTransform>(t);
        auto hdl = std::make_unique<MatrixTransformHandle>();
        auto r = std::make_shared<RealMatrixTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 22: { // Range
        auto ct = ocio::DynamicPtrCast<const ocio::RangeTransform>(t);
        auto hdl = std::make_unique<RangeTransformHandle>();
        auto r = std::make_shared<RealRangeTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 7: { // ExposureContrast
        auto ct = ocio::DynamicPtrCast<const ocio::ExposureContrastTransform>(t);
        auto hdl = std::make_unique<ExposureContrastTransformHandle>();
        auto r = std::make_shared<RealExposureContrastTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 4: { // ColorSpace
        auto ct = ocio::DynamicPtrCast<const ocio::ColorSpaceTransform>(t);
        auto hdl = std::make_unique<ColorSpaceTransformHandle>();
        auto r = std::make_shared<RealColorSpaceTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 18: { // Look
        auto ct = ocio::DynamicPtrCast<const ocio::LookTransform>(t);
        auto hdl = std::make_unique<LookTransformHandle>();
        auto r = std::make_shared<RealLookTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 0: { // Allocation
        auto ct = ocio::DynamicPtrCast<const ocio::AllocationTransform>(t);
        auto hdl = std::make_unique<AllocationTransformHandle>();
        auto r = std::make_shared<RealAllocationTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 15: { // LogAffine
        auto ct = ocio::DynamicPtrCast<const ocio::LogAffineTransform>(t);
        auto hdl = std::make_unique<LogAffineTransformHandle>();
        auto r = std::make_shared<RealLogAffineTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 16: { // LogCamera
        auto ct = ocio::DynamicPtrCast<const ocio::LogCameraTransform>(t);
        auto hdl = std::make_unique<LogCameraTransformHandle>();
        auto r = std::make_shared<RealLogCameraTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 11: { // GradingPrimary
        auto ct = ocio::DynamicPtrCast<const ocio::GradingPrimaryTransform>(t);
        auto hdl = std::make_unique<GradingPrimaryTransformHandle>();
        auto r = std::make_shared<RealGradingPrimaryTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 12: { // GradingRGBCurve
        auto ct = ocio::DynamicPtrCast<const ocio::GradingRGBCurveTransform>(t);
        auto hdl = std::make_unique<GradingRGBCurveTransformHandle>();
        auto r = std::make_shared<RealGradingRGBCurveTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      case 13: { // GradingTone
        auto ct = ocio::DynamicPtrCast<const ocio::GradingToneTransform>(t);
        auto hdl = std::make_unique<GradingToneTransformHandle>();
        auto r = std::make_shared<RealGradingToneTransform>();
        r->transform = ct->createEditableCopy();
        hdl->inner = r;
        out = hdl.release();
        break;
      }
      default: return nullptr;
    }
    return out;
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_transform(void* colorSpace, const void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)transform; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    auto cs = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace;
    auto* base = static_cast<const ocio_rs_bridge::TransformHandleBase*>(transform);
    auto ocio_transform = base->get_ocio_transform();
    if (ocio_transform) {
      cs->setTransform(ocio_transform, static_cast<ocio::ColorSpaceDirection>(direction));
    }
  } catch (...) {}
#endif
}

const char* ocio_color_space_get_encoding(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getEncoding();
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_set_encoding(void* colorSpace, const char* encoding) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)encoding;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->setEncoding(encoding);
  } catch (...) {}
#endif
}

const char* ocio_color_space_get_cache_id(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace;
  return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getCacheID();
  } catch (...) { return nullptr; }
#endif
}

bool ocio_color_space_is_transform_defined(void* colorSpace, int direction) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)direction;
  return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->isTransformDefined(
        static_cast<ocio::ColorSpaceDirection>(direction));
  } catch (...) { return false; }
#endif
}

void ocio_color_space_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ColorSpaceHandle*>(handle);
}

// --- ColorSpace: aliases ---

int ocio_color_space_get_num_aliases(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getNumAliases();
  } catch (...) { return 0; }
#endif
}

const char* ocio_color_space_get_alias(void* colorSpace, int index) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->getAlias(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_color_space_add_alias(void* colorSpace, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)alias;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->addAlias(alias);
  } catch (...) {}
#endif
}

void ocio_color_space_remove_alias(void* colorSpace, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)alias;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->removeAlias(alias);
  } catch (...) {}
#endif
}

void ocio_color_space_clear_aliases(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->clearAliases();
  } catch (...) {}
#endif
}

// --- ColorSpace: inactive ---

bool ocio_color_space_is_inactive(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    return std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->isInactive();
  } catch (...) { return false; }
#endif
}

void ocio_color_space_set_inactive(void* colorSpace, bool inactive) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)inactive;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner)->colorSpace->setIsInactive(inactive);
  } catch (...) {}
#endif
}

// --- Config: ColorSpace by object ---

void* ocio_config_get_color_space(void* config, const char* name) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name; return nullptr;
#else
  try {
    auto cs = ocio_rs_bridge::get_real_config(config)->getColorSpace(name);
    if (!cs) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::ColorSpaceHandle>();
    auto wrapper = std::make_shared<ocio_rs_bridge::RealColorSpace>();
    wrapper->colorSpace = cs->createEditableCopy();
    handle->inner = wrapper;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

int ocio_config_get_index_for_color_space(void* config, const char* name) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name; return -1;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getIndexForColorSpace(name);
  } catch (...) { return -1; }
#endif
}

void ocio_config_add_color_space(void* config, void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)config; (void)colorSpace;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace);
    auto cs = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(h->inner);
    ocio_rs_bridge::get_real_config(config)->addColorSpace(cs->colorSpace);
  } catch (...) {}
#endif
}

void ocio_config_remove_color_space(void* config, const char* name) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->removeColorSpace(name);
  } catch (...) {}
#endif
}

bool ocio_config_is_color_space_used(void* config, const char* name) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name; return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->isColorSpaceUsed(name);
  } catch (...) { return false; }
#endif
}

// --- Config: Looks by object ---

void* ocio_config_get_look(void* config, const char* name) {
#ifdef OCIO_RS_STUB
  (void)config; (void)name; return nullptr;
#else
  try {
    auto lk = ocio_rs_bridge::get_real_config(config)->getLook(name);
    if (!lk) return nullptr;
    auto handle = std::make_unique<ocio_rs_bridge::LookHandle>();
    auto wrapper = std::make_shared<ocio_rs_bridge::RealLook>();
    wrapper->look = lk->createEditableCopy();
    handle->inner = wrapper;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_add_look(void* config, void* look) {
#ifdef OCIO_RS_STUB
  (void)config; (void)look;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    auto lk = std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner);
    ocio_rs_bridge::get_real_config(config)->addLook(lk->look);
  } catch (...) {}
#endif
}

// --- Look ---

void* ocio_look_create(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::LookHandle{};
#else
  try {
    auto handle = std::make_unique<ocio_rs_bridge::LookHandle>();
    auto wrapper = std::make_shared<ocio_rs_bridge::RealLook>();
    wrapper->look = ocio::Look::Create();
    handle->inner = wrapper;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_look_create_editable_copy(void* look) {
#ifdef OCIO_RS_STUB
  (void)look; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    auto lk = std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner);
    auto handle = std::make_unique<ocio_rs_bridge::LookHandle>();
    auto wrapper = std::make_shared<ocio_rs_bridge::RealLook>();
    wrapper->look = lk->look->createEditableCopy();
    handle->inner = wrapper;
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_look_get_name(void* look) {
#ifdef OCIO_RS_STUB
  (void)look; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    auto lk = std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner);
    static thread_local std::string cached;
    cached = lk->look->getName();
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_set_name(void* look, const char* name) {
#ifdef OCIO_RS_STUB
  (void)look; (void)name;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    auto lk = std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner);
    lk->look->setName(name);
  } catch (...) {}
#endif
}

const char* ocio_look_get_process_space(void* look) {
#ifdef OCIO_RS_STUB
  (void)look; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    auto lk = std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner);
    static thread_local std::string cached;
    cached = lk->look->getProcessSpace();
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_set_process_space(void* look, const char* processSpace) {
#ifdef OCIO_RS_STUB
  (void)look; (void)processSpace;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    auto lk = std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner);
    lk->look->setProcessSpace(processSpace);
  } catch (...) {}
#endif
}

int ocio_look_get_direction(void* look) {
#ifdef OCIO_RS_STUB
  (void)look; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    auto lk = std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner);
    return static_cast<int>(lk->look->getDirection());
  } catch (...) { return 0; }
#endif
}

void ocio_look_set_direction(void* look, int direction) {
#ifdef OCIO_RS_STUB
  (void)look; (void)direction;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    auto lk = std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner);
    lk->look->setDirection(static_cast<ocio::TransformDirection>(direction));
  } catch (...) {}
#endif
}

const char* ocio_look_get_description(void* look) {
#ifdef OCIO_RS_STUB
  (void)look; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    return std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner)->look->getDescription();
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_set_description(void* look, const char* description) {
#ifdef OCIO_RS_STUB
  (void)look; (void)description;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner)->look->setDescription(description);
  } catch (...) {}
#endif
}

void* ocio_look_get_transform(void* look) {
#ifdef OCIO_RS_STUB
  (void)look; return nullptr;
#else
  BEGIN_TRY
  auto lk = static_cast<ocio_rs_bridge::LookHandle*>(look);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLook>(lk->inner);
  ocio::ConstTransformRcPtr t = real->look->getTransform();
  if (!t) return nullptr;
  switch (t->getTransformType()) {
    case ocio::TRANSFORM_TYPE_ALLOCATION: {
      auto alloc = ocio::DynamicPtrCast<ocio::AllocationTransform>(t);
      auto handle = new ocio_rs_bridge::AllocationTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealAllocationTransform>(ocio_rs_bridge::RealAllocationTransform{alloc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_BUILTIN: {
      auto builtin = ocio::DynamicPtrCast<ocio::BuiltinTransform>(t);
      auto handle = new ocio_rs_bridge::BuiltinTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealBuiltinTransform>(ocio_rs_bridge::RealBuiltinTransform{builtin});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_CDL: {
      auto cdl = ocio::DynamicPtrCast<ocio::CDLTransform>(t);
      auto handle = new ocio_rs_bridge::CDLTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealCDLTransform>(ocio_rs_bridge::RealCDLTransform{cdl});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_COLOR_SPACE: {
      auto cs = ocio::DynamicPtrCast<ocio::ColorSpaceTransform>(t);
      auto handle = new ocio_rs_bridge::ColorSpaceTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceTransform>(ocio_rs_bridge::RealColorSpaceTransform{cs});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_EXPONENT: {
      auto exp = ocio::DynamicPtrCast<ocio::ExponentTransform>(t);
      auto handle = new ocio_rs_bridge::ExponentTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealExponentTransform>(ocio_rs_bridge::RealExponentTransform{exp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_EXPOSURE_CONTRAST: {
      auto ec = ocio::DynamicPtrCast<ocio::ExposureContrastTransform>(t);
      auto handle = new ocio_rs_bridge::ExposureContrastTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealExposureContrastTransform>(ocio_rs_bridge::RealExposureContrastTransform{ec});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_FILE: {
      auto file = ocio::DynamicPtrCast<ocio::FileTransform>(t);
      auto handle = new ocio_rs_bridge::FileTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealFileTransform>(ocio_rs_bridge::RealFileTransform{file});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_FIXED_FUNCTION: {
      auto ff = ocio::DynamicPtrCast<ocio::FixedFunctionTransform>(t);
      auto handle = new ocio_rs_bridge::FixedFunctionHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealFixedFunctionTransform>(ocio_rs_bridge::RealFixedFunctionTransform{ff});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_HUE_CURVE: {
      auto ghc = ocio::DynamicPtrCast<ocio::GradingHueCurveTransform>(t);
      auto handle = new ocio_rs_bridge::GradingHueCurveTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingHueCurveTransform>(ocio_rs_bridge::RealGradingHueCurveTransform{ghc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_PRIMARY: {
      auto gp = ocio::DynamicPtrCast<ocio::GradingPrimaryTransform>(t);
      auto handle = new ocio_rs_bridge::GradingPrimaryTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingPrimaryTransform>(ocio_rs_bridge::RealGradingPrimaryTransform{gp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_RGB_CURVE: {
      auto gc = ocio::DynamicPtrCast<ocio::GradingRGBCurveTransform>(t);
      auto handle = new ocio_rs_bridge::GradingRGBCurveTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingRGBCurveTransform>(ocio_rs_bridge::RealGradingRGBCurveTransform{gc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_TONE: {
      auto gt = ocio::DynamicPtrCast<ocio::GradingToneTransform>(t);
      auto handle = new ocio_rs_bridge::GradingToneTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingToneTransform>(ocio_rs_bridge::RealGradingToneTransform{gt});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GROUP: {
      auto grp = ocio::DynamicPtrCast<ocio::GroupTransform>(t);
      auto handle = new ocio_rs_bridge::GroupTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGroupTransform>(ocio_rs_bridge::RealGroupTransform{grp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG: {
      auto log = ocio::DynamicPtrCast<ocio::LogTransform>(t);
      auto handle = new ocio_rs_bridge::LogTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogTransform>(ocio_rs_bridge::RealLogTransform{log});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG_AFFINE: {
      auto la = ocio::DynamicPtrCast<ocio::LogAffineTransform>(t);
      auto handle = new ocio_rs_bridge::LogAffineTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogAffineTransform>(ocio_rs_bridge::RealLogAffineTransform{la});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG_CAMERA: {
      auto lc = ocio::DynamicPtrCast<ocio::LogCameraTransform>(t);
      auto handle = new ocio_rs_bridge::LogCameraTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogCameraTransform>(ocio_rs_bridge::RealLogCameraTransform{lc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOOK: {
      auto lk2 = ocio::DynamicPtrCast<ocio::LookTransform>(t);
      auto handle = new ocio_rs_bridge::LookTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLookTransform>(ocio_rs_bridge::RealLookTransform{lk2});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LUT1D: {
      auto lut1d = ocio::DynamicPtrCast<ocio::Lut1DTransform>(t);
      auto handle = new ocio_rs_bridge::Lut1DHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLut1DTransform>(ocio_rs_bridge::RealLut1DTransform{lut1d});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LUT3D: {
      auto lut3d = ocio::DynamicPtrCast<ocio::Lut3DTransform>(t);
      auto handle = new ocio_rs_bridge::Lut3DHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLut3DTransform>(ocio_rs_bridge::RealLut3DTransform{lut3d});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_MATRIX: {
      auto mat = ocio::DynamicPtrCast<ocio::MatrixTransform>(t);
      auto handle = new ocio_rs_bridge::MatrixTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealMatrixTransform>(ocio_rs_bridge::RealMatrixTransform{mat});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_RANGE: {
      auto range = ocio::DynamicPtrCast<ocio::RangeTransform>(t);
      auto handle = new ocio_rs_bridge::RangeTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealRangeTransform>(ocio_rs_bridge::RealRangeTransform{range});
      return handle;
    }
    default: return nullptr;
  }
  END_TRY(nullptr)
#endif
}

void ocio_look_set_transform(void* look, const void* transform) {
#ifdef OCIO_RS_STUB
  (void)look; (void)transform;
#else
  BEGIN_TRY
  auto lk = static_cast<ocio_rs_bridge::LookHandle*>(look);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLook>(lk->inner);
  if (transform) {
    auto th = static_cast<const ocio_rs_bridge::TransformHandleBase*>(transform);
    real->look->setTransform(th->get_ocio_transform());
  } else {
    real->look->setTransform(nullptr);
  }
  END_TRY_VOID
#endif
}

void* ocio_look_get_inverse_transform(void* look) {
#ifdef OCIO_RS_STUB
  (void)look; return nullptr;
#else
  BEGIN_TRY
  auto lk = static_cast<ocio_rs_bridge::LookHandle*>(look);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLook>(lk->inner);
  ocio::ConstTransformRcPtr t = real->look->getInverseTransform();
  if (!t) return nullptr;
  switch (t->getTransformType()) {
    case ocio::TRANSFORM_TYPE_ALLOCATION: {
      auto alloc = ocio::DynamicPtrCast<ocio::AllocationTransform>(t);
      auto handle = new ocio_rs_bridge::AllocationTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealAllocationTransform>(ocio_rs_bridge::RealAllocationTransform{alloc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_BUILTIN: {
      auto builtin = ocio::DynamicPtrCast<ocio::BuiltinTransform>(t);
      auto handle = new ocio_rs_bridge::BuiltinTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealBuiltinTransform>(ocio_rs_bridge::RealBuiltinTransform{builtin});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_CDL: {
      auto cdl = ocio::DynamicPtrCast<ocio::CDLTransform>(t);
      auto handle = new ocio_rs_bridge::CDLTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealCDLTransform>(ocio_rs_bridge::RealCDLTransform{cdl});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_COLOR_SPACE: {
      auto cs = ocio::DynamicPtrCast<ocio::ColorSpaceTransform>(t);
      auto handle = new ocio_rs_bridge::ColorSpaceTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceTransform>(ocio_rs_bridge::RealColorSpaceTransform{cs});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_EXPONENT: {
      auto exp = ocio::DynamicPtrCast<ocio::ExponentTransform>(t);
      auto handle = new ocio_rs_bridge::ExponentTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealExponentTransform>(ocio_rs_bridge::RealExponentTransform{exp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_EXPOSURE_CONTRAST: {
      auto ec = ocio::DynamicPtrCast<ocio::ExposureContrastTransform>(t);
      auto handle = new ocio_rs_bridge::ExposureContrastTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealExposureContrastTransform>(ocio_rs_bridge::RealExposureContrastTransform{ec});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_FILE: {
      auto file = ocio::DynamicPtrCast<ocio::FileTransform>(t);
      auto handle = new ocio_rs_bridge::FileTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealFileTransform>(ocio_rs_bridge::RealFileTransform{file});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_FIXED_FUNCTION: {
      auto ff = ocio::DynamicPtrCast<ocio::FixedFunctionTransform>(t);
      auto handle = new ocio_rs_bridge::FixedFunctionHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealFixedFunctionTransform>(ocio_rs_bridge::RealFixedFunctionTransform{ff});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_HUE_CURVE: {
      auto ghc = ocio::DynamicPtrCast<ocio::GradingHueCurveTransform>(t);
      auto handle = new ocio_rs_bridge::GradingHueCurveTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingHueCurveTransform>(ocio_rs_bridge::RealGradingHueCurveTransform{ghc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_PRIMARY: {
      auto gp = ocio::DynamicPtrCast<ocio::GradingPrimaryTransform>(t);
      auto handle = new ocio_rs_bridge::GradingPrimaryTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingPrimaryTransform>(ocio_rs_bridge::RealGradingPrimaryTransform{gp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_RGB_CURVE: {
      auto gc = ocio::DynamicPtrCast<ocio::GradingRGBCurveTransform>(t);
      auto handle = new ocio_rs_bridge::GradingRGBCurveTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingRGBCurveTransform>(ocio_rs_bridge::RealGradingRGBCurveTransform{gc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_TONE: {
      auto gt = ocio::DynamicPtrCast<ocio::GradingToneTransform>(t);
      auto handle = new ocio_rs_bridge::GradingToneTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingToneTransform>(ocio_rs_bridge::RealGradingToneTransform{gt});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GROUP: {
      auto grp = ocio::DynamicPtrCast<ocio::GroupTransform>(t);
      auto handle = new ocio_rs_bridge::GroupTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGroupTransform>(ocio_rs_bridge::RealGroupTransform{grp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG: {
      auto log = ocio::DynamicPtrCast<ocio::LogTransform>(t);
      auto handle = new ocio_rs_bridge::LogTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogTransform>(ocio_rs_bridge::RealLogTransform{log});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG_AFFINE: {
      auto la = ocio::DynamicPtrCast<ocio::LogAffineTransform>(t);
      auto handle = new ocio_rs_bridge::LogAffineTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogAffineTransform>(ocio_rs_bridge::RealLogAffineTransform{la});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG_CAMERA: {
      auto lc = ocio::DynamicPtrCast<ocio::LogCameraTransform>(t);
      auto handle = new ocio_rs_bridge::LogCameraTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogCameraTransform>(ocio_rs_bridge::RealLogCameraTransform{lc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOOK: {
      auto lk2 = ocio::DynamicPtrCast<ocio::LookTransform>(t);
      auto handle = new ocio_rs_bridge::LookTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLookTransform>(ocio_rs_bridge::RealLookTransform{lk2});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LUT1D: {
      auto lut1d = ocio::DynamicPtrCast<ocio::Lut1DTransform>(t);
      auto handle = new ocio_rs_bridge::Lut1DHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLut1DTransform>(ocio_rs_bridge::RealLut1DTransform{lut1d});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LUT3D: {
      auto lut3d = ocio::DynamicPtrCast<ocio::Lut3DTransform>(t);
      auto handle = new ocio_rs_bridge::Lut3DHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLut3DTransform>(ocio_rs_bridge::RealLut3DTransform{lut3d});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_MATRIX: {
      auto mat = ocio::DynamicPtrCast<ocio::MatrixTransform>(t);
      auto handle = new ocio_rs_bridge::MatrixTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealMatrixTransform>(ocio_rs_bridge::RealMatrixTransform{mat});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_RANGE: {
      auto range = ocio::DynamicPtrCast<ocio::RangeTransform>(t);
      auto handle = new ocio_rs_bridge::RangeTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealRangeTransform>(ocio_rs_bridge::RealRangeTransform{range});
      return handle;
    }
    default: return nullptr;
  }
  END_TRY(nullptr)
#endif
}

void ocio_look_set_inverse_transform(void* look, const void* transform) {
#ifdef OCIO_RS_STUB
  (void)look; (void)transform;
#else
  BEGIN_TRY
  auto lk = static_cast<ocio_rs_bridge::LookHandle*>(look);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLook>(lk->inner);
  if (transform) {
    auto th = static_cast<const ocio_rs_bridge::TransformHandleBase*>(transform);
    real->look->setInverseTransform(th->get_ocio_transform());
  } else {
    real->look->setInverseTransform(nullptr);
  }
  END_TRY_VOID
#endif
}

void ocio_look_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LookHandle*>(handle);
}

// --- Look: aliases ---

int ocio_look_get_num_aliases(void* look) {
#ifdef OCIO_RS_STUB
  (void)look; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    return std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner)->look->getNumAliases();
  } catch (...) { return 0; }
#endif
}

const char* ocio_look_get_alias(void* look, int index) {
#ifdef OCIO_RS_STUB
  (void)look; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    return std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner)->look->getAlias(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_look_add_alias(void* look, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)look; (void)alias;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner)->look->addAlias(alias);
  } catch (...) {}
#endif
}

void ocio_look_remove_alias(void* look, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)look; (void)alias;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner)->look->removeAlias(alias);
  } catch (...) {}
#endif
}

void ocio_look_clear_aliases(void* look) {
#ifdef OCIO_RS_STUB
  (void)look;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner)->look->clearAliases();
  } catch (...) {}
#endif
}

// --- Look: inactive ---

bool ocio_look_is_inactive(void* look) {
#ifdef OCIO_RS_STUB
  (void)look; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    return std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner)->look->isInactive();
  } catch (...) { return false; }
#endif
}

void ocio_look_set_inactive(void* look, bool inactive) {
#ifdef OCIO_RS_STUB
  (void)look; (void)inactive;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookHandle*>(look);
    std::static_pointer_cast<ocio_rs_bridge::RealLook>(h->inner)->look->setIsInactive(inactive);
  } catch (...) {}
#endif
}

// --- ViewTransform ---

void* ocio_view_transform_create(int /*referenceSpace*/) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::ViewTransformHandle{};
#else
  BEGIN_TRY
  auto vt = ocio::ViewTransform::Create(static_cast<ocio::ReferenceSpaceType>(0));
  return new ocio_rs_bridge::ViewTransformHandle{std::make_shared<ocio_rs_bridge::RealViewTransform>(ocio_rs_bridge::RealViewTransform{vt})};
  END_TRY(nullptr)
#endif
}

const char* ocio_view_transform_get_name(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  const char* result = real->transform->getName();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_view_transform_set_name(void* viewTransform, const char* name) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)name;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  real->transform->setName(name);
  END_TRY_VOID
#endif
}

const char* ocio_view_transform_get_src(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  const char* result = real->transform->getSrc();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_view_transform_set_src(void* viewTransform, const char* src) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)src;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  real->transform->setSrc(src);
  END_TRY_VOID
#endif
}

const char* ocio_view_transform_get_display(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  const char* result = real->transform->getDisplay();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_view_transform_set_display(void* viewTransform, const char* display) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)display;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  real->transform->setDisplay(display);
  END_TRY_VOID
#endif
}

const char* ocio_view_transform_get_view(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  const char* result = real->transform->getView();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_view_transform_set_view(void* viewTransform, const char* view) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)view;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  real->transform->setView(view);
  END_TRY_VOID
#endif
}

const char* ocio_view_transform_get_family(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  const char* result = real->transform->getFamily();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_view_transform_set_family(void* viewTransform, const char* family) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)family;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  real->transform->setFamily(family);
  END_TRY_VOID
#endif
}

const char* ocio_view_transform_get_encoding(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  const char* result = real->transform->getEncoding();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_view_transform_set_encoding(void* viewTransform, const char* encoding) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)encoding;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  real->transform->setEncoding(encoding);
  END_TRY_VOID
#endif
}

bool ocio_view_transform_get_looks_bypass(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return false;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  return real->transform->getLooksBypass();
  END_TRY(false)
#endif
}

void ocio_view_transform_set_looks_bypass(void* viewTransform, bool bypass) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)bypass;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  real->transform->setLooksBypass(bypass);
  END_TRY_VOID
#endif
}

const char* ocio_view_transform_get_rule(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  const char* result = real->transform->getRule();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_view_transform_set_rule(void* viewTransform, const char* rule) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)rule;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  real->transform->setRule(rule);
  END_TRY_VOID
#endif
}

void* ocio_view_transform_get_transform(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  ocio::ConstTransformRcPtr t = real->transform->getTransform();
  if (!t) return nullptr;
  // Type dispatch on the inner transform to create appropriate handle
  // Follow the same pattern as ocio_color_space_get_transform
  switch (t->getTransformType()) {
    case ocio::TRANSFORM_TYPE_BUILTIN: {
      auto builtin = ocio::DynamicPtrCast<ocio::BuiltinTransform>(t);
      auto handle = new ocio_rs_bridge::BuiltinTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealBuiltinTransform>(ocio_rs_bridge::RealBuiltinTransform{builtin});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_CDL: {
      auto cdl = ocio::DynamicPtrCast<ocio::CDLTransform>(t);
      auto handle = new ocio_rs_bridge::CDLTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealCDLTransform>(ocio_rs_bridge::RealCDLTransform{cdl});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_EXPONENT: {
      auto exp = ocio::DynamicPtrCast<ocio::ExponentTransform>(t);
      auto handle = new ocio_rs_bridge::ExponentTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealExponentTransform>(ocio_rs_bridge::RealExponentTransform{exp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_FILE: {
      auto file = ocio::DynamicPtrCast<ocio::FileTransform>(t);
      auto handle = new ocio_rs_bridge::FileTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealFileTransform>(ocio_rs_bridge::RealFileTransform{file});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_FIXED_FUNCTION: {
      auto ff = ocio::DynamicPtrCast<ocio::FixedFunctionTransform>(t);
      auto handle = new ocio_rs_bridge::FixedFunctionHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealFixedFunctionTransform>(ocio_rs_bridge::RealFixedFunctionTransform{ff});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GROUP: {
      auto grp = ocio::DynamicPtrCast<ocio::GroupTransform>(t);
      auto handle = new ocio_rs_bridge::GroupTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGroupTransform>(ocio_rs_bridge::RealGroupTransform{grp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG: {
      auto log = ocio::DynamicPtrCast<ocio::LogTransform>(t);
      auto handle = new ocio_rs_bridge::LogTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogTransform>(ocio_rs_bridge::RealLogTransform{log});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LUT1D: {
      auto lut1d = ocio::DynamicPtrCast<ocio::Lut1DTransform>(t);
      auto handle = new ocio_rs_bridge::Lut1DHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLut1DTransform>(ocio_rs_bridge::RealLut1DTransform{lut1d});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LUT3D: {
      auto lut3d = ocio::DynamicPtrCast<ocio::Lut3DTransform>(t);
      auto handle = new ocio_rs_bridge::Lut3DHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLut3DTransform>(ocio_rs_bridge::RealLut3DTransform{lut3d});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_MATRIX: {
      auto mat = ocio::DynamicPtrCast<ocio::MatrixTransform>(t);
      auto handle = new ocio_rs_bridge::MatrixTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealMatrixTransform>(ocio_rs_bridge::RealMatrixTransform{mat});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_RANGE: {
      auto range = ocio::DynamicPtrCast<ocio::RangeTransform>(t);
      auto handle = new ocio_rs_bridge::RangeTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealRangeTransform>(ocio_rs_bridge::RealRangeTransform{range});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_ALLOCATION: {
      auto alloc = ocio::DynamicPtrCast<ocio::AllocationTransform>(t);
      auto handle = new ocio_rs_bridge::AllocationTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealAllocationTransform>(ocio_rs_bridge::RealAllocationTransform{alloc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_COLOR_SPACE: {
      auto cs = ocio::DynamicPtrCast<ocio::ColorSpaceTransform>(t);
      auto handle = new ocio_rs_bridge::ColorSpaceTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceTransform>(ocio_rs_bridge::RealColorSpaceTransform{cs});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_EXPOSURE_CONTRAST: {
      auto ec = ocio::DynamicPtrCast<ocio::ExposureContrastTransform>(t);
      auto handle = new ocio_rs_bridge::ExposureContrastTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealExposureContrastTransform>(ocio_rs_bridge::RealExposureContrastTransform{ec});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_HUE_CURVE: {
      auto ghc = ocio::DynamicPtrCast<ocio::GradingHueCurveTransform>(t);
      auto handle = new ocio_rs_bridge::GradingHueCurveTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingHueCurveTransform>(ocio_rs_bridge::RealGradingHueCurveTransform{ghc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_PRIMARY: {
      auto gp = ocio::DynamicPtrCast<ocio::GradingPrimaryTransform>(t);
      auto handle = new ocio_rs_bridge::GradingPrimaryTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingPrimaryTransform>(ocio_rs_bridge::RealGradingPrimaryTransform{gp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_RGB_CURVE: {
      auto gc = ocio::DynamicPtrCast<ocio::GradingRGBCurveTransform>(t);
      auto handle = new ocio_rs_bridge::GradingRGBCurveTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingRGBCurveTransform>(ocio_rs_bridge::RealGradingRGBCurveTransform{gc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_TONE: {
      auto gt = ocio::DynamicPtrCast<ocio::GradingToneTransform>(t);
      auto handle = new ocio_rs_bridge::GradingToneTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingToneTransform>(ocio_rs_bridge::RealGradingToneTransform{gt});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG_AFFINE: {
      auto la = ocio::DynamicPtrCast<ocio::LogAffineTransform>(t);
      auto handle = new ocio_rs_bridge::LogAffineTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogAffineTransform>(ocio_rs_bridge::RealLogAffineTransform{la});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG_CAMERA: {
      auto lc = ocio::DynamicPtrCast<ocio::LogCameraTransform>(t);
      auto handle = new ocio_rs_bridge::LogCameraTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogCameraTransform>(ocio_rs_bridge::RealLogCameraTransform{lc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOOK: {
      auto lk = ocio::DynamicPtrCast<ocio::LookTransform>(t);
      auto handle = new ocio_rs_bridge::LookTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLookTransform>(ocio_rs_bridge::RealLookTransform{lk});
      return handle;
    }
    default: return nullptr;
  }
  END_TRY(nullptr)
#endif
}

void ocio_view_transform_set_transform(void* viewTransform, const void* transform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)transform;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  if (transform) {
    auto th = static_cast<const ocio_rs_bridge::TransformHandleBase*>(transform);
    real->transform->setTransform(th->get_ocio_transform());
  } else {
    real->transform->setTransform(nullptr);
  }
  END_TRY_VOID
#endif
}

void* ocio_view_transform_get_inverse_transform(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  ocio::ConstTransformRcPtr t = real->transform->getInverseTransform();
  if (!t) return nullptr;
  switch (t->getTransformType()) {
    case ocio::TRANSFORM_TYPE_BUILTIN: {
      auto builtin = ocio::DynamicPtrCast<ocio::BuiltinTransform>(t);
      auto handle = new ocio_rs_bridge::BuiltinTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealBuiltinTransform>(ocio_rs_bridge::RealBuiltinTransform{builtin});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_CDL: {
      auto cdl = ocio::DynamicPtrCast<ocio::CDLTransform>(t);
      auto handle = new ocio_rs_bridge::CDLTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealCDLTransform>(ocio_rs_bridge::RealCDLTransform{cdl});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_EXPONENT: {
      auto exp = ocio::DynamicPtrCast<ocio::ExponentTransform>(t);
      auto handle = new ocio_rs_bridge::ExponentTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealExponentTransform>(ocio_rs_bridge::RealExponentTransform{exp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_FILE: {
      auto file = ocio::DynamicPtrCast<ocio::FileTransform>(t);
      auto handle = new ocio_rs_bridge::FileTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealFileTransform>(ocio_rs_bridge::RealFileTransform{file});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_FIXED_FUNCTION: {
      auto ff = ocio::DynamicPtrCast<ocio::FixedFunctionTransform>(t);
      auto handle = new ocio_rs_bridge::FixedFunctionHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealFixedFunctionTransform>(ocio_rs_bridge::RealFixedFunctionTransform{ff});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GROUP: {
      auto grp = ocio::DynamicPtrCast<ocio::GroupTransform>(t);
      auto handle = new ocio_rs_bridge::GroupTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGroupTransform>(ocio_rs_bridge::RealGroupTransform{grp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG: {
      auto log = ocio::DynamicPtrCast<ocio::LogTransform>(t);
      auto handle = new ocio_rs_bridge::LogTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogTransform>(ocio_rs_bridge::RealLogTransform{log});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LUT1D: {
      auto lut1d = ocio::DynamicPtrCast<ocio::Lut1DTransform>(t);
      auto handle = new ocio_rs_bridge::Lut1DHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLut1DTransform>(ocio_rs_bridge::RealLut1DTransform{lut1d});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LUT3D: {
      auto lut3d = ocio::DynamicPtrCast<ocio::Lut3DTransform>(t);
      auto handle = new ocio_rs_bridge::Lut3DHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLut3DTransform>(ocio_rs_bridge::RealLut3DTransform{lut3d});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_MATRIX: {
      auto mat = ocio::DynamicPtrCast<ocio::MatrixTransform>(t);
      auto handle = new ocio_rs_bridge::MatrixTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealMatrixTransform>(ocio_rs_bridge::RealMatrixTransform{mat});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_RANGE: {
      auto range = ocio::DynamicPtrCast<ocio::RangeTransform>(t);
      auto handle = new ocio_rs_bridge::RangeTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealRangeTransform>(ocio_rs_bridge::RealRangeTransform{range});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_ALLOCATION: {
      auto alloc = ocio::DynamicPtrCast<ocio::AllocationTransform>(t);
      auto handle = new ocio_rs_bridge::AllocationTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealAllocationTransform>(ocio_rs_bridge::RealAllocationTransform{alloc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_COLOR_SPACE: {
      auto cs = ocio::DynamicPtrCast<ocio::ColorSpaceTransform>(t);
      auto handle = new ocio_rs_bridge::ColorSpaceTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceTransform>(ocio_rs_bridge::RealColorSpaceTransform{cs});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_EXPOSURE_CONTRAST: {
      auto ec = ocio::DynamicPtrCast<ocio::ExposureContrastTransform>(t);
      auto handle = new ocio_rs_bridge::ExposureContrastTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealExposureContrastTransform>(ocio_rs_bridge::RealExposureContrastTransform{ec});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_HUE_CURVE: {
      auto ghc = ocio::DynamicPtrCast<ocio::GradingHueCurveTransform>(t);
      auto handle = new ocio_rs_bridge::GradingHueCurveTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingHueCurveTransform>(ocio_rs_bridge::RealGradingHueCurveTransform{ghc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_PRIMARY: {
      auto gp = ocio::DynamicPtrCast<ocio::GradingPrimaryTransform>(t);
      auto handle = new ocio_rs_bridge::GradingPrimaryTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingPrimaryTransform>(ocio_rs_bridge::RealGradingPrimaryTransform{gp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_RGB_CURVE: {
      auto gc = ocio::DynamicPtrCast<ocio::GradingRGBCurveTransform>(t);
      auto handle = new ocio_rs_bridge::GradingRGBCurveTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingRGBCurveTransform>(ocio_rs_bridge::RealGradingRGBCurveTransform{gc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_TONE: {
      auto gt = ocio::DynamicPtrCast<ocio::GradingToneTransform>(t);
      auto handle = new ocio_rs_bridge::GradingToneTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingToneTransform>(ocio_rs_bridge::RealGradingToneTransform{gt});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG_AFFINE: {
      auto la = ocio::DynamicPtrCast<ocio::LogAffineTransform>(t);
      auto handle = new ocio_rs_bridge::LogAffineTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogAffineTransform>(ocio_rs_bridge::RealLogAffineTransform{la});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG_CAMERA: {
      auto lc = ocio::DynamicPtrCast<ocio::LogCameraTransform>(t);
      auto handle = new ocio_rs_bridge::LogCameraTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogCameraTransform>(ocio_rs_bridge::RealLogCameraTransform{lc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOOK: {
      auto lk = ocio::DynamicPtrCast<ocio::LookTransform>(t);
      auto handle = new ocio_rs_bridge::LookTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLookTransform>(ocio_rs_bridge::RealLookTransform{lk});
      return handle;
    }
    default: return nullptr;
  }
  END_TRY(nullptr)
#endif
}

void ocio_view_transform_set_inverse_transform(void* viewTransform, const void* transform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)transform;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  if (transform) {
    auto th = static_cast<const ocio_rs_bridge::TransformHandleBase*>(transform);
    real->transform->setInverseTransform(th->get_ocio_transform());
  } else {
    real->transform->setInverseTransform(nullptr);
  }
  END_TRY_VOID
#endif
}

int ocio_view_transform_get_direction(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return 0;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_view_transform_set_direction(void* viewTransform, int direction) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)direction;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

int ocio_view_transform_get_reference_space_type(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
  return 0;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  return static_cast<int>(real->transform->getReferenceSpaceType());
  END_TRY(0)
#endif
}

void ocio_view_transform_set_reference_space_type(void* viewTransform, int refType) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)refType;
#else
  BEGIN_TRY
  auto vt = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(vt->inner);
  real->transform->setReferenceSpaceType(static_cast<ocio::ReferenceSpaceType>(refType));
  END_TRY_VOID
#endif
}

void ocio_view_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ViewTransformHandle*>(handle);
}

// --- ViewTransform: aliases ---

int ocio_view_transform_get_num_aliases(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    return std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->getNumAliases();
  } catch (...) { return 0; }
#endif
}

const char* ocio_view_transform_get_alias(void* viewTransform, int index) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    return std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->getAlias(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_view_transform_add_alias(void* viewTransform, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)alias;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->addAlias(alias);
  } catch (...) {}
#endif
}

void ocio_view_transform_remove_alias(void* viewTransform, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)alias;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->removeAlias(alias);
  } catch (...) {}
#endif
}

void ocio_view_transform_clear_aliases(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->clearAliases();
  } catch (...) {}
#endif
}

// --- ViewTransform: inactive, category, description ---

bool ocio_view_transform_is_inactive(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    return std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->isInactive();
  } catch (...) { return false; }
#endif
}

void ocio_view_transform_set_inactive(void* viewTransform, bool inactive) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)inactive;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->setIsInactive(inactive);
  } catch (...) {}
#endif
}

const char* ocio_view_transform_get_category(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    return std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->getCategory();
  } catch (...) { return nullptr; }
#endif
}

void ocio_view_transform_set_category(void* viewTransform, const char* category) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)category;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->setCategory(category);
  } catch (...) {}
#endif
}

const char* ocio_view_transform_get_description(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    return std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->getDescription();
  } catch (...) { return nullptr; }
#endif
}

void ocio_view_transform_set_description(void* viewTransform, const char* description) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; (void)description;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->setDescription(description);
  } catch (...) {}
#endif
}

// --- ViewTransform: editable copy ---

void* ocio_view_transform_create_editable_copy(void* viewTransform) {
#ifdef OCIO_RS_STUB
  (void)viewTransform; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ViewTransformHandle*>(viewTransform);
    auto vt = std::static_pointer_cast<ocio_rs_bridge::RealViewTransform>(h->inner)->transform->createEditableCopy();
    auto handle = std::make_unique<ocio_rs_bridge::ViewTransformHandle>();
    handle->inner = std::make_shared<ocio_rs_bridge::RealViewTransform>(ocio_rs_bridge::RealViewTransform{vt});
    return handle.release();
  } catch (...) { return nullptr; }
#endif
}

// --- NamedTransform ---

void* ocio_named_transform_create(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::NamedTransformHandle{};
#else
  BEGIN_TRY
  auto nt = ocio::NamedTransform::Create();
  return new ocio_rs_bridge::NamedTransformHandle{std::make_shared<ocio_rs_bridge::RealNamedTransform>(ocio_rs_bridge::RealNamedTransform{nt})};
  END_TRY(nullptr)
#endif
}

void* ocio_named_transform_create_editable_copy(void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)namedTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  auto copy = real->transform->createEditableCopy();
  return new ocio_rs_bridge::NamedTransformHandle{std::make_shared<ocio_rs_bridge::RealNamedTransform>(ocio_rs_bridge::RealNamedTransform{copy})};
  END_TRY(nullptr)
#endif
}

const char* ocio_named_transform_get_name(void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)namedTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  const char* result = real->transform->getName();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_named_transform_set_name(void* namedTransform, const char* name) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; (void)name;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  real->transform->setName(name);
  END_TRY_VOID
#endif
}

const char* ocio_named_transform_get_family(void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)namedTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  const char* result = real->transform->getFamily();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_named_transform_set_family(void* namedTransform, const char* family) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; (void)family;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  real->transform->setFamily(family);
  END_TRY_VOID
#endif
}

const char* ocio_named_transform_get_description(void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)namedTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  const char* result = real->transform->getDescription();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_named_transform_set_description(void* namedTransform, const char* description) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; (void)description;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  real->transform->setDescription(description);
  END_TRY_VOID
#endif
}

const char* ocio_named_transform_get_encoding(void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)namedTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  const char* result = real->transform->getEncoding();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_named_transform_set_encoding(void* namedTransform, const char* encoding) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; (void)encoding;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  real->transform->setEncoding(encoding);
  END_TRY_VOID
#endif
}

const char* ocio_named_transform_get_cache_id(void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)namedTransform;
  return nullptr;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  const char* result = real->transform->getCacheID();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void* ocio_named_transform_get_transform(void* namedTransform, int direction) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; (void)direction;
  return nullptr;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  ocio::ConstTransformRcPtr t = real->transform->getTransform(static_cast<ocio::TransformDirection>(direction));
  if (!t) return nullptr;
  switch (t->getTransformType()) {
    case ocio::TRANSFORM_TYPE_BUILTIN: {
      auto builtin = ocio::DynamicPtrCast<ocio::BuiltinTransform>(t);
      auto handle = new ocio_rs_bridge::BuiltinTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealBuiltinTransform>(ocio_rs_bridge::RealBuiltinTransform{builtin});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_CDL: {
      auto cdl = ocio::DynamicPtrCast<ocio::CDLTransform>(t);
      auto handle = new ocio_rs_bridge::CDLTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealCDLTransform>(ocio_rs_bridge::RealCDLTransform{cdl});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_EXPONENT: {
      auto exp = ocio::DynamicPtrCast<ocio::ExponentTransform>(t);
      auto handle = new ocio_rs_bridge::ExponentTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealExponentTransform>(ocio_rs_bridge::RealExponentTransform{exp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_FILE: {
      auto file = ocio::DynamicPtrCast<ocio::FileTransform>(t);
      auto handle = new ocio_rs_bridge::FileTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealFileTransform>(ocio_rs_bridge::RealFileTransform{file});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_FIXED_FUNCTION: {
      auto ff = ocio::DynamicPtrCast<ocio::FixedFunctionTransform>(t);
      auto handle = new ocio_rs_bridge::FixedFunctionHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealFixedFunctionTransform>(ocio_rs_bridge::RealFixedFunctionTransform{ff});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GROUP: {
      auto grp = ocio::DynamicPtrCast<ocio::GroupTransform>(t);
      auto handle = new ocio_rs_bridge::GroupTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGroupTransform>(ocio_rs_bridge::RealGroupTransform{grp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG: {
      auto log = ocio::DynamicPtrCast<ocio::LogTransform>(t);
      auto handle = new ocio_rs_bridge::LogTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogTransform>(ocio_rs_bridge::RealLogTransform{log});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LUT1D: {
      auto lut1d = ocio::DynamicPtrCast<ocio::Lut1DTransform>(t);
      auto handle = new ocio_rs_bridge::Lut1DHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLut1DTransform>(ocio_rs_bridge::RealLut1DTransform{lut1d});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LUT3D: {
      auto lut3d = ocio::DynamicPtrCast<ocio::Lut3DTransform>(t);
      auto handle = new ocio_rs_bridge::Lut3DHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLut3DTransform>(ocio_rs_bridge::RealLut3DTransform{lut3d});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_MATRIX: {
      auto mat = ocio::DynamicPtrCast<ocio::MatrixTransform>(t);
      auto handle = new ocio_rs_bridge::MatrixTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealMatrixTransform>(ocio_rs_bridge::RealMatrixTransform{mat});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_RANGE: {
      auto range = ocio::DynamicPtrCast<ocio::RangeTransform>(t);
      auto handle = new ocio_rs_bridge::RangeTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealRangeTransform>(ocio_rs_bridge::RealRangeTransform{range});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_ALLOCATION: {
      auto alloc = ocio::DynamicPtrCast<ocio::AllocationTransform>(t);
      auto handle = new ocio_rs_bridge::AllocationTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealAllocationTransform>(ocio_rs_bridge::RealAllocationTransform{alloc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_COLOR_SPACE: {
      auto cs = ocio::DynamicPtrCast<ocio::ColorSpaceTransform>(t);
      auto handle = new ocio_rs_bridge::ColorSpaceTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceTransform>(ocio_rs_bridge::RealColorSpaceTransform{cs});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_EXPOSURE_CONTRAST: {
      auto ec = ocio::DynamicPtrCast<ocio::ExposureContrastTransform>(t);
      auto handle = new ocio_rs_bridge::ExposureContrastTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealExposureContrastTransform>(ocio_rs_bridge::RealExposureContrastTransform{ec});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_HUE_CURVE: {
      auto ghc = ocio::DynamicPtrCast<ocio::GradingHueCurveTransform>(t);
      auto handle = new ocio_rs_bridge::GradingHueCurveTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingHueCurveTransform>(ocio_rs_bridge::RealGradingHueCurveTransform{ghc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_PRIMARY: {
      auto gp = ocio::DynamicPtrCast<ocio::GradingPrimaryTransform>(t);
      auto handle = new ocio_rs_bridge::GradingPrimaryTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingPrimaryTransform>(ocio_rs_bridge::RealGradingPrimaryTransform{gp});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_RGB_CURVE: {
      auto gc = ocio::DynamicPtrCast<ocio::GradingRGBCurveTransform>(t);
      auto handle = new ocio_rs_bridge::GradingRGBCurveTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingRGBCurveTransform>(ocio_rs_bridge::RealGradingRGBCurveTransform{gc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_GRADING_TONE: {
      auto gt = ocio::DynamicPtrCast<ocio::GradingToneTransform>(t);
      auto handle = new ocio_rs_bridge::GradingToneTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealGradingToneTransform>(ocio_rs_bridge::RealGradingToneTransform{gt});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG_AFFINE: {
      auto la = ocio::DynamicPtrCast<ocio::LogAffineTransform>(t);
      auto handle = new ocio_rs_bridge::LogAffineTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogAffineTransform>(ocio_rs_bridge::RealLogAffineTransform{la});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOG_CAMERA: {
      auto lc = ocio::DynamicPtrCast<ocio::LogCameraTransform>(t);
      auto handle = new ocio_rs_bridge::LogCameraTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLogCameraTransform>(ocio_rs_bridge::RealLogCameraTransform{lc});
      return handle;
    }
    case ocio::TRANSFORM_TYPE_LOOK: {
      auto lk = ocio::DynamicPtrCast<ocio::LookTransform>(t);
      auto handle = new ocio_rs_bridge::LookTransformHandle{};
      handle->inner = std::make_shared<ocio_rs_bridge::RealLookTransform>(ocio_rs_bridge::RealLookTransform{lk});
      return handle;
    }
    default: return nullptr;
  }
  END_TRY(nullptr)
#endif
}

void ocio_named_transform_set_transform(void* namedTransform, const void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto nt = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(nt->inner);
  if (transform) {
    auto th = static_cast<const ocio_rs_bridge::TransformHandleBase*>(transform);
    real->transform->setTransform(th->get_ocio_transform(), static_cast<ocio::TransformDirection>(direction));
  } else {
    real->transform->setTransform(nullptr, static_cast<ocio::TransformDirection>(direction));
  }
  END_TRY_VOID
#endif
}

void ocio_named_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::NamedTransformHandle*>(handle);
}

// --- NamedTransform: aliases ---

int ocio_named_transform_get_num_aliases(void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    return std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(h->inner)->transform->getNumAliases();
  } catch (...) { return 0; }
#endif
}

const char* ocio_named_transform_get_alias(void* namedTransform, int index) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    return std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(h->inner)->transform->getAlias(index);
  } catch (...) { return nullptr; }
#endif
}

void ocio_named_transform_add_alias(void* namedTransform, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; (void)alias;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(h->inner)->transform->addAlias(alias);
  } catch (...) {}
#endif
}

void ocio_named_transform_remove_alias(void* namedTransform, const char* alias) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; (void)alias;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(h->inner)->transform->removeAlias(alias);
  } catch (...) {}
#endif
}

void ocio_named_transform_clear_aliases(void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)namedTransform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(h->inner)->transform->clearAliases();
  } catch (...) {}
#endif
}

// --- NamedTransform: inactive & category ---

bool ocio_named_transform_is_inactive(void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    return std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(h->inner)->transform->isInactive();
  } catch (...) { return false; }
#endif
}

void ocio_named_transform_set_inactive(void* namedTransform, bool inactive) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; (void)inactive;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(h->inner)->transform->setIsInactive(inactive);
  } catch (...) {}
#endif
}

const char* ocio_named_transform_get_category(void* namedTransform) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    return std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(h->inner)->transform->getCategory();
  } catch (...) { return nullptr; }
#endif
}

void ocio_named_transform_set_category(void* namedTransform, const char* category) {
#ifdef OCIO_RS_STUB
  (void)namedTransform; (void)category;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::NamedTransformHandle*>(namedTransform);
    std::static_pointer_cast<ocio_rs_bridge::RealNamedTransform>(h->inner)->transform->setCategory(category);
  } catch (...) {}
#endif
}

// --- DynamicProperty ---

void* ocio_processor_get_dynamic_property(void* processor, int propertyType) {
#ifdef OCIO_RS_STUB
  (void)processor; (void)propertyType;
  return nullptr;
#else
  BEGIN_TRY
  auto proc = static_cast<ocio_rs_bridge::ProcessorHandle*>(processor);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealProcessor>(proc->inner);
  auto dp = real->processor->getDynamicProperty(static_cast<ocio::DynamicPropertyType>(propertyType));
  if (!dp) return nullptr;
  return new ocio_rs_bridge::DynamicPropertyHandle{std::make_shared<ocio_rs_bridge::RealDynamicProperty>(ocio_rs_bridge::RealDynamicProperty{dp})};
  END_TRY(nullptr)
#endif
}

int ocio_dynamic_property_get_type(void* prop) {
#ifdef OCIO_RS_STUB
  (void)prop;
  return -1;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  return static_cast<int>(real->prop->getType());
  END_TRY(-1)
#endif
}

double ocio_dynamic_property_double_get_value(void* prop) {
#ifdef OCIO_RS_STUB
  (void)prop;
  return 0.0;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto dbl = ocio::DynamicPtrCast<ocio::DynamicPropertyDouble>(real->prop);
  if (!dbl) return 0.0;
  return dbl->getValue();
  END_TRY(0.0)
#endif
}

void ocio_dynamic_property_double_set_value(void* prop, double value) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)value;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto dbl = ocio::DynamicPtrCast<ocio::DynamicPropertyDouble>(real->prop);
  if (dbl) dbl->setValue(value);
  END_TRY_VOID
#endif
}

void ocio_dynamic_property_grading_primary_get_value(void* prop, double* values) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)values;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingPrimary>(real->prop);
  if (typed && values) {
    auto& g = typed->getValue();
    int off = 0;
    auto write_rgbm = [&](const ocio::GradingRGBM& c) {
      values[off++] = c.m_red; values[off++] = c.m_green;
      values[off++] = c.m_blue; values[off++] = c.m_master;
    };
    write_rgbm(g.m_brightness); write_rgbm(g.m_contrast);
    write_rgbm(g.m_gamma); write_rgbm(g.m_offset);
    write_rgbm(g.m_exposure); write_rgbm(g.m_lift);
    write_rgbm(g.m_gain);
    values[off++] = g.m_saturation;
    values[off++] = g.m_pivot;
    values[off++] = g.m_pivotBlack;
    values[off++] = g.m_pivotWhite;
    values[off++] = g.m_clampBlack;
    values[off++] = g.m_clampWhite;
  }
  END_TRY_VOID
#endif
}

void ocio_dynamic_property_grading_primary_set_value(void* prop, const double* values) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)values;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingPrimary>(real->prop);
  if (typed && values) {
    auto g = typed->getValue();
    int off = 0;
    auto read_rgbm = [&](ocio::GradingRGBM& c) {
      c.m_red = values[off++]; c.m_green = values[off++];
      c.m_blue = values[off++]; c.m_master = values[off++];
    };
    read_rgbm(g.m_brightness); read_rgbm(g.m_contrast);
    read_rgbm(g.m_gamma); read_rgbm(g.m_offset);
    read_rgbm(g.m_exposure); read_rgbm(g.m_lift);
    read_rgbm(g.m_gain);
    g.m_saturation = values[off++];
    g.m_pivot = values[off++];
    g.m_pivotBlack = values[off++];
    g.m_pivotWhite = values[off++];
    g.m_clampBlack = values[off++];
    g.m_clampWhite = values[off++];
    typed->setValue(g);
  }
  END_TRY_VOID
#endif
}

void ocio_dynamic_property_grading_tone_get_value(void* prop, double* values) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)values;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingTone>(real->prop);
  if (typed && values) {
    auto& g = typed->getValue();
    int off = 0;
    auto write_rgbmsw = [&](const ocio::GradingRGBMSW& c) {
      values[off++] = c.m_red; values[off++] = c.m_green;
      values[off++] = c.m_blue; values[off++] = c.m_master;
      values[off++] = c.m_start; values[off++] = c.m_width;
    };
    write_rgbmsw(g.m_blacks); write_rgbmsw(g.m_shadows);
    write_rgbmsw(g.m_midtones); write_rgbmsw(g.m_highlights);
    write_rgbmsw(g.m_whites);
    values[off++] = g.m_scontrast;
  }
  END_TRY_VOID
#endif
}

void ocio_dynamic_property_grading_tone_set_value(void* prop, const double* values) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)values;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingTone>(real->prop);
  if (typed && values) {
    auto g = typed->getValue();
    int off = 0;
    auto read_rgbmsw = [&](ocio::GradingRGBMSW& c) {
      c.m_red = values[off++]; c.m_green = values[off++];
      c.m_blue = values[off++]; c.m_master = values[off++];
      c.m_start = values[off++]; c.m_width = values[off++];
    };
    read_rgbmsw(g.m_blacks); read_rgbmsw(g.m_shadows);
    read_rgbmsw(g.m_midtones); read_rgbmsw(g.m_highlights);
    read_rgbmsw(g.m_whites);
    g.m_scontrast = values[off++];
    typed->setValue(g);
  }
  END_TRY_VOID
#endif
}

int ocio_dynamic_property_grading_rgb_curve_get_num_control_points(void* prop, int curveType) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType;
  return 0;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingRGBCurve>(real->prop);
  if (!typed) return 0;
  return typed->getNumControlPoints(static_cast<ocio::RGBCurveType>(curveType));
  END_TRY(0)
#endif
}

void ocio_dynamic_property_grading_rgb_curve_set_num_control_points(void* prop, int curveType, int num) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType; (void)num;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingRGBCurve>(real->prop);
  if (typed) typed->setNumControlPoints(static_cast<ocio::RGBCurveType>(curveType), num);
  END_TRY_VOID
#endif
}

void ocio_dynamic_property_grading_rgb_curve_get_control_point(void* prop, int curveType, int index, float* x, float* y) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType; (void)index;
  if (x) *x = 0.0f;
  if (y) *y = 0.0f;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingRGBCurve>(real->prop);
  if (typed) {
    typed->getControlPoint(static_cast<ocio::RGBCurveType>(curveType), index, *x, *y);
  }
  END_TRY_VOID
#endif
}

void ocio_dynamic_property_grading_rgb_curve_set_control_point(void* prop, int curveType, int index, float x, float y) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType; (void)index; (void)x; (void)y;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingRGBCurve>(real->prop);
  if (typed) typed->setControlPoint(static_cast<ocio::RGBCurveType>(curveType), index, x, y);
  END_TRY_VOID
#endif
}

float ocio_dynamic_property_grading_rgb_curve_get_slope(void* prop, int curveType, int index) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType; (void)index;
  return 0.0f;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingRGBCurve>(real->prop);
  if (!typed) return 0.0f;
  return typed->getSlope(static_cast<ocio::RGBCurveType>(curveType), index);
  END_TRY(0.0f)
#endif
}

void ocio_dynamic_property_grading_rgb_curve_set_slope(void* prop, int curveType, int index, float slope) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType; (void)index; (void)slope;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingRGBCurve>(real->prop);
  if (typed) typed->setSlope(static_cast<ocio::RGBCurveType>(curveType), index, slope);
  END_TRY_VOID
#endif
}

bool ocio_dynamic_property_grading_rgb_curve_slopes_are_default(void* prop, int curveType) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType;
  return true;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingRGBCurve>(real->prop);
  if (!typed) return true;
  return typed->slopesAreDefault(static_cast<ocio::RGBCurveType>(curveType));
  END_TRY(true)
#endif
}

int ocio_dynamic_property_grading_hue_curve_get_num_control_points(void* prop, int curveType) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType;
  return 0;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingHueCurve>(real->prop);
  if (!typed) return 0;
  return typed->getNumControlPoints(static_cast<ocio::RGBCurveType>(curveType));
  END_TRY(0)
#endif
}

void ocio_dynamic_property_grading_hue_curve_set_num_control_points(void* prop, int curveType, int num) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType; (void)num;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingHueCurve>(real->prop);
  if (typed) typed->setNumControlPoints(static_cast<ocio::RGBCurveType>(curveType), num);
  END_TRY_VOID
#endif
}

void ocio_dynamic_property_grading_hue_curve_get_control_point(void* prop, int curveType, int index, float* x, float* y) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType; (void)index;
  if (x) *x = 0.0f;
  if (y) *y = 0.0f;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingHueCurve>(real->prop);
  if (typed) {
    typed->getControlPoint(static_cast<ocio::RGBCurveType>(curveType), index, *x, *y);
  }
  END_TRY_VOID
#endif
}

void ocio_dynamic_property_grading_hue_curve_set_control_point(void* prop, int curveType, int index, float x, float y) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType; (void)index; (void)x; (void)y;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingHueCurve>(real->prop);
  if (typed) typed->setControlPoint(static_cast<ocio::RGBCurveType>(curveType), index, x, y);
  END_TRY_VOID
#endif
}

float ocio_dynamic_property_grading_hue_curve_get_slope(void* prop, int curveType, int index) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType; (void)index;
  return 0.0f;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingHueCurve>(real->prop);
  if (!typed) return 0.0f;
  return typed->getSlope(static_cast<ocio::RGBCurveType>(curveType), index);
  END_TRY(0.0f)
#endif
}

void ocio_dynamic_property_grading_hue_curve_set_slope(void* prop, int curveType, int index, float slope) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType; (void)index; (void)slope;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingHueCurve>(real->prop);
  if (typed) typed->setSlope(static_cast<ocio::RGBCurveType>(curveType), index, slope);
  END_TRY_VOID
#endif
}

bool ocio_dynamic_property_grading_hue_curve_slopes_are_default(void* prop, int curveType) {
#ifdef OCIO_RS_STUB
  (void)prop; (void)curveType;
  return true;
#else
  BEGIN_TRY
  auto dp = static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(prop);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDynamicProperty>(dp->inner);
  auto typed = ocio::DynamicPtrCast<ocio::DynamicPropertyGradingHueCurve>(real->prop);
  if (!typed) return true;
  return typed->slopesAreDefault(static_cast<ocio::RGBCurveType>(curveType));
  END_TRY(true)
#endif
}

void ocio_dynamic_property_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::DynamicPropertyHandle*>(handle);
}

// --- ExposureContrastTransform ---

void* ocio_exposure_contrast_transform_create(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::ExposureContrastTransformHandle{};
#else
  BEGIN_TRY
  auto t = ocio::ExposureContrastTransform::Create();
  auto handle = new ocio_rs_bridge::ExposureContrastTransformHandle{};
  handle->inner = std::make_shared<ocio_rs_bridge::RealExposureContrastTransform>(ocio_rs_bridge::RealExposureContrastTransform{t});
  return handle;
  END_TRY(nullptr)
#endif
}

double ocio_exposure_contrast_transform_get_exposure(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0.0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  return real->transform->getExposure();
  END_TRY(0.0)
#endif
}

void ocio_exposure_contrast_transform_set_exposure(void* transform, double exposure) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)exposure;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->setExposure(exposure);
  END_TRY_VOID
#endif
}

double ocio_exposure_contrast_transform_get_contrast(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 1.0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  return real->transform->getContrast();
  END_TRY(1.0)
#endif
}

void ocio_exposure_contrast_transform_set_contrast(void* transform, double contrast) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)contrast;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->setContrast(contrast);
  END_TRY_VOID
#endif
}

double ocio_exposure_contrast_transform_get_gamma(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 1.0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  return real->transform->getGamma();
  END_TRY(1.0)
#endif
}

void ocio_exposure_contrast_transform_set_gamma(void* transform, double gamma) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)gamma;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->setGamma(gamma);
  END_TRY_VOID
#endif
}

double ocio_exposure_contrast_transform_get_pivot(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0.18;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  return real->transform->getPivot();
  END_TRY(0.18)
#endif
}

void ocio_exposure_contrast_transform_set_pivot(void* transform, double pivot) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)pivot;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->setPivot(pivot);
  END_TRY_VOID
#endif
}

int ocio_exposure_contrast_transform_get_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  return static_cast<int>(real->transform->getStyle());
  END_TRY(0)
#endif
}

void ocio_exposure_contrast_transform_set_style(void* transform, int style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->setStyle(static_cast<ocio::ExposureContrastStyle>(style));
  END_TRY_VOID
#endif
}

bool ocio_exposure_contrast_transform_is_exposure_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  return real->transform->isExposureDynamic();
  END_TRY(false)
#endif
}

void ocio_exposure_contrast_transform_make_exposure_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->makeExposureDynamic();
  END_TRY_VOID
#endif
}

bool ocio_exposure_contrast_transform_is_contrast_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  return real->transform->isContrastDynamic();
  END_TRY(false)
#endif
}

void ocio_exposure_contrast_transform_make_contrast_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->makeContrastDynamic();
  END_TRY_VOID
#endif
}

bool ocio_exposure_contrast_transform_is_gamma_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  return real->transform->isGammaDynamic();
  END_TRY(false)
#endif
}

void ocio_exposure_contrast_transform_make_gamma_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->makeGammaDynamic();
  END_TRY_VOID
#endif
}

int ocio_exposure_contrast_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_exposure_contrast_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

void ocio_exposure_contrast_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(handle);
}

// --- ColorSpaceTransform ---

void* ocio_color_space_transform_create(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::ColorSpaceTransformHandle{};
#else
  BEGIN_TRY
  auto t = ocio::ColorSpaceTransform::Create();
  auto handle = new ocio_rs_bridge::ColorSpaceTransformHandle{};
  handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceTransform>(ocio_rs_bridge::RealColorSpaceTransform{t});
  return handle;
  END_TRY(nullptr)
#endif
}

const char* ocio_color_space_transform_get_src(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return nullptr;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceTransform>(t->inner);
  const char* result = real->transform->getSrc();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_color_space_transform_set_src(void* transform, const char* src) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)src;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceTransform>(t->inner);
  real->transform->setSrc(src);
  END_TRY_VOID
#endif
}

const char* ocio_color_space_transform_get_dst(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return nullptr;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceTransform>(t->inner);
  const char* result = real->transform->getDst();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_color_space_transform_set_dst(void* transform, const char* dst) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)dst;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceTransform>(t->inner);
  real->transform->setDst(dst);
  END_TRY_VOID
#endif
}

bool ocio_color_space_transform_get_data_bypass(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceTransform>(t->inner);
  return real->transform->getDataBypass();
  END_TRY(false)
#endif
}

void ocio_color_space_transform_set_data_bypass(void* transform, bool bypass) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)bypass;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceTransform>(t->inner);
  real->transform->setDataBypass(bypass);
  END_TRY_VOID
#endif
}

int ocio_color_space_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceTransform>(t->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_color_space_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceTransform>(t->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

void ocio_color_space_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ColorSpaceTransformHandle*>(handle);
}

// --- LookTransform ---

void* ocio_look_transform_create(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::LookTransformHandle{};
#else
  BEGIN_TRY
  auto t = ocio::LookTransform::Create();
  auto handle = new ocio_rs_bridge::LookTransformHandle{};
  handle->inner = std::make_shared<ocio_rs_bridge::RealLookTransform>(ocio_rs_bridge::RealLookTransform{t});
  return handle;
  END_TRY(nullptr)
#endif
}

const char* ocio_look_transform_get_src(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return nullptr;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LookTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(t->inner);
  const char* result = real->transform->getSrc();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_look_transform_set_src(void* transform, const char* src) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)src;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LookTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(t->inner);
  real->transform->setSrc(src);
  END_TRY_VOID
#endif
}

const char* ocio_look_transform_get_dst(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return nullptr;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LookTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(t->inner);
  const char* result = real->transform->getDst();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_look_transform_set_dst(void* transform, const char* dst) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)dst;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LookTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(t->inner);
  real->transform->setDst(dst);
  END_TRY_VOID
#endif
}

const char* ocio_look_transform_get_looks(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return nullptr;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LookTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(t->inner);
  const char* result = real->transform->getLooks();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_look_transform_set_looks(void* transform, const char* looks) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)looks;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LookTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(t->inner);
  real->transform->setLooks(looks);
  END_TRY_VOID
#endif
}

int ocio_look_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LookTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(t->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_look_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LookTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(t->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

void ocio_look_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LookTransformHandle*>(handle);
}

// --- GradingPrimaryTransform ---

void* ocio_grading_primary_transform_create(int style) {
#ifdef OCIO_RS_STUB
  (void)style;
  return new ocio_rs_bridge::GradingPrimaryTransformHandle{};
#else
  BEGIN_TRY
  auto r = std::make_shared<ocio_rs_bridge::RealGradingPrimaryTransform>();
  r->transform = ocio::GradingPrimaryTransform::Create(static_cast<ocio::GradingStyle>(style));
  auto hdl = std::make_unique<ocio_rs_bridge::GradingPrimaryTransformHandle>();
  hdl->inner = r;
  return hdl.release();
  END_TRY(nullptr)
#endif
}

int ocio_grading_primary_transform_get_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryTransform>(t->inner);
  return static_cast<int>(real->transform->getStyle());
  END_TRY(0)
#endif
}

void ocio_grading_primary_transform_set_style(void* transform, int style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryTransform>(t->inner);
  real->transform->setStyle(static_cast<ocio::GradingStyle>(style));
  END_TRY_VOID
#endif
}

void ocio_grading_primary_transform_get_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryTransform>(t->inner);
  const auto& v = real->transform->getValue();
  // Serialize: 7 GradingRGBM (4 doubles each) + 6 scalar doubles = 34 doubles
  int off = 0;
  auto write_rgbm = [&](const ocio::GradingRGBM& g) {
    values[off++] = g.m_red;
    values[off++] = g.m_green;
    values[off++] = g.m_blue;
    values[off++] = g.m_master;
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
  END_TRY_VOID
#endif
}

void ocio_grading_primary_transform_set_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryTransform>(t->inner);
  ocio::GradingPrimary v(ocio::GRADING_LOG);
  int off = 0;
  auto read_rgbm = [&](ocio::GradingRGBM& g) {
    g.m_red   = values[off++];
    g.m_green = values[off++];
    g.m_blue  = values[off++];
    g.m_master= values[off++];
  };
  read_rgbm(v.m_brightness);
  read_rgbm(v.m_contrast);
  read_rgbm(v.m_gamma);
  read_rgbm(v.m_offset);
  read_rgbm(v.m_exposure);
  read_rgbm(v.m_lift);
  read_rgbm(v.m_gain);
  v.m_saturation = values[off++];
  v.m_pivot      = values[off++];
  v.m_pivotBlack = values[off++];
  v.m_pivotWhite = values[off++];
  v.m_clampBlack = values[off++];
  v.m_clampWhite = values[off++];
  real->transform->setValue(v);
  END_TRY_VOID
#endif
}

bool ocio_grading_primary_transform_is_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryTransform>(t->inner);
  return real->transform->isDynamic();
  END_TRY(false)
#endif
}

void ocio_grading_primary_transform_make_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryTransform>(t->inner);
  real->transform->makeDynamic();
  END_TRY_VOID
#endif
}

void ocio_grading_primary_transform_make_non_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryTransform>(t->inner);
  real->transform->makeNonDynamic();
  END_TRY_VOID
#endif
}

int ocio_grading_primary_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryTransform>(t->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_grading_primary_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingPrimaryTransform>(t->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

void ocio_grading_primary_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingPrimaryTransformHandle*>(handle);
}

// --- GradingToneTransform ---

void* ocio_grading_tone_transform_create(int style) {
#ifdef OCIO_RS_STUB
  (void)style;
  return new ocio_rs_bridge::GradingToneTransformHandle{};
#else
  BEGIN_TRY
  auto r = std::make_shared<ocio_rs_bridge::RealGradingToneTransform>();
  r->transform = ocio::GradingToneTransform::Create(static_cast<ocio::GradingStyle>(style));
  auto hdl = std::make_unique<ocio_rs_bridge::GradingToneTransformHandle>();
  hdl->inner = r;
  return hdl.release();
  END_TRY(nullptr)
#endif
}

int ocio_grading_tone_transform_get_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingToneTransform>(t->inner);
  return static_cast<int>(real->transform->getStyle());
  END_TRY(0)
#endif
}

void ocio_grading_tone_transform_set_style(void* transform, int style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingToneTransform>(t->inner);
  real->transform->setStyle(static_cast<ocio::GradingStyle>(style));
  END_TRY_VOID
#endif
}

void ocio_grading_tone_transform_get_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingToneTransform>(t->inner);
  const auto& v = real->transform->getValue();
  int off = 0;
  auto write_rgbmsw = [&](const ocio::GradingRGBMSW& g) {
    values[off++] = g.m_red;
    values[off++] = g.m_green;
    values[off++] = g.m_blue;
    values[off++] = g.m_master;
    values[off++] = g.m_start;
    values[off++] = g.m_width;
  };
  write_rgbmsw(v.m_blacks);
  write_rgbmsw(v.m_shadows);
  write_rgbmsw(v.m_midtones);
  write_rgbmsw(v.m_highlights);
  write_rgbmsw(v.m_whites);
  values[off++] = v.m_scontrast;
  END_TRY_VOID
#endif
}

void ocio_grading_tone_transform_set_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingToneTransform>(t->inner);
  ocio::GradingTone v(ocio::GRADING_LOG);
  int off = 0;
  auto read_rgbmsw = [&](ocio::GradingRGBMSW& g) {
    g.m_red    = values[off++];
    g.m_green  = values[off++];
    g.m_blue   = values[off++];
    g.m_master = values[off++];
    g.m_start  = values[off++];
    g.m_width  = values[off++];
  };
  read_rgbmsw(v.m_blacks);
  read_rgbmsw(v.m_shadows);
  read_rgbmsw(v.m_midtones);
  read_rgbmsw(v.m_highlights);
  read_rgbmsw(v.m_whites);
  v.m_scontrast = values[off++];
  real->transform->setValue(v);
  END_TRY_VOID
#endif
}

bool ocio_grading_tone_transform_is_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingToneTransform>(t->inner);
  return real->transform->isDynamic();
  END_TRY(false)
#endif
}

void ocio_grading_tone_transform_make_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingToneTransform>(t->inner);
  real->transform->makeDynamic();
  END_TRY_VOID
#endif
}

void ocio_grading_tone_transform_make_non_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingToneTransform>(t->inner);
  real->transform->makeNonDynamic();
  END_TRY_VOID
#endif
}

int ocio_grading_tone_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingToneTransform>(t->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_grading_tone_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingToneTransform>(t->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

void ocio_grading_tone_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingToneTransformHandle*>(handle);
}

// --- GradingRGBCurveTransform ---

void* ocio_grading_rgb_curve_transform_create(int style) {
#ifdef OCIO_RS_STUB
  (void)style;
  return new ocio_rs_bridge::GradingRGBCurveTransformHandle{};
#else
  BEGIN_TRY
  auto r = std::make_shared<ocio_rs_bridge::RealGradingRGBCurveTransform>();
  r->transform = ocio::GradingRGBCurveTransform::Create(static_cast<ocio::GradingStyle>(style));
  auto hdl = std::make_unique<ocio_rs_bridge::GradingRGBCurveTransformHandle>();
  hdl->inner = r;
  return hdl.release();
  END_TRY(nullptr)
#endif
}

int ocio_grading_rgb_curve_transform_get_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  return static_cast<int>(real->transform->getStyle());
  END_TRY(0)
#endif
}

void ocio_grading_rgb_curve_transform_set_style(void* transform, int style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  real->transform->setStyle(static_cast<ocio::GradingStyle>(style));
  END_TRY_VOID
#endif
}

int ocio_grading_rgb_curve_transform_get_num_control_points(void* transform, int curveType) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  auto curve = real->transform->getValue();
  return static_cast<int>(curve->getNumControlPoints(static_cast<ocio::RGBCurveType>(curveType)));
  END_TRY(0)
#endif
}

void ocio_grading_rgb_curve_transform_get_control_point(void* transform, int curveType, int index, float* x, float* y) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; (void)index; (void)x; (void)y;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  auto curve = real->transform->getValue();
  const auto& pt = curve->getControlPoint(static_cast<ocio::RGBCurveType>(curveType), index);
  *x = pt.m_x;
  *y = pt.m_y;
  END_TRY_VOID
#endif
}

void ocio_grading_rgb_curve_transform_set_num_control_points(void* transform, int curveType, int num) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; (void)num;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  auto curve = real->transform->getValue();
  curve->setNumControlPoints(static_cast<ocio::RGBCurveType>(curveType), num);
  END_TRY_VOID
#endif
}

void ocio_grading_rgb_curve_transform_set_control_point(void* transform, int curveType, int index, float x, float y) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; (void)index; (void)x; (void)y;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  auto curve = real->transform->getValue();
  curve->getControlPoint(static_cast<ocio::RGBCurveType>(curveType), index) = ocio::GradingControlPoint(x, y);
  END_TRY_VOID
#endif
}

float ocio_grading_rgb_curve_transform_get_slope(void* transform, int curveType, int index) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; (void)index;
  return 0.0f;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  return real->transform->getSlope(static_cast<ocio::RGBCurveType>(curveType), index);
  END_TRY(0.0f)
#endif
}

void ocio_grading_rgb_curve_transform_set_slope(void* transform, int curveType, int index, float slope) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; (void)index; (void)slope;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  real->transform->setSlope(static_cast<ocio::RGBCurveType>(curveType), index, slope);
  END_TRY_VOID
#endif
}

bool ocio_grading_rgb_curve_transform_slopes_are_default(void* transform, int curveType) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType;
  return true;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  return real->transform->slopesAreDefault(static_cast<ocio::RGBCurveType>(curveType));
  END_TRY(true)
#endif
}

bool ocio_grading_rgb_curve_transform_get_bypass_lin_to_log(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  return real->transform->getBypassLinToLog();
  END_TRY(false)
#endif
}

void ocio_grading_rgb_curve_transform_set_bypass_lin_to_log(void* transform, bool bypass) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)bypass;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  real->transform->setBypassLinToLog(bypass);
  END_TRY_VOID
#endif
}

bool ocio_grading_rgb_curve_transform_is_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  return real->transform->isDynamic();
  END_TRY(false)
#endif
}

void ocio_grading_rgb_curve_transform_make_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  real->transform->makeDynamic();
  END_TRY_VOID
#endif
}

void ocio_grading_rgb_curve_transform_make_non_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  real->transform->makeNonDynamic();
  END_TRY_VOID
#endif
}

int ocio_grading_rgb_curve_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_grading_rgb_curve_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingRGBCurveTransform>(t->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

void ocio_grading_rgb_curve_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingRGBCurveTransformHandle*>(handle);
}

// --- GradingHueCurveTransform ---

void* ocio_grading_hue_curve_transform_create(int style) {
#ifdef OCIO_RS_STUB
  (void)style;
  return new ocio_rs_bridge::GradingHueCurveTransformHandle{};
#else
  BEGIN_TRY
  auto r = std::make_shared<ocio_rs_bridge::RealGradingHueCurveTransform>();
  r->transform = ocio::GradingHueCurveTransform::Create(static_cast<ocio::GradingStyle>(style));
  auto hdl = std::make_unique<ocio_rs_bridge::GradingHueCurveTransformHandle>();
  hdl->inner = r;
  return hdl.release();
  END_TRY(nullptr)
#endif
}

int ocio_grading_hue_curve_transform_get_style(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 1;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  return static_cast<int>(real->transform->getStyle());
  END_TRY(0)
#endif
}

void ocio_grading_hue_curve_transform_set_style(void* transform, int style) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)style;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  real->transform->setStyle(static_cast<ocio::GradingStyle>(style));
  END_TRY_VOID
#endif
}

int ocio_grading_hue_curve_transform_get_num_control_points(void* transform, int curveType) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  auto curve = real->transform->getValue();
  return static_cast<int>(curve->getNumControlPoints(static_cast<ocio::RGBCurveType>(curveType)));
  END_TRY(0)
#endif
}

void ocio_grading_hue_curve_transform_get_control_point(void* transform, int curveType, int index, float* x, float* y) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; (void)index; (void)x; (void)y;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  auto curve = real->transform->getValue();
  const auto& pt = curve->getControlPoint(static_cast<ocio::RGBCurveType>(curveType), index);
  *x = pt.m_x;
  *y = pt.m_y;
  END_TRY_VOID
#endif
}

void ocio_grading_hue_curve_transform_set_num_control_points(void* transform, int curveType, int num) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; (void)num;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  auto curve = real->transform->getValue();
  curve->setNumControlPoints(static_cast<ocio::RGBCurveType>(curveType), num);
  END_TRY_VOID
#endif
}

void ocio_grading_hue_curve_transform_set_control_point(void* transform, int curveType, int index, float x, float y) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; (void)index; (void)x; (void)y;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  auto curve = real->transform->getValue();
  curve->getControlPoint(static_cast<ocio::RGBCurveType>(curveType), index) = ocio::GradingControlPoint(x, y);
  END_TRY_VOID
#endif
}

float ocio_grading_hue_curve_transform_get_slope(void* transform, int curveType, int index) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; (void)index; return 0.0f;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  return real->transform->getSlope(static_cast<ocio::RGBCurveType>(curveType), index);
  END_TRY(0.0f)
#endif
}

void ocio_grading_hue_curve_transform_set_slope(void* transform, int curveType, int index, float slope) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; (void)index; (void)slope;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  real->transform->setSlope(static_cast<ocio::RGBCurveType>(curveType), index, slope);
  END_TRY_VOID
#endif
}

bool ocio_grading_hue_curve_transform_slopes_are_default(void* transform, int curveType) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)curveType; return true;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  return real->transform->slopesAreDefault(static_cast<ocio::RGBCurveType>(curveType));
  END_TRY(true)
#endif
}

bool ocio_grading_hue_curve_transform_get_bypass_lin_to_log(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  return real->transform->getBypassLinToLog();
  END_TRY(false)
#endif
}

void ocio_grading_hue_curve_transform_set_bypass_lin_to_log(void* transform, bool bypass) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)bypass;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  real->transform->setBypassLinToLog(bypass);
  END_TRY_VOID
#endif
}

bool ocio_grading_hue_curve_transform_is_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  return real->transform->isDynamic();
  END_TRY(false)
#endif
}

void ocio_grading_hue_curve_transform_make_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  real->transform->makeDynamic();
  END_TRY_VOID
#endif
}

void ocio_grading_hue_curve_transform_make_non_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  real->transform->makeNonDynamic();
  END_TRY_VOID
#endif
}

int ocio_grading_hue_curve_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_grading_hue_curve_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealGradingHueCurveTransform>(t->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

void ocio_grading_hue_curve_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::GradingHueCurveTransformHandle*>(handle);
}

// --- AllocationTransform ---

void* ocio_allocation_transform_create(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::AllocationTransformHandle{};
#else
  BEGIN_TRY
  auto r = std::make_shared<ocio_rs_bridge::RealAllocationTransform>();
  r->transform = ocio::AllocationTransform::Create();
  auto hdl = std::make_unique<ocio_rs_bridge::AllocationTransformHandle>();
  hdl->inner = r;
  return hdl.release();
  END_TRY(nullptr)
#endif
}

int ocio_allocation_transform_get_allocation(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::AllocationTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealAllocationTransform>(t->inner);
  return static_cast<int>(real->transform->getAllocation());
  END_TRY(0)
#endif
}

void ocio_allocation_transform_set_allocation(void* transform, int allocation) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)allocation;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::AllocationTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealAllocationTransform>(t->inner);
  real->transform->setAllocation(static_cast<ocio::Allocation>(allocation));
  END_TRY_VOID
#endif
}

int ocio_allocation_transform_get_num_vars(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::AllocationTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealAllocationTransform>(t->inner);
  return real->transform->getNumVars();
  END_TRY(0)
#endif
}

void ocio_allocation_transform_get_vars(void* transform, float* vars) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)vars;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::AllocationTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealAllocationTransform>(t->inner);
  real->transform->getVars(vars);
  END_TRY_VOID
#endif
}

void ocio_allocation_transform_set_vars(void* transform, int numvars, const float* vars) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)numvars; (void)vars;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::AllocationTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealAllocationTransform>(t->inner);
  real->transform->setVars(numvars, vars);
  END_TRY_VOID
#endif
}

int ocio_allocation_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::AllocationTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealAllocationTransform>(t->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_allocation_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::AllocationTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealAllocationTransform>(t->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

void ocio_allocation_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::AllocationTransformHandle*>(handle);
}

// --- LogAffineTransform ---

void* ocio_log_affine_transform_create(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::LogAffineTransformHandle{};
#else
  BEGIN_TRY
  auto r = std::make_shared<ocio_rs_bridge::RealLogAffineTransform>();
  r->transform = ocio::LogAffineTransform::Create();
  auto hdl = std::make_unique<ocio_rs_bridge::LogAffineTransformHandle>();
  hdl->inner = r;
  return hdl.release();
  END_TRY(nullptr)
#endif
}

double ocio_log_affine_transform_get_base(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 2.0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  return real->transform->getBase();
  END_TRY(2.0)
#endif
}

void ocio_log_affine_transform_set_base(void* transform, double base) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)base;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  real->transform->setBase(base);
  END_TRY_VOID
#endif
}

void ocio_log_affine_transform_destroy(void* handle);
void ocio_log_camera_transform_destroy(void* handle);

void ocio_log_affine_transform_get_log_side_slope_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  real->transform->getLogSideSlopeValue(reinterpret_cast<double(&)[3]>(*values));
  END_TRY_VOID
#endif
}

void ocio_log_affine_transform_set_log_side_slope_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  real->transform->setLogSideSlopeValue(*reinterpret_cast<const double(*)[3]>(values));
  END_TRY_VOID
#endif
}

void ocio_log_affine_transform_get_log_side_offset_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  real->transform->getLogSideOffsetValue(reinterpret_cast<double(&)[3]>(*values));
  END_TRY_VOID
#endif
}

void ocio_log_affine_transform_set_log_side_offset_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  real->transform->setLogSideOffsetValue(*reinterpret_cast<const double(*)[3]>(values));
  END_TRY_VOID
#endif
}

void ocio_log_affine_transform_get_lin_side_slope_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  real->transform->getLinSideSlopeValue(reinterpret_cast<double(&)[3]>(*values));
  END_TRY_VOID
#endif
}

void ocio_log_affine_transform_set_lin_side_slope_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  real->transform->setLinSideSlopeValue(*reinterpret_cast<const double(*)[3]>(values));
  END_TRY_VOID
#endif
}

void ocio_log_affine_transform_get_lin_side_offset_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  real->transform->getLinSideOffsetValue(reinterpret_cast<double(&)[3]>(*values));
  END_TRY_VOID
#endif
}

void ocio_log_affine_transform_set_lin_side_offset_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  real->transform->setLinSideOffsetValue(*reinterpret_cast<const double(*)[3]>(values));
  END_TRY_VOID
#endif
}

void ocio_log_affine_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(handle);
}

// --- LogCameraTransform ---

void* ocio_log_camera_transform_create(const double* linSideBreakValues) {
#ifdef OCIO_RS_STUB
  (void)linSideBreakValues;
  return new ocio_rs_bridge::LogCameraTransformHandle{};
#else
  BEGIN_TRY
  auto r = std::make_shared<ocio_rs_bridge::RealLogCameraTransform>();
  r->transform = ocio::LogCameraTransform::Create(*reinterpret_cast<const double(*)[3]>(linSideBreakValues));
  auto hdl = std::make_unique<ocio_rs_bridge::LogCameraTransformHandle>();
  hdl->inner = r;
  return hdl.release();
  END_TRY(nullptr)
#endif
}

double ocio_log_camera_transform_get_base(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 2.0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  return real->transform->getBase();
  END_TRY(2.0)
#endif
}

void ocio_log_camera_transform_set_base(void* transform, double base) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)base;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->setBase(base);
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_get_log_side_slope_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->getLogSideSlopeValue(reinterpret_cast<double(&)[3]>(*values));
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_set_log_side_slope_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->setLogSideSlopeValue(*reinterpret_cast<const double(*)[3]>(values));
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_get_log_side_offset_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->getLogSideOffsetValue(reinterpret_cast<double(&)[3]>(*values));
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_set_log_side_offset_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->setLogSideOffsetValue(*reinterpret_cast<const double(*)[3]>(values));
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_get_lin_side_slope_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->getLinSideSlopeValue(reinterpret_cast<double(&)[3]>(*values));
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_set_lin_side_slope_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->setLinSideSlopeValue(*reinterpret_cast<const double(*)[3]>(values));
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_get_lin_side_offset_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->getLinSideOffsetValue(reinterpret_cast<double(&)[3]>(*values));
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_set_lin_side_offset_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->setLinSideOffsetValue(*reinterpret_cast<const double(*)[3]>(values));
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_get_lin_side_break_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->getLinSideBreakValue(reinterpret_cast<double(&)[3]>(*values));
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_set_lin_side_break_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->setLinSideBreakValue(*reinterpret_cast<const double(*)[3]>(values));
  END_TRY_VOID
#endif
}

bool ocio_log_camera_transform_get_linear_slope_value(void* transform, double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
  return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  return real->transform->getLinearSlopeValue(reinterpret_cast<double(&)[3]>(*values));
  END_TRY(false)
#endif
}

void ocio_log_camera_transform_set_linear_slope_value(void* transform, const double* values) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)values;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->setLinearSlopeValue(*reinterpret_cast<const double(*)[3]>(values));
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_unset_linear_slope_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->unsetLinearSlopeValue();
  END_TRY_VOID
#endif
}

void ocio_log_camera_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(handle);
}

// --- DisplayViewTransform ---

void* ocio_display_view_transform_create(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::DisplayViewTransformHandle{};
#else
  BEGIN_TRY
  auto t = ocio::DisplayViewTransform::Create();
  auto handle = new ocio_rs_bridge::DisplayViewTransformHandle{};
  handle->inner = std::make_shared<ocio_rs_bridge::RealDisplayViewTransform>(ocio_rs_bridge::RealDisplayViewTransform{t});
  return handle;
  END_TRY(nullptr)
#endif
}

const char* ocio_display_view_transform_get_src(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return nullptr;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  const char* result = real->transform->getSrc();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_display_view_transform_set_src(void* transform, const char* src) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)src;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  real->transform->setSrc(src);
  END_TRY_VOID
#endif
}

const char* ocio_display_view_transform_get_display(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return nullptr;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  const char* result = real->transform->getDisplay();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_display_view_transform_set_display(void* transform, const char* display) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)display;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  real->transform->setDisplay(display);
  END_TRY_VOID
#endif
}

const char* ocio_display_view_transform_get_view(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return nullptr;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  const char* result = real->transform->getView();
  static thread_local std::string cached;
  cached = result ? result : "";
  return result ? cached.c_str() : nullptr;
  END_TRY(nullptr)
#endif
}

void ocio_display_view_transform_set_view(void* transform, const char* view) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)view;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  real->transform->setView(view);
  END_TRY_VOID
#endif
}

bool ocio_display_view_transform_get_looks_bypass(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  return real->transform->getLooksBypass();
  END_TRY(false)
#endif
}

void ocio_display_view_transform_set_looks_bypass(void* transform, bool bypass) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)bypass;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  real->transform->setLooksBypass(bypass);
  END_TRY_VOID
#endif
}

int ocio_display_view_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_display_view_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

void ocio_display_view_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(handle);
}

// --- FileRules ---

void* ocio_file_rules_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_file_rules().release();
#else
  try {
    auto r = ocio::FileRules::Create();
    if (!r) return nullptr;
    auto* h = new ocio_rs_bridge::FileRulesHandle;
    h->inner = std::make_shared<ocio_rs_bridge::RealFileRules>(ocio_rs_bridge::RealFileRules{r});
    return static_cast<void*>(h);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_file_rules_create_editable_copy(void* rules) {
#ifdef OCIO_RS_STUB
  (void)rules; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    auto copy = r->createEditableCopy();
    if (!copy) return nullptr;
    auto* new_h = new ocio_rs_bridge::FileRulesHandle;
    new_h->inner = std::make_shared<ocio_rs_bridge::RealFileRules>(ocio_rs_bridge::RealFileRules{copy});
    return static_cast<void*>(new_h);
  } catch (...) { return nullptr; }
#endif
}

unsigned long long ocio_file_rules_get_num_entries(void* rules) {
#ifdef OCIO_RS_STUB
  (void)rules; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    return static_cast<unsigned long long>(r->getNumEntries());
  } catch (...) { return 0; }
#endif
}

unsigned long long ocio_file_rules_get_index_for_rule(void* rules, const char* ruleName) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleName; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    return static_cast<unsigned long long>(r->getIndexForRule(ruleName));
  } catch (...) { return 0; }
#endif
}

const char* ocio_file_rules_get_name(void* rules, unsigned long long ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    thread_local std::string cached;
    cached = r->getName((size_t)ruleIndex);
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_file_rules_get_pattern(void* rules, unsigned long long ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    thread_local std::string cached;
    cached = r->getPattern((size_t)ruleIndex);
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_rules_set_pattern(void* rules, unsigned long long ruleIndex, const char* pattern) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; (void)pattern;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->setPattern((size_t)ruleIndex, pattern);
  } catch (...) {}
#endif
}

const char* ocio_file_rules_get_extension(void* rules, unsigned long long ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    thread_local std::string cached;
    cached = r->getExtension((size_t)ruleIndex);
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_rules_set_extension(void* rules, unsigned long long ruleIndex, const char* extension) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; (void)extension;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->setExtension((size_t)ruleIndex, extension);
  } catch (...) {}
#endif
}

const char* ocio_file_rules_get_regex(void* rules, unsigned long long ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    thread_local std::string cached;
    cached = r->getRegex((size_t)ruleIndex);
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_rules_set_regex(void* rules, unsigned long long ruleIndex, const char* regex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; (void)regex;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->setRegex((size_t)ruleIndex, regex);
  } catch (...) {}
#endif
}

const char* ocio_file_rules_get_color_space(void* rules, unsigned long long ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    thread_local std::string cached;
    cached = r->getColorSpace((size_t)ruleIndex);
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_rules_set_color_space(void* rules, unsigned long long ruleIndex, const char* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; (void)colorSpace;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->setColorSpace((size_t)ruleIndex, colorSpace);
  } catch (...) {}
#endif
}

unsigned long long ocio_file_rules_get_num_custom_keys(void* rules, unsigned long long ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    return static_cast<unsigned long long>(r->getNumCustomKeys((size_t)ruleIndex));
  } catch (...) { return 0; }
#endif
}

const char* ocio_file_rules_get_custom_key_name(void* rules, unsigned long long ruleIndex, unsigned long long key) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; (void)key; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    thread_local std::string cached;
    cached = r->getCustomKeyName((size_t)ruleIndex, (size_t)key);
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_file_rules_get_custom_key_value(void* rules, unsigned long long ruleIndex, unsigned long long key) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; (void)key; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    thread_local std::string cached;
    cached = r->getCustomKeyValue((size_t)ruleIndex, (size_t)key);
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_rules_set_custom_key(void* rules, unsigned long long ruleIndex, const char* key, const char* value) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; (void)key; (void)value;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->setCustomKey((size_t)ruleIndex, key, value);
  } catch (...) {}
#endif
}

void ocio_file_rules_insert_rule(void* rules, unsigned long long ruleIndex, const char* name, const char* colorSpace, const char* pattern, const char* extension) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; (void)name; (void)colorSpace; (void)pattern; (void)extension;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->insertRule((size_t)ruleIndex, name, colorSpace, pattern, extension);
  } catch (...) {}
#endif
}

void ocio_file_rules_insert_rule_regex(void* rules, unsigned long long ruleIndex, const char* name, const char* colorSpace, const char* regex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex; (void)name; (void)colorSpace; (void)regex;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->insertRule((size_t)ruleIndex, name, colorSpace, regex);
  } catch (...) {}
#endif
}

void ocio_file_rules_insert_path_search_rule(void* rules, unsigned long long ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->insertPathSearchRule((size_t)ruleIndex);
  } catch (...) {}
#endif
}

void ocio_file_rules_set_default_rule_color_space(void* rules, const char* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)colorSpace;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->setDefaultRuleColorSpace(colorSpace);
  } catch (...) {}
#endif
}

void ocio_file_rules_remove_rule(void* rules, unsigned long long ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->removeRule((size_t)ruleIndex);
  } catch (...) {}
#endif
}

void ocio_file_rules_increase_rule_priority(void* rules, unsigned long long ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->increaseRulePriority((size_t)ruleIndex);
  } catch (...) {}
#endif
}

void ocio_file_rules_decrease_rule_priority(void* rules, unsigned long long ruleIndex) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)ruleIndex;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    r->decreaseRulePriority((size_t)ruleIndex);
  } catch (...) {}
#endif
}

bool ocio_file_rules_is_default(void* rules) {
#ifdef OCIO_RS_STUB
  (void)rules; return true;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    return r->isDefault();
  } catch (...) { return true; }
#endif
}

const char* ocio_file_rules_get_color_space_from_filepath(void* rules, const char* filePath) {
#ifdef OCIO_RS_STUB
  (void)rules; (void)filePath; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(rules);
    auto r = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    return r->getColorSpaceFromFilepath(filePath);
  } catch (...) { return nullptr; }
#endif
}

void ocio_file_rules_destroy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
#else
  try { delete static_cast<ocio_rs_bridge::FileRulesHandle*>(handle); } catch (...) {}
#endif
}

// --- Config: FileRules ---

void* ocio_config_get_file_rules(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return ocio_rs_bridge::make_stub_file_rules().release();
#else
  try {
    auto r = ocio_rs_bridge::get_real_config(config)->getFileRules();
    if (!r) return nullptr;
    auto* h = new ocio_rs_bridge::FileRulesHandle;
    h->inner = std::make_shared<ocio_rs_bridge::RealFileRules>(ocio_rs_bridge::RealFileRules{r});
    return static_cast<void*>(h);
  } catch (...) { return nullptr; }
#endif
}

// --- Lut1DTransform: data ---

unsigned long long ocio_lut1d_transform_get_length(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    return static_cast<unsigned long long>(t->getLength());
  } catch (...) { return 0; }
#endif
}

void ocio_lut1d_transform_get_values(void* transform, double* data) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)data;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    t->getValues(data);
  } catch (...) {}
#endif
}

void ocio_lut1d_transform_set_length(void* transform, unsigned long long len) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)len;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    t->setLength(static_cast<unsigned long>(len));
  } catch (...) {}
#endif
}

void ocio_lut1d_transform_set_values(void* transform, const double* data) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)data;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    t->setValues(data);
  } catch (...) {}
#endif
}

// --- Lut3DTransform: data ---

unsigned long long ocio_lut3d_transform_get_grid_size(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform;
    return static_cast<unsigned long long>(t->getGridSize());
  } catch (...) { return 0; }
#endif
}

void ocio_lut3d_transform_get_values(void* transform, double* data) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)data;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform;
    t->getValues(data);
  } catch (...) {}
#endif
}

void ocio_lut3d_transform_set_grid_size(void* transform, unsigned long long size) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)size;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform;
    t->setGridSize(static_cast<unsigned long>(size));
  } catch (...) {}
#endif
}

void ocio_lut3d_transform_set_values(void* transform, const double* data) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)data;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut3DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut3DTransform>(h->inner)->transform;
    t->setValues(data);
  } catch (...) {}
#endif
}

// --- GroupTransform: remove/clear ---

void ocio_group_transform_remove_transform(void* transform, unsigned long long index) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)index;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::GroupTransformHandle*>(transform);
    auto group = std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(h->inner);
    group->transform->removeTransform(static_cast<size_t>(index));
  } catch (...) {}
#endif
}

void ocio_group_transform_clear_transforms(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::GroupTransformHandle*>(transform);
    auto group = std::static_pointer_cast<ocio_rs_bridge::RealGroupTransform>(h->inner);
    group->transform->clearTransforms();
  } catch (...) {}
#endif
}

// --- Config: clear all, version setters, interpolation, working dir ---

void ocio_config_clear_all(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->clearAll();
  } catch (...) {}
#endif
}

void ocio_config_set_major_version(void* config, unsigned int version) {
#ifdef OCIO_RS_STUB
  (void)config; (void)version;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setMajorVersion(version);
  } catch (...) {}
#endif
}

void ocio_config_set_minor_version(void* config, unsigned int version) {
#ifdef OCIO_RS_STUB
  (void)config; (void)version;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setMinorVersion(version);
  } catch (...) {}
#endif
}

int ocio_config_get_default_interpolation(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return 0;
#else
  try {
    return static_cast<int>(ocio_rs_bridge::get_real_config(config)->getDefaultInterpolation());
  } catch (...) { return 0; }
#endif
}

void ocio_config_set_default_interpolation(void* config, int interpolation) {
#ifdef OCIO_RS_STUB
  (void)config; (void)interpolation;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setDefaultInterpolation(
        static_cast<ocio::Interpolation>(interpolation));
  } catch (...) {}
#endif
}

const char* ocio_config_get_working_dir(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return nullptr;
#else
  try {
    thread_local std::string cached;
    cached = ocio_rs_bridge::get_real_config(config)->getWorkingDir();
    return cached.empty() ? nullptr : cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_working_dir(void* config, const char* dirName) {
#ifdef OCIO_RS_STUB
  (void)config; (void)dirName;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setWorkingDir(dirName);
  } catch (...) {}
#endif
}

// --- Config: inactive color spaces, archivable, processor cache, file rules ---

const char* ocio_config_get_inactive_color_spaces(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return nullptr;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->getInactiveColorSpaces();
  } catch (...) { return nullptr; }
#endif
}

void ocio_config_set_inactive_color_spaces(void* config, const char* inactive) {
#ifdef OCIO_RS_STUB
  (void)config; (void)inactive;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setInactiveColorSpaces(inactive);
  } catch (...) {}
#endif
}

bool ocio_config_is_archivable(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return false;
#else
  try {
    return ocio_rs_bridge::get_real_config(config)->isArchivable();
  } catch (...) { return false; }
#endif
}

void ocio_config_clear_processor_cache(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->clearProcessorCache();
  } catch (...) {}
#endif
}

void ocio_config_set_file_rules(void* config, void* fileRules) {
#ifdef OCIO_RS_STUB
  (void)config; (void)fileRules;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FileRulesHandle*>(fileRules);
    auto rules = std::static_pointer_cast<ocio_rs_bridge::RealFileRules>(h->inner)->rules;
    ocio_rs_bridge::get_real_config(config)->setFileRules(rules);
  } catch (...) {}
#endif
}

// --- Config: environment mode ---

void ocio_config_set_environment_mode(void* config, int mode) {
#ifdef OCIO_RS_STUB
  (void)config; (void)mode;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->setEnvironmentMode(static_cast<OCIO_NAMESPACE::EnvironmentMode>(mode));
  } catch (...) {}
#endif
}

int ocio_config_get_environment_mode(void* config) {
#ifdef OCIO_RS_STUB
  (void)config; return 0;
#else
  try {
    return static_cast<int>(ocio_rs_bridge::get_real_config(config)->getEnvironmentMode());
  } catch (...) { return 0; }
#endif
}

void ocio_config_load_environment(void* config) {
#ifdef OCIO_RS_STUB
  (void)config;
#else
  try {
    ocio_rs_bridge::get_real_config(config)->loadEnvironment();
  } catch (...) {}
#endif
}

// --- ColorSpace: visibility + set_reference_space_type ---

int ocio_color_space_get_visibility(void* colorSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; return 0;
#else
  try {
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(
        static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace)->inner);
    return static_cast<int>(real->colorSpace->getVisibility());
  } catch (...) { return 0; }
#endif
}

void ocio_color_space_set_visibility(void* colorSpace, int visibility) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)visibility;
#else
  try {
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(
        static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace)->inner);
    real->colorSpace->setVisibility(static_cast<ocio::ColorSpaceVisibility>(visibility));
  } catch (...) {}
#endif
}

void ocio_color_space_set_reference_space_type(void* colorSpace, int referenceSpace) {
#ifdef OCIO_RS_STUB
  (void)colorSpace; (void)referenceSpace;
#else
  try {
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpace>(
        static_cast<ocio_rs_bridge::ColorSpaceHandle*>(colorSpace)->inner);
    real->colorSpace->setReferenceSpaceType(static_cast<ocio::ReferenceSpaceType>(referenceSpace));
  } catch (...) {}
#endif
}

// --- LookTransform: skip color space conversion ---

bool ocio_look_transform_get_skip_color_space_conversion(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookTransformHandle*>(transform);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(h->inner);
    return real->transform->getSkipColorSpaceConversion();
  } catch (...) { return false; }
#endif
}

void ocio_look_transform_set_skip_color_space_conversion(void* transform, bool skip) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)skip;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::LookTransformHandle*>(transform);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealLookTransform>(h->inner);
    real->transform->setSkipColorSpaceConversion(skip);
  } catch (...) {}
#endif
}

// --- Global: processor cache flags ---

int ocio_get_processor_cache_flags(void) {
#ifdef OCIO_RS_STUB
  return 0;
#else
  try {
    return static_cast<int>(ocio::GetProcessorCacheFlags());
  } catch (...) { return 0; }
#endif
}

void ocio_set_processor_cache_flags(int flags) {
#ifdef OCIO_RS_STUB
  (void)flags;
#else
  try {
    ocio::SetProcessorCacheFlags(static_cast<unsigned int>(flags));
  } catch (...) {}
#endif
}

// --- CPUProcessor: batch pixel processing ---

void ocio_cpu_processor_apply_rgba_pixels(void* cpu_processor, float* rgba, long numPixels, long stride) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor; (void)rgba; (void)numPixels; (void)stride;
#else
  try {
    ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->applyRGBA(rgba, numPixels, stride);
  } catch (...) {}
#endif
}

void ocio_cpu_processor_apply_rgb_pixels(void* cpu_processor, float* rgb, long numPixels, long stride) {
#ifdef OCIO_RS_STUB
  (void)cpu_processor; (void)rgb; (void)numPixels; (void)stride;
#else
  try {
    ocio_rs_bridge::get_real_cpu_processor(cpu_processor)->applyRGB(rgb, numPixels, stride);
  } catch (...) {}
#endif
}

// --- Processor: batch pixel processing ---

void ocio_processor_apply_rgba_pixels(void* processor, float* rgba, long numPixels, long stride) {
#ifdef OCIO_RS_STUB
  (void)processor; (void)rgba; (void)numPixels; (void)stride;
#else
  try {
    auto proc = ocio_rs_bridge::get_real_processor(processor);
    auto cpu = proc->getDefaultCPUProcessor();
    cpu->applyRGBA(rgba, numPixels, stride);
  } catch (...) {}
#endif
}

// --- Baker: format metadata ---

void* ocio_baker_get_format_metadata(void* baker) {
#ifdef OCIO_RS_STUB
  (void)baker; return ocio_rs_bridge::make_stub_format_metadata().release();
#else
  try {
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealBaker>(
        static_cast<ocio_rs_bridge::BakerHandle*>(baker)->inner);
    auto metadata = &real->baker->getFormatMetadata();
    if (!metadata) return nullptr;
    auto* handle = new ocio_rs_bridge::FormatMetadataHandle;
    handle->inner = std::make_shared<ocio_rs_bridge::RealFormatMetadata>(
        ocio_rs_bridge::RealFormatMetadata{const_cast<ocio::FormatMetadata*>(metadata), false});
    return static_cast<void*>(handle);
  } catch (...) { return nullptr; }
#endif
}

// --- Transform: format metadata ---

void* ocio_transform_get_format_metadata(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return ocio_rs_bridge::make_stub_format_metadata().release();
#else
  try {
    auto* base = static_cast<ocio_rs_bridge::TransformHandleBase*>(transform);
    if (!base) return nullptr;
    auto ocio_transform = base->get_ocio_transform();
    if (!ocio_transform) return nullptr;
    auto metadata = &ocio_transform->getFormatMetadata();
    if (!metadata) return nullptr;
    auto* handle = new ocio_rs_bridge::FormatMetadataHandle;
    handle->inner = std::make_shared<ocio_rs_bridge::RealFormatMetadata>(
        ocio_rs_bridge::RealFormatMetadata{const_cast<ocio::FormatMetadata*>(metadata), false});
    return static_cast<void*>(handle);
  } catch (...) { return nullptr; }
#endif
}

// --- ColorSpaceSet ---

void* ocio_color_space_set_create(void) {
#ifdef OCIO_RS_STUB
  return ocio_rs_bridge::make_stub_color_space_set().release();
#else
  try {
    auto set = ocio::ColorSpaceSet::Create();
    if (!set) return nullptr;
    auto* handle = new ocio_rs_bridge::ColorSpaceSetHandle;
    handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceSet>(
        ocio_rs_bridge::RealColorSpaceSet{set});
    return static_cast<void*>(handle);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_color_space_set_create_editable_copy(void* set) {
#ifdef OCIO_RS_STUB
  (void)set; return ocio_rs_bridge::make_stub_color_space_set().release();
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(set);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceSet>(h->inner);
    auto copy = real->set->createEditableCopy();
    if (!copy) return nullptr;
    auto* new_h = new ocio_rs_bridge::ColorSpaceSetHandle;
    new_h->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceSet>(
        ocio_rs_bridge::RealColorSpaceSet{copy});
    return static_cast<void*>(new_h);
  } catch (...) { return nullptr; }
#endif
}

int ocio_color_space_set_get_num_color_spaces(void* set) {
#ifdef OCIO_RS_STUB
  (void)set; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(set);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceSet>(h->inner);
    return real->set->getNumColorSpaces();
  } catch (...) { return 0; }
#endif
}

const char* ocio_color_space_set_get_color_space_name_by_index(void* set, int index) {
#ifdef OCIO_RS_STUB
  (void)set; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(set);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceSet>(h->inner);
    thread_local std::string cached;
    cached = real->set->getColorSpaceNameByIndex(index);
    return cached.empty() ? nullptr : cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void* ocio_color_space_set_get_color_space_by_index(void* set, int index) {
#ifdef OCIO_RS_STUB
  (void)set; (void)index; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(set);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceSet>(h->inner);
    auto cs = real->set->getColorSpaceByIndex(index);
    if (!cs) return nullptr;
    auto* cs_h = new ocio_rs_bridge::ColorSpaceHandle;
    cs_h->inner = std::make_shared<ocio_rs_bridge::RealColorSpace>(
        ocio_rs_bridge::RealColorSpace{cs});
    return static_cast<void*>(cs_h);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_color_space_set_get_color_space(void* set, const char* name) {
#ifdef OCIO_RS_STUB
  (void)set; (void)name; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(set);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceSet>(h->inner);
    auto cs = real->set->getColorSpace(name);
    if (!cs) return nullptr;
    auto* cs_h = new ocio_rs_bridge::ColorSpaceHandle;
    cs_h->inner = std::make_shared<ocio_rs_bridge::RealColorSpace>(
        ocio_rs_bridge::RealColorSpace{cs});
    return static_cast<void*>(cs_h);
  } catch (...) { return nullptr; }
#endif
}

int ocio_color_space_set_get_color_space_index(void* set, const char* name) {
#ifdef OCIO_RS_STUB
  (void)set; (void)name; return -1;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(set);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceSet>(h->inner);
    return real->set->getColorSpaceIndex(name);
  } catch (...) { return -1; }
#endif
}

bool ocio_color_space_set_has_color_space(void* set, const char* name) {
#ifdef OCIO_RS_STUB
  (void)set; (void)name; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(set);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealColorSpaceSet>(h->inner);
    return real->set->hasColorSpace(name);
  } catch (...) { return false; }
#endif
}

void ocio_color_space_set_destroy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
#else
  try { delete static_cast<ocio_rs_bridge::ColorSpaceSetHandle*>(handle); } catch (...) {}
#endif
}

// --- Config: ColorSpaceSet ---

void* ocio_config_get_color_space_set(void* config, const char* search) {
#ifdef OCIO_RS_STUB
  (void)config; (void)search;
  return ocio_rs_bridge::make_stub_color_space_set().release();
#else
  try {
    auto cfg = ocio_rs_bridge::get_real_config(config);
    auto set = cfg->getColorSpaceSet(search);
    if (!set) return nullptr;
    auto* handle = new ocio_rs_bridge::ColorSpaceSetHandle;
    handle->inner = std::make_shared<ocio_rs_bridge::RealColorSpaceSet>(
        ocio_rs_bridge::RealColorSpaceSet{set});
    return static_cast<void*>(handle);
  } catch (...) { return nullptr; }
#endif
}

// --- FormatMetadata ---

const char* ocio_format_metadata_get_element_name(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    thread_local std::string cached;
    cached = real->metadata->getElementName();
    return cached.empty() ? nullptr : cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_format_metadata_set_element_name(void* metadata, const char* name) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)name;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    real->metadata->setElementName(name);
  } catch (...) {}
#endif
}

const char* ocio_format_metadata_get_element_value(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    thread_local std::string cached;
    cached = real->metadata->getElementValue();
    return cached.empty() ? nullptr : cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_format_metadata_set_element_value(void* metadata, const char* value) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)value;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    real->metadata->setElementValue(value);
  } catch (...) {}
#endif
}

int ocio_format_metadata_get_num_attributes(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    return real->metadata->getNumAttributes();
  } catch (...) { return 0; }
#endif
}

const char* ocio_format_metadata_get_attribute_name(void* metadata, int i) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)i; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    thread_local std::string cached;
    cached = real->metadata->getAttributeName(i);
    return cached.empty() ? nullptr : cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_format_metadata_get_attribute_value_by_index(void* metadata, int i) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)i; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    thread_local std::string cached;
    cached = real->metadata->getAttributeValue(i);
    return cached.empty() ? nullptr : cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

const char* ocio_format_metadata_get_attribute_value(void* metadata, const char* name) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)name; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    thread_local std::string cached;
    cached = real->metadata->getAttributeValue(name);
    return cached.empty() ? nullptr : cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_format_metadata_add_attribute(void* metadata, const char* name, const char* value) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)name; (void)value;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    real->metadata->addAttribute(name, value);
  } catch (...) {}
#endif
}

void ocio_format_metadata_remove_attribute(void* metadata, const char* name) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)name;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    real->metadata->removeAttribute(name);
  } catch (...) {}
#endif
}

int ocio_format_metadata_get_num_children_elements(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    return real->metadata->getNumChildrenElements();
  } catch (...) { return 0; }
#endif
}

void* ocio_format_metadata_get_child_element(void* metadata, int i) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)i; return ocio_rs_bridge::make_stub_format_metadata().release();
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    auto child = &real->metadata->getChildElement(i);
    if (!child) return nullptr;
    auto* child_h = new ocio_rs_bridge::FormatMetadataHandle;
    child_h->inner = std::make_shared<ocio_rs_bridge::RealFormatMetadata>(
        ocio_rs_bridge::RealFormatMetadata{const_cast<ocio::FormatMetadata*>(child), false});
    return static_cast<void*>(child_h);
  } catch (...) { return nullptr; }
#endif
}

void ocio_format_metadata_add_child_element(void* metadata, const char* name, const char* value) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)name; (void)value;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    real->metadata->addChildElement(name, value);
  } catch (...) {}
#endif
}

void ocio_format_metadata_clear(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    real->metadata->clear();
  } catch (...) {}
#endif
}

const char* ocio_format_metadata_get_name(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    thread_local std::string cached;
    cached = real->metadata->getName();
    return cached.empty() ? nullptr : cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_format_metadata_set_name(void* metadata, const char* name) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)name;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    real->metadata->setName(name);
  } catch (...) {}
#endif
}

const char* ocio_format_metadata_get_id(void* metadata) {
#ifdef OCIO_RS_STUB
  (void)metadata; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    thread_local std::string cached;
    cached = real->metadata->getID();
    return cached.empty() ? nullptr : cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

void ocio_format_metadata_set_id(void* metadata, const char* id) {
#ifdef OCIO_RS_STUB
  (void)metadata; (void)id;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::FormatMetadataHandle*>(metadata);
    auto real = std::static_pointer_cast<ocio_rs_bridge::RealFormatMetadata>(h->inner);
    real->metadata->setID(id);
  } catch (...) {}
#endif
}

void ocio_format_metadata_destroy(void* handle) {
#ifdef OCIO_RS_STUB
  (void)handle;
#else
  try {
    // Note: FormatMetadata is never owned by our handle, so we only delete the handle struct
    delete static_cast<ocio_rs_bridge::FormatMetadataHandle*>(handle);
  } catch (...) {}
#endif
}

// --- BuiltinTransform: description ---

const char* ocio_builtin_transform_get_description(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::BuiltinTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealBuiltinTransform>(h->inner)->transform;
    thread_local std::string cached;
    cached = t->getDescription();
    return cached.c_str();
  } catch (...) { return nullptr; }
#endif
}

// --- DisplayViewTransform: data bypass ---

bool ocio_display_view_transform_get_data_bypass(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return false;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  return real->transform->getDataBypass();
  END_TRY(false)
#endif
}

void ocio_display_view_transform_set_data_bypass(void* transform, bool bypass) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)bypass;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::DisplayViewTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealDisplayViewTransform>(t->inner);
  real->transform->setDataBypass(bypass);
  END_TRY_VOID
#endif
}

// --- ExposureContrastTransform: log & non-dynamic ---

double ocio_exposure_contrast_transform_get_log_exposure_step(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0.0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  return real->transform->getLogExposureStep();
  END_TRY(0.0)
#endif
}

void ocio_exposure_contrast_transform_set_log_exposure_step(void* transform, double step) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)step;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->setLogExposureStep(step);
  END_TRY_VOID
#endif
}

double ocio_exposure_contrast_transform_get_log_mid_gray(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0.0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  return real->transform->getLogMidGray();
  END_TRY(0.0)
#endif
}

void ocio_exposure_contrast_transform_set_log_mid_gray(void* transform, double mid_gray) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)mid_gray;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->setLogMidGray(mid_gray);
  END_TRY_VOID
#endif
}

void ocio_exposure_contrast_transform_make_exposure_non_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->makeExposureNonDynamic();
  END_TRY_VOID
#endif
}

void ocio_exposure_contrast_transform_make_contrast_non_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->makeContrastNonDynamic();
  END_TRY_VOID
#endif
}

void ocio_exposure_contrast_transform_make_gamma_non_dynamic(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::ExposureContrastTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealExposureContrastTransform>(t->inner);
  real->transform->makeGammaNonDynamic();
  END_TRY_VOID
#endif
}

// --- LogAffineTransform: direction ---

int ocio_log_affine_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_log_affine_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogAffineTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogAffineTransform>(t->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

// --- LogCameraTransform: direction ---

int ocio_log_camera_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  return static_cast<int>(real->transform->getDirection());
  END_TRY(0)
#endif
}

void ocio_log_camera_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#else
  BEGIN_TRY
  auto t = static_cast<ocio_rs_bridge::LogCameraTransformHandle*>(transform);
  auto real = std::static_pointer_cast<ocio_rs_bridge::RealLogCameraTransform>(t->inner);
  real->transform->setDirection(static_cast<ocio::TransformDirection>(direction));
  END_TRY_VOID
#endif
}

// --- Lut1DTransform: half domain, raw halfs, hue adjust ---

bool ocio_lut1d_transform_get_input_half_domain(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    return t->getInputHalfDomain();
  } catch (...) { return false; }
#endif
}

void ocio_lut1d_transform_set_input_half_domain(void* transform, bool half_domain) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)half_domain;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    t->setInputHalfDomain(half_domain);
  } catch (...) {}
#endif
}

bool ocio_lut1d_transform_get_output_raw_halfs(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    return t->getOutputRawHalfs();
  } catch (...) { return false; }
#endif
}

void ocio_lut1d_transform_set_output_raw_halfs(void* transform, bool raw_halfs) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)raw_halfs;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    t->setOutputRawHalfs(raw_halfs);
  } catch (...) {}
#endif
}

int ocio_lut1d_transform_get_hue_adjust(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    return static_cast<int>(t->getHueAdjust());
  } catch (...) { return 0; }
#endif
}

void ocio_lut1d_transform_set_hue_adjust(void* transform, int hue_adjust) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)hue_adjust;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::Lut1DTransformHandle*>(transform);
    auto t = std::static_pointer_cast<ocio_rs_bridge::RealLut1DTransform>(h->inner)->transform;
    t->setHueAdjust(static_cast<ocio::Lut1DHueAdjust>(hue_adjust));
  } catch (...) {}
#endif
}

// --- MatrixTransform: bit depth & static helpers ---

int ocio_matrix_transform_get_file_input_bit_depth(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform->getFileInputBitDepth());
  } catch (...) { return 0; }
#endif
}

void ocio_matrix_transform_set_file_input_bit_depth(void* transform, int bit_depth) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)bit_depth;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform->setFileInputBitDepth(
        static_cast<ocio::BitDepth>(bit_depth));
  } catch (...) {}
#endif
}

int ocio_matrix_transform_get_file_output_bit_depth(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform->getFileOutputBitDepth());
  } catch (...) { return 0; }
#endif
}

void ocio_matrix_transform_set_file_output_bit_depth(void* transform, int bit_depth) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)bit_depth;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::MatrixTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealMatrixTransform>(h->inner)->transform->setFileOutputBitDepth(
        static_cast<ocio::BitDepth>(bit_depth));
  } catch (...) {}
#endif
}

void* ocio_matrix_transform_create_fit(const char* inputColorSpace, const char* outputColorSpace) {
#ifdef OCIO_RS_STUB
  (void)inputColorSpace; (void)outputColorSpace;
  return new ocio_rs_bridge::MatrixTransformHandle{};
#else
  try {
    auto t = ocio::MatrixTransform::Create();
    if (!t) return nullptr;
    double m44[16] = {0.0};
    double offset4[4] = {0.0};
    // Fit is not available in this OCIO version; fall back to identity
    (void)inputColorSpace; (void)outputColorSpace;
    ocio::MatrixTransform::Identity(m44, offset4);
    t->setMatrix(m44);
    t->setOffset(offset4);
    auto* h = new ocio_rs_bridge::MatrixTransformHandle;
    h->inner = std::make_shared<ocio_rs_bridge::RealMatrixTransform>(ocio_rs_bridge::RealMatrixTransform{t});
    return static_cast<void*>(h);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_matrix_transform_create_identity(void) {
#ifdef OCIO_RS_STUB
  return new ocio_rs_bridge::MatrixTransformHandle{};
#else
  try {
    auto t = ocio::MatrixTransform::Create();
    if (!t) return nullptr;
    double m44[16] = {0.0};
    double offset4[4] = {0.0};
    ocio::MatrixTransform::Identity(m44, offset4);
    t->setMatrix(m44);
    t->setOffset(offset4);
    auto* h = new ocio_rs_bridge::MatrixTransformHandle;
    h->inner = std::make_shared<ocio_rs_bridge::RealMatrixTransform>(ocio_rs_bridge::RealMatrixTransform{t});
    return static_cast<void*>(h);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_matrix_transform_create_sat(double sat, const double* luma) {
#ifdef OCIO_RS_STUB
  (void)sat; (void)luma;
  return new ocio_rs_bridge::MatrixTransformHandle{};
#else
  try {
    auto t = ocio::MatrixTransform::Create();
    if (!t) return nullptr;
    double m44[16] = {0.0};
    double offset4[4] = {0.0};
    ocio::MatrixTransform::Sat(m44, offset4, sat, luma);
    t->setMatrix(m44);
    t->setOffset(offset4);
    auto* h = new ocio_rs_bridge::MatrixTransformHandle;
    h->inner = std::make_shared<ocio_rs_bridge::RealMatrixTransform>(ocio_rs_bridge::RealMatrixTransform{t});
    return static_cast<void*>(h);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_matrix_transform_create_scale(const double* scale) {
#ifdef OCIO_RS_STUB
  (void)scale;
  return new ocio_rs_bridge::MatrixTransformHandle{};
#else
  try {
    auto t = ocio::MatrixTransform::Create();
    if (!t) return nullptr;
    double m44[16] = {0.0};
    double offset4[4] = {0.0};
    ocio::MatrixTransform::Scale(m44, offset4, scale);
    t->setMatrix(m44);
    t->setOffset(offset4);
    auto* h = new ocio_rs_bridge::MatrixTransformHandle;
    h->inner = std::make_shared<ocio_rs_bridge::RealMatrixTransform>(ocio_rs_bridge::RealMatrixTransform{t});
    return static_cast<void*>(h);
  } catch (...) { return nullptr; }
#endif
}

void* ocio_matrix_transform_create_view(int* channels, const char* gamma) {
#ifdef OCIO_RS_STUB
  (void)channels; (void)gamma;
  return new ocio_rs_bridge::MatrixTransformHandle{};
#else
  try {
    auto t = ocio::MatrixTransform::Create();
    if (!t) return nullptr;
    double m44[16] = {0.0};
    double offset4[4] = {0.0};
    // View takes double lumaCoef3 not char* gamma; adapt to available API
    double lumaCoef3[3] = {0.2126, 0.7152, 0.0722};
    (void)gamma;
    ocio::MatrixTransform::View(m44, offset4, channels, lumaCoef3);
    t->setMatrix(m44);
    t->setOffset(offset4);
    auto* h = new ocio_rs_bridge::MatrixTransformHandle;
    h->inner = std::make_shared<ocio_rs_bridge::RealMatrixTransform>(ocio_rs_bridge::RealMatrixTransform{t});
    return static_cast<void*>(h);
  } catch (...) { return nullptr; }
#endif
}

// --- RangeTransform: has/unset value & bit depth ---

bool ocio_range_transform_has_min_in_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->hasMinInValue();
  } catch (...) { return false; }
#endif
}

void ocio_range_transform_unset_min_in_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->unsetMinInValue();
  } catch (...) {}
#endif
}

bool ocio_range_transform_has_max_in_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->hasMaxInValue();
  } catch (...) { return false; }
#endif
}

void ocio_range_transform_unset_max_in_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->unsetMaxInValue();
  } catch (...) {}
#endif
}

bool ocio_range_transform_has_min_out_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->hasMinOutValue();
  } catch (...) { return false; }
#endif
}

void ocio_range_transform_unset_min_out_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->unsetMinOutValue();
  } catch (...) {}
#endif
}

bool ocio_range_transform_has_max_out_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return false;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->hasMaxOutValue();
  } catch (...) { return false; }
#endif
}

void ocio_range_transform_unset_max_out_value(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->unsetMaxOutValue();
  } catch (...) {}
#endif
}

int ocio_range_transform_get_file_input_bit_depth(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->getFileInputBitDepth());
  } catch (...) { return 0; }
#endif
}

void ocio_range_transform_set_file_input_bit_depth(void* transform, int bit_depth) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)bit_depth;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->setFileInputBitDepth(
        static_cast<ocio::BitDepth>(bit_depth));
  } catch (...) {}
#endif
}

int ocio_range_transform_get_file_output_bit_depth(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return 0;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    return static_cast<int>(std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->getFileOutputBitDepth());
  } catch (...) { return 0; }
#endif
}

void ocio_range_transform_set_file_output_bit_depth(void* transform, int bit_depth) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)bit_depth;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::RangeTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealRangeTransform>(h->inner)->transform->setFileOutputBitDepth(
        static_cast<ocio::BitDepth>(bit_depth));
  } catch (...) {}
#endif
}

// --- CDLTransform: SOP & description ---

void ocio_cdl_transform_get_sop(void* transform, double* vec9) {
  vec9[0] = vec9[1] = vec9[2] = 1.0;
  vec9[3] = vec9[4] = vec9[5] = 0.0;
  vec9[6] = vec9[7] = vec9[8] = 1.0;
#ifdef OCIO_RS_STUB
  (void)transform;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->getSOP(vec9);
  } catch (...) {}
#endif
}

void ocio_cdl_transform_set_sop(void* transform, const double* vec9) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)vec9;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->setSOP(vec9);
  } catch (...) {}
#endif
}

const char* ocio_cdl_transform_get_first_sop_description(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform; return nullptr;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    return std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->getFirstSOPDescription();
  } catch (...) { return nullptr; }
#endif
}

void ocio_cdl_transform_set_first_sop_description(void* transform, const char* description) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)description;
#else
  try {
    auto* h = static_cast<ocio_rs_bridge::CDLTransformHandle*>(transform);
    std::static_pointer_cast<ocio_rs_bridge::RealCDLTransform>(h->inner)->transform->setFirstSOPDescription(description);
  } catch (...) {}
#endif
}

}  // extern "C"
