mod tree;

use tree::list_dir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let paths: Vec<String> = args[1..].to_vec();

    for path in paths {
        list_dir(path.as_str(), 0);
    }
}
