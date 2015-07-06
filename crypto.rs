mod lib;

fn main() {
    let mut s = String::new();
    s.push_str("Hello, world!");
    let e = lib::classical::affine_encrypt(25, 25, s);
    println!("{}", e.to_string());
}
