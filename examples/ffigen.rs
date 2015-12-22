extern crate ffigen;

use std::env::args;
use ffigen::GenOptions;


fn main() {
    // ffigen <link> <out type>

    let mut argv = args().skip(1);
    let result = GenOptions::new()
        .arg("-I/usr/lib/clang/3.7.0/include/")
        .link(&argv.next().unwrap())
        .pat(&argv.next().unwrap())
        .gen();

    println!("{}", String::from_utf8_lossy(&result));
}
