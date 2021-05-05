use crate::App;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{stdin, stdout, Write};

const FIRST_VALID_TOKENS: [&str; 3] = ["mv", "pre", "suf"];
const SECOND_VALID_TOKENS: [&str; 2] = ["to", "with"];

const TOKEN_PAIRS_INDEXES: [[i8; 2]; 3] = [
    [0, 0], // mv  _ to   _
    [1, 1], // pre _ with _
    [2, 1], // suf _ with _
];

/// A Rule is a line that was parsed and verify to work.
/// It can execute the following tasks :
///     Move multiple files to a destination path;
///     Add a prefix to multiple files;
///     Add a suffix to multiple files.
/// All these actions are determined by the Configuration structure and implementation
#[derive(Clone, Debug)]
pub struct Rule {
    line_number: usize,
    line_value: String,
    valid: bool,
    tokens: Vec<String>,
    error_message: String,
    first_keyword: String,
    first_pattern: String,
    second_keyword: String,
    second_pattern: String,
}
impl Rule {

    /// Return the line number, where the current `Rule` was defined
    pub fn get_line_number(&self) -> &usize {
        &self.line_number
    }

    /// Return the line value of the current `Rule`
    pub fn get_line_value(&self) -> &String {
        &self.line_value
    }

    /// Return a boolean defining if the current `Rule` can be executed or not
    pub fn is_valid(&self) -> &bool {
        &self.valid
    }

    /// Return all the tokens in the current `Rule`
    pub fn get_tokens(&self) -> &Vec<String> {
        &self.tokens
    }

    /// Return a specific token in the current `Rule`
    pub fn get_token(&self, index: usize) -> &String {
        &self.tokens[index]
    }

    /// Add a token to the current `Rule`
    pub fn add_token(&mut self, token: String) {
        &self.tokens.push(token);
    }

    /// Return the error message
    pub fn get_error_message(&self) -> &String {
        &self.error_message
    }

    /// Create a new `Rule`
    pub fn new(
        line_value: String,
        line_number: usize,
        valid: bool,
        tokens: Vec<String>,
        error_message: String,
        first_keyword: String,
        first_pattern: String,
        second_keyword: String,
        second_pattern: String,
    ) -> Self {
        Self {
            line_value,
            line_number,
            valid,
            tokens,
            error_message,
            first_keyword,
            first_pattern,
            second_keyword,
            second_pattern,
        }
    }

    /// Add a prefix to a file
    fn prefix(&mut self, file_path: &String, prefix: &String) -> bool {
        match fs::rename(file_path, format!("{}{}", prefix, file_path)) {
            Err(_) => return false,
            Ok(_) => return true,
        };
    }

    /// Add a suffix to a file
    fn suffix(&self, file_path: &String, suffix: &String) -> bool {
        let v: Vec<&str> = file_path.split(".").collect();
        let file_name: &str = v[0];
        let extension: &str = v[1];
        match fs::rename(file_path, format!("{}{}.{}", file_name, suffix, extension)) {
            Err(_) => return false,
            Ok(_) => return true,
        };
    }

    /// Make a copy of a file from the source full path towards the destination full path
    fn copy_file(src: &String, dest: &String) -> std::io::Result<()> {
        File::create(dest)?;
        fs::copy(src, dest)?;
        Ok(())
    }

