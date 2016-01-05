use clang::{ Entity, EntityKind, TypeKind };
use super::gen::{ UnnamedMap, trim };
use super::types::typeconv;


pub fn rust_dump<'tu>(
    entity: &Entity<'tu>,
    depth: usize,
    mut unmap: &mut UnnamedMap<'tu>,
    pat: String
) -> String {
    format!("//! ffigen generate.
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use libc::*;

{}", dump(&entity, depth, &mut unmap, &pat))
}


pub fn dump<'tu>(
    entity: &Entity<'tu>,
    depth: usize,
    mut unmap: &mut UnnamedMap<'tu>,
    pat: &str
) -> String {
    let mut out = String::new();
    out.push_str(&format!("{}", dump_tab!(depth)));

    match entity.get_kind() {
        EntityKind::TranslationUnit => {
            let mut t = Vec::new();
            let mut s = Vec::new();
            let mut f = Vec::new();

            for e in entity.get_children().iter()
                .filter(|&r| r.get_location().map_or(false, |r| r.is_in_main_file()))
            {
                match e.get_kind() {
                    EntityKind::TypedefDecl => t.push(e.clone()),
                    EntityKind::FunctionDecl => f.push(e.clone()),
                    _ => s.push(e.clone())
                };
            };

            for e in t {
                let se = e.get_children();
                if se.len() == 1 && match se[0].get_kind() {
                    EntityKind::StructDecl => true,
                    EntityKind::TypeRef => true,
                    EntityKind::EnumDecl => true,
                    _ => false
                } {
                    let name = dump_name!(unmap, e.clone());
                    unmap.insert(se[0], name);
                } else {
                    out.push_str(&dump(&e, depth, &mut unmap, pat));
                };
            };

            out.push_str(&format!("{}", dump_continue!(
                e in s,
                dump(&e, depth, &mut unmap, pat))
            ));

            out.push_str(&format!(
                "#[link(name=\"{}\")]\nextern \"C\" {{\n{}\n}}",
                pat,
                dump_continue!(e in f, dump(&e, depth + 1, &mut unmap, pat))
            ));
        },
        EntityKind::StructDecl => {
            let se = entity.get_children();
            if se.len() == 0 {
                out.push_str(&format!(
                    "pub enum {} {{}}\n\n",
                    dump_name!(unmap, entity.clone())
                ))
            } else {
                out.push_str("#[repr(C)]\n#[derive(Copy, Clone, Debug)]\n");
                out.push_str(&format!(
                    "pub struct {} {{\n{}\n{}}}\n\n",
                    dump_name!(unmap, entity.clone()),
                    dump_continue!(e in se, dump(&e, depth + 1, &mut unmap, pat)),
                    dump_tab!(depth)
                ));
            }
        },
        EntityKind::FieldDecl => {
            out.push_str(&format!(
                "pub {}: {},\n",
                dump_name!(unmap, entity.clone()),
                if entity.get_type()
                    .map(|r| r.get_kind())
                    .map_or(false, |r| r == TypeKind::ConstantArray)
                {
                    format!(
                        "[{}; {}]",
                        format!(
                            "{}{}",
                            dump_const!(entity.get_type().unwrap()),
                            dump_type!(unmap, depth, entity, dump_continue!(
                                e of entity,
                                dump(&e, depth + 1, &mut unmap, pat)
                            ))
                        ),
                        entity.get_type().and_then(|r| r.get_size()).unwrap_or(0)
                    )
                } else {
                    format!(
                        "{}{}",
                        dump_const!(entity.get_type().unwrap()),
                        dump_type!(unmap, depth, entity, dump_continue!(
                            e of entity,
                            dump(&e, depth + 1, &mut unmap, pat)
                        ))
                    )
                }
            ));
        },
        EntityKind::EnumDecl => {
            out.push_str("#[repr(C)]\n#[derive(Copy, Clone, Debug)]\n");
            out.push_str(&format!(
                "pub enum {} {{\n{}\n{}}}\n\n",
                dump_name!(unmap, entity.clone()),
                dump_continue!(e of entity, dump(&e, depth + 1, &mut unmap, pat)),
                dump_tab!(depth)
            ));;
        },
        EntityKind::EnumConstantDecl => {
            out.push_str(&format!(
                "{}{},\n",
                dump_name!(unmap, entity.clone()),
                match entity.get_enum_constant_value().map(|(r, _)| r) {
                    Some(r) => format!(" = {}", r),
                    _ => "".into()
                }
            ));
        },
        EntityKind::FunctionDecl => {
            out.push_str(&format!(
                "pub fn {}(\n{}{}){};\n",
                dump_name!(unmap, entity.clone()),
                dump_continue!(
                    e in entity.get_children().iter()
                        .filter(|r| r.get_kind() == EntityKind::ParmDecl),
                    dump(&e, depth + 1, &mut unmap, pat)
                ),
                dump_tab!(depth),
                dump_res!(unmap, entity)
            ));
        },
        EntityKind::ParmDecl => {
            out.push_str(&format!(
                "{}: {},\n",
                dump_name!(unmap, entity.clone()),
                format!(
                    "{}{}",
                    entity.get_type().map_or("", |r| dump_const!(r)),
                    dump_type!(unmap, depth, entity, dump_continue!(
                        e of entity,
                        dump(&e, depth + 1, &mut unmap, pat)
                    ))
                )
            ));
        },
        EntityKind::TypedefDecl => {
            out.push_str(&format!(
                "pub type {} = {};\n",
                dump_name!(unmap, entity.clone()),
                dump_type!(unmap, depth, entity, dump_continue!(
                    e of entity,
                    dump(&e, depth + 1, &mut unmap, pat)
                ))
            ));
        },
        _ => {
            out.push_str(&format!(
                "(Unknown {}: {:?})\n{}",
                dump_name!(unmap, entity.clone()),
                entity.get_kind(),
                dump_continue!(e of entity, dump(&e, depth + 1, &mut unmap, pat))
            ));
        }
    };

    out
}
