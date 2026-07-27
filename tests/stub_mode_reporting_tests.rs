//! Guards the `ocio_stub` cfg against drifting from the real build state.
//!
//! Tests that cannot assert anything without a real OpenColorIO are marked
//! `#[cfg_attr(ocio_stub, ignore)]` so a stub run reports them as ignored
//! rather than as passing. That is only honest while the cfg agrees with what
//! `ocio-sys` actually linked.

#[test]
fn ocio_stub_cfg_matches_the_runtime_stub_query() {
    assert_eq!(
        cfg!(ocio_stub),
        ocio_rs::is_stub_build(),
        "the ocio_stub cfg and is_stub_build() disagree; stub-gated tests would be \
         reported incorrectly"
    );
}
