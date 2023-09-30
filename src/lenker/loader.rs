use super::io;
use super::registry::Registry;
use super::directive;
use super::fileresolve;

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
    match directive::parse_directive(line) {
        None => String::from(line),
        Some(direc) => {
            match direc.command.as_str() {
                "#%insert" => { do_directive_insert(&direc.file_path.as_str(), caller_file, registry) }

                "#%include" => { do_directive_include(&direc.file_path.as_str(), caller_file, registry) }

                _ => {
                    // Found an invalid directive - return the line as-is
                    //   this may still have been deliberate
                    eprintln!("WARN: {}: unresolvable directive '{}'", caller_file, direc.command);
                    String::from(line)
                }
            }
        }
    }
}


fn do_directive_include(file_path:&str, caller_file:&str, registry:&mut Registry) -> String {
    let caller_dir:&str = fileresolve::parent_of(caller_file);

    match fileresolve::get_target(file_path, &vec![caller_dir]) {
        Ok(target) => {
            if ! registry.contains(&target) {
                registry.register(&target);
                return load(&target, registry);
            } else {
                // Not great - inserts a blank line that wasn't there...
                return String::from("");
            }
        },
        Err(e) => {
            eprintln!("ERROR: #%include in {} : {}", caller_file, e);
            std::process::exit(100); // FIXME:errcode
        }
    }
}

fn do_directive_insert(file_path:&str, caller_file:&str, registry:&mut Registry) -> String {
    let caller_dir:&str = fileresolve::parent_of(caller_file);

    // FIXME - need to print calling _file_ and line number, ideally
    match fileresolve::get_target(file_path, &vec![caller_dir]) {
        Ok(target) => load(&target, registry),
        Err(e) => {
            eprintln!("ERROR: #%insert in {} : {}", caller_file, e);
            std::process::exit(100); // FIXME:errcode
        }
    }
}

