use clang::TypeKind;


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
        for _ in 0..$depth { out.push('\t'); };
        out
    }}
}

pub fn typeconv(ty: TypeKind) -> String {
    let r = match ty {
        TypeKind::Void => "c_void",
        TypeKind::Bool => "bool",
        TypeKind::CharS => "c_char",
        TypeKind::CharU => "c_char",
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
