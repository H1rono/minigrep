extern crate minigrep;

use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("error occured while parsing arguments: {}", err);
        process::exit(1);
    });
    println!("required query: {}", &config.query);
    println!("reading file {}...", &config.filename);
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
