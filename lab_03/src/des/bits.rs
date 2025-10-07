pub fn bytes_to_bitvec(b: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(b.len() * 8);
    for &byte in b {
        for i in (0..8).rev() {
            v.push((byte >> i) & 1);
        }
    }
    v
}

pub fn bitvec_to_bytes(bits: &[u8]) -> Vec<u8> {
    assert!(bits.len() % 8 == 0);
    let mut out = Vec::with_capacity(bits.len() / 8);
    for chunk in bits.chunks(8) {
        let mut val = 0u8;
        for &b in chunk {
            val = (val << 1) | b;
        }
        out.push(val);
    }
    out
}

pub fn permute(bits: &[u8], table: &[usize]) -> Vec<u8> {
    table.iter().map(|&i| bits[i - 1]).collect()
}

pub fn left_rotate_bits(bits: &[u8], n: usize) -> Vec<u8> {
    let len = bits.len();
    let n = n % len;
    bits[n..].iter().chain(bits[..n].iter()).cloned().collect()
}

pub fn xor_bits(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
}
