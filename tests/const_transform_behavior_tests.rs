use ocio_rs::transform::{ConstTransform, MatrixTransform, Transform};
use ocio_rs::{
    ColorSpace, ColorSpaceDirection, Config, Look, NamedTransform, ReferenceSpaceType,
    TransformDirection, ViewTransform, ViewTransformDirection,
};

fn check_independent(copy: Transform, shared: ConstTransform) {
    let Transform::Matrix(copy) = copy else {
        panic!("expected matrix")
    };
    let mut matrix = copy.matrix();
    matrix[0] = 9.0;
    copy.set_matrix(&matrix).unwrap();
    let Transform::Matrix(original) = shared.create_editable_copy().unwrap() else {
        panic!("expected matrix")
    };
    assert_eq!(original.matrix()[0], 2.0);
    assert_eq!(
        shared.type_tag().unwrap(),
        ocio_rs::TransformType::Matrix as i32
    );
    let config = Config::raw().unwrap();
    let cpu = config
        .processor_from_const_transform(&shared, TransformDirection::Forward, None)
        .unwrap()
        .default_cpu_processor()
        .unwrap();
    let mut pixel = [0.25, 0.25, 0.25, 1.0];
    cpu.try_apply_rgba(&mut pixel).unwrap();
    assert!((pixel[0] - 0.5).abs() < 1e-6);
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn parent_transform_getters_preserve_native_constness() {
    let matrix = MatrixTransform::scale(&[2.0, 1.0, 1.0, 1.0]).unwrap();
    let color = ColorSpace::create().unwrap();
    color
        .try_set_transform(&matrix, ColorSpaceDirection::ToReference)
        .unwrap();
    let copy = color
        .try_transform(ColorSpaceDirection::ToReference)
        .unwrap()
        .unwrap();
    let shared = color
        .try_const_transform(ColorSpaceDirection::ToReference)
        .unwrap()
        .unwrap();
    drop(color);
    check_independent(copy, shared);

    let look = Look::create().unwrap();
    look.try_set_transform(&matrix).unwrap();
    look.try_set_inverse_transform(&matrix).unwrap();
    check_independent(
        look.try_transform().unwrap().unwrap(),
        look.try_const_transform().unwrap().unwrap(),
    );
    check_independent(
        look.try_inverse_transform().unwrap().unwrap(),
        look.try_const_inverse_transform().unwrap().unwrap(),
    );

    let named = NamedTransform::create().unwrap();
    named
        .try_set_transform(&matrix, TransformDirection::Forward)
        .unwrap();
    check_independent(
        named
            .try_transform(TransformDirection::Forward)
            .unwrap()
            .unwrap(),
        named
            .try_const_transform(TransformDirection::Forward)
            .unwrap()
            .unwrap(),
    );

    let view = ViewTransform::create(ReferenceSpaceType::Scene).unwrap();
    view.try_set_transform(Some(&matrix), ViewTransformDirection::ToReference)
        .unwrap();
    check_independent(
        view.try_transform(ViewTransformDirection::ToReference)
            .unwrap()
            .unwrap(),
        view.try_const_transform(ViewTransformDirection::ToReference)
            .unwrap()
            .unwrap(),
    );
}

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn readonly_transform_processor_uses_supplied_context() {
    let file = ocio_rs::transform::FileTransform::create().unwrap();
    file.set_src("${LUT_DIR}/lut1.clf").unwrap();
    let look = Look::create().unwrap();
    look.try_set_transform(&file).unwrap();
    let shared = look.try_const_transform().unwrap().unwrap();
    drop(look);
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/configs/context_test1");
    let config = Config::raw().unwrap();
    for (dir, expected) in [("shot1", 0.1f32), ("shot4", 0.4f32)] {
        let context = ocio_rs::Context::create().unwrap();
        context.set_working_dir(root.to_str().unwrap()).unwrap();
        context.set_string_var("LUT_DIR", dir).unwrap();
        let cpu = config
            .processor_from_const_transform(&shared, TransformDirection::Forward, Some(&context))
            .unwrap()
            .default_cpu_processor()
            .unwrap();
        let mut pixel = [0.01, 0.01, 0.01, 1.0];
        cpu.try_apply_rgba(&mut pixel).unwrap();
        assert!((pixel[0] - expected).abs() < 1e-5);
    }
}
