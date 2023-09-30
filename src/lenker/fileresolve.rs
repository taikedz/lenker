use std::path::Path;
use super::lenkerpath;

// TODO:unittest
pub fn parent_of(caller_file:&str) -> &str {
    /* This function seems unnecessarily complicated and nested
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


fn is_explicit_path(path_str:&str) -> bool {
    path_str.starts_with("/") || path_str.starts_with("./") || path_str.starts_with("../")
}

// TODO:unittest
fn generate_all_applicable_paths(path_str:&str, all_base_paths:&Vec<String>) -> Vec<String> {
    if is_explicit_path(path_str) {
        return vec![String::from(path_str)];
    }

    all_base_paths.iter()
        // FIXME use system path sep
        .map(|base_path| format!("{}/{}", base_path, path_str))
        .collect::<Vec<String>>()
}


pub fn get_target(path_str:&str, path_list:&Vec<&str>) -> Result<String,String> {
    let all_paths = lenkerpath::get_paths_with(&path_list);

    for resolved_path in generate_all_applicable_paths(path_str, &all_paths).iter() {
        if Path::new(&resolved_path).exists() {
            return Ok(resolved_path.to_string());
        }
    }
    Err(format!("Could not find '{}'", path_str))
}

