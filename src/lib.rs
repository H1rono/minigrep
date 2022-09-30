use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: env::Args) -> Result<Config, &'static str> {
        let mut args = args;
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Self {
            query: args.next().unwrap(),
            filename: args.next().unwrap(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err()
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            line.contains(query)
        })
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lowercase = query.to_lowercase();
    contents
        .lines()
        .filter(|line| {
            line.to_lowercase().contains(&query_lowercase)
        })
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(&config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let result = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        }
        .into_iter()
        .fold(String::new(), |acc, m| format!("{}{}\n", acc, m));
    print!("{}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
