fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let s1 = String::from("abcde");
    let s2 = "wxyz";

    println!("Longest is {}", longest(s1.as_str(), s2));

    {
        let s3 = "UwU";
        
        let result = longest(s3, s2);

        println!("Longest is {}", result);
    }

    // result is no longer valid since its lifetime is inherited from s3.
    // The SMALLER lifetime is inherited even though s2 was returned.
}
