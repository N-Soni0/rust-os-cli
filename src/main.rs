use args::{Cli, Commands};

use clap::Parser;
use colored::Colorize;
extern crate colored;

mod args;
mod cat;
mod echo;
mod grep;
mod ls;

fn main() {
    let args = Cli::parse();
    let command = args.command.expect("Command not valid");

    match command {
        Commands::Cat { path } => cat::run(cat::Config { file_path: path }),

        Commands::Echo { text } => echo::run(echo::Config { input: text }),

        Commands::Ls => {
            ls::run();
        }

        Commands::Grep { file_path, query } => grep::run(grep::Config::new(file_path, query)),
    }
}
