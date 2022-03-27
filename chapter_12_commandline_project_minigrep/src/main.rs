use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    println!("Hello, world of commandline programs!");
    // let arguments: Vec<String> = env::args().collect();
    // let config = Config::new(&arguments).unwrap_or_else(|error| {
    //     eprintln!("Problem parsing arguments: {}", error);
    //     process::exit(1);
    // });
    let arguments = env::args();
    let config = Config::new(arguments).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(error) = run(config) {
        eprintln!("Application error: {:?}", error);
        // process::exit(1);
    }
}