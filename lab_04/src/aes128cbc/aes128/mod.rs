mod consts;
use consts::*;

type State = [[u8; 4]; 4];

pub struct Aes128 {
    round_keys: Vec<[u32; 4]>, 
}

impl Aes128 {
    pub fn new(key: &[u8; 16]) -> Self {
        let round_keys = Self::key_expansion(key);
        Self { round_keys }
    }
    
    fn key_expansion(key: &[u8; 16]) -> Vec<[u32; 4]> {
        let mut round_keys = Vec::with_capacity(11);
        
        
        let mut w: Vec<u32> = key.chunks(4)
            .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        
        for i in 4..44 {
            let mut temp = w[i - 1];
            
            if i % 4 == 0 {
                temp = Self::sub_word(Self::rot_word(temp)) ^ RCON[i / 4 - 1];
            }
            
            w.push(w[i - 4] ^ temp);
        }

        
        for i in 0..11 {
            round_keys.push([
                w[4 * i],
                w[4 * i + 1],
                w[4 * i + 2],
                w[4 * i + 3],
            ]);
        }

        round_keys
    }

    fn sub_word(word: u32) -> u32 {
        let bytes = word.to_be_bytes();
        u32::from_be_bytes([
            S_BOX[bytes[0] as usize],
            S_BOX[bytes[1] as usize],
            S_BOX[bytes[2] as usize],
            S_BOX[bytes[3] as usize],
        ])
    }

    fn rot_word(word: u32) -> u32 {
        word.rotate_left(8)
    }

    pub fn encrypt(&self, input: &[u8; 16]) -> [u8; 16] {
        let mut state = Self::bytes_to_state(input);

        
        self.add_round_key(&mut state, 0);

        
        for round in 1..10 {
            self.sub_bytes(&mut state);
            self.shift_rows(&mut state);
            self.mix_columns(&mut state);
            self.add_round_key(&mut state, round);
        }

        self.sub_bytes(&mut state);
        self.shift_rows(&mut state);
        self.add_round_key(&mut state, 10);

        Self::state_to_bytes(&state)
    }

    pub fn decrypt(&self, input: &[u8; 16]) -> [u8; 16] {
        let mut state = Self::bytes_to_state(input);

        self.add_round_key(&mut state, 10);
        self.inv_shift_rows(&mut state);
        self.inv_sub_bytes(&mut state);

        for round in (1..10).rev() {
            self.add_round_key(&mut state, round);
            self.inv_mix_columns(&mut state);
            self.inv_shift_rows(&mut state);
            self.inv_sub_bytes(&mut state);
        }

        self.add_round_key(&mut state, 0);

        Self::state_to_bytes(&state)
    }

    fn bytes_to_state(bytes: &[u8; 16]) -> State {
        let mut state = [[0u8; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                state[j][i] = bytes[i * 4 + j];
            }
        }
        state
    }

    fn state_to_bytes(state: &State) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        for i in 0..4 {
            for j in 0..4 {
                bytes[i * 4 + j] = state[j][i];
            }
        }
        bytes
    }

    fn sub_bytes(&self, state: &mut State) {
        for row in state.iter_mut() {
            for byte in row.iter_mut() {
                *byte = S_BOX[*byte as usize];
            }
        }
    }

    fn inv_sub_bytes(&self, state: &mut State) {
        for row in state.iter_mut() {
            for byte in row.iter_mut() {
                *byte = INV_S_BOX[*byte as usize];
            }
        }
    }

    fn shift_rows(&self, state: &mut State) {
        for i in 1..4 {
            state[i].rotate_left(i);
        }
    }

    fn inv_shift_rows(&self, state: &mut State) {
        for i in 1..4 {
            state[i].rotate_right(i);
        }
    }

    fn mix_columns(&self, state: &mut State) {
        for i in 0..4 {
            let a = state[0][i];
            let b = state[1][i];
            let c = state[2][i];
            let d = state[3][i];

            state[0][i] = Self::gmul(0x02, a) ^ Self::gmul(0x03, b) ^ c ^ d;
            state[1][i] = a ^ Self::gmul(0x02, b) ^ Self::gmul(0x03, c) ^ d;
            state[2][i] = a ^ b ^ Self::gmul(0x02, c) ^ Self::gmul(0x03, d);
            state[3][i] = Self::gmul(0x03, a) ^ b ^ c ^ Self::gmul(0x02, d);
        }
    }

    fn inv_mix_columns(&self, state: &mut State) {
        for i in 0..4 {
            let a = state[0][i];
            let b = state[1][i];
            let c = state[2][i];
            let d = state[3][i];

            state[0][i] = Self::gmul(0x0e, a) ^ Self::gmul(0x0b, b) ^ Self::gmul(0x0d, c) ^ Self::gmul(0x09, d);
            state[1][i] = Self::gmul(0x09, a) ^ Self::gmul(0x0e, b) ^ Self::gmul(0x0b, c) ^ Self::gmul(0x0d, d);
            state[2][i] = Self::gmul(0x0d, a) ^ Self::gmul(0x09, b) ^ Self::gmul(0x0e, c) ^ Self::gmul(0x0b, d);
            state[3][i] = Self::gmul(0x0b, a) ^ Self::gmul(0x0d, b) ^ Self::gmul(0x09, c) ^ Self::gmul(0x0e, d);
        }
    }

    fn gmul(a: u8, b: u8) -> u8 {
        let mut p = 0;
        let mut a = a;
        let mut b = b;
        
        for _ in 0..8 {
            if b & 1 != 0 {
                p ^= a;
            }
            
            let hi_bit_set = a & 0x80 != 0;
            a <<= 1;
            if hi_bit_set {
                a ^= 0x1b; 
            }
            b >>= 1;
        }
        
        p
    }

    fn add_round_key(&self, state: &mut State, round: usize) {
        let round_key = &self.round_keys[round];
        
        for i in 0..4 {
            let key_bytes = round_key[i].to_be_bytes();
            for j in 0..4 {
                state[j][i] ^= key_bytes[j];
            }
        }
    }
}