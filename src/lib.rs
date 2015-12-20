extern crate clang;

#[macro_use] pub mod gen;
mod ast;

use clang::{ Clang, Index, ParseOptions, TranslationUnit };
use gen::UnnamedMap;
use ast::ast_dump;


#[derive(Debug, Clone)]
pub enum OutType {
    Ast, // Debug
    Rust
}

#[derive(Debug, Clone)]
pub struct GenOptions<'g> {
    pub args: Vec<&'g str>,
    pub link: Option<&'g str>,
    pub parse: ParseOptions,
    pub outtype: OutType,
}

impl<'g> GenOptions<'g> {
    pub fn new() -> GenOptions<'g> {
        GenOptions {
            args: Vec::new(),
            link: None,
            parse: ParseOptions::default(),
            outtype: OutType::Ast
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
    pub fn out(mut self, t: OutType) -> GenOptions<'g> {
        self.outtype = t;
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

        (match self.outtype {
            OutType::Ast => ast_dump,
            OutType::Rust => panic!("TODO")
        })(&entity, 0, &mut UnnamedMap::new()).into_bytes()
    }
}
