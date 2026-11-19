#include <OpenColorIO/OpenColorIO.h>
#include <memory>
#include <atomic>
#include <cstring>
#include <sstream>
namespace OCIO = OCIO_NAMESPACE;
static std::atomic<unsigned> destroyed{0};
static unsigned calls[7] = {};
static bool sections_match = false;

static std::atomic<unsigned> proxy_destroyed{0};
class NativeProxy final : public OCIO::ConfigIOProxy {
    bool valid_config;
public:
    explicit NativeProxy(bool valid = false) : valid_config(valid) {}
    ~NativeProxy() override { ++proxy_destroyed; }
    std::vector<uint8_t> getLutData(const char* path) const override {
        if (std::strcmp(path, "raise") == 0) throw OCIO::Exception("proxy fixture error");
        return {0, 1, 255};
    }
    std::string getConfigData() const override {
        if (!valid_config) return std::string("cfg\0bytes", 9);
        std::ostringstream stream;
        OCIO::Config::CreateRaw()->serialize(stream);
        return stream.str();
    }
    std::string getFastLutFileHash(const char*) const override { return "native-hash"; }
};
extern "C" void* proxy_fixture_create() {
    return new OCIO::ConfigIOProxyRcPtr(std::make_shared<NativeProxy>());
}
extern "C" void* proxy_fixture_create_config() {
    return new OCIO::ConfigIOProxyRcPtr(std::make_shared<NativeProxy>(true));
}
extern "C" void* proxy_fixture_empty() { return new OCIO::ConfigIOProxyRcPtr(); }
extern "C" void proxy_fixture_destroy(void* ptr) {
    delete static_cast<OCIO::ConfigIOProxyRcPtr*>(ptr);
}
extern "C" unsigned proxy_fixture_destroyed() { return proxy_destroyed.load(); }

