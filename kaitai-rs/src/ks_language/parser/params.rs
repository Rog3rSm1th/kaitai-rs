use crate::ks_language::language::kaitai_type::Type;
use crate::ks_language::language::params::ParamSpec;
use crate::ks_language::language::params::Params;
use crate::ks_language::parser::doc::parse_doc;
use crate::ks_language::parser::doc_ref::parse_doc_ref;
use crate::ks_language::parser::identifier::parse_identifier;
use crate::ks_language::parser::kaitai_type::parse_kaitai_type;
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

/// Parses the "type" field of params specs and updates the `params_instance` with the parsed type.
pub fn parse_type(params_instance: &mut Option<Type>, param_type: &Value) -> Result<(), io::Error> {
    if let Value::String(type_str) = param_type {
        let type_instance = parse_kaitai_type(type_str)?;
        *params_instance = Some(type_instance);
    }

    Ok(())
}
