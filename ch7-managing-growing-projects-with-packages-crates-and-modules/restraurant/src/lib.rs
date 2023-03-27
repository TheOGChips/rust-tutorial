use std::fmt::Result;
use std::io::Result as ioResult;
use std::io::{self,
              Write};

mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    
    impl Breakfast {
        //NOTE: This function is necessary because Breakfast has a private field
        pub fn summer (toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            };
        }
    }
    
    pub enum Appetizer {
        Soup,
        Salad,
    }
    
    fn fix_incorrect_order () {
        cook_order();
        super::deliver_order();
    }
    
    fn cook_order () {}
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant () {
    //absolute path
    //crate::front_of_house::hosting::add_to_waitlist();
    
    //relative path
    //front_of_house::hosting::add_to_waitlist();
    
    hosting::add_to_waitlist();
    
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind aobut what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    //meal.seasonal_fruit = String::from("blueberries");
    
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order () {}
