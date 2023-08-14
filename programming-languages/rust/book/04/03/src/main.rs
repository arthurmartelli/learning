#[allow(unused)]

// A string slice is a reference to part of a String

fn main() {
    let mut s: String = String::from("hi 🔥 world 🌎!");

    let hello: &str = &s[..7];
    let world: &str = &s[8..];

    let test: &str = first_word(&s);

    println!("{hello}, {world}, {test}");

    let a: [i32; 4] = [-1, 0, 1, 2];
    let b: &[i32] = &a[1..=2]; // Slices are also available for arrays

    assert_eq!(b, &[0, 1]);

    return;
}

fn first_word(s: &str) -> &str {
    // using s: &str allows using types &str, &String and their slices
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}
