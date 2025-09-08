use std::{
    fs::{File, OpenOptions},
    io::{self, Error, Read, Write},
};

use enigma::{Enigma, cfg::{BinConfigSerializer, ConfigSerializer}};

use clap::{ArgAction, Parser};

/// Электронный аналог шифровальной машины "Энигма"
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Имя шифруемого файла (обязательный)
    filename: String,

    /// Имя конфигурационного файла рефлектора и роторов
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    /// Имя зашифрованного выходного файла [default: e<FILENAME>]
    #[arg(short, long, value_name = "FILE")]
    out: Option<String>,

    /// Количество роторов (0-255)
    #[arg(
        short = 'n',
        long = "rotors-num",
        value_name = "NUM",
        default_value_t = 3
    )]
    rotors_num: u8,

    /// Использовать коммутатор (панель подключений)
    /// 
    /// Коммутатор позволяет производить дополнительные замены символов
    /// перед подачей на роторы, повышая криптостойкость шифрования. 
    #[arg(
        short = 'm',
        long = "with-commutator", 
        default_value_t = false,
        action = ArgAction::SetTrue
    )]
    with_commutator: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut enigma = get_enigma(cli.config.clone(), cli.rotors_num, cli.with_commutator)
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });

    let file_data = read_file_data(&cli.filename).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let encrypted_data = enigma.encrypt(&file_data).unwrap_or_else(|e| {
        eprintln!("Pos: {}; Error: {}", e.0, e.1);
        std::process::exit(1);
    });

    write_encrypted_data(
        match cli.out {
            Some(filename) => filename,
            None => add_e_prefix(&cli.filename),
        },
        &encrypted_data,
    )
    .unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    if cli.config.is_none() {
        save_config(&enigma).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });
    }
}

fn get_enigma(
    cfg_filename: Option<String>,
    rotors_num: u8,
    wirh_commutator: bool,
) -> io::Result<Enigma<u8>> {
    let enigma: Enigma<u8>;
    if let Some(cfg_filename) = &cfg_filename {
        let mut cfg_file = File::open(cfg_filename)
            .map_err(|err| io::Error::new(err.kind(), format!("Ошибка открытия кофигурационного файла '{}': {}", cfg_filename, err)))?;

        let (commutator_cfg, reflector_cfg, rotors_cfgs) = BinConfigSerializer::get_configs(&mut cfg_file)?;

        enigma = Enigma::from_config(
            commutator_cfg.as_ref().map(|v| v.as_slice()),
            &reflector_cfg,
            &rotors_cfgs,
        )
        .map_err(|e| Error::new(io::ErrorKind::InvalidData, e))?;
    } else {
        enigma =
            Enigma::from_alphabet(&(0..=255).collect::<Vec<u8>>(), rotors_num, wirh_commutator)
                .map_err(|e| Error::new(io::ErrorKind::InvalidData, e))?;
    }

    Ok(enigma)
}

fn read_file_data(filename: &str) -> io::Result<Vec<u8>> {
    let mut file_data = Vec::new();
    let mut file = OpenOptions::new()
        .read(true)
        .open(filename)
        .map_err(|err| io::Error::new(err.kind(), format!("Ошибка открытия шифруемого файла '{}': {}", filename, err)))?;


    let _bytes = file.read_to_end(&mut file_data);
    Ok(file_data)
}

fn add_e_prefix(filename: &str) -> String {
    if let Some((dir, file)) = filename.rsplit_once('\\') {
        format!("{}\\e{}", dir, file)
    } else if let Some((dir, file)) = filename.rsplit_once('/') {
        format!("{}/e{}", dir, file)
    } else {
        format!("e{}", filename)
    }
}

fn write_encrypted_data(filename: String, encrypted_data: &[u8]) -> io::Result<()> {
    println!("Зашифрованные данные сохранены в файл {}", filename);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)?;

    file.write_all(&encrypted_data)?;
    Ok(())
}

fn save_config(enigma: &Enigma<u8>) -> io::Result<()> {
    print!("Введите файл в который сохранить конфигурацию Энигмы (./enigma.conf): ");
    io::stdout().flush()?;

    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;

    let filename = filename.trim();

    let filename = if filename.is_empty() {
        "./enigma.conf".to_string()
    } else {
        filename.to_string()
    };

    println!("Сохранение в файл: {}", filename);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)?;

    let (com_cfg, ref_cfg, rotors_cfgs) = enigma.get_config();
    BinConfigSerializer::save_configs(
        &mut file,
        com_cfg.as_ref().map(|v| v.as_slice()),
        &ref_cfg,
        &rotors_cfgs,
    )?;

    Ok(())
}
