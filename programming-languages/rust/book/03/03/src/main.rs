fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(6, 'h');
    statements_and_expressions();
    let x: i32 = print_and_return_num(5); // same as let x = 5, but prints the num
    println!("x = {x}");
}

fn another_function(x: i32) {
    // x is a parameter of this function
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    // multiple parameters get separated by a comma
    println!("The measurement is: {value}{unit_label}");
}

fn statements_and_expressions() {
    // statements are instructions that perform some action and do not return a value
    // expressions evaluate to a resulting value
    let y: i32 = 6; // statement, does not return a value
                    // you cannot do let x = (let y = 6)

    let x: i32 = {
        let x: i32 = 5;
        x + 1 // expression
    };

    println!("{y} {x}")
}

fn print_and_return_num(x: i32) -> i32 {
    println!("The number is: {x}");
    x
}
