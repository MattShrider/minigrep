use minigrep::{run, MinigrepError};
use std::{env, process::ExitCode};

const USAGE: &str = "Usage: minigrep <search> <...files>";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let search_pattern = args.get(0);
    if search_pattern.is_none() {
        eprintln!("Search pattern not provided. {}", USAGE);
        return ExitCode::FAILURE;
    }

    let files = &args[1..];
    if files.is_empty() {
        eprintln!("File not provided. {}", USAGE);
        return ExitCode::FAILURE;
    }

    match run(&files, &search_pattern.unwrap()) {
        Err(why) => match why {
            MinigrepError::FileNotFound(file) => {
                eprintln!("File not found: {}", file);
                ExitCode::FAILURE
            }
            MinigrepError::UnexpectedFS(kind) => {
                eprintln!("Unexpected error {:?}", kind);
                ExitCode::FAILURE
            }
        },
        Ok(result) => {
            for line in result {
                if files.len() > 1 {
                    println!("{} {}: {}", line.filename, line.line_num, line.line_content);
                } else {
                    println!("{}: {}", line.line_num, line.line_content);
                }
            }
            ExitCode::SUCCESS
        }
    }
}
