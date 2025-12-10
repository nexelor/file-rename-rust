use std::{ffi::OsString, path::PathBuf};

pub struct DirectoryFile {
    pub name: OsString,
    pub path: PathBuf
}