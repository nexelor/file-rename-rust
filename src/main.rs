mod utils;

use anyhow::Ok;
use clap::Parser;
use colored::Colorize;

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

    Ok(())
}