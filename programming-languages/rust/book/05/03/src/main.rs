fn main() {
    let mut rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    rect1.scale(5);

    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    let sq: Rectangle = Rectangle::square(3);

    dbg!(sq.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn scale(&mut self, scale: u32) -> () {
        self.width = self.width * scale;
        self.height = self.height * scale;
        return;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
