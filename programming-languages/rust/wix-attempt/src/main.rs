use std::fs;

fn main() {
    println!("Hello, world!");
    let content = fs::read("./assets/hi.txt").unwrap();
    println!("{:?}", content);
}
