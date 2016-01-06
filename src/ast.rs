use clang::{ Entity, EntityKind, TypeKind };
use super::gen::{ UnnamedMap, KeywordSet, Status };
use super::utils::typeconv;


pub fn rust_dump<'tu>(
    entity: &Entity<'tu>,
    mut status: &mut Status<'tu>,
) -> String {
    format!(concat!(
        "//! ffigen generate.\n",
        "\n",
        "#![allow(non_camel_case_types)]\n",
        "#![allow(dead_code)]\n",
        "#![allow(unused_attributes)]\n",
        "#![allow(non_snake_case)]\n",
        "#![allow(non_upper_case_globals)]\n",
        "\n",
        "use libc::*;",
        "\n",
        "{}"
    ), dump(&entity, &mut status, 0, None))
}

fn dump<'tu>(
    entity: &Entity<'tu>,
    mut status: &mut Status<'tu>,
    depth: usize,
    prefix: Option<String>
) -> String {
    let mut out = String::new();
    out.push_str(dump_tab!(depth).as_ref());

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
            }

            for e in t {
                let se = e.get_children();
                if se.len() == 1 && match se[0].get_kind() {
                    EntityKind::StructDecl => true,
                    EntityKind::TypeRef => true,
                    EntityKind::EnumDecl => true,
                    _ => false
                } {
                    let name = status.takename(e.clone());
                    status.unmap.insert(se[0], name);
                } else {
                    out.push_str(&dump(&e, &mut status, depth, None));
                };
            }

            out.push_str(dump_continue!(
                e in s,
                dump(&e, &mut status, depth+1, None)
            ).as_ref());

            out.push_str(&format!(
                "\n#[link(name=\"{}\")]\nextern \"C\" {{\n{}}}",
                status.link.clone(),
                dump_continue!(e in f, dump(&e, &mut status, depth+1, None))
            ));
        },

        EntityKind::StructDecl => {
            let se = entity.get_children();
            if se.len() == 0 {
                out.push_str(&format!(
                    "\npub enum {} {{}}\n",
                    status.takename(entity.clone())
                ));
            } else {
                out.push_str("\n#[repr(C)]\n#[derive(Copy, Clone, Debug)]\n");
                out.push_str(&format!(
                    "\npub struct {} {{\n{}{}}}\n",
                    status.takename(entity.clone()),
                    dump_continue!(e in se, dump(&e, &mut status, depth+1, None)),
                    dump_tab!(depth)
                ));
            }
        },

        EntityKind::FieldDecl => {
            out.push_str(&format!(
                "pub {}: {},\n",
                status.takename(entity.clone()),
                status.taketype(entity.clone())
            ));
        },

        EntityKind::EnumDecl => {
            out.push_str("\n#[repr(C)]\n#[derive(Copy, Clone, Debug)]\n");
            out.push_str(&format!(
                "pub enum {} {{\n{}{}}}\n",
                status.takename(entity.clone()),
                dump_continue!(e of entity, dump(&e, &mut status, depth+1, None)),
                dump_tab!(depth)
            ));
        },

        kind @ _ => out.push_str(&format!(
            "(UNknown {}: {:?})\n{}",
            status.takename(entity.clone()),
            kind,
            dump_continue!(e of entity, dump(&e, &mut status, depth+1, None)),
        ))
    }

    out
}
