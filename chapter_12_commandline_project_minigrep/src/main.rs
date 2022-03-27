use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    println!("Hello, world of commandline programs!");
    let arguments: Vec<String> = env::args().collect();
    let config = Config::new(&arguments).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });
    
    if let Err(error) = run(config) {
        println!("Application error: {:?}", error);
    }
}