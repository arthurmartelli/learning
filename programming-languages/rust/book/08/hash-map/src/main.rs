#![allow(unused)]

pub use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // we can get the value on the hash map
    let blue_team: String = String::from("Blue");
    let blue_score: i32 = scores.get(&blue_team).copied().unwrap_or_default();

    // we can iterate over the hash map
    for (k, v) in &scores {
        println!("{k}: {v}")
    }

    // we can add values (overwrite if key is present)

    scores.insert("Blue".to_string(), 3);
    scores.insert("Blue".to_string(), 6);
    println!("{:?}", scores); // Blue will be 6

    // we can add values (if key isn't present)
    scores.entry("Yellow".to_string()).or_insert(1);
    scores.entry("Red".to_string()).or_insert(3);
    scores.entry("Blue".to_string()).or_insert(6);

    println!("{:?}", scores);

    // updating a value based on an old value

    let text: &str = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    return;
}
