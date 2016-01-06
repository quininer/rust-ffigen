#![feature(set_recovery)]

extern crate clang;
#[macro_use] extern crate lazy_static;

#[macro_use] pub mod gen;
mod utils;
mod ast;

use clang::{ Clang, Index, ParseOptions, TranslationUnit };
use gen::{ UnnamedMap, KeywordSet, Status };
use ast::rust_dump;

macro_rules! set {
    ( $( $e:expr ),* ) => {{
        let mut tmp_set = KeywordSet::new();
        $(
            tmp_set.insert(String::from($e));
        )*
        tmp_set
    }}
}


#[derive(Debug, Clone)]
pub struct GenOptions<'g> {
    pub args: Vec<&'g str>,
    pub header: Option<&'g str>,
    pub link: String,
    pub parse: ParseOptions
}

impl<'g> GenOptions<'g> {
    pub fn new() -> GenOptions<'g> {
        GenOptions {
            args: Vec::new(),
            header: None,
            link: String::new(),
            parse: ParseOptions::default(),
        }
    }

    pub fn arg(mut self, a: &'g str) -> GenOptions<'g> {
        self.args.push(a);
        self
    }
    pub fn header(mut self, l: &'g str) -> GenOptions<'g> {
        self.header = Some(l);
        self
    }
    pub fn link(mut self, m: &'g str) -> GenOptions<'g> {
        self.link = m.into();
        self
    }

    pub fn gen(self) -> Vec<u8> {
        let c = Clang::new().unwrap();
        let mut i = Index::new(&c, true, false);
        let t = TranslationUnit::from_source(
            &mut i,
            self.header.unwrap(),
            &self.args[..],
            &[],
            self.parse
        ).unwrap();
        let mut kwset = set![
            "abstract", "alignof", "as", "become", "box",
            "break", "const", "continue", "crate", "do",
            "else", "enum", "extern", "false", "final",
            "fn", "for", "if", "impl", "in",
            "let", "loop", "macro", "match", "mod",
            "move", "mut", "offsetof", "override", "priv",
            "proc", "pub", "pure", "ref", "return",
            "Self", "self", "sizeof", "static", "struct",
            "super", "trait", "true", "type", "typeof",
            "unsafe", "unsized", "use", "virtual", "where",
            "while", "yield"
        ];

        let entity = t.get_entity();
        let mut status = Status {
            unmap: &mut UnnamedMap::new(),
            kwset: &mut kwset,
            link: self.link,
        };

        rust_dump(&entity, &mut status).into_bytes()
    }
}
