extern crate clang;

use clang::{ Entity, EntityKind, TypeKind };
use super::gen::UnnamedMap;
use super::types::typeconv;


pub fn rust_dump<'tu>(
    entity: &Entity<'tu>,
    depth: usize,
    mut unmap: &mut UnnamedMap<'tu>,
    pat: String
) -> String {
    dump(&entity, depth, &mut unmap, &pat)
}


pub fn dump<'tu>(
    entity: &Entity<'tu>,
    depth: usize,
    mut unmap: &mut UnnamedMap<'tu>,
    pat: &str
) -> String {
    let mut out = String::new();

    if entity.get_kind() != EntityKind::TranslationUnit &&
        entity.get_location().map_or(true, |r| !r.is_in_main_file())
    {
        return out;
    };

    dump_tab!(out, depth);

    match entity.get_kind() {
        EntityKind::TranslationUnit => {
            for e in entity.get_children().iter()
                .filter(|r| r.get_kind() == EntityKind::TypedefDecl)
            {
                let sube = e.get_children();
                if sube.len() == 1 && match sube[0].get_kind() {
                    EntityKind::StructDecl => true,
                    EntityKind::TypeRef => true,
                    EntityKind::EnumDecl => true,
                    _ => false
                } {
                    let name = dump_name!(unmap, e.clone());
                    unmap.insert(sube[0], name);
                } else {
                    out.push_str(&dump(&e, depth, &mut unmap, pat));
                };
            }
            dump_continue!(
                e in entity.get_children().iter()
                    .filter(|r| match r.get_kind() {
                        EntityKind::TypedefDecl => false,
                        EntityKind::FunctionDecl => false,
                        _ => true
                    }),
                out <- dump(&e, depth, &mut unmap, pat)
            );
            out.push_str(&format!(
                "#[link(name=\"{}\")]\nextern \"C\" {{\n",
                pat
            ));
            dump_continue!(
                e in entity.get_children().iter()
                    .filter(|r| r.get_kind() == EntityKind::FunctionDecl),
                out <- dump(&e, depth + 1, &mut unmap, pat)
            );
            out.push_str("}\n");
        },
        EntityKind::StructDecl => {
            let se = entity.get_children();
            if se.len() == 0 {
                out.push_str(&format!(
                    "pub enum {} {{",
                    dump_name!(unmap, entity.clone())
                ))
            } else {
                out.push_str(&format!(
                    "pub struct {} {{\n",
                    dump_name!(unmap, entity.clone())
                ));
                dump_continue!(
                    e in se,
                    out <- dump(&e, depth + 1, &mut unmap, pat)
                );
                dump_tab!(out, depth);
            }
            out.push_str("}\n");
        },
        EntityKind::FieldDecl => {
            out.push_str(&format!(
                "{}: {}\n",
                dump_name!(unmap, entity.clone()),
                if entity.get_type()
                    .map(|r| r.get_kind())
                    .map_or(false, |r| r == TypeKind::ConstantArray)
                {
                    format!(
                        "[{}; {}]",
                        dump_name!(unmap, entity.get_children()[0]),
                        entity.get_type().and_then(|r| r.get_size()).unwrap_or(0)
                    )
                } else {
                    dump(&entity, depth + 1, &mut unmap, pat)
                }
            ));
        },
        EntityKind::EnumDecl => {
            out.push_str(&format!(
                "pub enum {} {{\n",
                dump_name!(unmap, entity.clone())
            ));
            dump_continue!(
                e of entity,
                out <- dump(&e, depth + 1, &mut unmap, pat)
            );
            dump_tab!(out, depth);
            out.push_str("}\n");
        },
        EntityKind::EnumConstantDecl => {
            out.push_str(&format!(
                "{}: {}{},\n",
                dump_name!(unmap, entity.clone()),
                typeconv(entity.get_type().map_or(TypeKind::Int, |r| r.get_kind())),
                match entity.get_enum_constant_value().map(|(r, _)| r) {
                    Some(r) => format!(" = {}", r),
                    _ => "".into()
                }
            ));
        },
        EntityKind::FunctionDecl => {
            out.push_str(&format!(
                "pub fn {}(\n",
                dump_name!(unmap, entity.clone())
            ));
            dump_continue!(
                e of entity,
                out <- dump(&e, depth + 1, &mut unmap, pat)
            );
            dump_tab!(out, depth);
            match entity.get_type()
                .and_then(|r| r.get_result_type())
                .map(|r| typeconv(r.get_kind()))
            {
                Some(name) => out.push_str(&format!(") -> {};\n", name)),
                _ => out.push_str(");\n")
            };
        },
        EntityKind::ParmDecl => {
            // FIXME callback function
            out.push_str(&format!(
                "(ParmDecl {}: {:?} {:?}\n",
                dump_name!(unmap, entity.clone()),
                entity
                    .get_type()
                    .and_then(|r| r.get_pointee_type())
                    .map_or(TypeKind::Unexposed, |r| r.get_kind()),
                entity
                    .get_type()
                    .map_or(TypeKind::Unexposed, |r| r.get_kind())
            ));
            dump_continue!(
                e of entity,
                out <- dump(&e, depth + 1, &mut unmap, pat)
            );
            dump_tab!(out, depth);
            out.push_str(")\n");
        },
        EntityKind::TypeRef => {
            out.push_str(&format!(
                "(TypeRef {})\n",
                dump_name!(unmap, entity.clone())
            ));
        },
        EntityKind::TypedefDecl => {
            out.push_str(&format!(
                "(TypedefDecl {}\n",
                dump_name!(unmap, entity.clone())
            ));
            dump_continue!(
                e of entity,
                out <- dump(&e, depth + 1, &mut unmap, pat)
            );
            dump_tab!(out, depth);
            out.push_str(")\n");
        },
        _ => {
            out.push_str(&format!(
                "{}: {:?}\n",
                dump_name!(unmap, entity.clone()),
                entity.get_kind()
            ));

            dump_continue!(
                e of entity,
                out <- dump(&e, depth + 1, &mut unmap, pat)
            );
        }
    };

    out
}
