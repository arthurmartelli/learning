#![allow(unused)]

use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        println!("{}", x);
        return x;
    } else {
        println!("{}", y);
        return y;
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_return(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        return self.part;
    }
}

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn longest_with_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

fn main() {
    let result: &str;
    let s1: String = String::from("abcd");

    {
        let s2: String = String::from("abc");
        result = longest(s1.as_str(), s2.as_str());
    };

    let novel: String = String::from("This will be a Novel. A long novel.");
    let fist_sentence: &str = novel.split(".").next().expect("Could not find a '.'");
    let i: ImportantExcerpt = ImportantExcerpt {
        part: fist_sentence,
    };
}
