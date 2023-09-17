mod arguments;

use arguments::parsing;

fn main() {
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
}
