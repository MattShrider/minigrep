use minigrep::{run, MinigrepError};
use std::{env, process::ExitCode};

const USAGE: &str = "Usage: minigrep <search> <file>";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let search_pattern = args.get(0);
    if search_pattern.is_none() {
        eprintln!("Search pattern not provided. {}", USAGE);
        return ExitCode::FAILURE;
    }

    let file_path = args.get(1);
    if file_path.is_none() {
        eprintln!("File not provided. {}", USAGE);
        return ExitCode::FAILURE;
    }

    match run(file_path.unwrap(), search_pattern.unwrap()) {
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
                println!("{}: {}", line.line_num, line.line_content);
            }
            ExitCode::SUCCESS
        }
    }
}
