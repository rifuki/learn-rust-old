#![allow(unused)]
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn parse_config(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("please provide at least 2 arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Self { query, filename })
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::parse_config(&args).unwrap_or_else(| err | {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    println!("searching for: {}", config.query);
    println!("in file: {}", config.filename);

    if let Err(error) = run(&config) {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>>{
    let contents = std::fs::read_to_string(&config.filename)?;
    println!("\nwith text:\n{}", contents);
    Ok(())
}