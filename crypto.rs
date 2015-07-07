#![feature(concat_idents)]

mod lib;
use std::env;
use std::collections::HashMap;
use syntax::parse::token;

#[cfg(test)]
mod test;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let argv: Vec<_> = env::args().collect();
    if argv.len() < 3 {
        println!("Error: expected two arguments - cipher and message.");
        return
    }

    let ciphers = ["atbash", "affine"];

    if ciphers.contains(argv[1]) {
        let f = concat_idents!(token::str_to_ident(argv[1]), _encrypt);
        let e = f("hello");
        println!("{}", e);
    }
}
