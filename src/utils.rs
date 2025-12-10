use std::path::Path;

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