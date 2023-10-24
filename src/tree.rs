use std::fs;

const INDENT_SYMBOL: &str = "\t";
const DIR_SIGN: &str = "[D]";
const FILE_SIGN: &str = "[F]";
const RO_SIGN: &str = "(ro)";
const RW_SIGN: &str = "(rw)";
const UNKNOWN_SIGN: &str = "(unknown)";

pub fn list_dir(dir: &str, indent_depth: usize) {
    let dir = dir.trim_end_matches('/');

    if !fs::metadata(dir).is_ok_and(|metadata| metadata.is_dir()) {
        return;
    }

    let dir_indent = INDENT_SYMBOL.repeat(indent_depth);
    let file_indent = format!("{dir_indent}{INDENT_SYMBOL}");

    println!("{dir_indent}{DIR_SIGN} {dir}:");

    let read_dir = match fs::read_dir(dir) {
        Ok(read_dir) => read_dir,
        Err(_) => return,
    };

    for dir_entry_or_err in read_dir {
        let dir_entry = match dir_entry_or_err {
            Ok(dir_entry) => dir_entry,
            Err(_) => continue,
        };

        let file_name = dir_entry.file_name();
        let file_name = match file_name.to_str() {
            Some(file_name) => file_name,
            None => continue,
        };

        let metadata = dir_entry.metadata();
        if (&metadata.as_ref()).is_ok_and(|metadata| metadata.is_dir()) {
            let dir_path = format!("{dir}/{file_name}");
            list_dir(dir_path.as_str(), indent_depth + 1);
        } else {
            let permissions = match metadata {
                Ok(metadata) => {
                    if metadata.permissions().readonly() {
                        RO_SIGN
                    } else {
                        RW_SIGN
                    }
                }
                Err(_) => UNKNOWN_SIGN,
            };
            println!("{file_indent}{FILE_SIGN} {permissions} {file_name}");
        }
    }
}
