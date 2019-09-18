mod front_of_house;
mod back_of_house;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Realtive path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    // We can't modify seasonal_fruit since the field is private
}


// Alternatively, we can bring hosting into scope with use
/*
use crate::front_of_house::hosting;

// Or, as a relative path:
// use self::front_of_house::hosting;

// Or, re-export under new domain:
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
*/
