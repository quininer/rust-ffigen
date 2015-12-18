extern crate clang;

use clang::{ Clang, Index, TranslationUnit, Entity, EntityKind };

pub fn generate(
    opts: super::GenOptions,
    dump: fn(entity: &Entity, depth: usize) -> Vec<u8>
) -> Vec<u8>{
    let c = Clang::new().unwrap();
    let mut i = Index::new(&c, true, false);
    let t = TranslationUnit::from_source(
        &mut i,
        opts.link.unwrap(),
        &opts.args[..],
        &[],
        opts.parse
    ).unwrap();

    let entity = t.get_entity();

    dump(&entity, 0)
}

pub fn ast_dump(entity: &Entity, depth: usize) -> Vec<u8> {
    let mut result = Vec::new();

    macro_rules! dump_continue {
        ( $depth:expr ) => {
            for e in entity.get_children() {
                result.append(&mut ast_dump(&e, $depth));
            }
        }
    }
    macro_rules! dump_tab {
        ( $depth:expr ) => {{
            let mut count = 0;
            while count < $depth {
                result.push(b'\t');
                count += 1;
            }
        }}
    }

    if entity.get_location().map_or(true, |r| !r.is_in_main_file()) {
        dump_continue!(depth);
        return result;
    };

    dump_tab!(depth);

    match entity.get_kind() {
        EntityKind::StructDecl => {
            // FIXME name missing
            result.append(&mut format!(
                "(StructDecl {}",
                entity.get_name().unwrap_or("???".into())
            ).into_bytes());

            result.push(b'\n');
            dump_continue!(depth + 1);
            dump_tab!(depth);
            result.push(b')');
            result.push(b'\n');
        },
        EntityKind::EnumDecl => {
            result.append(&mut format!(
                "(EnumDecl {}",
                entity.get_name().unwrap_or("???".into())
            ).into_bytes());

            result.push(b'\n');
            dump_continue!(depth + 1);
            dump_tab!(depth);
            result.push(b')');
            result.push(b'\n');
        },
        EntityKind::EnumConstantDecl => {
            result.append(&mut format!(
                "(EnumConstantDecl {}: {})",
                entity.get_name().unwrap_or("???".into()),
                entity.get_type().map_or(String::from("???"), |r| r.get_display_name())
            ).into_bytes());

            result.push(b'\n');
        },
        EntityKind::FunctionDecl => {
            result.append(&mut format!(
                "(FunctionDecl {} -> {}",
                entity.get_name().unwrap_or("???".into()),
                entity.get_type().map_or(
                    String::from("None"),
                    |r| r.get_result_type().map_or(
                        String::from("None"),
                        |x| x.get_display_name()
                    )
                )
            ).into());

            result.push(b'\n');
            dump_continue!(depth + 1);
            dump_tab!(depth);
            result.push(b')');
            result.push(b'\n');
        },
        EntityKind::ParmDecl => {
            result.append(&mut format!(
                "(ParmDecl {}: {})",
                entity.get_name().unwrap_or("???".into()),
                entity.get_type().map_or(String::from("???"), |r| r.get_display_name())
            ).into());

            result.push(b'\n');
        },
        // EntityKind::TypedefDecl => {},
        // EntityKind::TypeRef => {},
        // EntityKind::FieldDecl => {},
        _ => {
            result.append(&mut entity.get_name().unwrap_or(String::from("None")).into_bytes());
            result.push(b'\n');

            dump_continue!(depth + 1);
        }
    };

    result
}

#[allow(unused_variables)]
pub fn rust_dump(entity: &Entity, depth: usize) -> Vec<u8> {
    unimplemented!()
}
