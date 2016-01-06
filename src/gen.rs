use std::collections::{ HashMap, HashSet };
use clang::{ Entity, EntityKind, Type, TypeKind };

use super::utils::typeconv;


pub type UnnamedMap<'tu> = HashMap<Entity<'tu>, String>;
pub type KeywordSet = HashSet<String>;


#[derive(Debug)]
pub struct Status<'tu> {
    pub unmap: &'tu mut UnnamedMap<'tu>,
    pub kwset: &'tu mut KeywordSet,
    pub link: String,
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

    pub fn takename(&mut self, entity: Entity<'tu>) -> String {
        match entity.get_name() {
            Some(name) => self.trim(name),
            None => {
                let name = format!("Unnamed{}", self.unmap.len());
                self.unmap.entry(entity).or_insert(name.into()).clone()
            }
        }
    }

    // FIXME Should give priority to use .next ?
    // TODO use dump
    pub fn taketype(&mut self, entity: Entity<'tu>, enty: Option<Type>) -> String {
        let enty = enty.or(entity.get_type());
        match enty.map(|r| r.get_kind()) {
            // Some(TypeKind::Pointer) => { /* x fn */ },
            Some(TypeKind::Typedef) | Some(TypeKind::Unexposed) => {
                unimplemented!()
            },
            // Some(TypeKind::ConstantArray) => { /* array */ },
            // Some(TypeKind::IncompleteArray) => { /* x array */ },
            _ => self.takenext(entity, enty)
        }
    }

    pub fn takenext(&mut self, entity: Entity<'tu>, enty: Option<Type>) -> String {
        let enty = enty.or(entity.get_type());
        match entity.get_children().iter()
            .filter(|r| r.get_kind() == EntityKind::TypeRef)
            .next()
        {
            Some(se) => self.takename(se.clone()),
            None => enty
                .and_then(|r| r.get_element_type())
                .or(enty)
                .map(|r| r.get_kind())
                .map(|r| typeconv(r))
                .unwrap()
        }
    }

    pub fn takeres(&mut self, entity: Entity<'tu>, enty: Option<Type>) -> String {
        let enty = enty.or(entity.get_type());
        match enty
            .and_then(|r| r.get_result_type())
            .and_then(|r| if r.get_kind() == TypeKind::Void { None } else { Some(r) } )
        {
            Some(ty) => format!(
                "{}{}",
                dump_const!(Some(ty)).unwrap_or(""),
                self.taketype(entity, Some(ty))
            ),
            None => String::from("()")
        }
    }
}
