use std::{fs, io::ErrorKind, path::PathBuf};

pub fn remove_files(inside: &str) -> std::io::Result<()> {
    let path = PathBuf::from(inside);

    match fs::remove_dir_all(&path) {
        Err(e) if e.kind() != ErrorKind::NotFound => Err(e),
        _ => Ok(()), // success, or folder was already gone
    }
}
