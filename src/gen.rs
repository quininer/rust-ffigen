use std::collections::{ HashMap, HashSet };
use clang::Entity;


macro_rules! dump_continue {
    ( $sub:ident in $entitys:expr, $exec:expr ) => {{
        let mut out = String::new();
        for $sub in $entitys { out.push_str(&$exec) };
        out
    }};
    ( $sub:ident of $entity:expr, $exec:expr ) => {
        dump_continue!( $sub in $entity.get_children(), $exec )
    }
}

macro_rules! dump_tab {
    ( $depth:expr ) => {{
        let mut out = String::new();
        for _ in 0..$depth { out.push('\t'); };
        out
    }}
}


pub type UnnamedMap<'tu> = HashMap<Entity<'tu>, String>;
pub type KeywordSet = HashSet<String>;


#[derive(Debug)]
pub struct Status<'tu> {
    pub unmap: &'tu mut UnnamedMap<'tu>,
    pub kwset: &'tu mut KeywordSet,
    pub link: String,
}

impl<'tu> Status<'tu> {
    pub fn trim(&mut self, name: String) -> String {
        let name = name.split_whitespace().last().unwrap();
        if self.kwset.get(name).is_none() {
            name.into()
        } else {
            let name = format!("{}_", name);
            self.kwset.insert(name.clone());
            name
        }
    }

    pub fn takename(&mut self, entity: Entity<'tu>) -> String {
        match entity.get_name() {
            Some(name) => self.trim(name),
            None => {
                let name = format!("Unnamed{}", self.unmap.len());
                self.unmap.entry(entity).or_insert(name.into()).clone()
            }
        }
    }

    pub fn taketype(&mut self, entity: Entity<'tu>) -> String {
        String::from("TODO")
    }
}
