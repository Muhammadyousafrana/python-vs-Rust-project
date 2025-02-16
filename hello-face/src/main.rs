// command line tool to play with the hello-face library

use hello_face::marco_polo;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let response = marco_polo(input);
    println!("{}", response);
}