    /// Creates a new file in the destination path
    fn copy_to(&self, file_path: &String, destination: &String) -> bool {
        let destination_full_path = match file_path.chars().last().unwrap() {
            '\\' => format!("{}{}", destination, file_path),
            '/' => format!("{}{}", destination, file_path),
            _ => format!("{}\\{}", destination, file_path),
        };
        match Self::copy_file(&file_path, &destination_full_path) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    /// Delete origin file from the origin file path
    fn delete_origin(&self, origin_file_path: &String) -> bool {
        match fs::remove_file(origin_file_path) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    /// Return a `Vec<String>` with the names of files concerned by a `Rule`
    fn files_concerned(&mut self, dir: String) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let paths = fs::read_dir(dir).unwrap();

        for path in paths {
            let tmp0 = path.unwrap().path();
            let tmp = tmp0.to_str().unwrap().split("\\");
            for el in tmp {
                let replaced_pattern =
                    &self.first_pattern.replace("*", "[\\w,\\s,\\-,\\[,\\]]{0,}");
                if Regex::new(&format!(r"^{}$", &replaced_pattern))
                    .unwrap()
                    .is_match(el)
                {
                    &res.push(el.to_string());
                }
            }
        }
        res
    }

    /// Asks the user if he wants to pursue a `Rule` execution
    fn interactive_mode_question(&self, question: String, file: &str, input: &mut String) -> bool {
        println!("{}", question);
        stdout().flush().unwrap();
        stdin().read_line(input).expect("Failed to read line");
        if input.to_string().replace("\r", "").replace("\n", "") == String::from("y") {
            println!("continuing...");
            true
        } else if input.to_string().replace("\r", "").replace("\n", "") == String::from("n") {
            println!("skipping...");
            false
        } else {
            return self.interactive_mode_question(question, file, input);
        }
    }

    /// Move the files that matches with the first pattern to the second pattern (PATH)
    fn mv_to(&mut self, dir: String, dry_run: bool, interactive_mode: bool) -> bool {
        let mut input: String = String::new();
        let files: Vec<String> = self.files_concerned(dir);
        for file in files {
            if interactive_mode {
                let _ = stdout().flush();
                if self.interactive_mode_question(
                    format!(
                        "Are you sure you want to move {} to {} ? (y or n)",
                        &file, &self.second_pattern
                    ),
                    &file,
                    &mut input,
                ) {
                    if dry_run {
                        self.dry_run("mv");
                        return true;
                    } else {
                        return self.copy_to(&file, &self.second_pattern)
                            && self.delete_origin(&file);
                    }
                } else {
                    continue;
                }
            } else {
                if dry_run {
                    self.dry_run("mv");
                    return true;
                } else {
                    return self.copy_to(&file, &self.second_pattern) && self.delete_origin(&file);
                }
            }
        }
        false
    }

    /// Add a prefix at the beginning of the files that matches with the first pattern
    fn pre_with(&mut self, dir: String, dry_run: bool, interactive_mode: bool) -> bool {
        let mut input: String = String::new();
        let files: Vec<String> = self.files_concerned(dir);
        for file in files {
            if interactive_mode {
                let _ = stdout().flush();
                if self.interactive_mode_question(
                    format!(
                        "Are you sure you want to prefix {} with {} ? (y or n)",
                        &file, &self.second_pattern
                    ),
                    &file,
                    &mut input,
                ) {
                    if dry_run {
                        self.dry_run("pre");
                        return true;
                    } else {
                        return self.prefix(&file, &self.second_pattern.clone());
                    }
                } else {
                    continue;
                }
            } else {
                if dry_run {
                    self.dry_run("pre");
                    return true;
                } else {
                    return self.prefix(&file, &self.second_pattern.clone());
                }
            }
        }
        false
    }

    /// Add a suffix at the end of files that matches with the first pattern
    fn suf_with(&mut self, dir: String, dry_run: bool, interactive_mode: bool) -> bool {
        let mut input: String = String::new();
        let files: Vec<String> = self.files_concerned(dir);
        for file in files {
            if interactive_mode {
                let _ = stdout().flush();
                if self.interactive_mode_question(
                    format!(
                        "Are you sure you want to suffix {} with {} ? (y or n)",
                        &file, &self.second_pattern
                    ),
                    &file,
                    &mut input,
                ) {
                    if dry_run {
                        self.dry_run("suf");
                        return true;
                    } else {
                        return self.suffix(&file, &self.second_pattern.clone());
                    }
                } else {
                    continue;
                }
            } else {
                if dry_run {
                    self.dry_run("suf");
                    return true;
                } else {
                    return self.suffix(&file, &self.second_pattern.clone());
                }
            }
        }
        false
    }

    /// Prints a success message
    fn success(&self) {
        println!(
            "success: rule line {} - {}",
            self.line_number, self.line_value
        );
    }

    /// Prints an error message
    fn error(&self) {
        println!("error: rule line {} is invalid.", self.line_number);
    }

    /// Shows a message if the app is launched with the dry-run
    fn dry_run(&self, action: &str) {
        match action {
            "mv" => {
                println!(
                    "[dry-run][line {}] move all files following the pattern `{}` to `{}`",
                    self.line_number, self.first_pattern, self.second_pattern
                );
            }
            "pre" => {
                println!(
                    "[dry-run][line {}] add prefix `{}` to all files following the pattern `{}`",
                    self.line_number, self.second_pattern, self.first_pattern,
                );
            }
            "suf" => {
                println!(
                    "[dry-run][line {}] add suffix `{}` to all files following the pattern `{}`",
                    self.line_number, self.second_pattern, self.first_pattern,
                );
            }
            _ => panic!(""),
        }
    }

    /// Execute the current `Rule`
    pub fn execute(&mut self, dir: String, dry_run: bool, interactive_mode: bool) {
        if self.error_message == String::from("") {
            if self.first_keyword == "mv" && self.second_keyword == "to" {
                if dry_run && !interactive_mode {
                    self.dry_run("mv");
                } else {
                    if self.mv_to(dir.clone(), dry_run, interactive_mode) && !dry_run {
                        self.success();
                    } else {
                        if !interactive_mode && !dry_run {
                            self.error();
                        }
                    }
                }
            } else if self.first_keyword == "pre" && self.second_keyword == "with" {
                if dry_run && !interactive_mode {
                    self.dry_run("pre");
                } else {
                    if self.pre_with(dir.clone(), dry_run, interactive_mode) && !dry_run {
                        self.success();
                    } else {
                        if !interactive_mode && !dry_run {
                            self.error();
                        }
                    }
                }
            } else if self.first_keyword == "suf" && self.second_keyword == "with" {
                if dry_run && !interactive_mode {
                    self.dry_run("suf");
                } else {
                    if self.suf_with(dir.clone(), dry_run, interactive_mode) && !dry_run {
                        self.success();
                    } else {
                        if !interactive_mode && !dry_run {
                            self.error();
                        }
                    }
                }
            } else {
                println!(
                    "error: rule line {},token {} and {} are not handled.",
                    self.line_number, self.first_keyword, self.second_keyword
                );
            }
        } else {
            self.error();
        }
    }

    /// Verify if valid pairs of keyword exists
    fn check_pairs(
        tokens: &Vec<String>,
        error_state: &bool,
        error_message: &String,
    ) -> (bool, String) {
        if tokens.len() < 4 {
            return (*error_state, error_message.to_string());
        }
        let error = (true, String::from("Keywords' pair doesn't exists."));
        for indexes in TOKEN_PAIRS_INDEXES.iter() {
            if FIRST_VALID_TOKENS[indexes[0] as usize] == tokens[0]
                && SECOND_VALID_TOKENS[indexes[1] as usize] == tokens[2]
            {
                return (*error_state, error_message.to_string());
            }
        }
        error
    }

    /// Verify if there is an error in the current `Rule`
    fn check_error_message(token: &String, i: usize) -> (bool, String) {
        let default = (false, String::new());
        match i {
            0 => match !FIRST_VALID_TOKENS.iter().any(|v| v == &token) {
                true => (true, String::from("Invalid first keyword.")),
                false => default,
            },
            1 => default,
            2 => match !SECOND_VALID_TOKENS.iter().any(|v| v == &token) {
                true => (true, String::from("Invalid second keyword.")),
                false => default,
            },
            3 => default,
            _ => (true, String::from("Query has too much elements.")),
        }
    }

    /// Returns an empty `Rule` to act as a skipped one
    fn skipped_rule(line_value: &str, tokens: Vec<String>) -> Self {
        Self::new(
            line_value.to_string(),
            0,
            true,
            tokens,
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        )
    }

    /// Returns a `Rule` with missing elements
    fn rule_missing_element(
        line_value: &str,
        line_number: usize,
        tokens: Vec<String>,
        message: String,
    ) -> Self {
        return Self::new(
            line_value.to_string(),
            line_number,
            false,
            tokens,
            message,
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        );
    }

    /// Creates a new `Rule` from a string
    pub fn new_from_line(line: &String, line_number: &usize) -> Self {
        let line_number = line_number + 1;
        let line_value = App::remove_comment(&line);
        let line_value = line_value.trim_end();
        let strings: Vec<&str> = line_value.split(" ").filter(|x| x != &"").collect();
        let mut tokens: Vec<String> = vec![];
        let mut error_message: (bool, String) = (false, String::new());

        if (strings.len() == 1 && &strings[0] == &"\r") || strings.len() == 0 {
            return Self::skipped_rule(line_value, tokens);
        } else if strings.len() < 4 {
            return Self::rule_missing_element(
                line_value,
                line_number,
                tokens,
                String::from("Missing keyword(s) or pattern(s)."),
            );
        }

        for i in 0..strings.len() {
            let token: String = if i <= 3 {
                strings[i].to_string()
            } else {
                String::new()
            };
            error_message = Self::check_error_message(&token, i);
            match error_message.0 {
                true => break,
                _ => tokens.push(token.clone()),
            }
        }

        error_message = Self::check_pairs(&tokens, &error_message.0, &error_message.1);

        Self::new(
            line_value.to_string(),
            line_number,
            !error_message.0,
            tokens.clone(),
            error_message.1,
            match tokens.len() >= 1 {
                true => tokens[0].to_string(),
                _ => String::new(),
            },
            match tokens.len() >= 2 {
                true => tokens[1].to_string(),
                _ => String::new(),
            },
            match tokens.len() >= 3 {
                true => tokens[2].to_string(),
                _ => String::new(),
            },
            match tokens.len() == 4 {
                true => tokens[3].to_string(),
                _ => String::new(),
            },
        )
    }
}
