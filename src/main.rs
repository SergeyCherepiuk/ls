mod tree;

use tree::list_dir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let paths: Vec<String> = args[1..].to_vec();

    paths.iter().for_each(|path| {
        match list_dir(path.as_str(), 0) {
            Ok(_) => {}
            Err(_) => println!("Failed to list {path} dir"),
        };
    })
}
