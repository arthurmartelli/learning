use std::{thread, time::Duration};

fn main() {
    let message = "Message!";

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Hi number {i} from thread spawned {message}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    println!("{}", message);

    handle.join().unwrap();
}
