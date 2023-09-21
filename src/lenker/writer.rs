use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;

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

