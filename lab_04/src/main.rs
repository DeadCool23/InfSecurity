mod aes128cbc;
mod files;

use aes128cbc::Aes128Cbc;
use aes128cbc::keygen;

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about = "DES ECB учебная реализация", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate AES key
    Genkey {
        /// Key filename
        #[arg(default_value = STD_KEYFILE)]
        keyfile: String,
    },
    /// Encrypt file
    Encrypt {
        /// Encrypting filename
        infile: String,
        /// Key filename
        #[arg(short, long = "key")]
        keyfile: Option<String>,
        /// Output filename
        #[arg(short, long = "out")]
        outfile: Option<String>,
    },
    /// Decrypt file
    Decrypt {
        /// Decrypting filename
        infile: String,
        /// Key filename
        #[arg(short, long = "key")]
        keyfile: Option<String>,
        /// Output filename
        #[arg(short, long = "out")]
        outfile: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let res = match cli.command {
        Commands::Genkey { keyfile } => {
            let res = keygen::gen_keyfile(Path::new(&keyfile));
            if res.is_ok() {
                println!("Key successfully saved in {}", keyfile);
            }
            res
        },
        Commands::Encrypt {
            keyfile,
            infile,
            outfile,
        } => {
            handle_encrypt(keyfile, &infile, outfile)
        }
        Commands::Decrypt {
            keyfile,
            infile,
            outfile,
        } => {
            handle_decrypt(keyfile, &infile, outfile)
        }
    };
    match res {
        Ok(()) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}

const STD_KEYFILE: &'static str = ".aes.key";
fn add_fileprefix(filename: &str, prefix: &str) -> String {
    if let Some((dir, file)) = filename.rsplit_once('\\') {
        format!("{}\\{}{}", dir, prefix, file)
    } else if let Some((dir, file)) = filename.rsplit_once('/') {
        format!("{}/{}{}", dir, prefix, file)
    } else {
        format!("{}{}", prefix, filename)
    }
}

fn resolve_paths(
    keyfile: Option<String>,
    infile: &str,
    outfile: Option<String>,
    prefix: &str,
) -> (PathBuf, PathBuf) {
    let keypath = if let Some(kfile) = keyfile {
        PathBuf::from(kfile)
    } else {
        let std_keyfile = PathBuf::from(STD_KEYFILE);
        let _ = keygen::gen_keyfile(&std_keyfile);
        println!("Key saved in {} file", STD_KEYFILE);
        std_keyfile
    };

    let outpath = if let Some(ofile) = outfile {
        PathBuf::from(ofile)
    } else {
        PathBuf::from(add_fileprefix(infile, prefix))
    };

    (keypath, outpath)
}

pub fn handle_encrypt(
    keyfile: Option<String>,
    infile: &str,
    outfile: Option<String>,
) -> Result<(), String> {
    let (keypath, outpath) = resolve_paths(keyfile, infile, outfile.clone(), "encrypted_");
    
    let (key, iv) = keygen::read_keyfile(&keypath)?;
    
    let aes = Aes128Cbc::new(&key, &iv);
    
    let plaintext = files::read_file(infile)?;
    
    let ciphertext = aes.encrypt(&plaintext);
    
    files::write_file(&outpath, &ciphertext)?;
    
    println!("File '{}' successfully encrypted to '{}'", infile, outpath.display());
    Ok(())
}

pub fn handle_decrypt(
    keyfile: Option<String>,
    infile: &str,
    outfile: Option<String>,
) -> Result<(), String> {
    let (keypath, outpath) = resolve_paths(keyfile, infile, outfile.clone(), "decrypted_");
    
    let (key, iv) = keygen::read_keyfile(&keypath)?;
    
    let aes = Aes128Cbc::new(&key, &iv);
    
    let ciphertext = files::read_file(infile)?;
    
    let plaintext = aes.decrypt(&ciphertext);
    
    files::write_file(&outpath, &plaintext)?;
    
    println!("File '{}' successfully encrypted to '{}'", infile, outpath.display());
    Ok(())
}