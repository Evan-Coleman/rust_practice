use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
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
}