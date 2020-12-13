use anyhow::Result;
use std::env;
use std::fs;
use std::path::Path;

static DEFAULT_FILE: &str = "./inputs/input.txt";

pub struct Input {
    contents: String,
}

impl Input {
    pub fn new(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        Ok(Self { contents })
    }

    pub fn strings<'a>(&'a self, delimiter: &'a str) -> impl Iterator<Item = &str> + 'a {
        self.contents.split(delimiter).filter(|x| !x.is_empty())
    }

    pub fn numbers<'a, N: std::str::FromStr>(
        &'a self,
        delimiter: &'a str,
    ) -> impl Iterator<Item = N> + 'a {
        self.strings(delimiter).filter_map(|x| x.parse().ok())
    }

    pub fn from_args() -> Result<Self> {
        let mut args = env::args().skip(1);
        let arg = args.next().unwrap_or_else(|| DEFAULT_FILE.to_string());
        let path = Path::new(&arg);
        Self::new(&path)
    }
}
