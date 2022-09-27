use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = &args[2];
    println!("reading file {}...", filename);
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("error while reading file");
    println!("The file contents is:\n{}", contents);
}
