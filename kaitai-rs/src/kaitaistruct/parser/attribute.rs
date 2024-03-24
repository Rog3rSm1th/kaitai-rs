use crate::kaitaistruct::language::attribute::Attribute;
use crate::kaitaistruct::language::attribute::Process;
use crate::kaitaistruct::language::attribute::ProcessType;
use crate::kaitaistruct::language::attribute::Repeat;
use crate::kaitaistruct::language::doc::Doc;
use crate::kaitaistruct::language::doc_ref::DocRef;
use crate::kaitaistruct::language::identifier::Identifier;
use crate::kaitaistruct::parser::doc::parse_doc;
use crate::kaitaistruct::parser::doc_ref::parse_doc_ref;
use crate::kaitaistruct::parser::identifier::parse_identifier;
use crate::kaitaistruct::parser::kaitai_type::parse_kaitai_type;
use serde_yaml::Value;
use std::io;

// Parses an attribute from a Kaitai Struct definition and returns it
pub fn parse_attribute(attribute: &Value) -> Result<Attribute, io::Error> {
    // Check if the "id" field exists before parsing it
    let identifier = if let Some(id_value) = attribute.get("id").and_then(|value| value.as_str()) {
        let mut identifier = Identifier::new();
        parse_identifier(&mut identifier, id_value)?;

        Some(identifier)
    } else {
        None
    };

    // Check if the "doc" field exists and parse it if it does
    let doc = if let Some(doc_value) = attribute.get("doc") {
        let mut doc = Doc::new();
        parse_doc(&mut doc, doc_value)?;
        Some(doc)
    } else {
        None
    };

    // Check if the "doc_ref" field exists and parse it if it does
    let doc_ref = if let Some(doc_ref_value) = attribute.get("doc_ref") {
        let mut doc_ref = DocRef::new();
        parse_doc_ref(&mut doc_ref, doc_ref_value)?;
        Some(doc_ref)
    } else {
        None
    };

    // Check if the "seq_type" field exists and parse it if it does
    let seq_type = if let Some(seq_type_value) = attribute.get("type") {
        let seq_type_str = seq_type_value.as_str().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'seq_type' field. Expected a string.",
            )
        })?;
        Some(parse_kaitai_type(seq_type_str)?)
    } else {
        None
    };

    // Create a new Attribute instance with the parsed values
    let mut new_attribute = Attribute::new(
        identifier, doc, doc_ref, None, seq_type, None, None, None, None, None, None, None, None,
        None, None, None, true, false, true, None, None, None,
    );

    // Define a macro to simplify parsing each attribute field
    macro_rules! parse_attribute_field {
        ($attr:expr, $field_name:literal, $func:ident) => {
            if let Some(value) = $attr.get($field_name.replace("-", "_")) {
                $func(&mut new_attribute, value)?;
            }
        };
    }

    // Use the macro to parse each attribute field
    parse_attribute_field!(attribute, "contents", parse_contents);
    parse_attribute_field!(attribute, "repeat", parse_repeat);
    parse_attribute_field!(attribute, "repeat-expr", parse_repeat_expr);
    parse_attribute_field!(attribute, "repeat-until", parse_repeat_until);
    parse_attribute_field!(attribute, "if", parse_optional_if);
    parse_attribute_field!(attribute, "size", parse_size);
    parse_attribute_field!(attribute, "size-eos", parse_size_eos);
    parse_attribute_field!(attribute, "process", parse_process);
    // TODO: Parse the "enum" field
    parse_attribute_field!(attribute, "encoding", parse_encoding);
    parse_attribute_field!(attribute, "pad-right", parse_pad_right);
    parse_attribute_field!(attribute, "terminator", parse_terminator);
    parse_attribute_field!(attribute, "consume", parse_consume);
    parse_attribute_field!(attribute, "include", parse_include);
    parse_attribute_field!(attribute, "eos-error", parse_eos_error);
    parse_attribute_field!(attribute, "io", parse_io);
    parse_attribute_field!(attribute, "value", parse_value);

    Ok(new_attribute)
}

