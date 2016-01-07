use std::collections::{ HashMap, HashSet };
use clang::{ Entity, EntityKind, Type, TypeKind };

use super::utils::typeconv;


pub type UnnamedMap<'tu> = HashMap<Entity<'tu>, String>;
pub type KeywordSet = HashSet<String>;
pub type DumpFn<'tu> = fn(
    entity: &Entity<'tu>,
    mut status: &mut Status<'tu>,
    depth: usize,
    prefix: Option<String>
) -> String;


pub struct Status<'tu> {
    pub unmap: &'tu mut UnnamedMap<'tu>,
    pub kwset: &'tu mut KeywordSet,
    pub headers: Vec<String>,
    pub link: String,
    pub dump: Option<DumpFn<'tu>>
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

    pub fn inheader(&self, entity: Entity<'tu>) -> bool {
        entity.get_location()
            .map(|r| r.get_file_location().file.get_path())
            .map(|r| dump_is!(r.to_str().unwrap(), in &self.headers))
            .unwrap_or(false)
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

    pub fn takenext(&mut self, entity: Entity<'tu>, enty: Option<Type>, depth: usize) -> String {
        match enty.map(|r| r.get_kind()) {
            Some(TypeKind::Pointer) => {
                format!(
                    "{}{}",
                    dump_const!(enty).unwrap_or(""),
                    self.taketype(
                        entity,
                        enty.and_then(|r| r.get_pointee_type()),
                        depth
                    )
                )
            },
            Some(TypeKind::Typedef) | Some(TypeKind::Unexposed) => {
                format!(
                    r#"extern "C" fn({}{}{}) -> {}"#,
                    "\n",
                    dump_continue!(
                        e of entity,
                        self.dump.unwrap()(&e, self, depth+1, None)
                    ),
                    dump_tab!(depth),
                    self.takeres(entity, enty, depth)
                )
            },
            Some(TypeKind::IncompleteArray) => {
                format!(
                    "{}{}",
                    "*mut ", // FIXME
                    self.taketype(
                        entity,
                        enty.and_then(|r| r.get_element_type()),
                        depth
                    )
                )
            },
            Some(TypeKind::ConstantArray) => {
                match enty.and_then(|r| r.get_size()) {
                    Some(len) => format!(
                        "[{}{}; {}]",
                        dump_const!(enty).unwrap_or(""),
                        self.taketype(
                            entity,
                            enty.and_then(|r| r.get_element_type()),
                            depth
                        ),
                        len
                    ),
                    None => format!(
                        "{}{}",
                        dump_const!(enty).unwrap_or(""),
                        self.taketype(
                            entity,
                            enty.and_then(|r| r.get_element_type()),
                            depth
                        )
                    )
                }
            },
            _ => enty.or(entity.get_type())
                .map(|r| r.get_kind())
                .map(|r| typeconv(r))
                .map(|r| format!(
                    "{}{}",
                    dump_const!(enty).unwrap_or(""),
                    r
                ))
                .unwrap()
        }
    }

    pub fn taketype(&mut self, entity: Entity<'tu>, enty: Option<Type>, depth: usize) -> String {
        match entity.get_children().iter()
            .filter(|r| r.get_kind() == EntityKind::TypeRef &&
                match enty.map(|x| x.get_kind()) {
                    Some(TypeKind::ConstantArray) => false,
                    Some(TypeKind::IncompleteArray) => false,
                    Some(TypeKind::DependentSizedArray) => false,
                    None => false,
                    _ => true
                }
            )
            .next()
        {
            Some(se) => format!(
                "{}{}",
                dump_const!(enty).unwrap_or(""),
                self.takename(se.clone())
            ),
            None => self.takenext(entity, enty, depth)
        }
    }

    pub fn takeres(&mut self, entity: Entity<'tu>, enty: Option<Type>, depth: usize) -> String {
        match enty
            .and_then(|r| r.get_result_type())
            .and_then(|r| if r.get_kind() == TypeKind::Void { None } else { Some(r) } )
        {
            Some(ty) => format!(
                "{}",
                self.taketype(entity, Some(ty), depth)
            ),
            None => String::from("()")
        }
    }
}
