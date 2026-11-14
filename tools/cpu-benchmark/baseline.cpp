#include <OpenColorIO/OpenColorIO.h>
#include <cstddef>
#include <memory>

namespace OCIO = OCIO_NAMESPACE;

extern "C" {
void* ocio_bench_create() noexcept {
    try {
        const double matrix[16] = {0,0,1,0, 0,1,0,0, 1,0,0,0, 0,0,0,1};
        auto transform = OCIO::MatrixTransform::Create();
        transform->setMatrix(matrix);
        auto cpu = OCIO::Config::CreateRaw()->getProcessor(transform)->getDefaultCPUProcessor();
        return new OCIO::ConstCPUProcessorRcPtr(std::move(cpu));
    } catch (...) { return nullptr; }
}

void ocio_bench_destroy(void* handle) noexcept {
    delete static_cast<OCIO::ConstCPUProcessorRcPtr*>(handle);
}

bool ocio_bench_apply_single(void* handle, float* pixels) noexcept {
    try { (*static_cast<OCIO::ConstCPUProcessorRcPtr*>(handle))->applyRGBA(pixels); return true; }
    catch (...) { return false; }
}

bool ocio_bench_apply_batch(void* handle, float* pixels, long count) noexcept {
    try {
        const auto stride = static_cast<ptrdiff_t>(4 * sizeof(float));
        OCIO::PackedImageDesc img(pixels, count, 1, OCIO::CHANNEL_ORDERING_RGBA,
                                  OCIO::BIT_DEPTH_F32, sizeof(float), stride, stride * count);
        (*static_cast<OCIO::ConstCPUProcessorRcPtr*>(handle))->apply(img);
        return true;
    } catch (...) { return false; }
}

int ocio_bench_compiled_version() noexcept { return OCIO_VERSION_HEX; }
}
