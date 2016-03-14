use clang::TypeKind;
use super::trie::Trie;

pub const TAB: &'static str = "    ";

#[allow(dead_code)]
pub const COMMENT_LONG: usize = 75;


macro_rules! dump_const {
    ( $ety:expr ) => {
        $ety
            .and_then(|r| r.get_pointee_type())
            .map(|r| if r.is_const_qualified() { "*const " } else { "*mut " })
    }
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

macro_rules! dump_tab {
    ( $depth:expr ) => {{
        let mut out = String::new();
        for _ in 0..$depth { out.push_str(super::utils::TAB); };
        out
    }}
}


#[allow(dead_code)]
pub fn split_comment(comment: String, tab: String) -> String {
    // FIXME don't split word, don't split code
    comment.into_bytes().chunks(COMMENT_LONG)
        .map(|r| format!(
            "{}/// {}\n",
            tab,
            String::from_utf8(r.to_vec()).unwrap()
        ))
        .collect::<Vec<String>>()
        .concat()
}

pub fn typeconv(ty: TypeKind) -> String {
    let r = match ty {
        TypeKind::Void => "c_void",
        TypeKind::Bool => "bool",
        TypeKind::CharS | TypeKind::CharU => "c_char",
        TypeKind::SChar => "c_schar",
        TypeKind::UChar => "c_uchar",
        TypeKind::WChar => "wchat_t",
        TypeKind::Char16 => "uint16_t",
        TypeKind::Char32 => "uint32_t",
        TypeKind::Short => "c_short",
        TypeKind::UShort => "c_ushort",
        TypeKind::Int => "c_int",
        TypeKind::UInt => "c_uint",
        TypeKind::Long => "c_long",
        TypeKind::ULong => "c_ulong",
        TypeKind::LongLong => "c_longlong",
        TypeKind::ULongLong => "c_ulonglong",
        TypeKind::Float => "c_float",
        TypeKind::Double => "c_double",
        TypeKind::LongDouble => panic!("hmm.."),
        TypeKind::Int128 | TypeKind::UInt128 | _ => panic!("Unknown type. {:?}", ty)
    };
    r.into()
}

/// Convert naming style.
///
/// # Example
///
/// ```
/// use ffigen::utils::to_hump;
///
/// assert_eq!(to_hump(String::from("BOOB_NAME")), "BoobName");
/// assert_eq!(to_hump(String::from("boob_name")), "BoobName");
/// ```
#[allow(dead_code)]
pub fn to_hump(name: String) -> String {
    name.split('_')
        .map(|r| format!(
            "{}{}",
            &r[0..1].to_uppercase(),
            &r[1..].to_lowercase()
        ))
        .collect::<Vec<String>>()
        .concat()
}

/// Fetch strings prefix.
///
/// # Example
///
/// ```
/// use ffigen::utils::fetch_prefix;
///
/// assert_eq!(
///     "dump",
///     &fetch_prefix(vec![
///         String::from("dump_continue"),
///         String::from("dump_const"),
///         String::from("dump_tab")
///     ])
/// )
/// ```
pub fn fetch_prefix(strings: Vec<String>) -> String {
    let mut t = Trie::default();
    for s in strings {
        t.insert(s.split('_').map(|r| r.to_owned()).collect());
    }
    t.prefix().join("_")
}


pub fn trim_prefix(name: &str, prefix: &str) -> String {
    name.split('_')
        .skip(prefix.split('_').count())
        .map(|r| r.to_owned())
        .collect::<Vec<String>>()
        .join("_")
}
