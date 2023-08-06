use std::fs;


pub struct Config {
    pub file_path: String
}

impl Config {
    pub fn new(file_path: String) -> Config {
        Config { 
            file_path
        }
    }
}

pub fn run(config: Config) {
    let output = fs::read_to_string(config.file_path);

    match output {
        Ok(output) => println!("{}", output),
        Err(_) => panic!("Error while reading the file"),
    }
}
