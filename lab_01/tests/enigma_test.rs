use enigma::Enigma;

#[test]
fn test_enigma_en() {
    let crypto_str = "HELLOWORLD";

    let mut e = Enigma::from_alphabet("ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes(), 3, true)
        .expect("Incorrect alphabet");

    let crypto = e
        .encrypt(crypto_str.as_bytes())
        .expect("Symbol in alphabet not founded");
    let crypto_string = String::from_utf8(crypto.clone()).expect("Invalid UTF-8 sequence");

    println!("Crypto msg: {}", crypto_string);

    e.reset();

    let decrypt = e.encrypt(&crypto).expect("Symbol in alphabet not founded");
    let decript_string = String::from_utf8(decrypt.clone()).expect("Invalid UTF-8 sequence");

    println!("Decrypto msg: {}", decript_string);

    assert_eq!(crypto_str, decript_string)
}

#[test]
fn test_enigma_ru() {
    let crypto_str = "ПРИВЕТМИР";
    
    let alphabet: Vec<char> = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ".chars().collect();
    
    let mut e = Enigma::from_alphabet(&alphabet, 3, true)
        .expect("Incorrect alphabet");

    let input_chars: Vec<char> = crypto_str.chars().collect();

    let crypto = e
        .encrypt(&input_chars)
        .expect("Symbol in alphabet not founded");
    
    let crypto_string: String = crypto.iter().collect();
    println!("Зашифрованное сообщение: {}", crypto_string);

    e.reset();

    let decrypt = e.encrypt(&crypto).expect("Symbol in alphabet not founded");
    let decript_string: String = decrypt.iter().collect();

    println!("Расшифрованное сообщение: {}", decript_string);

    assert_eq!(crypto_str, decript_string)
}