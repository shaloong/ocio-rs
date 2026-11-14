use ocio_rs::transform::MatrixTransform;
use ocio_rs::{
    Config, Look, NamedTransform, ReferenceSpaceType, TransformDirection, ViewTransform,
    ViewTransformDirection,
};

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_named_items_are_copied_or_shared_read_only() {
    let config = Config::raw().unwrap();
    let matrix = MatrixTransform::scale(&[2.0, 1.0, 1.0, 1.0]).unwrap();
    let look = Look::create().unwrap();
    look.set_name("test-look").unwrap();
    look.set_process_space("raw").unwrap();
    look.try_set_transform(&matrix).unwrap();
    config.try_add_look(&look).unwrap();
    let named = NamedTransform::create().unwrap();
    named.set_name("test-named").unwrap();
    named
        .try_set_transform(&matrix, TransformDirection::Forward)
        .unwrap();
    config.try_add_named_transform(&named).unwrap();
    let view = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
    view.set_name("test-view").unwrap();
    view.try_set_transform(Some(&matrix), ViewTransformDirection::ToReference)
        .unwrap();
    config.try_add_view_transform(&view).unwrap();
    config.set_default_view_transform_name("test-view").unwrap();

    let shared_look = config.try_const_look(c"test-look").unwrap().unwrap();
    let shared_named = config
        .try_const_named_transform(c"test-named")
        .unwrap()
        .unwrap();
    let shared_view = config
        .try_const_view_transform(c"test-view")
        .unwrap()
        .unwrap();
    let shared_default = config
        .try_const_default_scene_to_display_view_transform()
        .unwrap()
        .unwrap();
    let default_copy = config.default_scene_to_display_view_transform().unwrap();
    default_copy.set_name("changed-default").unwrap();
    assert_eq!(shared_default.name().unwrap().as_deref(), Some("test-view"));
    let look_copy = config.try_look("test-look").unwrap().unwrap();
    let named_copy = config.try_named_transform("test-named").unwrap().unwrap();
    let view_copy = config.try_view_transform("test-view").unwrap().unwrap();
    look_copy.set_name("changed-look").unwrap();
    named_copy.set_name("changed-named").unwrap();
    view_copy.set_name("changed-view").unwrap();
    assert_eq!(shared_look.name().unwrap().as_deref(), Some("test-look"));
    assert_eq!(shared_named.name().unwrap().as_deref(), Some("test-named"));
    assert_eq!(shared_view.name().unwrap().as_deref(), Some("test-view"));
    assert!(config.try_look("changed-look").unwrap().is_none());
    assert!(config
        .try_named_transform("changed-named")
        .unwrap()
        .is_none());
    assert!(config.try_view_transform("changed-view").unwrap().is_none());
    let edited_look = shared_look.create_editable_copy().unwrap();
    edited_look.set_description("edited").unwrap();
    config.try_add_look(&edited_look).unwrap();
    assert_eq!(
        config
            .try_look("test-look")
            .unwrap()
            .unwrap()
            .description()
            .as_deref(),
        Some("edited")
    );
    let context = config.current_context().unwrap();
    for ctx in [None, Some(&context)] {
        let processor = config
            .processor_from_const_named_transform(&shared_named, TransformDirection::Forward, ctx)
            .unwrap();
        let mut pixel = [0.25, 0.25, 0.25, 1.0];
        processor
            .default_cpu_processor()
            .unwrap()
            .try_apply_rgba(&mut pixel)
            .unwrap();
        assert!((pixel[0] - 0.5).abs() < 1e-6);
    }
    drop(config);
    assert_eq!(
        shared_look
            .create_editable_copy()
            .unwrap()
            .name()
            .as_deref(),
        Some("test-look")
    );
    assert_eq!(
        shared_named
            .create_editable_copy()
            .unwrap()
            .name()
            .as_deref(),
        Some("test-named")
    );
    assert_eq!(
        shared_view
            .create_editable_copy()
            .unwrap()
            .name()
            .as_deref(),
        Some("test-view")
    );
    assert_eq!(
        shared_look
            .try_const_transform()
            .unwrap()
            .unwrap()
            .type_tag()
            .unwrap(),
        ocio_rs::TransformType::Matrix as i32
    );
    assert_eq!(
        shared_named
            .try_const_transform(TransformDirection::Forward)
            .unwrap()
            .unwrap()
            .type_tag()
            .unwrap(),
        ocio_rs::TransformType::Matrix as i32
    );
    assert_eq!(
        shared_view
            .try_const_transform(ViewTransformDirection::ToReference)
            .unwrap()
            .unwrap()
            .type_tag()
            .unwrap(),
        ocio_rs::TransformType::Matrix as i32
    );
}
