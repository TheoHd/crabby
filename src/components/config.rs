use crate::components::Rule;
use crate::App;

/// A Configuration is created from a `.crc` file specified in the Application structure and implementation
#[derive(Clone, Debug)]
pub struct Config {
    filename: String,
    rules: Vec<Rule>,
}

impl Config {
    /// Execute all the rules in the current `Configuration`
    pub fn execute_rules(&self, path: String, dry_run: bool, interactive_mode: bool) {
        for mut rule in self.rules.clone() {
            rule.execute(path.clone(), dry_run, interactive_mode);
        }
    }

    /// Return the filename related to the current `Configuration`
    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    /// Return a specific `Rule` present in the current `Configuration`
    pub fn get_rule(&self, index: usize) -> Rule {
        self.rules[index].clone()
    }

    /// Return all the rules
    pub fn get_rules(&self) -> &Vec<Rule> {
        &self.rules
    }

    /// Create a new Config object from the path `test.crc` file
    pub fn new_for_test() -> Self {
        let conf_file_props: (String, String) =
            App::conf_file_lines(&String::from("./tests/test.crc"));
        let conf_lines: Vec<&str> = conf_file_props.1.split('\n').collect();
        let mut rules: Vec<Rule> = vec![];

        for i in 0..conf_lines.len() {
            let rule = Rule::new_from_line(&conf_lines.clone()[i].to_string(), &i);
            if rule.get_line_number() != &0 {
                rules.push(rule);
            }
        }

        let filename = conf_file_props.0.to_string();

        Self { filename, rules }
    }

    /// Create a new Config object from the path of a `.crc` file
    pub fn new(file: String) -> Self {
        let conf_file_props: (String, String) = App::conf_file_lines(&file.to_string());
        let conf_lines: Vec<&str> = conf_file_props.1.split('\n').collect();
        let mut rules: Vec<Rule> = vec![];

        for i in 0..conf_lines.len() {
            let rule = Rule::new_from_line(&conf_lines.clone()[i].to_string(), &i);
            if rule.get_line_number() != &0 {
                rules.push(rule);
            }
        }

        let filename = conf_file_props.0.to_string();

        Self { filename, rules }
    }
}
