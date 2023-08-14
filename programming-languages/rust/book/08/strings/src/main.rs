#![allow(unused)]

fn main() -> () {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    // or you can create it directly
    let s = String::from(data);
    let s = String::from("initial contents");

    // strings are UTF-8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // you can add to a string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // does not take ownership of s2
    println!("s2 is {s2}");
    s1.push('s'); // push takes one literal

    // using the add() method
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // and using format!() macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // avoid using string indexing directly on the string, use the .chars() first
    // then you can iterate over the characters
    for i in s.chars() {
        println!("{i}");
    }
    return;
}
