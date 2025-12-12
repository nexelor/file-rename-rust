use std::{ffi::OsString, fs::FileType, path::PathBuf};

pub struct DirectoryFile {
    pub name: OsString,
    pub file_type: FileType,
    pub path: PathBuf
}