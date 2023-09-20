use std::path::Path;

use super::io;
use super::registry::Registry;

struct Directive {
    command: String,
    file_path: String,
    options: Vec<String>,
}

pub fn load(filename:&String, registry:&mut Registry) -> String {
    let lines:Vec<String>;

    match io::read_lines(filename) {
        Err(e) => {
            // We can't "just" catch and re-throw - `Err(e) => Err(e)` - and let the Ok case do an assignment
            // we must exit the match always with the same type
            eprintln!("{}", e);
            std::process::exit(1);
        }

        Ok(res) => lines = res,
    };

    lines.iter()
        // using `map` assumes we always get a valid line resolution
        // but if we want to handle (e.g.) non-found files properly:
        // TODO use for loop, track line numbers, and print helpful error
        .map(|line| resolve_line(line, filename, registry))
        .collect::<Vec<String>>()
        .join("\n")
}

fn resolve_line(line:&str, caller_file:&str, registry:&mut Registry) -> String {
    let caller_file_p:&Path = Path::new(caller_file);
    let caller_dir:&str;

    match caller_file_p.parent() {
        None => caller_dir = ".",
        Some(p) => {
            match p.to_str() {
                None => caller_dir = ".",
                Some(s) => caller_dir = s,
            }
        }
    }

    match parse_directive(line) {
        None => String::from(line),
        Some(directive) => {
            match directive.command.as_str() {
                // FIXME don't use unwrap - if line not found, print linenum and file
                "#%insert" => { do_directive_insert(&directive.file_path.as_str(), caller_dir, registry) }

                "#%include" => { do_directive_include(&directive.file_path.as_str(), caller_dir, registry) }

                _ => {
                    // Found an invalid directive - return the line as-is
                    //   this may still have been deliberate
                    eprintln!("WARN: {}: unresolvable directive '{}'", caller_file, directive.command);
                    String::from(line)
                }
            }
        }
    }
}


fn do_directive_include(file_path:&str, caller_dir:&str, registry:&mut Registry) -> String {
    let target:String = get_target(file_path, &vec![caller_dir]).unwrap();

    if ! registry.contains(&target) {
        registry.register(&target);
        return load(&target, registry);
    } else {
        // Not great - inserts a blank line that wasn't there...
        return String::from("");
    }
}

fn do_directive_insert(file_path:&str, caller_dir:&str, registry:&mut Registry) -> String {
    let target:String = get_target(file_path, &vec![caller_dir]).unwrap();
    load(&target, registry)
}

fn parse_directive(line:&str) -> Option<Directive> {
    if ! line.starts_with("#%") {
        return None;
    }

    let mut tokens = line.split(" ");
    match tokens.next() {
        None => None,

        Some(command) => {
            let path:String = tokens.collect::<Vec<&str>>().join(" ");
            let options:Vec<String> = vec![];

            Some(Directive { command: String::from(command), file_path: path, options: options })
        }
    }
}

fn get_target(path_str:&str, path_list:&Vec<&str>) -> Result<String,String> {
    for base_path in path_list.iter() {
        let resolved_path = vec![base_path, path_str].join("/"); // FIXME use system path sep
        if Path::new(&resolved_path).exists() {
            return Ok(String::from(&resolved_path));
        }
    }
    Err(format!("Could not find '{}'", path_str))
}

