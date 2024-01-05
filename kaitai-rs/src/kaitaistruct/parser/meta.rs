use crate::kaitaistruct::language::meta::EndianEnum;
use crate::kaitaistruct::language::meta::Meta;
use crate::kaitaistruct::parser::identifier::parse_identifier;
use crate::kaitaistruct::parser::xref::parse_xref;
use serde_yaml::Value;
use std::io;

/// Parses the "meta" section
pub fn parse_meta(meta_instance: &mut Meta, meta: &Value) -> Result<(), io::Error> {
    if let Value::Mapping(meta_map) = meta {
        if let Some(id_value) = meta_map.get(&Value::String("id".to_string())) {
            if let Value::String(id_str) = id_value {
                parse_identifier(&mut meta_instance.identifier, id_str).unwrap();
            }
        }

        if let Some(title_value) = meta_map.get(&Value::String("title".to_string())) {
            if let Value::String(title_str) = title_value {
                parse_title(meta_instance, title_str)?;
            }
        }

        if let Some(applications_value) = meta_map.get(&Value::String("application".to_string())) {
            if let Value::Sequence(applications_seq) = applications_value {
                parse_applications(meta_instance, applications_seq)?;
            }
        }

        if let Some(ks_debug_value) = meta_map.get(&Value::String("ks-debug".to_string())) {
            parse_ks_debug(meta_instance, ks_debug_value)?;
        }

        if let Some(ks_opaque_types_value) =
            meta_map.get(&Value::String("ks-opaque-types".to_string()))
        {
            parse_ks_opaque_types(meta_instance, ks_opaque_types_value)?;
        }

        if let Some(license_value) = meta_map.get(&Value::String("license".to_string())) {
            parse_license(meta_instance, license_value)?;
        }

        if let Some(endian_value) = meta_map.get(&Value::String("endian".to_string())) {
            parse_endian(meta_instance, endian_value)?;
        }

        if let Some(imports_value) = meta_map.get(&Value::String("imports".to_string())) {
            parse_imports(meta_instance, imports_value)?;
        }

        if let Some(encoding_value) = meta_map.get(&Value::String("encoding".to_string())) {
            parse_encoding(meta_instance, encoding_value)?;
        }

        if let Some(file_extension_value) =
            meta_map.get(&Value::String("file-extension".to_string()))
        {
            parse_file_extension(meta_instance, file_extension_value)?;
        }

        if let Some(version_value) = meta_map.get(&Value::String("ks-version".to_string())) {
            parse_version(meta_instance, version_value)?;
        }

        if let Some(xref_value) = meta_map.get(&Value::String("xref".to_string())) {
            parse_xref(meta_instance, xref_value)?;
        }

        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Unexpected or missing 'id' field in meta section",
    ))
}

/// Parses the title field within the "meta" section
fn parse_title(meta_instance: &mut Meta, title_str: &str) -> Result<(), io::Error> {
    meta_instance.set_title(title_str.to_string());
    Ok(())
}

/// Parses the applications field within the "meta" section
fn parse_applications(
    meta_instance: &mut Meta,
    applications_seq: &Vec<Value>,
) -> Result<(), io::Error> {
    let values: Vec<String> = applications_seq
        .iter()
        .filter_map(|v| {
            if let Value::String(s) = v {
                Some(s.clone())
            } else {
                None
            }
        })
        .collect();

    meta_instance.set_application(values);
    Ok(())
}

/// Parses the ks_debug field within the "meta" section
fn parse_ks_debug(meta_instance: &mut Meta, ks_debug_value: &Value) -> Result<(), io::Error> {
    if let Value::Bool(ks_debug) = ks_debug_value {
        meta_instance.set_ks_debug(*ks_debug);
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid or missing 'ks-debug' field in meta section",
        ))
    }
}

/// Parses the ks_opaque_types field within the "meta" section
fn parse_ks_opaque_types(
    meta_instance: &mut Meta,
    ks_opaque_types_value: &Value,
) -> Result<(), io::Error> {
    if let Value::Bool(ks_opaque_types) = ks_opaque_types_value {
        meta_instance.set_ks_opaque_types(*ks_opaque_types);
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid or missing 'ks-opaque-types' field in meta section",
        ))
    }
}

/// Parses the license field within the "meta" section
fn parse_license(meta_instance: &mut Meta, license_value: &Value) -> Result<(), io::Error> {
    if let Value::String(license) = license_value {
        meta_instance.set_license(license.clone());
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid or missing 'license' field in meta section",
        ))
    }
}

/// Parses the endian field within the "meta" section
fn parse_endian(meta_instance: &mut Meta, endian_value: &Value) -> Result<(), io::Error> {
    if let Value::String(s) = endian_value {
        match s.to_lowercase().as_str() {
            "le" => {
                meta_instance.set_endian(EndianEnum::Le);
                Ok(())
            }
            "be" => {
                meta_instance.set_endian(EndianEnum::Be);
                Ok(())
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'endian' type in meta section",
            )),
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid or missing 'endian' field in meta section",
        ))
    }
}

/// Parses the imports field within the "meta" section
fn parse_imports(meta_instance: &mut Meta, imports_value: &Value) -> Result<(), io::Error> {
    if let Value::Sequence(imports_seq) = imports_value {
        let values: Vec<String> = imports_seq
            .iter()
            .filter_map(|v| {
                if let Value::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();

        meta_instance.set_imports(values);
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid or missing 'imports' field in meta section",
        ))
    }
}

/// Parses the encoding field within the "meta" section
fn parse_encoding(meta_instance: &mut Meta, encoding_value: &Value) -> Result<(), io::Error> {
    if let Value::String(encoding) = encoding_value {
        meta_instance.set_encoding(encoding.clone());
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid or missing 'encoding' field in meta section",
        ))
    }
}

/// Parses the file_extension field within the "meta" section
fn parse_file_extension(
    meta_instance: &mut Meta,
    file_extension_value: &Value,
) -> Result<(), io::Error> {
    match file_extension_value {
        Value::String(s) => {
            // If the file_extension is a single string, convert it to a Vec<String>
            meta_instance.set_file_extension(vec![s.clone()]);
            Ok(())
        }
        Value::Sequence(file_extension_seq) => {
            // If the file_extension is a sequence of strings, parse each string
            let values: Vec<String> = file_extension_seq
                .iter()
                .filter_map(|v| {
                    if let Value::String(s) = v {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .collect();

            meta_instance.set_file_extension(values);
            Ok(())
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid or missing 'file_extension' field in meta section",
        )),
    }
}

/// Parses the ks-version field within the "meta" section
fn parse_version(meta_instance: &mut Meta, version_value: &Value) -> Result<(), io::Error> {
    if let Some(version_f64) = match version_value {
        Value::Number(version_num) => version_num.as_f64(),
        Value::String(version_str) => version_str.parse::<f64>().ok(),
        _ => None,
    } {
        meta_instance.set_ks_version(version_f64);
        Ok(())
    } else {
        let error_msg = "Invalid or missing 'version' field in meta section";
        eprintln!("{}", error_msg);
        Err(io::Error::new(io::ErrorKind::InvalidData, error_msg))
    }
}
