use clang::{ Entity, EntityKind };
use super::gen::Status;
use super::utils::{ fetch_prefix, trim_prefix };


pub fn rust_dump<'tu>(
    entity: &Entity<'tu>,
    mut status: &mut Status<'tu>,
) -> String {
    status.dump = Some(dump);

    let out = dump(&entity, &mut status, 0, None);

    if status.optformat {
        format!(concat!(
            "//! ffigen generate.\n",
            "\n",
            "#![allow(dead_code)]\n",
            "#![allow(non_snake_case)]\n",
            "#![allow(unused_attributes)]\n",
            "#![allow(non_camel_case_types)]\n",
            "#![allow(non_upper_case_globals)]\n",
            "\n",
            "use libc::*;\n",
            "\n",
            "{}"
        ), out)
    } else {
        out
    }
}

fn dump<'tu>(
    entity: &Entity<'tu>,
    mut status: &mut Status<'tu>,
    depth: usize,
    prefix: Option<String>
) -> String {
    let mut out = String::new();

    if status.optcomment {
        out.push_str(
            &entity.get_comment()
                .map(|r| (dump_tab!(depth), r))
                .map_or(String::new(), |(d, r)| r.lines()
                     .map(|x| format!("{}{}\n", d, x))
                     .collect::<Vec<String>>()
                     .concat()
                )
        )
    };

    out.push_str(dump_tab!(depth).as_ref());

    match entity.get_kind() {
        EntityKind::TranslationUnit => {
            let mut t = Vec::new();
            let mut s = Vec::new();
            let mut f = Vec::new();

            for e in entity.get_children().iter()
                .filter(|&r| status.inheader(*r))
            {
                match e.get_kind() {
                    EntityKind::TypedefDecl => t.push(*e),
                    EntityKind::FunctionDecl => f.push(*e),
                    _ => s.push(*e)
                };
            }

            for e in t {
                let se = e.get_children();
                if se.len() == 1 && match se[0].get_kind() {
                    EntityKind::StructDecl
                    | EntityKind::TypeRef
                    | EntityKind::EnumDecl => true,
                    _ => false
                } {
                    let name = status.takename(e);
                    status.unmap.insert(se[0], name);
                } else {
                    out.push_str(&dump(&e, &mut status, depth, None));
                };
            }

            out.push_str(dump_continue!(
                e in s,
                dump(&e, &mut status, depth, None)
            ).as_ref());

            out.push_str(&format!(
                "\n#[link(name=\"{}\")]\nextern \"C\" {{\n{}}}",
                status.link.clone(),
                dump_continue!(e in f, dump(&e, &mut status, depth+1, None))
            ));
        },

        EntityKind::StructDecl => {
            let se = entity.get_children();
            if se.is_empty() {
                out.push_str(&format!(
                    "\npub enum {} {{}}\n",
                    status.takename(entity.clone())
                ));
            } else {
                out.push_str("\n#[repr(C)]\n#[derive(Copy, Clone, Debug, PartialEq)]\n");
                out.push_str(&format!(
                    "pub struct {} {{\n{}{}}}\n",
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
                status.taketype(entity.clone(), entity.get_type(), depth)
            ));
        },

        EntityKind::EnumDecl => {
            out.push_str("\n#[repr(C)]\n#[derive(Copy, Clone, Debug, PartialEq)]\n");
            out.push_str(&format!(
                "pub enum {} {{\n{}{}}}\n",
                status.takename(entity.clone()),
                dump_continue!(
                    e of entity,
                    dump(&e, &mut status, depth+1, Some(fetch_prefix(
                        entity.get_children().iter()
                            .filter(|r| r.get_kind() == EntityKind::EnumConstantDecl)
                            .map(|r| r.get_name().unwrap_or(String::new()))
                            .collect()
                    )))
                ),
                dump_tab!(depth)
            ));
        },

        EntityKind::EnumConstantDecl => {
            let name = status.takename(*entity);
            out.push_str(&format!(
                "{}{},\n",
                if status.optformat {
                    trim_prefix(&name, &prefix.unwrap())
                } else { name },
                match entity.get_enum_constant_value().map(|(r, _)| r) {
                    Some(r) => format!(" = {}", r),
                    None => String::new()
                }
            ));
        },

        EntityKind::FunctionDecl => {
            out.push_str(&format!(
                "pub fn {}(\n{}{}) -> {};\n",
                status.takename(entity.clone()),
                dump_continue!(
                    e in entity.get_children().iter()
                        .filter(|r| r.get_kind() == EntityKind::ParmDecl),
                    dump(&e, &mut status, depth+1, None)
                ),
                dump_tab!(depth),
                status.takeres(entity.clone(), entity.get_type(), depth)
            ));
        },

        EntityKind::ParmDecl => {
            out.push_str(&format!(
                "{}: {},\n",
                status.takename(entity.clone()),
                status.taketype(entity.clone(), entity.get_type(), depth)
            ));
        },

        EntityKind::TypedefDecl => {
            out.push_str(&format!(
                "pub type {} = {};\n",
                status.takename(entity.clone()),
                format!(
                    "{}{}",
                    dump_const!(entity.get_type()).unwrap_or(""),
                    status.taketype(entity.clone(), entity.get_type(), depth)
                )
            ));
        },

        kind => panic!("{}: {:?}", status.takename(entity.clone()), kind)

        // kind => out.push_str(&format!(
        //     "(Unknown {}: {:?})\n{}",
        //     status.takename(entity.clone()),
        //     kind,
        //     dump_continue!(e of entity, dump(&e, &mut status, depth+1, None)),
        // ))
    }

    out
}
