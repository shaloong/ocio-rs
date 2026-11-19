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
    let original_flags = shared.processor_cache_flags().unwrap();
    let other_flags = if original_flags == ocio_rs::ProcessorCacheFlags::OFF {
        ocio_rs::ProcessorCacheFlags::ENABLED
    } else {
        ocio_rs::ProcessorCacheFlags::OFF
    };
    // This snapshot is local to the test thread. Changing its flags must not
    // affect the global config or a later independently acquired snapshot.
    unsafe { shared.try_set_processor_cache_flags(other_flags).unwrap() };
    assert_eq!(
        try_const_current_config()
            .unwrap()
            .unwrap()
            .processor_cache_flags()
            .unwrap(),
        original_flags
    );
    unsafe {
        shared
            .try_set_processor_cache_flags(original_flags)
            .unwrap()
    };
    // The explicit raw-sharing entry preserves native aliasing for callers
    // who can serialize all access to the global config.
    let native_shared = unsafe { ocio_rs::try_const_current_config_shared().unwrap().unwrap() };
    unsafe {
        native_shared
            .try_set_processor_cache_flags(other_flags)
            .unwrap()
    };
    assert_eq!(
        try_const_current_config()
            .unwrap()
            .unwrap()
            .processor_cache_flags()
            .unwrap(),
        other_flags
    );
    unsafe {
        native_shared
            .try_set_processor_cache_flags(original_flags)
            .unwrap()
    };
    drop(native_shared);
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
    let full = shared.cache_id().unwrap().unwrap();
    let unmodified_copy = shared.create_editable_copy().unwrap();
    assert_eq!(
        shared
            .cache_id_for_context(&shared_context)
            .unwrap()
            .as_deref(),
        Some(full.as_str())
    );
    assert_eq!(
        unmodified_copy
            .try_cache_id_for_const_context(&shared_context)
            .unwrap()
            .as_deref(),
        Some(full.as_str())
    );
    let authored = shared.cache_id_without_file_references().unwrap().unwrap();
    assert_ne!(authored, full);
    assert_eq!(
        unmodified_copy
            .try_cache_id_without_file_references()
            .unwrap()
            .as_deref(),
        Some(authored.as_str())
    );
    assert_eq!(
        shared.environment_mode().unwrap(),
        editable.try_environment_mode().unwrap()
    );
    assert_eq!(
        shared.environment_mode().unwrap(),
        shared_context.environment_mode().unwrap()
    );
    let raw = shared.try_const_color_space(c"raw").unwrap().unwrap();
    #[cfg(feature = "v2_6")]
    {
        let found = shared
            .find_const_color_space_for_id(c"vendor:raw")
            .unwrap()
            .unwrap();
        assert_eq!(found.name().unwrap(), c"raw");
        assert!(shared
            .find_const_color_space_for_id(c"missing")
            .unwrap()
            .is_none());
        assert!(shared.find_const_color_space_for_id(c"").unwrap().is_none());
        let local_id = shared
            .generate_local_id_for_color_space(c"raw")
            .unwrap()
            .unwrap();
        assert_eq!(
            Some(local_id.as_str()),
            first
                .generate_local_id_for_color_space(c"raw")
                .unwrap()
                .as_deref()
        );
        let local_id = std::ffi::CString::new(local_id).unwrap();
        assert_eq!(
            shared
                .find_const_color_space_for_id(&local_id)
                .unwrap()
                .unwrap()
                .name()
                .unwrap(),
            c"raw"
        );
        assert!(shared
            .generate_local_id_for_color_space(c"missing")
            .is_err());
        assert!(shared
            .generate_local_id_for_color_space(c"raw")
            .unwrap()
            .is_some());
        let copy = found.create_editable_copy().unwrap();
        copy.set_name("independent").unwrap();
        assert_eq!(found.name().unwrap(), c"raw");
    }
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
    dynamic_config
        .set_role("aces_interchange", "reference")
        .unwrap();
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
    assert!(snapshot
        .processor_from_const_transform_default_direction(&transform)
        .unwrap()
        .is_dynamic());
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
    check_cross_config_processors(&snapshot);
    check_global_group_children_are_isolated();
    let registry = ocio_rs::BuiltinConfigRegistry::get().unwrap();
    let name = registry.config_name(0).unwrap();
    let builtin = Config::create_from_builtin_config(&name).unwrap();
    let before = builtin.serialize().unwrap();
    try_set_current_config(&builtin).unwrap();
    assert_eq!(
        try_current_config().unwrap().unwrap().serialize().unwrap(),
        before
    );
    try_set_current_config(&previous).unwrap();
}

