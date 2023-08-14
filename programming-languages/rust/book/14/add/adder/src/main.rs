use add_one;

fn main() -> () {
    let number: usize = 10;
    println!("Hello, adding one is {}!", add_one::add_one(number));
    println!("Hello, the answer is {}!", add_one::add_rand(number));
}
