fn main() {
    let duh = if 1 == 1 {
        1
    } else {
        0
    };
    
    println!("{}", duh);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("result is {}", result);

    for i in (1..10).rev() {
        print!("{}", i);
    }
    println!();

    let ten_to_fifty = [10, 20, 30, 40, 50];
    for elem in ten_to_fifty.iter() {
        print!("{}", elem);
    }
    println!();
}
