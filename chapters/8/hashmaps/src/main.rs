use std::collections::HashMap;

fn main() {
    //let scores = HashMap::new();
    
    //scores.insert(String::from("Blue"), 10);
    //scores.insert(String::from("Orange"), 40);

    let teams = vec![String::from("Blue"), String::from("Orange")];
    let points = vec![10, 40];

    let scores: HashMap<&String, &i32> = teams.iter().zip(points.iter()).collect(); // Alternatively, HashMap<_, _>

    println!("{:?}", scores);

    let blue_score = scores.get(&"Blue".to_string());

    match blue_score {
        Some(v) => println!("{}", v),
        None => (),
    }

    for (k, v) in scores {
        println!("{}: {}", k, v);
    }

    let name = String::from("Favourite colour");
    let value = String::from("Blue");

    let mut map = HashMap::new();
    
    //map.insert(&name, &value); // map does not take ownership

    map.insert(name, value);
    // At this point, name and value are invalid since map takes ownership.
    // This would not occur with types which implement the Copy trait, like i32.

    map.insert(String::from("Favourite colour"), String::from("Green")); // Overwrites previous value

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for w in text.split_whitespace() {
        let count = map.entry(w.to_string()).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
