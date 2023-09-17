use std::fs::read_to_string;
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
                Ok(data.lines()            // split the string into an iterator of string slices
                    .map(String::from)  // make each slice into an owned string
                    .collect()          // gather them together into a vector
                )
            }
        }
    }
}

