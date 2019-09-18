use std::process;

use minigrep::Config;
use minigrep::parser::Params;

fn main() {
    let mut params = Params::new();
    
    let cfg = Config::new(&mut params).unwrap_or_else(|_err| {
        eprintln!("Too few arguments specified. Usage: minigrep [query] [filename]");
        process::exit(1);
    }); 
    
    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
