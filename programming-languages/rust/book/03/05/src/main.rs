fn if_statement(num: i32) {
    if {
        let val = num * num;
        num < val // block returns if the number is less than itself squared
    } {
        // and prints this if the above is true
        println!("condition was true");
    } else {
        // or this if the expression was false
        println!("condition was false");
    }

    fn test_div(num: i32, den: i32) -> bool {
        let flag: bool = { num % den == 0 };
        if flag {
            println!("{num} is divisible by {den}")
        };

        flag
    }

    for den in 2..=5 {
        test_div(num, den);
    }

    let x: i32 = if num > 5 { 1 } else { 0 }; // as if is an statement, it returns a value

    // but the two things have to be of the same type
    // let x = if num > 5 { 1 } else { "0" } wont work
    println!("{x}")
}

fn repetition_with_loops() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result = {result}");

    let mut count = 0;
    'counting_up: loop {
        // loop name is 'counting_up (has to have the ')
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 8 {
                break; // breaks inner loop
            }
            if count == 2 {
                break 'counting_up; // breaks the main loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        // loops while the condition is true
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < 5 {
        // this works for looping through the collection
        // but is very error prone, as we hard code the index maximum value
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        // this is a better approach for looping through a collection
        // it is a safer code and produces the same output
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    println!("Hello, world!");
    if_statement(5);
    repetition_with_loops();
    while_loop();
    for_loop();
}
