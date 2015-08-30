#![feature(convert)]
extern crate num;
extern crate rand;

mod lib;
use std::env;
use std::io;
use std::io::prelude::*;
use num::integer::gcd;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let argv: Vec<_> = env::args().collect();
    if argv.len() < 4 {
        println!("Error: expected three arguments - mode, cipher and message.");
        return
    }

    let msg = argv[3].as_str();
    let stdin = io::stdin();
    let mut input = String::new();
    let mut output = "Plain text: ";
    let mut e = String::new();

    let encrypt = argv[1].as_str() == "encrypt";
    if encrypt { output = "Ciphertext: "; }

    match argv[2].as_str() {
        "atbash" => {
            e = lib::classical::atbash_cipher(msg, encrypt);
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
            e = lib::classical::affine_cipher(a, b, msg, encrypt);
        },
        "autokey" => {
            println!("Enter keyword: ");
            stdin.read_line(&mut input).unwrap();
            let key = input.trim_right();
            e = lib::classical::autokey_cipher(key, msg, encrypt);
        }
        "caesar" => {
            let mut k: u32 = 26;
            while k > 25 {
                println!("Enter key (must be between 1 and 25): ");
                stdin.read_line(&mut input).unwrap();
                k = input.trim_right().parse::<u32>().ok().unwrap();
                input.clear();
            }
            e = lib::classical::caesar_cipher(k as i32, msg, encrypt);
        },
        "rot13" => {
            e = lib::classical::rot13_cipher(msg, encrypt);
        },
        "railfence" => {
            println!("Enter number of rails: ");
            stdin.read_line(&mut input).unwrap();
            let k = input.trim_right().parse::<i32>().ok().unwrap();
            e = lib::classical::railfence_cipher(k, msg, encrypt);
        },
        "substitution" => {
            println!("Enter keyphrase: ");
            stdin.read_line(&mut input).unwrap();
            e = lib::classical::substitution_cipher(input.as_str(), msg, encrypt);
        }
        "vigenere" => {
            println!("Enter keyword: ");
            stdin.read_line(&mut input).unwrap();
            let key = input.trim_right();
            e = lib::classical::vigenere_cipher(key, msg, encrypt);
        },
        _ => println!("Invalid cipher selected: {}", argv[2])
    }
    println!("{} {}", output, e.as_str());
}
