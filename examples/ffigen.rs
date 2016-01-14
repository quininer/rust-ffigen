extern crate ffigen;

use std::env::{ var, args };
use ffigen::GenOptions;

const CLANG_INCLUDE_PATH: &'static str = "/usr/lib/clang/3.7.0/include/";

fn main() {
    // ffigen <header> <link>

    let mut argv = args().skip(1);
    let out = GenOptions::new()
        .arg(&format!("-I{}", var("CLANG_INCLUDE_PATH").unwrap_or(CLANG_INCLUDE_PATH.into())))
        .header(&argv.next().unwrap())
        .link(&argv.next().unwrap())
        .gen();

    println!("{}", String::from_utf8_lossy(&out));
}
