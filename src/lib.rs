use colored::Colorize;
use std::env;
use std::error::Error;
use std::fs;

/// Main function to run the program after collenting arguments
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(config.query, &contents)
    } else {
        search(config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Splits the differet 'queries' from the program arguments.
/// Returns an array with the queries
pub fn parse_query(query: String) -> Vec<String> {
    let mut queries = Vec::new();
    for word in query.split("|") {
        queries.push(String::from(word));
    }
    queries
}

/// Main Configuration struct to define program behaviour
pub struct Config {
    pub query: Vec<String>,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Builder Method to try and construct a valid Configuration
    /// Returns the configuration for search or an error
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let query = parse_query(query);
        let file_path = args[2].clone();
        let ignore_case = if args.len() < 4 {
            env::var("IGNORE_CASE").is_ok()
        } else {
            args[3].to_lowercase() == "ic"
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Function to add color to the queried sections of the output.
/// Returns the colorized String.
pub fn colorize(query: &str, line: &str) -> String {
    line.replace(query, &query.green().to_string())
}

/// Case Sensitive Search funtion to find all the lines that include a query.
/// Returns an array with all lines that include the query.
pub fn search<'a>(queries: Vec<String>, contents: &'a str) -> Vec<String> {
    let mut results = Vec::new();

    for query in queries {
        for (i, line) in contents.lines().enumerate() {
            if line.contains(&query) {
                let colored = colorize(&query, line);
                results.push(format!("{i} {colored}"));
            }
        }
    }

    results
}

/// Case Insensitive Search function to find all the lines that include a query, regardless of Lowercase or Uppercase.
/// Returns an array with all lines that include the query.
pub fn search_case_insensitive<'a>(queries: Vec<String>, contents: &'a str) -> Vec<String> {
    let mut results = Vec::new();

    for query in queries {
        let query = query.to_lowercase();
        for (i, line) in contents.lines().enumerate() {
            if line.to_lowercase().contains(&query) {
                let colored = colorize(&query, line);
                results.push(format!("{i} {colored}"));
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple() {
        let query = String::from("duct|rust");

        assert_eq!(vec!["duct", "rust"], parse_query(query));
    }

    #[test]
    fn case_sensitive() {
        let query = vec![String::from("duct")];
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = vec![String::from("rUsT")];
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

    #[test]
    fn one_result() {
        let query = vec![String::from("duct")];
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
