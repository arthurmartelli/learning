fn main() -> () {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for i in v1_iter {
        println!("Got: {i}")
    }

    let squares = vec![1, 2, 3];

    let squares: Vec<_> = squares.iter().map(|i| i * i).collect();

    println!("{:#?}", &squares);

    return;
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

impl Shoe {
    fn new(size: u32, style: String) -> Self {
        Self { size, style }
    }
}

fn shoes_by_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use crate::{shoes_by_size, Shoe};

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn shoe_filter_by_size() {
        let shoes = vec![
            Shoe::new(10, String::from("sneaker")),
            Shoe::new(13, String::from("sandal")),
            Shoe::new(10, String::from("boot")),
        ];

        let in_my_size = shoes_by_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe::new(10, String::from("sneaker")),
                Shoe::new(10, String::from("boot")),
            ]
        );
    }
}
