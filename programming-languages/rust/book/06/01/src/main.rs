#[allow(unused)]

fn main() {
    let home: IpAddr = IpAddr::V4(127, 0, 0, 1);
    let loop_back: IpAddr = IpAddr::V6(String::from("::1"));

    dbg!(home);
    let m: Message = Message::Write(String::from("Hi"));

    let some_number = Some(5);
    let some_char: Option<String> = Some("hi".to_string());
    let absent_number: Option<i32> = None;

    return;
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
