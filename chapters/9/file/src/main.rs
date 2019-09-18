use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;

/*
fn main() {
    let f = File::open("hello world");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("{:?}", err),
            },
            other_error => panic!("{:?}", other_error),
        }
    };

    // If we ONLY want to unwrap:
    // let f = File::open("hello world").unwrap();
    // OR
    // let f = File::open("hello world").expect("Failed to open file.");
}
*/

fn read_username_from_file(fc: &str) -> Result<String, io::Error> {
    let mut f = File::open(fc)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Alternatively:

// let mut s = String::new();
// File::open(&str)?.read_to_string(&mut s)?;
// Ok(s)

// OR

// fs.read_to_string(&str)

fn main() {
    read_username_from_file("hello.txt");
}

// Alternatively,

// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//     Ok(())
// }
