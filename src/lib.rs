use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();

        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case }) 
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    

    let results = if config.ignore_case {
        search_case_sensetive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search_case_sensetive<'a>(query: & str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

pub fn search<'a>(query: & str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(vec!["safe, fast, productive.", "Duct tape."], search_case_sensetive(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_sensetive(query, contents));
    }
}
