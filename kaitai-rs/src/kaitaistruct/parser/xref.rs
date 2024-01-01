use crate::kaitaistruct::language::meta::{
    ForensicWiki, JustSolve, LocIdentifier, MIMEType, Meta, PronomIdentifier, RFCIdentifier,
    WikiDataIdentifier, ISO,
};
use serde_yaml::Value;
use std::io;

// Macro to simplify the parsing of xref fields and updating the corresponding Meta instance.
macro_rules! parse_xref_field {
    ($meta_instance:expr, $xref_value:expr, $field:ident, $field_type:ty) => {
        if let Some(value) = $xref_value {
            match value {
                Value::String(s) => {
                    // Creating a new instance of the specified field type with a single value.
                    let field_instance = <$field_type>::new(vec![s.clone()])?;
                    // Updating the Meta instance with the parsed field.
                    $meta_instance.xref.$field = Some(field_instance);
                }
                Value::Sequence(values) => {
                    // Extracting values from the sequence, converting them to strings, and collecting them into a vector.
                    let field_values: Vec<String> = values
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect();
                    // Creating a new instance of the specified field type with the collected values.
                    let field_instance = <$field_type>::new(field_values)?;
                    // Updating the Meta instance with the parsed field.
                    $meta_instance.xref.$field = Some(field_instance);
                }
                _ => {
                    // Returning an error if the value is not a String or a Sequence.
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        concat!("Invalid value for '", stringify!($field), "' field in xref section"),
                    ));
                }
            }
        }
    };
}

/// Parses the xref section of the Meta instance.
pub fn parse_xref(meta_instance: &mut Meta, xref_value: &Value) -> Result<(), io::Error> {
    // Checking if the xref_value is a mapping (an associative array).
    if let Value::Mapping(map) = xref_value {
        // Parsing each xref field using the parse_xref_field macro.
        parse_xref_field!(
            meta_instance,
            map.get(&Value::String("forensicswiki".to_string()))
                .cloned(),
            forensic_wiki,
            ForensicWiki
        );
        parse_xref_field!(
            meta_instance,
            map.get(&Value::String("wikidata".to_string())).cloned(),
            wikidata,
            WikiDataIdentifier
        );
        parse_xref_field!(
            meta_instance,
            map.get(&Value::String("iso".to_string()))
                .cloned(),
            iso,
            ISO
        );
        parse_xref_field!(
            meta_instance,
            map.get(&Value::String("justsolve".to_string())).cloned(),
            justsolve,
            JustSolve
        );
        parse_xref_field!(
            meta_instance,
            map.get(&Value::String("mime".to_string())).cloned(),
            mime,
            MIMEType
        );
        parse_xref_field!(
            meta_instance,
            map.get(&Value::String("pronom".to_string())).cloned(),
            pronom,
            PronomIdentifier
        );
        parse_xref_field!(
            meta_instance,
            map.get(&Value::String("loc".to_string())).cloned(),
            loc,
            LocIdentifier
        );
        parse_xref_field!(
            meta_instance,
            map.get(&Value::String("rfc".to_string())).cloned(),
            rfc,
            RFCIdentifier
        );
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid or missing 'xref' field in meta section",
        ));
    }

    Ok(())
}
