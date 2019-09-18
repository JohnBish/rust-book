fn to_pig_latin(s: &String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut chars = s.chars();

    let first_char = chars.next();

    match first_char {
        Some(c) => {
            for v in vowels.iter() {
                if c == *v {
                    return s.clone() + "-hay";
                }
            }

            format!("{}-{}ay", chars.as_str(), c)
        },
        None => String::new(),
    }
}

fn main() {
    println!("{}", to_pig_latin(&String::from("apple")));
    println!("{}", to_pig_latin(&String::from("first")));
}
