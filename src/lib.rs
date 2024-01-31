use ::std::error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_config() {
        let args = vec![
            "binary path".to_string(),
            "cat".to_string(),
            "test-file.log".to_string(),
        ];
        let _ = Config::build(&args).unwrap();
    }

    #[test]
    fn getting_content() {
        let args = vec![
            "binary path".to_string(),
            "cat".to_string(),
            "test-file.log".to_string(),
        ];
        let config = Config::build(&args).unwrap();
        let _ = run(config).unwrap();
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn not_enough_args() {
        let args = vec!["cat".to_string(), "../test-file.log".to_string()];
        let _ = Config::build(&args).unwrap();
    }

    #[test]
    #[should_panic(expected = "No such file")]
    fn file_not_found() {
        let args = vec![
            "...".to_string(),
            "cat".to_string(),
            "final-paper.txt".to_string(),
        ];
        let config = Config::build(&args).unwrap();
        let _ = run(config).unwrap();
    }
}

pub struct Config {
    query: String,
    file_path: String,
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
    pub fn get_query(&self) -> String {
        self.query.clone()
    }
    pub fn get_file_path(&self) -> String {
        self.file_path.clone()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(config.file_path)?;

    println!("{content}");
    Ok(())
}
