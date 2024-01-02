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
    let mut paramspec = ParamSpec::new();

    if let Value::Mapping(map) = param_spec {
        for (key, value) in map {
            if let Value::String(_str_value) = value {
                match key.as_str() {
                    Some("id") => {
                        let paramspec_identifier = parse_identifier(value.as_str().unwrap())?;
                        paramspec.set_identifier(paramspec_identifier);
                    }

                    // TODO : Write the type parser
                    // Some("type") => {todo!()},
                    Some("doc") => parse_doc(&mut paramspec.doc, value).unwrap(),
                    Some("doc-ref") => parse_doc_ref(&mut paramspec.doc_ref, value).unwrap(),

                    // TODO : Find the corresponding enum
                    // Some("enum") => {todo!()}
                    _ => (),
                }
            }
        }
    }

    params_instance.params_spec.push(paramspec);
    Ok(())
}
