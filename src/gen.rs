extern crate clang;

use std::collections::HashMap;
use clang::Entity;


macro_rules! dump_tab {
    ( $tab:expr ) => {{
        let mut out = String::new();
        for _ in 0..$tab { out.push('\t'); };
        out
    }}
}

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

macro_rules! dump_res {
    ( $unmap:expr, $entity:expr ) => {
        match $entity.get_type()
            .and_then(|r| r.get_result_type())
            .and_then(|r| if r.get_kind() == TypeKind::Void { None } else { Some(r) })
        {
            Some(ty) => format!(
                " -> {}{}",
                dump_const!(ty),
                match $entity.get_children().iter()
                    .filter(|r| r.get_kind() == EntityKind::TypeRef)
                    .next()
                {
                    Some(se) => dump_name!($unmap, se.clone()),
                    None => typeconv(ty.get_kind())
                }
            ),
            None => String::from("")
        }
    }
}

macro_rules! dump_type {
    ( $unmap:expr, $depth:expr, $entity:expr, $parm:expr ) => {
        match $entity.get_children().iter()
            .filter(|r| r.get_kind() == EntityKind::TypeRef)
            .next()
        {
            Some(se) => dump_name!($unmap, se.clone()),
            None => match $entity.get_type().map(|r| r.get_kind()) {
                Some(TypeKind::Pointer) => $entity.get_type()
                    .and_then(|r| r.get_pointee_type())
                    .map(|r| r.get_kind())
                    .map(|r| if r == TypeKind::Unexposed {
                            format!(
                                r#"extern "C" fn({}{}{}){}"#,
                                "\n",
                                $parm,
                                dump_tab!($depth),
                                dump_res!($unmap, $entity)
                            )
                        } else { typeconv(r) })
                    .unwrap(),
                // FIXME if fn
                Some(TypeKind::Typedef) => format!(
                    r#"extern "C" fn({}{}{}){}"#,
                    "\n",
                    $parm,
                    dump_tab!($depth),
                    dump_res!($unmap, $entity)
                ),
                _ => typeconv($entity.get_type().map(|r| r.get_kind()).unwrap())
            }
        }
    }
}

pub type UnnamedMap<'tu> = HashMap<Entity<'tu>, String>;
