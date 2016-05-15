extern crate clang;

#[macro_use] pub mod utils;
mod trie;
mod gen;
mod ast;

use std::env::var;
use std::path::{ Path, PathBuf };
use clang::{ Clang, Index, ParseOptions, TranslationUnit };
use gen::{ UnnamedMap, KeywordSet, Status };
use ast::rust_dump;


macro_rules! set {
    ( $( $e:expr ),* ) => {{
        let mut tmp_set = KeywordSet::new();
        $(
            tmp_set.insert(String::from($e));
        )*
        tmp_set
    }}
}


#[derive(Debug, Clone)]
pub struct GenOptions<'g> {
    pub args: Vec<&'g str>,
    pub headers: Vec<String>,
    pub link: String,
    pub parse: ParseOptions,
    pub optcomment: bool,
    pub optformat: bool
}

impl<'g> Default for GenOptions<'g> {
    fn default() -> GenOptions<'g> {
        GenOptions {
            args: Vec::new(),
            headers: Vec::new(),
            link: String::new(),
            parse: ParseOptions::default(),
            optcomment: false,
            optformat: true
        }
    }
}

impl<'g> GenOptions<'g> {
    pub fn arg(mut self, a: &'g str) -> GenOptions<'g> {
        self.args.push(a);
        self
    }
    pub fn header(mut self, l: &'g str) -> GenOptions<'g> {
        self.headers.push(String::from(l));
        self
    }
    pub fn link(mut self, m: &'g str) -> GenOptions<'g> {
        self.link = m.into();
        self
    }
    pub fn comment(mut self, enable: bool) -> GenOptions<'g> {
        self.optcomment = enable;
        self
    }
    pub fn format(mut self, enable: bool) -> GenOptions<'g> {
        self.optformat = enable;
        self
    }

    pub fn gen(self) -> Vec<u8> {
        let c = Clang::new().expect("clang init error.");
        let i = Index::new(&c, true, false);
        let t = TranslationUnit::from_source(
            &i,
            &self.headers[0],
            &self.args[..],
            &[],
            self.parse
        ).expect("TranslationUnit init error.");
        let mut kwset = set![
            "abstract", "alignof", "as", "become", "box",
            "break", "const", "continue", "crate", "do",
            "else", "enum", "extern", "false", "final",
            "fn", "for", "if", "impl", "in",
            "let", "loop", "macro", "match", "mod",
            "move", "mut", "offsetof", "override", "priv",
            "proc", "pub", "pure", "ref", "return",
            "Self", "self", "sizeof", "static", "struct",
            "super", "trait", "true", "type", "typeof",
            "unsafe", "unsized", "use", "virtual", "where",
            "while", "yield",

            // other
            "try", "catch"
        ];

        let entity = t.get_entity();
        let mut status = Status {
            unmap: &mut UnnamedMap::new(),
            kwset: &mut kwset,
            headers: self.headers,
            link: self.link,
            dump: None,
            optcomment: self.optcomment,
            optformat: self.optformat
        };

        rust_dump(&entity, &mut status).into_bytes()
    }
}

pub fn find_clang_include_path() -> PathBuf {
    let env_path = var("CLANG_PATH")
        .unwrap_or("clang".into());

    let clang_path = [
        Path::new(&env_path),
        Path::new("/usr/lib/clang"),
        Path::new("/usr/local/lib/clang")
    ].iter()
        .cloned()
        .find(|p| p.is_dir())
        .unwrap();

    clang_path.read_dir().ok()
        .and_then(|r| r.last())
        .and_then(|r| r.ok())
        .unwrap()
        .path()
        .join("include")
}

pub fn find_include_header<P: AsRef<Path>>(header: P) -> PathBuf {
    let header = header.as_ref();
    let env_path = var("INCLUDE_PATH")
        .unwrap_or(".".into());

    [
        Path::new(&env_path),
        Path::new("/usr/include"),
        Path::new("/usr/local/include")
    ].iter()
        .cloned()
        .map(|p| p.join(header))
        .find(|p| p.is_file())
        .unwrap()
}

#[macro_export]
macro_rules! gen {
    ( $l:expr, [ $( $h:expr ),* ] ) => {
        $crate::GenOptions::default()
            .arg(&format!(
                "-I{}",
                $crate::find_clang_include_path().to_string_lossy()
            ))
        $(
            .header(&$crate::find_include_header($h).to_string_lossy())
        )*
            .link($l)
            .gen()
    };
    ( $l:expr, [ $( $h:expr ),* ] -> $o:expr ) => {{
        use std::fs::File;
        use std::io::Write;
        File::create($o).unwrap()
            .write(&gen!($l, [ $( $h ),* ])).unwrap()
    }}
}
