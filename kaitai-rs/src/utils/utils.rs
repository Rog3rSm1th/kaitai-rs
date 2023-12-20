use crate::errors::KaitaiError;
use regex::Regex;

// Generic function for regex validation
pub fn validate_values(
    values: &[String],
    pattern: &str,
    error: KaitaiError,
) -> Result<(), KaitaiError> {
    let regex = Regex::new(pattern).map_err(|_| error.clone())?;

    for value in values {
        if !regex.is_match(value) {
            return Err(error.clone());
        }
    }

    Ok(())
}
