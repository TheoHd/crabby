use crabcleaner::{App, Rule};

#[test]
fn rule_get_line_value_works() {
    let rule: Rule = Rule::new_from_line(&"mv *.crc".to_string(), &1);
    assert_eq!("mv *.crc", rule.get_line_value());
}

#[test]
fn rule_get_line_number_works() {
    let rule: Rule = Rule::new_from_line(&"mv *.crc".to_string(), &2);
    assert_eq!(&3, rule.get_line_number());
}

#[test]
fn rule_get_error_message_works() {
    let rule: Rule = Rule::new_from_line(&"mv *.crc".to_string(), &1);
    assert_eq!(
        "Missing keyword(s) or pattern(s).",
        rule.get_error_message()
    );
}

#[test]
#[should_panic]
fn rule_get_token_works() {
    let rule: Rule = Rule::new_from_line(&"mv *.crc".to_string(), &1);
    assert_eq!(&String::from("*.crc"), rule.get_token(1));
}

#[test]
#[should_panic]
fn rule_get_tokens_works() {
    let rule: Rule = Rule::new_from_line(&"mv *.crc".to_string(), &1);
    assert_eq!(&String::from("*.crc"), &rule.get_tokens()[1].clone());
}

#[test]
fn rule_is_valid_works() {
    // TODO add more assertions for all the rules
    let app: App = App::new_for_test();
    assert!(*app.get_configuration(0).get_rule(0).is_valid());
}
