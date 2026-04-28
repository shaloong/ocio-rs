#pragma once

#include <stddef.h>
#include <stdbool.h>

extern "C" {

void* ocio_config_create_raw(void);
void* ocio_config_create_from_file(const char* path);
void* ocio_config_get_processor(void* config, const char* src, const char* dst);
void ocio_processor_apply_rgba(void* processor, float* rgba, size_t len);
bool ocio_runtime_is_stub(void);
void ocio_config_destroy(void* handle);
void ocio_processor_destroy(void* handle);

}
