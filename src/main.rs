use grep_mini::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1);
    });
    if let Err(error) = grep_mini::run(config) {
        eprintln!("{error}");
        process::exit(1);
    }
}
