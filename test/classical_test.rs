use super::super::lib;

#[test]
fn test_atbash_encrypt() {
    let mut s = String::new();
    s.push_str("Defend the east wall of the castle.");
    assert_eq!("wvuvmwgsvvzhgdzoolugsvxzhgov", lib::classical::atbash_encrypt(s).as_str())
}
