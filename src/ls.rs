use std::fs;

pub fn run() {
    let entries = fs::read_dir("./").unwrap();

    for entry in entries {
        let dir_entry = entry.unwrap();

        let is_dir = dir_entry.file_type().unwrap().is_dir();

        if is_dir { 
            if let Ok(file_name) = dir_entry.file_name().into_string() {
                print!("./{}   ", file_name);   
            }
        }
    };    
}
