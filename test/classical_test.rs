use super::super::lib;

#[test]
fn test_atbash_encrypt() {
    let mut s = String::new();
    s.push_str("Hello, world!");
    assert_eq!("qtmmjbjgmu", lib::classical::atbash_encrypt(s))
}
