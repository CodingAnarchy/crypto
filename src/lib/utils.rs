pub static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                                  'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

pub fn cipher_alphabet(keyword: &str) -> String {
    let mut alpha = String::new();
    for ch in keyword.chars() {
        if !ch.is_alphabetic() || alpha.contains(ch) { continue; }
        alpha.push(ch);
    }
    for c in ASCII_LOWER.iter() {
        if alpha.contains(*c) { continue; }
        alpha.push(*c);
    }
    return alpha;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_alphabet() {
        let k = "royalnewzealandnavy";
        assert_eq!("royalnewzdvbcfghijkmpqstux", cipher_alphabet(k));
    }
}
