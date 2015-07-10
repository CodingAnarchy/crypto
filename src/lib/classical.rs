use std::char;

pub fn atbash_encrypt(msg: &str) -> String {
    return affine_encrypt(25, 25, msg);
}

// Affine cipher done here assumes a 26 letter alphabet
pub fn affine_encrypt(a: u32, b: u32, msg: &str) -> String {
    let mut e = String::new();
    for ch in msg.chars() {
        if !ch.is_alphabetic() { continue; }
        let p = ch.to_digit(36).unwrap() - 10;  // Need to shift down by 10 for range of 0-25
        let c = char::from_digit((a * p + b) % 26 + 10, 36).unwrap();
        e.push(c);
    }
    return e;
}

pub fn atbash_decrypt(msg: &str) -> String {
    return affine_decrypt(25, 25, msg);
}

pub fn affine_decrypt(a: u32, b: u32, e: &str) -> String {
    let mut x = 0;
    for n in 1..26 {
        if (a * n) % 26 == 1 { x = n; break; }
    }
    let mut msg = String::new();
    for ch in e.chars() {
        if !ch.is_alphabetic() { continue; }
        let c = ch.to_digit(36).unwrap() - 10;  // Need to shift down by 10 for range of 0-25
        let sub = ((26 + (c as i32 - b as i32)) % 26) as u32;
        let p = char::from_digit((sub * x) % 26 + 10, 36).unwrap();
        msg.push(p);
    }
    return msg;
}

pub fn caesar_cipher(mut key: i32, msg: &str, encrypt: bool) -> String {
    let mut e = String::new();
    if !encrypt { key = -key; }
    for ch in msg.chars() {
        if !ch.is_alphabetic() { continue; }
        let c = (ch.to_digit(36).unwrap() - 10) as i32;  // Need to shift down by 10 for range of 0-25
        let s = char::from_digit((26i32 + c + key) as u32 % 26 + 10, 36).unwrap();
        e.push(s);
    }
    return e;
}

pub fn rot13_cipher(msg: &str, encrypt: bool) -> String {
    return caesar_cipher(13, msg, encrypt);
}

pub fn railfence_cipher(key: i32, msg: &str, encrypt: bool) -> String {
    let mut rails = vec![String::new(); key as usize];
    let mut e = String::new();
    if encrypt {
        let mut forward = true;
        let mut i = 0;
        for ch in msg.chars() {
            rails[i].push(ch);
            if forward {
                if i == rails.len() - 1 {
                    forward = false;
                    i -= 1;
                } else {
                    i += 1;
                }
            } else {
                if i == 0 {
                    forward = true;
                    i += 1;
                } else {
                    i -= 1;
                }
            }
        }
        for n in 0..rails.len() {
            println!("{}: {}", n, rails[n].as_str());
            e.push_str(rails[n].as_str());
        }
    }
    return e;
}

#[cfg(test)]
mod test {
    use super::*;
    use num::integer::gcd;
    use rand;
    use rand::distributions::{IndependentSample, Range};

    #[test]
    fn test_atbash_encrypt() {
        let s = "Defend the east wall of the castle.";
        assert_eq!("wvuvmwgsvvzhgdzoolugsvxzhgov", atbash_encrypt(s).as_str())
    }

    #[test]
    fn test_affine_encrypt() {
        let s = "Defend the east wall of the castle.";
        assert_eq!("kbsbykwabblfwvlqqpswabtlfwqb", affine_encrypt(17, 11, s).as_str())
    }

    #[test]
    fn test_atbash_decrypt() {
        let s = "wvuvmwgsvvzhgdzoolugsvxzhgov";
        assert_eq!("defendtheeastwallofthecastle", atbash_decrypt(s).as_str())
    }

    #[test]
    fn test_affine_decrypt() {
        let s = "ovcvgowqvvtpwrtssncwqvhtpwsv";
        assert_eq!("defendtheeastwallofthecastle", affine_decrypt(7, 19, s).as_str())
    }

    #[test]
    fn test_affine() {
        let mut rng = rand::thread_rng();
        let s = "defendtheeastwallofthecastle";
        let between = Range::new(1, 25);
        for _ in 1..50 {
            let mut a = 26;
            while gcd(a, 26) != 1 {
                a = between.ind_sample(&mut rng);
            }
            let b = between.ind_sample(&mut rng);
            let e = affine_encrypt(a, b, s);
            assert_eq!(s, affine_decrypt(a, b, e.as_str()).as_str())
        }
    }

    #[test]
    fn test_caesar() {
        let mut rng = rand::thread_rng();
        let s = "attackatdawn";
        let key_range = Range::new(1, 25);
        for _ in 1..50 {
            let k = key_range.ind_sample(&mut rng);
            let e = caesar_cipher(k, s, true);
            assert_eq!(s, caesar_cipher(k, e.as_str(), false));
        }
    }

    #[test]
    fn test_rot13() {
        let s = "attackatdawn";
        let e = "nggnpxngqnja";
        assert_eq!(e, rot13_cipher(s, true));
        assert_eq!(s, rot13_cipher(e, false));
    }

    #[test]
    fn test_railfence() {
        let s = "attackatdawn";
        assert_eq!("acdtaktantaw", railfence_cipher(3, s, true));
        assert_eq!("aatktntcdwaa", railfence_cipher(4, s, true));
        assert_eq!("adttatawaknc", railfence_cipher(5, s, true));
    }
}
