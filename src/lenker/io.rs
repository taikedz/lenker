use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::path::Path;


fn path_exists(path_str: &str) -> bool {
    let path = Path::new(&path_str);
    path.exists()
}


pub fn read_lines(path_str: &str) -> Result<Vec<String>,String> {
    if ! path_exists(path_str) {
        // Simple check. This will eventually become a search along LENKER_PATH
        Err(format!("Could not find '{}'", path_str).to_string())
    } else {

        match read_to_string(path_str) {
            Err(e) => {
                Err(format!("Could not open '{}' : {}", path_str, e))
            }
            Ok(data) => {
                Ok(data.lines()         // split the string into an iterator of string slices
                    .map(String::from)  // short-notation, make each slice into an owned string
                    .collect()          // gather them together into a vector
                )
            }
        }
    }
}

pub fn write_into(filename:&str, data: &str) {
    let mut fh = open_file(filename);

    // Using the Write module, we can only operate on bytes...
    match fh.write_all(&data.as_bytes()) {
        Err(e) => {
            eprintln!("Failed write: {e}");
            std::process::exit(1);
        }
        Ok(_) => {
            // nothing
        }
    }
}


fn open_file(filename:&str) -> File {
    // OpenOptions::new() creates a reference. We must capture it in a variable to continue using it in the function
    // we can't use `opt = OpenOptions::new().write(true)` as the last doesn't return the temporary itself back out
    let mut opt = OpenOptions::new();
    opt .write(true)
        .create(true);

    match opt.open(filename) {
        Err(e) => {
            println!("Error opening '{}': {}", filename, e);
            std::process::exit(1);
        }
        Ok(fh) => { return fh; }
    }
}
