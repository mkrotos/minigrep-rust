use std::env;
use std::process;

use minigrep;
use minigrep::config::Config;
use minigrep::custom_errors::ErrorType;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        print_error_message(err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        print_error_message(e);
        process::exit(1);
    }
}

fn print_error_message(e: ErrorType) {
    match e {
        ErrorType::MissingArgument => eprintln!("Missing command line argument"),
        ErrorType::FileAccessError(err_type) => eprintln!("File access error: {}", err_type),
    }
}
