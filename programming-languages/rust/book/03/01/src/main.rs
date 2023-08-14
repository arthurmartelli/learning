fn mutability() {
    println!("--- MUTABILITY ---");
    // variables are immutable by default

    let mut x: i32 = 5; // adding mut when creating the variable
    println!("The value of x is: {x}");

    x = 6; // lets us change the value later, but has to be of the same type
    println!("The value of x is: {x}");
}

fn shadowing() {
    println!("--- SHADOWING ---");
    // you can declare a variable with the same name of a previous one

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main() {
    mutability();
    shadowing();
}
