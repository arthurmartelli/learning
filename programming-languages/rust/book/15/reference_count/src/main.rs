use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

fn main() {
    let value5 = Rc::new(RefCell::new(5));
    let value3 = Rc::new(RefCell::new(3));
    let value4 = Rc::new(RefCell::new(4));

    let a = Rc::new(Cons(Rc::clone(&value5), Rc::new(Nil)));
    let b = Rc::new(Cons(Rc::clone(&value3), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::clone(&value4), Rc::clone(&a)));

    *value5.borrow_mut() += 10;

    dbg!(a);
    dbg!(b);
    dbg!(c);
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
