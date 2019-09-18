fn main() {
    let mut s1 = String::from("hello");

    let len = calc_len(&s1);

    println!("Lenghth of {} is {}", s1, len);

    change(&mut s1);

    println!("{}", s1);
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}
