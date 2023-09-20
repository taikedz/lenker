use std::path::Path;

#[derive(Debug)]
pub struct Registry {
    content: Vec<String>,
}

impl Registry {

    pub fn new() -> Registry {
        Registry { content: Vec::new() }
    }

    pub fn register(&mut self, relpath:&str) {
        let abspath = get_abspath(relpath);

        if self.contains_abs(&abspath) { return; }

        self.content.push(String::from(abspath));
    }

    pub fn contains(&self, relpath: &str) -> bool {
        let abspath = get_abspath(relpath);
        self.contains_abs(&abspath)
    }

    fn contains_abs(&self, abspath:&String) -> bool {
        let length = self.content.iter().filter(|S| S == &abspath).collect::<Vec<&String>>().len();
        length > 0
    }
}

fn get_abspath(relpath:&str) -> String {
    match std::fs::canonicalize(Path::new(relpath)) {
        Err(e) => {
            eprintln!("FATAL: cannot derive absolute path '{}' : {}", relpath, e);
            std::process::exit(100); // FIXME gather error codes
        },
        Ok(abspath) => match abspath.to_str() {
            None => {
                eprintln!("FATAL: invalid unicode in file declaration: {}", relpath);
                std::process::exit(101);
            },
            Some(s) => String::from(s)
        }
    }
}
