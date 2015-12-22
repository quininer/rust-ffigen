extern crate clang;

use clang::{ Entity, EntityKind, TypeKind };
use super::gen::UnnamedMap;


pub fn ast_dump<'tu>(
    entity: &Entity<'tu>,
    depth: usize,
    mut unmap: &mut UnnamedMap<'tu>
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
                    out.push_str(&ast_dump(&e, depth, &mut unmap));
                };
            }
            dump_continue!(
                e in entity.get_children().iter()
                    .filter(|r| match r.get_kind() {
                        EntityKind::TypedefDecl => false,
                        EntityKind::FunctionDecl => false,
                        _ => true
                    }),
                out <- ast_dump(&e, depth, &mut unmap)
            );
            out.push_str("(FunctionGroup\n");
            dump_continue!(
                e in entity.get_children().iter()
                    .filter(|r| r.get_kind() == EntityKind::FunctionDecl),
                out <- ast_dump(&e, depth + 1, &mut unmap)
            );
            out.push_str(")\n");
        },
        EntityKind::StructDecl => {
            out.push_str(&format!(
                "(StructDecl {}\n",
                dump_name!(unmap, entity.clone())
            ));
            dump_continue!(
                e of entity,
                out <- ast_dump(&e, depth + 1, &mut unmap)
            );
            dump_tab!(out, depth);
            out.push_str(")\n");
        },
        EntityKind::FieldDecl => {
            out.push_str(&format!(
                "(FieldDecl {}: {:?} {}\n",
                dump_name!(unmap, entity.clone()),
                entity.get_type()
                    .map_or(TypeKind::Unexposed, |r| r.get_kind()),
                entity.get_type()
                    .and_then(|r| r.get_size())
                    .unwrap_or(!0)
            ));
            dump_continue!(
                e of entity,
                out <- ast_dump(&e, depth + 1, &mut unmap)
            );
            dump_tab!(out, depth);
            out.push_str(")\n");
        },
        EntityKind::EnumDecl => {
            out.push_str(&format!(
                "(EnumDecl {}\n",
                dump_name!(unmap, entity.clone())
            ));
            dump_continue!(
                e of entity,
                out <- ast_dump(&e, depth + 1, &mut unmap)
            );
            dump_tab!(out, depth);
            out.push_str(")\n");
        },
        EntityKind::EnumConstantDecl => {
            out.push_str(&format!(
                "(EnumConstantDecl {}: {:?} = {})\n",
                dump_name!(unmap, entity.clone()),
                entity.get_type().map_or(TypeKind::Int, |r| r.get_kind()),
                entity.get_enum_constant_value().map_or(0, |(r, _)| r)
            ));
        },
        EntityKind::FunctionDecl => {
            out.push_str(&format!(
                "(FunctionDecl {} -> {:?}\n",
                dump_name!(unmap, entity.clone()),
                entity.get_type()
                    .and_then(|r| r.get_result_type())
                    .map_or(TypeKind::Unexposed, |r| r.get_kind())
            ));
            dump_continue!(
                e of entity,
                out <- ast_dump(&e, depth + 1, &mut unmap)
            );
            dump_tab!(out, depth);
            out.push_str(")\n");
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
                out <- ast_dump(&e, depth + 1, &mut unmap)
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
                out <- ast_dump(&e, depth + 1, &mut unmap)
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
                out <- ast_dump(&e, depth + 1, &mut unmap)
            );
        }
    };

    out
}
