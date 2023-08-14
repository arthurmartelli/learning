#![allow(unused)]

#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.width > other.height && self.height > other.width)
    }
}

fn add_tow(a: isize) -> isize {
    return a + 2;
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1 < value && value < 100) {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

pub fn greeting(name: &str) -> String {
    return format!("Hello {}", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_can_hold_test() -> () {
        let larger1: Rectangle = Rectangle {
            width: 8,
            height: 7,
        };
        let larger2: Rectangle = Rectangle {
            width: 2,
            height: 8,
        };
        let smaller: Rectangle = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger1.can_hold(&smaller));
        assert!(!larger1.can_hold(&larger2));

        assert!(larger2.can_hold(&smaller));
        assert!(!larger2.can_hold(&larger1));

        assert!(!smaller.can_hold(&larger1));
        assert!(!smaller.can_hold(&larger2));

        return;
    }

    #[test]
    fn fn_add_tow_test() -> Result<(), String> {
        if add_tow(2) == 4 {
            return Ok(());
        } else {
            return Err(String::from("2 + 2 has to be  4"));
        }
    }

    #[test]
    fn greeting_contains_name() -> () {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, the value was {}",
            result
        );
        return;
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guessing_new_test() -> () {
        Guess::new(100);
    }
}
