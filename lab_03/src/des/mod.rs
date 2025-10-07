use rand::RngCore;
use rand::rngs::OsRng;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod bits;
mod hex;

mod tables;
use tables::*;  

fn generate_subkeys(key: &[u8; 8]) -> Vec<Vec<u8>> {
    let key_bits = bits::bytes_to_bitvec(key);
    let permuted = bits::permute(&key_bits, &PC1);

    let mut c = permuted[..28].to_vec();
    let mut d = permuted[28..].to_vec();

    let mut subkeys = Vec::with_capacity(16);

    for &shift in SHIFTS.iter() {
        c = bits::left_rotate_bits(&c, shift);
        d = bits::left_rotate_bits(&d, shift);

        let cd: Vec<u8> = c.iter().chain(d.iter()).cloned().collect();
        let k = bits::permute(&cd, &PC2);

        subkeys.push(k);
    }
    subkeys
}

fn sbox_substitute(bits48: &[u8]) -> Vec<u8> {
    assert!(bits48.len() == 48);
    let mut out = Vec::with_capacity(32);

    for i in 0..8 {
        let chunk = &bits48[i * 6..(i + 1) * 6];
        let row = ((chunk[0] << 1) | chunk[5]) as usize;
        let col = ((chunk[1] << 3) | (chunk[2] << 2) | (chunk[3] << 1) | chunk[4]) as usize;
        let val = S_BOX[i][row][col];

        out.push((val >> 3) & 1);
        out.push((val >> 2) & 1);
        out.push((val >> 1) & 1);
        out.push(val & 1);
    }
    out
}

fn f_func(r: &[u8], k: &[u8]) -> Vec<u8> {
    let r_exp = bits::permute(r, &E);
    let x = bits::xor_bits(&r_exp, k);
    let s_out = sbox_substitute(&x);
    bits::permute(&s_out, &P)
}

fn des_block(block: &[u8; 8], subkeys: &Vec<Vec<u8>>, encrypt: bool) -> [u8; 8] {
    let bits = bits::bytes_to_bitvec(block);
    let ip = bits::permute(&bits, &IP);

    let mut l = ip[..32].to_vec();
    let mut r = ip[32..].to_vec();

    let keys_iter: Box<dyn Iterator<Item = &Vec<u8>>> = if encrypt {
        Box::new(subkeys.iter())
    } else {
        Box::new(subkeys.iter().rev())
    };

    for k in keys_iter {
        let new_l = r.clone();
        let f_out = f_func(&r, k);
        let new_r = bits::xor_bits(&l, &f_out);
        l = new_l;
        r = new_r;
    }

    let preout: Vec<u8> = r.into_iter().chain(l.into_iter()).collect();
    let final_bits = bits::permute(&preout, &FP);
    let out_bytes = bits::bitvec_to_bytes(&final_bits);

    let mut arr = [0u8; 8];
    arr.copy_from_slice(&out_bytes[..8]);
    arr
}

fn pkcs5_pad(data: &[u8]) -> Vec<u8> {
    let pad_len = 8 - (data.len() % 8);
    let mut out = data.to_vec();

    out.extend(std::iter::repeat(pad_len as u8).take(pad_len));
    out
}

fn pkcs5_unpad(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() == 0 || data.len() % 8 != 0 {
        return Err("Invalid padded data length".to_string());
    }

    let pad_len = *data.last().unwrap() as usize;
    if pad_len < 1 || pad_len > 8 {
        return Err("Invalid padding byte".to_string());
    }

    let end = data.len();
    for &b in &data[end - pad_len..] {
        if b as usize != pad_len {
            return Err("Invalid padding contents".to_string());
        }
    }

    Ok(data[..end - pad_len].to_vec())
}

pub fn encrypt_file(keyfile: &Path, infile: &Path, outfile: &Path) -> Result<(), String> {
    let key = load_key(keyfile)?;
    let subkeys = generate_subkeys(&key);

    let plaintext = fs::read(infile).map_err(|e| format!("Failed read infile: {}", e))?;
    let padded = pkcs5_pad(&plaintext);

    let mut out = Vec::with_capacity(padded.len());
    for chunk in padded.chunks(8) {
        let mut arr = [0u8; 8];
        arr.copy_from_slice(chunk);
        
        let enc = des_block(&arr, &subkeys, true);
        out.extend_from_slice(&enc);
    }

    let mut f = File::create(outfile).map_err(|e| format!("Failed create outfile: {}", e))?;
    f.write_all(&out)
        .map_err(|e| format!("Failed write outfile: {}", e))?;

    Ok(())
}

pub fn decrypt_file(keyfile: &Path, infile: &Path, outfile: &Path) -> Result<(), String> {
    let key = load_key(keyfile)?;
    let subkeys = generate_subkeys(&key);

    let ciphertext = fs::read(infile).map_err(|e| format!("Failed read infile: {}", e))?;
    if ciphertext.len() % 8 != 0 {
        return Err("Ciphertext length not multiple of 8".to_string());
    }

    let mut out = Vec::with_capacity(ciphertext.len());
    for chunk in ciphertext.chunks(8) {
        let mut arr = [0u8; 8];
        arr.copy_from_slice(chunk);

        let dec = des_block(&arr, &subkeys, false);
        out.extend_from_slice(&dec);
    }

    let unp = pkcs5_unpad(&out).map_err(|e| format!("Unpad error: {}", e))?;

    let mut f = File::create(outfile).map_err(|e| format!("Failed create outfile: {}", e))?;
    f.write_all(&unp)
        .map_err(|e| format!("Failed write outfile: {}", e))?;

    Ok(())
}

pub fn genkey_file(keyfile: &Path) -> Result<(), String> {
    let mut key = [0u8; 8];
    OsRng.fill_bytes(&mut key);

    let hex = hex::encode(&key);

    let mut f = File::create(keyfile).map_err(|e| format!("Failed create keyfile: {}", e))?;
    f.write_all(hex.as_bytes())
        .map_err(|e| format!("Failed write keyfile: {}", e))?;

    Ok(())
}

fn load_key(keyfile: &Path) -> Result<[u8; 8], String> {
    let s = fs::read_to_string(keyfile).map_err(|e| format!("Failed read keyfile: {}", e))?;
    let s = s.trim();
    if s.len() != 16 {
        return Err("Keyfile must contain 16 hex chars (8 bytes)".to_string());
    }

    let bytes = hex::decode(s).map_err(|e| format!("Hex decode error: {}", e))?;

    let mut key = [0u8; 8];
    key.copy_from_slice(&bytes);
    Ok(key)
}
