#![feature(set_recovery)] 

extern crate clang;
#[macro_use] extern crate lazy_static;

#[macro_use] pub mod gen;
mod types;
mod rust;

use clang::{ Clang, Index, ParseOptions, TranslationUnit };
use gen::UnnamedMap;
use rust::rust_dump;


#[derive(Debug, Clone)]
pub struct GenOptions<'g> {
    pub args: Vec<&'g str>,
    pub link: Option<&'g str>,
    pub matchpat: String,
    pub parse: ParseOptions
}

impl<'g> GenOptions<'g> {
    pub fn new() -> GenOptions<'g> {
        GenOptions {
            args: Vec::new(),
            link: None,
            matchpat: String::new(),
            parse: ParseOptions::default(),
        }
    }

    pub fn arg(mut self, a: &'g str) -> GenOptions<'g> {
        self.args.push(a);
        self
    }
    pub fn link(mut self, l: &'g str) -> GenOptions<'g> {
        self.link = Some(l);
        self
    }
    pub fn pat(mut self, m: &'g str) -> GenOptions<'g> {
        self.matchpat = m.into();
        self
    }

    pub fn gen(self) -> Vec<u8> {
        let c = Clang::new().unwrap();
        let mut i = Index::new(&c, true, false);
        let t = TranslationUnit::from_source(
            &mut i,
            self.link.unwrap(),
            &self.args[..],
            &[],
            self.parse
        ).unwrap();
        let entity = t.get_entity();

        rust_dump(&entity, 0, &mut UnnamedMap::new(), self.matchpat).into_bytes()
    }
}
