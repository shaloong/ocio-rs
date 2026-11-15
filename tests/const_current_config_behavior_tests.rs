use ocio_rs::{
    try_const_current_config, try_current_config, try_set_const_current_config,
    try_set_current_config, Config,
};

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn current_config_is_readonly_snapshot_or_explicit_editable_copy() {
    let previous = try_current_config().unwrap().unwrap();
    let first = Config::raw().unwrap();
    first.set_name("first-global").unwrap();
    first
        .add_display("TestDisplay", "TestView", "raw", "")
        .unwrap();
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
    let shared_context = shared.try_const_current_context().unwrap().unwrap();
    let raw = shared.try_const_color_space(c"raw").unwrap().unwrap();
    for context in [None, Some(&shared_context)] {
        let processor = shared
            .processor_from_const_color_spaces(&raw, &raw, context)
            .unwrap();
        let mut pixel = [0.2, 0.4, 0.6, 1.0];
        processor
            .default_cpu_processor()
            .unwrap()
            .try_apply_rgba(&mut pixel)
            .unwrap();
        assert_eq!(pixel, [0.2, 0.4, 0.6, 1.0]);
    }
    assert!(shared
        .processor_with_const_context(c"raw", c"raw", &shared_context)
        .is_ok());
    assert!(shared
        .processor_with_const_context(c"missing", c"raw", &shared_context)
        .is_err());
    for context in [None, Some(&shared_context)] {
        let processor = shared
            .processor_display(
                c"raw",
                c"TestDisplay",
                c"TestView",
                ocio_rs::TransformDirection::Forward,
                context,
            )
            .unwrap();
        let mut pixel = [0.2, 0.4, 0.6, 1.0];
        processor
            .default_cpu_processor()
            .unwrap()
            .try_apply_rgba(&mut pixel)
            .unwrap();
        assert_eq!(pixel, [0.2, 0.4, 0.6, 1.0]);
        assert!(shared
            .processor_display(
                c"raw",
                c"missing",
                c"TestView",
                ocio_rs::TransformDirection::Forward,
                context
            )
            .is_err());
    }
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
    try_set_const_current_config(&shared).unwrap();
    assert_eq!(
        try_const_current_config()
            .unwrap()
            .unwrap()
            .name()
            .unwrap()
            .as_deref(),
        Some("first-global")
    );
    second.set_name("modified-second").unwrap();
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
    let named = ocio_rs::NamedTransform::create().unwrap();
    named.set_name("dynamic-named").unwrap();
    named
        .try_set_transform(&exposure, ocio_rs::TransformDirection::Forward)
        .unwrap();
    dynamic_config.try_add_named_transform(&named).unwrap();
    input
        .try_set_transform(&exposure, ocio_rs::ColorSpaceDirection::ToReference)
        .unwrap();
    dynamic_config.try_add_color_space(&input).unwrap();
    try_set_current_config(&dynamic_config).unwrap();
    let snapshot = try_const_current_config().unwrap().unwrap();
    let context = snapshot.try_const_current_context().unwrap().unwrap();
    let source = snapshot
        .try_const_color_space(c"dynamic-input")
        .unwrap()
        .unwrap();
    let destination = snapshot
        .try_const_color_space(c"reference")
        .unwrap()
        .unwrap();
    let named_object = snapshot
        .try_const_named_transform(c"dynamic-named")
        .unwrap()
        .unwrap();
    for ctx in [None, Some(&context)] {
        let make_spaces = || snapshot.processor_from_const_color_spaces(&source, &destination, ctx);
        let make_named = || {
            snapshot.processor_from_const_named_transform(
                &named_object,
                ocio_rs::TransformDirection::Forward,
                ctx,
            )
        };
        for make in [&make_spaces as &dyn Fn() -> _, &make_named] {
            let left = make().unwrap();
            let right = make().unwrap();
            left.dynamic_property(ocio_rs::DynamicPropertyType::Exposure)
                .unwrap()
                .set_double_value(1.25)
                .unwrap();
            assert_eq!(
                right
                    .dynamic_property(ocio_rs::DynamicPropertyType::Exposure)
                    .unwrap()
                    .double_value()
                    .unwrap(),
                0.0
            );
        }
        let first = snapshot
            .processor_named_transform(c"dynamic-named", ocio_rs::TransformDirection::Forward, ctx)
            .unwrap();
        let second = snapshot
            .processor_named_transform(c"dynamic-named", ocio_rs::TransformDirection::Forward, ctx)
            .unwrap();
        first
            .dynamic_property(ocio_rs::DynamicPropertyType::Exposure)
            .unwrap()
            .set_double_value(1.5)
            .unwrap();
        assert_eq!(
            second
                .dynamic_property(ocio_rs::DynamicPropertyType::Exposure)
                .unwrap()
                .double_value()
                .unwrap(),
            0.0
        );
        assert!(snapshot
            .processor_named_transform(c"missing", ocio_rs::TransformDirection::Forward, ctx)
            .is_err());
    }
    let ctx_left = snapshot
        .processor_with_const_context(c"dynamic-input", c"reference", &context)
        .unwrap();
    let ctx_right = snapshot
        .processor_with_const_context(c"dynamic-input", c"reference", &context)
        .unwrap();
    let ctx_property = ctx_left
        .dynamic_property(ocio_rs::DynamicPropertyType::Exposure)
        .unwrap();
    ctx_property.set_double_value(1.25).unwrap();
    assert_eq!(
        ctx_right
            .dynamic_property(ocio_rs::DynamicPropertyType::Exposure)
            .unwrap()
            .double_value()
            .unwrap(),
        0.0
    );
    let transform = snapshot
        .try_const_color_space(c"dynamic-input")
        .unwrap()
        .unwrap()
        .try_const_transform(ocio_rs::ColorSpaceDirection::ToReference)
        .unwrap()
        .unwrap();
    for context_option in [None, Some(&context)] {
        let a = snapshot
            .processor_from_const_transform(
                &transform,
                ocio_rs::TransformDirection::Forward,
                context_option,
            )
            .unwrap();
        let b = snapshot
            .processor_from_const_transform(
                &transform,
                ocio_rs::TransformDirection::Forward,
                context_option,
            )
            .unwrap();
        a.dynamic_property(ocio_rs::DynamicPropertyType::Exposure)
            .unwrap()
            .set_double_value(0.75)
            .unwrap();
        assert_eq!(
            b.dynamic_property(ocio_rs::DynamicPropertyType::Exposure)
                .unwrap()
                .double_value()
                .unwrap(),
            0.0
        );
    }
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
                let context = config.try_const_current_context().unwrap().unwrap();
                let processor = config
                    .processor_with_const_context(c"dynamic-input", c"reference", &context)
                    .unwrap();
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
