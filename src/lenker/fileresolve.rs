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
        },
        Ok(res) => lines = res,
    };

    lines.iter()
        .map(|line| resolve_line(line))
        .collect::<Vec<String>>()
        .join("\n")
}

fn resolve_line(line:&str) -> String {
    match parse_directive(line) {
        None => String::from(line),
        Some(directive) => {
            match directive.command.as_str() {
                // FIXME don't use unwrap - if line not found, print linenum and file
                "#%insert" => {
                    let target:String = get_target(directive.file_path.as_str(), &vec!["."]).unwrap();
                    load(&target)
                },

                "#%include" => {
                    let target:String = get_target(directive.file_path.as_str(), &vec!["."]).unwrap();
                    // TODO - check this has not been included before
                    load(&target)
                }

                _ => {
                    // Found an invalid directive - return the line as-is
                    //   this may still have been deliberate
                    eprintln!("WARN: unresolvable directive '{}'", directive.command);
                    String::from(line)
                }
            }
        }
    }
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
            return Ok(String::from(path_str));
        }
    }
    Err(format!("Could not find '{}'", path_str))
}

