pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case: bool,
}

impl Config {
    pub fn parse_config(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("please provide at least 2 arguments");
        }
        let query = args.get(1).expect("didn't get a query string").clone();
        let filename = args.get(2).expect("didn't get a filename string").clone();
        let is_case = std::env::var("CASE").is_err();

        Ok(Self {
            query,
            filename,
            is_case,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(&config.filename)?;
    let result = if config.is_case {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_incase_sensitive(&config.query, &contents)
    };

    for line in result.iter() {
        println!("{}", line);
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

fn search_incase_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_ascii_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
}
