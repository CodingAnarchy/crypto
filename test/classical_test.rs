use super::super::lib;

#[test]
fn test_atbash_encrypt() {
    let s = "Defend the east wall of the castle.";
    assert_eq!("wvuvmwgsvvzhgdzoolugsvxzhgov", lib::classical::atbash_encrypt(s).as_str())
}
