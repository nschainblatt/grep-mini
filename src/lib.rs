use ::std::error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments provided");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

struct LineMatch<'a> {
    contents: &'a str,
    number: usize,
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result: Vec<LineMatch> = search(&config.query, &contents);
    if result.len() > 0 {
        for line in result {
            println!("Line: {} -> {}", line.number, line.contents);
        }
    } else {
        println!("No matches found");
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<LineMatch<'a>> {
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

        assert_eq!(vec!["safe, fast, productive."][0], search(query, contents)[0].contents);
    }
}
