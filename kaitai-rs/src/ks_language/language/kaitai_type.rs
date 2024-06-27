// Enumeration representing all existing types
#[derive(Debug)]
pub enum PureType {
    UnsignedInteger(u8),
    SignedInteger(u8),
    BitSizedInteger(u8),
    FloatingPoint(u8),
    ByteArray,
    String,
    StringZ,
    Boolean,
    UserType(String), // Placeholder for user-defined type
    ArbitraryStruct,
    IOStream,
    AnyType,
}

// Type structure
#[derive(Debug)]
pub struct Type {
    // The pure type associated with the Type instance
    pub pure_type: PureType,
    // Indicates whether the type is an array
    pub is_array: bool,
}

/// Parses an unsigned integer of the given size and returns a Vec<u8> of the specified size
pub fn parse_unsigned_integer(data: &[u8], size: usize) -> Vec<u8> {
    let mut value = 0;
    for i in 0..size {
        value |= (data[i] as u64) << (i * 8);
    }
    let mut result = vec![0; size];
    result.copy_from_slice(&value.to_le_bytes()[..size]);
    result
}

/// Parses a null-terminated string (StringZ) from the given data slice
/// Returns the parsed string as a Vec<u8>
/// TODO : Add a terminator parameter
pub fn parse_strz(data: &[u8], size: Option<usize>) -> Vec<u8> {
    let mut end_pos = data.len(); // Default to the entire length of the data

    // Check for the specified size and adjust end_pos if it is smaller
    if let Some(specified_size) = size {
        if specified_size < end_pos {
            end_pos = specified_size;
        }
    }

    // Check for the terminator and adjust end_pos if it is found earlier
    if let Some(null_terminator_pos) = data.iter().position(|&x| x == 0) {
        if null_terminator_pos < end_pos {
            end_pos = null_terminator_pos + 1; // Include the terminator
        }
    }

    // Return the string up to end_pos
    data[..end_pos].to_vec()
}
