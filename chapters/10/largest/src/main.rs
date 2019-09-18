fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list.iter() {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn main() {
    println!("{}", largest(&[1, 3, 5, 2, 4]));
}
