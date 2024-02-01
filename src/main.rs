use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1);
    });
    if let Err(error) = minigrep::run(&config) {
        println!("{error}");
        process::exit(1);
    }
    match minigrep::run(&config) {
        Ok(contents) => {
            let found_lines = minigrep::search(&config.query, &contents);
            if found_lines.len() == 0 {
                println!("Search argument not found in {}", &config.file_path);
                process::exit(0);
            }
            println!("Found {} line/s", found_lines.len()); 
            for line in found_lines {
                println!("line: {} -- {} ", &line.line_number, &line.contents);
            }

        },
        Err(error) => {
            println!("{error}");
            process::exit(1);
        }
    };
}
