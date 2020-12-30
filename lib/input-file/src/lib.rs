use anyhow::Result;
use std::env;
use std::fs;
use std::path::Path;

static DEFAULT_FILE: &str = "./inputs/input.txt";

pub struct Input {
    contents: String,
}

impl Input {
    /// Creates a new Input containing the contents of the file at the
    /// specified path.
    pub fn new(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        Ok(Self { contents })
    }

    /// Returns an iterator of strings split by the specified delimiter.
    pub fn strings<'a>(&'a self, delimiter: &'a str) -> impl Iterator<Item = &str> + 'a {
        self.contents.split(delimiter).filter(|x| !x.is_empty())
    }

    /// Returns an iterator of numbers split by the specified delimiter.
    pub fn numbers<'a, N: std::str::FromStr>(
        &'a self,
        delimiter: &'a str,
    ) -> impl Iterator<Item = N> + 'a {
        self.strings(delimiter).filter_map(|x| x.parse().ok())
    }

    /// Creates a new input containing the contents of the file specified in
    /// the first argument passed to the program.
    pub fn from_args() -> Result<Self> {
        let mut args = env::args().skip(1);
        let arg = args.next().unwrap_or_else(|| DEFAULT_FILE.to_string());
        let path = Path::new(&arg);
        Self::new(&path)
    }
}
