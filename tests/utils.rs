extern crate ffigen;

pub const CLANG_INCLUDE_PATH: &'static str = "/usr/lib/clang/3.7.0/include/";
pub const HEAD: &'static str = "//! ffigen generate.

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use libc::*;";
pub const TAIL: &'static str = r#"#[link(name="test")]
extern "C" {
}"#;


macro_rules! gen {
    ( $l:expr, [ $( $h:expr ),* ], $f:expr ) => {
        GenOptions::new()
            .arg(&format!("-I{}", CLANG_INCLUDE_PATH))
        $(
            .header(&format!("{}/tests/headers/{}", env!("PWD"), $h))
        )*
            .link($l)
            .format($f)
            .gen()
    };
    ( $l:expr, $h:expr ) => {
        gen!($l, [$h], true)
    };
    ( $h:expr ) => {
        gen!("test", $h)
    }
}
