#![allow(unused)]

use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    let f: Thunk = Box::new(|| println!("hi"));

    f();
}

type Thunk = Box<dyn Fn() + Send + 'static>; // instead of writing it in every function
type Kilometer = i32;

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
    todo!()
}

use std::fmt;
use std::io::Error;

type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
