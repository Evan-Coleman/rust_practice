use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        match args.next() {
            Some(arg) => return Err("Too many args!"),
            None => (),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config_too_few_args() {
        let args = [String::from("Hello")];

        let result = Config::new(&args);

        assert!(result.is_err());

        let args = [];

        let result = Config::new(&args);

        assert!(result.is_err());
    }

    #[test]
    fn config_variations() {
        let args = [String::from("fakepath"), String::from("livelong"), String::from("fakefile.txt")];
        let config = Config::new(&args).unwrap();
        assert_eq!("livelong", config.query);

        let args = [String::from("fakepath"), String::from("livelong"), String::from("fakefile.txt")];
        let config = Config::new(&args).unwrap();
        assert!(run(config).is_err());

        let args = [String::from("fakepath"), String::from("livelong"), String::from("poem.txt")];
        let config = Config::new(&args).unwrap();
        assert!(!run(config).is_err());
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents));
    }
}