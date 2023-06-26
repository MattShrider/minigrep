use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub enum MinigrepError {
    UnexpectedFS(std::io::ErrorKind),
    FileNotFound(String),
}

#[derive(Debug)]
pub struct SearchMatch {
    pub line_num: usize,
    pub line_content: String,
}

pub fn run(filename: &String, search: &String) -> Result<Vec<SearchMatch>, MinigrepError> {
    let file = open_file(filename)?;
    let reader = BufReader::new(file);

    let found: Vec<SearchMatch> = reader
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| match line {
            Ok(found) => {
                if found.to_lowercase().contains(&search.to_lowercase()) {
                    Some(SearchMatch {
                        line_num: idx,
                        line_content: found,
                    })
                } else {
                    None
                }
            }
            // This is an unexpected error reading lines. Attempt to use as
            // many lines as possible
            Err(_) => None,
        })
        .collect();

    Ok(found)
}

pub fn open_file(filename: &String) -> Result<File, MinigrepError> {
    File::open(Path::new(filename)).or_else(|error| {
        Err(match error.kind() {
            std::io::ErrorKind::NotFound => MinigrepError::FileNotFound(filename.to_owned()),
            kind => MinigrepError::UnexpectedFS(kind),
        })
    })
}
