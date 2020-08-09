use std::env;
use std::fs;
use std::error::Error;

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

fn run2(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // Listing 13-27: Changing the body of Config::new to use iterator methods, pp. 322
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
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    pub fn new2(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // We needed clone here because we have a slice with String elements in the parameter args,
        // but the new function doesn’t own args.
        // To return ownership of a Config instance, we had to clone the values from the query
        // and fields of Config so the instance can own its values.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    fn new1(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        // The Trade-Offs of Using clone, pp. 269
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Config {
            query,
            filename,
            case_sensitive,
        }
    }
}

// not used anymore, replaced by Config::new
pub fn parse_config(args: &[String]) -> Config {
    // The Trade-Offs of Using clone, pp. 269
    // There’s a tendency among many Rustaceans to avoid using clone
    // to fix ownership problems because of its runtime cost.
    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Config {
        query,
        filename,
        case_sensitive,
    }
}

// Using iterator adaptor methods.
// Doing so also lets us avoid having a mutable intermediate results vector.
// The functional programming style prefers to minimize the amount of mutable state make code clearer.
// Removing the mutable state might enable a future enhancement to make searching happen in parallel,
// because we wouldn’t have to manage concurrent access to the results vector, pp. 323
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search1<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>( query: &str,
                                    contents: &'a str, ) -> Vec<&'a str> {
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
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
        assert_eq!(
            vec!["Rust:", "Trust me."], search_case_insensitive(query, contents)
        );
    }
}