pub fn encode(b: &[u8]) -> String {
    let mut s = String::with_capacity(b.len() * 2);
    for &byte in b {
        s.push(hex_char(byte >> 4));
        s.push(hex_char(byte & 0xF));
    }
    s
}

fn hex_char(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + (n - 10)) as char,
        _ => '?',
    }
}

pub fn decode(s: &str) -> Result<Vec<u8>, String> {
    let s = s.trim();
    if s.len() % 2 != 0 {
        return Err("Odd hex length".to_string());
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let chars: Vec<char> = s.chars().collect();
    for i in (0..s.len()).step_by(2) {
        let hi = hex_val(chars[i])?;
        let lo = hex_val(chars[i + 1])?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

fn hex_val(c: char) -> Result<u8, String> {
    match c {
        '0'..='9' => Ok((c as u8) - b'0'),
        'a'..='f' => Ok(10 + (c as u8) - b'a'),
        'A'..='F' => Ok(10 + (c as u8) - b'A'),
        _ => Err(format!("Invalid hex char: {}", c)),
    }
}
