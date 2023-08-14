#![allow(unused)]

use std::{
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

const HELLO: &str = "hello.txt";

fn read_username_from_file_long() -> Result<String, io::Error> {
    let username_file_result = File::open(HELLO);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => return Ok(username),
        Err(e) => return Err(e),
    };
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(HELLO)?.read_to_string(&mut username)?;

    return Ok(username);
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    return fs::read_to_string(HELLO);
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    return text.lines().next()?.chars().last();
}

fn main_long() -> () {
    let greeting_file = match File::open(HELLO) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(HELLO) {
                Ok(new_file) => new_file,
                Err(new_error) => panic!("Problem creating the file: {new_error:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    return;
}

fn main() -> Result<(), Box<dyn Error>> {
    let greet_file: File = File::open(HELLO)?;

    return Ok(());
}
