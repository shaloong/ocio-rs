use ocio_rs::{try_const_current_config, try_current_config, try_set_current_config, Config};

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn current_config_is_readonly_snapshot_or_explicit_editable_copy() {
    let previous = try_current_config().unwrap().unwrap();
    let first = Config::raw().unwrap();
    first.set_name("first-global").unwrap();
    let look = ocio_rs::Look::create().unwrap();
    look.set_name("global-look").unwrap();
    look.set_process_space("raw").unwrap();
    first.try_add_look(&look).unwrap();
    try_set_current_config(&first).unwrap();
    let shared = try_const_current_config().unwrap().unwrap();
    let editable = try_current_config().unwrap().unwrap();
    editable.set_name("edited-copy").unwrap();
    assert_eq!(shared.name().unwrap().as_deref(), Some("first-global"));
    assert_eq!(
        try_const_current_config()
            .unwrap()
            .unwrap()
            .name()
            .unwrap()
            .as_deref(),
        Some("first-global")
    );
    assert!(shared.cache_id().unwrap().is_some());
    assert!(shared.try_const_current_context().unwrap().is_some());
    assert!(shared.try_const_color_space(c"raw").unwrap().is_some());
    assert_eq!(
        shared
            .try_const_look(c"global-look")
            .unwrap()
            .unwrap()
            .name_ref()
            .unwrap()
            .unwrap(),
        c"global-look"
    );
    assert!(shared
        .try_const_named_transform(c"missing")
        .unwrap()
        .is_none());
    assert!(shared
        .try_const_view_transform(c"missing")
        .unwrap()
        .is_none());
    assert_eq!(
        shared
            .try_const_file_rules()
            .unwrap()
            .unwrap()
            .num_entries()
            .unwrap(),
        1
    );
    let cpu = shared
        .processor(c"raw", c"raw")
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    let mut pixel = [0.2, 0.4, 0.6, 1.0];
    cpu.try_apply_rgba(&mut pixel).unwrap();
    assert_eq!(pixel, [0.2, 0.4, 0.6, 1.0]);
    let second = shared.create_editable_copy().unwrap();
    second.set_name("second-global").unwrap();
    try_set_current_config(&second).unwrap();
    assert_eq!(shared.name().unwrap().as_deref(), Some("first-global"));
    assert_eq!(
        try_const_current_config()
            .unwrap()
            .unwrap()
            .name()
            .unwrap()
            .as_deref(),
        Some("second-global")
    );

    // A shared global config may be queried independently by other threads.
    // Dynamic processors from those queries must not share mutable OCIO state.
    let dynamic_config = Config::raw().unwrap();
    let reference = ocio_rs::ColorSpace::create().unwrap();
    reference.set_name("reference").unwrap();
    dynamic_config.try_add_color_space(&reference).unwrap();
    let input = ocio_rs::ColorSpace::create().unwrap();
    input.set_name("dynamic-input").unwrap();
    let exposure = ocio_rs::transform::ExposureContrastTransform::create().unwrap();
    exposure.make_exposure_dynamic();
    input
        .try_set_transform(&exposure, ocio_rs::ColorSpaceDirection::ToReference)
        .unwrap();
    dynamic_config.try_add_color_space(&input).unwrap();
    try_set_current_config(&dynamic_config).unwrap();
    let snapshot = try_const_current_config().unwrap().unwrap();
    let left = snapshot.processor(c"dynamic-input", c"reference").unwrap();
    let right = snapshot.processor(c"dynamic-input", c"reference").unwrap();
    assert!(left.is_dynamic() && right.is_dynamic());
    let left_property = left
        .dynamic_property(ocio_rs::DynamicPropertyType::Exposure)
        .unwrap();
    let right_property = right
        .dynamic_property(ocio_rs::DynamicPropertyType::Exposure)
        .unwrap();
    left_property.set_double_value(1.5).unwrap();
    assert_eq!(left_property.double_value().unwrap(), 1.5);
    assert_eq!(right_property.double_value().unwrap(), 0.0);
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let workers: Vec<_> = [1.0, 2.0]
        .into_iter()
        .map(|value| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let config = try_const_current_config().unwrap().unwrap();
                let processor = config.processor(c"dynamic-input", c"reference").unwrap();
                let property = processor
                    .dynamic_property(ocio_rs::DynamicPropertyType::Exposure)
                    .unwrap();
                property.set_double_value(value).unwrap();
                barrier.wait();
                property.double_value().unwrap()
            })
        })
        .collect();
    let values: Vec<_> = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .collect();
    assert_eq!(values, [1.0, 2.0]);
    try_set_current_config(&previous).unwrap();
}
