#![allow(dead_code)]

mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

// Absolute use
use crate::back_of_house::{Appetizer, Breakfast};
// Relative use
use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::seat_at_table();

    // Relative path
    hosting::seat_at_table();

    // Order a breakfast in the summer with rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modofy the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let _appetizer = Appetizer::Soup;
}
