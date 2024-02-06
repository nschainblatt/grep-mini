use std::env;
use std::error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub check_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file path"),
        };

        let check_case: bool = if args.next() == Some("CHECK_CASE".to_string()) {
            true
        } else if args.next() == None && env::var("CHECK_CASE").is_ok() {
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
pub struct LineMatch<'a> {
    pub contents: &'a str,
    pub number: usize,
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

/// Searches a file for a specified string
///
/// # Example
///
/// ```
/// use grep_mini::{search_case_sensitive, LineMatch};
/// let lines_found = search_case_sensitive("Cat", "Cat in the hat");
/// assert_eq!(vec![LineMatch{ contents: "Cat in the hat", number: 1 }], lines_found );
/// ```
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<LineMatch<'a>> {
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

    #[test]
    fn example() {
        assert_eq!(
            vec![LineMatch {
                contents: "Cat in the hat",
                number: 1
            }],
            search_case_sensitive("Cat", "Cat in the hat"),
        );
    }
}
