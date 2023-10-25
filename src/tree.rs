use std::{fs, io};

const INDENT_SYMBOL: &str = "\t";
const DIR_SIGN: &str = "[D]";
const FILE_SIGN: &str = "[F]";
const RO_SIGN: &str = "(ro)";
const RW_SIGN: &str = "(rw)";

pub fn list_dir(dir: &str, indent_depth: usize) -> io::Result<()> {
    let dir_indent = INDENT_SYMBOL.repeat(indent_depth);
    let file_indent = format!("{dir_indent}{INDENT_SYMBOL}");

    let dir = dir.trim_end_matches('/');
    if !fs::metadata(dir)?.is_dir() {
        return Err(io::Error::from(io::ErrorKind::InvalidData));
    }

    println!("{dir_indent}{DIR_SIGN} {dir}:");

    let read_dir = fs::read_dir(dir)?;
    for dir_entry in read_dir {
        let (file_name, metadata) = dir_entry.map(|dir_entry| {
            dir_entry
                .metadata()
                .map(|metadata| (dir_entry.file_name(), metadata))
        })??; // Masterpiece

        let file_name = file_name.to_str().unwrap_or_default();
        if metadata.is_dir() {
            let dir_path = format!("{dir}/{file_name}");
            list_dir(dir_path.as_str(), indent_depth + 1)?;
        } else {
            let permissions = if metadata.permissions().readonly() {
                RO_SIGN
            } else {
                RW_SIGN
            };
            println!("{file_indent}{FILE_SIGN} {permissions} {file_name}");
        }
    }

    Ok(())
}
