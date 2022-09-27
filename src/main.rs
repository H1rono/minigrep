use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(&config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("The file contents is:\n{}", contents);
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("error occured while parsing arguments: {}", err);
        process::exit(1);
    });
    println!("required query: {}", &config.query);
    println!("reading file {}...", &config.filename);
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
