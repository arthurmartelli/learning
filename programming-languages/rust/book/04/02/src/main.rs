#[allow(unused)]

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

fn main() {
    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s: String = String::from("hello");
    {
        let r1: &mut String = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems

    let r1: &String = &s; // No problems as it is  not mutable
    let r3: &String = &s; // No problems as it is  not mutable
    let r2: &mut String = &mut s; // If there are immutable references, there cannot be a mutable one

    return;
}

fn calculate_length(s: &String) -> usize {
    // providing a reference (like a pointer to the value)
    // the function borrows the value, it does not take ownership
    return s.len();
}
