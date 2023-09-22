pub mod parsing;
mod lenkerpath;
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
    let resolved_data = loader::load(&args.source_file, &mut registry);

    io::write_into(&args.dest_file, &resolved_data);

    println!("Wrote: {}", &args.dest_file);
}

