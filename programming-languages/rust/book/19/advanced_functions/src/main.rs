fn main() {
    let add_two = adder_factory(2);
    let answer = do_twice(add_one, 5);

    let answer2 = do_twice_better(add_two, 5);

    println!("The answer is: {answer}");
    println!("The answer is: {answer2}");
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("{list_of_statuses:?}");
}

fn adder_factory(y: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + y)
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

fn do_twice_better(f: Box<dyn Fn(i32) -> i32>, arg: i32) -> i32 {
    f(f(arg))
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}
