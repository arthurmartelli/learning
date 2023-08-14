#![allow(unused)]
use std::io;

fn scalar_types() -> u32 {
    // values that represent a single value, like numbers
    let x = 57u32;
    return x;
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0 bc int division

    // remainder
    let remainder = 43 % 5;
}

fn boolean() {
    let t = true;
    let f: bool = false;
}

fn characters() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

fn compound_types() {
    // Tuples: can have different types, are inmutable once created
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // tuple deconstruction
    let second = tup.2; // to access a value use .index
    println!("The value of y is: {y}");
    println!("The value of second is: {second}");

    // Array: have to hold the same type, have fixed length
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let x: [i32; 5] = [3; 5]; // same as [3, 3, 3, 3, 3]

    // accessing array elements is done with array[index]
    let first = a[0];
    let second = a[1];
}

fn invalid_element() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // program will panic if the input is not 1, 2, 3 or 4
}

fn main() {
    let x = scalar_types();
    println!("{x}");
    numeric_operations();
    boolean();
    characters();
    compound_types();
    invalid_element();
}