// Deliberately not a GpuShaderDesc. It delegates storage to an OCIO descriptor
// solely to keep this test fixture small; its creator virtuals are independent.
class Creator final : public OCIO::GpuShaderCreator, public std::enable_shared_from_this<Creator> {
    OCIO::GpuShaderDescRcPtr storage = OCIO::GpuShaderDesc::CreateShaderDesc();
    bool alias_clone;
public:
    explicit Creator(bool alias = false) : alias_clone(alias) {}
    ~Creator() override { ++destroyed; }
    void begin(const char* uid) override { ++calls[0]; OCIO::GpuShaderCreator::begin(uid); }
    void end() override { ++calls[1]; OCIO::GpuShaderCreator::end(); }
    void finalize() override { ++calls[2]; OCIO::GpuShaderCreator::finalize(); }
    void addToFunctionShaderCode(const char* code) override {
        if (std::strcmp(code, "reject") == 0) throw OCIO::Exception("fixture rejected shader code");
        ++calls[3]; OCIO::GpuShaderCreator::addToFunctionShaderCode(code);
    }
#if OCIO_VERSION_HEX >= 0x02050000
    void createShaderText(const char* p,const char* t,const char* h,const char* f,const char* b,const char* e) override {
        sections_match = std::strcmp(p,"params") == 0 && std::strcmp(t,"textures") == 0 &&
                         std::strcmp(h,"helpers") == 0 && std::strcmp(f,"header") == 0 &&
                         std::strcmp(b,"body") == 0 && std::strcmp(e,"footer") == 0;
        ++calls[4]; OCIO::GpuShaderCreator::createShaderText(p,t,h,f,b,e);
    }
#else
    void createShaderText(const char* d,const char* h,const char* f,const char* b,const char* e) override {
        sections_match = std::strcmp(d,"paramstextures") == 0 && std::strcmp(h,"helpers") == 0 &&
                         std::strcmp(f,"header") == 0 && std::strcmp(b,"body") == 0 &&
                         std::strcmp(e,"footer") == 0;
        ++calls[4]; OCIO::GpuShaderCreator::createShaderText(d,h,f,b,e);
    }
#endif
    OCIO::GpuShaderCreatorRcPtr clone() const override {
        if (alias_clone) return std::const_pointer_cast<Creator>(shared_from_this());
        auto out = std::make_shared<Creator>();
        out->setFunctionName(getFunctionName());
        out->setLanguage(getLanguage());
        out->setTextureMaxWidth(getTextureMaxWidth());
        return out;
    }
    void setTextureMaxWidth(unsigned x) override { storage->setTextureMaxWidth(x); }
    unsigned getTextureMaxWidth() const noexcept override { return storage->getTextureMaxWidth(); }
    void setAllowTexture1D(bool x) override { storage->setAllowTexture1D(x); }
    bool getAllowTexture1D() const override { return storage->getAllowTexture1D(); }
    bool addUniform(const char* n, const DoubleGetter& v) override { return storage->addUniform(n,v); }
    bool addUniform(const char* n, const BoolGetter& v) override { return storage->addUniform(n,v); }
    bool addUniform(const char* n, const Float3Getter& v) override { return storage->addUniform(n,v); }
#if OCIO_VERSION_HEX >= 0x02050000
    bool addUniform(const char* n, const SizeGetter& s, const VectorFloatGetter& v, unsigned m) override { return storage->addUniform(n,s,v,m); }
    bool addUniform(const char* n, const SizeGetter& s, const VectorIntGetter& v, unsigned m) override { return storage->addUniform(n,s,v,m); }
    unsigned addTexture(const char* n,const char* s,unsigned w,unsigned h,TextureType c,TextureDimensions d,OCIO::Interpolation i,const float* v) override { ++calls[5]; return storage->addTexture(n,s,w,h,c,d,i,v); }
    unsigned add3DTexture(const char* n,const char* s,unsigned w,OCIO::Interpolation i,const float* v) override { ++calls[6]; return storage->add3DTexture(n,s,w,i,v); }
#else
    bool addUniform(const char* n, const SizeGetter& s, const VectorFloatGetter& v) override { return storage->addUniform(n,s,v); }
    bool addUniform(const char* n, const SizeGetter& s, const VectorIntGetter& v) override { return storage->addUniform(n,s,v); }
    void addTexture(const char* n,const char* s,unsigned w,unsigned h,TextureType c,TextureDimensions d,OCIO::Interpolation i,const float* v) override { ++calls[5]; storage->addTexture(n,s,w,h,c,d,i,v); }
    void add3DTexture(const char* n,const char* s,unsigned w,OCIO::Interpolation i,const float* v) override { ++calls[6]; storage->add3DTexture(n,s,w,i,v); }
#endif
};

static std::atomic<unsigned> custom_rgb_copies{0};
class CustomRgbCurve final : public OCIO::GradingRGBCurve {
    OCIO::GradingRGBCurveRcPtr inner = OCIO::GradingRGBCurve::Create(OCIO::GRADING_LOG);
public:
    OCIO::GradingRGBCurveRcPtr createEditableCopy() const override { ++custom_rgb_copies; return inner->createEditableCopy(); }
    void validate() const override { inner->validate(); }
    bool isIdentity() const override { return inner->isIdentity(); }
    OCIO::ConstGradingBSplineCurveRcPtr getCurve(OCIO::RGBCurveType c) const override { return inner->getCurve(c); }
    OCIO::GradingBSplineCurveRcPtr getCurve(OCIO::RGBCurveType c) override { return inner->getCurve(c); }
};

class NullRgbProperty final : public OCIO::DynamicProperty, public OCIO::DynamicPropertyGradingRGBCurve {
    OCIO::ConstGradingRGBCurveRcPtr value;
public:
    explicit NullRgbProperty(bool valid = false) {
        if (valid) value = OCIO::GradingRGBCurve::Create(OCIO::GRADING_LOG);
    }
    explicit NullRgbProperty(OCIO::ConstGradingRGBCurveRcPtr v) : value(std::move(v)) {}
    OCIO::DynamicPropertyType getType() const noexcept override { return OCIO::DYNAMIC_PROPERTY_GRADING_RGBCURVE; }
    const OCIO::ConstGradingRGBCurveRcPtr& getValue() const override { return value; }
    void setValue(const OCIO::ConstGradingRGBCurveRcPtr& v) override { value = v; }
};

