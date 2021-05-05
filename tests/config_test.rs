use crabby::App;

#[test]
fn config_get_filename_works() {
    let app: App = App::new_for_test();
    assert_eq!(".\\default.crc", app.get_configuration(0).get_filename());
}

#[test]
fn config_get_rule_works() {
    let app: App = App::new_for_test();
    assert_eq!(
        "mv file.mp3 to C:\\\\Users\\\\Username\\\\Music",
        app.get_configuration(0).get_rule(0).get_line_value()
    );
}

#[test]
fn config_get_rules_works() {
    let app: App = App::new_for_test();
    assert_eq!(
        "mv file.mp3 to C:\\\\Users\\\\Username\\\\Music",
        app.get_configuration(0).get_rules()[0].get_line_value()
    );
}
