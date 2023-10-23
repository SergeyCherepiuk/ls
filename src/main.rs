use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let paths: Vec<String> = args[1..].to_vec();
    let mut paths: VecDeque<String> = VecDeque::from(paths);

    while !paths.is_empty() {
        let path = match paths.pop_front() {
            Some(path) => path,
            None => break,
        };
        list_dir(path, "    ", 0);
    }
}

fn list_dir(path: String, indent_symbol: &str, indent_offset: usize) {
    // TODO: Check if "path" is a directory
    // TODO: Trim leading slashes

    // TODO: Get rid of .clone()
    let dir = match std::fs::read_dir(path.clone()) {
        Ok(dir) => dir,
        Err(_) => return,
    };

    let indent = indent_symbol.repeat(indent_offset);
    println!("{indent}D: {path}:");

    for entry in dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let filename = match entry.file_name().into_string() {
            Ok(filename) => filename,
            Err(_) => continue,
        };

        let filetype = match entry.file_type() {
            Ok(filetype) => filetype,
            Err(_) => continue,
        };

        if filetype.is_dir() {
            let dirpath = String::from(format!("{path}/{filename}"));
            list_dir(dirpath, indent_symbol, indent_offset + 1);
            continue;
        }

        match entry.file_name().into_string() {
            Ok(filename) => println!("{indent}{indent_symbol}F: {filename}"),
            Err(_) => continue,
        };
    }
}
