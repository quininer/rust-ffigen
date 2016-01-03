extern crate clang;

use clang::TypeKind;

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
        TypeKind::Int128 | TypeKind::UInt128 | _ => panic!("Unknown type.")
    };
    r.into()
}
