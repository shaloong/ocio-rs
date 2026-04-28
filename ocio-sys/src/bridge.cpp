#include "bridge.hpp"

#include <memory>
#include <stdexcept>
#include <string>

#ifndef OCIO_RS_STUB
#include <OpenColorIO/OpenColorIO.h>
#endif

namespace ocio_rs_bridge {

struct ConfigHandle {
  std::shared_ptr<void> inner;
};

struct ProcessorHandle {
  std::shared_ptr<void> inner;
};

#ifdef OCIO_RS_STUB

struct StubConfig {};
struct StubProcessor {};

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

#else

namespace ocio = OCIO_NAMESPACE;

struct RealConfig {
  ocio::ConstConfigRcPtr config;
};

struct RealProcessor {
  ocio::ConstProcessorRcPtr processor;
  ocio::ConstCPUProcessorRcPtr cpu;
};

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

static std::unique_ptr<ProcessorHandle> make_real_processor(void* config_handle, const char* src, const char* dst) {
  try {
    auto* config = static_cast<ConfigHandle*>(config_handle);
    auto handle = std::make_unique<ProcessorHandle>();
    auto processor = std::make_shared<RealProcessor>();
    auto real_config = std::static_pointer_cast<RealConfig>(config->inner);
    processor->processor = real_config->config->getProcessor(src, dst);
    processor->cpu = processor->processor->getDefaultCPUProcessor();
    handle->inner = processor;
    return handle;
  } catch (...) {
    return nullptr;
  }
}

#endif

}  // namespace ocio_rs_bridge

extern "C" {

bool ocio_runtime_is_stub(void) {
#ifdef OCIO_RS_STUB
  return true;
#else
  return false;
#endif
}

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
  if (!handle) {
    return nullptr;
  }
  auto* real_config = std::static_pointer_cast<ocio_rs_bridge::RealConfig>(handle->inner).get();
  if (!real_config || !real_config->config) {
    return nullptr;
  }
  return handle.release();
#endif
}

void* ocio_config_get_processor(void* config, const char* src, const char* dst) {
#ifdef OCIO_RS_STUB
  (void)config;
  (void)src;
  (void)dst;
  return nullptr;
#else
  auto handle = ocio_rs_bridge::make_real_processor(config, src, dst);
  if (!handle) {
    return nullptr;
  }
  auto* real_processor = std::static_pointer_cast<ocio_rs_bridge::RealProcessor>(handle->inner).get();
  if (!real_processor || !real_processor->processor || !real_processor->cpu) {
    return nullptr;
  }
  return handle.release();
#endif
}

void ocio_processor_apply_rgba(void* processor, float* rgba, size_t len) {
  if (len != 4) {
    return;
  }
#ifdef OCIO_RS_STUB
  (void)processor;
  (void)rgba;
#else
  try {
    auto* handle = static_cast<ocio_rs_bridge::ProcessorHandle*>(processor);
    auto real_processor = std::static_pointer_cast<ocio_rs_bridge::RealProcessor>(handle->inner);
    real_processor->cpu->applyRGBA(rgba);
  } catch (...) {
    return;
  }
#endif
}

void ocio_config_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ConfigHandle*>(handle);
}

void ocio_processor_destroy(void* handle) {
  delete static_cast<ocio_rs_bridge::ProcessorHandle*>(handle);
}

}  // extern "C"
