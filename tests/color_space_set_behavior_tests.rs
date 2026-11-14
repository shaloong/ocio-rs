//! ColorSpaceSet behavioral tests against real OCIO.
//!
//! In stub mode these tests return early after the unit-level smoke tests in
//! `src/color_space_set.rs`. In bundled/real mode they validate set mutation,
//! editable-copy independence, and config category-filter behavior.

mod common;
use common::*;

use std::collections::BTreeSet;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ocio_rs::{ColorSpace, ColorSpaceSet, Config};

fn color_space_set_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn named_color_space(name: &str, categories: &[&str]) -> ColorSpace {
    let cs = ColorSpace::create().expect("color space create");
    cs.set_name(name).expect("set color space name");
    cs.set_description("color space set behavior test")
        .expect("set description");
    for category in categories {
        cs.add_category(category).expect("add category");
    }
    cs
}

fn set_names(set: &ColorSpaceSet) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for index in 0..set.num_color_spaces() {
        let name = set
            .color_space_name_by_index(index)
            .expect("color space name by index");
        names.insert(name);
    }
    names
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn native_set_operations_preserve_results_and_name_equality() {
    let left = ColorSpaceSet::create().unwrap();
    let right = ColorSpaceSet::create().unwrap();
    left.add_color_space(&named_color_space("A", &[]));
    left.add_color_space(&named_color_space("B", &[]));
    let replacement = named_color_space("B", &[]);
    replacement.set_description("right side").unwrap();
    right.add_color_space(&replacement);
    right.add_color_space(&named_color_space("C", &[]));
    let union = left.union(&right).unwrap();
    let intersection = left.intersection(&right).unwrap();
    let difference = left.difference(&right).unwrap();
    assert_eq!(union.try_num_color_spaces().unwrap(), 3);
    let copied_b = union.copy_color_space("B").unwrap().unwrap();
    assert_eq!(copied_b.description().as_deref(), Some("right side"));
    copied_b.set_description("changed copy").unwrap();
    assert_eq!(
        union
            .copy_color_space("B")
            .unwrap()
            .unwrap()
            .description()
            .as_deref(),
        Some("right side")
    );
    assert!(union.copy_color_space("missing").unwrap().is_none());
    assert!(union.copy_color_space_by_index(-1).unwrap().is_none());
    assert!(union.copy_color_space_by_index(0).unwrap().is_some());
    assert_eq!(
        union
            .create_editable_copy()
            .unwrap()
            .color_space("B")
            .unwrap()
            .description()
            .as_deref(),
        Some("right side")
    );
    assert_eq!(intersection.try_num_color_spaces().unwrap(), 1);
    assert_eq!(
        intersection
            .try_color_space_name_by_index(0)
            .unwrap()
            .as_deref(),
        Some("B")
    );
    assert_eq!(
        difference
            .try_color_space_name_by_index(0)
            .unwrap()
            .as_deref(),
        Some("A")
    );
    assert!(!left.try_equals(&right).unwrap());
    assert!(left
        .try_equals(&left.create_editable_copy().unwrap())
        .unwrap());
    let same_names = ColorSpaceSet::create().unwrap();
    same_names.add_color_space(&named_color_space("A", &[]));
    same_names.add_color_space(&replacement);
    assert!(left.try_equals(&same_names).unwrap());
    assert!(intersection
        .union(&difference)
        .unwrap()
        .try_equals(&left)
        .unwrap());
    left.clear_color_spaces();
    right.clear_color_spaces();
    assert_eq!(union.try_num_color_spaces().unwrap(), 3);
    let editable = union.create_editable_copy().unwrap();
    editable.remove_color_space("A").unwrap();
    assert_eq!(union.try_num_color_spaces().unwrap(), 3);
    assert_eq!(editable.num_color_spaces(), 2);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn set_color_space_lookups_return_independent_editable_copies() {
    let set = ColorSpaceSet::create().unwrap();
    set.add_color_space(&named_color_space("Original", &[]));
    let by_name = set.color_space("Original").unwrap();
    let by_index = set.color_space_by_index(0).unwrap();
    by_name.set_name("Changed").unwrap();
    by_index.set_description("changed copy").unwrap();
    assert!(set.has_color_space("Original"));
    assert!(!set.has_color_space("Changed"));
    assert_ne!(
        set.color_space("Original")
            .unwrap()
            .description()
            .as_deref(),
        Some("changed copy")
    );
    set.add_color_space(&by_index);
    assert_eq!(
        set.color_space("Original")
            .unwrap()
            .description()
            .as_deref(),
        Some("changed copy")
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_color_space_lookup_requires_explicit_registration_for_edits() {
    let config = Config::raw().unwrap();
    let original = named_color_space("Original", &[]);
    config.try_add_color_space(&original).unwrap();
    let copy = config.color_space("Original").unwrap();
    copy.set_description("edited copy").unwrap();
    assert_ne!(
        config
            .color_space("Original")
            .unwrap()
            .description()
            .as_deref(),
        Some("edited copy")
    );
    config.try_add_color_space(&copy).unwrap();
    assert_eq!(
        config
            .color_space("Original")
            .unwrap()
            .description()
            .as_deref(),
        Some("edited copy")
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn const_color_spaces_survive_parent_drop_and_copy_independently() {
    let config = Config::raw().unwrap();
    config
        .try_add_color_space(&named_color_space("Stable", &[]))
        .unwrap();
    let view = config.try_const_color_space("Stable").unwrap().unwrap();
    let set = ColorSpaceSet::create().unwrap();
    set.add_color_space(&named_color_space("Other", &[]));
    let set_view = set.try_const_color_space("Other").unwrap().unwrap();
    drop(config);
    drop(set);
    assert_eq!(view.name().unwrap().to_str().unwrap(), "Stable");
    assert_eq!(set_view.name().unwrap().to_str().unwrap(), "Other");
    assert!(!view.description().unwrap().to_bytes().is_empty());
    let editable = view.create_editable_copy().unwrap();
    editable.set_name("Changed").unwrap();
    assert_eq!(view.name().unwrap().to_str().unwrap(), "Stable");
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn color_space_set_mutation_and_copy_behavior() {
    let _guard = color_space_set_test_lock();
    if is_stub() {
        return;
    }

    let set = ColorSpaceSet::create().expect("color space set create");
    let a = named_color_space("UnitColorSpaceSetA", &["unit_category_a"]);
    let b = named_color_space("UnitColorSpaceSetB", &["unit_category_b"]);
    let c = named_color_space("UnitColorSpaceSetC", &["unit_category_c"]);

    assert_eq!(set.num_color_spaces(), 0);

    set.add_color_space(&a);
    set.add_color_space(&b);
    assert_eq!(set.num_color_spaces(), 2);
    assert!(set.has_color_space("UnitColorSpaceSetA"));
    assert!(set.has_color_space("UnitColorSpaceSetB"));
    assert_eq!(set.color_space_index("UnitColorSpaceSetA"), 0);
    assert_eq!(
        set.try_color_space("UnitColorSpaceSetB")
            .expect("color-space lookup")
            .and_then(|cs| cs.name())
            .as_deref(),
        Some("UnitColorSpaceSetB")
    );
    assert_eq!(
        set.try_color_space_name_by_index(0)
            .expect("color-space name query")
            .as_deref(),
        Some("UnitColorSpaceSetA")
    );
    assert!(set.try_color_space("UnitColorSpaceSetA\0").is_err());
    assert_eq!(
        set_names(&set),
        BTreeSet::from([
            String::from("UnitColorSpaceSetA"),
            String::from("UnitColorSpaceSetB"),
        ])
    );

    let other = ColorSpaceSet::create().expect("other color space set create");
    other.add_color_space(&c);
    set.add_color_spaces(&other);
    assert_eq!(set.num_color_spaces(), 3);
    assert!(set.has_color_space("UnitColorSpaceSetC"));

    let copy = set.create_editable_copy().expect("editable copy");
    copy.remove_color_space("UnitColorSpaceSetB")
        .expect("remove from copy");
    assert_eq!(copy.num_color_spaces(), 2);
    assert!(!copy.has_color_space("UnitColorSpaceSetB"));
    assert!(set.has_color_space("UnitColorSpaceSetB"));
    assert_eq!(set.num_color_spaces(), 3);

    set.try_remove_color_spaces(&other)
        .expect("remove color spaces from set");
    assert_eq!(set.num_color_spaces(), 2);
    assert!(!set.has_color_space("UnitColorSpaceSetC"));

    set.remove_color_space("UnitColorSpaceSetA")
        .expect("remove color space A");
    assert_eq!(set.num_color_spaces(), 1);
    assert!(set.has_color_space("UnitColorSpaceSetB"));

    set.try_clear_color_spaces()
        .expect("clear color spaces from set");
    assert_eq!(set.num_color_spaces(), 0);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_color_space_set_category_filter_behavior() {
    let _guard = color_space_set_test_lock();
    if is_stub() {
        return;
    }

    let config = Config::raw().expect("raw config");
    let display_a = named_color_space("UnitDisplayCategoryA", &["unit_display"]);
    let display_b = named_color_space("UnitDisplayCategoryB", &["unit_display", "unit_extra"]);
    let scene = named_color_space("UnitSceneCategory", &["unit_scene"]);

    config.add_color_space(&display_a);
    config.add_color_space(&display_b);
    config.add_color_space(&scene);

    let all = config
        .color_space_set::<&str>(None)
        .expect("all color spaces");
    assert!(all.has_color_space("raw"));
    assert!(all.has_color_space("UnitDisplayCategoryA"));
    assert!(all.has_color_space("UnitDisplayCategoryB"));
    assert!(all.has_color_space("UnitSceneCategory"));

    let display = config
        .color_space_set(Some("unit_display"))
        .expect("display category color spaces");
    assert_eq!(display.num_color_spaces(), 2);
    assert!(display.has_color_space("UnitDisplayCategoryA"));
    assert!(display.has_color_space("UnitDisplayCategoryB"));
    assert!(!display.has_color_space("UnitSceneCategory"));
    assert_eq!(
        set_names(&display),
        BTreeSet::from([
            String::from("UnitDisplayCategoryA"),
            String::from("UnitDisplayCategoryB"),
        ])
    );

    let extra = config
        .color_space_set(Some("unit_extra"))
        .expect("extra category color spaces");
    assert_eq!(extra.num_color_spaces(), 1);
    assert!(extra.has_color_space("UnitDisplayCategoryB"));
    assert_eq!(
        extra
            .try_color_space_by_index(0)
            .expect("color-space index lookup")
            .and_then(|cs| cs.name())
            .as_deref(),
        Some("UnitDisplayCategoryB")
    );

    let missing = config
        .color_space_set(Some("definitely_missing_category"))
        .expect("missing category color spaces");
    assert_eq!(missing.num_color_spaces(), 0);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn color_space_set_registration_errors_surface_behavior() {
    let _guard = color_space_set_test_lock();
    if is_stub() {
        return;
    }

    let set = ColorSpaceSet::create().expect("color space set create");
    let unnamed = ColorSpace::create().expect("color space create");

    let add_err = set
        .try_add_color_space(&unnamed)
        .expect_err("unnamed color space should fail");
    assert!(
        matches!(add_err, ocio_rs::OcioError::Ocio(_)),
        "unexpected error variant: {add_err:?}"
    );
}