extern "C" void* creator_fixture_custom_rgb() noexcept {
    try {
        custom_rgb_copies = 0;
        auto creator = std::make_shared<Creator>();
        OCIO::DynamicPropertyRcPtr property = std::make_shared<NullRgbProperty>(std::make_shared<CustomRgbCurve>());
        creator->addDynamicProperty(property);
        return new OCIO::GpuShaderCreatorRcPtr(std::move(creator));
    } catch (...) { return nullptr; }
}
extern "C" unsigned creator_fixture_custom_rgb_copies() noexcept { return custom_rgb_copies.load(); }

extern "C" void* creator_fixture_valid_rgb() noexcept {
    try {
        auto creator = std::make_shared<Creator>();
        OCIO::DynamicPropertyRcPtr property = std::make_shared<NullRgbProperty>(true);
        creator->addDynamicProperty(property);
        return new OCIO::GpuShaderCreatorRcPtr(std::move(creator));
    } catch (...) { return nullptr; }
}

class UnknownProperty final : public OCIO::DynamicProperty {
public:
    OCIO::DynamicPropertyType getType() const noexcept override {
        return static_cast<OCIO::DynamicPropertyType>(7);
    }
};

extern "C" void* creator_fixture_unknown_property() noexcept {
    try {
        auto creator = std::make_shared<Creator>();
        OCIO::DynamicPropertyRcPtr property = std::make_shared<UnknownProperty>();
        creator->addDynamicProperty(property);
        return new OCIO::GpuShaderCreatorRcPtr(std::move(creator));
    } catch (...) { return nullptr; }
}

#if OCIO_VERSION_HEX >= 0x02050000
static std::atomic<unsigned> custom_hue_copies{0};
class CustomHueCurve final : public OCIO::GradingHueCurve {
    OCIO::GradingHueCurveRcPtr inner = OCIO::GradingHueCurve::Create(OCIO::GRADING_LOG);
public:
    CustomHueCurve() { inner->setDrawCurveOnly(true); }
    OCIO::GradingHueCurveRcPtr createEditableCopy() const override { ++custom_hue_copies; return inner->createEditableCopy(); }
    void validate() const override { inner->validate(); }
    bool isIdentity() const override { return inner->isIdentity(); }
    bool getDrawCurveOnly() const override { return inner->getDrawCurveOnly(); }
    void setDrawCurveOnly(bool value) override { inner->setDrawCurveOnly(value); }
    OCIO::ConstGradingBSplineCurveRcPtr getCurve(OCIO::HueCurveType c) const override { return inner->getCurve(c); }
    OCIO::GradingBSplineCurveRcPtr getCurve(OCIO::HueCurveType c) override { return inner->getCurve(c); }
};

class NullHueProperty final : public OCIO::DynamicProperty, public OCIO::DynamicPropertyGradingHueCurve {
    OCIO::ConstGradingHueCurveRcPtr value;
public:
    explicit NullHueProperty(OCIO::ConstGradingHueCurveRcPtr v = {}) : value(std::move(v)) {}
    OCIO::DynamicPropertyType getType() const noexcept override { return OCIO::DYNAMIC_PROPERTY_GRADING_HUECURVE; }
    const OCIO::ConstGradingHueCurveRcPtr& getValue() const override { return value; }
    void setValue(const OCIO::ConstGradingHueCurveRcPtr& v) override { value = v; }
};

extern "C" void* creator_fixture_custom_hue() noexcept {
    try {
        custom_hue_copies = 0;
        auto creator = std::make_shared<Creator>();
        OCIO::DynamicPropertyRcPtr property = std::make_shared<NullHueProperty>(std::make_shared<CustomHueCurve>());
        creator->addDynamicProperty(property);
        return new OCIO::GpuShaderCreatorRcPtr(std::move(creator));
    } catch (...) { return nullptr; }
}
extern "C" unsigned creator_fixture_custom_hue_copies() noexcept { return custom_hue_copies.load(); }

