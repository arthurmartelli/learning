fn main() {
    let person = Human;
    Wizard::fly(&person); // prints "Up!"
    person.fly(); // prints "*waving arms furiously*"

    <Human as Pilot>::fly(); // prints"This is your captain speaking."

    let p = Point { x: 1, y: 2 };

    p.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");

    w.outline_print();
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {
    value: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;
        Some(self.value)
    }
}

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Self;

    fn add(self, rhs: Meters) -> Self::Output {
        Self(self.0 + (rhs.0 * 1000))
    }
}

trait Pilot {
    fn fly() {
        // non-method function
        println!("This is your captain speaking.")
    }
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!")
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl OutlinePrint for Wrapper {}
