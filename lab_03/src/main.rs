mod des;

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
    /// Generate DES key
    Genkey {
        /// Key filename
        #[arg(default_value = "des.key")]
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
            let res = des::genkey_file(Path::new(&keyfile));
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
            let (keypath, outpath) = resolve_paths(keyfile, &infile, outfile, "encrypted_");
            let res = des::encrypt_file(&keypath, Path::new(&infile), &outpath);
            if res.is_ok() {
                println!("{} successfully encrypted in file {}", infile, outpath.display())
            }
            res
        }
        Commands::Decrypt {
            keyfile,
            infile,
            outfile,
        } => {
            let (keypath, outpath) = resolve_paths(keyfile, &infile, outfile, "decrypted_");
            let res = des::decrypt_file(&keypath, Path::new(&infile), &outpath);
            if res.is_ok() {
                println!("{} successfully decrypted in file {}", infile, outpath.display())
            }
            res
        }
    };
    match res {
        Ok(()) => {},
        Err(e) => eprintln!("Error: {}", e),
    }
}

const STD_KEYFILE: &'static str = ".des.key";
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
        let _ = des::genkey_file(&std_keyfile);
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