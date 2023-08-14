use rand::Rng;
use std::{cmp::Ordering, io, ops::Range};

fn get_secret_number(range: Range<u32>) -> u32 {
    let secret_number: u32 = rand::thread_rng().gen_range(range);
    println!("The secret number is: {secret_number}");
    return secret_number;
}

fn get_input() -> u32 {
    loop {
        let mut input: String = String::new();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
        println!("You guessed: {input}");
        return input;
    }
}

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = get_secret_number(1..101);

    loop {
        let guess: u32 = get_input();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
