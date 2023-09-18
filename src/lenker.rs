pub mod parsing;
mod fileresolve;
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

    println!("{}", fileresolve::load(&args.source_file));
    //let data = lenker::load(&args.source_file); // TYPE
    //lenker::write(&data);
}

