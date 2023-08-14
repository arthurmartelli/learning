use mybox::*;

fn main() {
    let x: i32 = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

mod mybox {
    use std::ops::Deref;

    pub struct MyBox<T> {
        value: T,
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    impl<T> MyBox<T> {
        pub fn new(value: T) -> MyBox<T> {
            MyBox { value }
        }
    }
}
