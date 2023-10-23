use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let paths: Vec<String> = args[1..].to_vec();
    let mut paths: VecDeque<String> = VecDeque::from(paths);
    list_dirs(&mut paths);
}

fn list_dirs(dirs: &mut VecDeque<String>) {
    let path = match dirs.pop_front() {
        Some(path) => path,
        None => return,
    };

    // TODO: Check if "path" is a directory
    // TODO: Trim leading slashes

    // TODO: Get rid of .clone()
    let dir = match std::fs::read_dir(path.clone()) {
        Ok(dir) => dir,
        Err(_) => return,
    };

    println!("D: {path}:");

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
            dirs.push_back(dirpath);
            continue;
        }

        match entry.file_name().into_string() {
            Ok(filename) => println!("F: {filename}"),
            Err(_) => continue,
        };
    }
    println!();

    return list_dirs(dirs);
}