fn check_global_group_children_are_isolated() {
    use ocio_rs::transform::{GroupTransform, MatrixTransform};
    let config = Config::raw().unwrap();
    config.try_set_processor_cache_flags(0).unwrap();
    let reference = ocio_rs::ColorSpace::create().unwrap();
    reference.set_name("group-reference").unwrap();
    config.try_add_color_space(&reference).unwrap();
    let group = GroupTransform::create().unwrap();
    let nested = GroupTransform::create().unwrap();
    let matrix = MatrixTransform::scale(&[2.0, 2.0, 2.0, 1.0]).unwrap();
    nested.append_transform(&matrix).unwrap();
    group.append_transform(&nested).unwrap();
    let color = ocio_rs::ColorSpace::create().unwrap();
    color.set_name("group-source").unwrap();
    color
        .try_set_transform(&group, ocio_rs::ColorSpaceDirection::ToReference)
        .unwrap();
    config.try_add_color_space(&color).unwrap();
    let hidden = ocio_rs::ColorSpace::create().unwrap();
    hidden.set_name("hidden-group").unwrap();
    hidden
        .try_set_transform(&group, ocio_rs::ColorSpaceDirection::FromReference)
        .unwrap();
    config.try_add_color_space(&hidden).unwrap();
    let look = ocio_rs::Look::create().unwrap();
    look.set_name("group-look").unwrap();
    look.set_process_space("group-reference").unwrap();
    look.try_set_transform(&group).unwrap();
    look.try_set_inverse_transform(&group).unwrap();
    config.try_add_look(&look).unwrap();
    let named = ocio_rs::NamedTransform::create().unwrap();
    named.set_name("group-named").unwrap();
    for direction in [
        ocio_rs::TransformDirection::Forward,
        ocio_rs::TransformDirection::Inverse,
    ] {
        named.try_set_transform(&group, direction).unwrap();
    }
    config.try_add_named_transform(&named).unwrap();
    config
        .set_inactive_color_spaces("hidden-group,group-named")
        .unwrap();
    let view = ocio_rs::ViewTransform::create(ocio_rs::ReferenceSpaceType::Scene).unwrap();
    view.set_name("group-view").unwrap();
    for direction in [
        ocio_rs::ViewTransformDirection::ToReference,
        ocio_rs::ViewTransformDirection::FromReference,
    ] {
        view.try_set_transform(Some(&group), direction).unwrap();
    }
    config.try_add_view_transform(&view).unwrap();
    let serialized = config.serialize().unwrap();
    try_set_current_config(&config).unwrap();
    assert_eq!(
        try_current_config().unwrap().unwrap().serialize().unwrap(),
        serialized
    );
    let editable_snapshot = try_current_config().unwrap().unwrap();
    let space = editable_snapshot
        .try_color_space("group-source")
        .unwrap()
        .unwrap();
    let Some(ocio_rs::transform::Transform::Group(group_copy)) = space
        .try_transform(ocio_rs::ColorSpaceDirection::ToReference)
        .unwrap()
    else {
        panic!("group expected")
    };
    let Some(ocio_rs::transform::Transform::Group(nested_copy)) =
        group_copy.try_transform(0).unwrap()
    else {
        panic!("nested group expected")
    };
    let Some(ocio_rs::transform::Transform::Matrix(snapshot_leaf)) =
        nested_copy.try_transform(0).unwrap()
    else {
        panic!("matrix expected")
    };
    snapshot_leaf.set_offset(&[0.5, 0.0, 0.0, 0.0]).unwrap();
    assert_eq!(
        try_current_config().unwrap().unwrap().serialize().unwrap(),
        serialized,
        "editing a snapshot changed the global config"
    );
    let look = editable_snapshot.try_look("group-look").unwrap().unwrap();
    let Some(ocio_rs::transform::Transform::Group(look_group)) = look.try_transform().unwrap()
    else {
        panic!("look group expected")
    };
    let Some(ocio_rs::transform::Transform::Group(look_nested)) =
        look_group.try_transform(0).unwrap()
    else {
        panic!("look nested group expected")
    };
    let Some(ocio_rs::transform::Transform::Matrix(look_leaf)) =
        look_nested.try_transform(0).unwrap()
    else {
        panic!("look matrix expected")
    };
    assert_eq!(
        look_leaf.offset()[0],
        0.5,
        "snapshot lost native sharing between group roots"
    );
    let readonly = try_const_current_config().unwrap().unwrap();
    let editable = readonly.create_editable_copy().unwrap();
    let space = editable.try_color_space("group-source").unwrap().unwrap();
    let Some(ocio_rs::transform::Transform::Group(group)) = space
        .try_transform(ocio_rs::ColorSpaceDirection::ToReference)
        .unwrap()
    else {
        panic!("group expected")
    };
    let Some(ocio_rs::transform::Transform::Group(nested)) = group.try_transform(0).unwrap() else {
        panic!("nested group expected")
    };
    let Some(ocio_rs::transform::Transform::Matrix(leaf)) = nested.try_transform(0).unwrap() else {
        panic!("matrix expected")
    };
    leaf.set_offset(&[0.75, 0.0, 0.0, 0.0]).unwrap();
    assert_eq!(
        try_current_config().unwrap().unwrap().serialize().unwrap(),
        serialized,
        "editing a copy of a read-only snapshot changed the global config"
    );
    // Each thread acquires its own global snapshot. Mutating shared group
    // children within one snapshot must not affect concurrent acquisitions.
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let workers: Vec<_> = [0.25, 0.75]
        .into_iter()
        .map(|offset| {
            let barrier = barrier.clone();
            let expected = serialized.clone();
            std::thread::spawn(move || {
                let snapshot = try_current_config().unwrap().unwrap();
                let space = snapshot.try_color_space("group-source").unwrap().unwrap();
                let Some(ocio_rs::transform::Transform::Group(group)) = space
                    .try_transform(ocio_rs::ColorSpaceDirection::ToReference)
                    .unwrap()
                else {
                    panic!("group expected")
                };
                let Some(ocio_rs::transform::Transform::Group(nested)) =
                    group.try_transform(0).unwrap()
                else {
                    panic!("nested group expected")
                };
                let Some(ocio_rs::transform::Transform::Matrix(leaf)) =
                    nested.try_transform(0).unwrap()
                else {
                    panic!("matrix expected")
                };
                barrier.wait();
                for _ in 0..8 {
                    leaf.set_offset(&[offset, 0.0, 0.0, 0.0]).unwrap();
                    let current = try_const_current_config().unwrap().unwrap();
                    assert!(current.cache_id().unwrap().is_some());
                    assert_eq!(
                        current.create_editable_copy().unwrap().serialize().unwrap(),
                        expected
                    );
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    matrix
        .set_matrix(&[
            3.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
        .unwrap();
    assert_eq!(
        try_current_config().unwrap().unwrap().serialize().unwrap(),
        serialized
    );
    let snapshot = try_const_current_config().unwrap().unwrap();
    let cpu = snapshot
        .processor(c"group-source", c"group-reference")
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    let mut pixel = [0.1, 0.2, 0.3, 1.0];
    cpu.try_apply_rgba(&mut pixel).unwrap();
    assert!(
        (pixel[0] - 0.2).abs() < 1e-6,
        "global config retained the caller's mutable group child: {pixel:?}"
    );
    matrix
        .set_matrix(&[
            2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
        .unwrap();
    let baker = ocio_rs::Baker::create().unwrap();
    baker.set_config(&config).unwrap();
    let retained = baker.const_config().unwrap().unwrap();
    try_set_const_current_config(&retained).unwrap();
    matrix
        .set_matrix(&[
            3.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
        .unwrap();
    assert_eq!(
        try_current_config().unwrap().unwrap().serialize().unwrap(),
        serialized
    );
}

fn check_cross_config_processors(dynamic: &ocio_rs::ConstConfig) {
    use ocio_rs::transform::MatrixTransform;
    use ocio_rs::{
        ColorSpace, ColorSpaceDirection, ConfigProcessorTarget as Target, DynamicPropertyType,
        TransformDirection,
    };

    let src = Config::create().unwrap();
    let dst = Config::create().unwrap();
    for (config, name, factor, direction, interchange_alias) in [
        (
            &src,
            "input",
            2.0,
            ColorSpaceDirection::ToReference,
            "source-interchange",
        ),
        (
            &dst,
            "output",
            3.0,
            ColorSpaceDirection::FromReference,
            "destination-interchange",
        ),
    ] {
        let reference = ColorSpace::create().unwrap();
        reference.set_name("reference").unwrap();
        reference.add_alias(interchange_alias).unwrap();
        config.try_add_color_space(&reference).unwrap();
        config.set_role("aces_interchange", "reference").unwrap();
        let color = ColorSpace::create().unwrap();
        color.set_name(name).unwrap();
        color
            .try_set_transform(
                &MatrixTransform::scale(&[factor, factor, factor, 1.0]).unwrap(),
                direction,
            )
            .unwrap();
        config.try_add_color_space(&color).unwrap();
    }
    dst.add_display("Screen", "View", "output", "").unwrap();
    try_set_current_config(&src).unwrap();
    let src = try_const_current_config().unwrap().unwrap();
    try_set_current_config(&dst).unwrap();
    let dst = try_const_current_config().unwrap().unwrap();
    let to_builtin = src
        .processor_to_builtin_color_space(c"input", c"ACES2065-1")
        .unwrap();
    let from_builtin = src
        .processor_from_builtin_color_space(c"ACES2065-1", c"input")
        .unwrap();
    let mut builtin_pixel = [0.125, 0.25, 0.5, 1.0];
    to_builtin
        .default_cpu_processor()
        .unwrap()
        .try_apply_rgba(&mut builtin_pixel)
        .unwrap();
    assert_eq!(builtin_pixel, [0.25, 0.5, 1.0, 1.0]);
    from_builtin
        .default_cpu_processor()
        .unwrap()
        .try_apply_rgba(&mut builtin_pixel)
        .unwrap();
    assert_eq!(builtin_pixel, [0.125, 0.25, 0.5, 1.0]);
    assert!(src
        .processor_to_builtin_color_space(c"input", c"nonexistent")
        .is_err());
    let src_context = src.try_const_current_context().unwrap().unwrap();
    let dst_context = dst.try_const_current_context().unwrap().unwrap();
    let display_target = |direction| Target::DisplayView {
        display: c"Screen",
        view: c"View",
        direction,
    };
    for contexts in [None, Some((&src_context, &dst_context))] {
        for interchange in [
            None,
            Some((c"source-interchange", c"destination-interchange")),
        ] {
            for (target, input, expected) in [
                (Target::ColorSpace(c"output"), 0.125, 0.75),
                (display_target(TransformDirection::Forward), 0.125, 0.75),
                (display_target(TransformDirection::Inverse), 0.75, 0.125),
            ] {
                let processor = src
                    .processor_to_config(c"input", &dst, target, contexts, interchange)
                    .unwrap();
                let mut pixel = [input, input, input, 1.0];
                processor
                    .default_cpu_processor()
                    .unwrap()
                    .try_apply_rgba(&mut pixel)
                    .unwrap();
                assert!((pixel[0] - expected).abs() < 1e-6);
                assert_eq!(pixel[3], 1.0);
            }
        }
    }
    assert!(src
        .processor_to_config(
            c"input",
            &dst,
            Target::ColorSpace(c"output"),
            None,
            Some((c"reference", c"missing")),
        )
        .is_err());
    for destination in [dynamic, &dst] {
        let build = || {
            dynamic
                .processor_to_config(
                    c"dynamic-input",
                    destination,
                    Target::ColorSpace(c"reference"),
                    None,
                    None,
                )
                .unwrap()
        };
        let left = build();
        let right = build();
        left.dynamic_property(DynamicPropertyType::Exposure)
            .unwrap()
            .set_double_value(1.5)
            .unwrap();
        assert_eq!(
            right
                .dynamic_property(DynamicPropertyType::Exposure)
                .unwrap()
                .double_value()
                .unwrap(),
            0.0
        );
    }
}
