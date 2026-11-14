use std::ffi::c_void;

unsafe extern "C" {
    pub fn ocio_grading_bspline_curve_create(count: usize) -> *mut c_void;
    pub fn ocio_grading_bspline_curve_create_from_points(
        xy: *const [f32; 2],
        count: usize,
        kind: i32,
        mode: i32,
    ) -> *mut c_void;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_bspline_curve_create_with_spline_type(
        count: usize,
        kind: i32,
    ) -> *mut c_void;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_bspline_curve_create_with_hue_type(count: usize, kind: i32) -> *mut c_void;
    pub fn ocio_grading_bspline_curve_destroy(h: *mut c_void);
    pub fn ocio_grading_bspline_curve_create_editable_copy(h: *mut c_void) -> *mut c_void;
    pub fn ocio_grading_bspline_curve_get_num_control_points(h: *mut c_void) -> usize;
    pub fn ocio_grading_bspline_curve_set_num_control_points(h: *mut c_void, count: usize);
    pub fn ocio_grading_bspline_curve_get_control_point(
        h: *mut c_void,
        index: usize,
        x: *mut f32,
        y: *mut f32,
    );
    pub fn ocio_grading_bspline_curve_set_control_point(
        h: *mut c_void,
        index: usize,
        x: f32,
        y: f32,
    );
    pub fn ocio_grading_bspline_curve_get_slope(h: *mut c_void, index: usize) -> f32;
    pub fn ocio_grading_bspline_curve_set_slope(h: *mut c_void, index: usize, slope: f32);
    pub fn ocio_grading_bspline_curve_slopes_are_default(h: *mut c_void) -> bool;
    pub fn ocio_grading_bspline_curve_validate(h: *mut c_void);
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_bspline_curve_get_spline_type(h: *mut c_void) -> i32;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_bspline_curve_set_spline_type(h: *mut c_void, kind: i32);
    pub fn ocio_grading_bspline_curve_equals(h: *mut c_void, other: *mut c_void) -> bool;
    pub fn ocio_grading_rgb_curve_object_create(style: i32) -> *mut c_void;
    pub fn ocio_grading_rgb_curve_object_destroy(h: *mut c_void);
    pub fn ocio_grading_rgb_curve_object_create_editable_copy(h: *mut c_void) -> *mut c_void;
    pub fn ocio_grading_rgb_curve_object_validate(h: *mut c_void);
    pub fn ocio_grading_rgb_curve_object_is_identity(h: *mut c_void) -> bool;
    pub fn ocio_grading_rgb_curve_object_get_curve(h: *mut c_void, kind: i32) -> *mut c_void;
    pub fn ocio_grading_rgb_curve_object_equals(h: *mut c_void, other: *mut c_void) -> bool;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_create(style: i32) -> *mut c_void;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_destroy(h: *mut c_void);
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_create_editable_copy(h: *mut c_void) -> *mut c_void;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_validate(h: *mut c_void);
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_is_identity(h: *mut c_void) -> bool;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_get_curve(h: *mut c_void, kind: i32) -> *mut c_void;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_equals(h: *mut c_void, other: *mut c_void) -> bool;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_get_draw_curve_only(h: *mut c_void) -> bool;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_set_draw_curve_only(h: *mut c_void, value: bool);
    pub fn ocio_grading_rgb_curve_object_from_transform(transform: *mut c_void) -> *mut c_void;
    pub fn ocio_grading_rgb_curve_object_set_transform(h: *mut c_void, transform: *mut c_void);
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_from_transform(transform: *mut c_void) -> *mut c_void;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_set_transform(h: *mut c_void, transform: *mut c_void);
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_get_b_spline_type_for_hue_curve_type(kind: i32) -> i32;
    pub fn ocio_grading_rgb_curve_object_create_from_curves(
        curves: *const *mut c_void,
    ) -> *mut c_void;
    #[cfg(feature = "v2_5")]
    pub fn ocio_grading_hue_curve_object_create_from_curves(
        curves: *const *mut c_void,
    ) -> *mut c_void;
    pub fn ocio_dynamic_property_grading_rgb_curve_object_get(h: *mut c_void) -> *mut c_void;
    pub fn ocio_dynamic_property_grading_rgb_curve_object_set(h: *mut c_void, value: *mut c_void);
    #[cfg(feature = "v2_5")]
    pub fn ocio_dynamic_property_grading_hue_curve_object_get(h: *mut c_void) -> *mut c_void;
    #[cfg(feature = "v2_5")]
    pub fn ocio_dynamic_property_grading_hue_curve_object_set(h: *mut c_void, value: *mut c_void);
}
