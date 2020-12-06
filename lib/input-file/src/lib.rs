use std::env;
use std::fs;
use std::path::{Path, PathBuf};

static DEFAULT_FILE: &'static str = "./inputs/input.txt";

/// Returns list of numbers from file at path specified in arguments passed in
/// to the app.  If no file is specified, it uses the default.
///
/// # Arguments
///
/// * `delimiter` - sets the delimiter used to split the file
pub fn get_numbers(delimiter: &str) -> Vec<i32> {
    return get_strings(delimiter)
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}

/// Returns the path to the file specified in the arguments passed into the app
/// and returns a default if no arguments can be found.
fn get_path_from_args() -> PathBuf {
    return Path::new(
        &((env::args().collect::<Vec<String>>()).get(1))
            .cloned()
            .unwrap_or(DEFAULT_FILE.to_string()),
    )
    .to_path_buf();
}

/// Returns list of strings from file at path specified in arguments passed in
/// to the app with blank entries removed.  If no file is specified, it uses
/// the default.
///
/// # Arguments
///
/// * `delimiter` - sets the delimiter used to split the file
pub fn get_strings(delimiter: &str) -> Vec<String> {
    let path = get_path_from_args();
    return fs::read_to_string(&path)
        .expect(&format!(
            "failed to read file: {}, put file where it belongs or specify a file",
            path.as_path().display().to_string()
        ))
        .split(delimiter)
        .into_iter()
        .filter(|x| x.to_owned() != "")
        .map(|x| x.to_owned())
        .collect();
}
