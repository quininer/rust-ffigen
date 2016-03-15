#![allow(dead_code)]

extern crate ffigen;

pub const HEAD: &'static str = "//! ffigen generate.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use libc::*;";
pub const TAIL: &'static str = r#"#[link(name="test")]
extern "C" {
}"#;


macro_rules! gen {
    ( $l:expr, [ $( $h:expr ),* ], $f:expr ) => {
        GenOptions::default()
            .arg(&format!("-I{}", ffigen::find_clang_include_path().to_string_lossy()))
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
