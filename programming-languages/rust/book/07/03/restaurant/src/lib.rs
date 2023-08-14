#![allow(unused)]

use back_of_house::Breakfast;

use crate::back_of_house::Appetizer;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // deliver_order() is out of scope

        // super brings it into scope of the module
        // by going to the parent module
        super::deliver_order()
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod front_of_house {
    // modules needs to be public for it to be called outside
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // order a breakfast in the menu with Rye toast
    let mut meal: Breakfast = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1: Appetizer = back_of_house::Appetizer::Salad;
    let order2: Appetizer = back_of_house::Appetizer::Soup;

    front_of_house::hosting::add_to_waitlist();
}
