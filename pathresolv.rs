use std::path::Path;

fn main() {
    get_target("#include file1.txt", &vec!["."]);
}


fn get_target(line:&str, path_list:&Vec<&str>) -> Result<String,String> { // FIXME - receive the resolution locations as argument
    let mut tokens = line.split(" ");
    tokens.next();

    let path = tokens.collect::<Vec<&str>>().join(" ");

    for base_path in path_list.iter() {
        let resolved_path = vec![base_path, &path].join("/"); // FIXME use system path sep
        if Path::new(&resolved_path).exists() {
            // TODO - resolve against calling file
            // TODO - resolve against LENKER_PATH
            return Ok(path);
        }
    }
    Err(format!("Could not find {}", &path))
}
