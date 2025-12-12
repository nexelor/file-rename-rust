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

    #[arg(short, long)]
    pub find: Vec<String>,

    /// Also rename the file present in the sub-directories
    #[arg(short = 'r')] 
    pub recursive: bool,

    /// The number of sub-directory to look into
    #[arg(short, value_parser = clap::value_parser!(u8).range(0..8), default_value = "1")]
    pub nbr_sub_directory: u8,

    /// Also rename the sub-directories 
    #[arg(short = 'd')]
    pub rename_directories: bool
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if let Err(e) = utils::validate_directory(&args.dir) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        return Ok(());
    }

    let files: Vec<DirectoryFile> = match scan_folder(&args.dir, args.recursive, args.nbr_sub_directory, args.rename_directories) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            return Ok(());
        }
    };

    for file in files {
        eprintln!("Found file - name: {:?}, type: {:?}, path: {:?}", file.name, file.file_type, file.path);
    }

    Ok(())
}