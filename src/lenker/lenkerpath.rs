use std::env;


fn main() {
    let res = get_paths_with(&vec!["a","b"]);
    println!("{:?}", res);
}


pub fn get_paths_with<'lp>(local_paths:&'lp Vec<&'lp str>) -> &'lp Vec<String> {
    match env::var("LENKER_PATH") {
        Err(_e) => { return &local_paths.iter().map(|s| s.to_string()).collect::<Vec<String>>(); },
        Ok(raw_paths) => {
            let mut path_list = raw_paths.split(":").map(|p| String::from(p)).collect::<Vec<String>>();

            // https://stackoverflow.com/a/47039490/2703818
            path_list.splice(0..0, local_paths.iter().map(|p| p.to_string()).collect::<Vec<String>>());

            &path_list
        },
    }
}
