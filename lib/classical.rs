use std::char;

pub fn atbash_encrypt(msg: &str) -> String {
    return affine_encrypt(25, 25, msg);
}

// Affine cipher done here assumes a 26 letter alphabet
pub fn affine_encrypt(a: u32, b: u32, msg: &str) -> String {
    let mut e = String::new();
    for ch in msg.chars() {
        if !ch.is_alphabetic() { continue; }
        let p = ch.to_digit(36).unwrap() - 10;  // Need to shift down by 11 for range of 0-25
        let c = char::from_digit((a * p + b) % 26 + 10, 36).unwrap();
        e.push(c);
    }
    return e;
}

pub fn affine_decrypt(a: u32, b: u32, e: &str) -> String {
    let mut msg = String::new();
    for ch in e.chars() {
        if !ch.is_alphabetic() { continue; }
        let c = ch.to_digit(36).unwrap() - 10;  // Need to shift down by 11 for range of 0-25
        let p = char::from_digit(((c- b) / a) % 26 + 10, 36).unwrap();
        msg.push(p);
    }
    return msg;
}
