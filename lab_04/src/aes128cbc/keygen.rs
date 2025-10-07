use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;

use getrandom::getrandom;

fn gen_key_iv() -> ([u8; 16], [u8; 16]) {
    let mut key = [0u8; 16];
    let mut iv = [0u8; 16];
    
    getrandom(&mut key).expect("Failed to generate random key");
    getrandom(&mut iv).expect("Failed to generate random IV");
    
    (key, iv)
}

pub fn gen_keyfile(keyfile: &Path) -> Result<(), String> {
    let (key, iv) = gen_key_iv();

    let mut f = File::create(keyfile).map_err(|e| format!("Failed create keyfile: {}", e))?;
    f.write_all(&key)
        .map_err(|e| format!("Failed write keyfile: {}", e))?;
    f.write_all(&iv)
        .map_err(|e| format!("Failed write keyfile: {}", e))?;

    Ok(())
}

pub fn read_keyfile(keyfile: &Path) -> Result<([u8; 16], [u8; 16]), String> {
    let mut f = File::open(keyfile).map_err(|e| format!("Failed to open keyfile: {}", e))?;
    
    let mut contents = Vec::new();
    f.read_to_end(&mut contents)
        .map_err(|e| format!("Failed to read keyfile: {}", e))?;
    
    if contents.len() != 32 {
        return Err(format!(
            "Invalid keyfile size: expected 32 bytes, got {} bytes", 
            contents.len()
        ));
    }
    
    let mut key = [0u8; 16];
    key.copy_from_slice(&contents[0..16]);
    
    let mut iv = [0u8; 16];
    iv.copy_from_slice(&contents[16..32]);
    
    Ok((key, iv))
}