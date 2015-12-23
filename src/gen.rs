extern crate clang;

use std::collections::HashMap;
use clang::Entity;


macro_rules! dump_tab {
    ( $out:ident, $tab:expr ) => {
        for _ in 0..$tab { $out.push('\t'); }
    }
}

macro_rules! dump_continue {
    ( $sub:ident in $entitys:expr, $out:ident <- $exec:expr ) => {
        for $sub in $entitys { $out.push_str(&$exec) };
    };
    ( $sub:ident of $entity:expr, $out:ident <- $exec:expr ) => {
        dump_continue!( $sub in $entity.get_children(), $out <- $exec )
    }
}

macro_rules! dump_name {
    ( $unmap:expr, $entity:expr ) => {{
        match $entity.get_name() {
            Some(name) => name,
            None => {
                let name = format!("Unnamed{}", $unmap.len());
                $unmap.entry($entity).or_insert(name.into()).clone()
            }
        }
    }}
}

macro_rules! dump_const {
    ( $ety:expr ) => {
        $ety
            .get_pointee_type()
            .map(|r| if r.is_const_qualified() { "*const " } else { "*mut " })
            .unwrap_or("")
    }
}

pub type UnnamedMap<'tu> = HashMap<Entity<'tu>, String>;
