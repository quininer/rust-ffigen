#[macro_use] extern crate ffigen;

use std::env::args;

fn main() {
    // ffigen <header> <link>

    let mut argv = args().skip(1);

    println!(
        "{}",
        String::from_utf8_lossy(&gen!(
            &argv.next().unwrap(),
            [ &argv.next().unwrap() ]
        ))
    );
}
