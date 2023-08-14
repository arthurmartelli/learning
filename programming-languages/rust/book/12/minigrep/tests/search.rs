use minigrep::{self, Config};
use std::{env, vec::IntoIter};

const TEST_FILE_PATH: &str = "tests/test.txt";

#[test]
fn case_sensitive() {
    let first: String = String::from("NOT IMPORTANT");
    let query: String = String::from("duct");
    let file_path: String = String::from(TEST_FILE_PATH);

    let args: IntoIter<String> = vec![first, query, file_path].into_iter();
    let config: Config = Config::build(args).unwrap();
    env::remove_var(minigrep::SEARCH_CASE_ENV_KEY);

    assert_eq!(vec!["safe, fast, productive."], config.search().unwrap());
}

#[test]
fn case_insensitive() {
    let first = String::from("NOT IMPORTANT");
    let query = String::from("rust");
    let file_path = String::from(TEST_FILE_PATH);

    let args: IntoIter<String> = vec![first, query, file_path].into_iter();
    let config: Config = Config::build(args).unwrap();
    env::set_var(minigrep::SEARCH_CASE_ENV_KEY, "TRUE");

    assert_eq!(vec!["Rust:", "Trust me."], config.search().unwrap());
}
