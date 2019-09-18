#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(dim: u32) -> Rectangle {
        Rectangle { length: dim, width: dim }
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 64,
        width: 32,
    };

    let rect2 = Rectangle {
        length: 10,
        width: 30,
    };

    let rect3 = Rectangle {
        length: 69,
        width: 69,
    };

    let rect4 = Rectangle::square(69);

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    println!("Area is {}", rect1.area());

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    println!("rect4 is {:?}", rect4);
}

