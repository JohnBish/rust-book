fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("{}", y);

    println!("{}", square(3));
}

fn square(x: i32) -> i32 {
    x * x
}
