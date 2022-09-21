use super::parser::{parse_options, CliOption};
use std::collections::HashMap;

pub struct Config {
    value_options: HashMap<String, String>,
    positional: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config {
            value_options: HashMap::new(),
            positional: Vec::new(),
        };

        config
            .value_options
            .insert("pa".to_string(), "false".to_string());
        config
            .value_options
            .insert("pat".to_string(), "false".to_string());
        config
            .value_options
            .insert("ssir".to_string(), "false".to_string());

        let options = parse_options();
        for option in options {
            match option {
                CliOption::OptionValue(name, value) => {
                    config.value_options.insert(name, value);
                }
                CliOption::Positional(value) => {
                    config.positional.push(value);
                }
                CliOption::TrueBool(name) => {
                    config.value_options.insert(name, "true".to_string());
                }
            }
        }

        config
    }

    pub fn get_bool<S: Into<String>>(&self, name: S) -> bool {
        self.value_options
            .get(&name.into())
            .unwrap()
            .parse::<bool>()
            .unwrap()
    }
}
