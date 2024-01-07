use crate::kaitaistruct::language::kaitai_type::PureType;
use crate::kaitaistruct::language::kaitai_type::Type;
use crate::kaitaistruct::language::params::ParamSpec;
use crate::kaitaistruct::language::params::Params;
use crate::kaitaistruct::parser::doc::parse_doc;
use crate::kaitaistruct::parser::doc_ref::parse_doc_ref;
use crate::kaitaistruct::parser::identifier::parse_identifier;
use serde_yaml::Value;
use std::io;

/// Parses the "params" section of the Kaitai Struct definition.
pub fn parse_params(params_instance: &mut Params, params: &Value) -> Result<(), io::Error> {
    if let Value::Sequence(sequence) = params {
        for mapping in sequence {
            parse_paramspec(params_instance, mapping).unwrap();
        }
    }
    Ok(())
}

/// Parses a ParamSpec instance.
pub fn parse_paramspec(params_instance: &mut Params, param_spec: &Value) -> Result<(), io::Error> {
    let mut param_spec_instance = ParamSpec::new();

    if let Value::Mapping(param_map) = param_spec {
        for (key, value) in param_map {
            if let Value::String(_str_value) = value {
                match key.as_str() {
                    Some("id") => {
                        parse_identifier(&mut param_spec_instance.id, value.as_str().unwrap())
                            .unwrap();
                    }
                    Some("type") => parse_type(&mut param_spec_instance.param_type, value).unwrap(),
                    Some("doc") => parse_doc(&mut param_spec_instance.doc, value).unwrap(),
                    Some("doc-ref") => {
                        parse_doc_ref(&mut param_spec_instance.doc_ref, value).unwrap()
                    }

                    // TODO: Find the corresponding enum
                    // Some("enum") => {todo!()}
                    _ => (),
                }
            }
        }
    }

    params_instance.params_spec.push(param_spec_instance);
    Ok(())
}

/// Parses the "type" field of params specs
pub fn parse_type(params_instance: &mut Option<Type>, param_type: &Value) -> Result<(), io::Error> {
    if let Value::String(type_str) = param_type {
        let (base_type, is_array) = if type_str.ends_with("[]") {
            // If the last two characters are "[]", it's an array
            (&type_str[..type_str.len() - 2], true)
        } else {
            // Otherwise, it's a non-array type
            (type_str.as_str(), false)
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
                "struct" => PureType::ArbitraryStruct,
                "io" => PureType::IOStream,
                "any" => PureType::AnyType,
                // TODO : implement a parsing system for user-defined type
                _ => PureType::AnyType, // Placeholder for unknown types
            }
        };

        let type_instance = Type {
            pure_type,
            is_array,
        };

        *params_instance = Some(type_instance);
    }

    Ok(())
}
