use std::env;


pub struct BasicArguments {
    pub source_file:String,
    pub dest_file:String,
}


pub fn get_targets() -> Result<BasicArguments, String> {
    let arg_list:Vec<String> = env::args().collect();
    let args:BasicArguments;

    if arg_list.len() < 3 {
        // We can omit ';', thus `Err()` instead of `return Err();`
        // but it doesn't sit well with me no not obviate the return...
        return Err(format!("Usage:\n\t{} SOURCE_FILE DEST_FILE", &arg_list[0]))

    } else {
        args = BasicArguments {
            source_file: String::from(&arg_list[1]),
            dest_file: String::from(&arg_list[2]),
        };
        return Ok(args)
    }
}
