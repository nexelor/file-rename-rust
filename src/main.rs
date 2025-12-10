mod utils;
mod models;

use clap::Parser;
use colored::Colorize;

use crate::{models::DirectoryFile, utils::scan_folder};

#[derive(Parser, Debug)]
#[command(name = "file-rename-rs", version)]
#[command(about = "Batch rename files using string find/replace or regex")]
pub struct Args {
    pub dir: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if let Err(e) = utils::validate_directory(&args.dir) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        return Ok(());
    }

    let files: Vec<DirectoryFile> = match scan_folder(&args.dir) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            return Ok(());
        }
    };

    for file in files {
        eprintln!("Found file - name: {:?}, path: {:?}", file.name, file.path);
    }

    Ok(())
}