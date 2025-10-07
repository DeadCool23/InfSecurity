mod aes128;
pub mod keygen;

use aes128::Aes128;

pub struct Aes128Cbc {
    aes: Aes128,
    iv: [u8; 16],
}

impl Aes128Cbc {
    pub fn new(key: &[u8; 16], iv: &[u8; 16]) -> Self {
        Self {
            aes: Aes128::new(key),
            iv: *iv,
        }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::new();
        let mut prev_block = self.iv;

        for chunk in plaintext.chunks(16) {
            let mut block = [0u8; 16];
            let chunk_len = chunk.len();
            block[..chunk_len].copy_from_slice(chunk);
            
            if chunk_len < 16 {
                let pad_byte = (16 - chunk_len) as u8;
                for i in chunk_len..16 {
                    block[i] = pad_byte;
                }
            }

            for i in 0..16 {
                block[i] ^= prev_block[i];
            }

            let encrypted_block = self.aes.encrypt(&block);
            ciphertext.extend_from_slice(&encrypted_block);
            prev_block = encrypted_block;
        }

        ciphertext
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        let mut plaintext = Vec::new();
        let mut prev_block = self.iv;

        for chunk in ciphertext.chunks(16) {
            let block: [u8; 16] = chunk.try_into().unwrap();
            let decrypted_block = self.aes.decrypt(&block);
            
            let mut plain_block = [0u8; 16];
            for i in 0..16 {
                plain_block[i] = decrypted_block[i] ^ prev_block[i];
            }
            
            plaintext.extend_from_slice(&plain_block);
            prev_block = block;
        }

        if let Some(&last_byte) = plaintext.last() {
            let pad_len = last_byte as usize;
            if pad_len <= 16 {
                plaintext.truncate(plaintext.len() - pad_len);
            }
        }

        plaintext
    }
}