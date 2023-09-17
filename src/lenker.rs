pub mod parsing;


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

    //let data = lenker::load(&args.source_file); // TYPE
    //lenker::write(&data);
}
