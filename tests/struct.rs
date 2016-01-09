extern crate ffigen;

#[macro_use] mod utils;

use ffigen::GenOptions;
use utils::{
    CLANG_INCLUDE_PATH,
    HEAD, TAIL
};


#[test]
fn test_struct() {
    let out = format!("{}\n\n{}\n\n{}", HEAD, r#"
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct bar {
    pub baz: [c_int; 32],
    pub bax: c_int,
    pub bac: c_char,
}"#, TAIL);
    assert_eq!(out.into_bytes(), gen!("struct.h"));
}
