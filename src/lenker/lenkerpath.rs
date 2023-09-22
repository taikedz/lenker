use std::env;


pub fn get_paths_with<'lp>(local_paths:&'lp Vec<&'lp str>) -> Vec<String> {
    // We need to return owned-items for compatibility with the scenario where we add new paths
    // Note that map(|s| String::from(s)) complains about not being able to format `&&str`
    //   and the solution is to move one `&` into the argument signature
    let mut result_paths = local_paths
                            .iter()
                            .map(|&s| String::from(s))
                            .collect::<Vec<String>>();

    match env::var("LENKER_PATH") {
        Err(_e) => { return result_paths; },

        Ok(raw_paths) => {
            match raw_paths.as_str() {
                "" => result_paths,
                _ => {
                    let mut path_list = raw_paths
                                        .split(":")
                                        .map(|p| String::from(p))
                                        .collect::<Vec<String>>();
                    // append path_list to result_path
                    result_paths.append(&mut path_list);
                    result_paths

                    // Old code, retained for reference because I will probably need it again some time
                    // https://stackoverflow.com/a/47039490/2703818
                    //path_list.splice(0..0, local_paths.iter().map(|&p| String::from(p)).collect::<Vec<String>>());
                }
            }
        },
    }
}
