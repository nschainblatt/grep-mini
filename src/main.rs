use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);

    match config {
        Ok(val) => {
            println!("Searching for {} in file {}", val.query, val.file_path);
            let content =
                fs::read_to_string(val.file_path).expect("Should have been able to read from file");
            println!("{}", content);
        }
        Err(e) => println!("{}", e),
    };
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments were provided");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
