use super::*;

#[derive(Debug)]
pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Printing Button! {:#?}", self)
    }
}
