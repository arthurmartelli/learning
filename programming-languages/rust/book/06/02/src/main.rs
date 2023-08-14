fn main() {
    let q: u8 = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{q}")
}

#[derive(Debug)] // so we can inspect the state in a minute
#[allow(dead_code)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[allow(dead_code)] // so we can inspect the state in a minute
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => return 1,
        Coin::Nickel => return 5,
        Coin::Dime => return 10,
        Coin::Quarter(state) => {
            println!("Quarter from {state:?}");
            return 25;
        }
    }
}
