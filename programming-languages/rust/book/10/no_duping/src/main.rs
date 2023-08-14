#![allow(unused)]

pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> Point<X, Y> {
    fn x(&self) -> &X {
        return &self.x;
    }

    fn y(&self) -> &Y {
        return &self.y;
    }

    fn mixup<XO, YO>(self, other: Point<XO, YO>) -> Point<X, YO> {
        // Create a point with original X and the new Y
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() -> () {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let largest_num: &i32 = largest(&number_list);

    println!("The largest number is {}", largest_num);

    let p: Point<i32, i32> = Point { x: 5, y: 1 };

    return;
}
