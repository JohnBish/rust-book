use std::io::{Error, ErrorKind};

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, Error> {
        if value < 1 || value > 100 {
            return Err(Error::new(ErrorKind::InvalidInput, "Value must be between 1 and 100 inclusive."));
        }
        
        Ok(Guess { value })
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Hello, world!");
}
