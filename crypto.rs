mod lib;
use std::env;

#[cfg(test)]
mod test;

fn main() {
    let mut argv: Vec<_> = env::args().collect();
    if argv.len() < 3 {
        println!("Error: expected two arguments - cipher and message.");
        return
    }

    let ciphers = ["atbash", "affine"];
}
