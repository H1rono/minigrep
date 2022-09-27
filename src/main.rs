use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Self {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("error occured while parsing arguments: {}", err);
        process::exit(1);
    });
    println!("reading file {}...", &config.filename);
    let mut f = File::open(&config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("error while reading file");
    println!("The file contents is:\n{}", contents);
}
