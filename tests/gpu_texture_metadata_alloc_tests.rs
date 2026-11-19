//! Keep metadata-only queries independent of the size of the texel payload.
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct TrackingAllocator;

thread_local! {
    static TRACK_LARGE: Cell<bool> = const { Cell::new(false) };
    static LARGE_ALLOCS: Cell<usize> = const { Cell::new(0) };
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if layout.size() >= 4096 && TRACK_LARGE.try_with(Cell::get).unwrap_or(false) {
            let _ = LARGE_ALLOCS.try_with(|count| count.set(count.get() + 1));
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator;

#[test]
#[cfg_attr(ocio_stub, ignore = "requires native OpenColorIO")]
fn metadata_queries_do_not_allocate_texel_sized_buffers() {
    use ocio_rs::{GpuShaderDesc, GpuTextureChannel, GpuTextureDimensions, Interpolation};

    let desc = GpuShaderDesc::create().unwrap();
    let texels_2d = vec![0.25; 2048 * 3];
    let texels_3d = vec![0.5; 32 * 32 * 32 * 3];
    let uniform_values = vec![0.75; 8192];
    assert!(desc
        .add_uniform_f32_array("metadata_uniform", &uniform_values, 8192)
        .unwrap());
    let binding_2d = desc
        .add_texture_2d(
            "metadata2d",
            "sampler2d",
            2048,
            1,
            GpuTextureChannel::Rgb,
            GpuTextureDimensions::Texture1D,
            Interpolation::Linear,
            &texels_2d,
        )
        .unwrap();
    let binding_3d = desc
        .add_texture_3d(
            "metadata3d",
            "sampler3d",
            32,
            Interpolation::Linear,
            &texels_3d,
        )
        .unwrap();

    LARGE_ALLOCS.with(|count| count.set(0));
    TRACK_LARGE.with(|tracking| tracking.set(true));
    let info = desc.texture_info(0);
    let actual_2d = desc.texture_shader_binding_index(0);
    let actual_3d = desc.texture_3d_shader_binding_index(0);
    let uniform_name = desc.uniform_name(0);
    TRACK_LARGE.with(|tracking| tracking.set(false));

    assert_eq!(LARGE_ALLOCS.with(Cell::get), 0);
    assert_eq!(info.unwrap().width, 2048);
    assert_eq!(actual_2d, binding_2d);
    assert_eq!(actual_3d, binding_3d);
    assert_eq!(uniform_name.as_deref(), Some("metadata_uniform"));

    // Positive control: the full-resource APIs really do allocate payloads.
    LARGE_ALLOCS.with(|count| count.set(0));
    TRACK_LARGE.with(|tracking| tracking.set(true));
    let full_2d = desc.texture_2d(0);
    let full_3d = desc.texture_3d(0);
    let full_uniform = desc.uniform(0);
    TRACK_LARGE.with(|tracking| tracking.set(false));
    assert!(LARGE_ALLOCS.with(Cell::get) >= 3);
    assert_eq!(full_2d.unwrap().values, texels_2d);
    assert_eq!(full_3d.unwrap().values, texels_3d);
    assert!(full_uniform.is_some());
}
