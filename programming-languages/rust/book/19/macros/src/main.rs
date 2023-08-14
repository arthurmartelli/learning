fn main() {
    println!("Hello, world!");

    let x = vec_new!(1, 2, 4);
}

/// Implementation of vec! macro
#[macro_export]
macro_rules! vec_new {
    () => {
        Vec::new()
    };

    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