// Parses the "contents" attribute of an Attribute instance from the provided Value
pub fn parse_contents(
    attribute_instance: &mut Attribute,
    contents_value: &Value,
) -> Result<(), io::Error> {
    let contents = match contents_value {
        Value::Sequence(seq) => {
            let mut contents_vec = Vec::new();
            for value in seq {
                contents_vec.push(value.clone());
            }
            Some(contents_vec)
        }
        _ => None,
    };

    if let Some(contents) = contents {
        attribute_instance.set_contents(contents);
    }

    Ok(())
}

// Parses the "repeat" attribute of an Attribute instance from the provided Value
pub fn parse_repeat(
    attribute_instance: &mut Attribute,
    repeat_value: &Value,
) -> Result<(), io::Error> {
    let repeat = match repeat_value.as_str() {
        Some("eos") => Some(Repeat::Eos),
        Some("expr") => Some(Repeat::Expr),
        Some("until") => Some(Repeat::Until),
        _ => None,
    };

    attribute_instance.set_repeat(repeat.unwrap());
    Ok(())
}

// Parses the "repeat-expr" attribute of an Attribute instance from the provided Value
pub fn parse_repeat_expr(
    attribute_instance: &mut Attribute,
    repeat_expr_value: &Value,
) -> Result<(), io::Error> {
    let repeat_expr = match repeat_expr_value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n
            .as_u64()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid 'repeat-expr' field. Expected a u32 integer.",
                )
            })?
            .to_string(),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'repeat-expr' field. Expected a string or a u32 integer.",
            ));
        }
    };

    attribute_instance.set_repeat_expr(repeat_expr);
    Ok(())
}

// Parses the "repeat-until" attribute of an Attribute instance from the provided Value
pub fn parse_repeat_until(
    attribute_instance: &mut Attribute,
    repeat_until_value: &Value,
) -> Result<(), io::Error> {
    let repeat_until = match repeat_until_value {
        Value::Bool(b) => b.to_string(),
        Value::String(s) => s.clone(),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'repeat-until' field. Expected a boolean or a string.",
            ));
        }
    };

    attribute_instance.set_repeat_until(repeat_until);
    Ok(())
}

// Parses the "if" attribute of an Attribute instance from the provided Value
pub fn parse_optional_if(
    attribute_instance: &mut Attribute,
    optional_if_value: &Value,
) -> Result<(), io::Error> {
    let optional_if = match optional_if_value {
        Value::Bool(b) => b.to_string(),
        Value::String(s) => s.clone(),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'if' field. Expected a boolean or a string.",
            ));
        }
    };

    attribute_instance.set_optional_if(optional_if);
    Ok(())
}

// Parses the "size" attribute of an Attribute instance from the provided Value
pub fn parse_size(attribute_instance: &mut Attribute, size_value: &Value) -> Result<(), io::Error> {
    let size = match size_value {
        Value::Number(n) => n
            .as_u64()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid 'size' field. Expected a u32 integer.",
                )
            })?
            .to_string(),
        Value::String(s) => s.clone(),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'size' field. Expected a string or a u32 integer.",
            ));
        }
    };

    attribute_instance.set_size(size);
    Ok(())
}

// Parses the "size-eos" attribute of an Attribute instance from the provided Value
pub fn parse_size_eos(
    attribute_instance: &mut Attribute,
    size_eos_value: &Value,
) -> Result<(), io::Error> {
    let size_eos = match size_eos_value {
        Value::Bool(b) => b,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'size-eos' field. Expected a boolean.",
            ));
        }
    };

    attribute_instance.set_size_eos(*size_eos);
    Ok(())
}

// Parses the "process" attribute of an Attribute instance from the provided Value
pub fn parse_process(
    attribute_instance: &mut Attribute,
    process_value: &Value,
) -> Result<(), io::Error> {
    let process_type = match process_value.get("type").and_then(|value| value.as_str()) {
        Some("zlib") => ProcessType::Zlib,
        Some("xor") => ProcessType::Xor,
        Some("rol") => ProcessType::Rol,
        Some("ror") => ProcessType::Ror,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'process' field. Expected an object with a 'type' field specifying the processing type.",
            ))
        }
    };

    let process_parameter = match process_value.get("parameter").and_then(|value| value.as_str()) {
        Some(parameter) => parameter.to_string(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'process' field. Expected an object with a 'parameter' field specifying the processing parameter.",
            ))
        }
    };

    let process = Process {
        process_type,
        parameter: process_parameter,
    };

    attribute_instance.set_process(process);
    Ok(())
}

