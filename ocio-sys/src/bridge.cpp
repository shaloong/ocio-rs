#include "bridge.hpp"

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
struct ExposureContrastTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 7; }
#ifndef OCIO_RS_STUB
  ocio::TransformRcPtr get_ocio_transform() override;
#endif
};
struct ColorSpaceTransformHandle : TransformHandleBase { std::shared_ptr<void> inner;
  int get_transform_type_tag() const override { return 4; }
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
struct StubMatrixTransform {};
struct StubLogTransform {};
struct StubRangeTransform {};
struct StubGroupTransform {};
struct StubBaker {};
struct StubContext {};
struct StubColorSpace {};

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

// --- Transform base ---

int ocio_transform_get_direction(void* transform) {
#ifdef OCIO_RS_STUB
  (void)transform;
  return 0;
#else
  return 0;
#endif
}

void ocio_transform_set_direction(void* transform, int direction) {
#ifdef OCIO_RS_STUB
  (void)transform; (void)direction;
#endif
}

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

void ocio_transform_destroy(void* handle) {
  // Base transform destruction handled by subclass-specific destroy functions
  (void)handle;
}

// --- Config destruction ---

void ocio_config_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ConfigHandle*>(handle);
}

// --- Processor destruction ---

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

void ocio_color_space_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ColorSpaceHandle*>(handle);
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

void ocio_look_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::LookHandle*>(handle);
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

void ocio_view_transform_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ViewTransformHandle*>(handle);
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

}  // extern "C"
