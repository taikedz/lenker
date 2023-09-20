use std::path::Path;

use super::io;

struct Directive {
    command: String,
    file_path: String,
    options: Vec<String>,
}

pub fn load(filename:&String) -> String {
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
        .map(|line| resolve_line(line, filename))
        .collect::<Vec<String>>()
        .join("\n")
}

fn resolve_line(line:&str, caller_file:&str) -> String {
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
                "#%insert" => { do_directive_insert(&directive, caller_dir) }

                "#%include" => {
                    // TODO - check this has not been included before
                    // If not, perform insert
                    do_directive_insert(&directive, caller_dir)
                    // If so, do nothing
                }

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

fn do_directive_insert(directive:&Directive, caller_dir:&str) -> String {
    let target:String = get_target(directive.file_path.as_str(), &vec![caller_dir]).unwrap();
    // TODO - Register file's absolute path, load it
    load(&target)
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

