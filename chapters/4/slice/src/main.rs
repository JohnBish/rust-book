fn main() {
    let mut s = String::from("Hello World!");
    
    //let first = first_word(&s);
    let better_first = better_first_word(&s[..]);
    
    //println!("{}", first);

    //s.clear();

    //println!("{}", first);

    println!("{}", better_first);
}

fn first_word(s: &String) -> String {
    let mut first = String::from("");

    let s = s.as_bytes();

    for c in s {
        if *c == b' ' {
            break;
        }

        first.push(*c as char);
    }

    first
}

fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (idx, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[..idx];
        }
    }

    &s[..]
}
