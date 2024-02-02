use ::std::error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_config() {
        let args = vec![
            "".to_string(),
            "cat".to_string(),
            "test-file.log".to_string(),
        ];
        let _ = Config::build(&args).unwrap();
    }

    #[test]
    fn getting_content() {
        let args = vec![
            "".to_string(),
            "cat".to_string(),
            "test-file.log".to_string(),
        ];
        let config = Config::build(&args).unwrap();
        let _ = run(&config).unwrap();
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn not_enough_args() {
        let args = vec!["cat".to_string(), "test-file.log".to_string()];
        let _ = Config::build(&args).unwrap();
    }

    #[test]
    #[should_panic(expected = "No such file")]
    fn file_not_found() {
        let args = vec![
            "".to_string(),
            "cat".to_string(),
            "final-paper.txt".to_string(),
        ];
        let config = Config::build(&args).unwrap();
        let _ = run(&config).unwrap();
    }

    #[test]
    fn search_contents() {
        let args = vec![
            "".to_string(),
            "john".to_string(),
            "test-file.log".to_string(),
        ];
        let config = Config::build(&args).unwrap();
        let contents = run(&config).unwrap();
        let results = search(&config.query, &contents);
        assert_eq!(
            results[0].contents,
            "2024-01-28 09:15:03 INFO User john logged in from IP 192.168.1.10".to_string()
        );
    }

    #[test]
    fn one_result() {
        let query = "duct".to_string();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.".to_string();

        assert_eq!(vec!["safe, fast, productive.".to_string()][0], search(&query, &contents)[0].contents);
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub struct FoundLine {
    pub contents: String,
    pub line_number: usize,
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

pub fn run(config: &Config) -> Result<String, Box<dyn error::Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    Ok(contents)
}

pub fn search(query: &String, contents: &String) -> Vec<FoundLine> {
    let lines = contents.split("\n");
    let mut found_lines: Vec<FoundLine> = Vec::new();
    for (idx, line) in lines.enumerate() {
        if line.contains(query) {
            found_lines.push(FoundLine {
                contents: line.to_string(),
                line_number: idx + 1,
            });
        }
    }
    found_lines
}
