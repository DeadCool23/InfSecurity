use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn read_file(filename: &str) -> Result<Vec<u8>, String> {
    let mut file = File::open(filename)
        .map_err(|e| format!("Failed to open input file '{}': {}", filename, e))?;
    
    let mut content = Vec::new();
    file.read_to_end(&mut content)
        .map_err(|e| format!("Failed to read input file '{}': {}", filename, e))?;
    
    if content.is_empty() {
        return Err(format!("Input file '{}' is empty", filename));
    }
    
    Ok(content)
}

pub fn write_file(path: &Path, data: &[u8]) -> Result<(), String> {
    let mut file = File::create(path)
        .map_err(|e| format!("Failed to create output file '{}': {}", path.display(), e))?;
    
    file.write_all(data)
        .map_err(|e| format!("Failed to write output file '{}': {}", path.display(), e))?;
    
    Ok(())
}