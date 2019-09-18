pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

// A public enum's variants are all public
pub enum Soup {
    ChickenNoodle,
    RedPepper,
    RedHotChiliPepper,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
	    toast: String::from(toast),
	    seasonal_fruit: String::from("peaches"),
        }
    }
}