// Parses the "encoding" attribute of an Attribute instance from the provided Value
pub fn parse_encoding(
    attribute_instance: &mut Attribute,
    encoding_value: &Value,
) -> Result<(), io::Error> {
    let encoding_name = match encoding_value.as_str() {
        Some(name) => name,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'encoding' field. Expected a string.",
            ))
        }
    };

    attribute_instance.set_encoding(encoding_name.to_string());
    Ok(())
}

// Parses the "pad-right" attribute of an Attribute instance from the provided Value
pub fn parse_pad_right(
    attribute_instance: &mut Attribute,
    pad_right_value: &Value,
) -> Result<(), io::Error> {
    let pad_right = match pad_right_value.as_u64() {
        Some(n) => n as u8,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'pad-right' field. Expected an integer.",
            ))
        }
    };

    attribute_instance.set_pad_right(pad_right);
    Ok(())
}

// Parses the "terminator" attribute of an Attribute instance from the provided Value
pub fn parse_terminator(
    attribute_instance: &mut Attribute,
    terminator_value: &Value,
) -> Result<(), io::Error> {
    let terminator = match terminator_value.as_u64() {
        Some(n) => n as u8,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'terminator' field. Expected an integer.",
            ))
        }
    };

    attribute_instance.set_terminator(terminator);
    Ok(())
}

// Parses the "consume" attribute of an Attribute instance from the provided Value
pub fn parse_consume(
    attribute_instance: &mut Attribute,
    consume_value: &Value,
) -> Result<(), io::Error> {
    let consume = match consume_value.as_bool() {
        Some(b) => b,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'consume' field. Expected a boolean.",
            ))
        }
    };

    attribute_instance.set_consume(consume);
    Ok(())
}

// Parses the "include" attribute of an Attribute instance from the provided Value
pub fn parse_include(
    attribute_instance: &mut Attribute,
    include_value: &Value,
) -> Result<(), io::Error> {
    let include = match include_value.as_bool() {
        Some(b) => b,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'include' field. Expected a boolean.",
            ))
        }
    };

    attribute_instance.set_include(include);
    Ok(())
}

// Parses the "eos-error" attribute of an Attribute instance from the provided Value
pub fn parse_eos_error(
    attribute_instance: &mut Attribute,
    eos_error_value: &Value,
) -> Result<(), io::Error> {
    let eos_error = match eos_error_value.as_bool() {
        Some(b) => b,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'eos-error' field. Expected a boolean.",
            ))
        }
    };

    attribute_instance.set_eos_error(eos_error);
    Ok(())
}

// Parses the "pos" attribute of an Attribute instance from the provided Value
pub fn parse_pos(attribute_instance: &mut Attribute, pos_value: &Value) -> Result<(), io::Error> {
    let pos = match pos_value {
        Value::Number(n) => n
            .as_u64()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid 'pos' field. Expected a u32 integer.",
                )
            })?
            .to_string(),
        Value::String(s) => s.clone(),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'pos' field. Expected a string or a u32 integer.",
            ));
        }
    };

    attribute_instance.set_pos(pos);
    Ok(())
}

// Parses the "io" attribute of an Attribute instance from the provided Value
pub fn parse_io(attribute_instance: &mut Attribute, io_value: &Value) -> Result<(), io::Error> {
    let io = match io_value.as_str() {
        Some(s) => s.to_string(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 'io' field. Expected a string.",
            ))
        }
    };

    attribute_instance.set_io(io);
    Ok(())
}

// Parses the "value" attribute of an Attribute instance from the provided Value
pub fn parse_value(
    attribute_instance: &mut Attribute,
    value_value: &Value,
) -> Result<(), io::Error> {
    // Attempt to parse value as an integer
    if let Some(int_value) = value_value.as_i64() {
        attribute_instance.set_value(int_value.to_string());
        return Ok(());
    }

    // If parsing as an integer fails, treat it as a string
    if let Some(str_value) = value_value.as_str() {
        attribute_instance.set_value(str_value.to_string());
        return Ok(());
    }

    // If value is neither an integer nor a string, return an error
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Invalid 'value' field. Expected an integer or a string.",
    ))
}
