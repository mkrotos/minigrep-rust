use config::Config;
use custom_errors::ErrorType;
use std::fs;

pub mod config;
pub mod custom_errors;

pub fn run(config: Config) -> Result<(), ErrorType> {
    let file_content = fs::read_to_string(&config.file_name)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // for line in content.lines() {...}
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    trait MultilineString {
        fn trim_lines(&self) -> String {
            self.trim_lines_on('|')
        }
        fn trim_lines_on(&self, separator: char) -> String;
    }

    impl MultilineString for str {
        fn trim_lines_on(&self, separator: char) -> String {
            self.lines()
                .map(|line| line.trim_start().trim_start_matches(separator))
                .collect::<Vec<&str>>()
                .join("\n")
                .to_string()
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
        |Rust:
        |fast, productive?, safe.
        | Really?
        |Duct."
            .trim_lines();

        assert_eq!(vec!["fast, productive?, safe."], search(query, &content));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
        |Rust:
        |fast, productive?, safe.
        | Really?
        |Trust me."
            .trim_lines();

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, &content)
        );
    }
}
