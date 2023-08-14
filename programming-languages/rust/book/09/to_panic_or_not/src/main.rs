#![allow(unused)]

use std::net::IpAddr;

mod guess {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}

fn main() {
    use guess::Guess;

    let home: IpAddr = "127.0.0.1"
        .parse::<IpAddr>()
        .expect("Hardcoded IP addresses should be valid");

    let guess: Guess = Guess::new(5);
    guess.value();
}
