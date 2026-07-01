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
        set.color_space("UnitColorSpaceSetB")
            .and_then(|cs| cs.name())
            .as_deref(),
        Some("UnitColorSpaceSetB")
    );
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

    set.remove_color_spaces(&other);
    assert_eq!(set.num_color_spaces(), 2);
    assert!(!set.has_color_space("UnitColorSpaceSetC"));

    set.remove_color_space("UnitColorSpaceSetA")
        .expect("remove color space A");
    assert_eq!(set.num_color_spaces(), 1);
    assert!(set.has_color_space("UnitColorSpaceSetB"));

    set.clear_color_spaces();
    assert_eq!(set.num_color_spaces(), 0);
}

#[test]
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
            .color_space_by_index(0)
            .and_then(|cs| cs.name())
            .as_deref(),
        Some("UnitDisplayCategoryB")
    );

    let missing = config
        .color_space_set(Some("definitely_missing_category"))
        .expect("missing category color spaces");
    assert_eq!(missing.num_color_spaces(), 0);
}
