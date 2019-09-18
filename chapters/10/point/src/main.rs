struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 4, y: 5 };

    let p2 = Point { x: 4., y: 5. };

    println!("{}", p2.get_x());

    // let p3 = Point { x: 4., y: 5 }; // Invalid because of mismatched types

    // If we wanted to allow mismatched types, we could use:
    // struct Point<T, U> {
    //     x: T,
    //     y: U,
    // }
}
