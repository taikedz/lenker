use std::path::Path;

use super::io;

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
    if line.starts_with("#%insert") {
        let target:String = get_target(line, &vec!["."]).unwrap();
        load(&target)

    } else if line.starts_with("#%include") {
        let target:String = get_target(line, &vec!["."]).unwrap();
        // TODO - check this has not been included before
        load(&target)

    } else {
        String::from(line)
    }
}

fn get_target(line:&str, path_list:&Vec<&str>) -> Result<String,String> {
    let mut tokens = line.split(" ");
    tokens.next();

    let path:String = tokens.collect::<Vec<&str>>().join(" ");
    let path_str:&str = path.as_str();

    for base_path in path_list.iter() {
        let resolved_path = vec![base_path, path_str].join("/"); // FIXME use system path sep
        if Path::new(&resolved_path).exists() {
            // TODO - resolve against calling file
            // TODO - resolve against LENKER_PATH
            return Ok(String::from(path));
        }
    }
    Err(format!("Could not find {}", &path))
}

