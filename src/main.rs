#![feature(convert)]
extern crate num;

mod lib;
use std::env;
use std::io;
use std::io::prelude::*;
use num::integer::gcd;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let argv: Vec<_> = env::args().collect();
    if argv.len() < 3 {
        println!("Error: expected two arguments - cipher and message.");
        return
    }

    let msg = argv[2].as_str();
    let mut stdin = io::stdin();
    let mut input = String::new();

    match argv[1].as_str() {
        "atbash" => {
            let e = lib::classical::atbash_encrypt(msg);
            println!("Ciphertext: {}", e.as_str());
        },
        "affine" => {
            let mut a: u32 = 26;
            let mut b: u32 = 26;
            while gcd(a, 26) != 1 || a > 25 {
                println!("Enter a (must be relatively prime with 26): ");
                stdin.read_line(&mut input).unwrap();
                a = input.trim_right().parse::<u32>().ok().unwrap();
                input.clear();
            }
            while b > 25 {
                println!("Enter b (must be between 1 and 25): ");
                stdin.read_line(&mut input).unwrap();
                b = input.trim_right().parse::<u32>().ok().unwrap();
                input.clear();
            }
            let e = lib::classical::affine_encrypt(a, b, msg);
            println!("Ciphertext: {}", e.as_str());
        },
        _ => println!("Invalid cipher selected: {}", argv[1])
    }
}
