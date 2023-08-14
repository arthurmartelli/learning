use std::{env, fs, io, path::Path};

pub const SEARCH_CASE_ENV_KEY: &str = "IGNORE_CASE";

pub struct Config {
    query: String,
    file_path: Box<Path>,
    ignore_case: bool,
}

impl Config {
    pub fn build<I: Iterator<Item = String>>(mut args: I) -> Result<Config, &'static str> {
        args.next();

        let query: String = match args.next() {
            Some(s) => s,
            None => return Err("Not enough arguments!"),
        };

        let file_path: Box<Path> = match args.next() {
            Some(s) => Path::new(s.as_str()).into(),
            None => return Err("Not enough arguments!"),
        };

        let ignore_case = env::var(SEARCH_CASE_ENV_KEY).is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    pub fn info(&self) -> String {
        format!(
            "Searching for '{}'\nIn file '{}'",
            self.query,
            self.file_path.display()
        )
    }

    pub fn file_as_string(&self) -> Result<String, io::Error> {
        fs::read_to_string(&self.file_path)
    }

    fn search_with_case(&self) -> Result<Vec<String>, io::Error> {
        Ok(self
            .file_as_string()?
            .lines()
            .filter(|line| line.contains(self.query.as_str()))
            .map(|line| String::from(line))
            .collect())
    }

    fn search_without_case(&self) -> Result<Vec<String>, io::Error> {
        Ok(self
            .file_as_string()?
            .lines()
            .filter(|line| {
                line.to_lowercase()
                    .contains(self.query.to_lowercase().as_str())
            })
            .map(|line| String::from(line))
            .collect())
    }

    pub fn search(&self) -> Result<Vec<String>, io::Error> {
        match self.ignore_case {
            true => return self.search_with_case(),
            false => return self.search_without_case(),
        }
    }
}

pub fn run(config: Config) -> Result<(), io::Error> {
    for line in config.search()? {
        println!("{line}")
    }

    Ok(())
}
