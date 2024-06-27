use crate::ks_language::language::kaitai_type::PureType;
use crate::ks_language::language::kaitai_type::Type;
use std::io;

/// Parses a Kaitai type from a string representation.
pub fn parse_kaitai_type(type_str: &str) -> Result<Type, io::Error> {
    let (base_type, is_array) = if type_str.ends_with("[]") {
        // If the last two characters are "[]", it's an array
        (&type_str[..type_str.len() - 2], true)
    } else {
        // Otherwise, it's a non-array type
        (type_str, false)
    };

    let pure_type = if base_type.starts_with("bx") && base_type.len() > 2 {
        // If it starts with "bx" and has a size greater than 1
        let size: u8 = base_type[2..].parse().unwrap_or(0);
        if size != 1 {
            PureType::BitSizedInteger(size)
        } else {
            PureType::AnyType // Placeholder for unexpected cases
        }
    } else {
        match base_type {
            "u1" => PureType::UnsignedInteger(1),
            "u2" => PureType::UnsignedInteger(2),
            "u4" => PureType::UnsignedInteger(4),
            "u8" => PureType::UnsignedInteger(8),
            "s1" => PureType::SignedInteger(1),
            "s2" => PureType::SignedInteger(2),
            "s4" => PureType::SignedInteger(4),
            "s8" => PureType::SignedInteger(8),
            "f4" => PureType::FloatingPoint(4),
            "f8" => PureType::FloatingPoint(8),
            "bool" | "b1" => PureType::Boolean,
            "str" => PureType::String,
            "strz" => PureType::StringZ,
            "struct" => PureType::ArbitraryStruct,
            "io" => PureType::IOStream,
            "any" => PureType::AnyType,
            // TODO : implement a parsing system for user-defined type
            _ => PureType::AnyType, // Placeholder for unknown types
        }
    };

    Ok(Type {
        pure_type,
        is_array,
    })
}
