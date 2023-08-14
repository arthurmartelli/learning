use List::{Cons, Nil};

fn main() {
    let b: Box<i32> = Box::new(5);
    let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    dbg!(b);
    dbg!(list);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
