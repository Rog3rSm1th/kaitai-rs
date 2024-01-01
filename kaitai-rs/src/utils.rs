use regex::Regex;
use std::io;

// Generic function for regex validation
pub fn validate_values(values: &[String], pattern: &str) -> Result<(), io::Error> {
    let regex = Regex::new(pattern).map_err(|e| {
        let err_msg = format!("Regex error: {}", e);
        io::Error::new(io::ErrorKind::InvalidData, err_msg)
    })?;

    for value in values {
        if !regex.is_match(value) {
            let err_msg = format!("Validation error for value: {}", value);
            return Err(io::Error::new(io::ErrorKind::InvalidData, err_msg));
        }
    }

    Ok(())
}
