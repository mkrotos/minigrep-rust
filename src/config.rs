use crate::custom_errors::ErrorType;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, ErrorType> {
        let ignore_case_option = "-i";

        if args.len() < 3 {
            return Err(ErrorType::MissingArgument);
        }
        let query = args[1].clone();
        let file_name = args[2].clone();

        let ignore_case = if args.contains(&ignore_case_option.to_string()) {
            true
        } else {
            env::var("IGNORE_CASE").is_ok()
        };

        Ok(Config { query, file_name, ignore_case })
    }
}
