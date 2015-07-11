use std::char;

pub fn atbash_cipher(msg: &str, encrypt: bool) -> String {
    return affine_cipher(25, 25, msg, encrypt);
}

// Affine cipher done here assumes a 26 letter alphabet
pub fn affine_cipher(a: u32, b: u32, msg: &str, encrypt: bool) -> String {
    let mut e = String::new();
    if encrypt {
        for ch in msg.chars() {
            if !ch.is_alphabetic() { continue; }
            let p = ch.to_digit(36).unwrap() - 10;  // Need to shift down by 10 for range of 0-25
            let c = char::from_digit((a * p + b) % 26 + 10, 36).unwrap();
            e.push(c);
        }
    } else {
        let mut x = 0;
        for n in 1..26 {
            if (a * n) % 26 == 1 { x = n; break; }
        }
        for ch in msg.chars() {
            if !ch.is_alphabetic() { continue; }
            let c = ch.to_digit(36).unwrap() - 10;  // Need to shift down by 10 for range of 0-25
            let sub = ((26 + (c as i32 - b as i32)) % 26) as u32;
            let p = char::from_digit((sub * x) % 26 + 10, 36).unwrap();
            e.push(p);
        }
    }
    return e;
}

fn caesar_shift(ch: char, k: i32) -> char {
    let c = (ch.to_digit(36).unwrap() - 10) as i32;  // Need to shift down by 10 for range of 0-25
    let s = char::from_digit((26i32 + c + k) as u32 % 26 + 10, 36).unwrap();
    return s;
}

pub fn caesar_cipher(mut key: i32, msg: &str, encrypt: bool) -> String {
    let mut e = String::new();
    if !encrypt { key = -key; }
    for ch in msg.chars() {
        if !ch.is_alphabetic() { continue; }
        let s = caesar_shift(ch, key);
        e.push(s);
    }
    return e;
}

pub fn rot13_cipher(msg: &str, encrypt: bool) -> String {
    return caesar_cipher(13, msg, encrypt);
}

pub fn vigenere_cipher(keyword: &str, msg: &str, encrypt: bool) -> String {
    let mut e = String::new();
    let mut keys = keyword.chars();
    let mut kch;
    let mut k: i32;
    for ch in msg.chars() {
        kch = keys.next();
        if kch == None {
            keys = keyword.chars();
            kch = keys.next();
        }
        k = (kch.unwrap().to_digit(36).unwrap() - 10) as i32;
        if !encrypt { k = -k; }
        let s = caesar_shift(ch, k);
        e.push(s);
    }
    return e;
}

pub fn railfence_cipher(key: i32, msg: &str, encrypt: bool) -> String {
    let mut rails = vec![String::new(); key as usize];
    let mut e = String::new();
    if encrypt {
        // Possibly this could be optimized to not have to iterate through string array
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
            e.push_str(rails[n].as_str());
        }
    } else {
        e = String::from(msg);
        let mut gap: i32;
        let mut alt_gap: i32;
        let mut idx = 0;
        for r in 0..key {
            let mut pos = r;
            let mut gap_used = false;
            gap = (key - 1 - r) * 2 ;
            if r != 0 && gap != 0 { alt_gap = (key - 1) * 2 - gap; } else { alt_gap = 0; }
            if gap == 0 { gap = (key - 1) * 2; }
            while pos < msg.len() as i32 {
                e.remove(pos as usize);
                e.insert(pos as usize, msg.chars().nth(idx as usize).unwrap());
                if alt_gap != 0 && gap_used {
                    gap_used = false;
                    pos += alt_gap;
                } else {
                    gap_used = true;
                    pos += gap;
                }
                idx += 1;
            }
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
    fn test_atbash() {
        let s = "defendtheeastwallofthecastle";
        let e = "wvuvmwgsvvzhgdzoolugsvxzhgov";
        assert_eq!(e, atbash_cipher(s, true).as_str());
        assert_eq!(s, atbash_cipher(e, false).as_str());
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
            let e = affine_cipher(a, b, s, true);
            assert_eq!(s, affine_cipher(a, b, e.as_str(), false).as_str());
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
        // Encrypt
        let s = "attackatdawn";
        assert_eq!("acdtaktantaw", railfence_cipher(3, s, true));
        assert_eq!("aatktntcdwaa", railfence_cipher(4, s, true));
        assert_eq!("adttatawaknc", railfence_cipher(5, s, true));

        // decrypt
        assert_eq!(s, railfence_cipher(3, "acdtaktantaw", false));
        assert_eq!(s, railfence_cipher(4, "aatktntcdwaa", false));
        assert_eq!(s, railfence_cipher(5, "adttatawaknc", false));
    }

    #[test]
    fn test_vigenere() {
        let keyword = "fortification";
        let s = "defendtheeastwallofthecastle";
        let e = "iswxvibjexiggbocewkbjeviggqs";
        assert_eq!(e, vigenere_cipher(keyword, s, true));
        assert_eq!(s, vigenere_cipher(keyword, e, false));
    }
}
