use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1);
    });
    if let Err(error) = minigrep::run(config) {
        eprintln!("{error}");
        process::exit(1);
    }
}
