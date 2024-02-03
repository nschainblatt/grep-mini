use std::env;
use std::error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub check_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments provided");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let check_case: bool = if args.get(3).is_some() && &args[3] == "CHECK_CASE" {
            true
        } else if args.get(3).is_none() && env::var("CHECK_CASE").is_ok() {
            true
        } else {
            false
        };

        Ok(Config {
            query,
            file_path,
            check_case,
        })
    }
}

#[derive(Debug, PartialEq)]
struct LineMatch<'a> {
    contents: &'a str,
    number: usize,
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result: Vec<LineMatch> = if config.check_case {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    if result.len() > 0 {
        for line in result {
            println!("Line: {} -> {}", line.number, line.contents);
        }
    } else {
        println!("No matches found");
    }
    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<LineMatch<'a>> {
    let mut results: Vec<LineMatch<'a>> = Vec::new();
    for (number, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push(LineMatch {
                contents: line,
                number: number + 1,
            });
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<LineMatch<'a>> {
    let mut results: Vec<LineMatch<'a>> = Vec::new();
    let query = query.to_uppercase();
    for (number, line) in contents.lines().enumerate() {
        if line.to_uppercase().contains(&query) {
            results.push(LineMatch {
                contents: line,
                number: number + 1,
            });
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";

        assert_eq!(
            vec![LineMatch {
                contents: "safe, fast, productive.",
                number: 2
            }],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![
                LineMatch {
                    contents: "Rust:",
                    number: 1
                },
                LineMatch {
                    contents: "Trust me.",
                    number: 4
                }
            ],
            search_case_insensitive(query, contents)
        );
    }
}
