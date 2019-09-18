use std::{env, process};

#[derive(Debug)]
pub struct Params {
    args: Vec<String>,
}

impl Params {
    pub fn new() -> Params {
        Params {
            args: env::args()
               .rev().collect::<Vec<String>>()
        }
    }

    pub fn next(&mut self) -> Result<String, ()> {
        match self.args.pop() {
            Some(value) => Ok(value),
            None => Err(()),
        }
    }
}
