use std::{env, process};

mod cat;
mod echo;
mod ls;
mod grep;


fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args.get(1).unwrap();

    match &command[..] {
        "echo" => {
            let input: &str = &args[2..].join(" ");

            echo::run(echo::Config::new(input.to_owned()));
        },
        "cat" => {
            let file_path = args
                .get(2)
                .unwrap_or_else(|| {
                    process::exit(1)
                });
            
            cat::run(cat::Config::new(file_path.to_owned()));
        },
        "ls" => {
            ls::run();
        },
        "grep" => {
            if args.len() == 4 {
                let (file_path, query) 
                    = (args.get(2).unwrap(), args.get(3).unwrap());

                grep::run(
                    grep::Config::new(file_path.to_owned(), query.to_owned())
                );
            }
        },
        _ => panic!("No such command")
    }
}