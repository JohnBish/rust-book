#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

pub fn add_two(n: i32) -> i32 {
    n + 2
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn pain() {
        panic!("Crash and burn!");
    }
    */

    #[test]
    fn rect_can_hold() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn rect_cant_hold() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(1), 3);
    }

    #[test]
    fn it_still_adds_two() {
        assert_ne!(add_two(2), 5);
    }

    #[test]
    fn greet_contains_name() {
        let result = greet("Carol");

        assert!(
            result.contains("Carol"),
            "Actual value {} did not contain name", result
        );
    }

    #[test]
//  #[should_panic]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // We can return a result instead of conditionally panicking
    // This allows us to use the ? operator
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {} // We'll pretend this takes a while
}
