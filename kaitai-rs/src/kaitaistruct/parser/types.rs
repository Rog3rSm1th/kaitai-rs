use crate::kaitaistruct::language::doc::Doc;
use crate::kaitaistruct::language::doc_ref::DocRef;
use crate::kaitaistruct::language::enums::Enums;
use crate::kaitaistruct::language::identifier::Identifier;
use crate::kaitaistruct::language::meta::Meta;
use crate::kaitaistruct::language::params::Params;
use crate::kaitaistruct::language::seq::Seq;
use crate::kaitaistruct::language::types::TypeSpec;
use crate::kaitaistruct::language::types::Types;
use crate::kaitaistruct::parser::doc::parse_doc;
use crate::kaitaistruct::parser::doc_ref::parse_doc_ref;
use crate::kaitaistruct::parser::meta::parse_meta;
use crate::kaitaistruct::parser::params::parse_params;
use serde_yaml::Value;
use std::io;

/// Parses the "types" section of the Kaitai Struct definition.
pub fn parse_types(types_instance: &mut Types, types_section: &Value) -> Result<(), io::Error> {
    match types_section {
        Value::Mapping(types_map) => {
            // Iterate over each entry in the types map and parse individual types
            for (type_name, type_values) in types_map {
                parse_typespec(types_instance, &type_name, &type_values)?;
            }
            Ok(())
        }
        // If "types" section is not a mapping, return an error
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unexpected YAML structure in types section",
        )),
    }
}

/// Parses an individual TypeSpec within the "types" section of the Kaitai Struct definition.
fn parse_typespec(
    types_instance: &mut Types,
    typespec_name: &Value,
    typespec_values: &Value,
) -> Result<(), io::Error> {
    // If "type_values" is a mapping, proceed with parsing
    if let Value::Mapping(variant_map) = typespec_values {
        // Create an Identifier for the TypeSpec
        let mut typespec_identifier = Identifier::new();

        typespec_identifier
            .from_string_vec(vec![typespec_name.as_str().unwrap().to_string()])
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        // Create an TypeSpec instance to store parsed variant values
        let mut typespec_instance = TypeSpec::new(
            Meta::new(),
            Params::new(),
            Seq::new(),
            Types::new(),
            Enums::new(),
            String::new(),
            Doc::new(),
            DocRef::new(),
        );

        for (key, value) in variant_map {
            if let Value::String(_str_value) = value {
                match key.as_str() {
                    Some("meta") => parse_meta(&mut typespec_instance.meta, value).unwrap(),
                    Some("types") => parse_types(&mut typespec_instance.type_types, value).unwrap(),
                    Some("params") => parse_params(&mut typespec_instance.params, value).unwrap(),
                    Some("doc") => parse_doc(&mut typespec_instance.doc, value).unwrap(),
                    Some("doc-ref") => {
                        parse_doc_ref(&mut typespec_instance.doc_ref, value).unwrap()
                    }

                    // TODO: Parse the enum / seq / instances fields
                    // Some("enum") => {todo!()}
                    // Some("seq") => {todo!()}
                    // Some("instances") => {todo!()}
                    _ => (),
                }
            }
        }

        types_instance.add_typespec(typespec_identifier, typespec_instance)?;

        Ok(())
    } else {
        // If "type_values" is not a mapping, return an error
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unexpected YAML structure in type section",
        ))
    }
}
