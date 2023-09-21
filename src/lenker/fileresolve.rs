use std::path::Path;

pub fn parent_of(caller_file:&str) -> &str {
    /* This function seem unnecessarily complicated and nested
     * If we don't get parent from a Path, it could be current dir or root
     * Then check if we can extract the str. If it's invalid UTF-8, treat as an error
     * Finally if it still came out empty, it's certainly the current dir
     * Else return the resolved string ...
     */

    let caller_file_p:&Path = Path::new(caller_file);

    match caller_file_p.parent() {
        // This should be unnecessary... Path::new("/").parent() should return "/"
        None => if caller_file.starts_with("/") { "/" } else { "." },
        Some(p) => match p.to_str() {
            None => {
                eprintln!("Bad UTF-8 sequence specified : '{}'", caller_file);
                std::process::exit(100);
            },
            Some(s) => match s {
                "" => ".",
                // Interestingly, we can return &str here...
                //   because it is owned in this scope??
                _ => s,
            },
        }
    }
}

pub fn get_target(path_str:&str, path_list:&Vec<&str>) -> Result<String,String> {
    for base_path in path_list.iter() {
        let resolved_path = vec![base_path, path_str].join("/"); // FIXME use system path sep
        if Path::new(&resolved_path).exists() {
            return Ok(String::from(&resolved_path));
        }
    }
    Err(format!("Could not find '{}'", path_str))
}

