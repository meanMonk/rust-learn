pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn deliver_order() {
    
}

pub mod back_of_house {
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
    
    pub enum Appetizer {
        Salad,
        Soup, 
        Maincourse
        
    }
}

pub mod front_of_house;

// making `use` to incrase scope and reduce repitative call of names.
use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    // absolute path .
    hosting::add_to_waitlist();
    
    front_of_house::fix_incorrect_order();
    
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please with", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    
    let _order1 = back_of_house::Appetizer::Salad;
    let _order2 = back_of_house::Appetizer::Soup;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
