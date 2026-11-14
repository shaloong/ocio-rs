use ocio_rs::{Config, FileRules, ViewingRules};

#[test]
#[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
fn config_rules_are_const_and_editable_copies_are_independent() {
    let config = Config::raw().unwrap();
    let file = FileRules::create().unwrap();
    file.insert_rule(0, "Images", "raw", "*", "exr").unwrap();
    file.set_custom_key(0, "key", "value").unwrap();
    config.set_file_rules(&file).unwrap();
    let viewing = ViewingRules::create().unwrap();
    viewing.insert_rule(0, "Raw").unwrap();
    viewing.add_color_space(0, "raw").unwrap();
    viewing.set_custom_key(0, "vkey", "vvalue").unwrap();
    viewing.insert_rule(1, "Linear").unwrap();
    viewing.add_encoding(1, "scene-linear").unwrap();
    config.set_viewing_rules_object(&viewing).unwrap();
    let files = config.try_const_file_rules().unwrap().unwrap();
    let views = config.try_const_viewing_rules().unwrap().unwrap();
    assert_eq!(files.num_entries().unwrap(), 2);
    assert_eq!(files.index_for_rule(c"Images").unwrap(), 0);
    assert_eq!(files.name(0).unwrap().unwrap(), c"Images");
    assert_eq!(files.pattern(0).unwrap().unwrap(), c"*");
    assert_eq!(files.extension(0).unwrap().unwrap(), c"exr");
    assert_eq!(files.color_space(0).unwrap().unwrap(), c"raw");
    assert!(!files.is_default().unwrap());
    assert_eq!(files.num_custom_keys(0).unwrap(), 1);
    assert_eq!(files.custom_key_name(0, 0).unwrap().unwrap(), c"key");
    assert_eq!(files.custom_key_value(0, 0).unwrap().unwrap(), c"value");
    assert_eq!(views.num_entries().unwrap(), 2);
    assert_eq!(views.index_for_rule(c"Raw").unwrap(), 0);
    assert_eq!(views.name(0).unwrap().unwrap(), c"Raw");
    assert_eq!(views.num_color_spaces(0).unwrap(), 1);
    assert_eq!(views.color_space(0, 0).unwrap().unwrap(), c"raw");
    assert_eq!(views.num_encodings(1).unwrap(), 1);
    assert_eq!(views.encoding(1, 0).unwrap().unwrap(), c"scene-linear");
    assert_eq!(views.num_custom_keys(0).unwrap(), 1);
    assert_eq!(views.custom_key_name(0, 0).unwrap().unwrap(), c"vkey");
    assert_eq!(views.custom_key_value(0, 0).unwrap().unwrap(), c"vvalue");
    assert!(files.name(usize::MAX).is_err());
    assert!(views.name(usize::MAX).is_err());
    let editable_files = config.file_rules().unwrap();
    editable_files.set_pattern(0, "changed*").unwrap();
    let editable_views = config.try_viewing_rules().unwrap().unwrap();
    editable_views.insert_rule(2, "New").unwrap();
    assert_eq!(files.pattern(0).unwrap().unwrap(), c"*");
    assert_eq!(views.num_entries().unwrap(), 2);
    let borrowed_name = files.name(0).unwrap().unwrap();
    config.set_file_rules(&editable_files).unwrap();
    config.set_viewing_rules_object(&editable_views).unwrap();
    assert_eq!(borrowed_name, c"Images");
    assert_eq!(
        config
            .try_const_file_rules()
            .unwrap()
            .unwrap()
            .pattern(0)
            .unwrap()
            .unwrap(),
        c"changed*"
    );
    assert_eq!(
        config
            .try_const_viewing_rules()
            .unwrap()
            .unwrap()
            .num_entries()
            .unwrap(),
        3
    );
    drop(config);
    assert_eq!(files.create_editable_copy().unwrap().num_entries(), 2);
    assert_eq!(views.create_editable_copy().unwrap().num_entries(), 2);
}
