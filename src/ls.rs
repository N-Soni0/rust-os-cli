use std::{fs, io};

use colored::Colorize;

pub fn run() -> io::Result<()> {
    let entries = fs::read_dir("./")?;

    for entry in entries {
        let dir_entry = entry?;

        let file_type = dir_entry.file_type()?;
        let file_name = dir_entry.file_name();

        let colored_name = match file_type.is_dir() {
            true => file_name.to_string_lossy().blue(),
            false => file_name.to_string_lossy().white(),
        };

        println!("{}", colored_name);
    }

    Ok(())
}
