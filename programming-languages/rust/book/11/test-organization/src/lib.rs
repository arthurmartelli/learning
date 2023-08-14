#![allow(unused)]

use num;

pub fn add_two<T: num::Num + std::convert::From<i32>>(a: T) -> T {
    return a + 2.into();
}

fn internal_adder<T, U>(a: T, b: U) -> T
where
    T: num::Num + std::convert::From<U>,
    U: num::Num,
{
    return a + b.into();
}

pub struct Tester {
    name: String,
}

impl Tester {
    pub fn new(name: String) -> Self {
        return Tester { name };
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result: i32 = 2 + 2;
        assert_eq!(4, result)
    }

    #[test]
    fn internal_adder_test() {
        assert_eq!(4_000_f32, internal_adder(3_998_f32, 2_i8))
    }
}
