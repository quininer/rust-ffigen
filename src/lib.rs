extern crate clang;

use clang::{ Clang, Index, TranslationUnit, ParseOptions, Entity };

pub struct GenOptions<'g> {
    pub args: Vec<&'g str>,
    pub link: Option<&'g str>,
}

impl<'g> GenOptions<'g> {
    pub fn new() -> GenOptions<'g> {
        GenOptions {
            args: Vec::new(),
            link: None
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

    pub fn dump(&self) {
        let c = Clang::new().unwrap();
        let mut i = Index::new(&c, true, false);
        let t = TranslationUnit::from_source(
            &mut i,
            self.link.unwrap(),
            &self.args[..],
            &[],
            ParseOptions::default()
        ).unwrap();

        let entity = t.get_entity();

        dump(&entity, 0);
    }
}

fn dump(entity: &Entity, depth: usize) {
    let mut i = 0;
    while i < depth {
        i += 1;
        print!("    ");
    }

    println!("{:?}", entity.get_name());
    for e in entity.get_children() {
        dump(&e, depth + 1);
    }
}
