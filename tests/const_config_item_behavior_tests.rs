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
    look.set_description("look description").unwrap();
    look.try_set_transform(&matrix).unwrap();
    config.try_add_look(&look).unwrap();
    let named = NamedTransform::create().unwrap();
    named.set_name("test-named").unwrap();
    named.set_family("named/family").unwrap();
    named.set_encoding("scene-linear").unwrap();
    named.set_description("named description").unwrap();
    named.add_alias("test-alias").unwrap();
    named.add_category("working-space").unwrap();
    named
        .try_set_transform(&matrix, TransformDirection::Forward)
        .unwrap();
    config.try_add_named_transform(&named).unwrap();
    let view = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
    view.set_name("test-view").unwrap();
    view.set_family("view/family").unwrap();
    view.set_description("view description").unwrap();
    view.add_category("display").unwrap();
    #[cfg(feature = "v2_5")]
    {
        look.set_interchange_attribute("amf_transform_ids", "look-id")
            .unwrap();
        view.set_interchange_attribute("amf_transform_ids", "view-id")
            .unwrap();
        config.try_add_look(&look).unwrap();
    }
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
    assert_eq!(shared_look.name_ref().unwrap().unwrap(), c"test-look");
    assert_eq!(shared_look.process_space().unwrap().unwrap(), c"raw");
    assert_eq!(
        shared_look.description().unwrap().unwrap(),
        c"look description"
    );
    assert_eq!(shared_named.family().unwrap().unwrap(), c"named/family");
    assert_eq!(
        shared_named.description().unwrap().unwrap(),
        c"named description"
    );
    assert_eq!(shared_named.encoding().unwrap().unwrap(), c"scene-linear");
    assert_eq!(shared_named.num_aliases().unwrap(), 1);
    assert_eq!(shared_named.alias(0).unwrap().unwrap(), c"test-alias");
    assert!(shared_named.alias(usize::MAX).unwrap().is_none());
    assert!(shared_named.has_alias(c"test-alias").unwrap());
    assert_eq!(shared_named.num_categories().unwrap(), 1);
    assert!(shared_named.has_category(c"working-space").unwrap());
    assert_eq!(shared_named.category(0).unwrap().unwrap(), c"working-space");
    assert_eq!(shared_view.family().unwrap().unwrap(), c"view/family");
    assert_eq!(
        shared_view.description().unwrap().unwrap(),
        c"view description"
    );
    assert_eq!(
        shared_view.reference_space_type().unwrap(),
        ReferenceSpaceType::Scene
    );
    assert_eq!(shared_view.num_categories().unwrap(), 1);
    assert_eq!(shared_view.category(0).unwrap().unwrap(), c"display");
    assert!(shared_view.has_category(c"display").unwrap());
    #[cfg(feature = "v2_5")]
    {
        assert_eq!(
            shared_look
                .interchange_attribute(c"amf_transform_ids")
                .unwrap()
                .unwrap(),
            c"look-id"
        );
        assert_eq!(
            shared_view
                .interchange_attribute(c"amf_transform_ids")
                .unwrap()
                .unwrap(),
            c"view-id"
        );
        assert!(shared_view.interchange_attribute(c"unknown").is_err());
    }
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
    assert!(shared_named
        .try_const_transform(TransformDirection::Inverse)
        .unwrap()
        .is_none());
    let inverse = shared_named
        .resolve_const_transform(TransformDirection::Inverse)
        .unwrap()
        .unwrap();
    assert_eq!(
        named
            .resolve_const_transform(TransformDirection::Inverse)
            .unwrap()
            .unwrap()
            .type_tag()
            .unwrap(),
        ocio_rs::TransformType::Matrix as i32
    );
    let processor = config
        .processor_from_const_transform(&inverse, TransformDirection::Forward, None)
        .unwrap();
    let mut pixel = [0.5, 0.25, 0.25, 1.0];
    processor
        .default_cpu_processor()
        .unwrap()
        .try_apply_rgba(&mut pixel)
        .unwrap();
    assert!((pixel[0] - 0.25).abs() < 1e-6);
    let empty = NamedTransform::create().unwrap();
    assert!(empty
        .resolve_const_transform(TransformDirection::Forward)
        .is_err());
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
