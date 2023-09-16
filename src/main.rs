use std::{env, process};

mod cat;
mod echo;
mod grep;
mod ls;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command: &str = args.get(1).unwrap_or_else(|| process::exit(1));

    match command {
        "echo" => {
            let input: &str = &args[2..].join(" ");

            echo::run(echo::Config::new(input.to_owned()));
        }
        "cat" => {
            let file_path = args.get(2).unwrap_or_else(|| process::exit(1));

            cat::run(cat::Config::new(file_path.to_owned()));
        }
        "ls" => {
            ls::run();
        }
        "grep" => {
            if args.len() == 4 {
                let (file_path, query) = (
                    args.get(2).unwrap_or_else(|| process::exit(1)),
                    args.get(3).unwrap_or_else(|| process::exit(1)),
                );

                grep::run(grep::Config::new(file_path.to_owned(), query.to_owned()));
            }
        }
        _ => panic!("No such command"),
    }
}
