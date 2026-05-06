//! Test utilities for ported OCIO C++ tests.
//!
//! Mirrors the OCIO C++ test framework helpers:
//!   OCIO_CHECK_EQUAL        → assert_eq!
//!   OCIO_CHECK_CLOSE        → assert_close!
//!   OCIO_CHECK_THROW_WHAT   → #[should_panic(expected = "...")]
//!   OCIO_CHECK_NO_THROW     → just call it

use ocio_rs;

/// Check if the build uses the OCIO stub (no real C++ library available).
/// Tests that need real pixel processing must gate on this.
pub fn is_stub() -> bool {
    ocio_rs::is_stub_build()
}

/// Assert two f64 values are within `tol` absolute difference.
#[track_caller]
pub fn assert_close(a: f64, b: f64, tol: f64) {
    let diff = (a - b).abs();
    if diff > tol {
        panic!(
            "assert_close failed: a={}, b={}, diff={}, tol={}",
            a, b, diff, tol
        );
    }
}

/// Assert two f64 slices are element-wise within `tol`.
#[track_caller]
pub fn assert_vec_close(a: &[f64], b: &[f64], tol: f64) {
    assert_eq!(
        a.len(),
        b.len(),
        "assert_vec_close: length mismatch: {} vs {}",
        a.len(),
        b.len()
    );
    for (i, (av, bv)) in a.iter().zip(b.iter()).enumerate() {
        let diff = (av - bv).abs();
        if diff > tol {
            panic!(
                "assert_vec_close failed at index {}: a={}, b={}, diff={}, tol={}",
                i, av, bv, diff, tol
            );
        }
    }
}

/// Assert two 4-element RGBA f64 arrays are element-wise within `tol`.
#[track_caller]
pub fn assert_rgba_close(a: &[f64; 4], b: &[f64; 4], tol: f64) {
    for i in 0..4 {
        let diff = (a[i] - b[i]).abs();
        if diff > tol {
            panic!(
                "assert_rgba_close failed at channel {}: a={}, b={}, diff={}, tol={}",
                i, a[i], b[i], diff, tol
            );
        }
    }
}

/// Assert two f32 slices are element-wise within `tol`.
#[track_caller]
pub fn assert_f32_vec_close(a: &[f32], b: &[f32], tol: f32) {
    assert_eq!(
        a.len(),
        b.len(),
        "assert_f32_vec_close: length mismatch: {} vs {}",
        a.len(),
        b.len()
    );
    for (i, (av, bv)) in a.iter().zip(b.iter()).enumerate() {
        let diff = (av - bv).abs();
        if diff > tol {
            panic!(
                "assert_f32_vec_close failed at index {}: a={}, b={}, diff={}, tol={}",
                i, av, bv, diff, tol
            );
        }
    }
}

/// Construct a minimal built-in config for testing.
/// Returns None in stub builds (no real OCIO to serve config data).
pub fn create_test_config() -> Option<ocio_rs::Config> {
    if is_stub() {
        return None;
    }
    ocio_rs::Config::raw().ok()
}

/// Normalize a f64 value for comparison (handle -0.0 vs 0.0).
pub fn normalize_zero(v: f64) -> f64 {
    if v == 0.0 { 0.0 } else { v }
}
