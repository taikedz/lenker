pub struct Directive {
    pub command: String,
    pub file_path: String,
    pub options: Vec<String>,
}

pub fn parse_directive(line:&str) -> Option<Directive> {
    if ! line.starts_with("#%") {
        return None;
    }

    let mut tokens = line.split(" ");
    match tokens.next() {
        None => None,

        Some(command) => {
            let path:String = tokens
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
                .join(" ");
            let options:Vec<String> = vec![];

            Some(Directive { command: String::from(command), file_path: path, options: options })
        }
    }
}