extern "C" void* creator_fixture_null_hue() noexcept {
    try {
        auto creator = std::make_shared<Creator>();
        OCIO::DynamicPropertyRcPtr property = std::make_shared<NullHueProperty>();
        creator->addDynamicProperty(property);
        return new OCIO::GpuShaderCreatorRcPtr(std::move(creator));
    } catch (...) { return nullptr; }
}
#else
extern "C" void* creator_fixture_null_hue() noexcept { return nullptr; }
#endif

extern "C" void* creator_fixture_null_rgb() noexcept {
    try {
        auto creator = std::make_shared<Creator>();
        OCIO::DynamicPropertyRcPtr property = std::make_shared<NullRgbProperty>();
        creator->addDynamicProperty(property);
        return new OCIO::GpuShaderCreatorRcPtr(std::move(creator));
    } catch (...) { return nullptr; }
}

extern "C" void* creator_fixture_create() noexcept {
    try { return new OCIO::GpuShaderCreatorRcPtr(std::make_shared<Creator>()); }
    catch (...) { return nullptr; }
}
extern "C" void* creator_fixture_create_alias() noexcept {
    try { return new OCIO::GpuShaderCreatorRcPtr(std::make_shared<Creator>(true)); }
    catch (...) { return nullptr; }
}
extern "C" void creator_fixture_destroy(void* ptr) noexcept {
    delete static_cast<OCIO::GpuShaderCreatorRcPtr*>(ptr);
}
extern "C" int creator_fixture_version() noexcept { return OCIO_VERSION_HEX; }
extern "C" unsigned creator_fixture_destroyed() noexcept { return destroyed.load(); }
extern "C" void creator_fixture_reset_destroyed() noexcept { destroyed.store(0); }
extern "C" unsigned creator_fixture_calls(unsigned index) noexcept { return index < 7 ? calls[index] : 0; }
extern "C" bool creator_fixture_sections_match() noexcept { return sections_match; }

extern "C" void* image_fixture_rgba(float* data) noexcept {
    try { return static_cast<OCIO::ImageDesc*>(new OCIO::PackedImageDesc(data, 1, 1, 4)); }
    catch (...) { return nullptr; }
}

extern "C" void* image_fixture_planar(float* data) noexcept {
    try { return static_cast<OCIO::ImageDesc*>(new OCIO::PlanarImageDesc(data, data + 4, data + 8, data + 12, 1, 1)); }
    catch (...) { return nullptr; }
}

// This irregular BGR layout cannot be represented by PackedImageDesc's
// single channel stride or PlanarImageDesc's separate planes.
class CustomImageDesc final : public OCIO::ImageDesc {
    float* data;
    int failure;
public:
    explicit CustomImageDesc(float* pixels, int failureMode = 0)
        : data(pixels), failure(failureMode) {}
    void* getRData() const override { return data + 3; }
    void* getGData() const override { return data + 1; }
    void* getBData() const override { return data; }
    void* getAData() const override { return nullptr; }
    OCIO::BitDepth getBitDepth() const override { return OCIO::BIT_DEPTH_F32; }
    long getWidth() const override {
        if (failure == 1) throw OCIO::Exception("");
        if (failure == 2) throw OCIO::ExceptionMissingFile("");
        return 2;
    }
    long getHeight() const override { return 1; }
    ptrdiff_t getXStrideBytes() const override { return 5 * sizeof(float); }
    ptrdiff_t getYStrideBytes() const override { return 10 * sizeof(float); }
    bool isRGBAPacked() const override { return false; }
    bool isFloat() const override { return true; }
};

extern "C" void* image_fixture_custom(float* data) noexcept {
    try { return static_cast<OCIO::ImageDesc*>(new CustomImageDesc(data)); }
    catch (...) { return nullptr; }
}

extern "C" void* image_fixture_throwing(float* data, int failure) noexcept {
    try { return static_cast<OCIO::ImageDesc*>(new CustomImageDesc(data, failure)); }
    catch (...) { return nullptr; }
}

extern "C" void image_fixture_destroy(void* ptr) noexcept {
    delete static_cast<OCIO::ImageDesc*>(ptr);
}
