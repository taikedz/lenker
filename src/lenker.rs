pub mod parsing;
mod registry;
mod fileresolve;
mod io;
mod directive;
mod loader;
mod writer;

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

    eprintln!("Linking '{}' into -> '{}'", args.source_file, args.dest_file);

    let mut registry = Registry::new();
    let resolved_data = loader::load(&args.source_file, &mut registry);

    writer::write_into(&args.dest_file, &resolved_data);
}

