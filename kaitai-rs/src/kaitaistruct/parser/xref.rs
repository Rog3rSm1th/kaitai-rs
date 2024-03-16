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
                    let field_instance = <$field_type>::new(vec![s.clone()])?;
                    $meta_instance.xref.$field = Some(field_instance);
                }
                Value::Sequence(values) => {
                    let field_values: Vec<String> = values
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect();
                    let field_instance = <$field_type>::new(field_values)?;
                    $meta_instance.xref.$field = Some(field_instance);
                }
                // For the RFC field 
                Value::Number(n) => {
                    let field_value = n.to_string();
                    let field_instance = <$field_type>::new(vec![field_value.clone()])?;
                    $meta_instance.xref.$field = Some(field_instance);
                }
                _ => {
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
            map.get(&Value::String("iso".to_string())).cloned(),
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
