//! fork from https://github.com/jmtuley/rust-trie

use std::collections::hash_map::HashMap;
use std::hash::Hash;


#[derive(Debug, Clone)]
pub struct Trie<V> where V: Eq+Hash+Clone {
    value: Option<V>,
    children: HashMap<V, Trie<V>>,
}

impl<V> Trie<V> where V: Eq+Hash+Clone {
    pub fn new() -> Trie<V> {
        Trie {
            value: None,
            children: HashMap::new(),
        }
    }
    pub fn set(mut self, v: V) -> Trie<V> {
        self.value = Some(v);
        self
    }

    pub fn insert(&mut self, path: Vec<V>) {
        if !path.is_empty() {
            self.children
                .entry(path[0].clone())
                .or_insert(Trie::new().set(path[0].clone()))
                .insert(path[1..].to_vec())
        };
    }

    pub fn prefix(&self) -> Vec<V> {
        let mut out = Vec::new();

        if let Some(ref v) = self.value {
            out.push(v.clone())
        };

        if self.children.len() == 1 {
            out.append(
                &mut self.children
                    .values()
                    .next().unwrap()
                    .prefix()
            )
        };

        out
    }
}


#[test]
fn prefix_works() {
    let mut t = Trie::new();
    t.insert(vec![1]);
    let p = t.prefix();
    assert_eq!(p, vec![1]);

    let mut t = Trie::new();
    t.insert("dump_continue".bytes().collect());
    t.insert("dump_tab".bytes().collect());
    let p = t.prefix();
    assert_eq!(p, "dump_".bytes().collect::<Vec<u8>>());

    let mut t = Trie::new();
    t.insert("x_dump_continue".bytes().collect());
    t.insert("dump_tab".bytes().collect());
    let p = t.prefix();
    assert_eq!(p, Vec::new());
}
