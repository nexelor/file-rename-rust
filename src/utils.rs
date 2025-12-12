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

pub fn scan_folder(dir: &str, recursive: bool, nbr_sub_directories: u8, rename_directories: bool) -> Result<Vec<DirectoryFile>, String> {
    let mut files = Vec::<DirectoryFile>::new();

    scan_folder_inner(
        Path::new(dir),
        recursive,
        nbr_sub_directories,
        0,
        rename_directories,
        &mut files,
    )?;

    Ok(files)
}

fn scan_folder_inner(path: &Path, recursive: bool, max_depth: u8, current_depth: u8, include_dirs: bool, out: &mut Vec<DirectoryFile>) -> Result<(), String> {
    let entries = fs::read_dir(path).map_err(|e| {
        format!("while reading directory '{}': {}", path.display(), e)
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("reading entry: {}", e))?;
        let file_type = entry.file_type().map_err(|e| format!("file type: {}", e))?;

        // --- FILE ---
        if file_type.is_file() {
            out.push(DirectoryFile {
                name: entry.file_name(),
                file_type,
                path: entry.path(),
            });
        }

        // --- DIRECTORY ---
        if file_type.is_dir() {
            if include_dirs {
                out.push(DirectoryFile {
                    name: entry.file_name(),
                    file_type: file_type.clone(),
                    path: entry.path(),
                }); 
            }

            if recursive && current_depth < max_depth {
                scan_folder_inner(
                    &entry.path(),
                    recursive,
                    max_depth,
                    current_depth + 1,
                    include_dirs,
                    out,
                )?;
            }
        }
    }

    Ok(())
}