use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        Self {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let config = Config::new(&args);
    println!("reading file {}...", &config.filename);
    let mut f = File::open(&config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("error while reading file");
    println!("The file contents is:\n{}", contents);
}
