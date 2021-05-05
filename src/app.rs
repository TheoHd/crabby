use crate::components::Config;
use clap::{App as Clap, Arg as ClapArg, ArgMatches as ClapArgMatches};
use regex::Regex;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
/// The App object is here to determine arguments and know what configuration(s) should run and how
pub struct App {
    configurations: Vec<Config>,
    interactive_mode: bool,
    clean_path: String,
    conf_file_path: String,
    dry_run: bool,
}

impl App {
    /// Get a specific `Configuration`
    pub fn get_configuration(&self, index: usize) -> Config {
        self.configurations[index].clone()
    }

    /// Check if an argument was specified by the user
    pub fn is_present(matches: &ClapArgMatches, arg_str: &str) -> bool {
        matches.is_present(arg_str)
    }

    /// Create a new `Application` in a test environment
    pub fn new_for_test() -> Self {
        let matches = Self::cli_for_test();
        let mut configurations: Vec<Config> = Vec::<Config>::new();

        configurations.push(Config::new(App::get_arg(&matches, "file")));

        Self {
            configurations: configurations,
            interactive_mode: App::is_present(&matches, "interactive"),
            clean_path: App::get_arg(&matches, "clean"),
            conf_file_path: App::get_arg(&matches, "file"),
            dry_run: App::is_present(&matches, "dry-run"),
        }
    }

    /// Return the current configurations
    pub fn get_configurations(&self) -> &Vec<Config> {
        &self.configurations
    }

    /// Get the left content of a split result
    fn left(line: &str, c: &str) -> String {
        for el in line.split(c) {
            return String::from(el);
        }
        String::new()
    }

    /// Remove comment from a string
    pub fn remove_comment(line: &str) -> String {
        Self::left(line, "//")
    }

    /// Execute the configurations rules
    pub fn launch(&self) {
        for config in self.configurations.clone() {
            config.execute_rules(self.clean_path.clone(), self.dry_run, self.interactive_mode);
        }
    }

    /// Add a `Configuration` to the current `Application`
    pub fn add_config(&mut self, config: Config) {
        &self.configurations.push(config);
    }

    /// Return a new `Application`
    pub fn new() -> Self {
        let matches = Self::cli();
        let mut configurations: Vec<Config> = Vec::<Config>::new();

        configurations.push(Config::new(App::get_arg(&matches, "file")));

        Self {
            configurations: configurations,
            interactive_mode: App::is_present(&matches, "interactive"),
            clean_path: App::get_arg(&matches, "clean"),
            conf_file_path: App::get_arg(&matches, "file"),
            dry_run: App::is_present(&matches, "dry-run"),
        }
    }

    /// Return the default configurations lines from a `.crc` file in the root folder
    pub fn default_conf_lines() -> (String, String) {
        let current_dir =
            fs::read_dir(Path::new(".")).expect("Could not read the current directory.");
        let conf_file_regex = Regex::new(r"^*.crc$").unwrap();

        for files_and_folders in current_dir {
            match files_and_folders {
                Ok(file_or_folder) => {
                    let path = file_or_folder.path();
                    let path_str = match path.to_str() {
                        Some(x) => x,
                        None => continue,
                    };
                    if conf_file_regex.is_match(path_str) {
                        return (
                            path_str.to_string(),
                            fs::read_to_string(path_str)
                                .expect("Could not read the configuration file specified."),
                        );
                    }
                }
                Err(_) => continue,
            };
        }
        panic!("Could not find a configuration file in the current directory.")
    }

    /// Return the configuration lines
    pub fn conf_file_lines(configuration_file: &str) -> (String, String) {
        match configuration_file {
            "" => Self::default_conf_lines(),
            x => (
                x.to_string(),
                fs::read_to_string(configuration_file)
                    .expect("Could not read the configuration file specified."),
            ),
        }
    }

    /// Return the value of an argument
    pub fn get_arg(matches: &ClapArgMatches, arg_str: &str) -> String {
        match matches.value_of(arg_str) {
            Some(x) => x.to_string(),
            None => String::from(""),
        }
    }

    /// Return a ClapArgMatches to retrieve arguments values
    fn cli_switch(test: bool) -> ClapArgMatches<'static> {
        Clap::new("crabcleaner")
        .version("0.1.0")
        .about("CrabCleaner is a fast and safe way to manipulate files directly from your command line.")
        .author("Théo Huchard <theo.huchard@protonmail.com>")
        .arg(
            ClapArg::with_name("clean")
                .short("c")
                .long("clean")
                .value_name("PATH")
                .help("Path of the directory to clean")
                .required(!test)
                .takes_value(true)
        )
        .arg(
            ClapArg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE PATH")
                .help("File path that must be used for cleaning")
                .required(false)
                .takes_value(true)
        )
        .arg(
            ClapArg::with_name("dry-run")
                .long("dry-run")
                .help("List the effects of the rules without executing them")
                .required(false)
                .takes_value(false)
        )
        .arg (
            ClapArg::with_name("interactive")
                .long("interactive")
                .help("Shows the user all the rules to make him able to execute them one by one")
                .required(false)
                .takes_value(false)
        )
        .get_matches()
    }

    /// Return a ClapArgMatches to retrieve arguments values in a production environment
    pub fn cli() -> ClapArgMatches<'static> {
        Self::cli_switch(false)
    }

    /// Return a ClapArgMatches to retrieve arguments values in the test environment
    pub fn cli_for_test() -> ClapArgMatches<'static> {
        Self::cli_switch(true)
    }
}
