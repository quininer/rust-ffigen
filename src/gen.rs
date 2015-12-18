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
            result.append(&mut format!("(StructDecl {}", entity.get_display_name().unwrap_or("???".into())).into_bytes());
            result.push(b'\n');
            dump_continue!(depth + 1);
            dump_tab!(depth);
            result.push(b')');
            result.push(b'\n');
        },
        // EntityKind::EnumDecl => {},
        // EntityKind::EnumConstantDecl => {},
        // EntityKind::TypedefDecl => {},
        // EntityKind::FunctionDecl => {},
        // EntityKind::ParmDecl => {},
        // EntityKind::TypeRef => {},
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
