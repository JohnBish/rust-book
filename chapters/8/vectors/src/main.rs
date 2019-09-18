#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    //let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    println!("{:?}", v);

    v.push(4);

    println!("{:?}", v);

    let third: &i32 = &v[2];

    match v.get(2) { // Returns Option<&i32>
        Some(third) => println!("The third element is {}", third),
	None => println!("There is no third element"),
    }

    {
        for i in &mut v {
            *i += 50;
        }
    }

    for i in &v {
        print!("{}, ", i);   
    }
    println!();

    let mut row = vec![ // Using an Enum allows us to store multiple types in a vector
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("1337 h4x0r")),
        SpreadsheetCell::Float(137.036),
    ];

    println!("{:?}", row);

    println!("{:?}", row.pop());
}
