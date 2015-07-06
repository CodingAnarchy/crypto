mod lib;

#[cfg(test)]
mod test;

fn main() {
    let mut s = String::new();
    s.push_str("Hello, world!");
    let e = lib::classical::atbash_encrypt(s);
    println!("{}", e.to_string());
}
