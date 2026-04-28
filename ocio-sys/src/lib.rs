use std::ffi::c_void;

unsafe extern "C" {
    pub fn ocio_runtime_is_stub() -> bool;
    pub fn ocio_config_create_raw() -> *mut c_void;
    pub fn ocio_config_create_from_file(path: *const i8) -> *mut c_void;
    pub fn ocio_config_get_processor(
        config: *mut c_void,
        src: *const i8,
        dst: *const i8,
    ) -> *mut c_void;
    pub fn ocio_processor_apply_rgba(processor: *mut c_void, rgba: *mut f32, len: usize);
    pub fn ocio_config_destroy(handle: *mut c_void);
    pub fn ocio_processor_destroy(handle: *mut c_void);
}
