use std::char;
use num::integer::gcd;
use num::abs;

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

pub fn atbash_decrypt(msg: &str) -> String {
    return affine_decrypt(25, 25, msg);
}

pub fn affine_decrypt(a: u32, b: u32, e: &str) -> String {
    let x = gcd(a, 26) % 26;
    let mut msg = String::new();
    for ch in e.chars() {
        if !ch.is_alphabetic() { continue; }
        let c = ch.to_digit(36).unwrap() - 10;  // Need to shift down by 11 for range of 0-25
        let sub = c as i32 - b as i32;
        let p = char::from_digit((abs(sub) as u32 * x) % 26 + 10, 36).unwrap();
        msg.push(p);
    }
    return msg;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_atbash_encrypt() {
        let s = "Defend the east wall of the castle.";
        assert_eq!("wvuvmwgsvvzhgdzoolugsvxzhgov", atbash_encrypt(s).as_str())
    }

    #[test]
    fn test_atbash_decrypt() {
        let s = "wvuvmwgsvvzhgdzoolugsvxzhgov";
        assert_eq!("defendtheeastwallofthecastle", atbash_decrypt(s).as_str())
    }
}
