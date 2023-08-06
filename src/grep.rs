use std::fs;

pub struct Config {
    file_path: String,
    query: String
}

impl Config {
    pub fn new(file_path: String, query: String) -> Config {
        Config { file_path, query }
    }
}

pub fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path).unwrap();

    for line in contents.lines() {
        let includes_query = find_query(line, &config.query);
        if includes_query {
            println!("{}", line);
        }
    }
}

fn find_query(text: &str, query: &str) -> bool {
    text.to_lowercase().contains(&query.to_lowercase())
}
