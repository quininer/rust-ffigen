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
    pub dump: Option<DumpFn<'tu>>,
    pub optcomment: bool,
    pub optformat: bool
}

impl<'tu> Status<'tu> {
    pub fn trim(&mut self, name: String) -> String {
        let name = name.split_whitespace().last().expect("trim split name.");
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
            .map(|r| self.headers.iter().any(|h| h == r.to_str().unwrap()))
            .unwrap_or(false)
    }

    pub fn takename(&mut self, entity: Entity<'tu>) -> String {
        let name = match self.unmap.clone().get(&entity) {
            Some(nm) => nm.clone(),
            None => entity.get_name()
                .map(|r| self.trim(r))
                .unwrap_or(format!("Unnamed{}", self.unmap.len()))
        };
        self.unmap.insert(entity, name.clone());
        name
    }

    pub fn takenext(&mut self, entity: Entity<'tu>, enty: Option<Type>, depth: usize) -> String {
        match enty.map(|r| r.get_kind()) {
            Some(TypeKind::Pointer) => {
                let poity = enty.and_then(|r| r.get_pointee_type());
                match poity.map(|r| r.get_kind()) {
                    Some(TypeKind::Typedef) | Some(TypeKind::Unexposed) =>
                        self.taketype(entity, poity, depth),
                    _ => format!(
                        "{}{}",
                        dump_const!(enty).unwrap_or(""),
                        self.taketype(
                            entity,
                            enty.and_then(|r| r.get_pointee_type()),
                            depth
                        )
                    )
                }
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
            Some(se) => match se.get_type()
                .map(|r| r.get_canonical_type())
                .map(|r| r.get_kind())
            {
                Some(TypeKind::FunctionPrototype) => self.takename(se.clone()),
                _ => format!(
                    "{}{}",
                    dump_const!(enty).unwrap_or(""),
                    self.takename(se.clone())
                )
            },
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
