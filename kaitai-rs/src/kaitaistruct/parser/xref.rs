use crate::kaitaistruct::language::meta::JustSolve;
use crate::kaitaistruct::language::meta::LocIdentifier;
use crate::kaitaistruct::language::meta::MIMEType;
use crate::kaitaistruct::language::meta::PronomIdentifier;
use crate::kaitaistruct::language::meta::RFCIdentifier;
use crate::kaitaistruct::language::meta::WikiDataIdentifier;
use crate::kaitaistruct::language::meta::ISO;
use crate::kaitaistruct::language::meta::{ForensicWiki, Meta};
use serde_yaml::Value;
use std::io;

/// Parses the xref field within the "meta" section
pub fn parse_xref(meta_instance: &mut Meta, xref_value: &Value) -> Result<(), io::Error> {
    // Match and process each section
    match xref_value {
        Value::Mapping(map) => {
            if let Some(forensic_wiki) = map.get(&Value::String("forensicswiki".to_string())) {
                parse_forensic_wiki(meta_instance, forensic_wiki).unwrap();
            }

            if let Some(wikidata) = map.get(&Value::String("wikidata".to_string())) {
                parse_wikidata(meta_instance, wikidata)?;
            }

            if let Some(iso_identifier) = map.get(&Value::String("iso_identifier".to_string())) {
                parse_iso(meta_instance, iso_identifier)?;
            }

            if let Some(justsolve) = map.get(&Value::String("justsolve".to_string())) {
                parse_justsolve(meta_instance, justsolve)?;
            }

            if let Some(mime_type) = map.get(&Value::String("mime".to_string())) {
                parse_mime(meta_instance, mime_type)?;
            }

            if let Some(pronom_identifier) = map.get(&Value::String("pronom".to_string())) {
                parse_pronom(meta_instance, pronom_identifier)?;
            }

            if let Some(loc_identifier) = map.get(&Value::String("loc".to_string())) {
                parse_loc(meta_instance, loc_identifier)?;
            }

            if let Some(rfc) = map.get(&Value::String("rfc".to_string())) {
                parse_rfc(meta_instance, rfc)?;
            }
        }
        _ => {
            // Handle unexpected YAML structure
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid or missing 'xref' field in meta section",
            ));
        }
    }

    Ok(())
}

/// Parses the "forensic-wiki" field within the "xref" section
fn parse_forensic_wiki(
    meta_instance: &mut Meta,
    forensic_wiki_value: &Value,
) -> Result<(), io::Error> {
    match forensic_wiki_value {
        Value::String(s) => {
            // If it's a string, create a ForensicWiki instance with a single value
            let forensic_wiki_instance = ForensicWiki::new(vec![s.clone()])?;

            // Set the ForensicWiki field in the XRef instance
            meta_instance.xref.forensic_wiki = Some(forensic_wiki_instance);
        }
        Value::Sequence(values) => {
            // If it's a sequence, parse and create a ForensicWiki instance
            let forensic_wiki_values: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();

            let forensic_wiki_instance = ForensicWiki::new(forensic_wiki_values)?;

            // Set the ForensicWiki field in the XRef instance
            meta_instance.xref.set_forensic_wiki(forensic_wiki_instance);
        }
        _ => {
            // Handle unexpected YAML structure
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid or missing 'forensic-wiki' field in xref section",
            ));
        }
    }

    Ok(())
}

/// Parses the "wikidata" field within the "xref" section
fn parse_wikidata(meta_instance: &mut Meta, wikidata_value: &Value) -> Result<(), io::Error> {
    match wikidata_value {
        Value::String(s) => {
            // If it's a string, create a WikiDataIdentifier instance with a single value
            let wikidata_instance = WikiDataIdentifier::new(vec![s.clone()])?;

            // Set the WikiData field in the XRef instance
            meta_instance.xref.set_wikidata(wikidata_instance);
        }
        Value::Sequence(values) => {
            // If it's a sequence, parse and create a WikiDataIdentifier instance
            let wikidata_values: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();

            let wikidata_instance = WikiDataIdentifier::new(wikidata_values)?;

            // Set the WikiData field in the XRef instance
            meta_instance.xref.set_wikidata(wikidata_instance);
        }
        _ => {
            // Handle unexpected YAML structure
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid or missing 'wikidata' field in xref section",
            ));
        }
    }

    Ok(())
}

/// Parses the "iso" field within the "xref" section
fn parse_iso(meta_instance: &mut Meta, iso_identifier_value: &Value) -> Result<(), io::Error> {
    match iso_identifier_value {
        Value::String(s) => {
            // If it's a string, create an ISO instance with a single value and set it in the XRef instance
            let iso_instance = ISO::new(vec![s.clone()])?;
            meta_instance.xref.set_iso(iso_instance);
        }
        Value::Sequence(values) => {
            // If it's a sequence, parse and create an ISO instance
            let iso_identifier_values: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();

            let iso_instance = ISO::new(iso_identifier_values)?;
            meta_instance.xref.set_iso(iso_instance);
        }
        _ => {
            // Handle unexpected YAML structure
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'iso_identifier' field in xref section",
            ));
        }
    }

    Ok(())
}

