pub mod parsing;
mod registry;
mod fileresolve;
mod io;
mod directive;
mod loader;

use registry::Registry;

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

    let mut registry = Registry::new();

    eprintln!("Linking '{}' into -> '{}'", args.source_file, args.dest_file);

    println!("{}", loader::load(&args.source_file, &mut registry));
    //let data = lenker::load(&args.source_file); // TYPE
    //lenker::write(&data);
}

