use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    println!(
        "Searching for {} in file {}",
        config.get_query(),
        config.get_file_path()
    );

    if let Err(error) = minigrep::run(config) {
        println!("{error}");
        process::exit(1);
    }
}
