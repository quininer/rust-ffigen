extern crate ffigen;

use std::env::args;
use ffigen::GenOptions;

fn main() {
    // ffigen <header> <link>

    let mut argv = args().skip(1);
    let out = GenOptions::default()
        .arg(&format!("-I{}", ffigen::find_clang_include_path().to_string_lossy()))
        .header(&argv.next().unwrap())
        .link(&argv.next().unwrap())
        .gen();

    println!("{}", String::from_utf8_lossy(&out));
}