/// Parses the "justsolve" field within the "xref" section
fn parse_justsolve(meta_instance: &mut Meta, justsolve_value: &Value) -> Result<(), io::Error> {
    match justsolve_value {
        Value::String(s) => {
            // If it's a string, create a JustSolve instance with a single value and set it in the XRef instance
            let justsolve_instance = JustSolve::new(vec![s.clone()])?;
            meta_instance.xref.justsolve = Some(justsolve_instance);
        }
        Value::Sequence(values) => {
            // If it's a sequence, parse and create a JustSolve instance
            let justsolve_values: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();

            let justsolve_instance = JustSolve::new(justsolve_values)?;
            meta_instance.xref.justsolve = Some(justsolve_instance);
        }
        _ => {
            // Handle unexpected YAML structure
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'justsolve' field in xref section",
            ));
        }
    }

    Ok(())
}

/// Parses the "mime_type" field within the "xref" section
fn parse_mime(meta_instance: &mut Meta, mime_type_value: &Value) -> Result<(), io::Error> {
    match mime_type_value {
        Value::String(s) => {
            // If it's a string, create a MIMEType instance with a single value
            let mime_type_instance = MIMEType::new(vec![s.clone()])?;

            // Set the MIMEType field in the XRef instance
            meta_instance.xref.set_mime(mime_type_instance);
        }
        Value::Sequence(values) => {
            // If it's a sequence, parse and create a MIMEType instance
            let mime_type_values: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();

            let mime_type_instance = MIMEType::new(mime_type_values)?;

            // Set the MIMEType field in the XRef instance
            meta_instance.xref.set_mime(mime_type_instance);
        }
        _ => {
            // Handle unexpected YAML structure
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid or missing 'mime_type' field in xref section",
            ));
        }
    }

    Ok(())
}

/// Parses the "pronom" field within the "xref" section
fn parse_pronom(
    meta_instance: &mut Meta,
    pronom_identifier_value: &Value,
) -> Result<(), io::Error> {
    match pronom_identifier_value {
        Value::String(s) => {
            // If it's a string, create a PronomIdentifier instance with a single value
            let pronom_identifier_instance = PronomIdentifier::new(vec![s.clone()])?;

            // Set the PronomIdentifier field in the XRef instance
            meta_instance.xref.set_pronom(pronom_identifier_instance);
        }
        Value::Sequence(values) => {
            // If it's a sequence, parse and create a PronomIdentifier instance
            let pronom_identifier_values: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();

            let pronom_identifier_instance = PronomIdentifier::new(pronom_identifier_values)?;

            // Set the PronomIdentifier field in the XRef instance
            meta_instance.xref.set_pronom(pronom_identifier_instance);
        }
        _ => {
            // Handle unexpected YAML structure
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid or missing 'pronom_identifier' field in xref section",
            ));
        }
    }

    Ok(())
}

/// Parses the "loc" field within the "xref" section
fn parse_loc(meta_instance: &mut Meta, loc_identifier_value: &Value) -> Result<(), io::Error> {
    match loc_identifier_value {
        Value::String(s) => {
            // If it's a string, create a LocIdentifier instance with a single value
            let loc_identifier_instance = LocIdentifier::new(vec![s.clone()])?;

            // Set the LocIdentifier field in the XRef instance
            meta_instance.xref.set_loc(loc_identifier_instance);
        }
        Value::Sequence(values) => {
            // If it's a sequence, parse and create a LocIdentifier instance
            let loc_identifier_values: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();

            let loc_identifier_instance = LocIdentifier::new(loc_identifier_values)?;

            // Set the LocIdentifier field in the XRef instance
            meta_instance.xref.set_loc(loc_identifier_instance);
        }
        _ => {
            // Handle unexpected YAML structure
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid or missing 'loc_identifier' field in xref section",
            ));
        }
    }

    Ok(())
}

/// Parses the "rfc" field within the "xref" section
fn parse_rfc(meta_instance: &mut Meta, rfc_value: &Value) -> Result<(), io::Error> {
    match rfc_value {
        Value::String(s) => {
            // If it's a string, create an RFCIdentifier instance with a single value
            let rfc_instance = RFCIdentifier::new(vec![s.clone()])?;

            // Set the RFC field in the XRef instance
            meta_instance.xref.set_rfc(rfc_instance);
        }
        Value::Sequence(values) => {
            // If it's a sequence, parse and create an RFCIdentifier instance
            let rfc_values: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();

            let rfc_instance = RFCIdentifier::new(rfc_values)?;

            // Set the RFC field in the XRef instance
            meta_instance.xref.set_rfc(rfc_instance);
        }
        _ => {
            // Handle unexpected YAML structure
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid or missing 'rfc' field in xref section",
            ));
        }
    }

    Ok(())
}
