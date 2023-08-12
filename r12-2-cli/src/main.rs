use r12_2_cli::{Config, run};
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::parse_config(&args).unwrap_or_else(| err | {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    println!("searching for: \"{}\"", config.query);
    println!("in file: \"{}\"", config.filename);

    if let Err(error) = run(&config) {
        eprintln!("application error: {}", error);
        std::process::exit(1);
    }
}
