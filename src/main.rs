#![feature(convert)]
extern crate num;

mod lib;
use std::env;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let argv: Vec<_> = env::args().collect();
    if argv.len() < 3 {
        println!("Error: expected two arguments - cipher and message.");
        return
    }

    let msg = argv[2].as_str();

    match argv[1].as_str() {
        "atbash" => { let e = lib::classical::atbash_encrypt(msg); println!("Ciphertext: {}", e.as_str()); },
        _ => println!("Invalid cipher selected: {}", argv[1])
    }
}
