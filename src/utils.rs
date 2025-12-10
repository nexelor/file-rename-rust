use std::{fs, path::Path};

use crate::models::DirectoryFile;

pub fn validate_directory(dir: &str) -> Result<(), String> {
    let path = Path::new(dir);
    
    if !path.exists() {
        return Err(format!("'{}' does not exist", dir));
    }
    
    if !path.is_dir() {
        return Err(format!("'{}' is not a directory", dir));
    }

    Ok(())
}

pub fn scan_folder(dir: &str) -> Result<Vec<DirectoryFile>, String> {
    let mut files = Vec::<DirectoryFile>::new();

    let entries = match fs::read_dir(Path::new(dir)) {
        Ok(e) => e,
        Err(e) => {
            return Err(format!("while reading files descriptions from directory: {}, error: {}", dir, e));
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                return Err(format!("reading detail for file, error: {}", e));
            }
        };

        let file = DirectoryFile {
            name: entry.file_name(),
            path: entry.path()
        };

        files.push(file);
    }

    Ok(files)
}