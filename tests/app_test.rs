use crabcleaner::App;

#[test]
fn app_get_configurations_works() {
    let app: App = App::new_for_test();
    assert!(!app.get_configurations().is_empty());
}

#[test]
fn app_get_configuration_works() {
    let app: App = App::new_for_test();
    assert_eq!(".\\default.crc", app.get_configuration(0).get_filename());
}

#[test]
#[should_panic]
fn app_cli_works_for_first_arg() {
    App::cli_for_test().value_of("clean").unwrap();
}

#[test]
#[should_panic]
fn app_cli_works_for_second_arg() {
    App::cli_for_test().value_of("file").unwrap();
}

#[test]
fn app_get_arg_works() {
    assert_eq!("", App::get_arg(&App::cli_for_test(), "clean"));
}

#[test]
fn app_default_conf_lines_works() {
    assert_eq!(String::from("mv file.mp3 to C:\\\\Users\\\\Username\\\\Music      // This rule moves `file.mp3` to the Music folder"),App::default_conf_lines().1);
}

#[test]
fn app_conf_file_lines_works() {
    let conf_file_lines: String = App::conf_file_lines("./tests/test.crc").1.to_string();
    assert!(conf_file_lines.contains("// Test Start"));
    assert!(conf_file_lines.contains("// Test End"));
}

#[test]
fn app_execute_configurations_works() {
    // TODO when execute_configurations() is created in App
}
