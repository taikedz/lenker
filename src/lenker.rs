pub mod parsing;
mod io;


pub fn run() {
    // load arguments - main file, dest file
    let args:parsing::BasicArguments;

    match parsing::get_targets() {
        Ok(res) => args = res,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    println!("Linking '{}' into -> '{}'", args.source_file, args.dest_file);

    println!("{}", load(&args.source_file));
    //let data = lenker::load(&args.source_file); // TYPE
    //lenker::write(&data);
}

fn load(filename:&String) -> String {
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

    // Iterate and resolve ?
    // lines.iter().filter(|line| {if line.startswith("#%include") {load(name)} else {line}})
    return lines.join("\n")
}
