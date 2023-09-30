use std::env;
use crate::version::VERSION;


pub struct BasicArguments {
    pub source_file:String,
    pub dest_file:String,
}


pub fn get_linker_targets() -> Result<BasicArguments, String> {
    let arg_list:Vec<String> = env::args().collect();
    let args:BasicArguments;

    match arg_list.len() == 3 {
        false => Err(format!("Lenker {}\n\nUsage:\n\t{} SOURCE_FILE DEST_FILE", VERSION, &arg_list[0])),

        true => {
            args = BasicArguments {
                source_file: String::from(&arg_list[1]),
                dest_file: String::from(&arg_list[2]),
            };
            Ok(args)
        }
    }
}
