
pub struct Config {
    pub input: String,
}

impl Config {
    pub fn new(input: String) -> Config {
        Config { input }
    }
}

pub fn run(config: Config) {
    println!("{}", config.input);
}
