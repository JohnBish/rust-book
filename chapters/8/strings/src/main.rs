fn main() {
    let data = "initial contents";

    let s = data.to_string();

    let s = "inital contents".to_string(); // Same thing

    let s = String::from("initial contents"); // Same thing

    let mut s = String::from("foo");
    
    s.push_str("bar");
    s.push('!');

    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{}-{}-{}", s1, s2, s3); // s4 does not take ownership of any string

    //let s4 = s1 + &s2 + &s3; // s4 takes ownership of s1
    let s1 = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s1);
    println!("{}", s4);

    let s5 = String::from("Hello World!");

    for c in s5.chars() {
        print!("{}", c);
    }
    println!();

    for b in s5.bytes() {
        print!("{}, ", b);
    }
    println!();

    println!("{}", &s5[0..5]);
}
